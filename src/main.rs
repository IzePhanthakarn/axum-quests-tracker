use std::sync::Arc;

use quests_tracker::{
    config::config_loader,
    infrastructure::{ axum_http::http_serve::start, postgres::postgres_connection },
};
use tracing::{ error, info };

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let config = match config_loader::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load config: {:#}", e);
            std::process::exit(1);
        }
    };

    info!("Config loaded: {:?}", config);

    let postgres_pool = match postgres_connection::establish_connection(&config.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to connect to database: {:#}", e);
            std::process::exit(1);
        }
    };

    info!("Connected to database");

    start(Arc::new(config), Arc::new(postgres_pool)).await.expect("Failed to start server");
}
