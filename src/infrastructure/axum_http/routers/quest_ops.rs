use std::sync::Arc;

use axum::{
    Extension, Json, Router, extract::{ Path, State }, middleware, response::IntoResponse, routing::{ delete, patch, post }
};

use crate::{
    application::usecases::quest_ops::QuestOpsUseCase,
    domain::{
        repositories::{ quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository },
        value_objects::quest_model::{ AddQuestModel, EditQuestModel },
    },
    infrastructure::{axum_http::middlewares::guild_commanders_authorization, postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{ quest_ops::QuestOpsPostgres, quest_viewing::QuestVieweingPostgres },
    }},
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let quest_ops_repository = QuestOpsPostgres::new(Arc::clone(&db_pool));
    let quest_viewing_repository = QuestVieweingPostgres::new(Arc::clone(&db_pool));
    let quest_ops_use_case = QuestOpsUseCase::new(
        Arc::new(quest_ops_repository),
        Arc::new(quest_viewing_repository)
    );

    Router::new()
        .route("/", post(add))
        .route("/:quest_id", patch(edit))
        .route("/:quest_id", delete(remove))
        .route_layer(middleware::from_fn(guild_commanders_authorization))
        .with_state(Arc::new(quest_ops_use_case))
}

pub async fn add<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Json(add_quest_model): Json<AddQuestModel>
)
    -> impl IntoResponse
    where T1: QuestOpsRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    match quest_ops_use_case.add(add_quest_model.guild_commander_id, add_quest_model).await {
        Ok(quest_id) => (axum::http::StatusCode::CREATED, Json(quest_id)).into_response(),
        Err(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response(),
    }
}

pub async fn edit<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Path(quest_id): Path<i32>,
    Json(edit_quest_model): Json<EditQuestModel>
)
    -> impl IntoResponse
    where T1: QuestOpsRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    match quest_ops_use_case.edit(quest_id, edit_quest_model.guild_commander_id, edit_quest_model).await {
        Ok(edited_quest_id) => (axum::http::StatusCode::OK, Json(edited_quest_id)).into_response(),
        Err(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response(),
    }
}

pub async fn remove<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: QuestOpsRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    match quest_ops_use_case.remove(quest_id, guild_commander_id).await {
        Ok(_) => (axum::http::StatusCode::NO_CONTENT).into_response(),
        Err(err) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string())).into_response(),
    }
}
