use std::sync::Arc;

use axum::{
    Extension,
    Json,
    Router,
    extract::{ Path, State },
    response::IntoResponse,
    routing::{ delete, patch, post },
};

use crate::{
    application::usecases::quest_ops::QuestOpsUseCase,
    domain::{
        repositories::{ quest_ops::QuestOpsRepository, quest_viewing::QuestViewingRepository },
        value_objects::quest_model::{ AddQuestModel, EditQuestModel },
    },
    infrastructure::postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{ quest_ops::QuestOpsPostgres, quest_viewing::QuestVieweingPostgres },
    },
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
        .with_state(Arc::new(quest_ops_use_case))
}

pub async fn add<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Json(add_quest_model): Json<AddQuestModel>
)
    -> impl IntoResponse
    where T1: QuestOpsRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    unimplemented!()
}

pub async fn edit<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>,
    Json(edit_quest_model): Json<EditQuestModel>
)
    -> impl IntoResponse
    where T1: QuestOpsRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    unimplemented!()
}

pub async fn remove<T1, T2>(
    State(quest_ops_use_case): State<Arc<QuestOpsUseCase<T1, T2>>>,
    Extension(guild_commander_id): Extension<i32>,
    Path(quest_id): Path<i32>
)
    -> impl IntoResponse
    where T1: QuestOpsRepository + Send + Sync, T2: QuestViewingRepository + Send + Sync
{
    unimplemented!()
}
