// pub mod tfserving {
//     tonic::include_proto!("tensorflowserving");
// }
//
// pub mod tensorflow {
//     tonic::include_proto!("tensorflow");
// }
//
// use tensorflow:

mod tensorflow;
mod tensorflowserving;

use tensorflowserving::prediction_service_client;
use crate::tensorflowserving::PredictRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let a = tonic::client::GrpcService;

    let mut client = prediction_service_client::PredictionServiceClient::connect("http://[::1]:8080").await?;

    let response = client.predict(PredictRequest{
        model_spec: None,
        inputs: Default::default(),
        output_filter: vec![]
    })
        .await?;
    println!("Response: {:?}", response.into_inner().outputs);

    Ok(())

}