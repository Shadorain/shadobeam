use std::collections::VecDeque;

use tokio::sync::RwLock;

use crate::Task;

#[derive(Debug, Default)]
pub struct Implant {
    tasks: RwLock<VecDeque<Task>>,
}
impl Implant {
    pub async fn push_task(&self, task: Task) {
        self.tasks.write().await.push_back(task.clone());
    }
    pub async fn pop_task(&self) -> Option<Task> {
        self.tasks.write().await.pop_front()
    }
}
