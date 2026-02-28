use std::collections::HashMap;

use serde_json::Value;

use crate::error::HookFreightError;
use crate::http_client::HFHttpClient;
use crate::pagination::{clamp, MAX_APPS_LIMIT};

#[derive(Clone)]
pub struct AppsService {
    pub(crate) http: HFHttpClient,
}

impl AppsService {
    pub(crate) fn new(http: HFHttpClient) -> Self {
        Self { http }
    }

    pub async fn list(
        &self,
        params: Option<&HashMap<String, Value>>,
    ) -> Result<Value, HookFreightError> {
        let query = clamp(params, MAX_APPS_LIMIT);
        self.http.get("/apps", query.as_ref()).await.map(unwrap_data)
    }

    pub async fn create(&self, params: &Value) -> Result<Value, HookFreightError> {
        self.http.post("/apps", Some(params)).await.map(unwrap_data)
    }

    pub async fn get(&self, app_id: &str) -> Result<Value, HookFreightError> {
        self.http
            .get(&format!("/apps/{app_id}"), None)
            .await
            .map(unwrap_data)
    }

    pub async fn update(&self, app_id: &str, params: &Value) -> Result<Value, HookFreightError> {
        self.http
            .put(&format!("/apps/{app_id}"), Some(params))
            .await
            .map(unwrap_data)
    }

    pub async fn delete(&self, app_id: &str) -> Result<Value, HookFreightError> {
        self.http
            .delete(&format!("/apps/{app_id}"))
            .await
            .map(unwrap_data)
    }
}

fn unwrap_data(payload: Value) -> Value {
    payload.get("data").cloned().unwrap_or(payload)
}
