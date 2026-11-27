//! Application use cases for DocuForge.

use std::path::PathBuf;

pub use docuforge_jobs::{JobEvent, JobQueue, QueueError};
use docuforge_pdf::MergeError;

/// The result produced by the PDF merge worker.
pub type MergeResult = Result<usize, MergeError>;

/// A failure while validating or submitting a merge request.
#[derive(Debug, thiserror::Error)]
pub enum StartMergeError {
    /// The request contains fewer than two sources.
    #[error(transparent)]
    Invalid(#[from] MergeError),
    /// The worker did not accept the job.
    #[error(transparent)]
    Queue(#[from] QueueError),
}

/// Validate a merge request and submit it for background processing.
///
/// Sources are merged in the order supplied. File validation and output
/// publication run on the blocking worker.
pub fn start_pdf_merge(
    queue: &JobQueue,
    inputs: Vec<PathBuf>,
    output: PathBuf,
) -> Result<tokio::sync::mpsc::Receiver<JobEvent<MergeResult>>, StartMergeError> {
    if inputs.len() < 2 {
        return Err(MergeError::TooFewInputs.into());
    }
    Ok(queue.submit(move || docuforge_pdf::merge(&inputs, &output))?)
}
