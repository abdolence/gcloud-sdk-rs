/// A \[Secret][google.cloud.secrets.v1beta1.Secret\] is a logical secret whose value and versions can
/// be accessed.
///
/// A \[Secret][google.cloud.secrets.v1beta1.Secret\] is made up of zero or more \[SecretVersions][google.cloud.secrets.v1beta1.SecretVersion\] that
/// represent the secret data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    /// Output only. The resource name of the \[Secret][google.cloud.secrets.v1beta1.Secret\] in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The replication policy of the secret data attached to the \[Secret][google.cloud.secrets.v1beta1.Secret\].
    ///
    /// The replication policy cannot be changed after the Secret has been created.
    #[prost(message, optional, tag = "2")]
    pub replication: ::core::option::Option<Replication>,
    /// Output only. The time at which the \[Secret][google.cloud.secrets.v1beta1.Secret\] was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The labels assigned to this Secret.
    ///
    /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
    /// of maximum 128 bytes, and must conform to the following PCRE regular
    /// expression: `\[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-\]{0,62}`
    ///
    /// Label values must be between 0 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, and must conform to the following PCRE
    /// regular expression: `\[\p{Ll}\p{Lo}\p{N}_-\]{0,63}`
    ///
    /// No more than 64 labels can be assigned to a given resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// A secret version resource in the Secret Manager API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretVersion {
    /// Output only. The resource name of the \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] in the
    /// format `projects/*/secrets/*/versions/*`.
    ///
    /// \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] IDs in a \[Secret][google.cloud.secrets.v1beta1.Secret\] start at 1 and
    /// are incremented for each subsequent version of the secret.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time at which the \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] was destroyed.
    /// Only present if \[state][google.cloud.secrets.v1beta1.SecretVersion.state\] is
    /// \[DESTROYED][google.cloud.secrets.v1beta1.SecretVersion.State.DESTROYED\].
    #[prost(message, optional, tag = "3")]
    pub destroy_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\].
    #[prost(enumeration = "secret_version::State", tag = "4")]
    pub state: i32,
}
/// Nested message and enum types in `SecretVersion`.
pub mod secret_version {
    /// The state of a \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\], indicating if it can be accessed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified. This value is unused and invalid.
        Unspecified = 0,
        /// The \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] may be accessed.
        Enabled = 1,
        /// The \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] may not be accessed, but the secret data
        /// is still available and can be placed back into the \[ENABLED][google.cloud.secrets.v1beta1.SecretVersion.State.ENABLED\]
        /// state.
        Disabled = 2,
        /// The \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] is destroyed and the secret data is no longer
        /// stored. A version may not leave this state once entered.
        Destroyed = 3,
    }
}
/// A policy that defines the replication configuration of data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Replication {
    /// The replication policy for this secret.
    #[prost(oneof = "replication::Replication", tags = "1, 2")]
    pub replication: ::core::option::Option<replication::Replication>,
}
/// Nested message and enum types in `Replication`.
pub mod replication {
    /// A replication policy that replicates the \[Secret][google.cloud.secrets.v1beta1.Secret\] payload without any
    /// restrictions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Automatic {}
    /// A replication policy that replicates the \[Secret][google.cloud.secrets.v1beta1.Secret\] payload into the
    /// locations specified in \[Secret.replication.user_managed.replicas][\]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserManaged {
        /// Required. The list of Replicas for this \[Secret][google.cloud.secrets.v1beta1.Secret\].
        ///
        /// Cannot be empty.
        #[prost(message, repeated, tag = "1")]
        pub replicas: ::prost::alloc::vec::Vec<user_managed::Replica>,
    }
    /// Nested message and enum types in `UserManaged`.
    pub mod user_managed {
        /// Represents a Replica for this \[Secret][google.cloud.secrets.v1beta1.Secret\].
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Replica {
            /// The canonical IDs of the location to replicate data.
            /// For example: `"us-east1"`.
            #[prost(string, tag = "1")]
            pub location: ::prost::alloc::string::String,
        }
    }
    /// The replication policy for this secret.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Replication {
        /// The \[Secret][google.cloud.secrets.v1beta1.Secret\] will automatically be replicated without any restrictions.
        #[prost(message, tag = "1")]
        Automatic(Automatic),
        /// The \[Secret][google.cloud.secrets.v1beta1.Secret\] will only be replicated into the locations specified.
        #[prost(message, tag = "2")]
        UserManaged(UserManaged),
    }
}
/// A secret payload resource in the Secret Manager API. This contains the
/// sensitive secret data that is associated with a \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretPayload {
    /// The secret data. Must be no larger than 64KiB.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Request message for \[SecretManagerService.ListSecrets][google.cloud.secrets.v1beta1.SecretManagerService.ListSecrets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretsRequest {
    /// Required. The resource name of the project associated with the
    /// \[Secrets][google.cloud.secrets.v1beta1.Secret\], in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// \[ListSecretsResponse.next_page_token][google.cloud.secrets.v1beta1.ListSecretsResponse.next_page_token\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[SecretManagerService.ListSecrets][google.cloud.secrets.v1beta1.SecretManagerService.ListSecrets\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretsResponse {
    /// The list of \[Secrets][google.cloud.secrets.v1beta1.Secret\] sorted in reverse by create_time (newest
    /// first).
    #[prost(message, repeated, tag = "1")]
    pub secrets: ::prost::alloc::vec::Vec<Secret>,
    /// A token to retrieve the next page of results. Pass this value in
    /// \[ListSecretsRequest.page_token][google.cloud.secrets.v1beta1.ListSecretsRequest.page_token\] to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of \[Secrets][google.cloud.secrets.v1beta1.Secret\].
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for \[SecretManagerService.CreateSecret][google.cloud.secrets.v1beta1.SecretManagerService.CreateSecret\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSecretRequest {
    /// Required. The resource name of the project to associate with the
    /// \[Secret][google.cloud.secrets.v1beta1.Secret\], in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. This must be unique within the project.
    ///
    /// A secret ID is a string with a maximum length of 255 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and
    /// underscore (`_`) characters.
    #[prost(string, tag = "2")]
    pub secret_id: ::prost::alloc::string::String,
    /// Required. A \[Secret][google.cloud.secrets.v1beta1.Secret\] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub secret: ::core::option::Option<Secret>,
}
/// Request message for \[SecretManagerService.AddSecretVersion][google.cloud.secrets.v1beta1.SecretManagerService.AddSecretVersion\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSecretVersionRequest {
    /// Required. The resource name of the \[Secret][google.cloud.secrets.v1beta1.Secret\] to associate with the
    /// \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The secret payload of the \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\].
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<SecretPayload>,
}
/// Request message for \[SecretManagerService.GetSecret][google.cloud.secrets.v1beta1.SecretManagerService.GetSecret\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretRequest {
    /// Required. The resource name of the \[Secret][google.cloud.secrets.v1beta1.Secret\], in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[SecretManagerService.ListSecretVersions][google.cloud.secrets.v1beta1.SecretManagerService.ListSecretVersions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretVersionsRequest {
    /// Required. The resource name of the \[Secret][google.cloud.secrets.v1beta1.Secret\] associated with the
    /// \[SecretVersions][google.cloud.secrets.v1beta1.SecretVersion\] to list, in the format
    /// `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// ListSecretVersionsResponse.next_page_token][].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[SecretManagerService.ListSecretVersions][google.cloud.secrets.v1beta1.SecretManagerService.ListSecretVersions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretVersionsResponse {
    /// The list of \[SecretVersions][google.cloud.secrets.v1beta1.SecretVersion\] sorted in reverse by
    /// create_time (newest first).
    #[prost(message, repeated, tag = "1")]
    pub versions: ::prost::alloc::vec::Vec<SecretVersion>,
    /// A token to retrieve the next page of results. Pass this value in
    /// \[ListSecretVersionsRequest.page_token][google.cloud.secrets.v1beta1.ListSecretVersionsRequest.page_token\] to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of \[SecretVersions][google.cloud.secrets.v1beta1.SecretVersion\].
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for \[SecretManagerService.GetSecretVersion][google.cloud.secrets.v1beta1.SecretManagerService.GetSecretVersion\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretVersionRequest {
    /// Required. The resource name of the \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] in the format
    /// `projects/*/secrets/*/versions/*`.
    /// `projects/*/secrets/*/versions/latest` is an alias to the `latest`
    /// \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\].
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[SecretManagerService.UpdateSecret][google.cloud.secrets.v1beta1.SecretManagerService.UpdateSecret\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecretRequest {
    /// Required. \[Secret][google.cloud.secrets.v1beta1.Secret\] with updated field values.
    #[prost(message, optional, tag = "1")]
    pub secret: ::core::option::Option<Secret>,
    /// Required. Specifies the fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for \[SecretManagerService.AccessSecretVersion][google.cloud.secrets.v1beta1.SecretManagerService.AccessSecretVersion\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessSecretVersionRequest {
    /// Required. The resource name of the \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for \[SecretManagerService.AccessSecretVersion][google.cloud.secrets.v1beta1.SecretManagerService.AccessSecretVersion\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessSecretVersionResponse {
    /// The resource name of the \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Secret payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::core::option::Option<SecretPayload>,
}
/// Request message for \[SecretManagerService.DeleteSecret][google.cloud.secrets.v1beta1.SecretManagerService.DeleteSecret\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSecretRequest {
    /// Required. The resource name of the \[Secret][google.cloud.secrets.v1beta1.Secret\] to delete in the format
    /// `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[SecretManagerService.DisableSecretVersion][google.cloud.secrets.v1beta1.SecretManagerService.DisableSecretVersion\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableSecretVersionRequest {
    /// Required. The resource name of the \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] to disable in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[SecretManagerService.EnableSecretVersion][google.cloud.secrets.v1beta1.SecretManagerService.EnableSecretVersion\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableSecretVersionRequest {
    /// Required. The resource name of the \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] to enable in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[SecretManagerService.DestroySecretVersion][google.cloud.secrets.v1beta1.SecretManagerService.DestroySecretVersion\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroySecretVersionRequest {
    /// Required. The resource name of the \[SecretVersion][google.cloud.secrets.v1beta1.SecretVersion\] to destroy in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod secret_manager_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Secret Manager Service"]
    #[doc = ""]
    #[doc = " Manages secrets and operations using those secrets. Implements a REST"]
    #[doc = " model with the following objects:"]
    #[doc = ""]
    #[doc = " * [Secret][google.cloud.secrets.v1beta1.Secret]"]
    #[doc = " * [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion]"]
    #[derive(Debug, Clone)]
    pub struct SecretManagerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SecretManagerServiceClient<T>
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
        ) -> SecretManagerServiceClient<InterceptedService<T, F>>
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
            SecretManagerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists [Secrets][google.cloud.secrets.v1beta1.Secret]."]
        pub async fn list_secrets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSecretsRequest>,
        ) -> Result<tonic::Response<super::ListSecretsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/ListSecrets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new [Secret][google.cloud.secrets.v1beta1.Secret] containing no [SecretVersions][google.cloud.secrets.v1beta1.SecretVersion]."]
        pub async fn create_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/CreateSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion] containing secret data and attaches"]
        #[doc = " it to an existing [Secret][google.cloud.secrets.v1beta1.Secret]."]
        pub async fn add_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::AddSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/AddSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata for a given [Secret][google.cloud.secrets.v1beta1.Secret]."]
        pub async fn get_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/GetSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates metadata of an existing [Secret][google.cloud.secrets.v1beta1.Secret]."]
        pub async fn update_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/UpdateSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a [Secret][google.cloud.secrets.v1beta1.Secret]."]
        pub async fn delete_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSecretRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/DeleteSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists [SecretVersions][google.cloud.secrets.v1beta1.SecretVersion]. This call does not return secret"]
        #[doc = " data."]
        pub async fn list_secret_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSecretVersionsRequest>,
        ) -> Result<tonic::Response<super::ListSecretVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/ListSecretVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata for a [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion]."]
        #[doc = ""]
        #[doc = " `projects/*/secrets/*/versions/latest` is an alias to the `latest`"]
        #[doc = " [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion]."]
        pub async fn get_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/GetSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Accesses a [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion]. This call returns the secret data."]
        #[doc = ""]
        #[doc = " `projects/*/secrets/*/versions/latest` is an alias to the `latest`"]
        #[doc = " [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion]."]
        pub async fn access_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::AccessSecretVersionRequest>,
        ) -> Result<tonic::Response<super::AccessSecretVersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/AccessSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Disables a [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secrets.v1beta1.SecretVersion.state] of the [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion] to"]
        #[doc = " [DISABLED][google.cloud.secrets.v1beta1.SecretVersion.State.DISABLED]."]
        pub async fn disable_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/DisableSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Enables a [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secrets.v1beta1.SecretVersion.state] of the [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion] to"]
        #[doc = " [ENABLED][google.cloud.secrets.v1beta1.SecretVersion.State.ENABLED]."]
        pub async fn enable_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/EnableSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Destroys a [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secrets.v1beta1.SecretVersion.state] of the [SecretVersion][google.cloud.secrets.v1beta1.SecretVersion] to"]
        #[doc = " [DESTROYED][google.cloud.secrets.v1beta1.SecretVersion.State.DESTROYED] and irrevocably destroys the"]
        #[doc = " secret data."]
        pub async fn destroy_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroySecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/DestroySecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on the specified secret. Replaces any"]
        #[doc = " existing policy."]
        #[doc = ""]
        #[doc = " Permissions on [SecretVersions][google.cloud.secrets.v1beta1.SecretVersion] are enforced according"]
        #[doc = " to the policy set on the associated [Secret][google.cloud.secrets.v1beta1.Secret]."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a secret."]
        #[doc = " Returns empty policy if the secret exists and does not have a policy set."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secrets.v1beta1.SecretManagerService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that a caller has for the specified secret."]
        #[doc = " If the secret does not exist, this call returns an empty set of"]
        #[doc = " permissions, not a NOT_FOUND error."]
        #[doc = ""]
        #[doc = " Note: This operation is designed to be used for building permission-aware"]
        #[doc = " UIs and command-line tools, not for authorization checking. This operation"]
        #[doc = " may \"fail open\" without warning."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.cloud.secrets.v1beta1.SecretManagerService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
