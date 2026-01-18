use std::sync::Arc;

use axum::{ Extension, Router, extract::{ Path, State }, middleware, response::IntoResponse, routing::{delete, post} };

use crate::{
    application::usecases::crew_switchboard::CrewSwitchboardUseCase,
    domain::repositories::{
        crew_switchboard::CrewSwitchboardRepository, quest_viewing::QuestViewingRepository,
    },
    infrastructure::{axum_http::middlewares::adventurers_authorization, postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            crew_switchboard::CrewSwitchboardPostgres,
            quest_viewing::QuestVieweingPostgres,
        },
    }},
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let crew_swichboard_repository = CrewSwitchboardPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestVieweingPostgres::new(Arc::clone(&db_pool));
    let crew_swichboard_use_case = CrewSwitchboardUseCase::new(
        Arc::new(quest_viewing_repository),
        Arc::new(crew_swichboard_repository)
    );
    Router::new()
    .route("/join/:quest_id", post(join))
    .route("/leave/:quest_id", delete(leave))
    .route_layer(middleware::from_fn(adventurers_authorization))
    .with_state(Arc::new(crew_swichboard_use_case))
}

pub async fn join<T1, T2>(
    State(crew_swichboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: QuestViewingRepository + Send + Sync, T2: CrewSwitchboardRepository + Send + Sync
{
    match crew_swichboard_use_case.join(quest_id, adventurer_id).await {
        Ok(_) => (axum::http::StatusCode::OK, "Joined the quest successfully").into_response(),
        Err(e) => (axum::http::StatusCode::BAD_REQUEST, format!("Failed to join the quest: {}", e)).into_response(),
    }
}

pub async fn leave<T1, T2>(
    State(crew_swichboard_use_case): State<Arc<CrewSwitchboardUseCase<T1, T2>>>,
    Extension(adventurer_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: QuestViewingRepository + Send + Sync, T2: CrewSwitchboardRepository + Send + Sync
{
    match crew_swichboard_use_case.leave(quest_id, adventurer_id).await {
        Ok(_) => (axum::http::StatusCode::OK, "Left the quest successfully").into_response(),
        Err(e) => (axum::http::StatusCode::BAD_REQUEST, format!("Failed to leave the quest: {}", e)).into_response(),
    }
}
