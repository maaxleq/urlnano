use std::sync::Arc;

use axum::Extension;
use urlnano::{axum_extension::RepositoryExtension, http_handler, repository::InMemoryRepository};

static DEFAULT_HTTP_HOST: &str = "127.0.0.1";
static DEFAULT_HTTP_PORT: &str = "8000";

#[tokio::main]
async fn main() {
    let repository = InMemoryRepository::new();

    let app = axum::Router::new()
        .route("/set", axum::routing::post(http_handler::set_url))
        .route("/get/:key", axum::routing::get(http_handler::get_url))
        .route("/r/:key", axum::routing::get(http_handler::redirect))
        .layer(Extension::<RepositoryExtension>(Arc::new(repository)));

    let host = dotenv::var("HTTP_HOST").unwrap_or(DEFAULT_HTTP_HOST.to_string());
    let port = dotenv::var("HTTP_PORT").unwrap_or(DEFAULT_HTTP_PORT.to_string());

    let address = format!("{}:{}", host, port);

    println!("Server will listen on {}", address);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
