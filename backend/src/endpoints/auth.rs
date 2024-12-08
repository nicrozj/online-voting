use axum::http::HeaderMap;
use axum::routing::get;
use axum::Form;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use anyhow::Result;
use chrono::{Duration, Utc};
use serde::Deserialize;

use crate::model::database::Database;
use crate::model::user::User;

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/", get(login))
}

#[derive(Deserialize)]
pub struct LoginPayload {
    login: String,
    password: String
}

async fn login(State(db): State<Database>, headers: HeaderMap, Json(payload): Json<LoginPayload>) -> Result<impl IntoResponse, StatusCode> {
    let is_auth = match User::login(&payload.login, &payload.password, &db).await {
        Ok(is_auth) => is_auth,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    let user_id = match User::get_user_id_from_login(&payload.login, &db).await {
        Ok(value) => value,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    let user_agent = headers.get("User-Agent")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let expire_date = Utc::now().naive_utc() + Duration::days(14);

    let token = User::create_token(user_id, &payload.login, user_agent, &expire_date, &db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let cookie = format!("token={}", token);
    let headers: [(&str, String); 1] = [
        ("set-cookie", cookie),
    ];

    match is_auth {
        true => Ok((headers, StatusCode::OK)),
        false => Err(StatusCode::FORBIDDEN)
    }
}