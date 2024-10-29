// Copyright 2024 NewtTheWolf
//
// Licensed under the MIT License <LICENSE-MIT or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

//! Latitude API Client
//!
//! This crate provides a client for interacting with the Latitude API, allowing users to run
//! prompts and handle AI-powered conversations seamlessly.
//!
//! # Overview
//!
//! The Latitude API enables users to interact with AI models through a simple and efficient
//! HTTP-based interface. This crate wraps the API and exposes easy-to-use methods for common tasks.
//!
//! # Features
//!
//! - **Run Documents**: Execute specific documents (prompts) with custom parameters.
//! - **Stream Responses**: Optionally receive responses as a real-time stream.
//! - **Manage Projects**: Easily set and manage project and version IDs for different interactions.
//!
//! # Installation
//!
//! Add this crate to your `Cargo.toml` file:
//!
//! ```sh
//! cargo add latitude
//! ```
//!
//! Make sure to include the `tokio` runtime, as this crate is asynchronous:
//!
//! ```sh
//! cargo add tokio --features full
//! ```
//!
//! # Usage
//!
//! To use the Latitude API client, create an instance of `Client` with your API key.
//!
//! ```rust
//! use latitude::Client;
//!
//! let client = Client::new("your_api_key".into());
//! client.set_project_id(123);
//! client.set_version_uuid("version-uuid".parse().unwrap());
//! ```
//!
//! # Examples
//!
//! ## Running a Document
//!
//! ```rust
//! use latitude::Client;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct MyParameters {
//!     key1: String,
//!     key2: i32,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("your_api_key".into());
//!     client.set_project_id(12345);
//!     let parameters = MyParameters {
//!         key1: "value1".into(),
//!         key2: 42,
//!     };
//!
//!     match client.run_document("path/to/document", parameters, false).await {
//!         Ok(response) => println!("Document run successfully: {:?}", response),
//!         Err(e) => eprintln!("Error running document: {:?}", e),
//!     }
//! }
//! ```
//!
//! ## Streaming a Document
//!
//! ```rust
//! use latitude::Client;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct MyParameters {
//!     query: String,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("your_api_key".into());
//!     client.set_project_id(12345);
//!     let parameters = MyParameters {
//!         query: "What is the weather like?".into(),
//!     };
//!
//!     match client.run_document("path/to/document", parameters, true).await {
//!         Ok(response) => println!("Streaming response received: {:?}", response),
//!         Err(e) => eprintln!("Error streaming document: {:?}", e),
//!     }
//! }
//! ```

use models::document::{self, RunDocument, RunResult};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as ReqwestClient,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio_stream::StreamExt;
use uuid::Uuid;

pub mod models;

static BASE_URL: &str = "https://gateway.latitude.so/api/v2";
static APP_USER_AGENT: &str = env!("CARGO_PKG_NAME");

/// Client for interacting with the Latitude API.
///
/// The `Client` struct provides methods for running documents and continuing conversations with AI models using Latitude's HTTP API.
///
/// # Examples
///
/// ```
/// use latitude::Client;
///
/// let client = Client::new("your_api_key".into());
/// client.set_project_id(123);
/// client.set_version_id("version-uuid".parse().unwrap());
/// ```
#[derive(Clone)]
pub struct Client {
    /// The API key for authentication.
    pub api_key: String,
    /// The default project ID used in requests.
    pub project_id: Option<u64>,
    /// The default version UUID used in requests.
    pub version_id: Option<String>,
    /// Internal HTTP client for making requests.
    pub(crate) client: ReqwestClient,
    /// The base URL for API requests.
    pub(crate) base_url: String,
}

impl Client {
    /// Creates a new Client with the provided API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for authenticating with the Latitude API.
    ///
    /// # Returns
    ///
    /// * `Client` - A new instance of the Latitude client.
    ///
    /// # Examples
    ///
    /// ```
    /// let client = Client::new("your_api_key".into());
    /// ```
    pub fn new(api_key: String) -> Self {
        let mut headers = HeaderMap::new();
        let api_key_value =
            HeaderValue::from_str(&format!("Bearer {}", api_key)).expect("Invalid API key");
        headers.insert("Authorization", api_key_value);

        let client = ReqwestClient::builder()
            .default_headers(headers)
            .https_only(true)
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            project_id: None,
            version_id: None,
            client,
            base_url: BASE_URL.into(),
        }
    }

    /// Creates a new Client using the `LATITUDE_API_KEY` environment variable.
    ///
    /// # Panics
    ///
    /// Panics if the `LATITUDE_API_KEY` environment variable is not set.
    ///
    /// # Returns
    ///
    /// * `Client` - A new instance of the Latitude client.
    ///
    /// # Examples
    ///
    /// ```
    /// let client = Client::from_env();
    /// ```
    pub fn from_env() -> Self {
        let api_key = std::env::var("LATITUDE_API_KEY")
            .expect("LATITUDE_API_KEY environment variable is not set");
        Self::new(api_key)
    }

    /// Sets the base URL for the API requests.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The new base URL to use for requests.
    ///
    /// # Examples
    ///
    /// ```
    /// client.set_base_url("https://custom-api.latitude.so/api/v2");
    /// ```
    pub fn set_base_url(&mut self, base_url: &str) {
        self.base_url = base_url.into();
    }

    /// Sets the default project ID.
    ///
    /// # Arguments
    ///
    /// * `project_id` - The project ID to use for subsequent API requests.
    ///
    /// # Examples
    ///
    /// ```
    /// client.set_project_id(123);
    /// ```
    pub fn set_project_id(&mut self, project_id: u64) {
        self.project_id = Some(project_id);
    }

    /// Sets the default version UUID.
    ///
    /// # Arguments
    ///
    /// * `version_id` - The version UUID to use for subsequent API requests.
    ///
    /// # Examples
    ///
    /// ```
    /// client.set_version_id("version-uuid".parse().unwrap());
    /// ```
    pub fn set_version_id(&mut self, version_id: String) {
        self.version_id = Some(version_id);
    }

    /// Runs a document with the specified path and user-defined parameters.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the document.
    /// * `parameters` - A user-defined struct containing the parameters for the document. This struct must implement `Serialize`.
    /// * `stream` - Indicates if the response should be streamed.
    ///
    /// # Returns
    ///
    /// * `Result<RunResponse, Box<dyn Error>>` - The response from the Latitude API, or an error if the request fails.
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(Serialize)]
    /// struct MyParameters {
    ///     key1: String,
    ///     key2: i32,
    /// }
    ///
    /// let parameters = MyParameters {
    ///     key1: "value1".into(),
    ///     key2: 42,
    /// };
    /// let response = client.run("path/to/document", parameters, false).await;
    /// ```
    pub async fn run<T>(&self, document: RunDocument<T>) -> Result<RunResult, Box<dyn Error>>
    where
        T: Serialize,
    {
        let url = format!(
            "{}/projects/{}/versions/{}/documents/run",
            self.base_url,
            self.project_id.expect("Project ID is not set"),
            self.version_id
                .clone()
                .unwrap_or_else(|| String::from("live"))
        );

        let response = self.client.post(&url).json(&document).send().await?;

        if document.stream {
            // Returns the stream in the enum variant
            let stream = response.bytes_stream();
            Ok(RunResult::Stream(Box::new(stream)))
        } else {
            // Parses JSON response into `RunResponse`
            let result = response.json::<RunResponse>().await?;
            Ok(RunResult::Json(result))
        }
    }
}

/// Request structure for running a document.
#[derive(Debug, Serialize)]
struct RunDocumentRequest<T> {
    /// The path to the document.
    pub path: String,
    /// The parameters for the document.
    pub parameters: T,
    /// Whether the response should be streamed.
    pub stream: bool,
}

/// Response structure for a document run.
#[derive(Debug, Deserialize)]
pub struct RunResponse {
    pub uuid: Option<String>,
    pub response: Option<ResponseDetail>,
}

impl Default for RunResponse {
    fn default() -> Self {
        Self {
            uuid: None,
            response: None,
        }
    }
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
