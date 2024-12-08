use crate::model::database::Database;
use anyhow::{Context, Result};
use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod endpoints;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(["http://localhost:5173".parse().unwrap()])
        .allow_methods(Any)
        .allow_headers(Any);

    let database = Database::from_env();
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let router: Router = Router::new()
        .nest("/", endpoints::get_nest())
        .layer(cors)
        .with_state(database);

    axum::serve(listener, router)
        .await
        .context("failed to serve")?;

    Ok(())
}
