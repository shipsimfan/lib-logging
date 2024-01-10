use super::LogRecord;
use shared_state::SharedState;
use std::{
    sync::{mpsc::SyncSender, Arc},
    thread::JoinHandle,
};

mod exec;
mod shared_state;

/// The thread which writes logs to the output
pub(super) struct LogOutputThread {
    /// The state shared between this struct and the log output thread
    shared_state: Arc<SharedState>,

    /// The queue to send records to the log output thread
    log_queue: SyncSender<LogRecord>,

    /// The join handle for the log output thread
    join_handle: Option<JoinHandle<()>>,
}

impl LogOutputThread {
    /// Creates a new [`LogOutputThread`]
    pub(super) fn new(log_queue_size: usize) -> Self {
        let (sender, receiver) = std::sync::mpsc::sync_channel(log_queue_size);

        let shared_state = SharedState::new();
        let child_shared_state = shared_state.clone();

        let join_handle =
            std::thread::spawn(move || exec::log_thread(child_shared_state, receiver));

        LogOutputThread {
            shared_state,
            log_queue: sender,
            join_handle: Some(join_handle),
        }
    }
}

impl Drop for LogOutputThread {
    fn drop(&mut self) {
        // Signal the thread to die
        self.shared_state.kill();

        // Wait for it to die
        self.join_handle.take().unwrap().join().unwrap();
    }
}
