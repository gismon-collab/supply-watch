use crate::config::AppConfig;
use reqwest::Client;

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub quantity: i32,
}

pub async fn fetch_inventory(
    config: &AppConfig,
) -> Result<Vec<Item>, Box<dyn std::error::Error>> {

    let client = Client::new();
    let mut items = Vec::new();

    for source in &config.sources {
        let response = client.get(source).send().await?;

        // Example expected JSON format:
        // [{"name":"Rice","quantity":3}]
        let data: Vec<serde_json::Value> = response.json().await?;

        for entry in data {
            items.push(Item {
                name: entry["name"].as_str().unwrap_or("Unknown").to_string(),
                quantity: entry["quantity"].as_i64().unwrap_or(0) as i32,
            });
        }
    }

    Ok(items)
}
