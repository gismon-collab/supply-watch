mod config;
mod monitor;
mod shortage;
mod alert;

use config::AppConfig;
use monitor::fetch_inventory;
use shortage::detect_shortages;
use alert::send_alerts;

#[tokio::main]
async fn main() {
    println!("Starting Supply Watch...");

    let config = AppConfig::load("config.json")
        .expect("Failed to load config");

    loop {
        println!("Running check cycle...");

        match fetch_inventory(&config).await {
            Ok(data) => {
                let shortages = detect_shortages(&config, &data);

                if !shortages.is_empty() {
                    send_alerts(&config, shortages).await;
                } else {
                    println!("No shortages detected.");
                }
            }
            Err(err) => {
                eprintln!("Monitoring error: {:?}", err);
            }
        }

        tokio::time::sleep(std::time::Duration::from_secs(config.interval_secs)).await;
    }
}
