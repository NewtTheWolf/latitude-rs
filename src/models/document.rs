use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::error::Error;

/// `RunDocument` represents a document request with specific parameters.
/// The `parameters` field is optional, allowing for requests without parameters.
#[derive(Debug, Deserialize, Serialize)]
pub struct RunDocument<T>
where
    T: Serialize,
{
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<T>,
    pub stream: bool,
    #[serde(skip)]
    pub options: Option<Options>,
}

impl<T> RunDocument<T>
where
    T: Serialize + Default + std::fmt::Debug,
{
    /// Creates a new `RunDocument` with the specified path, parameters, and stream options.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the document to be run.
    /// * `parameters` - Optional parameters for the document.
    /// * `stream` - Boolean indicating if the response should be streamed.
    pub fn new(
        path: String,
        parameters: Option<T>,
        stream: bool,
        options: Option<Options>,
    ) -> Self {
        Self {
            path,
            parameters: parameters.or_else(|| Some(T::default())),
            stream,
            options,
        }
    }

    pub fn builder() -> RunDocumentBuilder<T> {
        RunDocumentBuilder::default()
    }
}

/// A builder for creating `RunDocument` instances.
///
/// This builder allows you to set optional fields, such as `parameters`, before building
/// the `RunDocument` instance. By default, if `parameters` is not set, it will use `T::default()`.
pub struct RunDocumentBuilder<T>
where
    T: Serialize + Default,
{
    pub path: Option<String>,
    pub parameters: Option<T>,
    pub stream: Option<bool>,
    pub options: Option<Options>,
}

impl<T> Default for RunDocumentBuilder<T>
where
    T: Serialize + Default,
{
    fn default() -> Self {
        Self {
            path: None,
            parameters: None,
            stream: None,
            options: None,
        }
    }
}

impl<T> RunDocumentBuilder<T>
where
    T: Serialize + Default,
{
    /// Sets the path for the `RunDocument`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the document to be run.
    pub fn path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    /// Sets the optional parameters for the `RunDocument`.
    ///
    /// # Arguments
    ///
    /// * `parameters` - Optional parameters for the document.
    pub fn parameters(mut self, parameters: T) -> Self {
        self.parameters = Some(parameters);
        self
    }

    /// Sets the stream options for the `RunDocument`.
    ///
    /// # Arguments
    ///
    /// * `stream` - Boolean indicating if the response should be streamed.
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Sets the Options for the `RunDocument`.
    ///
    /// # Arguments
    /// * `options` - The options to be set in the configuration.
    pub fn options(mut self, options: Options) -> Self {
        self.options = Some(options);
        self
    }

    /// Builds the `RunDocument` instance with the specified parameters.
    ///
    /// If `parameters` is not provided, it will default to `T::default()`.
    /// If `stream` is not provided, it will default to `false`.
    ///
    /// # Returns
    ///
    /// A `RunDocument` instance.
    pub fn build(self) -> Result<RunDocument<T>, Error> {
        Ok(RunDocument {
            path: self
                .path
                .ok_or(Error::ConfigError("Path is required".to_string()))?,
            parameters: self.parameters,
            stream: self.stream.unwrap_or(false),
            options: self.options,
        })
    }
}

/// Represents the configuration settings
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Options {
    pub version_id: Option<String>,
    pub project_id: Option<u64>,
}

impl Options {
    /// Creates a new `Options` instance with the specified version ID and project ID.
    ///
    /// # Arguments
    ///
    /// * `version_id` - The version ID to be set in the configuration.
    /// * `project_id` - The project ID to be set in the configuration.
    pub fn new(version_id: Option<String>, project_id: Option<u64>) -> Self {
        Self {
            version_id,
            project_id,
        }
    }

    pub fn builder() -> OptionsBuilder {
        OptionsBuilder::default()
    }
}

pub struct OptionsBuilder {
    pub version_id: Option<String>,
    pub project_id: Option<u64>,
}

impl Default for OptionsBuilder {
    fn default() -> Self {
        Self {
            version_id: None,
            project_id: None,
        }
    }
}

impl OptionsBuilder {
    /// Sets the version ID for the `Options`.
    ///
    /// # Arguments
    ///
    /// * `version_id` - The version ID to be set in the configuration.
    pub fn version_id(mut self, version_id: String) -> Self {
        self.version_id = Some(version_id);
        self
    }

    /// Sets the project ID for the `Options`.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID to be set in the configuration.
    pub fn project_id(mut self, project_id: u64) -> Self {
        self.project_id = Some(project_id);
        self
    }

    /// Builds the `Options` instance with the specified version ID and project ID.
    ///
    /// # Returns
    ///
    /// An `Options` instance.
    pub fn build(self) -> Options {
        Options {
            version_id: self.version_id,
            project_id: self.project_id,
        }
    }
}

/// RunResponse represents the response returned after executing a document.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct RunResponse {
    pub uuid: String,
    pub response: ResponseDetail,
}

/// ResponseDetail provides detailed response data including generated text and token usage.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ResponseDetail {
    pub text: String,
    pub usage: UsageDetail,
}

/// UsageDetail contains detailed usage statistics, such as token counts.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct UsageDetail {
    pub prompt_tokens: Option<usize>,
    pub completion_tokens: Option<usize>,
    pub total_tokens: Option<usize>,
}

/// Event enumerates the possible event types, which may either be latitude events or provider events.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Event {
    LatitudeEvent(LatitudeEvent),
    ProviderEvent(ProviderEvent),
    UnknownEvent,
}

/// LatitudeEvent represents an event from Latitude, detailing event type and associated data.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct LatitudeEvent {
    #[serde(rename = "type", flatten)]
    pub event_type: LatitudeEventType,
}

/// LatitudeEventType specifies different types of Latitude events, such as steps in the execution chain.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum LatitudeEventType {
    ChainStep(ChainStep),
    ChainComplete(ChainComplete),
    #[serde(other)]
    ChainStepComplete,
}

/// ChainStep represents a single step in the execution chain, providing configuration and message details.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChainStep {
    pub is_last_step: bool,
    pub config: Config,
    pub messages: Vec<Message>,
    pub uuid: Uuid,
}

/// ChainComplete represents a completed chain with response and configuration details.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ChainComplete {
    pub config: Config,
    pub response: Response,
    pub messages: Vec<Message>,
}

/// Config contains provider and model information for executing a request.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub provider: String,
    pub model: String,
}

/// Message represents a message exchanged in the process, including role and content details.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: Role,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub content: String,
}

/// Role enumerates the different roles involved in message exchange (e.g., System, Assistant, User).
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
}

/// Content represents individual message content with type and text fields.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Content {
    pub r#type: String,
    pub text: String,
}

/// Response provides details of the generated response, including text, tool calls, and usage.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub text: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub usage: Usage,
}

/// ToolCall represents a call to an external tool with specific arguments.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

/// Usage provides the token usage statistics for a given response.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

/// ProviderEvent represents an event from the provider, with details about the event type.
#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct ProviderEvent {
    #[serde(rename = "type", flatten)]
    pub event_type: ProviderEventType,
}

/// ProviderEventType enumerates different provider event types (e.g., text deltas, tool results).
#[derive(Debug, Deserialize, Eq, PartialEq)]
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
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextDelta {
    pub text_delta: String,
}

/// ToolCallEvent represents an event indicating a call to an external tool.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallEvent {
    pub tool_call_id: String,
    pub tool_name: String,
    pub args: Value,
}

/// ToolResultEvent provides the result of a tool call, including result data.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultEvent {
    pub tool_call_id: String,
    pub tool_name: String,
    pub result: Value,
}

/// StepFinish represents the completion of a step, with details on usage and the response.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StepFinish {
    pub finish_reason: FinishReason,
    pub usage: Usage,
    pub response: ProviderResponse,
    pub is_continued: bool,
}

/// FinishReason enumerates the reasons why a step finished.
#[derive(Debug, Deserialize, Eq, PartialEq)]
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
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderFinish {
    pub finish_reason: String,
    pub usage: Usage,
    pub response: ProviderResponse,
    pub is_continued: Option<bool>,
}

/// ProviderResponse contains metadata for the provider's response, such as ID and timestamp.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderResponse {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub model_id: String,
}

/// ErrorEvent represents an error encountered during processing.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvent {
    pub error_message: String,
    pub error_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn test_run_document_new_with_parameters() {
        let options = Some(Options::new(Some("v1".to_string()), Some(123)));
        let document = RunDocument::new(
            "test/path".to_string(),
            Some("parameter_value".to_string()),
            true,
            options,
        );

        assert_eq!(document.path, "test/path");
        assert_eq!(document.parameters, Some("parameter_value".to_string()));
        assert_eq!(document.stream, true);
        assert_eq!(
            document.options.clone().unwrap().version_id,
            Some("v1".to_string())
        );
        assert_eq!(document.options.clone().unwrap().project_id, Some(123));
    }

    #[test]
    fn test_run_document_new_with_default_parameters() {
        let document: RunDocument<()> =
            RunDocument::new("test/path".to_string(), None, false, None);

        assert_eq!(document.path, "test/path");
        assert_eq!(document.parameters, Some(())); // Da T = (), wird Some(()) erwartet
        assert_eq!(document.stream, false);
        assert!(document.options.is_none());
    }

    #[test]
    fn test_run_document_builder_full_configuration() {
        let options = Options::builder()
            .version_id("v1".to_string())
            .project_id(123)
            .build();

        let document = RunDocument::builder()
            .path("test/path".to_string())
            .parameters("parameter_value".to_string())
            .stream(true)
            .options(options)
            .build()
            .unwrap();

        assert_eq!(document.path, "test/path");
        assert_eq!(document.parameters, Some("parameter_value".to_string()));
        assert_eq!(document.stream, true);
        assert_eq!(
            document.options.clone().unwrap().version_id,
            Some("v1".to_string())
        );
        assert_eq!(document.options.clone().unwrap().project_id, Some(123));
    }

    #[ignore]
    #[test]
    fn test_run_document_builder_with_defaults() {
        let document = RunDocument::<()>::builder()
            .path("test/path".to_string())
            .build()
            .unwrap();

        assert_eq!(document.path, "test/path");
        assert_eq!(document.parameters, Some(())); // Default für Option<T> ist Some(T::default())
        assert_eq!(document.stream, false); // Stream default ist `false`
        assert!(document.options.is_none()); // Options ist nicht gesetzt, also `None`
    }

    #[test]
    fn test_run_document_builder_missing_path() {
        let result = RunDocument::<()>::builder().build();

        assert!(result.is_err());
        if let Err(Error::ConfigError(msg)) = result {
            assert_eq!(msg, "Path is required");
        } else {
            panic!("Expected ConfigError for missing path");
        }
    }
}
