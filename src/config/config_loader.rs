use anyhow::{ Context, Result };
use std::{ env, str::FromStr };

use crate::config::config_model::{
    AdventurersSecret,
    AppConfig,
    Database,
    GuildCommandersSecret,
    Server,
};

use super::stage::Stage;

pub fn load() -> Result<AppConfig> {
    // โหลด .env ครั้งเดียวในที่นี้ (หรือจะไปไว้ใน main ก็ได้)
    dotenvy::dotenv().ok();

    let stage = load_stage()?;

    let server = Server {
        port: get_env_var("SERVER_PORT")?,
        body_limit: get_env_var("SERVER_BODY_LIMIT")?,
        timeout: get_env_var("SERVER_TIMEOUT")?,
    };

    let database = Database {
        url: env::var("DATABASE_URL").context("DATABASE_URL is not set")?,
    };

    let adventurers_secret = AdventurersSecret {
        secret: env::var("JWT_ADVENTURER_SECRET").context("JWT_ADVENTURER_SECRET is not set")?,
        refresh_secret: env
            ::var("JWT_ADVENTURER_REFRESH_SECRET")
            .context("JWT_ADVENTURER_REFRESH_SECRET is not set")?,
    };

    let guild_commanders_secret = GuildCommandersSecret {
        secret: env
            ::var("JWT_GUILD_COMMANDER_SECRET")
            .context("JWT_GUILD_COMMANDER_SECRET is not set")?,
        refresh_secret: env
            ::var("JWT_GUILD_COMMANDER_REFRESH_SECRET")
            .context("JWT_GUILD_COMMANDER_REFRESH_SECRET is not set")?,
    };

    Ok(AppConfig {
        stage,
        server,
        database,
        adventurers_secret,
        guild_commanders_secret,
    })
}

/// helper generic สำหรับ parse env เป็น type ที่ implement FromStr (u16, u64, ...)
fn get_env_var<T>(key: &str) -> Result<T>
    where T: FromStr, T::Err: std::error::Error + Send + Sync + 'static
{
    let raw = env::var(key).with_context(|| format!("{key} is not set"))?;
    raw.parse::<T>().with_context(|| format!("{key} is invalid: {raw}"))
}

fn load_stage() -> Result<Stage> {
    let stage_str = env::var("STAGE").unwrap_or_else(|_| "local".to_string());
    let stage = Stage::from_str(&stage_str).with_context(||
        format!("STAGE is invalid: {stage_str}")
    )?;
    Ok(stage)
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str[..]).unwrap_or_default()
}