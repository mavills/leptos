//! Types that handle asynchronous data loading via `<Suspense/>`.

#![forbid(unsafe_code)]
use crate::{
    create_rw_signal, create_signal, queue_microtask, store_value, ReadSignal,
    RwSignal, Scope, SignalUpdate, StoredValue, WriteSignal,
};
use futures::Future;
use std::{borrow::Cow, collections::VecDeque, pin::Pin};

/// Tracks [`Resource`](crate::Resource)s that are read under a suspense context,
/// i.e., within a [`Suspense`](https://docs.rs/leptos_core/latest/leptos_core/fn.Suspense.html) component.
#[derive(Copy, Clone, Debug)]
pub struct SuspenseContext {
    /// The number of resources that are currently pending.
    pub pending_resources: ReadSignal<usize>,
    set_pending_resources: WriteSignal<usize>,
    pub(crate) pending_serializable_resources: RwSignal<usize>,
    pub(crate) has_local_only: StoredValue<bool>,
    pub(crate) should_block: StoredValue<bool>,
}

impl SuspenseContext {
    /// Whether the suspense contains local resources at this moment,
    /// and therefore can't be serialized
    pub fn has_local_only(&self) -> bool {
        self.has_local_only.get_value()
    }

    /// Whether any blocking resources are read under this suspense context,
    /// meaning the HTML stream should not begin until it has resolved.
    pub fn should_block(&self) -> bool {
        self.should_block.get_value()
    }
}

impl std::hash::Hash for SuspenseContext {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pending_resources.id.hash(state);
    }
}

impl PartialEq for SuspenseContext {
    fn eq(&self, other: &Self) -> bool {
        self.pending_resources.id == other.pending_resources.id
    }
}

impl Eq for SuspenseContext {}

impl SuspenseContext {
    /// Creates an empty suspense context.
    pub fn new() -> Self {
        let (pending_resources, set_pending_resources) = create_signal(0);
        let pending_serializable_resources = create_rw_signal(0);
        let has_local_only = store_value(true);
        let should_block = store_value(false);
        Self {
            pending_resources,
            set_pending_resources,
            pending_serializable_resources,
            has_local_only,
            should_block,
        }
    }

    /// Notifies the suspense context that a new resource is now pending.
    pub fn increment(&self, serializable: bool) {
        let setter = self.set_pending_resources;
        let serializable_resources = self.pending_serializable_resources;
        let has_local_only = self.has_local_only;
        queue_microtask(move || {
            setter.update(|n| *n += 1);
            if serializable {
                serializable_resources.update(|n| *n += 1);
                has_local_only.set_value(false);
            }
        });
    }

    /// Notifies the suspense context that a resource has resolved.
    pub fn decrement(&self, serializable: bool) {
        let setter = self.set_pending_resources;
        let serializable_resources = self.pending_serializable_resources;
        queue_microtask(move || {
            setter.update(|n| {
                if *n > 0 {
                    *n -= 1
                }
            });
            if serializable {
                serializable_resources.update(|n| {
                    if *n > 0 {
                        *n -= 1;
                    }
                });
            }
        });
    }

    /// Tests whether all of the pending resources have resolved.
    pub fn ready(&self) -> bool {
        self.pending_resources
            .try_with(|n| *n == 0)
            .unwrap_or(false)
    }
}

/// Represents a chunk in a stream of HTML.
pub enum StreamChunk {
    /// A chunk of synchronous HTML.
    Sync(Cow<'static, str>),
    /// A future that resolves to be a list of additional chunks.
    Async {
        /// The HTML chunks this contains.
        chunks: Pin<Box<dyn Future<Output = VecDeque<StreamChunk>>>>,
        /// Whether this should block the stream.
        should_block: bool,
    },
}

impl std::fmt::Debug for StreamChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamChunk::Sync(data) => write!(f, "StreamChunk::Sync({data:?})"),
            StreamChunk::Async { .. } => write!(f, "StreamChunk::Async(_)"),
        }
    }
}
