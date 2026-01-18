use std::sync::Arc;

use axum::{ Router, extract::{ Path, Query, State }, response::IntoResponse, routing::get };

use crate::{
    application::usecases::quest_viewing::QuestViewingUseCase,
    domain::{
        repositories::quest_viewing::QuestViewingRepository,
        value_objects::board_checking_filter::BoardCheckingFilter,
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::quest_viewing::QuestVieweingPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_viewing_repository = QuestVieweingPostgres::new(db_pool);
    let adventurers_use_case = QuestViewingUseCase::new(Arc::new(quest_viewing_repository));

    Router::new()
        .route("/:quest_id", get(view_details))
        .route("/board_checking", get(board_checking))
        .with_state(Arc::new(adventurers_use_case))
}

pub async fn view_details<T>(
    State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
    Path(quest_id): Path<i32>
) -> impl IntoResponse
    where T: QuestViewingRepository + Send + Sync
{
    match quest_viewing_use_case.view_details(quest_id).await {
        Ok(quest_model) => (axum::http::StatusCode::OK, serde_json::to_string(&quest_model).unwrap()),
        Err(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}

pub async fn board_checking<T>(
    State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
    filter: Query<BoardCheckingFilter>
) -> impl IntoResponse
    where T: QuestViewingRepository + Send + Sync
{
    match quest_viewing_use_case.board_checking(&filter).await {
        Ok(quests_model) => (axum::http::StatusCode::OK, serde_json::to_string(&quests_model).unwrap()),
        Err(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}
