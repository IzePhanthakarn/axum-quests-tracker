use std::sync::Arc;

use axum::{
    Extension,
    Router,
    extract::{ Path, State },
    response::IntoResponse,
    routing::{ patch },
};

use crate::{
    application::usecases::journey_ledger::JourneyLedgerUseCase,
    domain::repositories::{
        journey_ledger::JourneyLedgerRepository,
        quest_viewing::QuestViewingRepository,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            journey_ledger::JourneyLedgerPostgres,
            quest_viewing::QuestVieweingPostgres,
        },
    },
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
    unimplemented!()
}

pub async fn to_completed<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: JourneyLedgerRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    unimplemented!()
}

pub async fn to_failed<T1, T2>(
    State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: JourneyLedgerRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    unimplemented!()
}
