use axum::{routing::{delete, get, post}, Router};
use crate::model::{database::Database, polling::Polling, option::Option};

pub fn get_nest() -> Router<Database> {
    Router::new()
        .route("/", post(Polling::add_polling))
        .route("/", get(Polling::get_pollings))
        .route("/:id", get(Polling::get_polling))
        .route("/:id", delete(Polling::delete_polling))
        .route("/:id/options", post(Option::add_options))
        .route("/:id/options", get(Option::get_options))
}