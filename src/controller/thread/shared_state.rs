use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// The state of the logging thread. This is shared between the [`LogController`] and the log
/// output thread.
pub(super) struct SharedState {
    /// Has the log output thread been killed?
    is_killed: AtomicBool,
}

impl SharedState {
    /// Creates a new [`SharedState`]
    pub(super) fn new() -> Arc<Self> {
        Arc::new(SharedState {
            is_killed: AtomicBool::new(false),
        })
    }

    /// Has the log output thread been killed?
    pub(super) fn is_killed(&self) -> bool {
        self.is_killed.load(Ordering::SeqCst)
    }

    /// Signals the log output thread to die
    pub(super) fn kill(&self) {
        self.is_killed.store(true, Ordering::SeqCst);
    }
}
