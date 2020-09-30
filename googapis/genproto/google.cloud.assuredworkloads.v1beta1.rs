/// Request for creating a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkloadRequest {
    /// Required. The resource name of the new Workload's parent.
    /// Must be of the form `organizations/{org_id}/locations/{location_id}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Assured Workload to create
    #[prost(message, optional, tag = "2")]
    pub workload: ::std::option::Option<Workload>,
    /// Optional. A identifier associated with the workload and underlying projects which
    /// allows for the break down of billing costs for a workload. The value
    /// provided for the identifier will add a label to the workload and contained
    /// projects with the identifier as the value.
    #[prost(string, tag = "3")]
    pub external_id: std::string::String,
}
/// Request for Updating a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkloadRequest {
    /// Required. The workload to update.
    /// The workload’s `name` field is used to identify the workload to be updated.
    /// Format:
    /// organizations/{org_id}/locations/{location_id}/workloads/{workload_id}
    #[prost(message, optional, tag = "1")]
    pub workload: ::std::option::Option<Workload>,
    /// Required. The list of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request for deleting a Workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkloadRequest {
    /// Required. The `name` field is used to identify the workload.
    /// Format:
    /// organizations/{org_id}/locations/{location_id}/workloads/{workload_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The etag of the workload.
    /// If this is provided, it must match the server's etag.
    #[prost(string, tag = "2")]
    pub etag: std::string::String,
}
/// Request for fetching a workload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkloadRequest {
    /// Required. The resource name of the Workload to fetch. This is the workloads's
    /// relative path in the API, formatted as
    /// "organizations/{organization_id}/locations/{location_id}/workloads/{workload_id}".
    /// For example,
    /// "organizations/123/locations/us-east1/workloads/assured-workload-1".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for fetching workloads in an organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadsRequest {
    /// Required. Parent Resource to list workloads from.
    /// Must be of the form `organizations/{org_id}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token returned from previous request. Page token contains context from
    /// previous request. Page token needs to be passed in the second and following
    /// requests.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// A custom filter for filtering by properties of a workload. At this time,
    /// only filtering by labels is supported.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// Response of ListWorkloads endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkloadsResponse {
    /// List of Workloads under a given parent.
    #[prost(message, repeated, tag = "1")]
    pub workloads: ::std::vec::Vec<Workload>,
    /// The next page token. Return empty if reached the last page.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// An Workload object for managing highly regulated workloads of cloud
/// customers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workload {
    /// Optional. The resource name of the workload.
    /// Format:
    /// organizations/{organization}/locations/{location}/workloads/{workload}
    ///
    /// Read-only.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The user-assigned display name of the Workload.
    /// When present it must be between 4 to 30 characters.
    /// Allowed characters are: lowercase and uppercase letters, numbers,
    /// hyphen, single-quote, double-quote, space, and exclamation point.
    ///
    /// Example: My Workload
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Output only. The resources associated with this workload.
    /// These resources will be created when creating the workload.
    /// If any of the projects already exist, the workload creation will fail.
    /// Always read only.
    #[prost(message, repeated, tag = "3")]
    pub resources: ::std::vec::Vec<workload::ResourceInfo>,
    /// Required. Immutable. Compliance Regime associated with this workload.
    #[prost(enumeration = "workload::ComplianceRegime", tag = "4")]
    pub compliance_regime: i32,
    /// Output only. Immutable. The Workload creation timestamp.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Required. Input only. The billing account used for the resources which are
    /// direct children of workload. This billing account is initially associated
    /// with the resources created as part of Workload creation.
    /// After the initial creation of these resources, the customer can change
    /// the assigned billing account.
    /// The resource name has the form
    /// `billingAccounts/{billing_account_id}`. For example,
    /// `billingAccounts/012345-567890-ABCDEF`.
    #[prost(string, tag = "6")]
    pub billing_account: std::string::String,
    /// Optional. ETag of the workload, it is calculated on the basis
    /// of the Workload contents. It will be used in Update & Delete operations.
    #[prost(string, tag = "9")]
    pub etag: std::string::String,
    /// Optional. Labels applied to the workload.
    #[prost(map = "string, string", tag = "10")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Settings specific to the selected [compliance_regime]
    #[prost(oneof = "workload::ComplianceRegimeSettings", tags = "7, 8, 11, 12")]
    pub compliance_regime_settings: ::std::option::Option<workload::ComplianceRegimeSettings>,
}
pub mod workload {
    /// Represent the resources that are children of this Workload.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceInfo {
        /// Resource identifier.
        /// For a project this represents project_number.
        #[prost(int64, tag = "1")]
        pub resource_id: i64,
        /// Indicates the type of resource.
        #[prost(enumeration = "resource_info::ResourceType", tag = "2")]
        pub resource_type: i32,
    }
    pub mod resource_info {
        /// The type of resource.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ResourceType {
            /// Unknown resource type.
            Unspecified = 0,
            /// Consumer project.
            ConsumerProject = 1,
            /// Consumer project containing encryption keys.
            EncryptionKeysProject = 2,
        }
    }
    /// Settings specific to the Key Management Service.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KmsSettings {
        /// Required. Input only. Immutable. The time at which the Key Management Service will automatically create a
        /// new version of the crypto key and mark it as the primary.
        #[prost(message, optional, tag = "1")]
        pub next_rotation_time: ::std::option::Option<::prost_types::Timestamp>,
        /// Required. Input only. Immutable. [next_rotation_time] will be advanced by this period when the Key
        /// Management Service automatically rotates a key. Must be at least 24 hours
        /// and at most 876,000 hours.
        #[prost(message, optional, tag = "2")]
        pub rotation_period: ::std::option::Option<::prost_types::Duration>,
    }
    /// Settings specific to resources needed for IL4.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Il4Settings {
        /// Required. Input only. Immutable. Settings used to create a CMEK crypto key.
        #[prost(message, optional, tag = "1")]
        pub kms_settings: ::std::option::Option<KmsSettings>,
    }
    /// Settings specific to resources needed for CJIS.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CjisSettings {
        /// Required. Input only. Immutable. Settings used to create a CMEK crypto key.
        #[prost(message, optional, tag = "1")]
        pub kms_settings: ::std::option::Option<KmsSettings>,
    }
    /// Settings specific to resources needed for FedRAMP High.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FedrampHighSettings {
        /// Required. Input only. Immutable. Settings used to create a CMEK crypto key.
        #[prost(message, optional, tag = "1")]
        pub kms_settings: ::std::option::Option<KmsSettings>,
    }
    /// Settings specific to resources needed for FedRAMP Moderate.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FedrampModerateSettings {
        /// Required. Input only. Immutable. Settings used to create a CMEK crypto key.
        #[prost(message, optional, tag = "1")]
        pub kms_settings: ::std::option::Option<KmsSettings>,
    }
    /// Supported Compliance Regimes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ComplianceRegime {
        /// Unknown compliance regime.
        Unspecified = 0,
        /// Information protection as per DoD IL4 requirements.
        Il4 = 1,
        /// Criminal Justice Information Services (CJIS) Security policies.
        Cjis = 2,
        /// FedRAMP High data protection controls
        FedrampHigh = 3,
        /// FedRAMP Moderate data protection controls
        FedrampModerate = 4,
    }
    /// Settings specific to the selected [compliance_regime]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ComplianceRegimeSettings {
        /// Required. Input only. Immutable. Settings specific to resources needed for IL4.
        #[prost(message, tag = "7")]
        Il4Settings(Il4Settings),
        /// Required. Input only. Immutable. Settings specific to resources needed for CJIS.
        #[prost(message, tag = "8")]
        CjisSettings(CjisSettings),
        /// Required. Input only. Immutable. Settings specific to resources needed for FedRAMP High.
        #[prost(message, tag = "11")]
        FedrampHighSettings(FedrampHighSettings),
        /// Required. Input only. Immutable. Settings specific to resources needed for FedRAMP Moderate.
        #[prost(message, tag = "12")]
        FedrampModerateSettings(FedrampModerateSettings),
    }
}
/// Operation metadata to give request details of CreateWorkload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkloadOperationMetadata {
    /// Optional. Time when the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. The display name of the workload.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Optional. The parent of the workload.
    #[prost(string, tag = "3")]
    pub parent: std::string::String,
    /// Optional. Compliance controls that should be applied to the resources managed by
    /// the workload.
    #[prost(enumeration = "workload::ComplianceRegime", tag = "4")]
    pub compliance_regime: i32,
}
#[doc = r" Generated client implementations."]
pub mod assured_workloads_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage AssuredWorkloads."]
    pub struct AssuredWorkloadsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssuredWorkloadsServiceClient<T>
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
        #[doc = " Creates Assured Workload."]
        pub async fn create_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkloadRequest>,
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
                "/google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService/CreateWorkload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing workload."]
        #[doc = " Currently allows updating of workload display_name and labels."]
        #[doc = " For force updates don't set etag field in the Workload."]
        #[doc = " Only one update operation per workload can be in progress."]
        pub async fn update_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkloadRequest>,
        ) -> Result<tonic::Response<super::Workload>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService/UpdateWorkload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the workload. Make sure that workload's direct children are already"]
        #[doc = " in a deleted state, otherwise the request will fail with a"]
        #[doc = " FAILED_PRECONDITION error."]
        pub async fn delete_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkloadRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService/DeleteWorkload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets Assured Workload associated with a CRM Node"]
        pub async fn get_workload(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkloadRequest>,
        ) -> Result<tonic::Response<super::Workload>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService/GetWorkload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Assured Workloads under a CRM Node."]
        pub async fn list_workloads(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkloadsRequest>,
        ) -> Result<tonic::Response<super::ListWorkloadsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.assuredworkloads.v1beta1.AssuredWorkloadsService/ListWorkloads",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AssuredWorkloadsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AssuredWorkloadsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AssuredWorkloadsServiceClient {{ ... }}")
        }
    }
}
