use quests_tracker::config::config_loader;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = match config_loader::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load config: {:#}", e);
            std::process::exit(1);
        }
    };

    info!("Config loaded: {:?}", config);
}
