use crate::model::database::Database;
use axum::Router;

mod vote;
mod votes;

pub fn get_nest() -> Router<Database> {
    Router::new()
        .nest("/vote", vote::get_nest())
        .nest("/votes", votes::get_nest())
}
