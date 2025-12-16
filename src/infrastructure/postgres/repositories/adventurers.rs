use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper, insert_into, query_dsl::methods::{FilterDsl, SelectDsl}};

use crate::{
    domain::{
        entities::adventures::{ AdventurerEntity, RegisterAdventurerEntity },
        repositories::adventurers::AdventurersRepository,
    },
    infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::adventurers},
};

pub struct AdventurerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AdventurerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AdventurersRepository for AdventurerPostgres {
    async fn register(&self, register_adventurers_entity: RegisterAdventurerEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(adventurers::table)
        .values(register_adventurers_entity)
        .returning(adventurers::id)
        .get_result::<i32>( &mut conn)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<AdventurerEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = adventurers::table
            .filter(adventurers::username.eq(username))
            .select(AdventurerEntity::as_select())
            .first::<AdventurerEntity>(&mut conn)?;
        
        Ok(result)
    }
}
