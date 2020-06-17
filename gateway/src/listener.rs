use crate::EventTypeFlags;
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
    pub events: EventTypeFlags,
    pub tx: UnboundedSender<T>,
}

#[derive(Debug)]
struct ListenersRef<T> {
    id: AtomicU64,
    listeners: Mutex<HashMap<u64, Listener<T>>>,
}

impl<T> Default for ListenersRef<T> {
    fn default() -> Self {
        Self {
            id: AtomicU64::new(0),
            listeners: Mutex::new(HashMap::default()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Listeners<T>(Arc<ListenersRef<T>>);

impl<T> Listeners<T> {
    pub async fn add(&self, events: EventTypeFlags) -> UnboundedReceiver<T> {
        let id = self.0.id.fetch_add(1, Ordering::Release) + 1;
        let (tx, rx) = mpsc::unbounded();

        self.0
            .listeners
            .lock()
            .await
            .insert(id, Listener { events, tx });

        rx
    }

    pub fn all(&self) -> &Mutex<HashMap<u64, Listener<T>>> {
        &self.0.listeners
    }

    pub async fn remove_all(&self) {
        self.0.listeners.lock().await.clear();
    }
}

impl<T> Default for Listeners<T> {
    fn default() -> Self {
        Self(Arc::new(ListenersRef::default()))
    }
}
