mod caching;
mod handler;
mod routes;
mod tonic_proto_out;
mod utils;

use std::net::SocketAddr;

use sunburst_utils::config::is_production;
use tonic::transport::Server;

use crate::routes::basic_routes::BasicRouter;
use crate::tonic_proto_out::article_service_server::ArticleServiceServer;
use crate::tonic_proto_out::basic_service_server::BasicServiceServer;
use crate::{routes::article_routes::ArticleRouter, utils::config::CONFIG};

#[tokio::main]
async fn main() -> Result<(), tonic::transport::Error> {
    caching::start_sequence::start_sequence().await;
    let host: &String;
    let port: &u16;

    match is_production() {
        true => {
            host = &CONFIG.distributor.prod.host;
            port = &CONFIG.distributor.prod.port;
        }
        false => {
            host = &CONFIG.distributor.dev.host;
            port = &CONFIG.distributor.dev.port;
        }
    };

    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    println!("Server running on: {}:{}", host, port);

    Server::builder()
        .add_service(ArticleServiceServer::new(ArticleRouter {}))
        .add_service(BasicServiceServer::new(BasicRouter {}))
        .serve(addr)
        .await?;

    Ok(())
}
