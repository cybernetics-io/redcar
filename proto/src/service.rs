#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutResponse {
    #[prost(message, optional, tag = "1")]
    pub kv: ::core::option::Option<super::txn::KeyValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "3")]
    pub ignore_value: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "2")]
    pub limit: i64,
    #[prost(enumeration = "range_request::SortOrder", tag = "3")]
    pub sort_order: i32,
    #[prost(enumeration = "range_request::SortTarget", tag = "4")]
    pub sort_target: i32,
    #[prost(bool, tag = "5")]
    pub keys_only: bool,
    #[prost(bool, tag = "6")]
    pub count_only: bool,
}
/// Nested message and enum types in `RangeRequest`.
pub mod range_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SortOrder {
        None = 0,
        Ascend = 1,
        Descend = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SortTarget {
        Key = 0,
        Version = 1,
        Create = 2,
        Mod = 3,
        Value = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeResponse {
    #[prost(message, repeated, tag = "1")]
    pub kvs: ::prost::alloc::vec::Vec<super::txn::KeyValue>,
    #[prost(bool, tag = "2")]
    pub more: bool,
    #[prost(int64, tag = "3")]
    pub count: i64,
}
#[doc = r" Generated client implementations."]
pub mod kv_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct KvClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl KvClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> KvClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> KvClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            KvClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn put(
            &mut self,
            request: impl tonic::IntoRequest<super::PutRequest>,
        ) -> Result<tonic::Response<super::PutResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/service.KV/Put");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn range(
            &mut self,
            request: impl tonic::IntoRequest<super::RangeRequest>,
        ) -> Result<tonic::Response<super::RangeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/service.KV/Range");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod kv_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with KvServer."]
    #[async_trait]
    pub trait Kv: Send + Sync + 'static {
        async fn put(
            &self,
            request: tonic::Request<super::PutRequest>,
        ) -> Result<tonic::Response<super::PutResponse>, tonic::Status>;
        async fn range(
            &self,
            request: tonic::Request<super::RangeRequest>,
        ) -> Result<tonic::Response<super::RangeResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct KvServer<T: Kv> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Kv> KvServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for KvServer<T>
    where
        T: Kv,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/service.KV/Put" => {
                    #[allow(non_camel_case_types)]
                    struct PutSvc<T: Kv>(pub Arc<T>);
                    impl<T: Kv> tonic::server::UnaryService<super::PutRequest> for PutSvc<T> {
                        type Response = super::PutResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).put(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service.KV/Range" => {
                    #[allow(non_camel_case_types)]
                    struct RangeSvc<T: Kv>(pub Arc<T>);
                    impl<T: Kv> tonic::server::UnaryService<super::RangeRequest> for RangeSvc<T> {
                        type Response = super::RangeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RangeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).range(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RangeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Kv> Clone for KvServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Kv> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Kv> tonic::transport::NamedService for KvServer<T> {
        const NAME: &'static str = "service.KV";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchRequest {
    #[prost(oneof = "watch_request::WatchType", tags = "1, 2")]
    pub watch_type: ::core::option::Option<watch_request::WatchType>,
}
/// Nested message and enum types in `WatchRequest`.
pub mod watch_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WatchType {
        #[prost(message, tag = "1")]
        Create(super::WatchCreate),
        #[prost(message, tag = "2")]
        Cancel(super::WatchCancel),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchCancel {
    #[prost(int64, tag = "1")]
    pub watch_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchCreate {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "watch_create::FilterType", repeated, tag = "2")]
    pub filters: ::prost::alloc::vec::Vec<i32>,
    #[prost(int64, tag = "3")]
    pub watch_id: i64,
}
/// Nested message and enum types in `WatchCreate`.
pub mod watch_create {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FilterType {
        Put = 0,
        Delete = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WatchResponse {
    #[prost(int64, tag = "1")]
    pub watch_id: i64,
    #[prost(bool, tag = "2")]
    pub created: bool,
    #[prost(bool, tag = "3")]
    pub canceled: bool,
    #[prost(string, tag = "4")]
    pub cancel_reason: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub events: ::prost::alloc::vec::Vec<super::txn::Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObserveRequest {
    #[prost(oneof = "observe_request::ObserveType", tags = "1, 2")]
    pub observe_type: ::core::option::Option<observe_request::ObserveType>,
}
/// Nested message and enum types in `ObserveRequest`.
pub mod observe_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObserveType {
        #[prost(message, tag = "1")]
        Create(super::ObserveCreate),
        #[prost(message, tag = "2")]
        Cancel(super::ObserveCancel),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObserveCancel {
    #[prost(int64, tag = "1")]
    pub observe_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObserveCreate {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag = "2")]
    pub observe_id: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObserveResponse {
    #[prost(int64, tag = "1")]
    pub observe_id: i64,
    #[prost(bool, tag = "2")]
    pub created: bool,
    #[prost(bool, tag = "3")]
    pub canceled: bool,
    #[prost(string, tag = "4")]
    pub cancel_reason: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub events: ::prost::alloc::vec::Vec<super::txn::Event>,
}
#[doc = r" Generated client implementations."]
pub mod event_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct EventClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EventClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> EventClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> EventClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            EventClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoRequest<super::WatchRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::WatchResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/service.Event/Watch");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn observe(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ObserveRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ObserveResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/service.Event/Observe");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod event_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with EventServer."]
    #[async_trait]
    pub trait Event: Send + Sync + 'static {
        #[doc = "Server streaming response type for the Watch method."]
        type WatchStream: futures_core::Stream<Item = Result<super::WatchResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn watch(
            &self,
            request: tonic::Request<super::WatchRequest>,
        ) -> Result<tonic::Response<Self::WatchStream>, tonic::Status>;
        #[doc = "Server streaming response type for the Observe method."]
        type ObserveStream: futures_core::Stream<Item = Result<super::ObserveResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn observe(
            &self,
            request: tonic::Request<tonic::Streaming<super::ObserveRequest>>,
        ) -> Result<tonic::Response<Self::ObserveStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct EventServer<T: Event> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Event> EventServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EventServer<T>
    where
        T: Event,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/service.Event/Watch" => {
                    #[allow(non_camel_case_types)]
                    struct WatchSvc<T: Event>(pub Arc<T>);
                    impl<T: Event> tonic::server::ServerStreamingService<super::WatchRequest> for WatchSvc<T> {
                        type Response = super::WatchResponse;
                        type ResponseStream = T::WatchStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WatchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).watch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service.Event/Observe" => {
                    #[allow(non_camel_case_types)]
                    struct ObserveSvc<T: Event>(pub Arc<T>);
                    impl<T: Event> tonic::server::StreamingService<super::ObserveRequest> for ObserveSvc<T> {
                        type Response = super::ObserveResponse;
                        type ResponseStream = T::ObserveStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::ObserveRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).observe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ObserveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Event> Clone for EventServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Event> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Event> tonic::transport::NamedService for EventServer<T> {
        const NAME: &'static str = "service.Event";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {}
#[doc = r" Generated client implementations."]
pub mod keepalive_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct KeepaliveClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl KeepaliveClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> KeepaliveClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> KeepaliveClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            KeepaliveClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn heartbeat(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::HeartbeatRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::txn::Message>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/service.Keepalive/Heartbeat");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::txn::Message>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/service.Keepalive/Status");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod keepalive_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with KeepaliveServer."]
    #[async_trait]
    pub trait Keepalive: Send + Sync + 'static {
        #[doc = "Server streaming response type for the Heartbeat method."]
        type HeartbeatStream: futures_core::Stream<Item = Result<super::super::txn::Message, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn heartbeat(
            &self,
            request: tonic::Request<tonic::Streaming<super::HeartbeatRequest>>,
        ) -> Result<tonic::Response<Self::HeartbeatStream>, tonic::Status>;
        #[doc = "Server streaming response type for the Status method."]
        type StatusStream: futures_core::Stream<Item = Result<super::super::txn::Message, tonic::Status>>
            + Send
            + Sync
            + 'static;
        async fn status(
            &self,
            request: tonic::Request<super::StatusRequest>,
        ) -> Result<tonic::Response<Self::StatusStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct KeepaliveServer<T: Keepalive> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Keepalive> KeepaliveServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for KeepaliveServer<T>
    where
        T: Keepalive,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/service.Keepalive/Heartbeat" => {
                    #[allow(non_camel_case_types)]
                    struct HeartbeatSvc<T: Keepalive>(pub Arc<T>);
                    impl<T: Keepalive> tonic::server::StreamingService<super::HeartbeatRequest> for HeartbeatSvc<T> {
                        type Response = super::super::txn::Message;
                        type ResponseStream = T::HeartbeatStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::HeartbeatRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).heartbeat(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HeartbeatSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/service.Keepalive/Status" => {
                    #[allow(non_camel_case_types)]
                    struct StatusSvc<T: Keepalive>(pub Arc<T>);
                    impl<T: Keepalive> tonic::server::ServerStreamingService<super::StatusRequest> for StatusSvc<T> {
                        type Response = super::super::txn::Message;
                        type ResponseStream = T::StatusStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Keepalive> Clone for KeepaliveServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Keepalive> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Keepalive> tonic::transport::NamedService for KeepaliveServer<T> {
        const NAME: &'static str = "service.Keepalive";
    }
}
