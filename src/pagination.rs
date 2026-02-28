use std::collections::HashMap;

use serde_json::Value;

pub const MAX_APPS_LIMIT: i64 = 1000;
pub const MAX_ENDPOINTS_LIMIT: i64 = 1000;
pub const MAX_EVENTS_LIMIT: i64 = 50;
pub const MAX_DELIVERIES_LIMIT: i64 = 1000;

pub fn clamp(
    params: Option<&HashMap<String, Value>>,
    max_limit: i64,
) -> Option<HashMap<String, String>> {
    let source = params?;
    let mut out = HashMap::new();

    for (k, v) in source {
        if v.is_null() {
            continue;
        }

        if k == "limit" {
            if let Some(mut limit) = v.as_i64() {
                if limit < 1 {
                    limit = 1;
                }
                if limit > max_limit {
                    limit = max_limit;
                }
                out.insert(k.clone(), limit.to_string());
                continue;
            }
        }

        if k == "offset" {
            if let Some(mut offset) = v.as_i64() {
                if offset < 0 {
                    offset = 0;
                }
                out.insert(k.clone(), offset.to_string());
                continue;
            }
        }

        if let Some(s) = v.as_str() {
            out.insert(k.clone(), s.to_string());
        } else {
            out.insert(k.clone(), v.to_string());
        }
    }

    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}
