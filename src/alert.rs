use crate::config::AppConfig;

pub async fn send_alerts(config: &AppConfig, alerts: Vec<String>) {
    println!("ALERTS:");

    for alert in alerts {
        println!("{}", alert);
    }

    // Extend later:
    // - Email
    // - Slack / Discord webhook
    // - PagerDuty
}
