use std::sync::Arc;

use axum::{ Extension, Router, extract::{ Path, State }, response::IntoResponse, routing::{delete, post} };

use crate::{
    application::usecases::crew_switchboard::CrewSwitchboardUseCase,
    domain::repositories::{
        adventurers::AdventurersRepository,
        crew_switchboard::CrewSwitchboardRepository,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            adventurers::AdventurerPostgres,
            crew_switchboard::CrewSwitchboardPostgres,
            quest_viewing::QuestVieweingPostgres,
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_swichboard_repository = CrewSwitchboardPostgres::new(Arc::clone(&db_pool));
    let adventurer_repository = AdventurerPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestVieweingPostgres::new(Arc::clone(&db_pool));
    let crew_swichboard_use_case = CrewSwitchboardUseCase::new(
        Arc::new(adventurer_repository),
        Arc::new(crew_swichboard_repository)
    );
    Router::new()
    .route("/join/:quest_id", post(join))
    .route("/leave/:quest_id", delete(leave))
    .with_state(Arc::new(crew_swichboard_use_case))
}

pub async fn join<T1, T2>(
    State(crew_swichboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: AdventurersRepository + Send + Sync, T2: CrewSwitchboardRepository + Send + Sync
{
    unimplemented!()
}

pub async fn leave<T1, T2>(
    State(crew_swichboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: AdventurersRepository + Send + Sync, T2: CrewSwitchboardRepository + Send + Sync
{
    unimplemented!()
}
