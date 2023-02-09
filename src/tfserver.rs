#![allow(missing_docs, unused)]

mod tensorflow;
mod tensorflowserving_short;

use crate::tensorflow::TensorProto;
use crate::tensorflowserving_short::{
    ClassificationRequest, ClassificationResponse, GetModelMetadataRequest,
    GetModelMetadataResponse, MultiInferenceRequest, MultiInferenceResponse, PredictRequest,
    PredictResponse, RegressionRequest, RegressionResponse,
};
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Sub;
use std::task::{Context, Poll};
use tensorflowserving_short::prediction_service_server;
use tonic::codec::{Codec, DecodeBuf, Decoder, EnabledCompressionEncodings, EncodeBuf, Encoder};
use tonic::codegen::{Service, StdError, BoxFuture};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tonic::body::BoxBody;
use http_body::Body;
use tonic::server::NamedService;
use bytes::{Buf, Bytes, BytesMut};
use bytes::buf::BufMut;
use crate::tensorflowserving_short::prediction_service_server::{PredictionServiceServer, PredictionService};

struct IdentityCodec {}

impl Codec for IdentityCodec {
    type Encode = Vec<u8>;
    type Decode = Vec<u8>;
    type Encoder = IdentityCodec;
    type Decoder = IdentityCodec;

    fn encoder(&mut self) -> Self::Encoder {
        Self {}
    }

    fn decoder(&mut self) -> Self::Decoder {
        Self {}
    }
}

impl Encoder for IdentityCodec {
    type Item = Vec<u8>;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        // self.encode_impl(item, dst).expect("problem encoding");
        println!("Starting buffer: {:?}", dst.buf.to_vec());
        println!("Printing output items");
        for i in item {
            print!("{} ", i);
            dst.put_u8(i);
        }
        println!("Dst buffer is {:?}", dst.buf.to_vec());
        Ok(())
    }
}

impl Decoder for IdentityCodec {
    type Item = Vec<u8>;
    type Error = Status;

    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        // todo!()
        let out = src.chunk().to_vec();
        src.advance(out.len());

        println!("HS: Received response: {:?}", out);
        Ok(Some(out))
    }
}

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
        request: Request<Vec<u8>>,
    ) -> Result<Response<Vec<u8>>, Status> {
        // todo!()
        // let reply = PredictResponse {
        //     model_spec: None,
        //     outputs: HashMap::from([(
        //         String::from("some-response"),
        //         TensorProto {
        //             int_val: vec![1, 1, 1, 1],
        //             ..Default::default()
        //         },
        //     )]),
        // };
        let reply = vec![ 10, 11, 111, 110, 101, 116, 119, 111, 116, 104, 114, 101, 101, 18, 5, 58, 3, 3, 2, 1];
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
    let addr = "127.0.0.1:8081".parse()?;

    let pred_server = MyPredService::default();
    // let server = prediction_service_server::PredictionServiceServer::

    Server::builder()
        .add_service(PredictionServiceServer::new(pred_server))
        .serve(addr)
        .await?;

    Ok(())
}
