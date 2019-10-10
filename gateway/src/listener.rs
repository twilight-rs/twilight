use crate::shard::EventType;
use futures_channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures_util::lock::Mutex;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

pub struct Listener<T> {
    pub events: EventType,
    pub tx: UnboundedSender<T>,
}

#[derive(Debug)]
pub struct Listeners<T> {
    id: AtomicU64,
    pub(crate) listeners: Arc<Mutex<HashMap<u64, Listener<T>>>>,
}

impl<T> Listeners<T> {
    pub async fn add(&self, events: EventType) -> UnboundedReceiver<T> {
        let id = self.id.fetch_add(1, Ordering::Release) + 1;
        let (tx, rx) = mpsc::unbounded();

        self.listeners.lock().await.insert(
            id,
            Listener {
                events,
                tx,
            },
        );

        rx
    }

    pub async fn remove_all(&self) {
        self.listeners.lock().await.clear();
    }
}

impl<T> Default for Listeners<T> {
    fn default() -> Self {
        Self {
            id: AtomicU64::new(0),
            listeners: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
