use crate::model::database::Database;
use crate::model::vote::Vote;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlx::{query, query_as};

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/votes", get(get_votes))
        .route("/votes/:id", get(get_vote))
        .route("/vote", post(add_vote))
}

async fn get_votes(State(database): State<Database>) -> Json<Vec<Vote>> {
    let data = query_as!(Vote, "SELECT * FROM votes")
        .fetch_all(database.get_pool())
        .await
        .unwrap();

    Json(data)
}

async fn get_vote(Path(id): Path<i32>, State(database): State<Database>) -> Json<Vote> {
    let data = query_as!(Vote, "SELECT * FROM votes WHERE id = ?", id)
        .fetch_one(database.get_pool())
        .await
        .unwrap();

    Json(data)
}

#[derive(Deserialize, Clone, Debug)]
struct AddVoteForm {
    title: String,
    description: String,
}

async fn add_vote(
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
