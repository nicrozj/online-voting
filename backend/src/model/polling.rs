use crate::model::database::Database;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, query_scalar};
use anyhow::Result;

#[derive(Clone, Serialize)]
pub struct Polling {
    pub id: u64,
    pub creator_id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddPollingForm {
    title: String,
    description: String,
}

impl Polling {
    pub async fn add_polling(
        State(database): State<Database>,
        Json(payload): Json<AddPollingForm>,
    ) -> Result<impl IntoResponse, StatusCode> {
        println!("add polling payload: {:?}", payload);
        let result = query!(
            "INSERT INTO pollings (title, description, creator_id) VALUES (?, ?, ?)",
            payload.title,
            payload.description,
            0
        )
        .execute(database.get_pool())
        .await
        .map_err(|e| {
            eprintln!("Error inserting polling or fetching last insert id: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
        let polling_id = result.last_insert_id();

        eprintln!("{}", polling_id);
        Ok(Json(polling_id))
    }

    pub async fn delete_polling(
        State(database): State<Database>,
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let _ = query!("DELETE FROM options WHERE polling_id = ?", id)
            .execute(database.get_pool())
            .await
            .map_err(|e| {
                eprintln!("delete options error: {e:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        let _ = query!("DELETE FROM pollings WHERE id = ?", id)
            .execute(database.get_pool()).await.map_err(|e| {
            eprintln!("delete polling error: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(StatusCode::OK)
    }

    pub async fn get_pollings(State(database): State<Database>) -> Result<Json<Vec<Polling>>, StatusCode> {
        let data = query_as!(Polling, "SELECT * FROM pollings")
            .fetch_all(database.get_pool())
            .await
            .map_err(|err| {
                eprintln!("get pollings error: {err:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    
        Ok(Json(data))
    }
    
    pub async fn get_polling(Path(id): Path<i32>, State(database): State<Database>) -> Result<Json<Polling>, StatusCode> {
        let data = query_as!(Polling, "SELECT * FROM pollings WHERE id = ?", id)
            .fetch_one(database.get_pool())
            .await
            .map_err(|err| {
                eprintln!("get polling error: {err:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    
        Ok(Json(data))
    }
}
