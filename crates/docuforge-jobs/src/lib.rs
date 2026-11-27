//! Bounded execution of blocking document jobs on Tokio.

use std::sync::Arc;

use tokio::{
    runtime::Handle,
    sync::{Semaphore, mpsc},
};

/// A shared executor allowing one active desktop job at a time.
#[derive(Clone)]
pub struct JobQueue {
    slot: Arc<Semaphore>,
}

impl Default for JobQueue {
    fn default() -> Self {
        Self {
            slot: Arc::new(Semaphore::new(1)),
        }
    }
}

/// A reason a job could not be submitted or executed.
#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    /// The sole worker slot is already occupied.
    #[error("another job is already running")]
    Busy,
    /// The caller did not enter a Tokio runtime.
    #[error("the job runtime is unavailable")]
    NoRuntime,
    /// The worker terminated before returning an operation result.
    #[error("the background worker stopped: {0}")]
    Worker(#[source] tokio::task::JoinError),
}

/// Progress and completion events for one accepted job.
pub enum JobEvent<T> {
    /// The blocking worker has started.
    Processing,
    /// The worker returned or stopped unexpectedly.
    Finished(Result<T, QueueError>),
}

impl JobQueue {
    /// Reserve a worker slot and submit a blocking operation. The receiver gets
    /// the processing event before completion and then closes.
    pub fn submit<T: Send + 'static>(
        &self,
        operation: impl FnOnce() -> T + Send + 'static,
    ) -> Result<mpsc::Receiver<JobEvent<T>>, QueueError> {
        let runtime = Handle::try_current().map_err(|_| QueueError::NoRuntime)?;
        let slot = Arc::clone(&self.slot)
            .try_acquire_owned()
            .map_err(|_| QueueError::Busy)?;
        let (sender, receiver) = mpsc::channel(2);
        let started = sender.clone();
        let worker = runtime.spawn_blocking(move || {
            let _slot = slot;
            // Once started, finish the operation even if the presentation closes.
            let _ = started.try_send(JobEvent::Processing);
            operation()
        });

        runtime.spawn(async move {
            let result = worker.await.map_err(QueueError::Worker);
            let _ = sender.send(JobEvent::Finished(result)).await;
        });
        Ok(receiver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{runtime::Builder, sync::oneshot};

    #[test]
    fn limits_processing_and_releases_the_slot_after_completion() {
        Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(async {
                let queue = JobQueue::default();
                let (release, wait) = oneshot::channel();
                let mut events = queue.submit(move || wait.blocking_recv().unwrap()).unwrap();

                assert!(matches!(events.recv().await, Some(JobEvent::Processing)));
                assert!(matches!(queue.submit(|| ()), Err(QueueError::Busy)));

                release.send(42).unwrap();
                assert!(matches!(
                    events.recv().await,
                    Some(JobEvent::Finished(Ok(42)))
                ));
                assert!(queue.submit(|| ()).is_ok());
            });
    }
}
