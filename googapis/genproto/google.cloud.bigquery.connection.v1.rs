/// The request for \[ConnectionService.CreateConnection][google.cloud.bigquery.connection.v1.ConnectionService.CreateConnection\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectionRequest {
    /// Required. Parent resource name.
    /// Must be in the format `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Connection id that should be assigned to the created connection.
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    /// Required. Connection to create.
    #[prost(message, optional, tag = "3")]
    pub connection: ::core::option::Option<Connection>,
}
/// The request for \[ConnectionService.GetConnection][google.cloud.bigquery.connection.v1.ConnectionService.GetConnection\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionRequest {
    /// Required. Name of the requested connection, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for \[ConnectionService.ListConnections][google.cloud.bigquery.connection.v1.ConnectionService.ListConnections\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsRequest {
    /// Required. Parent resource name.
    /// Must be in the form: `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Page size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for \[ConnectionService.ListConnections][google.cloud.bigquery.connection.v1.ConnectionService.ListConnections\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionsResponse {
    /// Next page token.
    #[prost(string, tag = "1")]
    pub next_page_token: ::prost::alloc::string::String,
    /// List of connections.
    #[prost(message, repeated, tag = "2")]
    pub connections: ::prost::alloc::vec::Vec<Connection>,
}
/// The request for \[ConnectionService.UpdateConnection][google.cloud.bigquery.connection.v1.ConnectionService.UpdateConnection\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectionRequest {
    /// Required. Name of the connection to update, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Connection containing the updated fields.
    #[prost(message, optional, tag = "2")]
    pub connection: ::core::option::Option<Connection>,
    /// Required. Update mask for the connection fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request for \[ConnectionService.DeleteConnectionRequest][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectionRequest {
    /// Required. Name of the deleted connection, for example:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Configuration parameters to establish connection with an external data
/// source, except the credential attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// The resource name of the connection in the form of:
    /// `projects/{project_id}/locations/{location_id}/connections/{connection_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User provided display name for the connection.
    #[prost(string, tag = "2")]
    pub friendly_name: ::prost::alloc::string::String,
    /// User provided description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
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
    #[prost(oneof = "connection::Properties", tags = "4, 8, 21")]
    pub properties: ::core::option::Option<connection::Properties>,
}
/// Nested message and enum types in `Connection`.
pub mod connection {
    /// Properties specific to the underlying data source.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Properties {
        /// Cloud SQL properties.
        #[prost(message, tag = "4")]
        CloudSql(super::CloudSqlProperties),
        /// Amazon Web Services (AWS) properties.
        #[prost(message, tag = "8")]
        Aws(super::AwsProperties),
        /// Cloud Spanner properties.
        #[prost(message, tag = "21")]
        CloudSpanner(super::CloudSpannerProperties),
    }
}
/// Connection properties specific to the Cloud SQL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlProperties {
    /// Cloud SQL instance ID in the form `project:location:instance`.
    #[prost(string, tag = "1")]
    pub instance_id: ::prost::alloc::string::String,
    /// Database name.
    #[prost(string, tag = "2")]
    pub database: ::prost::alloc::string::String,
    /// Type of the Cloud SQL database.
    #[prost(enumeration = "cloud_sql_properties::DatabaseType", tag = "3")]
    pub r#type: i32,
    /// Input only. Cloud SQL credential.
    #[prost(message, optional, tag = "4")]
    pub credential: ::core::option::Option<CloudSqlCredential>,
}
/// Nested message and enum types in `CloudSqlProperties`.
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
    pub username: ::prost::alloc::string::String,
    /// The password for the credential.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
}
/// Connection properties specific to Cloud Spanner.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSpannerProperties {
    /// Cloud Spanner database in the form `project/instance/database'
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// If parallelism should be used when reading from Cloud Spanner
    #[prost(bool, tag = "2")]
    pub use_parallelism: bool,
}
/// Connection properties specific to Amazon Web Services (AWS).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsProperties {
    /// Authentication method chosen at connection creation.
    #[prost(oneof = "aws_properties::AuthenticationMethod", tags = "2, 3")]
    pub authentication_method: ::core::option::Option<aws_properties::AuthenticationMethod>,
}
/// Nested message and enum types in `AwsProperties`.
pub mod aws_properties {
    /// Authentication method chosen at connection creation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuthenticationMethod {
        /// Authentication using Google owned AWS IAM user's access key to assume
        /// into customer's AWS IAM Role.
        #[prost(message, tag = "2")]
        CrossAccountRole(super::AwsCrossAccountRole),
        /// Authentication using Google owned service account to assume into
        /// customer's AWS IAM Role.
        #[prost(message, tag = "3")]
        AccessRole(super::AwsAccessRole),
    }
}
/// Authentication method for Amazon Web Services (AWS) that uses Google owned
/// AWS IAM user's access key to assume into customer's AWS IAM Role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsCrossAccountRole {
    /// The user’s AWS IAM Role that trusts the Google-owned AWS IAM user
    /// Connection.
    #[prost(string, tag = "1")]
    pub iam_role_id: ::prost::alloc::string::String,
    /// Output only. Google-owned AWS IAM User for a Connection.
    #[prost(string, tag = "2")]
    pub iam_user_id: ::prost::alloc::string::String,
    /// Output only. A Google-generated id for representing Connection’s identity in AWS.
    /// External Id is also used for preventing the Confused Deputy Problem. See
    /// <https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-user_externalid.html>
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
}
/// Authentication method for Amazon Web Services (AWS) that uses Google owned
/// Google service account to assume into customer's AWS IAM Role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsAccessRole {
    /// The user’s AWS IAM Role that trusts the Google-owned AWS IAM user
    /// Connection.
    #[prost(string, tag = "1")]
    pub iam_role_id: ::prost::alloc::string::String,
    /// A unique Google-owned and Google-generated identity for the Connection.
    /// This identity will be used to access the user's AWS IAM Role.
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod connection_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Manages external data source connections and credentials."]
    #[derive(Debug, Clone)]
    pub struct ConnectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConnectionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
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
        ) -> ConnectionServiceClient<InterceptedService<T, F>>
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
            ConnectionServiceClient::new(InterceptedService::new(inner, interceptor))
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
}
