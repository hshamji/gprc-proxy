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
use std::fmt::Debug;
use std::ops::Sub;
use std::task::{Context, Poll};
use tensorflowserving::prediction_service_server;
use tonic::codec::{Codec, DecodeBuf, Decoder, EnabledCompressionEncodings, EncodeBuf, Encoder};
use tonic::codegen::{Service, StdError, BoxFuture};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tonic::body::BoxBody;
use http_body::Body;
use tonic::server::NamedService;
use bytes::{Buf, Bytes, BytesMut};
use bytes::buf::BufMut;

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
        out = src.chunk().to_vec();
        src.advance(out.len());
        // let out = src.buf.to_vec();
        // Does the buffer need to be consumed? In the calling decode it seems to show that it is still in ReadHeader state
        // let mut out: Vec<u8> = vec![];
        // src.chunks_vectored(&mut out);
        // let b = src.copy_to_bytes(src.len).to_vec();

        println!("HS: Received response: {:?}", out);
        // let out: Self::Item = src.into();
        Ok(Some(out))
    }
}

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

struct StaticResponder {}

// impl<B> tonic::codegen::Service<http::Request<B>> for StaticResponder
//     where
//         B: Body + Send + 'static,
//         B::Error: Into<StdError> + Send + 'static,
// {
//     type Response = http::Response<tonic::body::BoxBody>;
//     type Error = std::convert::Infallible;
//     type Future = BoxFuture<Self::Response, Self::Error>;
//     fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }
//     fn call(&mut self, req: http::Request<B>) -> Self::Future {
//                 #[allow(non_camel_case_types)]
//                 struct SubType {};
//                 impl tonic::server::UnaryService<Vec<u8>> for SubType {
//                     type Response = Vec<u8>;
//                     type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
//
//                     fn call(&mut self, request: Request<Vec<u8>>) -> Self::Future {
//                         todo!()
//                     }
//                 }
//     }
//                 for ClassifySvc<T>
//                 {
//                     type Response = super::ClassificationResponse;
//                     type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
//                     fn call(
//                         &mut self,
//                         request: tonic::Request<super::ClassificationRequest>,
//                     ) -> Self::Future {
//                         let inner = self.0.clone();
//                         let fut = async move { (*inner).classify(request).await };
//                         Box::pin(fut)
//                     }
//                 }
//                 let accept_compression_encodings = self.accept_compression_encodings;
//                 let send_compression_encodings = self.send_compression_encodings;
//                 let inner = self.inner.clone();
//                 let fut = async move {
//                     let inner = inner.0;
//                     let method = ClassifySvc(inner);
//                     let codec = tonic::codec::ProstCodec::default();
//                     let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
//                         accept_compression_encodings,
//                         send_compression_encodings,
//                     );
//                     let res = grpc.unary(method, req).await;
//                     Ok(res)
//                 };
//                 Box::pin(fut)
//             }

struct MyService{}

impl <B>tonic::codegen::Service<http::Request<B>> for MyService
    where B: http_body::Body + Send + Debug,
          B::Error: Into<StdError> + Send + 'static,
{
    type Response = http::Response<BoxBody>;
    type Error = std::convert::Infallible;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // todo!()
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        // todo!()
        println!("HS: Request: {:?}", req.body());

        struct UnaryWrapper{}
        impl tonic::server::UnaryService<Vec<u8>> for UnaryWrapper {
            type Response = Vec<u8>;
            type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;

            fn call(&mut self, request: Request<Vec<u8>>) -> Self::Future {
                // todo!()
                // Might fail on the type below, needs to be a specific Result?
                let fut = async { Ok(Response::new((vec![ 10, 11, 111, 110, 101, 116, 119, 111, 116, 104, 114, 101, 101, 18, 5, 58, 3, 3, 2, 1])))};
                Box::pin(fut)
            }
        }

        let fut = async {
            let mut grpc = tonic::server::Grpc::new(IdentityCodec{});
            let resp  = grpc.unary(UnaryWrapper{}, req).await;
            Ok(resp)
        };
        Box::pin(fut)
    }
}

impl Clone for MyService{
    fn clone(&self) -> Self {
        // todo!()
        println!("Cloning");
        Self{}
    }
}

impl NamedService for MyService{ const NAME: &'static str = "someService"; }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8081".parse()?;

    // let pred_server = MyPredService::default();
    // let server = prediction_service_server::PredictionServiceServer::

    Server::builder()
        .add_service(MyService{})
        .serve(addr)
        .await?;

    Ok(())
}
