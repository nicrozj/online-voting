use crate::model::database::Database;
use axum::Router;

mod polling;

pub fn get_nest() -> Router<Database> {
    Router::new()
        .nest("/pollings", polling::get_nest())
}
