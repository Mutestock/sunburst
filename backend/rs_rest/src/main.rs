use axum::Router;
use routes::article_routes::RegisterArticleRoutes;
use tower_http::cors::CorsLayer;

use crate::{routes::basic_routes::RegisterBasicRoutes, utils::config::RS_REST_URL};

mod client;
mod routes;
mod tonic_proto_out;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .and_register_article_routes()
        .and_register_basic_routes()
        .layer(CorsLayer::permissive());

    tracing::debug!("Listening on {}", &RS_REST_URL.to_string());
    axum::Server::bind(&RS_REST_URL.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
