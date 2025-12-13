use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::{
        entities::adventures::{ AdventurerEntity, RegisterAdventurerEntity },
        repositories::adventurers::AdventurersRepository,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct AdventurerPostgres {
    db_poll: Arc<PgPoolSquad>,
}

impl AdventurerPostgres {
    pub fn new(db_poll: Arc<PgPoolSquad>) -> Self {
        Self { db_poll }
    }
}

#[async_trait]
impl AdventurersRepository for AdventurerPostgres {
    async fn register(&self, register_adventurers_entity: RegisterAdventurerEntity) -> Result<i32> {
        unimplemented!()
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        unimplemented!()
    }
}
