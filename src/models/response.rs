use crate::models::event::Event;
use tokio::sync::mpsc::Receiver;

use super::document::RunResponse;

/// Enum to represent the response type from the `run` method.
#[derive(Debug)]
pub enum Response {
    /// JSON response when `stream` is set to `false`.
    Json(RunResponse),
    /// Streaming response when `stream` is set to `true`.
    Stream(Receiver<Event>),
}
