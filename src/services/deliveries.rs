use std::collections::HashMap;

use serde_json::Value;

use crate::error::HookFreightError;
use crate::http_client::HFHttpClient;
use crate::pagination::{clamp, MAX_DELIVERIES_LIMIT};

#[derive(Clone)]
pub struct DeliveriesService {
    pub(crate) http: HFHttpClient,
}

impl DeliveriesService {
    pub(crate) fn new(http: HFHttpClient) -> Self {
        Self { http }
    }

    pub async fn list(
        &self,
        params: Option<&HashMap<String, Value>>,
    ) -> Result<Value, HookFreightError> {
        let query = clamp(params, MAX_DELIVERIES_LIMIT);
        self.http
            .get("/deliveries", query.as_ref())
            .await
            .map(unwrap_data)
    }

    pub async fn list_by_event(
        &self,
        event_id: &str,
        params: Option<&HashMap<String, Value>>,
    ) -> Result<Value, HookFreightError> {
        let query = clamp(params, MAX_DELIVERIES_LIMIT);
        self.http
            .get(&format!("/events/{event_id}/deliveries"), query.as_ref())
            .await
            .map(unwrap_data)
    }

    pub async fn retry(&self, delivery_id: &str) -> Result<(), HookFreightError> {
        self.http
            .post(&format!("/deliveries/{delivery_id}/retry"), None)
            .await
            .map(|_| ())
    }

    pub async fn queue_stats(&self) -> Result<Value, HookFreightError> {
        self.http
            .get("/deliveries/queue/stats", None)
            .await
            .map(unwrap_data)
    }
}

fn unwrap_data(payload: Value) -> Value {
    payload.get("data").cloned().unwrap_or(payload)
}
