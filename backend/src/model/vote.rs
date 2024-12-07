use crate::model::database::Database;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use anyhow::Result;

#[derive(Clone, Serialize)]
pub struct Vote {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddVoteForm {
    title: String,
    description: String,
}

impl Vote {
    pub async fn add_vote(
        State(database): State<Database>,
        Json(payload): Json<AddVoteForm>,
    ) -> Result<impl IntoResponse, StatusCode> {
        println!("{:?}", payload);
        let query = query!(
            "INSERT INTO votes(title, description) VALUES(?, ?)",
            payload.title,
            payload.description
        );

        query.execute(database.get_pool()).await.map_err(|e| {
            eprintln!("add vote error: {e:?}, payload: {payload:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(StatusCode::OK)
    }

    pub async fn delete_vote(
        State(database): State<Database>,
        Path(id): Path<i32>,
    ) -> Result<impl IntoResponse, StatusCode> {
        let query = query!("DELETE FROM votes WHERE id = ?", id);

        query.execute(database.get_pool()).await.map_err(|e| {
            eprintln!("delete vote error: {e:?}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        Ok(StatusCode::OK)
    }

    pub async fn get_votes(State(database): State<Database>) -> Result<Json<Vec<Vote>>, StatusCode> {
        let data = query_as!(Vote, "SELECT * FROM votes")
            .fetch_all(database.get_pool())
            .await
            .map_err(|err| {
                eprintln!("get votes error: {err:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    
        Ok(Json(data))
    }
    
    pub async fn get_vote(Path(id): Path<i32>, State(database): State<Database>) -> Result<Json<Vote>, StatusCode> {
        let data = query_as!(Vote, "SELECT * FROM votes WHERE id = ?", id)
            .fetch_one(database.get_pool())
            .await
            .map_err(|err| {
                eprintln!("get vote error: {err:?}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    
        Ok(Json(data))
    }
}
