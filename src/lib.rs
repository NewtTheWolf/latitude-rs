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
use models::document::{self, ChunkData, Message, RunDocument};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client as ReqwestClient,
};
use serde::Serialize;
use std::error::Error;
use tokio::io::BufReader;
use tokio_stream::StreamExt;
use tokio_util::{
    compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt},
    io::StreamReader,
};

pub mod models;

static BASE_URL: &str = "https://gateway.latitude.so/api/v2";
static APP_USER_AGENT: &str = env!("CARGO_PKG_NAME");

/// Enum to represent the response type from the `run` method.
pub enum RunResult {
    /// JSON response when `stream` is set to `false`.
    Json(document::RunResponse),
    /// Streaming response when `stream` is set to `true`.    
    Stream(),
}

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
    /// Creates a new `Client` with the provided API key.
    ///
    /// # Arguments
    /// * `api_key` - The API key for authenticating with the Latitude API.
    ///
    /// # Examples
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

    /// Runs a document with the specified path and user-defined parameters, with optional streaming.
    ///
    /// # Arguments
    /// * `document` - The `RunDocument` struct containing the path, parameters, and streaming option.
    ///
    /// # Returns
    /// * `RunResult` - The response from the Latitude API, either as JSON or a streaming response.
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
    ///         Ok(RunResult::Stream(mut stream)) => {
    ///             while let Some(chunk) = stream.next().await {
    ///                 match chunk {
    ///                     Ok(bytes) => println!("Chunk: {:?}", bytes),
    ///                     Err(e) => eprintln!("Stream error: {:?}", e),
    ///                 }
    ///             }
    ///         },
    ///         Err(e) => eprintln!("Error: {:?}", e),
    ///     }
    /// }
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
            let stream = response.bytes_stream();

            let reader = StreamReader::new(stream.map(|result| {
                result.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            }));

            let buffered_reader = BufReader::new(reader.compat().into_inner());

            let mut decoder = decode(buffered_reader.compat());

            while let Some(event) = decoder.next().await {
                match event {
                    Ok(async_sse::Event::Retry(duration)) => {
                        println!("Retry: {:?}", duration);
                    }
                    Ok(async_sse::Event::Message(message)) => {
                        //println!("Message: {:?}", message);

                        let data = message.into_bytes();
                        let message: ChunkData = serde_json::from_slice(&data).unwrap();

                        println!("Message: {:?}", message);
                    }
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        break;
                    }
                }
            }

            Ok(RunResult::Stream())
        } else {
            let result = response.json::<document::RunResponse>().await?;
            Ok(RunResult::Json(result))
        }
    }
}
