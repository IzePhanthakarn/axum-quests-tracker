use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use crate::{
    domain::{
        entities::quests::QuestEntity,
        repositories::quest_viewing::QuestViewingRepository,
        value_objects::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgres::postgres_connection::PgPoolSquad,
};

pub struct QuestVieweingPostgres {
    pg_pool: Arc<PgPoolSquad>,
}

impl QuestVieweingPostgres {
    pub fn new(pg_pool: Arc<PgPoolSquad>) -> Self {
        Self { pg_pool }
    }
}

#[async_trait]
impl QuestViewingRepository for QuestVieweingPostgres {
    async fn view_details(&self, quest_id: i32) -> Result<QuestEntity> {
        unimplemented!()
    }

    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        unimplemented!()
    }

    async fn adventurers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64> {
        unimplemented!()
    }
}
