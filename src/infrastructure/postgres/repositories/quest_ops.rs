use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::{entities::quests::{AddQuestEntity, EditQuestEntity}, repositories::quest_ops::QuestOpsRepository},
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct QuestOpsPostgres {
    pg_pool: Arc<PgPoolSquad>,
}

impl QuestOpsPostgres {
    pub fn new(pg_pool: Arc<PgPoolSquad>) -> Self {
        Self { pg_pool }
    }
}

#[async_trait]
impl QuestOpsRepository for QuestOpsPostgres {
    async fn add(&self, add_quest_entity: AddQuestEntity) -> Result<i32> {
        unimplemented!()
    }

    async fn edit(&self, quest_id: i32, edit_quest_entity: EditQuestEntity) -> Result<i32> {
        unimplemented!()
    }

    async fn remove(&self, quest_id: i32, guild_commander_id: EditQuestEntity) -> Result<()> {
        unimplemented!()
    }
}
