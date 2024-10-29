// models/document.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
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

// Types for streaming response

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Chunk {
    pub event: String,
    pub data: Data,
    pub id: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChunkData {
    pub event: String,
    pub data: serde_json::Value, // or a more specific type if desired
    pub id: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Data {
    ChainStep(ChainStepData),
    ProviderEvent(ProviderEventData),
    ChainComplete(ChainCompleteData),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChainStepData {
    pub r#type: String,
    pub is_last_step: bool,
    pub config: Config,
    pub messages: Vec<Message>,
    pub uuid: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProviderEventData {
    pub r#type: String,
    pub object: Option<HashMap<String, String>>,
    pub text_delta: Option<String>,
    pub finish_reason: Option<String>,
    pub usage: Option<UsageDetail>,
    pub response: Option<ResponseDetail>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChainCompleteData {
    pub r#type: String,
    pub config: Config,
    pub response: ChainCompleteResponse,
    pub messages: Vec<AssistantMessage>,
    pub uuid: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub provider: String,
    pub model: String,
    pub schema: Schema,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Schema {
    pub r#type: String,
    pub properties: HashMap<String, PropertyType>,
    pub required: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct PropertyType {
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Message {
    pub role: String,
    pub content: MessageContent,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    List(Vec<TextContent>),
}

#[derive(Debug, Deserialize)]
pub struct TextContent {
    pub r#type: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ChainCompleteResponse {
    pub stream_type: String,
    pub usage: UsageDetail,
    pub text: String,
    pub object: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssistantMessage {
    pub role: String,
    pub tool_calls: Vec<String>,
    pub content: String,
}
