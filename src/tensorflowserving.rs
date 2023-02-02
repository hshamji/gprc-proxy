use crate::tensorflow::TensorProto;
use crate::tensorflow::Example;
/// Specifies one or more fully independent input Examples.
/// See examples at:
///      <https://github.com/tensorflow/tensorflow/blob/master/tensorflow/core/example/example.proto>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExampleList {
    #[prost(message, repeated, tag = "1")]
    pub examples: ::prost::alloc::vec::Vec<Example>,
}
/// Specifies one or more independent input Examples, with a common context
/// Example.
///
/// The common use case for context is to cleanly and optimally specify some
/// features that are common across multiple examples.
///
/// See example below with a search query as the context and multiple restaurants
/// to perform some inference on.
///
/// context: {
///    features: {
///      feature: {
///        key  : "query"
///        value: {
///          bytes_list: {
///            value: [ "pizza" ]
///          }
///        }
///      }
///    }
/// }
/// examples: {
///    features: {
///      feature: {
///        key  : "cuisine"
///        value: {
///          bytes_list: {
///            value: [ "Pizzeria" ]
///          }
///        }
///      }
///    }
/// }
/// examples: {
///    features: {
///      feature: {
///        key  : "cuisine"
///        value: {
///          bytes_list: {
///            value: [ "Taqueria" ]
///          }
///        }
///      }
///    }
/// }
///
/// Implementations of ExampleListWithContext merge the context Example into each
/// of the Examples. Note that feature keys must not be duplicated between the
/// Examples and context Example, or the behavior is undefined.
///
/// See also:
///      tensorflow/core/example/example.proto
///      <https://developers.google.com/protocol-buffers/docs/proto3#maps>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExampleListWithContext {
    #[prost(message, repeated, tag = "1")]
    pub examples: ::prost::alloc::vec::Vec<Example>,
    #[prost(message, optional, tag = "2")]
    pub context: ::core::option::Option<Example>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    #[prost(oneof = "input::Kind", tags = "1, 2")]
    pub kind: ::core::option::Option<input::Kind>,
}
/// Nested message and enum types in `Input`.
pub mod input {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag = "1")]
        ExampleList(super::ExampleList),
        #[prost(message, tag = "2")]
        ExampleListWithContext(super::ExampleListWithContext),
    }
}
/// Metadata for an inference request such as the model name and version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelSpec {
    /// Required servable name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A named signature to evaluate. If unspecified, the default signature will
    /// be used.
    #[prost(string, tag = "3")]
    pub signature_name: ::prost::alloc::string::String,
    /// Optional choice of which version of the model to use.
    ///
    /// Recommended to be left unset in the common case. Should be specified only
    /// when there is a strong version consistency requirement.
    ///
    /// When left unspecified, the system will serve the best available version.
    /// This is typically the latest version, though during version transitions,
    /// notably when serving on a fleet of instances, may be either the previous or
    /// new version.
    #[prost(oneof = "model_spec::VersionChoice", tags = "2, 4")]
    pub version_choice: ::core::option::Option<model_spec::VersionChoice>,
}
/// Nested message and enum types in `ModelSpec`.
pub mod model_spec {
    /// Optional choice of which version of the model to use.
    ///
    /// Recommended to be left unset in the common case. Should be specified only
    /// when there is a strong version consistency requirement.
    ///
    /// When left unspecified, the system will serve the best available version.
    /// This is typically the latest version, though during version transitions,
    /// notably when serving on a fleet of instances, may be either the previous or
    /// new version.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VersionChoice {
        /// Use this specific version number.
        #[prost(message, tag = "2")]
        Version(i64),
        /// Use the version associated with the given label.
        #[prost(string, tag = "4")]
        VersionLabel(::prost::alloc::string::String),
    }
}
/// A single class.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Class {
    /// Label or name of the class.
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
    /// Score for this class (e.g., the probability the item belongs to this
    /// class). As per the proto3 default-value semantics, if the score is missing,
    /// it should be treated as 0.
    #[prost(float, tag = "2")]
    pub score: f32,
}
/// List of classes for a single item (tensorflow.Example).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Classifications {
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
}
/// Contains one result per input example, in the same order as the input in
/// ClassificationRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationResult {
    #[prost(message, repeated, tag = "1")]
    pub classifications: ::prost::alloc::vec::Vec<Classifications>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationRequest {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Input data.
    #[prost(message, optional, tag = "2")]
    pub input: ::core::option::Option<Input>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationResponse {
    /// Effective Model Specification used for classification.
    #[prost(message, optional, tag = "2")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Result of the classification.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<ClassificationResult>,
}
/// Message returned for "signature_def" field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignatureDefMap {
    #[prost(map = "string, message", tag = "1")]
    pub signature_def: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        crate::tensorflow::SignatureDef,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelMetadataRequest {
    /// Model Specification indicating which model we are querying for metadata.
    /// If version is not specified, will use the latest (numerical) version.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Metadata fields to get. Currently supported: "signature_def".
    #[prost(string, repeated, tag = "2")]
    pub metadata_field: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelMetadataResponse {
    /// Model Specification indicating which model this metadata belongs to.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Map of metadata field name to metadata field. The options for metadata
    /// field name are listed in GetModelMetadataRequest. Currently supported:
    /// "signature_def".
    #[prost(map = "string, message", tag = "2")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost_types::Any,
    >,
}
/// Regression result for a single item (tensorflow.Example).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Regression {
    #[prost(float, tag = "1")]
    pub value: f32,
}
/// Contains one result per input example, in the same order as the input in
/// RegressionRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegressionResult {
    #[prost(message, repeated, tag = "1")]
    pub regressions: ::prost::alloc::vec::Vec<Regression>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegressionRequest {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Input data.
    #[prost(message, optional, tag = "2")]
    pub input: ::core::option::Option<Input>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegressionResponse {
    /// Effective Model Specification used for regression.
    #[prost(message, optional, tag = "2")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<RegressionResult>,
}
/// Inference request such as classification, regression, etc...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferenceTask {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    /// All ModelSpecs in a MultiInferenceRequest must access the same model name.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Signature's method_name. Should be one of the method names defined in
    /// third_party/tensorflow/python/saved_model/signature_constants.py.
    /// e.g. "tensorflow/serving/classify".
    #[prost(string, tag = "2")]
    pub method_name: ::prost::alloc::string::String,
}
/// Inference result, matches the type of request or is an error.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferenceResult {
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    #[prost(oneof = "inference_result::Result", tags = "2, 3")]
    pub result: ::core::option::Option<inference_result::Result>,
}
/// Nested message and enum types in `InferenceResult`.
pub mod inference_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "2")]
        ClassificationResult(super::ClassificationResult),
        #[prost(message, tag = "3")]
        RegressionResult(super::RegressionResult),
    }
}
/// Inference request containing one or more requests.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiInferenceRequest {
    /// Inference tasks.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<InferenceTask>,
    /// Input data.
    #[prost(message, optional, tag = "2")]
    pub input: ::core::option::Option<Input>,
}
/// Inference request containing one or more responses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiInferenceResponse {
    /// List of results; one for each InferenceTask in the request, returned in the
    /// same order as the request.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<InferenceResult>,
}
/// PredictRequest specifies which TensorFlow model to run, as well as
/// how inputs are mapped to tensors and how outputs are filtered before
/// returning to user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictRequest {
    /// Model Specification. If version is not specified, will use the latest
    /// (numerical) version.
    #[prost(message, optional, tag = "1")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Input tensors.
    /// Names of input tensor are alias names. The mapping from aliases to real
    /// input tensor names is stored in the SavedModel export as a prediction
    /// SignatureDef under the 'inputs' field.
    #[prost(map = "string, message", tag = "2")]
    pub inputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        TensorProto,
    >,
    /// Output filter.
    /// Names specified are alias names. The mapping from aliases to real output
    /// tensor names is stored in the SavedModel export as a prediction
    /// SignatureDef under the 'outputs' field.
    /// Only tensors specified here will be run/fetched and returned, with the
    /// exception that when none is specified, all tensors specified in the
    /// named signature will be run/fetched and returned.
    #[prost(string, repeated, tag = "3")]
    pub output_filter: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response for PredictRequest on successful run.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictResponse {
    /// Effective Model Specification used to process PredictRequest.
    #[prost(message, optional, tag = "2")]
    pub model_spec: ::core::option::Option<ModelSpec>,
    /// Output tensors.
    #[prost(map = "string, message", tag = "1")]
    pub outputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        TensorProto,
    >,
}
/// Generated client implementations.
pub mod prediction_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// open source marker; do not remove
    /// PredictionService provides access to machine-learned models loaded by
    /// model_servers.
    #[derive(Debug, Clone)]
    pub struct PredictionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PredictionServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PredictionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PredictionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PredictionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Classify.
        pub async fn classify(
            &mut self,
            request: impl tonic::IntoRequest<super::ClassificationRequest>,
        ) -> Result<tonic::Response<super::ClassificationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/Classify",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Regress.
        pub async fn regress(
            &mut self,
            request: impl tonic::IntoRequest<super::RegressionRequest>,
        ) -> Result<tonic::Response<super::RegressionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/Regress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Predict -- provides access to loaded TensorFlow model.
        pub async fn predict(
            &mut self,
            request: impl tonic::IntoRequest<super::PredictRequest>,
        ) -> Result<tonic::Response<super::PredictResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/Predict",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// MultiInference API for multi-headed models.
        pub async fn multi_inference(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiInferenceRequest>,
        ) -> Result<tonic::Response<super::MultiInferenceResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/MultiInference",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GetModelMetadata - provides access to metadata for loaded models.
        pub async fn get_model_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelMetadataRequest>,
        ) -> Result<tonic::Response<super::GetModelMetadataResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tensorflow.serving.PredictionService/GetModelMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod prediction_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with PredictionServiceServer.
    #[async_trait]
    pub trait PredictionService: Send + Sync + 'static {
        /// Classify.
        async fn classify(
            &self,
            request: tonic::Request<super::ClassificationRequest>,
        ) -> Result<tonic::Response<super::ClassificationResponse>, tonic::Status>;
        /// Regress.
        async fn regress(
            &self,
            request: tonic::Request<super::RegressionRequest>,
        ) -> Result<tonic::Response<super::RegressionResponse>, tonic::Status>;
        /// Predict -- provides access to loaded TensorFlow model.
        async fn predict(
            &self,
            request: tonic::Request<super::PredictRequest>,
        ) -> Result<tonic::Response<super::PredictResponse>, tonic::Status>;
        /// MultiInference API for multi-headed models.
        async fn multi_inference(
            &self,
            request: tonic::Request<super::MultiInferenceRequest>,
        ) -> Result<tonic::Response<super::MultiInferenceResponse>, tonic::Status>;
        /// GetModelMetadata - provides access to metadata for loaded models.
        async fn get_model_metadata(
            &self,
            request: tonic::Request<super::GetModelMetadataRequest>,
        ) -> Result<tonic::Response<super::GetModelMetadataResponse>, tonic::Status>;
    }
    /// open source marker; do not remove
    /// PredictionService provides access to machine-learned models loaded by
    /// model_servers.
    #[derive(Debug)]
    pub struct PredictionServiceServer<T: PredictionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: PredictionService> PredictionServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PredictionServiceServer<T>
    where
        T: PredictionService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/tensorflow.serving.PredictionService/Classify" => {
                    #[allow(non_camel_case_types)]
                    struct ClassifySvc<T: PredictionService>(pub Arc<T>);
                    impl<
                        T: PredictionService,
                    > tonic::server::UnaryService<super::ClassificationRequest>
                    for ClassifySvc<T> {
                        type Response = super::ClassificationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClassificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).classify(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClassifySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tensorflow.serving.PredictionService/Regress" => {
                    #[allow(non_camel_case_types)]
                    struct RegressSvc<T: PredictionService>(pub Arc<T>);
                    impl<
                        T: PredictionService,
                    > tonic::server::UnaryService<super::RegressionRequest>
                    for RegressSvc<T> {
                        type Response = super::RegressionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegressionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).regress(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegressSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tensorflow.serving.PredictionService/Predict" => {
                    #[allow(non_camel_case_types)]
                    struct PredictSvc<T: PredictionService>(pub Arc<T>);
                    impl<
                        T: PredictionService,
                    > tonic::server::UnaryService<super::PredictRequest>
                    for PredictSvc<T> {
                        type Response = super::PredictResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PredictRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).predict(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PredictSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tensorflow.serving.PredictionService/MultiInference" => {
                    #[allow(non_camel_case_types)]
                    struct MultiInferenceSvc<T: PredictionService>(pub Arc<T>);
                    impl<
                        T: PredictionService,
                    > tonic::server::UnaryService<super::MultiInferenceRequest>
                    for MultiInferenceSvc<T> {
                        type Response = super::MultiInferenceResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MultiInferenceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).multi_inference(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MultiInferenceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/tensorflow.serving.PredictionService/GetModelMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct GetModelMetadataSvc<T: PredictionService>(pub Arc<T>);
                    impl<
                        T: PredictionService,
                    > tonic::server::UnaryService<super::GetModelMetadataRequest>
                    for GetModelMetadataSvc<T> {
                        type Response = super::GetModelMetadataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetModelMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_model_metadata(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetModelMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: PredictionService> Clone for PredictionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: PredictionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PredictionService> tonic::server::NamedService
    for PredictionServiceServer<T> {
        const NAME: &'static str = "tensorflow.serving.PredictionService";
    }
}
