use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{
        crew_switchboard::CrewSwitchboardRepository,
        quest_viewing::QuestViewingRepository,
    },
    value_objects::{
        quest_adventurer_junction::{ MAX_ADVENTURERS_PER_QUEST, QuestAdventurerJunction },
        quest_statuses::QuestStatus,
    },
};

pub struct CrewSwitchboardUseCase<T1, T2>
    where T1: QuestViewingRepository + Send + Sync, T2: CrewSwitchboardRepository + Send + Sync {
    quest_viewing_repository: Arc<T1>,
    crew_switchboard_repository: Arc<T2>,
}

impl<T1, T2> CrewSwitchboardUseCase<T1, T2>
    where T1: QuestViewingRepository + Send + Sync, T2: CrewSwitchboardRepository + Send + Sync
{
    pub fn new(quest_viewing_repository: Arc<T1>, crew_switchboard_repository: Arc<T2>) -> Self {
        Self {
            quest_viewing_repository,
            crew_switchboard_repository,
        }
    }

    pub async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurers_count =
            self.quest_viewing_repository.adventurers_counting_by_quest_id(quest_id).await?;

        let quest_status_condition =
            quest.status == QuestStatus::Open.to_string() ||
            quest.status == QuestStatus::Failed.to_string();

        let adventurers_count_condition = adventurers_count < MAX_ADVENTURERS_PER_QUEST;

        if !adventurers_count_condition {
            return Err(anyhow::anyhow!("Quest is full"));
        }

        if !quest_status_condition {
            return Err(anyhow::anyhow!("Quest is not joinable"));
        }

        self.crew_switchboard_repository.join(QuestAdventurerJunction {
            adventurer_id,
            quest_id,
        }).await?;

        Ok(())
    }

    pub async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<()> {
        let quest = self.quest_viewing_repository.view_details(quest_id).await?;

        let leaving_condition =
            quest.status == QuestStatus::Open.to_string() ||
            quest.status == QuestStatus::Failed.to_string();

        if !leaving_condition {
            return Err(anyhow::anyhow!("Quest is not leaveable"));
        }

        self.crew_switchboard_repository.leave(QuestAdventurerJunction {
            adventurer_id,
            quest_id,
        }).await?;

        Ok(())
    }
}
