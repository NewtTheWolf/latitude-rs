// lib.rs
// Copyright 2024 NewtTheWolf
//
// Licensed under the MIT License <LICENSE-MIT or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

//! # Latitude API Client
//!
//! This crate provides a client for interacting with the Latitude API, allowing users to execute
//! documents (prompts) and handle real-time AI-powered conversations through a simple HTTP-based interface.
//!
//! ## Features
//! - **Document Execution**: Run specific documents (prompts) with custom parameters.
//! - **Stream Responses**: Optionally receive responses as a real-time data stream.
//! - **Simple API Integration**: API key authentication and project/version management.
//!
//! ## Installation
//!
//! Add this crate to your `Cargo.toml` file:
//!
//! ```sh
//! cargo add latitude
//! ```
//!
//! ## Usage
//!
//! To use the Latitude API client, create an instance of `Client` with your API key, set the project ID, and run a document.
//!
//! ```rust
//! use latitude::Client;
//!
//! let client = Client::new("your_api_key".into());
//! client.set_project_id(123);
//! client.set_version_id("version-uuid".parse().unwrap());
//! ```

use async_sse::decode;
use error::{ApiErrorCodes, Error, LatitudeErrorCodes};
use models::document::{self, Event, Message, RunDocument, RunResponse};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as ReqwestClient, StatusCode,
};
use serde::Serialize;
use tokio::{
    io::BufReader,
    sync::mpsc::{self, Receiver},
};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tokio_util::{
    compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt},
    io::StreamReader,
};

pub mod error;
pub mod models;

static BASE_URL: &str = "https://gateway.latitude.so/api/v2";
static APP_USER_AGENT: &str = env!("CARGO_PKG_NAME");

/// Enum to represent the response type from the `run` method.
#[derive(Debug)]
pub enum RunResult {
    /// JSON response when `stream` is set to `false`.
    Json(document::RunResponse),
    /// Streaming response when `stream` is set to `true`.
    Stream(Receiver<Event>),
}

/// The `Client` for interacting with the Latitude API.
///
/// The `Client` provides methods to execute documents and handle real-time
/// responses via the Latitude API. It is configured using the `ClientBuilder`,
/// which allows for flexible and customizable initialization.
///
/// ## Usage Example
///
/// ```
/// use latitude::Client;
///
/// let client = Client::builder("your_api_key".into())
///     .project_id(123)
///     .version_id("version-uuid".to_string())
///     .base_url("https://custom.url/api".to_string())
///     .build();
/// ```
#[derive(Clone)]
pub struct Client {
    /// The API key for authentication.
    api_key: String,
    /// The default project ID used in requests.
    project_id: Option<u64>,
    /// The default version UUID used in requests.
    version_id: Option<String>,
    /// Internal HTTP client for making requests.
    client: ReqwestClient,
    /// The base URL for API requests.
    base_url: String,
}

impl Client {
    /// Creates a new `ClientBuilder` with the required API key.
    ///
    /// The `ClientBuilder` enables optional configuration of `project_id`,
    /// `version_id`, and `base_url`. This approach allows for flexible client
    /// initialization, where only the API key is required.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for authenticating requests with the Latitude API.
    ///
    /// # Example
    ///
    /// ```
    /// let client_builder = Client::builder("your_api_key".into());
    /// ```
    pub fn builder(api_key: String) -> ClientBuilder {
        ClientBuilder {
            api_key,
            project_id: None,
            version_id: None,
            base_url: BASE_URL.into(),
        }
    }

    /// Creates a new `Client` with the provided API key.
    ///
    /// # Arguments
    /// * `api_key` - The API key for authenticating with the Latitude API.
    ///
    /// # Examples
    /// ```
    /// let client = Client::new("your_api_key".into());
    /// ```
    pub fn new(
        api_key: String,
        project_id: Option<u64>,
        version_id: Option<String>,
        base_url: Option<String>,
    ) -> Self {
        let mut headers = HeaderMap::new();
        let api_key_value =
            HeaderValue::from_str(&format!("Bearer {}", api_key)).expect("Invalid API key");
        headers.insert("Authorization", api_key_value);

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("Failed to create HTTP client");

        let base_url = base_url.unwrap_or_else(|| BASE_URL.into());

        Self {
            api_key,
            project_id,
            version_id,
            client,
            base_url,
        }
    }

    /// Runs a document with the specified path and user-defined parameters, with an option for streaming responses.
    ///
    /// # Arguments
    /// * `document` - The `RunDocument` struct containing the path, parameters, and an option to enable streaming.
    ///
    /// # Returns
    /// * `RunResult` - The response from the Latitude API, either as JSON or a stream of events (`LatitudeEvent` or `ProviderEvent`).
    ///
    /// # Examples
    ///
    /// Running a document with a JSON response:
    /// ```rust
    /// use latitude::{Client, models::document::RunDocument};
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Params {
    ///     user_message: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("your_api_key".into());
    ///     client.set_project_id(12345);
    ///     let params = Params {
    ///         user_message: "Hello, world!".to_owned(),
    ///     };
    ///
    ///     let document = RunDocument {
    ///         path: "Workers/EmotionAnalyzer".to_owned(),
    ///         parameters: params,
    ///         stream: false,
    ///     };
    ///
    ///     match client.run(document).await {
    ///         Ok(RunResult::Json(response)) => println!("JSON Response: {:?}", response),
    ///         Err(e) => eprintln!("Error: {:?}", e),
    ///     }
    /// }
    /// ```
    ///
    /// Running a document with a streaming response:
    /// ```rust
    /// use latitude::{Client, models::document::RunDocument};
    /// use serde::Serialize;
    /// use tokio_stream::StreamExt;
    ///
    /// #[derive(Serialize)]
    /// struct Params {
    ///     user_message: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new("your_api_key".into());
    ///     client.set_project_id(12345);
    ///     let params = Params {
    ///         user_message: "Hello, world!".to_owned(),
    ///     };
    ///
    ///     let document = RunDocument {
    ///         path: "Workers/EmotionAnalyzer".to_owned(),
    ///         parameters: params,
    ///         stream: true,
    ///     };
    ///
    ///     match client.run(document).await {
    ///         Ok(RunResult::Stream(mut event_stream)) => {
    ///             while let Some(event) = event_stream.recv().await {
    ///                 match event {
    ///                     Event::LatitudeEvent(data) => println!("Latitude Event: {:?}", data),
    ///                     Event::ProviderEvent(data) => println!("Provider Event: {:?}", data),
    ///                 }
    ///             }
    ///         },
    ///         Err(e) => eprintln!("Error: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn run<T>(&self, document: RunDocument<T>) -> Result<RunResult, Error>
    where
        T: Serialize + std::fmt::Debug,
    {
        let project_id = document
            .options
            .as_ref()
            .and_then(|opts| opts.project_id)
            .or(self.project_id)
            .ok_or_else(|| Error::ConfigError("Project ID is required".to_owned()))?;

        let version_id = document
            .options
            .as_ref()
            .and_then(|opts| opts.version_id.clone())
            .or(self.version_id.clone())
            .unwrap_or_else(|| "live".to_string());

        let url = format!(
            "{}/projects/{}/versions/{}/documents/run",
            self.base_url, project_id, version_id
        );

        println!("Base URL: {}", url);

        let response = self.client.post(&url).json(&document).send().await?;

        Self::check_status(response.status())?;

        if document.stream {
            let stream = response.bytes_stream();
            let (sender, receiver) = mpsc::channel(100);

            tokio::spawn(async move {
                let reader = StreamReader::new(stream.map(|result| {
                    result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
                }));
                let buffered_reader = BufReader::new(reader.compat().into_inner());
                let mut decoder = decode(buffered_reader.compat());

                while let Some(event) = decoder.next().await {
                    match event {
                        Ok(async_sse::Event::Message(message)) => {
                            let data = message.data();
                            let parsed_event = match message.name().as_str() {
                                "latitude-event" => serde_json::from_slice(&data)
                                    .map(Event::LatitudeEvent)
                                    .map_err(Error::from),
                                "provider-event" => serde_json::from_slice(&data)
                                    .map(Event::ProviderEvent)
                                    .map_err(Error::from),
                                _ => Ok(Event::UnknownEvent),
                            };

                            if let Ok(event) = parsed_event {
                                if sender.send(event).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Ok(async_sse::Event::Retry(_)) => {}
                        Err(e) => {
                            eprintln!("Streaming error: {:?}", e);
                            break;
                        }
                    }
                }
            });

            return Ok(RunResult::Stream(receiver));
        }

        response
            .json::<RunResponse>()
            .await
            .map(RunResult::Json)
            .map_err(Error::from)
    }

    pub(crate) fn check_status(status: StatusCode) -> Result<(), Error> {
        match status {
            StatusCode::TOO_MANY_REQUESTS => {
                Err(Error::LatitudeError(LatitudeErrorCodes::RateLimitError))
            }
            StatusCode::UNAUTHORIZED => {
                Err(Error::LatitudeError(LatitudeErrorCodes::UnauthorizedError))
            }
            StatusCode::FORBIDDEN => Err(Error::LatitudeError(LatitudeErrorCodes::ForbiddenError)),
            StatusCode::BAD_REQUEST => {
                Err(Error::LatitudeError(LatitudeErrorCodes::BadRequestError))
            }
            StatusCode::NOT_FOUND => Err(Error::LatitudeError(LatitudeErrorCodes::NotFoundError)),
            StatusCode::CONFLICT => Err(Error::LatitudeError(LatitudeErrorCodes::ConflictError)),
            StatusCode::UNPROCESSABLE_ENTITY => Err(Error::LatitudeError(
                LatitudeErrorCodes::UnprocessableEntityError,
            )),
            _ => Ok(()),
        }
    }
}

/// Builder for configuring and creating a `Client` instance.
///
/// The `ClientBuilder` provides a fluent interface for setting optional parameters,
/// allowing customization of `project_id`, `version_id`, and `base_url`. Once all
/// desired parameters are set, call `build` to create a `Client` instance.
pub struct ClientBuilder {
    api_key: String,
    project_id: Option<u64>,
    version_id: Option<String>,
    base_url: String,
}

impl ClientBuilder {
    /// Sets the `project_id` for the `Client`.
    ///
    /// This `project_id` is used as the default project for API requests.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The ID of the project.
    ///
    /// # Example
    ///
    /// ```
    /// let client_builder = Client::builder("your_api_key".into())
    ///     .project_id(123);
    /// ```
    pub fn project_id(mut self, project_id: u64) -> Self {
        self.project_id = Some(project_id);
        self
    }

    /// Sets the `version_id` for the `Client`.
    ///
    /// This `version_id` represents the version of the project or document and
    /// will be used in API requests.
    ///
    /// # Arguments
    ///
    /// * `version_id` - The UUID of the version.
    ///
    /// # Example
    ///
    /// ```
    /// let client_builder = Client::builder("your_api_key".into())
    ///     .version_id("version-uuid".to_string());
    /// ```
    pub fn version_id(mut self, version_id: String) -> Self {
        self.version_id = Some(version_id);
        self
    }

    /// Sets a custom `base_url` for the API endpoint.
    ///
    /// This is useful if the API endpoint changes or if using a mock server
    /// for testing purposes.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the API as a string.
    ///
    /// # Example
    ///
    /// ```
    /// let client_builder = Client::builder("your_api_key".into())
    ///     .base_url("https://custom.url/api".to_string());
    /// ```
    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    /// Builds and returns a new `Client` instance.
    ///
    /// After setting the necessary parameters, call `build` to create the `Client`.
    /// Once built, the `Client` can be used to interact with the Latitude API.
    ///
    /// # Example
    ///
    /// ```
    /// let client = Client::builder("your_api_key".into())
    ///     .project_id(123)
    ///     .version_id("version-uuid".to_string())
    ///     .base_url("https://custom.url/api".to_string())
    ///     .build();
    /// ```
    pub fn build(self) -> Client {
        Client::new(
            self.api_key,
            self.project_id,
            self.version_id,
            Some(self.base_url),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use document::{ChainStep, Config, LatitudeEventType, Options, Role};
    use document::{ProviderEventType, TextDelta};
    use httpmock::Method::POST;
    use httpmock::Mock;
    use httpmock::MockServer;
    use serde_json::json;
    use uuid::Uuid;

    // Helper function to setup a client for testing
    fn setup_client(
        api_key: &str,
        project_id: Option<u64>,
        version_id: Option<&str>,
        base_url: Option<&str>,
    ) -> Client {
        let mut client_builder = Client::builder(api_key.into());
        if let Some(pid) = project_id {
            client_builder = client_builder.project_id(pid);
        }
        if let Some(vid) = version_id {
            client_builder = client_builder.version_id(vid.to_string());
        }
        if let Some(base_url) = base_url {
            client_builder = client_builder.base_url(base_url.to_string());
        }

        client_builder.build()
    }

    fn check_standard_result(result: Result<RunResult, Error>) {
        match result {
            Ok(RunResult::Json(response)) => {
                assert_eq!(response.uuid, "123e4567-e89b-12d3-a456-426614174000");
                assert_eq!(response.response.text, "Test response");
                assert_eq!(response.response.usage.prompt_tokens, Some(10));
                assert_eq!(response.response.usage.completion_tokens, Some(20));
                assert_eq!(response.response.usage.total_tokens, Some(30));
            }
            Ok(other) => {
                panic!(
                    "Expected JSON response but received a different result: {:?}",
                    other
                );
            }
            Err(e) => {
                panic!("Error occurred while running document: {:?}", e);
            }
        }
    }

    // Helper function to setup a Mock with a standard JSON response
    async fn setup_standard_mock(server: &MockServer) -> Mock<'_> {
        server.mock(|when, then| {
            when.method(POST)
                .path("/projects/12345/versions/live/documents/run")
                .header("authorization", "Bearer test_api_key")
                .header("content-type", "application/json");
            then.status(200).json_body(json!({
                "uuid": "123e4567-e89b-12d3-a456-426614174000",
                "response": {
                    "text": "Test response",
                    "usage": {
                        "prompt_tokens": 10,
                        "completion_tokens": 20,
                        "total_tokens": 30
                    }
                }
            }));
        })
    }

    /// Helper function to set up a streaming mock event
    async fn setup_mock_with_stream_event<'a>(
        server: &'a MockServer,
        event_name: &'a str,
        event_data: &'a str,
    ) -> Mock<'a> {
        server.mock(|when, then| {
            when.method(POST)
                .path("/projects/12345/versions/live/documents/run")
                .header("authorization", "Bearer test_api_key")
                .header("content-type", "application/json");
            then.status(200)
                .body(format!("event: {}\ndata: {}\n\n", event_name, event_data));
        })
    }

    #[tokio::test]
    async fn test_client_creation_with_builder() {
        let client = setup_client(
            "test_api_key",
            Some(12345),
            Some("test-version"),
            Some("https://test.url/api"),
        );

        assert_eq!(client.api_key, "test_api_key");
        assert_eq!(client.project_id, Some(12345));
        assert_eq!(client.version_id, Some("test-version".to_string()));
        assert_eq!(client.base_url, "https://test.url/api");
    }

    #[tokio::test]
    async fn test_client_creation_with_default_base_url() {
        let client = setup_client("test_api_key", Some(12345), Some("test-version"), None);

        assert_eq!(client.api_key, "test_api_key");
        assert_eq!(client.project_id, Some(12345));
        assert_eq!(client.version_id, Some("test-version".to_string()));
        assert_eq!(client.base_url, "https://gateway.latitude.so/api/v2");
    }

    #[tokio::test]
    async fn test_client_creation_new_with_default_base_url() {
        let client = Client::new(
            "test_api_key".into(),
            Some(12345),
            Some("test-version".into()),
            None,
        );

        assert_eq!(client.api_key, "test_api_key");
        assert_eq!(client.project_id, Some(12345));
        assert_eq!(client.version_id, Some("test-version".to_string()));
        assert_eq!(client.base_url, "https://gateway.latitude.so/api/v2");
    }

    #[tokio::test]
    async fn test_run_document_json_response() {
        let server = MockServer::start_async().await;
        let mock = setup_standard_mock(&server).await;

        let client = setup_client(
            "test_api_key",
            Some(12345),
            Some("live"),
            Some(&server.base_url()),
        );

        let document = RunDocument::<()>::builder()
            .path("test-path".into())
            .build()
            .expect("Failed to build RunDocument");

        let result = client.run(document).await;
        check_standard_result(result);
        mock.assert();
    }

    #[tokio::test]
    async fn test_run_document_with_options() {
        let server = MockServer::start_async().await;
        let mock = setup_standard_mock(&server).await;

        let client = setup_client(
            "test_api_key",
            Some(12345),
            Some("live"),
            Some(&server.base_url()),
        );

        let options = Options::builder()
            .project_id(12345)
            .version_id("live".into())
            .build();

        #[derive(Serialize, Debug, Default)]
        struct Params {
            user_message: String,
        }

        let parameters = Params {
            user_message: "Hello, world!".to_owned(),
        };

        let document = RunDocument::builder()
            .path("test-path".into())
            .options(options)
            .parameters(parameters)
            .build()
            .expect("Failed to build RunDocument");

        let result = client.run(document).await;
        check_standard_result(result);
        mock.assert();
    }

    #[tokio::test]
    async fn test_run_document_no_project_id() {
        let server = MockServer::start_async().await;
        let client = setup_client("test_api_key", None, Some("live"), Some(&server.base_url()));

        let document = RunDocument::<()>::builder()
            .path("test-path".into())
            .build()
            .expect("Failed to build RunDocument");

        let result = client.run(document).await;

        // Expect an error due to missing project ID
        assert!(
            matches!(result, Err(Error::ConfigError(msg)) if msg.contains("Project ID is required"))
        );
    }

    #[tokio::test]
    async fn test_run_document_no_version_id() {
        let server = MockServer::start_async().await;
        let mock = setup_standard_mock(&server).await;

        let client = setup_client("test_api_key", Some(12345), None, Some(&server.base_url()));

        let document = RunDocument::<()>::builder()
            .path("test-path".into())
            .build()
            .expect("Failed to build RunDocument");

        let result = client.run(document).await;
        check_standard_result(result);
        mock.assert();
    }

    #[tokio::test]
    async fn test_latitude_event_stream() {
        // Tests `latitude-event` streaming response
        let server = MockServer::start_async().await;
        let mock = setup_mock_with_stream_event(
            &server,
            "latitude-event",
            r#"{"type":"chain-step","isLastStep":true,"config":{"provider":"Latitude","model":"gpt-4o-mini"},"messages":[{"role":"system","content":"Generate a joke"}],"uuid":"58e86f35-293c-4f12-a412-9915cb385850"}"#
        ).await;

        let client = Client::builder("test_api_key".to_string())
            .project_id(12345)
            .version_id("live".to_string())
            .base_url(server.base_url())
            .build();

        let document = RunDocument::<()>::builder()
            .path("test-path".to_string())
            .stream(true)
            .build()
            .expect("Failed to build RunDocument");

        let result = client
            .run(document)
            .await
            .expect("Expected a stream response");

        if let RunResult::Stream(mut stream) = result {
            if let Some(event) = stream.recv().await {
                match event {
                    Event::LatitudeEvent(data) => {
                        assert_eq!(
                            data.event_type,
                            LatitudeEventType::ChainStep(ChainStep {
                                is_last_step: true,
                                config: Config {
                                    provider: "Latitude".to_string(),
                                    model: "gpt-4o-mini".to_string()
                                },
                                messages: vec![Message {
                                    role: Role::System,
                                    tool_calls: None,
                                    content: "Generate a joke".to_string()
                                }],
                                uuid: Uuid::from_str("58e86f35-293c-4f12-a412-9915cb385850")
                                    .expect("Failed to parse UUID"),
                            })
                        );
                    }
                    _ => panic!("Expected LatitudeEvent"),
                }
            } else {
                panic!("Expected an event in the stream");
            }
        } else {
            panic!("Expected stream response");
        }

        mock.assert();
    }

    #[tokio::test]
    async fn test_provider_event_stream() {
        // Tests `provider-event` streaming response
        let server = MockServer::start_async().await;
        let mock = setup_mock_with_stream_event(
            &server,
            "provider-event",
            r#"{"type":"text-delta","textDelta": "running"}"#,
        )
        .await;

        let client = Client::builder("test_api_key".to_string())
            .project_id(12345)
            .version_id("live".to_string())
            .base_url(server.base_url())
            .build();

        let document = RunDocument::<()>::builder()
            .path("test-path".to_string())
            .stream(true)
            .build()
            .expect("Failed to build RunDocument");

        let result = client
            .run(document)
            .await
            .expect("Expected a stream response");

        if let RunResult::Stream(mut stream) = result {
            if let Some(event) = stream.recv().await {
                match event {
                    Event::ProviderEvent(data) => {
                        assert_eq!(
                            data.event_type,
                            ProviderEventType::TextDelta(TextDelta {
                                text_delta: "running".to_string(),
                            })
                        );
                    }
                    _ => panic!("Expected ProviderEvent"),
                }
            } else {
                panic!("Expected an event in the stream");
            }
        } else {
            panic!("Expected stream response");
        }

        mock.assert();
    }

    #[tokio::test]
    async fn test_unknown_event_stream() {
        // Tests `unknown-event` streaming response
        let server = MockServer::start_async().await;
        let mock = setup_mock_with_stream_event(
            &server,
            "unknown-event",
            r#"{"type":"text-delta","textDelta": "running"}"#,
        )
        .await;

        let client = Client::builder("test_api_key".to_string())
            .project_id(12345)
            .version_id("live".to_string())
            .base_url(server.base_url())
            .build();

        let document = RunDocument::<()>::builder()
            .path("test-path".to_string())
            .stream(true)
            .build()
            .expect("Failed to build RunDocument");

        let result = client
            .run(document)
            .await
            .expect("Expected a stream response");

        if let RunResult::Stream(mut stream) = result {
            if let Some(event) = stream.recv().await {
                match event {
                    Event::UnknownEvent => {
                        assert!(true)
                    }
                    _ => panic!("Expected UnknownEvent"),
                }
            } else {
                panic!("Expected an event in the stream");
            }
        } else {
            panic!("Expected stream response");
        }

        mock.assert();
    }

    #[tokio::test]
    async fn test_streaming_error_handling() {
        // This test checks if the function handles a streaming error properly
        let server = MockServer::start_async().await;

        // Mock server response with an invalid streaming format to trigger an error
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/projects/12345/versions/live/documents/run")
                .header("authorization", "Bearer test_api_key")
                .header("content-type", "application/json");
            then.status(200).body("data: invalid-format\n\n");
        });

        let client = Client::builder("test_api_key".to_string())
            .project_id(12345)
            .version_id("live".to_string())
            .base_url(server.base_url())
            .build();

        let document = RunDocument::<()>::builder()
            .path("test-path".to_string())
            .stream(true)
            .build()
            .expect("Failed to build RunDocument");

        let result = client.run(document).await;

        if let Ok(RunResult::Stream(mut stream)) = result {
            if let Some(event) = stream.recv().await {
                match event {
                    Event::UnknownEvent => {
                        println!("Received UnknownEvent as expected.");
                    }
                    other_event => {
                        panic!(
                            "Expected UnknownEvent but got a different event type: {:?}",
                            other_event
                        );
                    }
                }
            } else {
                panic!("Expected an event in the stream, but none was received.");
            }
        } else {
            panic!("Expected stream response, but got an error: {:?}", result);
        }

        mock.assert();
    }

    #[test]
    fn test_check_status() {
        // Test TOO_MANY_REQUESTS status
        let result = Client::check_status(StatusCode::TOO_MANY_REQUESTS);
        assert!(matches!(
            result,
            Err(Error::LatitudeError(LatitudeErrorCodes::RateLimitError))
        ));

        // Test UNAUTHORIZED status
        let result = Client::check_status(StatusCode::UNAUTHORIZED);
        assert!(matches!(
            result,
            Err(Error::LatitudeError(LatitudeErrorCodes::UnauthorizedError))
        ));

        // Test FORBIDDEN status
        let result = Client::check_status(StatusCode::FORBIDDEN);
        assert!(matches!(
            result,
            Err(Error::LatitudeError(LatitudeErrorCodes::ForbiddenError))
        ));

        // Test BAD_REQUEST status
        let result = Client::check_status(StatusCode::BAD_REQUEST);
        assert!(matches!(
            result,
            Err(Error::LatitudeError(LatitudeErrorCodes::BadRequestError))
        ));

        // Test NOT_FOUND status
        let result = Client::check_status(StatusCode::NOT_FOUND);
        assert!(matches!(
            result,
            Err(Error::LatitudeError(LatitudeErrorCodes::NotFoundError))
        ));

        // Test CONFLICT status
        let result = Client::check_status(StatusCode::CONFLICT);
        assert!(matches!(
            result,
            Err(Error::LatitudeError(LatitudeErrorCodes::ConflictError))
        ));

        // Test UNPROCESSABLE_ENTITY status
        let result = Client::check_status(StatusCode::UNPROCESSABLE_ENTITY);
        assert!(matches!(
            result,
            Err(Error::LatitudeError(
                LatitudeErrorCodes::UnprocessableEntityError
            ))
        ));

        // Test OK status (expecting no error)
        let result = Client::check_status(StatusCode::OK);
        assert!(matches!(result, Ok(())));

        // Test another success status (e.g., CREATED)
        let result = Client::check_status(StatusCode::CREATED);
        assert!(matches!(result, Ok(())));
    }
}
