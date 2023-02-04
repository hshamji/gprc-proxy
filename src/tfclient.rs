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

use std::collections::HashMap;
use bytes::{Buf, BufMut};
use tensorflowserving::prediction_service_client;
use crate::prediction_service_client::PredictionServiceClient;
use crate::tensorflowserving::{ClassificationResult, GetModelMetadataRequest, PredictRequest, PredictResponse};
use http;
use tonic::codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder};
use tonic::Status;
use crate::tensorflow::TensorProto;


struct IdentityCodec {}

impl Codec for IdentityCodec {
    type Encode = Vec<u8>;
    type Decode = Vec<u8>;
    type Encoder = IdentityCodec;
    type Decoder = IdentityCodec;

    fn encoder(&mut self) -> Self::Encoder {
        Self{}
    }

    fn decoder(&mut self) -> Self::Decoder {
        Self{}
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let a = tonic::client::GrpcService;
    let dst = "http://[::1]:8080";
    // let mut client = prediction_service_client::PredictionServiceClient::connect(dst).await?;
    let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
    // let mut c2 = PredictionServiceClient::new(conn.clone());
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
    // let mut codec = tonic::codec::ProstCodec::default();
    let identity_codec = IdentityCodec{};
    let path = http::uri::PathAndQuery::from_static(
        "/tensorflow.serving.PredictionService/Predict",
    );
    let request = PredictRequest{
        model_spec: None,
        inputs: HashMap::from([(String::from("onetwothree"), TensorProto{int_val: vec![1,2,3], ..Default::default()})]),
        output_filter: vec![]
    };
    // let request: Vec<u8> = vec![0, 0, 0, 0, 0, 18, 20, 10, 11, 111, 110, 101, 116, 119, 111, 116, 104, 114, 101, 101, 18, 5, 58, 3, 1, 2, 3];
    let request: Vec<u8> = vec![18, 20, 10, 11, 111, 110, 101, 116, 119, 111, 116, 104, 114, 101, 101, 18, 5, 58, 3, 1, 2, 3];

    let r2 = tonic::IntoRequest::into_request(request);
    // let r2 = tonic::IntoRequest::into_request(vec![18, 20, 10, 11, 111, 110, 101, 116, 119, 111, 116, 104, 114, 101, 101, 18, 5, 58, 3, 1, 2, 3]);
    // let resp: tonic::Response<Vec<u8>> = c3.unary(r2, path, identity_codec).await?;
    let resp: tonic::Response<Vec<u8>> = c3.unary(r2, path, codec).await?;
    // let resp: tonic::Response<PredictResponse> = c3.unary(r2, path, codec).await?;
    println!("Response: {:?}", resp.into_inner());


    let decoded: PredictResponse = prost::Message::decode(Buf:: resp.into_inner()).expect("error decoding");
    println!("Decoded: {:?}", decoded.outputs);
    // let mut buf = tonic::codec::EncodeBuf::
    // codec.encoder().encode(request, &mut buf );


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