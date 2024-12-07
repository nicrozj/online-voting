use axum::{routing::{delete, get, post}, Router};
use crate::model::{database::Database, vote::Vote};

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/", get(Vote::get_votes))
}