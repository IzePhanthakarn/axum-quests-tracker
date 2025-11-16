use crate::config::stage::Stage;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub stage: Stage,
    pub server: Server,
    pub database: Database,
    pub adventurers_secret: AdventurersSecret,
    pub guild_commanders_secret: GuildCommandersSecret,
}

#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout: u64,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct AdventurersSecret {
    pub secret: String,
    pub refresh_secret: String,
}

#[derive(Debug, Clone)]
pub struct GuildCommandersSecret {
    pub secret: String,
    pub refresh_secret: String,
}