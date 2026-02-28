use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: u16,
    pub body: Value,
    pub server_message: Option<String>,
}

impl ApiError {
    pub fn new(status: u16, body: Value) -> Self {
        let server_message = body
            .as_object()
            .and_then(|obj| obj.get("message"))
            .and_then(|v| v.as_str())
            .map(ToString::to_string);

        Self {
            status,
            body,
            server_message,
        }
    }

    fn message(&self) -> String {
        self.server_message
            .clone()
            .unwrap_or_else(|| format!("API request failed with status {}", self.status))
    }
}

#[derive(Debug, Error)]
pub enum HookfreightError {
    #[error("{0}")]
    Validation(ApiError),
    #[error("{0}")]
    Authentication(ApiError),
    #[error("{0}")]
    Permission(ApiError),
    #[error("{0}")]
    NotFound(ApiError),
    #[error("{0}")]
    Api(ApiError),
    #[error("{0}")]
    Connection(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}
