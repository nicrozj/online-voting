use axum::routing::get;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use anyhow::Result;
use serde::Deserialize;
use sqlx::database;

use crate::model::database::Database;
use crate::model::user::User;

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/", get(registration))
}

#[derive(Deserialize)]
pub struct RegistrationPayload {
    login: String,
    password: String
}

async fn registration (
    State(database): State<Database>,
    Json(payload): Json<RegistrationPayload>, 
) -> Result<impl IntoResponse, StatusCode> {
    let user_id = User::registration(&payload.login, &payload.password, &database)
        .await
        .map_err(|err| {
            eprintln!("registration error {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(user_id))
}