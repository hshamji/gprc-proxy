mod tensorflowserving;

use tonic::transport::Server;
use tensorflowserving::prediction_service_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1}:8081".parse().unwrap();
    let server = prediction_service_server::PredictionServiceServer::;

    Server::builder()
        .add_service()
        .serve(addr)
        .await?;

    Ok(())
}