use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::{ prelude::* };

use crate::{
    domain::{
        entities::quests::QuestEntity,
        repositories::quest_viewing::QuestViewingRepository,
        value_objects::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        schema::{ quest_adventurer_junction, quests },
    },
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
        let mut conn = Arc::clone(&self.pg_pool).get()?;

        let result = quests::table
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .select(QuestEntity::as_select())
            .first::<QuestEntity>(&mut conn)?;

        Ok(result)
    }

    async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestEntity>> {
        let mut conn = Arc::clone(&self.pg_pool).get()?;

        let mut query = quests::table.filter(quests::deleted_at.is_null()).into_boxed();

        if let Some(name) = &filter.name {
            query = query.filter(quests::name.ilike(format!("%{}%", name)));
        }

        if let Some(status) = &filter.status {
            query = query.filter(quests::status.eq(status.to_string()));
        }

        let result = query
            .select(QuestEntity::as_select())
            .order(quests::created_at.desc())
            .load::<QuestEntity>(&mut conn)?;

        Ok(result)
    }

    async fn adventurers_counting_by_quest_id(&self, quest_id: i32) -> Result<i64> {
        let mut conn = Arc::clone(&self.pg_pool).get()?;

        let result = quest_adventurer_junction::table
            .count()
            .filter(quest_adventurer_junction::quest_id.eq(quest_id))
            .first::<i64>(&mut conn)?;

        Ok(result)
    }
}
