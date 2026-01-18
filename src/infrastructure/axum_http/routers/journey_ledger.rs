use std::sync::Arc;

use axum::{
    Extension, Router, extract::{ Path, State }, middleware, response::IntoResponse, routing::patch
};

use crate::{
    application::usecases::journey_ledger::JourneyLedgerUseCase,
    domain::repositories::{
        journey_ledger::JourneyLedgerRepository,
        quest_viewing::QuestViewingRepository,
    },
    infrastructure::{axum_http::middlewares::guild_commanders_authorization, postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            journey_ledger::JourneyLedgerPostgres,
            quest_viewing::QuestVieweingPostgres,
        },
    }},
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let journey_ledger_repository = JourneyLedgerPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestVieweingPostgres::new(Arc::clone(&db_pool));
    let journey_ledger_use_case = JourneyLedgerUseCase::new(
        Arc::new(journey_ledger_repository),
        Arc::new(quest_viewing_repository)
    );

    Router::new()
        .route("/in-journey/:quest_id", patch(in_journey))
        .route("/to-completed/:quest_id", patch(to_completed))
        .route("/to-failed/:quest_id", patch(to_failed))
        .route_layer(middleware::from_fn(guild_commanders_authorization))
        .with_state(Arc::new(journey_ledger_use_case))
}

pub async fn in_journey<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: JourneyLedgerRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    match journey_ledger_use_case.in_journey(quest_id, guild_commander_id).await {
        Ok(result) => (axum::http::StatusCode::OK, format!("Quest set to In Journey status. ID: {}", result)).into_response(),
        Err(err) => (axum::http::StatusCode::BAD_REQUEST, format!("Error: {}", err)).into_response(),
    }
}

pub async fn to_completed<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: JourneyLedgerRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    match journey_ledger_use_case.to_completed(quest_id, guild_commander_id).await {
        Ok(result) => (axum::http::StatusCode::OK, format!("Quest set to Completed status. ID: {}", result)).into_response(),
        Err(err) => (axum::http::StatusCode::BAD_REQUEST, format!("Error: {}", err)).into_response(),
    }
}

pub async fn to_failed<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: JourneyLedgerRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    match journey_ledger_use_case.to_failed(quest_id, guild_commander_id).await {
        Ok(result) => (axum::http::StatusCode::OK, format!("Quest set to Failed status. ID: {}", result)).into_response(),
        Err(err) => (axum::http::StatusCode::BAD_REQUEST, format!("Error: {}", err)).into_response(),
    }
}
