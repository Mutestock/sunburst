mod utils;
mod routes;
mod handler;
mod tonic_proto_out;
use std::net::SocketAddr;

use tonic::transport::Server;

use crate::{utils::config::CONFIG, routes::article_routes::ArticleRouter};
use crate::tonic_proto_out::article_service_server::ArticleServiceServer;



#[tokio::main]
async fn main() -> Result<(), tonic::transport::Error> {
    let addr: SocketAddr = format!("{}:{}",&CONFIG.distributor.host, &CONFIG.distributor.port)
    .parse()
    .unwrap();


    println!(
        "Server running on: {}:{}",
        &CONFIG.distributor.host, &CONFIG.distributor.port
    );

      Server::builder()
          .add_service(ArticleServiceServer::new(ArticleRouter{}))
          .serve(addr)
          .await?;

    Ok(())
}
