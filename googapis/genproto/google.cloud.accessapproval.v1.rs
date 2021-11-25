/// Home office and physical location of the principal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLocations {
    /// The "home office" location of the principal. A two-letter country code
    /// (ISO 3166-1 alpha-2), such as "US", "DE" or "GB" or a region code. In some
    /// limited situations Google systems may refer refer to a region code instead
    /// of a country code.
    /// Possible Region Codes:
    ///
    /// - ASI: Asia
    /// - EUR: Europe
    /// - OCE: Oceania
    /// - AFR: Africa
    /// - NAM: North America
    /// - SAM: South America
    /// - ANT: Antarctica
    /// - ANY: Any location
    ///
    #[prost(string, tag = "1")]
    pub principal_office_country: ::prost::alloc::string::String,
    /// Physical location of the principal at the time of the access. A
    /// two-letter country code (ISO 3166-1 alpha-2), such as "US", "DE" or "GB" or
    /// a region code. In some limited situations Google systems may refer refer to
    /// a region code instead of a country code.
    /// Possible Region Codes:
    ///
    /// - ASI: Asia
    /// - EUR: Europe
    /// - OCE: Oceania
    /// - AFR: Africa
    /// - NAM: North America
    /// - SAM: South America
    /// - ANT: Antarctica
    /// - ANY: Any location
    ///
    #[prost(string, tag = "2")]
    pub principal_physical_location_country: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessReason {
    /// Type of access justification.
    #[prost(enumeration = "access_reason::Type", tag = "1")]
    pub r#type: i32,
    /// More detail about certain reason types. See comments for each type above.
    #[prost(string, tag = "2")]
    pub detail: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AccessReason`.
pub mod access_reason {
    /// Type of access justification.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value for proto, shouldn't be used.
        Unspecified = 0,
        /// Customer made a request or raised an issue that required the principal to
        /// access customer data. `detail` is of the form ("#####" is the issue ID):
        ///
        /// - "Feedback Report: #####"
        /// - "Case Number: #####"
        /// - "Case ID: #####"
        /// - "E-PIN Reference: #####"
        /// - "Google-#####"
        /// - "T-#####"
        ///
        CustomerInitiatedSupport = 1,
        /// The principal accessed customer data in order to diagnose or resolve a
        /// suspected issue in services or a known outage. Often this access is used
        /// to confirm that customers are not affected by a suspected service issue
        /// or to remediate a reversible system issue.
        GoogleInitiatedService = 2,
        /// Google initiated service for security, fraud, abuse, or compliance
        /// purposes.
        GoogleInitiatedReview = 3,
    }
}
/// A decision that has been made to approve access to a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveDecision {
    /// The time at which approval was granted.
    #[prost(message, optional, tag = "1")]
    pub approve_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which the approval expires.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A decision that has been made to dismiss an approval request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DismissDecision {
    /// The time at which the approval request was dismissed.
    #[prost(message, optional, tag = "1")]
    pub dismiss_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The properties associated with the resource of the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceProperties {
    /// Whether an approval will exclude the descendants of the resource being
    /// requested.
    #[prost(bool, tag = "1")]
    pub excludes_descendants: bool,
}
/// A request for the customer to approve access to a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApprovalRequest {
    /// The resource name of the request. Format is
    /// "{projects|folders|organizations}/{id}/approvalRequests/{approval_request_id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The resource for which approval is being requested. The format of the
    /// resource name is defined at
    /// <https://cloud.google.com/apis/design/resource_names.> The resource name here
    /// may either be a "full" resource name (e.g.
    /// "//library.googleapis.com/shelves/shelf1/books/book2") or a "relative"
    /// resource name (e.g. "shelves/shelf1/books/book2") as described in the
    /// resource name specification.
    #[prost(string, tag = "2")]
    pub requested_resource_name: ::prost::alloc::string::String,
    /// Properties related to the resource represented by requested_resource_name.
    #[prost(message, optional, tag = "9")]
    pub requested_resource_properties: ::core::option::Option<ResourceProperties>,
    /// The justification for which approval is being requested.
    #[prost(message, optional, tag = "3")]
    pub requested_reason: ::core::option::Option<AccessReason>,
    /// The locations for which approval is being requested.
    #[prost(message, optional, tag = "4")]
    pub requested_locations: ::core::option::Option<AccessLocations>,
    /// The time at which approval was requested.
    #[prost(message, optional, tag = "5")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The requested expiration for the approval. If the request is approved,
    /// access will be granted from the time of approval until the expiration time.
    #[prost(message, optional, tag = "6")]
    pub requested_expiration: ::core::option::Option<::prost_types::Timestamp>,
    /// The current decision on the approval request.
    #[prost(oneof = "approval_request::Decision", tags = "7, 8")]
    pub decision: ::core::option::Option<approval_request::Decision>,
}
/// Nested message and enum types in `ApprovalRequest`.
pub mod approval_request {
    /// The current decision on the approval request.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Decision {
        /// Access was approved.
        #[prost(message, tag = "7")]
        Approve(super::ApproveDecision),
        /// The request was dismissed.
        #[prost(message, tag = "8")]
        Dismiss(super::DismissDecision),
    }
}
/// Represents the enrollment of a cloud resource into a specific service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnrolledService {
    /// The product for which Access Approval will be enrolled. Allowed values are
    /// listed below (case-sensitive):
    ///
    /// - all
    /// - appengine.googleapis.com
    /// - bigquery.googleapis.com
    /// - bigtable.googleapis.com
    /// - cloudkms.googleapis.com
    /// - compute.googleapis.com
    /// - dataflow.googleapis.com
    /// - iam.googleapis.com
    /// - pubsub.googleapis.com
    /// - storage.googleapis.com
    ///
    #[prost(string, tag = "1")]
    pub cloud_product: ::prost::alloc::string::String,
    /// The enrollment level of the service.
    #[prost(enumeration = "EnrollmentLevel", tag = "2")]
    pub enrollment_level: i32,
}
/// Settings on a Project/Folder/Organization related to Access Approval.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessApprovalSettings {
    /// The resource name of the settings. Format is one of:
    ///
    /// - "projects/{project_id}/accessApprovalSettings"
    /// - "folders/{folder_id}/accessApprovalSettings"
    /// - "organizations/{organization_id}/accessApprovalSettings"
    ///
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of email addresses to which notifications relating to approval
    /// requests should be sent. Notifications relating to a resource will be sent
    /// to all emails in the settings of ancestor resources of that resource. A
    /// maximum of 50 email addresses are allowed.
    #[prost(string, repeated, tag = "2")]
    pub notification_emails: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of Google Cloud Services for which the given resource has Access
    /// Approval enrolled. Access requests for the resource given by name against
    /// any of these services contained here will be required to have explicit
    /// approval. If name refers to an organization, enrollment can be done for
    /// individual services. If name refers to a folder or project, enrollment can
    /// only be done on an all or nothing basis.
    ///
    /// If a cloud_product is repeated in this list, the first entry will be
    /// honored and all following entries will be discarded. A maximum of 10
    /// enrolled services will be enforced, to be expanded as the set of supported
    /// services is expanded.
    #[prost(message, repeated, tag = "3")]
    pub enrolled_services: ::prost::alloc::vec::Vec<EnrolledService>,
    /// Output only. This field is read only (not settable via
    /// UpdateAccessAccessApprovalSettings method). If the field is true, that
    /// indicates that at least one service is enrolled for Access Approval in one
    /// or more ancestors of the Project or Folder (this field will always be
    /// unset for the organization since organizations do not have ancestors).
    #[prost(bool, tag = "4")]
    pub enrolled_ancestor: bool,
}
/// Request to list approval requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApprovalRequestsMessage {
    /// The parent resource. This may be "projects/{project_id}",
    /// "folders/{folder_id}", or "organizations/{organization_id}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// A filter on the type of approval requests to retrieve. Must be one of the
    /// following values:
    ///
    /// - [not set]: Requests that are pending or have active approvals.
    /// - ALL: All requests.
    /// - PENDING: Only pending requests.
    /// - ACTIVE: Only active (i.e. currently approved) requests.
    /// - DISMISSED: Only dismissed (including expired) requests.
    ///
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Requested page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// A token identifying the page of results to return.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response to listing of ApprovalRequest objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApprovalRequestsResponse {
    /// Approval request details.
    #[prost(message, repeated, tag = "1")]
    pub approval_requests: ::prost::alloc::vec::Vec<ApprovalRequest>,
    /// Token to retrieve the next page of results, or empty if there are no more.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to get an approval request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApprovalRequestMessage {
    /// Name of the approval request to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to approve an ApprovalRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveApprovalRequestMessage {
    /// Name of the approval request to approve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The expiration time of this approval.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request to dismiss an approval request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DismissApprovalRequestMessage {
    /// Name of the ApprovalRequest to dismiss.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to get access approval settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccessApprovalSettingsMessage {
    /// Name of the AccessApprovalSettings to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to update access approval settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAccessApprovalSettingsMessage {
    /// The new AccessApprovalSettings.
    #[prost(message, optional, tag = "1")]
    pub settings: ::core::option::Option<AccessApprovalSettings>,
    /// The update mask applies to the settings. Only the top level fields of
    /// AccessApprovalSettings (notification_emails & enrolled_services) are
    /// supported. For each field, if it is included, the currently stored value
    /// will be entirely overwritten with the value of the field passed in this
    /// request.
    ///
    /// For the `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    /// If this field is left unset, only the notification_emails field will be
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to delete access approval settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAccessApprovalSettingsMessage {
    /// Name of the AccessApprovalSettings to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the type of enrollment for a given service to Access Approval.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EnrollmentLevel {
    /// Default value for proto, shouldn't be used.
    Unspecified = 0,
    /// Service is enrolled in Access Approval for all requests
    BlockAll = 1,
}
#[doc = r" Generated client implementations."]
pub mod access_approval_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " This API allows a customer to manage accesses to cloud resources by"]
    #[doc = " Google personnel. It defines the following resource model:"]
    #[doc = ""]
    #[doc = " - The API has a collection of"]
    #[doc = "   [ApprovalRequest][google.cloud.accessapproval.v1.ApprovalRequest]"]
    #[doc = "   resources, named `approvalRequests/{approval_request_id}`"]
    #[doc = " - The API has top-level settings per Project/Folder/Organization, named"]
    #[doc = "   `accessApprovalSettings`"]
    #[doc = ""]
    #[doc = " The service also periodically emails a list of recipients, defined at the"]
    #[doc = " Project/Folder/Organization level in the accessApprovalSettings, when there"]
    #[doc = " is a pending ApprovalRequest for them to act on. The ApprovalRequests can"]
    #[doc = " also optionally be published to a Cloud Pub/Sub topic owned by the customer"]
    #[doc = " (for Beta, the Pub/Sub setup is managed manually)."]
    #[doc = ""]
    #[doc = " ApprovalRequests can be approved or dismissed. Google personel can only"]
    #[doc = " access the indicated resource or resources if the request is approved"]
    #[doc = " (subject to some exclusions:"]
    #[doc = " https://cloud.google.com/access-approval/docs/overview#exclusions)."]
    #[doc = ""]
    #[doc = " Note: Using Access Approval functionality will mean that Google may not be"]
    #[doc = " able to meet the SLAs for your chosen products, as any support response times"]
    #[doc = " may be dramatically increased. As such the SLAs do not apply to any service"]
    #[doc = " disruption to the extent impacted by Customer's use of Access Approval. Do"]
    #[doc = " not enable Access Approval for projects where you may require high service"]
    #[doc = " availability and rapid response by Google Cloud Support."]
    #[doc = ""]
    #[doc = " After a request is approved or dismissed, no further action may be taken on"]
    #[doc = " it. Requests with the requested_expiration in the past or with no activity"]
    #[doc = " for 14 days are considered dismissed. When an approval expires, the request"]
    #[doc = " is considered dismissed."]
    #[doc = ""]
    #[doc = " If a request is not approved or dismissed, we call it pending."]
    #[derive(Debug, Clone)]
    pub struct AccessApprovalClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccessApprovalClient<T>
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
        ) -> AccessApprovalClient<InterceptedService<T, F>>
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
            AccessApprovalClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists approval requests associated with a project, folder, or organization."]
        #[doc = " Approval requests can be filtered by state (pending, active, dismissed)."]
        #[doc = " The order is reverse chronological."]
        pub async fn list_approval_requests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApprovalRequestsMessage>,
        ) -> Result<tonic::Response<super::ListApprovalRequestsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.accessapproval.v1.AccessApproval/ListApprovalRequests",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets an approval request. Returns NOT_FOUND if the request does not exist."]
        pub async fn get_approval_request(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApprovalRequestMessage>,
        ) -> Result<tonic::Response<super::ApprovalRequest>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.accessapproval.v1.AccessApproval/GetApprovalRequest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Approves a request and returns the updated ApprovalRequest."]
        #[doc = ""]
        #[doc = " Returns NOT_FOUND if the request does not exist. Returns"]
        #[doc = " FAILED_PRECONDITION if the request exists but is not in a pending state."]
        pub async fn approve_approval_request(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveApprovalRequestMessage>,
        ) -> Result<tonic::Response<super::ApprovalRequest>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.accessapproval.v1.AccessApproval/ApproveApprovalRequest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Dismisses a request. Returns the updated ApprovalRequest."]
        #[doc = ""]
        #[doc = " NOTE: This does not deny access to the resource if another request has been"]
        #[doc = " made and approved. It is equivalent in effect to ignoring the request"]
        #[doc = " altogether."]
        #[doc = ""]
        #[doc = " Returns NOT_FOUND if the request does not exist."]
        #[doc = ""]
        #[doc = " Returns FAILED_PRECONDITION if the request exists but is not in a pending"]
        #[doc = " state."]
        pub async fn dismiss_approval_request(
            &mut self,
            request: impl tonic::IntoRequest<super::DismissApprovalRequestMessage>,
        ) -> Result<tonic::Response<super::ApprovalRequest>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.accessapproval.v1.AccessApproval/DismissApprovalRequest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the settings associated with a project, folder, or organization."]
        pub async fn get_access_approval_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccessApprovalSettingsMessage>,
        ) -> Result<tonic::Response<super::AccessApprovalSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.accessapproval.v1.AccessApproval/GetAccessApprovalSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the settings associated with a project, folder, or organization."]
        #[doc = " Settings to update are determined by the value of field_mask."]
        pub async fn update_access_approval_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAccessApprovalSettingsMessage>,
        ) -> Result<tonic::Response<super::AccessApprovalSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.accessapproval.v1.AccessApproval/UpdateAccessApprovalSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the settings associated with a project, folder, or organization."]
        #[doc = " This will have the effect of disabling Access Approval for the project,"]
        #[doc = " folder, or organization, but only if all ancestors also have Access"]
        #[doc = " Approval disabled. If Access Approval is enabled at a higher level of the"]
        #[doc = " hierarchy, then Access Approval will still be enabled at this level as"]
        #[doc = " the settings are inherited."]
        pub async fn delete_access_approval_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAccessApprovalSettingsMessage>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.accessapproval.v1.AccessApproval/DeleteAccessApprovalSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
