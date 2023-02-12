mod client;
mod routes;
mod utils;
mod tonic_proto_out;


#[tokio::main]
async fn main() -> Result<(), tonic::transport::Error> {
    println!("Hello, world!");
    Ok(())
}
