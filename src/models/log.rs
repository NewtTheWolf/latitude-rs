use super::{message::Message, options::Options};
use crate::error::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a structured log with a path, a collection of messages, a response, and options.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Log {
    pub path: String,
    pub messages: Vec<Message>,
    pub response: String,
    #[serde(skip)]
    pub options: Option<Options>,
}

impl Log {
    /// Creates a new `Log` instance with the given path, messages, response, and options.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the log.
    /// * `messages` - A vector of `Message` items to be logged.
    /// * `response` - The response associated with the log.
    /// * `options` - Additional options for the log.
    ///
    /// # Returns
    ///
    /// A new `Log` instance.
    pub fn new(
        path: String,
        messages: Vec<Message>,
        response: String,
        options: Option<Options>,
    ) -> Self {
        Self {
            path,
            messages,
            response,
            options,
        }
    }

    /// Creates a new `LogBuilder` instance.
    ///
    /// # Returns
    ///
    /// A `LogBuilder` for constructing a `Log` instance step-by-step.
    pub fn builder() -> LogBuilder {
        LogBuilder::new()
    }
}

/// A builder for creating `Log` instances with a specified path, messages, response, and options.
pub struct LogBuilder {
    path: Option<String>,
    messages: Vec<Message>,
    response: Option<String>,
    options: Option<Options>,
}

impl LogBuilder {
    /// Creates a new `LogBuilder` instance.
    ///
    /// # Returns
    ///
    /// A `LogBuilder` instance with empty path, messages, response, and options fields.
    pub fn new() -> Self {
        Self {
            path: None,
            messages: vec![],
            response: None,
            options: None,
        }
    }

    /// Sets the path for the `Log`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the log (e.g., file path or identifier).
    ///
    /// # Returns
    ///
    /// The builder instance with the specified path.
    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_owned());
        self
    }

    /// Adds a message to the `Log`.
    ///
    /// # Arguments
    ///
    /// * `message` - A `Message` instance to be added to the log.
    ///
    /// # Returns
    ///
    /// The builder instance with the new message added.
    pub fn add_message(mut self, message: Message) -> Self {
        self.messages.push(message);
        self
    }

    /// Sets the response for the `Log`.
    ///
    /// # Arguments
    ///
    /// * `response` - The response associated with the log.
    ///
    /// # Returns
    ///
    /// The builder instance with the specified response.
    pub fn response(mut self, response: &str) -> Self {
        self.response = Some(response.to_owned());
        self
    }

    /// Sets the options for the `Log`.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional `Options` for configuring the log.
    ///
    /// # Returns
    ///
    /// The builder instance with the specified options.
    pub fn options(mut self, options: Options) -> Self {
        self.options = Some(options);
        self
    }

    /// Builds the `Log` instance.
    ///
    /// # Returns
    ///
    /// A `Log` instance with the specified path, messages, response, and options.
    /// Returns an `Error` if any required field is missing.
    pub fn build(self) -> Result<Log, Error> {
        Ok(Log {
            path: self
                .path
                .ok_or(Error::ConfigError("Path is required".to_owned()))?,
            messages: self.messages,
            response: self
                .response
                .ok_or(Error::ConfigError("Response is required".to_owned()))?,
            options: self.options,
        })
    }
}

impl Default for LogBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogResponse {
    pub id: i64,
    pub uuid: String,
    pub document_uuid: String,
    pub commit_id: i64,
    pub resolved_content: String,
    pub content_hash: String,
    pub parameters: Value,
    pub custom_identifier: Value,
    pub duration: Value,
    pub source: String,
    pub created_at: String,
    pub updated_at: String,
}
