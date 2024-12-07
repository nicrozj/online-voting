use axum::{routing::{delete, get, post}, Router};
use crate::model::{database::Database, vote::Vote, option::Option};

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/", post(Vote::add_vote))
        .route("/:id", get(Vote::get_vote))
        .route("/:id", delete(Vote::delete_vote))
        .route("/:id/options", post(Option::add_options))
        .route("/:id/options", get(Option::get_options))
        .route("/votes", get(Vote::get_votes))
}