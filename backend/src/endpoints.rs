use crate::model::database::Database;
use axum::Router;

mod polling;
mod registration;
mod auth;

pub fn get_nest() -> Router<Database> {
    Router::new()
        .nest("/pollings", polling::get_nest())
        .nest("/registration", registration::get_nest())
        .nest("/login", auth::get_nest())
}
