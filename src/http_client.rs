use std::collections::HashMap;

use reqwest::{header, Method};
use serde_json::Value;

use crate::config::HookfreightConfig;
use crate::error::{ApiError, HookfreightError};

const SDK_VERSION: &str = "0.1.0";

#[derive(Clone)]
pub struct HFHttpClient {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
}

impl HFHttpClient {
    pub fn new(config: HookfreightConfig) -> Result<Self, HookfreightError> {
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| HookfreightError::Connection(e.to_string()))?;

        Ok(Self {
            client,
            base_url: config.base_url.trim_end_matches('/').to_string(),
            api_key: config.api_key,
        })
    }

    pub async fn get(
        &self,
        path: &str,
        query: Option<&HashMap<String, String>>,
    ) -> Result<Value, HookfreightError> {
        self.request(Method::GET, path, query, None).await
    }

    pub async fn post(&self, path: &str, body: Option<&Value>) -> Result<Value, HookfreightError> {
        self.request(Method::POST, path, None, body).await
    }

    pub async fn put(&self, path: &str, body: Option<&Value>) -> Result<Value, HookfreightError> {
        self.request(Method::PUT, path, None, body).await
    }

    pub async fn delete(&self, path: &str) -> Result<Value, HookfreightError> {
        self.request(Method::DELETE, path, None, None).await
    }

    async fn request(
        &self,
        method: Method,
        path: &str,
        query: Option<&HashMap<String, String>>,
        body: Option<&Value>,
    ) -> Result<Value, HookfreightError> {
        let normalized = format!("/{path}").replace("//", "/");
        let url = format!("{}{}", self.base_url, normalized);

        let mut request = self
            .client
            .request(method, url)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::USER_AGENT, format!("hookfreight-rust/{SDK_VERSION}"));

        if let Some(api_key) = &self.api_key {
            if !api_key.is_empty() {
                request = request.header(header::AUTHORIZATION, format!("Bearer {api_key}"));
            }
        }

        if let Some(q) = query {
            request = request.query(q);
        }

        if let Some(payload) = body {
            request = request.json(payload);
        }

        let response = request
            .send()
            .await
            .map_err(|e| HookfreightError::Connection(e.to_string()))?;

        let status = response.status().as_u16();
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        let text = response
            .text()
            .await
            .map_err(|e| HookfreightError::Connection(e.to_string()))?;

        let body_value = decode_body(&text, &content_type);

        if !(200..300).contains(&status) {
            return Err(map_error(status, body_value));
        }

        Ok(body_value)
    }
}

fn decode_body(raw: &str, content_type: &str) -> Value {
    if raw.is_empty() {
        return Value::Object(Default::default());
    }

    if content_type.contains("application/json") {
        serde_json::from_str(raw).unwrap_or_else(|_| Value::String(raw.to_string()))
    } else {
        Value::String(raw.to_string())
    }
}

fn map_error(status: u16, body: Value) -> HookfreightError {
    let api_error = ApiError::new(status, body);

    match status {
        400 => HookfreightError::Validation(api_error),
        401 => HookfreightError::Authentication(api_error),
        403 => HookfreightError::Permission(api_error),
        404 => HookfreightError::NotFound(api_error),
        _ => HookfreightError::Api(api_error),
    }
}
