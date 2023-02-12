use std::net::SocketAddr;

use axum::Router;
use routes::article_routes::RegisterArtistRoutes;
use tower_http::cors::CorsLayer;

mod client;
mod routes;
mod tonic_proto_out;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .and_register_artist_routes()
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from((([127, 0, 0, 1]), 20490));
    tracing::debug!("Listening on {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
