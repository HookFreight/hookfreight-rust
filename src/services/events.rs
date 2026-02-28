use std::collections::HashMap;

use serde_json::Value;

use crate::error::HookFreightError;
use crate::http_client::HFHttpClient;
use crate::pagination::{clamp, MAX_EVENTS_LIMIT};

#[derive(Clone)]
pub struct EventsService {
    pub(crate) http: HFHttpClient,
}

impl EventsService {
    pub(crate) fn new(http: HFHttpClient) -> Self {
        Self { http }
    }

    pub async fn list(
        &self,
        params: Option<&HashMap<String, Value>>,
    ) -> Result<Value, HookFreightError> {
        let query = clamp(params, MAX_EVENTS_LIMIT);
        self.http
            .get("/events", query.as_ref())
            .await
            .map(unwrap_data)
    }

    pub async fn get(&self, event_id: &str) -> Result<Value, HookFreightError> {
        self.http
            .get(&format!("/events/{event_id}"), None)
            .await
            .map(unwrap_data)
    }

    pub async fn list_by_endpoint(
        &self,
        endpoint_id: &str,
        params: Option<&HashMap<String, Value>>,
    ) -> Result<Value, HookFreightError> {
        let query = clamp(params, MAX_EVENTS_LIMIT);
        self.http
            .get(&format!("/endpoints/{endpoint_id}/events"), query.as_ref())
            .await
            .map(unwrap_data)
    }

    pub async fn replay(&self, event_id: &str) -> Result<(), HookFreightError> {
        self.http
            .post(&format!("/events/{event_id}/replay"), None)
            .await
            .map(|_| ())
    }
}

fn unwrap_data(payload: Value) -> Value {
    payload.get("data").cloned().unwrap_or(payload)
}
