use crate::model::{database::Database, vote::Vote};
use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/vote", post(Vote::add_vote))
        .route("/vote/:id", get(Vote::get_vote))
        .route("/vote/:id", delete(Vote::delete_vote))
        .route("/votes", get(Vote::get_votes))
}
