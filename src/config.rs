use std::time::Duration;

#[derive(Clone, Debug)]
pub struct HookFreightConfig {
    pub api_key: Option<String>,
    pub base_url: String,
    pub timeout: Duration,
}

impl Default for HookFreightConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: "https://api.hookfreight.com/v1".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}
