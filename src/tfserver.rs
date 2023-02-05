#![allow(missing_docs, unused)]

mod tensorflow;
mod tensorflowserving;

use crate::prediction_service_server::{PredictionService, PredictionServiceServer};
use crate::tensorflow::TensorProto;
use crate::tensorflowserving::{
    ClassificationRequest, ClassificationResponse, GetModelMetadataRequest,
    GetModelMetadataResponse, MultiInferenceRequest, MultiInferenceResponse, PredictRequest,
    PredictResponse, RegressionRequest, RegressionResponse,
};
use std::collections::HashMap;
use std::task::{Context, Poll};
use tensorflowserving::prediction_service_server;
use tonic::codec::EnabledCompressionEncodings;
use tonic::codegen::Service;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// struct StaticResponder {
//     inner: _Inner<T>,
//     accept_compression_encodings: EnabledCompressionEncodings,
//     send_compression_encodings: EnabledCompressionEncodings,
// }

// impl Service<T> for StaticResponder {
//     type Response = http::Response<tonic::body::BoxBody>;
//     type Error = std::convert::Infallible;
//     type Future = BoxFuture<Self::Response, Self::Error>;
//     fn poll_ready(
//         &mut self,
//         _cx: &mut Context<'_>,
//     ) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }
//
//     fn call(&mut self, req: T) -> Self::Future {
//         todo!()
//     }
// }

#[derive(Debug, Default)]
pub struct MyPredService {}

#[tonic::async_trait]
impl PredictionService for MyPredService {
    async fn classify(
        &self,
        request: Request<ClassificationRequest>,
    ) -> Result<Response<ClassificationResponse>, Status> {
        todo!()
    }

    async fn regress(
        &self,
        request: Request<RegressionRequest>,
    ) -> Result<Response<RegressionResponse>, Status> {
        todo!()
    }

    async fn predict(
        &self,
        request: Request<PredictRequest>,
    ) -> Result<Response<PredictResponse>, Status> {
        // todo!()
        let reply = PredictResponse {
            model_spec: None,
            outputs: HashMap::from([(
                String::from("some-response"),
                TensorProto {
                    int_val: vec![1, 1, 1, 1],
                    ..Default::default()
                },
            )]),
        };
        Ok(Response::new(reply))
    }

    async fn multi_inference(
        &self,
        request: Request<MultiInferenceRequest>,
    ) -> Result<Response<MultiInferenceResponse>, Status> {
        todo!()
    }

    async fn get_model_metadata(
        &self,
        request: Request<GetModelMetadataRequest>,
    ) -> Result<Response<GetModelMetadataResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use server codegen to create static responder ...
    println!("About to get socket");
    // let addr = "[::1}:8081".parse(); //.unwrap();
    let addr = "127.0.0.1:8081".parse();
    // if let Err(e) = addr {
    //     println!("Error is {:?}", e);
    // }

    // if addr.is_err() {
    //     println!("Error: {}", addr.into_err());
    //     addr.map_err(|e| {
    //         println!("Error is {}", e);
    //         panic!("error getting socket");
    //     })
    // };
    let addr = addr.unwrap();
    println!("Obtained socket address");
    let pred_server = MyPredService::default();
    // let server = prediction_service_server::PredictionServiceServer::

    Server::builder()
        .add_service(PredictionServiceServer::new(pred_server))
        .serve(addr)
        .await?;

    Ok(())
}
