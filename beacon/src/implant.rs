use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use shadobeam_proto::{ImplantInfo, Task};
use tokio::sync::RwLock;
use uuid::Uuid;

type HM = HashMap<Uuid, Arc<Implant>>;

#[derive(Debug, Default)]
pub struct Implants {
    implants: Arc<RwLock<HM>>,
}

impl Implants {
    pub async fn add(&self, id: Uuid, implant: Implant) {
        let mut map = self.implants.write().await;
        map.insert(id, implant.into());
    }

    // pub async fn add_task(&self, task: Task) {
    //     let map = self.implants.read().await;
    //
    //     for (_, val) in map.iter() {
    //         val.push_task(task.clone()).await;
    //     }
    // }

    pub async fn list(&self) -> Vec<ImplantInfo> {
        let map = self.implants.read().await;
        map.values().map(|i| i.info.clone()).collect()
    }

    pub async fn get<'a>(&'a self, id: &'a Uuid) -> Option<Arc<Implant>> {
        let map = self.implants.read().await;
        map.get(id).cloned()
    }
}

#[derive(Debug)]
pub struct Implant {
    pub tasks: RwLock<VecDeque<Task>>,
    info: ImplantInfo,
}

impl Implant {
    pub fn new(info: ImplantInfo) -> Self {
        Self {
            tasks: RwLock::new(VecDeque::new()),
            info,
        }
    }
    pub async fn push_task(&self, task: Task) {
        self.tasks.write().await.push_back(task.clone());
    }
    pub async fn pop_task(&self) -> Option<Task> {
        self.tasks.write().await.pop_front()
    }
}
