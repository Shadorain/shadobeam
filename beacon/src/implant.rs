use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::task::Task;

type HM = HashMap<Uuid, Arc<Implant>>;

#[derive(Debug, Default)]
pub struct Implants {
    implants: Arc<RwLock<HM>>,
}

impl Implants {
    pub async fn add_task(&self, task: Task) {
        let map = self.implants.read().await;

        for (_, val) in map.iter() {
            val.push_task(task.clone()).await;
        }
    }

    pub async fn list(&self) -> Vec<String> {
        let map = self.implants.read().await;
        map.keys().map(|i| i.to_string()).collect()
    }

    pub async fn get<'a>(&'a self, id: &'a Uuid) -> Option<Arc<Implant>> {
        let map = self.implants.read().await;
        map.get(id).cloned()
    }

    pub async fn insert(&self, id: Uuid, implant: Implant) {
        let mut map = self.implants.write().await;
        map.insert(id, implant.into());
    }
}

#[derive(Debug, Default)]
pub struct Implant {
    pub tasks: RwLock<VecDeque<Task>>,
}

impl Implant {
    pub async fn push_task(&self, task: Task) {
        self.tasks.write().await.push_back(task.clone());
    }
    pub async fn pop_task(&self) -> Option<Task> {
        self.tasks.write().await.pop_front()
    }
}
