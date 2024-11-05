use reqwest;
use serde::Serialize;
use thiserror::Error;

/// The main error type for the Latitude API client, encapsulating all possible error scenarios.
#[derive(Debug, Error)]
pub enum Error {
    /// Error originating from the Latitude API with specific error codes.
    #[error("Latitude API error: {0:?}")]
    LatitudeError(LatitudeErrorCodes),

    /// Error encountered during document execution (Run) with specific error codes.
    #[error("Run error: {0:?}")]
    RunError(RunErrorCodes),

    /// General API error, indicating issues unrelated to document execution or Latitude-specific codes.
    #[error("API error: {0:?}")]
    ApiError(ApiErrorCodes),

    /// Specific error related to chain compilation with additional details.
    #[error("Chain compile error with details: {0:?}")]
    ChainCompileError(RunErrorDetails),

    /// Error referencing a database entity, containing entity UUID and type information.
    #[error("Database reference error: {0:?}")]
    DatabaseError(DbErrorRef),

    /// Error indicating an unexpected response format from the API.
    #[error("Unexpected response format: {0}")]
    ResponseFormatError(String),

    /// HTTP request-related error, mapped directly from `reqwest::Error`.
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Serialization or deserialization error, mapped directly from `serde_json::Error`.
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Configuration error, such as missing or invalid configuration values.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// A catch-all error for miscellaneous cases.
    #[error("Other error: {0}")]
    Other(String),
}

/// Latitude API-specific error codes.
#[derive(Debug, Serialize)]
pub enum LatitudeErrorCodes {
    /// An unexpected error occurred.
    UnexpectedError,
    /// The request was rate-limited.
    RateLimitError,
    /// The request was unauthorized.
    UnauthorizedError,
    /// The request was forbidden.
    ForbiddenError,
    /// The request was malformed or incorrect.
    BadRequestError,
    /// The requested resource was not found.
    NotFoundError,
    /// A conflict occurred during the request.
    ConflictError,
    /// The request could not be processed.
    UnprocessableEntityError,
}

/// Error codes related to document execution (Run) within the Latitude API.
#[derive(Debug, Serialize)]
pub enum RunErrorCodes {
    /// An unknown error occurred during document execution.
    Unknown,
    /// The provider's quota was exceeded.
    DefaultProviderExceededQuota,
    /// The requested model is invalid for the provider.
    DefaultProviderInvalidModel,
    /// The document configuration is incorrect.
    DocumentConfigError,
    /// No provider was specified for the document run.
    MissingProvider,
    /// An error occurred while compiling a chain of actions.
    ChainCompileError,
    /// An error occurred while running an AI model.
    AIRunError,
    /// The provider returned an unsupported response type.
    UnsupportedProviderResponseType,
    /// The AI provider configuration is incorrect.
    AIProviderConfigError,
    /// Missing provider log for evaluation run.
    EvaluationRunMissingProviderLog,
    /// Missing workspace for evaluation run.
    EvaluationRunMissingWorkspace,
    /// The result type is unsupported for evaluation run.
    EvaluationRunUnsupportedResultType,
    /// The response JSON format was invalid for evaluation run.
    EvaluationRunResponseJsonFormat,
}

/// General API error codes used by the Latitude API.
#[derive(Debug, Serialize)]
pub enum ApiErrorCodes {
    /// An HTTP-related exception occurred.
    HTTPException,
    /// An internal server error occurred.
    InternalServerError,
}

/// Additional details for specific `RunErrorCodes`, such as compilation errors.
#[derive(Debug, Serialize)]
pub struct RunErrorDetails {
    /// The code associated with the compilation error.
    pub compile_code: String,
    /// Detailed message describing the error.
    pub message: String,
}

/// Reference details for errors that involve a database entity.
#[derive(Debug, Serialize)]
pub struct DbErrorRef {
    /// UUID of the entity involved in the error.
    pub entity_uuid: String,
    /// Type of entity involved in the error.
    pub entity_type: String,
}

/// General structure for handling API error responses in JSON format.
#[derive(Debug, Serialize)]
pub struct ApiErrorJsonResponse {
    /// Name of the error.
    pub name: String,
    /// Descriptive error message.
    pub message: String,
    /// Detailed information related to the error.
    pub details: serde_json::Value,
    /// The specific error code for the response.
    pub error_code: ApiResponseCode,
    /// Optional reference to a database entity involved in the error.
    pub db_error_ref: Option<DbErrorRef>,
}

/// Unified error code type that includes all possible error codes returned by the API.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ApiResponseCode {
    /// Error code for Latitude-specific issues.
    LatitudeError(LatitudeErrorCodes),
    /// Error code for document execution (Run) issues.
    RunError(RunErrorCodes),
    /// General API error code.
    ApiError(ApiErrorCodes),
}
