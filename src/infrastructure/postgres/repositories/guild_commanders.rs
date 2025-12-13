use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::{entities::guild_commanders::{GuildCommanderEntity, RegisterGuildCommanderEntity}, repositories::guild_commanders::GuildCommanderRepository},
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct GuildCommandersPostgres {
    pg_pool: Arc<PgPoolSquad>,
}

impl GuildCommandersPostgres {
    pub fn new(pg_pool: Arc<PgPoolSquad>) -> Self {
        Self { pg_pool }
    }
}

#[async_trait]
impl GuildCommanderRepository for GuildCommandersPostgres {
    async fn register(
        &self,
        register_guild_commander_entity: RegisterGuildCommanderEntity
    ) -> Result<i32> {
        unimplemented!()
    }
    async fn find_by_username(&self, username: String) -> Result<GuildCommanderEntity> {
        unimplemented!()
    }
}
