use std::sync::Arc;

use anyhow::Result;

use crate::{domain::repositories::{
    adventurers::AdventurersRepository,
    crew_switchboard::CrewSwitchboardRepository,
}};

pub struct CrewSwitchboardUseCase<T1, T2>
    where T1: AdventurersRepository + Send + Sync, T2: CrewSwitchboardRepository + Send + Sync {
    adventurers_repository: Arc<T1>,
    crew_switchboard_repository: Arc<T2>,
}

impl <T1,T2> CrewSwitchboardUseCase<T1, T2>
where T1: AdventurersRepository + Send + Sync, T2: CrewSwitchboardRepository + Send + Sync {
    pub fn new(adventurers_repository: Arc<T1>, crew_switchboard_repository: Arc<T2>) -> Self {
        Self {
            adventurers_repository,
            crew_switchboard_repository,
        }
    }

    pub async fn join(&self, quest_id: i32, adventurer_id: i32) -> Result<i32> {
        unimplemented!()
    }
    pub async fn leave(&self, quest_id: i32, adventurer_id: i32) -> Result<i32> {
        unimplemented!()
    }
}