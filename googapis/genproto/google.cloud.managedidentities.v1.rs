/// Represents a managed Microsoft Active Directory domain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Domain {
    /// Required. The unique name of the domain using the form:
    /// `projects/{project_id}/locations/global/domains/{domain_name}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. Resource labels that can contain user-provided metadata.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Optional. The full names of the Google Compute Engine
    /// [networks](/compute/docs/networks-and-firewalls#networks) the domain
    /// instance is connected to. Networks can be added using UpdateDomain.
    /// The domain is only available on networks listed in `authorized_networks`.
    /// If CIDR subnets overlap between networks, domain creation will fail.
    #[prost(string, repeated, tag = "3")]
    pub authorized_networks: ::std::vec::Vec<std::string::String>,
    /// Required. The CIDR range of internal addresses that are reserved for this
    /// domain. Reserved networks must be /24 or larger. Ranges must be
    /// unique and non-overlapping with existing subnets in
    /// [Domain].[authorized_networks].
    #[prost(string, tag = "4")]
    pub reserved_ip_range: std::string::String,
    /// Required. Locations where domain needs to be provisioned.
    /// [regions][compute/docs/regions-zones/]
    /// e.g. us-west1 or us-east4
    /// Service supports up to 4 locations at once. Each location will use a /26
    /// block.
    #[prost(string, repeated, tag = "5")]
    pub locations: ::std::vec::Vec<std::string::String>,
    /// Optional. The name of delegated administrator account used to perform
    /// Active Directory operations. If not specified, `setupadmin` will be used.
    #[prost(string, tag = "6")]
    pub admin: std::string::String,
    /// Output only. The fully-qualified domain name of the exposed domain used by
    /// clients to connect to the service. Similar to what would be chosen for an
    /// Active Directory set up on an internal network.
    #[prost(string, tag = "10")]
    pub fqdn: std::string::String,
    /// Output only. The time the instance was created.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update time.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of this domain.
    #[prost(enumeration = "domain::State", tag = "13")]
    pub state: i32,
    /// Output only. Additional information about the current status of this
    /// domain, if available.
    #[prost(string, tag = "14")]
    pub status_message: std::string::String,
    /// Output only. The current trusts associated with the domain.
    #[prost(message, repeated, tag = "15")]
    pub trusts: ::std::vec::Vec<Trust>,
}
pub mod domain {
    /// Represents the different states of a managed domain.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not set.
        Unspecified = 0,
        /// The domain is being created.
        Creating = 1,
        /// The domain has been created and is fully usable.
        Ready = 2,
        /// The domain's configuration is being updated.
        Updating = 3,
        /// The domain is being deleted.
        Deleting = 4,
        /// The domain is being repaired and may be unusable. Details
        /// can be found in the `status_message` field.
        Repairing = 5,
        /// The domain is undergoing maintenance.
        PerformingMaintenance = 6,
        /// The domain is not serving requests.
        Unavailable = 7,
    }
}
/// Represents a relationship between two domains. This allows a controller in
/// one domain to authenticate a user in another domain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trust {
    /// Required. The fully qualified target domain name which will be in trust with the
    /// current domain.
    #[prost(string, tag = "1")]
    pub target_domain_name: std::string::String,
    /// Required. The type of trust represented by the trust resource.
    #[prost(enumeration = "trust::TrustType", tag = "2")]
    pub trust_type: i32,
    /// Required. The trust direction, which decides if the current domain is trusted,
    /// trusting, or both.
    #[prost(enumeration = "trust::TrustDirection", tag = "3")]
    pub trust_direction: i32,
    /// Optional. The trust authentication type, which decides whether the trusted side has
    /// forest/domain wide access or selective access to an approved set of
    /// resources.
    #[prost(bool, tag = "4")]
    pub selective_authentication: bool,
    /// Required. The target DNS server IP addresses which can resolve the remote domain
    /// involved in the trust.
    #[prost(string, repeated, tag = "5")]
    pub target_dns_ip_addresses: ::std::vec::Vec<std::string::String>,
    /// Required. The trust secret used for the handshake with the target domain. This will
    /// not be stored.
    #[prost(string, tag = "6")]
    pub trust_handshake_secret: std::string::String,
    /// Output only. The time the instance was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update time.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the trust.
    #[prost(enumeration = "trust::State", tag = "9")]
    pub state: i32,
    /// Output only. Additional information about the current state of the trust, if available.
    #[prost(string, tag = "11")]
    pub state_description: std::string::String,
    /// Output only. The last heartbeat time when the trust was known to be connected.
    #[prost(message, optional, tag = "12")]
    pub last_trust_heartbeat_time: ::std::option::Option<::prost_types::Timestamp>,
}
pub mod trust {
    /// Represents the different states of a domain trust.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not set.
        Unspecified = 0,
        /// The domain trust is being created.
        Creating = 1,
        /// The domain trust is being updated.
        Updating = 2,
        /// The domain trust is being deleted.
        Deleting = 3,
        /// The domain trust is connected.
        Connected = 4,
        /// The domain trust is disconnected.
        Disconnected = 5,
    }
    /// Represents the different inter-forest trust types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TrustType {
        /// Not set.
        Unspecified = 0,
        /// The forest trust.
        Forest = 1,
        /// The external domain trust.
        External = 2,
    }
    /// Represents the direction of trust.
    /// See
    /// [System.DirectoryServices.ActiveDirectory.TrustDirection](https://docs.microsoft.com/en-us/dotnet/api/system.directoryservices.activedirectory.trustdirection?view=netframework-4.7.2)
    /// for more information.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TrustDirection {
        /// Not set.
        Unspecified = 0,
        /// The inbound direction represents the trusting side.
        Inbound = 1,
        /// The outboud direction represents the trusted side.
        Outbound = 2,
        /// The bidirectional direction represents the trusted / trusting side.
        Bidirectional = 3,
    }
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: std::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: std::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "5")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "6")]
    pub api_version: std::string::String,
}
/// Request message for
/// [CreateMicrosoftAdDomain][google.cloud.managedidentities.v1.CreateMicrosoftAdDomain]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMicrosoftAdDomainRequest {
    /// Required. The resource project name and location using the form:
    /// `projects/{project_id}/locations/global`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The fully qualified domain name.
    /// e.g. mydomain.myorganization.com, with the following restrictions:
    ///
    ///  * Must contain only lowercase letters, numbers, periods and hyphens.
    ///  * Must start with a letter.
    ///  * Must contain between 2-64 characters.
    ///  * Must end with a number or a letter.
    ///  * Must not start with period.
    ///  * First segement length (mydomain form example above) shouldn't exceed
    ///    15 chars.
    ///  * The last segment cannot be fully numeric.
    ///  * Must be unique within the customer project.
    #[prost(string, tag = "2")]
    pub domain_name: std::string::String,
    /// Required. A Managed Identity domain resource.
    #[prost(message, optional, tag = "3")]
    pub domain: ::std::option::Option<Domain>,
}
/// Request message for
/// [ResetAdminPassword][google.cloud.managedidentities.v1.ResetAdminPassword]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetAdminPasswordRequest {
    /// Required. The domain resource name using the form:
    /// `projects/{project_id}/locations/global/domains/{domain_name}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for
/// [ResetAdminPassword][google.cloud.managedidentities.v1.ResetAdminPassword]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetAdminPasswordResponse {
    /// A random password. See [admin][google.cloud.managedidentities.v1.Domain.admin] for more information.
    #[prost(string, tag = "1")]
    pub password: std::string::String,
}
/// Request message for
/// [ListDomains][google.cloud.managedidentities.v1.ListDomains]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDomainsRequest {
    /// Required. The resource name of the domain location using the form:
    /// `projects/{project_id}/locations/global`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return.
    /// If not specified, a default value of 1000 will be used.
    /// Regardless of the page_size value, the response may include a partial list.
    /// Callers should rely on a response's
    /// [next_page_token][google.cloud.managedidentities.v1.ListDomainsResponse.next_page_token]
    /// to determine if there are additional results to list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The `next_page_token` value returned from a previous ListDomainsRequest
    /// request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. A filter specifying constraints of a list operation.
    /// For example, `Domain.fqdn="mydomain.myorginization"`.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specifies the ordering of results. See
    /// [Sorting
    /// order](https://cloud.google.com/apis/design/design_patterns#sorting_order)
    /// for more information.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Response message for
/// [ListDomains][google.cloud.managedidentities.v1.ListDomains]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDomainsResponse {
    /// A list of Managed Identities Service domains in the project.
    #[prost(message, repeated, tag = "1")]
    pub domains: ::std::vec::Vec<Domain>,
    /// A token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// A list of locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request message for [GetDomain][google.cloud.managedidentities.v1.GetDomain]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDomainRequest {
    /// Required. The domain resource name using the form:
    /// `projects/{project_id}/locations/global/domains/{domain_name}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [UpdateDomain][google.cloud.managedidentities.v1.UpdateDomain]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDomainRequest {
    /// Required. Mask of fields to update. At least one path must be supplied in this
    /// field. The elements of the repeated paths field may only include
    /// fields from [Domain][google.cloud.managedidentities.v1.Domain]:
    ///  * `labels`
    ///  * `locations`
    ///  * `authorized_networks`
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Required. Domain message with updated fields. Only supported fields specified in
    /// update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub domain: ::std::option::Option<Domain>,
}
/// Request message for
/// [DeleteDomain][google.cloud.managedidentities.v1.DeleteDomain]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDomainRequest {
    /// Required. The domain resource name using the form:
    /// `projects/{project_id}/locations/global/domains/{domain_name}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [AttachTrust][google.cloud.managedidentities.v1.AttachTrust]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachTrustRequest {
    /// Required. The resource domain name, project name and location using the form:
    /// `projects/{project_id}/locations/global/domains/{domain_name}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The domain trust resource.
    #[prost(message, optional, tag = "2")]
    pub trust: ::std::option::Option<Trust>,
}
/// Request message for
/// [ReconfigureTrust][google.cloud.managedidentities.v1.ReconfigureTrust]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconfigureTrustRequest {
    /// Required. The resource domain name, project name and location using the form:
    /// `projects/{project_id}/locations/global/domains/{domain_name}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The fully-qualified target domain name which will be in trust with current
    /// domain.
    #[prost(string, tag = "2")]
    pub target_domain_name: std::string::String,
    /// Required. The target DNS server IP addresses to resolve the remote domain involved
    /// in the trust.
    #[prost(string, repeated, tag = "3")]
    pub target_dns_ip_addresses: ::std::vec::Vec<std::string::String>,
}
/// Request message for
/// [DetachTrust][google.cloud.managedidentities.v1.DetachTrust]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetachTrustRequest {
    /// Required. The resource domain name, project name, and location using the form:
    /// `projects/{project_id}/locations/global/domains/{domain_name}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The domain trust resource to removed.
    #[prost(message, optional, tag = "2")]
    pub trust: ::std::option::Option<Trust>,
}
/// Request message for
/// [ValidateTrust][google.cloud.managedidentities.v1.ValidateTrust]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateTrustRequest {
    /// Required. The resource domain name, project name, and location using the form:
    /// `projects/{project_id}/locations/global/domains/{domain_name}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The domain trust to validate trust state for.
    #[prost(message, optional, tag = "2")]
    pub trust: ::std::option::Option<Trust>,
}
#[doc = r" Generated client implementations."]
pub mod managed_identities_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " ## API Overview"]
    #[doc = ""]
    #[doc = " The `managedidentites.googleapis.com` service implements the Google Cloud"]
    #[doc = " Managed Identites API for identity services"]
    #[doc = " (e.g. Microsoft Active Directory)."]
    #[doc = ""]
    #[doc = " The Managed Identities service provides methods to manage"]
    #[doc = " (create/read/update/delete) domains, reset managed identities admin password,"]
    #[doc = " add/remove domain controllers in GCP regions and add/remove VPC peering."]
    #[doc = ""]
    #[doc = " ## Data Model"]
    #[doc = ""]
    #[doc = " The Managed Identities service exposes the following resources:"]
    #[doc = ""]
    #[doc = " * Locations as global, named as follows:"]
    #[doc = "   `projects/{project_id}/locations/global`."]
    #[doc = ""]
    #[doc = " * Domains, named as follows:"]
    #[doc = "   `/projects/{project_id}/locations/global/domain/{domain_name}`."]
    #[doc = ""]
    #[doc = " The `{domain_name}` refers to fully qualified domain name in the customer"]
    #[doc = " project e.g. mydomain.myorganization.com, with the following restrictions:"]
    #[doc = ""]
    #[doc = "  * Must contain only lowercase letters, numbers, periods and hyphens."]
    #[doc = "  * Must start with a letter."]
    #[doc = "  * Must contain between 2-64 characters."]
    #[doc = "  * Must end with a number or a letter."]
    #[doc = "  * Must not start with period."]
    #[doc = "  * First segement length (mydomain form example above) shouldn't exceed"]
    #[doc = "    15 chars."]
    #[doc = "  * The last segment cannot be fully numeric."]
    #[doc = "  * Must be unique within the customer project."]
    pub struct ManagedIdentitiesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ManagedIdentitiesServiceClient<T>
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
        #[doc = " Creates a Microsoft AD domain."]
        pub async fn create_microsoft_ad_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMicrosoftAdDomainRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.managedidentities.v1.ManagedIdentitiesService/CreateMicrosoftAdDomain" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Resets a domain's administrator password."]
        pub async fn reset_admin_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetAdminPasswordRequest>,
        ) -> Result<tonic::Response<super::ResetAdminPasswordResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedidentities.v1.ManagedIdentitiesService/ResetAdminPassword",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists domains in a project."]
        pub async fn list_domains(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDomainsRequest>,
        ) -> Result<tonic::Response<super::ListDomainsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedidentities.v1.ManagedIdentitiesService/ListDomains",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a domain."]
        pub async fn get_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDomainRequest>,
        ) -> Result<tonic::Response<super::Domain>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.managedidentities.v1.ManagedIdentitiesService/GetDomain",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the metadata and configuration of a domain."]
        pub async fn update_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDomainRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.managedidentities.v1.ManagedIdentitiesService/UpdateDomain",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a domain."]
        pub async fn delete_domain(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDomainRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.managedidentities.v1.ManagedIdentitiesService/DeleteDomain",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds an AD trust to a domain."]
        pub async fn attach_trust(
            &mut self,
            request: impl tonic::IntoRequest<super::AttachTrustRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.managedidentities.v1.ManagedIdentitiesService/AttachTrust",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the DNS conditional forwarder."]
        pub async fn reconfigure_trust(
            &mut self,
            request: impl tonic::IntoRequest<super::ReconfigureTrustRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.managedidentities.v1.ManagedIdentitiesService/ReconfigureTrust",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Removes an AD trust."]
        pub async fn detach_trust(
            &mut self,
            request: impl tonic::IntoRequest<super::DetachTrustRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.managedidentities.v1.ManagedIdentitiesService/DetachTrust",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Validates a trust state, that the target domain is reachable, and that the"]
        #[doc = " target domain is able to accept incoming trust requests."]
        pub async fn validate_trust(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateTrustRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.managedidentities.v1.ManagedIdentitiesService/ValidateTrust",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ManagedIdentitiesServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ManagedIdentitiesServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ManagedIdentitiesServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod managed_identities_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ManagedIdentitiesServiceServer."]
    #[async_trait]
    pub trait ManagedIdentitiesService: Send + Sync + 'static {
        #[doc = " Creates a Microsoft AD domain."]
        async fn create_microsoft_ad_domain(
            &self,
            request: tonic::Request<super::CreateMicrosoftAdDomainRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Resets a domain's administrator password."]
        async fn reset_admin_password(
            &self,
            request: tonic::Request<super::ResetAdminPasswordRequest>,
        ) -> Result<tonic::Response<super::ResetAdminPasswordResponse>, tonic::Status>;
        #[doc = " Lists domains in a project."]
        async fn list_domains(
            &self,
            request: tonic::Request<super::ListDomainsRequest>,
        ) -> Result<tonic::Response<super::ListDomainsResponse>, tonic::Status>;
        #[doc = " Gets information about a domain."]
        async fn get_domain(
            &self,
            request: tonic::Request<super::GetDomainRequest>,
        ) -> Result<tonic::Response<super::Domain>, tonic::Status>;
        #[doc = " Updates the metadata and configuration of a domain."]
        async fn update_domain(
            &self,
            request: tonic::Request<super::UpdateDomainRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes a domain."]
        async fn delete_domain(
            &self,
            request: tonic::Request<super::DeleteDomainRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Adds an AD trust to a domain."]
        async fn attach_trust(
            &self,
            request: tonic::Request<super::AttachTrustRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Updates the DNS conditional forwarder."]
        async fn reconfigure_trust(
            &self,
            request: tonic::Request<super::ReconfigureTrustRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Removes an AD trust."]
        async fn detach_trust(
            &self,
            request: tonic::Request<super::DetachTrustRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Validates a trust state, that the target domain is reachable, and that the"]
        #[doc = " target domain is able to accept incoming trust requests."]
        async fn validate_trust(
            &self,
            request: tonic::Request<super::ValidateTrustRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " ## API Overview"]
    #[doc = ""]
    #[doc = " The `managedidentites.googleapis.com` service implements the Google Cloud"]
    #[doc = " Managed Identites API for identity services"]
    #[doc = " (e.g. Microsoft Active Directory)."]
    #[doc = ""]
    #[doc = " The Managed Identities service provides methods to manage"]
    #[doc = " (create/read/update/delete) domains, reset managed identities admin password,"]
    #[doc = " add/remove domain controllers in GCP regions and add/remove VPC peering."]
    #[doc = ""]
    #[doc = " ## Data Model"]
    #[doc = ""]
    #[doc = " The Managed Identities service exposes the following resources:"]
    #[doc = ""]
    #[doc = " * Locations as global, named as follows:"]
    #[doc = "   `projects/{project_id}/locations/global`."]
    #[doc = ""]
    #[doc = " * Domains, named as follows:"]
    #[doc = "   `/projects/{project_id}/locations/global/domain/{domain_name}`."]
    #[doc = ""]
    #[doc = " The `{domain_name}` refers to fully qualified domain name in the customer"]
    #[doc = " project e.g. mydomain.myorganization.com, with the following restrictions:"]
    #[doc = ""]
    #[doc = "  * Must contain only lowercase letters, numbers, periods and hyphens."]
    #[doc = "  * Must start with a letter."]
    #[doc = "  * Must contain between 2-64 characters."]
    #[doc = "  * Must end with a number or a letter."]
    #[doc = "  * Must not start with period."]
    #[doc = "  * First segement length (mydomain form example above) shouldn't exceed"]
    #[doc = "    15 chars."]
    #[doc = "  * The last segment cannot be fully numeric."]
    #[doc = "  * Must be unique within the customer project."]
    #[derive(Debug)]
    pub struct ManagedIdentitiesServiceServer<T: ManagedIdentitiesService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ManagedIdentitiesService> ManagedIdentitiesServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ManagedIdentitiesServiceServer<T>
    where
        T: ManagedIdentitiesService,
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
            match req . uri ( ) . path ( ) { "/google.cloud.managedidentities.v1.ManagedIdentitiesService/CreateMicrosoftAdDomain" => { # [ allow ( non_camel_case_types ) ] struct CreateMicrosoftAdDomainSvc < T : ManagedIdentitiesService > ( pub Arc < T > ) ; impl < T : ManagedIdentitiesService > tonic :: server :: UnaryService < super :: CreateMicrosoftAdDomainRequest > for CreateMicrosoftAdDomainSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateMicrosoftAdDomainRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . create_microsoft_ad_domain ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateMicrosoftAdDomainSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.managedidentities.v1.ManagedIdentitiesService/ResetAdminPassword" => { # [ allow ( non_camel_case_types ) ] struct ResetAdminPasswordSvc < T : ManagedIdentitiesService > ( pub Arc < T > ) ; impl < T : ManagedIdentitiesService > tonic :: server :: UnaryService < super :: ResetAdminPasswordRequest > for ResetAdminPasswordSvc < T > { type Response = super :: ResetAdminPasswordResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ResetAdminPasswordRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . reset_admin_password ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ResetAdminPasswordSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.managedidentities.v1.ManagedIdentitiesService/ListDomains" => { # [ allow ( non_camel_case_types ) ] struct ListDomainsSvc < T : ManagedIdentitiesService > ( pub Arc < T > ) ; impl < T : ManagedIdentitiesService > tonic :: server :: UnaryService < super :: ListDomainsRequest > for ListDomainsSvc < T > { type Response = super :: ListDomainsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListDomainsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . list_domains ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListDomainsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.managedidentities.v1.ManagedIdentitiesService/GetDomain" => { # [ allow ( non_camel_case_types ) ] struct GetDomainSvc < T : ManagedIdentitiesService > ( pub Arc < T > ) ; impl < T : ManagedIdentitiesService > tonic :: server :: UnaryService < super :: GetDomainRequest > for GetDomainSvc < T > { type Response = super :: Domain ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetDomainRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . get_domain ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetDomainSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.managedidentities.v1.ManagedIdentitiesService/UpdateDomain" => { # [ allow ( non_camel_case_types ) ] struct UpdateDomainSvc < T : ManagedIdentitiesService > ( pub Arc < T > ) ; impl < T : ManagedIdentitiesService > tonic :: server :: UnaryService < super :: UpdateDomainRequest > for UpdateDomainSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateDomainRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . update_domain ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateDomainSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.managedidentities.v1.ManagedIdentitiesService/DeleteDomain" => { # [ allow ( non_camel_case_types ) ] struct DeleteDomainSvc < T : ManagedIdentitiesService > ( pub Arc < T > ) ; impl < T : ManagedIdentitiesService > tonic :: server :: UnaryService < super :: DeleteDomainRequest > for DeleteDomainSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteDomainRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . delete_domain ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteDomainSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.managedidentities.v1.ManagedIdentitiesService/AttachTrust" => { # [ allow ( non_camel_case_types ) ] struct AttachTrustSvc < T : ManagedIdentitiesService > ( pub Arc < T > ) ; impl < T : ManagedIdentitiesService > tonic :: server :: UnaryService < super :: AttachTrustRequest > for AttachTrustSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: AttachTrustRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . attach_trust ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = AttachTrustSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.managedidentities.v1.ManagedIdentitiesService/ReconfigureTrust" => { # [ allow ( non_camel_case_types ) ] struct ReconfigureTrustSvc < T : ManagedIdentitiesService > ( pub Arc < T > ) ; impl < T : ManagedIdentitiesService > tonic :: server :: UnaryService < super :: ReconfigureTrustRequest > for ReconfigureTrustSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ReconfigureTrustRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . reconfigure_trust ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ReconfigureTrustSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.managedidentities.v1.ManagedIdentitiesService/DetachTrust" => { # [ allow ( non_camel_case_types ) ] struct DetachTrustSvc < T : ManagedIdentitiesService > ( pub Arc < T > ) ; impl < T : ManagedIdentitiesService > tonic :: server :: UnaryService < super :: DetachTrustRequest > for DetachTrustSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DetachTrustRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . detach_trust ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DetachTrustSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.managedidentities.v1.ManagedIdentitiesService/ValidateTrust" => { # [ allow ( non_camel_case_types ) ] struct ValidateTrustSvc < T : ManagedIdentitiesService > ( pub Arc < T > ) ; impl < T : ManagedIdentitiesService > tonic :: server :: UnaryService < super :: ValidateTrustRequest > for ValidateTrustSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ValidateTrustRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . validate_trust ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ValidateTrustSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: ManagedIdentitiesService> Clone for ManagedIdentitiesServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ManagedIdentitiesService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
