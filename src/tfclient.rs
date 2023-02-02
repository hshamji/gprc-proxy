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
use crate::prediction_service_client::PredictionServiceClient;
use crate::tensorflowserving::{PredictRequest, PredictResponse};
use http;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let a = tonic::client::GrpcService;
    let dst = "http://[::1]:8080";
    let mut client = prediction_service_client::PredictionServiceClient::connect(dst).await?;
    let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
    let mut c2 = PredictionServiceClient::new(conn.clone());
    let mut c3 = tonic::client::Grpc::new(conn);

    // let a = c3.ready();
    c3
        .ready()
        .await
        .map_err(|e| {
            tonic::Status::new(
                tonic::Code::Unknown,
                format!("Service was not ready: {}", e),
            )
        })?;
    let codec = tonic::codec::ProstCodec::default();
    let path = http::uri::PathAndQuery::from_static(
        "/tensorflow.serving.PredictionService/Predict",
    );
    let request = PredictRequest{
        model_spec: None,
        inputs: Default::default(),
        output_filter: vec![]
    };

    let r2 = tonic::IntoRequest::into_request(request);
    let resp: tonic::Response<PredictResponse> = c3.unary(r2, path, codec).await?;
    println!("Response: {:?}", resp.into_inner().outputs);



    // let response = client.predict(PredictRequest{
    // let response = c2.predict(PredictRequest{
    //     model_spec: None,
    //     inputs: Default::default(),
    //     output_filter: vec![]
    // })
    //     .await?;
    // println!("Response: {:?}", response.into_inner().outputs);

    Ok(())

}