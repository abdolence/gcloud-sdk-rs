/// Describes a Cloud Function that contains user computation executed in
/// response to an event. It encapsulate function and triggers configurations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudFunction {
    /// A user-defined name of the function. Function names must be unique
    /// globally and match pattern `projects/*/locations/*/functions/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User-provided description of a function.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Status of the function deployment.
    #[prost(enumeration = "CloudFunctionStatus", tag = "7")]
    pub status: i32,
    /// The name of the function (as defined in source code) that will be
    /// executed. Defaults to the resource name suffix, if not specified. For
    /// backward compatibility, if function with given name is not found, then the
    /// system will try to use function named "function".
    /// For Node.js this is name of a function exported by the module specified
    /// in `source_location`.
    #[prost(string, tag = "8")]
    pub entry_point: ::prost::alloc::string::String,
    /// The runtime in which to run the function. Required when deploying a new
    /// function, optional when updating an existing function. For a complete
    /// list of possible choices, see the
    /// [`gcloud` command
    /// reference](/sdk/gcloud/reference/functions/deploy#--runtime).
    #[prost(string, tag = "19")]
    pub runtime: ::prost::alloc::string::String,
    /// The function execution timeout. Execution is considered failed and
    /// can be terminated if the function is not completed at the end of the
    /// timeout period. Defaults to 60 seconds.
    #[prost(message, optional, tag = "9")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// The amount of memory in MB available for a function.
    /// Defaults to 256MB.
    #[prost(int32, tag = "10")]
    pub available_memory_mb: i32,
    /// The email of the function's service account. If empty, defaults to
    /// `{project_id}@appspot.gserviceaccount.com`.
    #[prost(string, tag = "11")]
    pub service_account_email: ::prost::alloc::string::String,
    /// Output only. The last update timestamp of a Cloud Function.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The version identifier of the Cloud Function. Each deployment attempt
    /// results in a new version of a function being created.
    #[prost(int64, tag = "14")]
    pub version_id: i64,
    /// Labels associated with this Cloud Function.
    #[prost(map = "string, string", tag = "15")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Environment variables that shall be available during function execution.
    #[prost(map = "string, string", tag = "17")]
    pub environment_variables:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The VPC Network that this cloud function can connect to. It can be
    /// either the fully-qualified URI, or the short name of the network resource.
    /// If the short network name is used, the network must belong to the same
    /// project. Otherwise, it must belong to a project within the same
    /// organization. The format of this field is either
    /// `projects/{project}/global/networks/{network}` or `{network}`, where
    /// {project} is a project id where the network is defined, and {network} is
    /// the short name of the network.
    ///
    /// This field is mutually exclusive with `vpc_connector` and will be replaced
    /// by it.
    ///
    /// See [the VPC documentation](https://cloud.google.com/compute/docs/vpc) for
    /// more information on connecting Cloud projects.
    #[prost(string, tag = "18")]
    pub network: ::prost::alloc::string::String,
    /// The limit on the maximum number of function instances that may coexist at a
    /// given time.
    #[prost(int32, tag = "20")]
    pub max_instances: i32,
    /// The VPC Network Connector that this cloud function can connect to. It can
    /// be either the fully-qualified URI, or the short name of the network
    /// connector resource. The format of this field is
    /// `projects/*/locations/*/connectors/*`
    ///
    /// This field is mutually exclusive with `network` field and will eventually
    /// replace it.
    ///
    /// See [the VPC documentation](https://cloud.google.com/compute/docs/vpc) for
    /// more information on connecting Cloud projects.
    #[prost(string, tag = "22")]
    pub vpc_connector: ::prost::alloc::string::String,
    /// The egress settings for the connector, controlling what traffic is diverted
    /// through it.
    #[prost(enumeration = "cloud_function::VpcConnectorEgressSettings", tag = "23")]
    pub vpc_connector_egress_settings: i32,
    /// The ingress settings for the function, controlling what traffic can reach
    /// it.
    #[prost(enumeration = "cloud_function::IngressSettings", tag = "24")]
    pub ingress_settings: i32,
    /// Output only. The Cloud Build ID of the latest successful deployment of the
    /// function.
    #[prost(string, tag = "27")]
    pub build_id: ::prost::alloc::string::String,
    /// The location of the function source code.
    #[prost(oneof = "cloud_function::SourceCode", tags = "3, 4, 16")]
    pub source_code: ::core::option::Option<cloud_function::SourceCode>,
    /// An event that triggers the function.
    #[prost(oneof = "cloud_function::Trigger", tags = "5, 6")]
    pub trigger: ::core::option::Option<cloud_function::Trigger>,
}
/// Nested message and enum types in `CloudFunction`.
pub mod cloud_function {
    /// Available egress settings.
    ///
    /// This controls what traffic is diverted through the VPC Access Connector
    /// resource. By default PRIVATE_RANGES_ONLY will be used.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VpcConnectorEgressSettings {
        /// Unspecified.
        Unspecified = 0,
        /// Use the VPC Access Connector only for private IP space from RFC1918.
        PrivateRangesOnly = 1,
        /// Force the use of VPC Access Connector for all egress traffic from the
        /// function.
        AllTraffic = 2,
    }
    /// Available ingress settings.
    ///
    /// This controls what traffic can reach the function.
    ///
    /// If unspecified, ALLOW_ALL will be used.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IngressSettings {
        /// Unspecified.
        Unspecified = 0,
        /// Allow HTTP traffic from public and private sources.
        AllowAll = 1,
        /// Allow HTTP traffic from only private VPC sources.
        AllowInternalOnly = 2,
        /// Allow HTTP traffic from private VPC sources and through GCLB.
        AllowInternalAndGclb = 3,
    }
    /// The location of the function source code.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceCode {
        /// The Google Cloud Storage URL, starting with gs://, pointing to the zip
        /// archive which contains the function.
        #[prost(string, tag = "3")]
        SourceArchiveUrl(::prost::alloc::string::String),
        /// **Beta Feature**
        ///
        /// The source repository where a function is hosted.
        #[prost(message, tag = "4")]
        SourceRepository(super::SourceRepository),
        /// The Google Cloud Storage signed URL used for source uploading, generated
        /// by [google.cloud.functions.v1.GenerateUploadUrl][]
        #[prost(string, tag = "16")]
        SourceUploadUrl(::prost::alloc::string::String),
    }
    /// An event that triggers the function.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trigger {
        /// An HTTPS endpoint type of source that can be triggered via URL.
        #[prost(message, tag = "5")]
        HttpsTrigger(super::HttpsTrigger),
        /// A source that fires events in response to a condition in another service.
        #[prost(message, tag = "6")]
        EventTrigger(super::EventTrigger),
    }
}
/// Describes SourceRepository, used to represent parameters related to
/// source repository where a function is hosted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceRepository {
    /// The URL pointing to the hosted repository where the function is defined.
    /// There are supported Cloud Source Repository URLs in the following
    /// formats:
    ///
    /// To refer to a specific commit:
    /// `https://source.developers.google.com/projects/*/repos/*/revisions/*/paths/*`
    /// To refer to a moveable alias (branch):
    /// `https://source.developers.google.com/projects/*/repos/*/moveable-aliases/*/paths/*`
    /// In particular, to refer to HEAD use `master` moveable alias.
    /// To refer to a specific fixed alias (tag):
    /// `https://source.developers.google.com/projects/*/repos/*/fixed-aliases/*/paths/*`
    ///
    /// You may omit `paths/*` if you want to use the main directory.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// Output only. The URL pointing to the hosted repository where the function
    /// were defined at the time of deployment. It always points to a specific
    /// commit in the format described above.
    #[prost(string, tag = "2")]
    pub deployed_url: ::prost::alloc::string::String,
}
/// Describes HttpsTrigger, could be used to connect web hooks to function.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpsTrigger {
    /// Output only. The deployed url for the function.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
/// Describes EventTrigger, used to request events be sent from another
/// service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTrigger {
    /// Required. The type of event to observe. For example:
    /// `providers/cloud.storage/eventTypes/object.change` and
    /// `providers/cloud.pubsub/eventTypes/topic.publish`.
    ///
    /// Event types match pattern `providers/*/eventTypes/*.*`.
    /// The pattern contains:
    ///
    /// 1. namespace: For example, `cloud.storage` and
    ///    `google.firebase.analytics`.
    /// 2. resource type: The type of resource on which event occurs. For
    ///    example, the Google Cloud Storage API includes the type `object`.
    /// 3. action: The action that generates the event. For example, action for
    ///    a Google Cloud Storage Object is 'change'.
    /// These parts are lower case.
    #[prost(string, tag = "1")]
    pub event_type: ::prost::alloc::string::String,
    /// Required. The resource(s) from which to observe events, for example,
    /// `projects/_/buckets/myBucket`.
    ///
    /// Not all syntactically correct values are accepted by all services. For
    /// example:
    ///
    /// 1. The authorization model must support it. Google Cloud Functions
    ///    only allows EventTriggers to be deployed that observe resources in the
    ///    same project as the `CloudFunction`.
    /// 2. The resource type must match the pattern expected for an
    ///    `event_type`. For example, an `EventTrigger` that has an
    ///    `event_type` of "google.pubsub.topic.publish" should have a resource
    ///    that matches Google Cloud Pub/Sub topics.
    ///
    /// Additionally, some services may support short names when creating an
    /// `EventTrigger`. These will always be returned in the normalized "long"
    /// format.
    ///
    /// See each *service's* documentation for supported formats.
    #[prost(string, tag = "2")]
    pub resource: ::prost::alloc::string::String,
    /// The hostname of the service that should be observed.
    ///
    /// If no string is provided, the default service implementing the API will
    /// be used. For example, `storage.googleapis.com` is the default for all
    /// event types in the `google.storage` namespace.
    #[prost(string, tag = "3")]
    pub service: ::prost::alloc::string::String,
    /// Specifies policy for failed executions.
    #[prost(message, optional, tag = "5")]
    pub failure_policy: ::core::option::Option<FailurePolicy>,
}
/// Describes the policy in case of function's execution failure.
/// If empty, then defaults to ignoring failures (i.e. not retrying them).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailurePolicy {
    /// Defines the action taken in case of a function execution failure.
    #[prost(oneof = "failure_policy::Action", tags = "1")]
    pub action: ::core::option::Option<failure_policy::Action>,
}
/// Nested message and enum types in `FailurePolicy`.
pub mod failure_policy {
    /// Describes the retry policy in case of function's execution failure.
    /// A function execution will be retried on any failure.
    /// A failed execution will be retried up to 7 days with an exponential backoff
    /// (capped at 10 seconds).
    /// Retried execution is charged as any other execution.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Retry {}
    /// Defines the action taken in case of a function execution failure.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        /// If specified, then the function will be retried in case of a failure.
        #[prost(message, tag = "1")]
        Retry(Retry),
    }
}
/// Request for the `CreateFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFunctionRequest {
    /// Required. The project and location in which the function should be created, specified
    /// in the format `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    /// Required. Function to be created.
    #[prost(message, optional, tag = "2")]
    pub function: ::core::option::Option<CloudFunction>,
}
/// Request for the `UpdateFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFunctionRequest {
    /// Required. New version of the function.
    #[prost(message, optional, tag = "1")]
    pub function: ::core::option::Option<CloudFunction>,
    /// Required list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the `GetFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFunctionRequest {
    /// Required. The name of the function which details should be obtained.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListFunctions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsRequest {
    /// The project and location from which the function should be listed,
    /// specified in the format `projects/*/locations/*`
    /// If you want to list functions in all locations, use "-" in place of a
    /// location. When listing functions in all locations, if one or more
    /// location(s) are unreachable, the response will contain functions from all
    /// reachable locations along with the names of any unreachable locations.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of functions to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last
    /// `ListFunctionsResponse`; indicates that
    /// this is a continuation of a prior `ListFunctions` call, and that the
    /// system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for the `ListFunctions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsResponse {
    /// The functions that match the request.
    #[prost(message, repeated, tag = "1")]
    pub functions: ::prost::alloc::vec::Vec<CloudFunction>,
    /// If not empty, indicates that there may be more functions that match
    /// the request; this value should be passed in a new
    /// [google.cloud.functions.v1.ListFunctionsRequest][google.cloud.functions.v1.ListFunctionsRequest]
    /// to get more functions.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached. The response does not include any
    /// functions from these locations.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `DeleteFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFunctionRequest {
    /// Required. The name of the function which should be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `CallFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallFunctionRequest {
    /// Required. The name of the function to be called.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Input to be passed to the function.
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// Response of `CallFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallFunctionResponse {
    /// Execution id of function invocation.
    #[prost(string, tag = "1")]
    pub execution_id: ::prost::alloc::string::String,
    /// Result populated for successful execution of synchronous function. Will
    /// not be populated if function does not return a result through context.
    #[prost(string, tag = "2")]
    pub result: ::prost::alloc::string::String,
    /// Either system or user-function generated error. Set if execution
    /// was not successful.
    #[prost(string, tag = "3")]
    pub error: ::prost::alloc::string::String,
}
/// Request of `GenerateSourceUploadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateUploadUrlRequest {
    /// The project and location in which the Google Cloud Storage signed URL
    /// should be generated, specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response of `GenerateSourceUploadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateUploadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for a
    /// function source code upload. The uploaded file should be a zip archive
    /// which contains a function.
    #[prost(string, tag = "1")]
    pub upload_url: ::prost::alloc::string::String,
}
/// Request of `GenerateDownloadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDownloadUrlRequest {
    /// The name of function for which source code Google Cloud Storage signed
    /// URL should be generated.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The optional version of function. If not set, default, current version
    /// is used.
    #[prost(uint64, tag = "2")]
    pub version_id: u64,
}
/// Response of `GenerateDownloadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDownloadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for
    /// function source code download.
    #[prost(string, tag = "1")]
    pub download_url: ::prost::alloc::string::String,
}
/// Describes the current stage of a deployment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CloudFunctionStatus {
    /// Not specified. Invalid state.
    Unspecified = 0,
    /// Function has been successfully deployed and is serving.
    Active = 1,
    /// Function deployment failed and the function isn’t serving.
    Offline = 2,
    /// Function is being created or updated.
    DeployInProgress = 3,
    /// Function is being deleted.
    DeleteInProgress = 4,
    /// Function deployment failed and the function serving state is undefined.
    /// The function should be updated or deleted to move it out of this state.
    Unknown = 5,
}
#[doc = r" Generated client implementations."]
pub mod cloud_functions_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service that application uses to manipulate triggers and functions."]
    pub struct CloudFunctionsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudFunctionsServiceClient<T>
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
        #[doc = " Returns a list of functions that belong to the requested project."]
        pub async fn list_functions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFunctionsRequest>,
        ) -> Result<tonic::Response<super::ListFunctionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.functions.v1.CloudFunctionsService/ListFunctions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a function with the given name from the requested project."]
        pub async fn get_function(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFunctionRequest>,
        ) -> Result<tonic::Response<super::CloudFunction>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.functions.v1.CloudFunctionsService/GetFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new function. If a function with the given name already exists in"]
        #[doc = " the specified project, the long running operation will return"]
        #[doc = " `ALREADY_EXISTS` error."]
        pub async fn create_function(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFunctionRequest>,
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
                "/google.cloud.functions.v1.CloudFunctionsService/CreateFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates existing function."]
        pub async fn update_function(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFunctionRequest>,
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
                "/google.cloud.functions.v1.CloudFunctionsService/UpdateFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a function with the given name from the specified project. If the"]
        #[doc = " given function is used by some trigger, the trigger will be updated to"]
        #[doc = " remove this function."]
        pub async fn delete_function(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFunctionRequest>,
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
                "/google.cloud.functions.v1.CloudFunctionsService/DeleteFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Synchronously invokes a deployed Cloud Function. To be used for testing"]
        #[doc = " purposes as very limited traffic is allowed. For more information on"]
        #[doc = " the actual limits, refer to"]
        #[doc = " [Rate Limits](https://cloud.google.com/functions/quotas#rate_limits)."]
        pub async fn call_function(
            &mut self,
            request: impl tonic::IntoRequest<super::CallFunctionRequest>,
        ) -> Result<tonic::Response<super::CallFunctionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.functions.v1.CloudFunctionsService/CallFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a signed URL for uploading a function source code."]
        #[doc = " For more information about the signed URL usage see:"]
        #[doc = " https://cloud.google.com/storage/docs/access-control/signed-urls."]
        #[doc = " Once the function source code upload is complete, the used signed"]
        #[doc = " URL should be provided in CreateFunction or UpdateFunction request"]
        #[doc = " as a reference to the function source code."]
        #[doc = ""]
        #[doc = " When uploading source code to the generated signed URL, please follow"]
        #[doc = " these restrictions:"]
        #[doc = ""]
        #[doc = " * Source file type should be a zip file."]
        #[doc = " * Source file size should not exceed 100MB limit."]
        #[doc = " * No credentials should be attached - the signed URLs provide access to the"]
        #[doc = "   target bucket using internal service identity; if credentials were"]
        #[doc = "   attached, the identity from the credentials would be used, but that"]
        #[doc = "   identity does not have permissions to upload files to the URL."]
        #[doc = ""]
        #[doc = " When making a HTTP PUT request, these two headers need to be specified:"]
        #[doc = ""]
        #[doc = " * `content-type: application/zip`"]
        #[doc = " * `x-goog-content-length-range: 0,104857600`"]
        #[doc = ""]
        #[doc = " And this header SHOULD NOT be specified:"]
        #[doc = ""]
        #[doc = " * `Authorization: Bearer YOUR_TOKEN`"]
        pub async fn generate_upload_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateUploadUrlRequest>,
        ) -> Result<tonic::Response<super::GenerateUploadUrlResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.functions.v1.CloudFunctionsService/GenerateUploadUrl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a signed URL for downloading deployed function source code."]
        #[doc = " The URL is only valid for a limited period and should be used within"]
        #[doc = " minutes after generation."]
        #[doc = " For more information about the signed URL usage see:"]
        #[doc = " https://cloud.google.com/storage/docs/access-control/signed-urls"]
        pub async fn generate_download_url(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateDownloadUrlRequest>,
        ) -> Result<tonic::Response<super::GenerateDownloadUrlResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.functions.v1.CloudFunctionsService/GenerateDownloadUrl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the IAM access control policy on the specified function."]
        #[doc = " Replaces any existing policy."]
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
                "/google.cloud.functions.v1.CloudFunctionsService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the IAM access control policy for a function."]
        #[doc = " Returns an empty policy if the function exists and does not have a policy"]
        #[doc = " set."]
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
                "/google.cloud.functions.v1.CloudFunctionsService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Tests the specified permissions against the IAM access control policy"]
        #[doc = " for a function."]
        #[doc = " If the function does not exist, this will return an empty set of"]
        #[doc = " permissions, not a NOT_FOUND error."]
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
                "/google.cloud.functions.v1.CloudFunctionsService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CloudFunctionsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CloudFunctionsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CloudFunctionsServiceClient {{ ... }}")
        }
    }
}
/// Metadata describing an [Operation][google.longrunning.Operation]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadataV1 {
    /// Target of the operation - for example
    /// projects/project-1/locations/region-1/functions/function-1
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    /// Type of operation.
    #[prost(enumeration = "OperationType", tag = "2")]
    pub r#type: i32,
    /// The original request that started the operation.
    #[prost(message, optional, tag = "3")]
    pub request: ::core::option::Option<::prost_types::Any>,
    /// Version id of the function created or updated by an API call.
    /// This field is only populated for Create and Update operations.
    #[prost(int64, tag = "4")]
    pub version_id: i64,
    /// The last update timestamp of the operation.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The Cloud Build ID of the function created or updated by an API call.
    /// This field is only populated for Create and Update operations.
    #[prost(string, tag = "6")]
    pub build_id: ::prost::alloc::string::String,
}
/// A type of an operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    /// Unknown operation type.
    OperationUnspecified = 0,
    /// Triggered by CreateFunction call
    CreateFunction = 1,
    /// Triggered by UpdateFunction call
    UpdateFunction = 2,
    /// Triggered by DeleteFunction call.
    DeleteFunction = 3,
}
