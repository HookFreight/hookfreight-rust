use std::collections::HashMap;

use serde_json::Value;

use crate::error::HookFreightError;
use crate::http_client::HFHttpClient;
use crate::pagination::{clamp, MAX_ENDPOINTS_LIMIT};

#[derive(Clone)]
pub struct EndpointsService {
    pub(crate) http: HFHttpClient,
}

impl EndpointsService {
    pub(crate) fn new(http: HFHttpClient) -> Self {
        Self { http }
    }

    pub async fn list(
        &self,
        app_id: &str,
        params: Option<&HashMap<String, Value>>,
    ) -> Result<Value, HookFreightError> {
        let query = clamp(params, MAX_ENDPOINTS_LIMIT);
        self.http
            .get(&format!("/apps/{app_id}/endpoints"), query.as_ref())
            .await
            .map(unwrap_data)
    }

    pub async fn create(&self, params: &Value) -> Result<Value, HookFreightError> {
        self.http
            .post("/endpoints", Some(params))
            .await
            .map(unwrap_data)
    }

    pub async fn get(&self, endpoint_id: &str) -> Result<Value, HookFreightError> {
        self.http
            .get(&format!("/endpoints/{endpoint_id}"), None)
            .await
            .map(unwrap_data)
    }

    pub async fn update(
        &self,
        endpoint_id: &str,
        params: &Value,
    ) -> Result<Value, HookFreightError> {
        self.http
            .put(&format!("/endpoints/{endpoint_id}"), Some(params))
            .await
            .map(unwrap_data)
    }

    pub async fn delete(&self, endpoint_id: &str) -> Result<Value, HookFreightError> {
        self.http
            .delete(&format!("/endpoints/{endpoint_id}"))
            .await
            .map(unwrap_data)
    }
}

fn unwrap_data(payload: Value) -> Value {
    payload.get("data").cloned().unwrap_or(payload)
}
