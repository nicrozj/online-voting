use axum::{debug_handler, extract::{Path, State}, handler, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::model::database::Database;

#[derive(Deserialize, Serialize, Clone)]
pub struct Option {
    pub id: u64,
    pub polling_id: u64,
    pub option_text: String,
}

#[derive(Deserialize)]
pub struct Options {
    options: Vec<String>
}
impl Option {
    pub async fn add_options(
        State(database): State<Database>,
        Path(id): Path<u64>,
        Json(options): Json<Options> 
    ) -> Result<impl IntoResponse, StatusCode> {
        eprintln!("options: {:?}", options.options);
        for option in options.options {
            sqlx::query!(
                "INSERT INTO options (polling_id, option_text) VALUES (?, ?)",
                id,
                option,
            )
            .execute(database.get_pool())
            .await
            .map_err(|err| {
                eprintln!("add options err: {err}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        }
        Ok(StatusCode::OK)
    }

    pub async fn get_options(
        State(database): State<Database>,
        Path(id): Path<u64>,
    ) -> Result<Json<Vec<Option>>, StatusCode> {
        let options = sqlx::query_as!(
            Option,
            "SELECT * FROM options WHERE (polling_id) = ?",
            id,
        )
        .fetch_all(database.get_pool())
        .await
        .map_err(|err| {
            eprintln!("add options err: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(Json(options))
    }
}