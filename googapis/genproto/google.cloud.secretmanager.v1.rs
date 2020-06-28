/// A [Secret][google.cloud.secretmanager.v1.Secret] is a logical secret whose value and versions can
/// be accessed.
///
/// A [Secret][google.cloud.secretmanager.v1.Secret] is made up of zero or more [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] that
/// represent the secret data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    /// Output only. The resource name of the [Secret][google.cloud.secretmanager.v1.Secret] in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Immutable. The replication policy of the secret data attached to the [Secret][google.cloud.secretmanager.v1.Secret].
    ///
    /// The replication policy cannot be changed after the Secret has been created.
    #[prost(message, optional, tag = "2")]
    pub replication: ::std::option::Option<Replication>,
    /// Output only. The time at which the [Secret][google.cloud.secretmanager.v1.Secret] was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The labels assigned to this Secret.
    ///
    /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
    /// of maximum 128 bytes, and must conform to the following PCRE regular
    /// expression: `[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}`
    ///
    /// Label values must be between 0 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, and must conform to the following PCRE
    /// regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}`
    ///
    /// No more than 64 labels can be assigned to a given resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// A secret version resource in the Secret Manager API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretVersion {
    /// Output only. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the
    /// format `projects/*/secrets/*/versions/*`.
    ///
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] IDs in a [Secret][google.cloud.secretmanager.v1.Secret] start at 1 and
    /// are incremented for each subsequent version of the secret.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The time at which the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] was destroyed.
    /// Only present if [state][google.cloud.secretmanager.v1.SecretVersion.state] is
    /// [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED].
    #[prost(message, optional, tag = "3")]
    pub destroy_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[prost(enumeration = "secret_version::State", tag = "4")]
    pub state: i32,
}
pub mod secret_version {
    /// The state of a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion], indicating if it can be accessed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified. This value is unused and invalid.
        Unspecified = 0,
        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] may be accessed.
        Enabled = 1,
        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] may not be accessed, but the secret data
        /// is still available and can be placed back into the [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED]
        /// state.
        Disabled = 2,
        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] is destroyed and the secret data is no longer
        /// stored. A version may not leave this state once entered.
        Destroyed = 3,
    }
}
/// A policy that defines the replication configuration of data.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Replication {
    /// The replication policy for this secret.
    #[prost(oneof = "replication::Replication", tags = "1, 2")]
    pub replication: ::std::option::Option<replication::Replication>,
}
pub mod replication {
    /// A replication policy that replicates the [Secret][google.cloud.secretmanager.v1.Secret] payload without any
    /// restrictions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Automatic {}
    /// A replication policy that replicates the [Secret][google.cloud.secretmanager.v1.Secret] payload into the
    /// locations specified in [Secret.replication.user_managed.replicas][]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserManaged {
        /// Required. The list of Replicas for this [Secret][google.cloud.secretmanager.v1.Secret].
        ///
        /// Cannot be empty.
        #[prost(message, repeated, tag = "1")]
        pub replicas: ::std::vec::Vec<user_managed::Replica>,
    }
    pub mod user_managed {
        /// Represents a Replica for this [Secret][google.cloud.secretmanager.v1.Secret].
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Replica {
            /// The canonical IDs of the location to replicate data.
            /// For example: `"us-east1"`.
            #[prost(string, tag = "1")]
            pub location: std::string::String,
        }
    }
    /// The replication policy for this secret.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Replication {
        /// The [Secret][google.cloud.secretmanager.v1.Secret] will automatically be replicated without any restrictions.
        #[prost(message, tag = "1")]
        Automatic(Automatic),
        /// The [Secret][google.cloud.secretmanager.v1.Secret] will only be replicated into the locations specified.
        #[prost(message, tag = "2")]
        UserManaged(UserManaged),
    }
}
/// A secret payload resource in the Secret Manager API. This contains the
/// sensitive secret payload that is associated with a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretPayload {
    /// The secret data. Must be no larger than 64KiB.
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
}
/// Request message for [SecretManagerService.ListSecrets][google.cloud.secretmanager.v1.SecretManagerService.ListSecrets].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretsRequest {
    /// Required. The resource name of the project associated with the
    /// [Secrets][google.cloud.secretmanager.v1.Secret], in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListSecretsResponse.next_page_token][google.cloud.secretmanager.v1.ListSecretsResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for [SecretManagerService.ListSecrets][google.cloud.secretmanager.v1.SecretManagerService.ListSecrets].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretsResponse {
    /// The list of [Secrets][google.cloud.secretmanager.v1.Secret] sorted in reverse by create_time (newest
    /// first).
    #[prost(message, repeated, tag = "1")]
    pub secrets: ::std::vec::Vec<Secret>,
    /// A token to retrieve the next page of results. Pass this value in
    /// [ListSecretsRequest.page_token][google.cloud.secretmanager.v1.ListSecretsRequest.page_token] to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// The total number of [Secrets][google.cloud.secretmanager.v1.Secret].
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for [SecretManagerService.CreateSecret][google.cloud.secretmanager.v1.SecretManagerService.CreateSecret].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSecretRequest {
    /// Required. The resource name of the project to associate with the
    /// [Secret][google.cloud.secretmanager.v1.Secret], in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. This must be unique within the project.
    ///
    /// A secret ID is a string with a maximum length of 255 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and
    /// underscore (`_`) characters.
    #[prost(string, tag = "2")]
    pub secret_id: std::string::String,
    /// Required. A [Secret][google.cloud.secretmanager.v1.Secret] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub secret: ::std::option::Option<Secret>,
}
/// Request message for [SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSecretVersionRequest {
    /// Required. The resource name of the [Secret][google.cloud.secretmanager.v1.Secret] to associate with the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The secret payload of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<SecretPayload>,
}
/// Request message for [SecretManagerService.GetSecret][google.cloud.secretmanager.v1.SecretManagerService.GetSecret].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretRequest {
    /// Required. The resource name of the [Secret][google.cloud.secretmanager.v1.Secret], in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [SecretManagerService.ListSecretVersions][google.cloud.secretmanager.v1.SecretManagerService.ListSecretVersions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretVersionsRequest {
    /// Required. The resource name of the [Secret][google.cloud.secretmanager.v1.Secret] associated with the
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] to list, in the format
    /// `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// ListSecretVersionsResponse.next_page_token][].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for [SecretManagerService.ListSecretVersions][google.cloud.secretmanager.v1.SecretManagerService.ListSecretVersions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretVersionsResponse {
    /// The list of [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] sorted in reverse by
    /// create_time (newest first).
    #[prost(message, repeated, tag = "1")]
    pub versions: ::std::vec::Vec<SecretVersion>,
    /// A token to retrieve the next page of results. Pass this value in
    /// [ListSecretVersionsRequest.page_token][google.cloud.secretmanager.v1.ListSecretVersionsRequest.page_token] to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// The total number of [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for [SecretManagerService.GetSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.GetSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretVersionRequest {
    /// Required. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*`.
    /// `projects/*/secrets/*/versions/latest` is an alias to the `latest`
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [SecretManagerService.UpdateSecret][google.cloud.secretmanager.v1.SecretManagerService.UpdateSecret].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecretRequest {
    /// Required. [Secret][google.cloud.secretmanager.v1.Secret] with updated field values.
    #[prost(message, optional, tag = "1")]
    pub secret: ::std::option::Option<Secret>,
    /// Required. Specifies the fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for [SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessSecretVersionRequest {
    /// Required. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for [SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessSecretVersionResponse {
    /// The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Secret payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<SecretPayload>,
}
/// Request message for [SecretManagerService.DeleteSecret][google.cloud.secretmanager.v1.SecretManagerService.DeleteSecret].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSecretRequest {
    /// Required. The resource name of the [Secret][google.cloud.secretmanager.v1.Secret] to delete in the format
    /// `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [SecretManagerService.DisableSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.DisableSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableSecretVersionRequest {
    /// Required. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to disable in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [SecretManagerService.EnableSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.EnableSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableSecretVersionRequest {
    /// Required. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to enable in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [SecretManagerService.DestroySecretVersion][google.cloud.secretmanager.v1.SecretManagerService.DestroySecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroySecretVersionRequest {
    /// Required. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to destroy in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod secret_manager_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Secret Manager Service"]
    #[doc = ""]
    #[doc = " Manages secrets and operations using those secrets. Implements a REST"]
    #[doc = " model with the following objects:"]
    #[doc = ""]
    #[doc = " * [Secret][google.cloud.secretmanager.v1.Secret]"]
    #[doc = " * [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]"]
    pub struct SecretManagerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SecretManagerServiceClient<tonic::transport::Channel> {
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
    impl<T> SecretManagerServiceClient<T>
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
        #[doc = " Lists [Secrets][google.cloud.secretmanager.v1.Secret]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/ListSecrets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new [Secret][google.cloud.secretmanager.v1.Secret] containing no [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/CreateSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] containing secret data and attaches"]
        #[doc = " it to an existing [Secret][google.cloud.secretmanager.v1.Secret]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/AddSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata for a given [Secret][google.cloud.secretmanager.v1.Secret]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/GetSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates metadata of an existing [Secret][google.cloud.secretmanager.v1.Secret]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/UpdateSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a [Secret][google.cloud.secretmanager.v1.Secret]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/DeleteSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]. This call does not return secret"]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/ListSecretVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata for a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " `projects/*/secrets/*/versions/latest` is an alias to the `latest`"]
        #[doc = " [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/GetSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Accesses a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. This call returns the secret data."]
        #[doc = ""]
        #[doc = " `projects/*/secrets/*/versions/latest` is an alias to the `latest`"]
        #[doc = " [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/AccessSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Disables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to"]
        #[doc = " [DISABLED][google.cloud.secretmanager.v1.SecretVersion.State.DISABLED]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/DisableSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Enables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to"]
        #[doc = " [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/EnableSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Destroys a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to"]
        #[doc = " [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED] and irrevocably destroys the"]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/DestroySecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on the specified secret. Replaces any"]
        #[doc = " existing policy."]
        #[doc = ""]
        #[doc = " Permissions on [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] are enforced according"]
        #[doc = " to the policy set on the associated [Secret][google.cloud.secretmanager.v1.Secret]."]
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
                "/google.cloud.secretmanager.v1.SecretManagerService/SetIamPolicy",
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
                "/google.cloud.secretmanager.v1.SecretManagerService/GetIamPolicy",
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
                "/google.cloud.secretmanager.v1.SecretManagerService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SecretManagerServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SecretManagerServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SecretManagerServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod secret_manager_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SecretManagerServiceServer."]
    #[async_trait]
    pub trait SecretManagerService: Send + Sync + 'static {
        #[doc = " Lists [Secrets][google.cloud.secretmanager.v1.Secret]."]
        async fn list_secrets(
            &self,
            request: tonic::Request<super::ListSecretsRequest>,
        ) -> Result<tonic::Response<super::ListSecretsResponse>, tonic::Status>;
        #[doc = " Creates a new [Secret][google.cloud.secretmanager.v1.Secret] containing no [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]."]
        async fn create_secret(
            &self,
            request: tonic::Request<super::CreateSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status>;
        #[doc = " Creates a new [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] containing secret data and attaches"]
        #[doc = " it to an existing [Secret][google.cloud.secretmanager.v1.Secret]."]
        async fn add_secret_version(
            &self,
            request: tonic::Request<super::AddSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status>;
        #[doc = " Gets metadata for a given [Secret][google.cloud.secretmanager.v1.Secret]."]
        async fn get_secret(
            &self,
            request: tonic::Request<super::GetSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status>;
        #[doc = " Updates metadata of an existing [Secret][google.cloud.secretmanager.v1.Secret]."]
        async fn update_secret(
            &self,
            request: tonic::Request<super::UpdateSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status>;
        #[doc = " Deletes a [Secret][google.cloud.secretmanager.v1.Secret]."]
        async fn delete_secret(
            &self,
            request: tonic::Request<super::DeleteSecretRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]. This call does not return secret"]
        #[doc = " data."]
        async fn list_secret_versions(
            &self,
            request: tonic::Request<super::ListSecretVersionsRequest>,
        ) -> Result<tonic::Response<super::ListSecretVersionsResponse>, tonic::Status>;
        #[doc = " Gets metadata for a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " `projects/*/secrets/*/versions/latest` is an alias to the `latest`"]
        #[doc = " [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        async fn get_secret_version(
            &self,
            request: tonic::Request<super::GetSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status>;
        #[doc = " Accesses a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. This call returns the secret data."]
        #[doc = ""]
        #[doc = " `projects/*/secrets/*/versions/latest` is an alias to the `latest`"]
        #[doc = " [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        async fn access_secret_version(
            &self,
            request: tonic::Request<super::AccessSecretVersionRequest>,
        ) -> Result<tonic::Response<super::AccessSecretVersionResponse>, tonic::Status>;
        #[doc = " Disables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to"]
        #[doc = " [DISABLED][google.cloud.secretmanager.v1.SecretVersion.State.DISABLED]."]
        async fn disable_secret_version(
            &self,
            request: tonic::Request<super::DisableSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status>;
        #[doc = " Enables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to"]
        #[doc = " [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED]."]
        async fn enable_secret_version(
            &self,
            request: tonic::Request<super::EnableSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status>;
        #[doc = " Destroys a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to"]
        #[doc = " [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED] and irrevocably destroys the"]
        #[doc = " secret data."]
        async fn destroy_secret_version(
            &self,
            request: tonic::Request<super::DestroySecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status>;
        #[doc = " Sets the access control policy on the specified secret. Replaces any"]
        #[doc = " existing policy."]
        #[doc = ""]
        #[doc = " Permissions on [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] are enforced according"]
        #[doc = " to the policy set on the associated [Secret][google.cloud.secretmanager.v1.Secret]."]
        async fn set_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Gets the access control policy for a secret."]
        #[doc = " Returns empty policy if the secret exists and does not have a policy set."]
        async fn get_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Returns permissions that a caller has for the specified secret."]
        #[doc = " If the secret does not exist, this call returns an empty set of"]
        #[doc = " permissions, not a NOT_FOUND error."]
        #[doc = ""]
        #[doc = " Note: This operation is designed to be used for building permission-aware"]
        #[doc = " UIs and command-line tools, not for authorization checking. This operation"]
        #[doc = " may \"fail open\" without warning."]
        async fn test_iam_permissions(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::TestIamPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        >;
    }
    #[doc = " Secret Manager Service"]
    #[doc = ""]
    #[doc = " Manages secrets and operations using those secrets. Implements a REST"]
    #[doc = " model with the following objects:"]
    #[doc = ""]
    #[doc = " * [Secret][google.cloud.secretmanager.v1.Secret]"]
    #[doc = " * [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]"]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct SecretManagerServiceServer<T: SecretManagerService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: SecretManagerService> SecretManagerServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for SecretManagerServiceServer<T>
    where
        T: SecretManagerService,
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
                "/google.cloud.secretmanager.v1.SecretManagerService/ListSecrets" => {
                    #[allow(non_camel_case_types)]
                    struct ListSecretsSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::ListSecretsRequest>
                        for ListSecretsSvc<T>
                    {
                        type Response = super::ListSecretsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSecretsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_secrets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListSecretsSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/CreateSecret" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSecretSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::CreateSecretRequest>
                        for CreateSecretSvc<T>
                    {
                        type Response = super::Secret;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateSecretRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_secret(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateSecretSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/AddSecretVersion" => {
                    #[allow(non_camel_case_types)]
                    struct AddSecretVersionSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::AddSecretVersionRequest>
                        for AddSecretVersionSvc<T>
                    {
                        type Response = super::SecretVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddSecretVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.add_secret_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AddSecretVersionSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/GetSecret" => {
                    #[allow(non_camel_case_types)]
                    struct GetSecretSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::GetSecretRequest> for GetSecretSvc<T>
                    {
                        type Response = super::Secret;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSecretRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_secret(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSecretSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/UpdateSecret" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSecretSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::UpdateSecretRequest>
                        for UpdateSecretSvc<T>
                    {
                        type Response = super::Secret;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSecretRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_secret(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateSecretSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/DeleteSecret" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSecretSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::DeleteSecretRequest>
                        for DeleteSecretSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteSecretRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_secret(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteSecretSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/ListSecretVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListSecretVersionsSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::ListSecretVersionsRequest>
                        for ListSecretVersionsSvc<T>
                    {
                        type Response = super::ListSecretVersionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSecretVersionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_secret_versions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListSecretVersionsSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/GetSecretVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetSecretVersionSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::GetSecretVersionRequest>
                        for GetSecretVersionSvc<T>
                    {
                        type Response = super::SecretVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSecretVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_secret_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSecretVersionSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/AccessSecretVersion" => {
                    #[allow(non_camel_case_types)]
                    struct AccessSecretVersionSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::AccessSecretVersionRequest>
                        for AccessSecretVersionSvc<T>
                    {
                        type Response = super::AccessSecretVersionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AccessSecretVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.access_secret_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AccessSecretVersionSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/DisableSecretVersion" => {
                    #[allow(non_camel_case_types)]
                    struct DisableSecretVersionSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::DisableSecretVersionRequest>
                        for DisableSecretVersionSvc<T>
                    {
                        type Response = super::SecretVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisableSecretVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.disable_secret_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DisableSecretVersionSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/EnableSecretVersion" => {
                    #[allow(non_camel_case_types)]
                    struct EnableSecretVersionSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::EnableSecretVersionRequest>
                        for EnableSecretVersionSvc<T>
                    {
                        type Response = super::SecretVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EnableSecretVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.enable_secret_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = EnableSecretVersionSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/DestroySecretVersion" => {
                    #[allow(non_camel_case_types)]
                    struct DestroySecretVersionSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<super::DestroySecretVersionRequest>
                        for DestroySecretVersionSvc<T>
                    {
                        type Response = super::SecretVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DestroySecretVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.destroy_secret_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DestroySecretVersionSvc(inner);
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
                "/google.cloud.secretmanager.v1.SecretManagerService/SetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetIamPolicySvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::SetIamPolicyRequest,
                        > for SetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::SetIamPolicyRequest,
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
                "/google.cloud.secretmanager.v1.SecretManagerService/GetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetIamPolicySvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::GetIamPolicyRequest,
                        > for GetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::GetIamPolicyRequest,
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
                "/google.cloud.secretmanager.v1.SecretManagerService/TestIamPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct TestIamPermissionsSvc<T: SecretManagerService>(pub Arc<T>);
                    impl<T: SecretManagerService>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::TestIamPermissionsRequest,
                        > for TestIamPermissionsSvc<T>
                    {
                        type Response =
                            super::super::super::super::iam::v1::TestIamPermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::TestIamPermissionsRequest,
                            >,
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
    impl<T: SecretManagerService> Clone for SecretManagerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: SecretManagerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SecretManagerService> tonic::transport::NamedService for SecretManagerServiceServer<T> {
        const NAME: &'static str = "google.cloud.secretmanager.v1.SecretManagerService";
    }
}
