use super::{LogRecord, SharedState};
use std::{
    sync::{mpsc::Receiver, Arc},
    time::Duration,
};

/// How long should the thread wait before checking if it is killed?
const TIMEOUT: Duration = Duration::from_millis(500);

/// Run the log_thread
pub(super) fn log_thread(state: Arc<SharedState>, log_queue: Receiver<LogRecord>) {
    while !state.is_killed() {
        // Get the next record, or continue to check if we are killed on a timeout
        let record = match log_queue.recv_timeout(TIMEOUT) {
            Ok(record) => record,
            Err(_) => continue,
        };

        todo!("Handle record");
    }
}
