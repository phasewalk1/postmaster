#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sent_at: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub text: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgInTransit {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub text: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SendResponse {
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sent_at: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgRequest {
    #[prost(string, tag = "1")]
    pub message_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct MsgResponse {
    #[prost(message, optional, tag = "1")]
    pub msg: ::core::option::Option<Msg>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AllMsgsRequest {
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct SentMsgsRequest {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ReceivedMsgsRequest {
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Thread {
    #[prost(string, tag = "1")]
    pub peer1: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub peer2: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub created_at: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub msgs: ::prost::alloc::vec::Vec<Msg>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ThreadRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct ThreadResponse {
    #[prost(message, repeated, tag = "1")]
    pub msgs: ::prost::alloc::vec::Vec<Msg>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CreateThreadRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub other_user_id: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CreateThreadResponse {
    #[prost(string, tag = "1")]
    pub thread_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod messenger_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MessengerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MessengerClient<tonic::transport::Channel> {
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
    impl<T> MessengerClient<T>
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
        ) -> MessengerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MessengerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn send_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgInTransit>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/messenger.Messenger/SendMsg");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_msg(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRequest>,
        ) -> Result<tonic::Response<super::Msg>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/messenger.Messenger/GetMsg");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_all(
            &mut self,
            request: impl tonic::IntoRequest<super::AllMsgsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Msg>>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/messenger.Messenger/GetAll");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_sent_msgs(
            &mut self,
            request: impl tonic::IntoRequest<super::SentMsgsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Msg>>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/messenger.Messenger/GetSentMsgs");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_received_msgs(
            &mut self,
            request: impl tonic::IntoRequest<super::ReceivedMsgsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Msg>>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/messenger.Messenger/GetReceivedMsgs");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        pub async fn get_thread(
            &mut self,
            request: impl tonic::IntoRequest<super::ThreadRequest>,
        ) -> Result<tonic::Response<super::ThreadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/messenger.Messenger/GetThread");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_thread(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateThreadRequest>,
        ) -> Result<tonic::Response<super::CreateThreadResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/messenger.Messenger/CreateThread");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod messenger_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MessengerServer.
    #[async_trait]
    pub trait Messenger: Send + Sync + 'static {
        async fn send_msg(
            &self,
            request: tonic::Request<super::MsgInTransit>,
        ) -> Result<tonic::Response<super::SendResponse>, tonic::Status>;
        async fn get_msg(
            &self,
            request: tonic::Request<super::MsgRequest>,
        ) -> Result<tonic::Response<super::Msg>, tonic::Status>;
        ///Server streaming response type for the GetAll method.
        type GetAllStream: futures_core::Stream<Item = Result<super::Msg, tonic::Status>>
            + Send
            + 'static;
        async fn get_all(
            &self,
            request: tonic::Request<super::AllMsgsRequest>,
        ) -> Result<tonic::Response<Self::GetAllStream>, tonic::Status>;
        ///Server streaming response type for the GetSentMsgs method.
        type GetSentMsgsStream: futures_core::Stream<Item = Result<super::Msg, tonic::Status>>
            + Send
            + 'static;
        async fn get_sent_msgs(
            &self,
            request: tonic::Request<super::SentMsgsRequest>,
        ) -> Result<tonic::Response<Self::GetSentMsgsStream>, tonic::Status>;
        ///Server streaming response type for the GetReceivedMsgs method.
        type GetReceivedMsgsStream: futures_core::Stream<Item = Result<super::Msg, tonic::Status>>
            + Send
            + 'static;
        async fn get_received_msgs(
            &self,
            request: tonic::Request<super::ReceivedMsgsRequest>,
        ) -> Result<tonic::Response<Self::GetReceivedMsgsStream>, tonic::Status>;
        async fn get_thread(
            &self,
            request: tonic::Request<super::ThreadRequest>,
        ) -> Result<tonic::Response<super::ThreadResponse>, tonic::Status>;
        async fn create_thread(
            &self,
            request: tonic::Request<super::CreateThreadRequest>,
        ) -> Result<tonic::Response<super::CreateThreadResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MessengerServer<T: Messenger> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Messenger> MessengerServer<T> {
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MessengerServer<T>
    where
        T: Messenger,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/messenger.Messenger/SendMsg" => {
                    #[allow(non_camel_case_types)]
                    struct SendMsgSvc<T: Messenger>(pub Arc<T>);
                    impl<T: Messenger> tonic::server::UnaryService<super::MsgInTransit> for SendMsgSvc<T> {
                        type Response = super::SendResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgInTransit>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).send_msg(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendMsgSvc(inner);
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
                "/messenger.Messenger/GetMsg" => {
                    #[allow(non_camel_case_types)]
                    struct GetMsgSvc<T: Messenger>(pub Arc<T>);
                    impl<T: Messenger> tonic::server::UnaryService<super::MsgRequest> for GetMsgSvc<T> {
                        type Response = super::Msg;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_msg(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMsgSvc(inner);
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
                "/messenger.Messenger/GetAll" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllSvc<T: Messenger>(pub Arc<T>);
                    impl<T: Messenger> tonic::server::ServerStreamingService<super::AllMsgsRequest> for GetAllSvc<T> {
                        type Response = super::Msg;
                        type ResponseStream = T::GetAllStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AllMsgsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllSvc(inner);
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
                "/messenger.Messenger/GetSentMsgs" => {
                    #[allow(non_camel_case_types)]
                    struct GetSentMsgsSvc<T: Messenger>(pub Arc<T>);
                    impl<T: Messenger> tonic::server::ServerStreamingService<super::SentMsgsRequest>
                        for GetSentMsgsSvc<T>
                    {
                        type Response = super::Msg;
                        type ResponseStream = T::GetSentMsgsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SentMsgsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_sent_msgs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSentMsgsSvc(inner);
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
                "/messenger.Messenger/GetReceivedMsgs" => {
                    #[allow(non_camel_case_types)]
                    struct GetReceivedMsgsSvc<T: Messenger>(pub Arc<T>);
                    impl<T: Messenger>
                        tonic::server::ServerStreamingService<super::ReceivedMsgsRequest>
                        for GetReceivedMsgsSvc<T>
                    {
                        type Response = super::Msg;
                        type ResponseStream = T::GetReceivedMsgsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReceivedMsgsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_received_msgs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReceivedMsgsSvc(inner);
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
                "/messenger.Messenger/GetThread" => {
                    #[allow(non_camel_case_types)]
                    struct GetThreadSvc<T: Messenger>(pub Arc<T>);
                    impl<T: Messenger> tonic::server::UnaryService<super::ThreadRequest> for GetThreadSvc<T> {
                        type Response = super::ThreadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ThreadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_thread(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetThreadSvc(inner);
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
                "/messenger.Messenger/CreateThread" => {
                    #[allow(non_camel_case_types)]
                    struct CreateThreadSvc<T: Messenger>(pub Arc<T>);
                    impl<T: Messenger> tonic::server::UnaryService<super::CreateThreadRequest> for CreateThreadSvc<T> {
                        type Response = super::CreateThreadResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateThreadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_thread(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateThreadSvc(inner);
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
    impl<T: Messenger> Clone for MessengerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Messenger> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Messenger> tonic::server::NamedService for MessengerServer<T> {
        const NAME: &'static str = "messenger.Messenger";
    }
}
