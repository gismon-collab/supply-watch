use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct AppConfig {
    pub interval_secs: u64,
    pub shortage_threshold: i32,
    pub sources: Vec<String>,
}

impl AppConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }
}
