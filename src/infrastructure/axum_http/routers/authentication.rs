use std::sync::Arc;

use axum::{
    Json, Router, extract::State, http::{ HeaderMap, HeaderValue, StatusCode, header }, middleware, response::IntoResponse, routing::post
};
use axum_extra::extract::cookie::{ Cookie, CookieJar };
use cookie::time::Duration;

use crate::{
    application::usecases::authentication::AuthenticationUseCase,
    config::{ config_loader::{ get_stage }, stage::Stage },
    domain::repositories::{
        adventurers::AdventurersRepository,
        guild_commanders::GuildCommanderRepository,
    },
    infrastructure::{
        axum_http::{middlewares::{adventurers_authorization, guild_commanders_authorization}, response::{
            api_response::ApiResponse,
            auth_response::LoginResponse,
            err_response::{ ErrMessage, ErrResponse },
        }},
        jwt_authentication::authentication_model::LoginModel,
        postgres::{
            postgres_connection::PgPoolSquad,
            repositories::{
                adventurers::AdventurerPostgres,
                guild_commanders::GuildCommandersPostgres,
            },
        },
    },
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
        .route("/guild-commanders/login", post(guild_commanders_login))
        .route("/adventurers/refresh_token", post(adventurers_refresh_token).route_layer(middleware::from_fn(adventurers_authorization)))
        .route("/guild-commanders/refresh_token", post(guild_commanders_refresh_token).route_layer(middleware::from_fn(guild_commanders_authorization)))
        .with_state(Arc::new(authentication_use_case))
}

pub async fn adventurers_login<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    Json(login_model): Json<LoginModel>
)
    -> impl IntoResponse
    where T1: AdventurersRepository + Send + Sync, T2: GuildCommanderRepository + Send + Sync
{
    match authentication_use_case.adventurers_login(login_model).await {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                act_cookie = act_cookie.secure(true);
                rft_cookie = rft_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap()
            );
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap()
            );

            (
                StatusCode::OK,
                headers,
                Json(ApiResponse {
                    success: true,
                    message: "Login successful".to_string(),
                    data: LoginResponse {
                        access_token: passport.access_token,
                        refresh_token: passport.refresh_token,
                    },
                }),
            ).into_response()
        }
        Err(err) => {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrResponse {
                    success: false,
                    message: "Login failed".to_string(),
                    error: ErrMessage {
                        message: err.to_string(),
                    },
                }),
            ).into_response()
        }
    }
}

pub async fn adventurers_refresh_token<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    jar: CookieJar
)
    -> impl IntoResponse
    where T1: AdventurersRepository + Send + Sync, T2: GuildCommanderRepository + Send + Sync
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();

        let response = match authentication_use_case.adventurers_refresh_token(refresh_token).await {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    act_cookie = act_cookie.secure(true);
                    rft_cookie = rft_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap()
                );
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap()
                );

                (
                    StatusCode::OK,
                    headers,
                    Json(ApiResponse {
                        success: true,
                        message: "Login successful".to_string(),
                        data: LoginResponse {
                            access_token: passport.access_token,
                            refresh_token: passport.refresh_token,
                        },
                    }),
                ).into_response()
            }
            Err(err) => {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrResponse {
                        success: false,
                        message: "Login failed".to_string(),
                        error: ErrMessage {
                            message: err.to_string(),
                        },
                    }),
                ).into_response()
            }
        };

        return response;
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(ErrResponse {
            success: false,
            message: "Failed to get refresh token".to_string(),
            error: ErrMessage {
                message: "No refresh token cookie found".to_string(),
            },
        }),
    ).into_response()
}

pub async fn guild_commanders_login<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    Json(login_model): Json<LoginModel>
)
    -> impl IntoResponse
    where T1: AdventurersRepository + Send + Sync, T2: GuildCommanderRepository + Send + Sync
{
    match authentication_use_case.guild_commanders_login(login_model).await {
        Ok(passport) => {
            let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                .path("/")
                .same_site(cookie::SameSite::Lax)
                .http_only(true)
                .max_age(Duration::days(14));

            if get_stage() == Stage::Production {
                act_cookie = act_cookie.secure(true);
                rft_cookie = rft_cookie.secure(true);
            }

            let mut headers = HeaderMap::new();

            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&act_cookie.to_string()).unwrap()
            );
            headers.append(
                header::SET_COOKIE,
                HeaderValue::from_str(&rft_cookie.to_string()).unwrap()
            );

            (StatusCode::OK, headers, "Adventurer logged in").into_response()
        }
        Err(err) => (StatusCode::UNAUTHORIZED, err.to_string()).into_response(),
    }
}

pub async fn guild_commanders_refresh_token<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUseCase<T1, T2>>>,
    jar: CookieJar
)
    -> impl IntoResponse
    where T1: AdventurersRepository + Send + Sync, T2: GuildCommanderRepository + Send + Sync
{
    if let Some(rft) = jar.get("rft") {
        let refresh_token = rft.value().to_string();

        let response = match authentication_use_case.guild_commanders_refresh_token(refresh_token).await {
            Ok(passport) => {
                let mut act_cookie = Cookie::build(("act", passport.access_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                let mut rft_cookie = Cookie::build(("rft", passport.refresh_token.clone()))
                    .path("/")
                    .same_site(cookie::SameSite::Lax)
                    .http_only(true)
                    .max_age(Duration::days(14));

                if get_stage() == Stage::Production {
                    act_cookie = act_cookie.secure(true);
                    rft_cookie = rft_cookie.secure(true);
                }

                let mut headers = HeaderMap::new();

                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&act_cookie.to_string()).unwrap()
                );
                headers.append(
                    header::SET_COOKIE,
                    HeaderValue::from_str(&rft_cookie.to_string()).unwrap()
                );

                (
                    StatusCode::OK,
                    headers,
                    Json(ApiResponse {
                        success: true,
                        message: "Login successful".to_string(),
                        data: LoginResponse {
                            access_token: passport.access_token,
                            refresh_token: passport.refresh_token,
                        },
                    }),
                ).into_response()
            }
            Err(err) => {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(ErrResponse {
                        success: false,
                        message: "Login failed".to_string(),
                        error: ErrMessage {
                            message: err.to_string(),
                        },
                    }),
                ).into_response()
            }
        };

        return response;
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(ErrResponse {
            success: false,
            message: "Failed to get refresh token".to_string(),
            error: ErrMessage {
                message: "No refresh token cookie found".to_string(),
            },
        }),
    ).into_response()
}
