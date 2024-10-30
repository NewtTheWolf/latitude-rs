use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Result as JsonResult;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct RunDocument<T> {
    pub path: String,
    pub parameters: T,
    pub stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct RunResponse {
    pub uuid: Option<String>,
    pub response: Option<ResponseDetail>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseDetail {
    pub text: String,
    pub usage: UsageDetail,
}

#[derive(Debug, Deserialize)]
pub struct UsageDetail {
    pub prompt_tokens: Option<usize>,
    pub completion_tokens: Option<usize>,
    pub total_tokens: Option<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Event {
    LatitudeEvent(LatitudeEvent),
    ProviderEvent(ProviderEvent),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatitudeEvent {
    #[serde(rename = "type")]
    pub event_type: LatitudeEventType,
    pub is_last_step: bool,
    pub uuid: Uuid,
    pub config: Config,
    pub messages: Vec<Message>,
    pub response: Response,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LatitudeEventType {
    ChainStep,
    ChainStepComplete,
    ChainComplete,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub provider: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    role: Role,
    content: Vec<Content>,
}

#[derive(Debug, Deserialize)]
pub enum Role {
    System,
    Assistant,
    User,
}

#[derive(Debug, Deserialize)]
pub struct Content {
    r#type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub text: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Debug, Deserialize)]
pub struct ProviderEvent {
    #[serde(rename = "type")]
    pub event_type: ProviderEventType,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderEventType {
    TextDelta(TextDelta),
    StepFinish(ProviderFinish),
    Finish(ProviderFinish),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDelta {
    pub text_delta: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderFinish {
    pub finish_reason: String,
    pub usage: Usage,
    pub response: ProviderResponse,
    pub is_continued: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderResponse {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub model_id: String,
}
