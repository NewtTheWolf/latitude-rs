use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::message::Role;

/// Event enumerates the possible event types, which may either be latitude events or provider events.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Event {
    LatitudeEvent(LatitudeEvent),
    ProviderEvent(ProviderEvent),
    UnknownEvent,
}

/// LatitudeEvent represents an event from Latitude, detailing event type and associated data.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LatitudeEvent {
    #[serde(rename = "type", flatten)]
    pub event_type: LatitudeEventType,
}

/// LatitudeEventType specifies different types of Latitude events, such as steps in the execution chain.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LatitudeEventType {
    ChainStep(ChainStep),
    ChainStepComplete(ChainStepComplete),
    ChainComplete(ChainComplete),
}

/// ChainStep represents a single step in the execution chain, providing configuration and message details.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChainStep {
    pub is_last_step: bool,
    pub config: Config,
    pub messages: Vec<Message>,
    pub uuid: Uuid,
}

/// ChainStepComplete represents the completion of a chain step, indicating that the step has finished.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChainStepComplete {
    response: Response,
    uuid: String,
}

/// ChainComplete represents a completed chain with response and configuration details.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ChainComplete {
    pub config: Config,
    pub response: Response,
    pub messages: Vec<Message>,
}

/// Config contains provider and model information for executing a request.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub provider: String,
    pub model: String,
}

/// Message represents a message exchanged in the process, including role and content details.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: Role,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub content: String,
}

/// Content represents individual message content with type and text fields.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Content {
    pub r#type: String,
    pub text: String,
}

/// Response provides details of the generated response, including text, tool calls, and usage.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub stream_type: Option<String>,
    pub document_log_uuid: Option<String>,
    pub text: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub usage: Usage,
}

/// ToolCall represents a call to an external tool with specific arguments.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

/// Usage provides the token usage statistics for a given response.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// ProviderEvent represents an event from the provider, with details about the event type.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ProviderEvent {
    #[serde(rename = "type", flatten)]
    pub event_type: ProviderEventType,
}

/// ProviderEventType enumerates different provider event types (e.g., text deltas, tool results).
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ProviderEventType {
    TextDelta(TextDelta),
    ToolCall(ToolCallEvent),
    ToolResult(ToolResultEvent),
    StepFinish(StepFinish),
    Finish(ProviderFinish),
    Error(ErrorEvent),
}

/// TextDelta provides a delta update for streamed text content.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextDelta {
    pub text_delta: String,
}

/// ToolCallEvent represents an event indicating a call to an external tool.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallEvent {
    pub tool_call_id: String,
    pub tool_name: String,
    pub args: Value,
}

/// ToolResultEvent provides the result of a tool call, including result data.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultEvent {
    pub tool_call_id: String,
    pub tool_name: String,
    pub result: Value,
}

/// StepFinish represents the completion of a step, with details on usage and the response.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StepFinish {
    pub finish_reason: FinishReason,
    pub usage: Usage,
    pub response: ProviderResponse,
    pub is_continued: bool,
}

/// FinishReason enumerates the reasons why a step finished.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum FinishReason {
    Stop,
    Length,
    ContentFilter,
    ToolCalls,
    Error,
    Other,
    Unknown,
}

/// ProviderFinish represents the final result from the provider, including usage and continuation status.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderFinish {
    pub finish_reason: String,
    pub usage: Usage,
    pub response: ProviderResponse,
    pub is_continued: Option<bool>,
}

/// ProviderResponse contains metadata for the provider's response, such as ID and timestamp.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderResponse {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub model_id: String,
}

/// ErrorEvent represents an error encountered during processing.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvent {
    pub error_message: String,
    pub error_code: Option<String>,
}
