use std::sync::Arc;

use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use axum_extra::extract::CookieJar;

use crate::{
    application::usecases::authentication::AuthenticationUseCase, domain::repositories::{adventurers::AdventurersRepository, guild_commanders::GuildCommanderRepository}, infrastructure::{jwt_authentication::authentication_model::LoginModel, postgres::{
        postgres_connection::PgPoolSquad,
        repositories::{
            adventurers::AdventurerPostgres,
            guild_commanders::GuildCommandersPostgres,
        },
    }}
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repository = AdventurerPostgres::new(Arc::clone(&db_pool));
    let guild_commanders_repository = GuildCommandersPostgres::new(Arc::clone(&db_pool));
    let authentication_use_case = AuthenticationUseCase::new(
        Arc::new(adventurers_repository),
        Arc::new(guild_commanders_repository)
    );

    Router::new()
        .route("/adventurers/login", post(adventurers_login))
        .route("/adventurers/refresh_token", post(adventurers_refresh_token))
        .route("/guild-commanders/login", post(guild_commanders_login))
        .route("/guild-commanders/refresh_token", post(guild_commanders_refresh_token))
        .with_state(Arc::new(authentication_use_case))
}

pub async fn adventurers_login<T1,T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where 
    T1: AdventurersRepository + Send + Sync, 
    T2: GuildCommanderRepository + Send + Sync 
{
    unimplemented!()
}

pub async fn adventurers_refresh_token<T1,T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    jar: CookieJar,
) -> impl IntoResponse
where 
    T1: AdventurersRepository + Send + Sync, 
    T2: GuildCommanderRepository + Send + Sync 
{
    unimplemented!()
}

pub async fn guild_commanders_login<T1,T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    Json(login_model): Json<LoginModel>,
) -> impl IntoResponse
where 
    T1: AdventurersRepository + Send + Sync, 
    T2: GuildCommanderRepository + Send + Sync 
{
    unimplemented!()
}

pub async fn guild_commanders_refresh_token<T1,T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    jar: CookieJar,
) -> impl IntoResponse
where 
    T1: AdventurersRepository + Send + Sync, 
    T2: GuildCommanderRepository + Send + Sync 
{
    unimplemented!()
}
