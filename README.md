# Latitude API Client

This crate provides a client for interacting with the Latitude API, allowing users to execute documents (prompts) and handle real-time AI-powered conversations through a simple HTTP-based interface.

## Features

- **Document Execution**: Run specific documents (prompts) with custom parameters.
- **Stream Responses**: Optionally receive responses as a real-time data stream.
- **Simple API Integration**: API key authentication and project/version management.

## Installation

Add this crate to your `Cargo.toml` file:

```
cargo add latitude-sdk
```

## Usage

To use the Latitude API client, create an instance of `Client` with your API key, set the project ID, and run a document.

```rust
use latitude_sdk::Client;

let client = Client::builder("your_api_key".into())
    .project_id(123)
    .version_id("version-uuid".to_string())
    .base_url("https://custom.url/api".to_string())
    .build();
```

## Example

Here is a simple example of how to use the client to run a document:

```rust
use latitude_sdk::{Client, Document};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("your_api_key".into())
        .project_id(123)
        .version_id("version-uuid".to_string())
        .base_url("https://custom.url/api".to_string())
        .build();

    let document = Document::new("document_id".to_string());
    let response = client.run_document(document).await?;

    println!("Response: {:?}", response);

    Ok(())
}
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
