use crate::config::AppConfig;
use crate::monitor::Item;

pub fn detect_shortages(
    config: &AppConfig,
    items: &Vec<Item>,
) -> Vec<String> {

    items.iter()
        .filter(|item| item.quantity <= config.shortage_threshold)
        .map(|item| format!("Shortage detected: {} (qty {})", item.name, item.quantity))
        .collect()
}
