/// The request for [ConnectionService.CreateConnection][google.cloud.bigquery.connection.v1.ConnectionService.CreateConnection].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectionRequest {
    /// Required. Parent resource name.
    /// Must be in the format `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Connection id that should be assigned to the created connection.
    #[prost(string, tag = "2")]
    pub connection_id: std::string::String,
    /// Required. Connection to create.
    #[prost(message, optional, tag = "3")]
    pub connection: ::std::option::Option<Connection>,
}
/// The request for [ConnectionService.GetConnection][google.cloud.bigquery.connection.v1.ConnectionService.GetConnection].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionRequest {
    /// Required. Name of the requested connection, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for [ConnectionService.ListConnections][google.cloud.bigquery.connection.v1.ConnectionService.ListConnections].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsRequest {
    /// Required. Parent resource name.
    /// Must be in the form: `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Page size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response for [ConnectionService.ListConnections][google.cloud.bigquery.connection.v1.ConnectionService.ListConnections].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsResponse {
    /// Next page token.
    #[prost(string, tag = "1")]
    pub next_page_token: std::string::String,
    /// List of connections.
    #[prost(message, repeated, tag = "2")]
    pub connections: ::std::vec::Vec<Connection>,
}
/// The request for [ConnectionService.UpdateConnection][google.cloud.bigquery.connection.v1.ConnectionService.UpdateConnection].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectionRequest {
    /// Required. Name of the connection to update, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Connection containing the updated fields.
    #[prost(message, optional, tag = "2")]
    pub connection: ::std::option::Option<Connection>,
    /// Required. Update mask for the connection fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request for [ConnectionService.DeleteConnectionRequest][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectionRequest {
    /// Required. Name of the deleted connection, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Configuration parameters to establish connection with an external data
/// source, except the credential attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// The resource name of the connection in the form of:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// User provided display name for the connection.
    #[prost(string, tag = "2")]
    pub friendly_name: std::string::String,
    /// User provided description.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// Output only. The creation timestamp of the connection.
    #[prost(int64, tag = "5")]
    pub creation_time: i64,
    /// Output only. The last update timestamp of the connection.
    #[prost(int64, tag = "6")]
    pub last_modified_time: i64,
    /// Output only. True, if credential is configured for this connection.
    #[prost(bool, tag = "7")]
    pub has_credential: bool,
    /// Properties specific to the underlying data source.
    #[prost(oneof = "connection::Properties", tags = "4")]
    pub properties: ::std::option::Option<connection::Properties>,
}
pub mod connection {
    /// Properties specific to the underlying data source.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Properties {
        /// Cloud SQL properties.
        #[prost(message, tag = "4")]
        CloudSql(super::CloudSqlProperties),
    }
}
/// Connection properties specific to the Cloud SQL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlProperties {
    /// Cloud SQL instance ID in the form `project:location:instance`.
    #[prost(string, tag = "1")]
    pub instance_id: std::string::String,
    /// Database name.
    #[prost(string, tag = "2")]
    pub database: std::string::String,
    /// Type of the Cloud SQL database.
    #[prost(enumeration = "cloud_sql_properties::DatabaseType", tag = "3")]
    pub r#type: i32,
    /// Input only. Cloud SQL credential.
    #[prost(message, optional, tag = "4")]
    pub credential: ::std::option::Option<CloudSqlCredential>,
}
pub mod cloud_sql_properties {
    /// Supported Cloud SQL database types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DatabaseType {
        /// Unspecified database type.
        Unspecified = 0,
        /// Cloud SQL for PostgreSQL.
        Postgres = 1,
        /// Cloud SQL for MySQL.
        Mysql = 2,
    }
}
/// Credential info for the Cloud SQL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlCredential {
    /// The username for the credential.
    #[prost(string, tag = "1")]
    pub username: std::string::String,
    /// The password for the credential.
    #[prost(string, tag = "2")]
    pub password: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod connection_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages external data source connections and credentials."]
    pub struct ConnectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConnectionServiceClient<tonic::transport::Channel> {
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
    impl<T> ConnectionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates a new connection."]
        pub async fn create_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectionRequest>,
        ) -> Result<tonic::Response<super::Connection>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/CreateConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns specified connection."]
        pub async fn get_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectionRequest>,
        ) -> Result<tonic::Response<super::Connection>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/GetConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a list of connections in the given project."]
        pub async fn list_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectionsRequest>,
        ) -> Result<tonic::Response<super::ListConnectionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/ListConnections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified connection. For security reasons, also resets"]
        #[doc = " credential if connection properties are in the update field mask."]
        pub async fn update_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectionRequest>,
        ) -> Result<tonic::Response<super::Connection>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/UpdateConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes connection and associated credential."]
        pub async fn delete_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/DeleteConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a resource."]
        #[doc = " Returns an empty policy if the resource exists and does not have a policy"]
        #[doc = " set."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on the specified resource. Replaces any"]
        #[doc = " existing policy."]
        #[doc = ""]
        #[doc = " Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that a caller has on the specified resource."]
        #[doc = " If the resource does not exist, this will return an empty set of"]
        #[doc = " permissions, not a `NOT_FOUND` error."]
        #[doc = ""]
        #[doc = " Note: This operation is designed to be used for building permission-aware"]
        #[doc = " UIs and command-line tools, not for authorization checking. This operation"]
        #[doc = " may \"fail open\" without warning."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.connection.v1.ConnectionService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ConnectionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ConnectionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ConnectionServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod connection_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ConnectionServiceServer."]
    #[async_trait]
    pub trait ConnectionService: Send + Sync + 'static {
        #[doc = " Creates a new connection."]
        async fn create_connection(
            &self,
            request: tonic::Request<super::CreateConnectionRequest>,
        ) -> Result<tonic::Response<super::Connection>, tonic::Status>;
        #[doc = " Returns specified connection."]
        async fn get_connection(
            &self,
            request: tonic::Request<super::GetConnectionRequest>,
        ) -> Result<tonic::Response<super::Connection>, tonic::Status>;
        #[doc = " Returns a list of connections in the given project."]
        async fn list_connections(
            &self,
            request: tonic::Request<super::ListConnectionsRequest>,
        ) -> Result<tonic::Response<super::ListConnectionsResponse>, tonic::Status>;
        #[doc = " Updates the specified connection. For security reasons, also resets"]
        #[doc = " credential if connection properties are in the update field mask."]
        async fn update_connection(
            &self,
            request: tonic::Request<super::UpdateConnectionRequest>,
        ) -> Result<tonic::Response<super::Connection>, tonic::Status>;
        #[doc = " Deletes connection and associated credential."]
        async fn delete_connection(
            &self,
            request: tonic::Request<super::DeleteConnectionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Gets the access control policy for a resource."]
        #[doc = " Returns an empty policy if the resource exists and does not have a policy"]
        #[doc = " set."]
        async fn get_iam_policy(
            &self,
            request: tonic::Request<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        >;
        #[doc = " Sets the access control policy on the specified resource. Replaces any"]
        #[doc = " existing policy."]
        #[doc = ""]
        #[doc = " Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors."]
        async fn set_iam_policy(
            &self,
            request: tonic::Request<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        >;
        #[doc = " Returns permissions that a caller has on the specified resource."]
        #[doc = " If the resource does not exist, this will return an empty set of"]
        #[doc = " permissions, not a `NOT_FOUND` error."]
        #[doc = ""]
        #[doc = " Note: This operation is designed to be used for building permission-aware"]
        #[doc = " UIs and command-line tools, not for authorization checking. This operation"]
        #[doc = " may \"fail open\" without warning."]
        async fn test_iam_permissions(
            &self,
            request: tonic::Request<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        >;
    }
    #[doc = " Manages external data source connections and credentials."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ConnectionServiceServer<T: ConnectionService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ConnectionService> ConnectionServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ConnectionServiceServer<T>
    where
        T: ConnectionService,
        B: HttpBody + Send + Sync + 'static,
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
                "/google.cloud.bigquery.connection.v1.ConnectionService/CreateConnection" => {
                    #[allow(non_camel_case_types)]
                    struct CreateConnectionSvc<T: ConnectionService>(pub Arc<T>);
                    impl<T: ConnectionService>
                        tonic::server::UnaryService<super::CreateConnectionRequest>
                        for CreateConnectionSvc<T>
                    {
                        type Response = super::Connection;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateConnectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_connection(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateConnectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.connection.v1.ConnectionService/GetConnection" => {
                    #[allow(non_camel_case_types)]
                    struct GetConnectionSvc<T: ConnectionService>(pub Arc<T>);
                    impl<T: ConnectionService>
                        tonic::server::UnaryService<super::GetConnectionRequest>
                        for GetConnectionSvc<T>
                    {
                        type Response = super::Connection;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetConnectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_connection(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetConnectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.connection.v1.ConnectionService/ListConnections" => {
                    #[allow(non_camel_case_types)]
                    struct ListConnectionsSvc<T: ConnectionService>(pub Arc<T>);
                    impl<T: ConnectionService>
                        tonic::server::UnaryService<super::ListConnectionsRequest>
                        for ListConnectionsSvc<T>
                    {
                        type Response = super::ListConnectionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListConnectionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_connections(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListConnectionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.connection.v1.ConnectionService/UpdateConnection" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateConnectionSvc<T: ConnectionService>(pub Arc<T>);
                    impl<T: ConnectionService>
                        tonic::server::UnaryService<super::UpdateConnectionRequest>
                        for UpdateConnectionSvc<T>
                    {
                        type Response = super::Connection;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateConnectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_connection(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateConnectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.connection.v1.ConnectionService/DeleteConnection" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteConnectionSvc<T: ConnectionService>(pub Arc<T>);
                    impl<T: ConnectionService>
                        tonic::server::UnaryService<super::DeleteConnectionRequest>
                        for DeleteConnectionSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteConnectionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_connection(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteConnectionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.connection.v1.ConnectionService/GetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetIamPolicySvc<T: ConnectionService>(pub Arc<T>);
                    impl<T: ConnectionService>
                        tonic::server::UnaryService<
                            super::super::super::super::super::iam::v1::GetIamPolicyRequest,
                        > for GetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetIamPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.connection.v1.ConnectionService/SetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetIamPolicySvc<T: ConnectionService>(pub Arc<T>);
                    impl<T: ConnectionService>
                        tonic::server::UnaryService<
                            super::super::super::super::super::iam::v1::SetIamPolicyRequest,
                        > for SetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetIamPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.bigquery.connection.v1.ConnectionService/TestIamPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct TestIamPermissionsSvc<T: ConnectionService>(pub Arc<T>);
                    impl<T: ConnectionService>
                        tonic::server::UnaryService<
                            super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
                        > for TestIamPermissionsSvc<T>
                    {
                        type Response =
                            super::super::super::super::super::iam::v1::TestIamPermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: super :: iam :: v1 :: TestIamPermissionsRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.test_iam_permissions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TestIamPermissionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ConnectionService> Clone for ConnectionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ConnectionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ConnectionService> tonic::transport::NamedService for ConnectionServiceServer<T> {
        const NAME: &'static str = "google.cloud.bigquery.connection.v1.ConnectionService";
    }
}
