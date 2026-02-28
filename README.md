# hookfreight-rust

Official Rust SDK for [HookFreight](https://hookfreight.com).

## Installation

```bash
cargo add hookfreight
```

## Quick Start

```rust
use std::collections::HashMap;

use hookfreight::{HookFreight, HookFreightConfig};
use serde_json::json;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HookFreight::new(HookFreightConfig {
        api_key: Some("hf_sk_...".to_string()),
        ..HookFreightConfig::default()
    })?;

    let mut params = HashMap::new();
    params.insert("limit".to_string(), json!(10));

    let deliveries = client.deliveries.list(Some(&params)).await?;
    println!("{}", deliveries);

    Ok(())
}
```

## Features

- Apps: list/create/get/update/delete
- Endpoints: list/create/get/update/delete
- Events: list/get/list_by_endpoint/replay
- Deliveries: list/list_by_event/retry/queue_stats
- API and connection error mapping

## License

Apache-2.0
