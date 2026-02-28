use std::collections::HashMap;

use hookfreight::{Hookfreight, HookfreightConfig};
use serde_json::{json, Value};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Hookfreight::new(HookfreightConfig {
        api_key: std::env::var("HOOKFREIGHT_API_KEY").ok(),
        ..HookfreightConfig::default()
    })?;

    let mut params: HashMap<String, Value> = HashMap::new();
    params.insert("limit".to_string(), json!(10));

    let deliveries = client.deliveries.list(Some(&params)).await?;
    println!("{}", deliveries);

    Ok(())
}
