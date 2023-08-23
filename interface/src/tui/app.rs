use std::sync::Arc;

use anyhow::Result;
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinHandle,
};

use super::{
    components::{Base, Component},
    Action, EventHandler, Message, TerminalHandler,
};

pub struct App {
    tick_rate: (u64, u64),
    should_quit: bool,
    should_suspend: bool,

    base: Arc<Mutex<Base>>,

    task: Option<JoinHandle<()>>,
}

impl App {
    pub fn new(tick_rate: (u64, u64)) -> Result<Self> {
        Ok(Self {
            tick_rate,
            base: Arc::new(Mutex::new(Base::new())),
            should_quit: false,
            should_suspend: false,
            task: None,
        })
    }

    pub async fn run(
        &mut self,
        message_tx: Option<mpsc::UnboundedSender<Message>>,
        message_rx: Option<mpsc::UnboundedReceiver<Message>>,
    ) -> Result<()> {
        let (action_tx, mut action_rx) = mpsc::unbounded_channel();

        let mut terminal = TerminalHandler::new(self.base.clone());
        let mut event = EventHandler::new(self.tick_rate, self.base.clone(), action_tx.clone());
        self.message_task(action_tx.clone(), message_rx);

        self.base
            .lock()
            .await
            .init(action_tx.clone(), message_tx.clone())?;

        loop {
            if let Some(action) = action_rx.recv().await {
                match action {
                    Action::RenderTick => terminal.render()?,
                    Action::Quit => self.should_quit = true,
                    Action::Suspend => self.should_suspend = true,
                    Action::Resume => self.should_suspend = false,
                    _ => {
                        if action == Action::Tick {
                            if let Some(tx) = &message_tx {
                                tx.send(Message::Tick).unwrap();
                            }
                        }
                        if let Some(_action) = self.base.lock().await.dispatch(action) {
                            action_tx.send(_action)?
                        };
                    }
                }
            }
            if self.should_suspend {
                terminal.suspend()?;
                event.stop();
                terminal.task.await?;
                event.task.await?;
                terminal = TerminalHandler::new(self.base.clone());
                event = EventHandler::new(self.tick_rate, self.base.clone(), action_tx.clone());
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::RenderTick)?;
            } else if self.should_quit {
                if let Some(tx) = &message_tx {
                    tx.send(Message::Quit).unwrap();
                }
                terminal.stop()?;
                event.stop();
                if let Some(task) = self.task.take() {
                    task.await?;
                }
                terminal.task.await?;
                event.task.await?;
                break;
            }
        }
        Ok(())
    }

    fn message_task(
        &mut self,
        action_tx: mpsc::UnboundedSender<Action>,
        message_rx: Option<mpsc::UnboundedReceiver<Message>>,
    ) {
        let base = self.base.clone();
        if message_rx.is_some() {
            let task = tokio::spawn(async move {
                #[allow(clippy::unnecessary_unwrap)]
                let mut rx = message_rx.unwrap();
                loop {
                    if let Some(message) = rx.recv().await {
                        if let Some(action) = base.lock().await.message(message) {
                            action_tx.send(action).unwrap();
                        }
                    }
                }
            });
            self.task = Some(task);
        }
    }
}
