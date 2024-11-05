use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;

use super::options::Options;

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
    pub fn stream(mut self) -> Self {
        self.stream = Some(true);
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: i64,
    pub document_uuid: String,
    pub path: String,
    pub content: String,
    pub resolved_content: String,
    pub content_hash: String,
    pub commit_id: i64,
    pub deleted_at: Value,
    pub created_at: String,
    pub updated_at: String,
    pub merged_at: Option<String>,
    pub project_id: i64,
    pub config: Config,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub provider: String,
    pub model: String,
}
