use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Deref,
    sync::Arc,
};

use arc_swap::ArcSwap;
use tokio::sync::{RwLock as AsyncRwLock, broadcast::Sender};
use uuid::Uuid;

#[derive(Clone)]
pub struct Authenticated(Arc<AuthenticatedInner>);

struct AuthenticatedInner {
    /// uuid that waiting for auth
    uuid: ArcSwap<Option<Uuid>>,
    tx: AsyncRwLock<Option<Sender<()>>>,
}

// The whole thing below is just an observable boolean
impl Authenticated {
    pub fn new() -> Self {
        Self(Arc::new(AuthenticatedInner {
            uuid: ArcSwap::new(Arc::new(Some(
                Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(),
            ))),
            tx: AsyncRwLock::new(None),
        }))
    }

    /// invoking 'set' means auth success
    pub async fn set(&self, uuid: Uuid) {
        self.0.uuid.store(Some(uuid).into());
        if let Some(tx) = self.0.tx.read().await.deref() {
            // It will fail if there is no active receiver
            _ = tx.send(());
        } else {
            // TODO LOGGIING multi auth packet
        }
        // Drop broadcast sender
        self.0.tx.write().await.take();
    }

    pub fn get(&self) -> Option<Uuid> {
        **self.0.uuid.load()
    }

    /// waiting for auth success
    pub async fn wait(&self) {
        let guard = self.0.tx.read().await;
        if let Some(tx) = guard.deref() {
            let mut rx = tx.subscribe();
            drop(guard);
            // It will fail when 1. sender been dropped 2. channel buffer overflow(multi
            // auth packet)
            _ = rx.recv().await;
        }
        // If the `tx` already `None`, that's meaning `set` had been invoked
    }
}

impl Display for Authenticated {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(uuid) = self.get() {
            write!(f, "{uuid}")
        } else {
            write!(f, "unauthenticated")
        }
    }
}
