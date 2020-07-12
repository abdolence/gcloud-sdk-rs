/// Describes a Cloud Function that contains user computation executed in
/// response to an event. It encapsulate function and triggers configurations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudFunction {
    /// A user-defined name of the function. Function names must be unique
    /// globally and match pattern `projects/*/locations/*/functions/*`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. Status of the function deployment.
    #[prost(enumeration = "CloudFunctionStatus", tag = "7")]
    pub status: i32,
    /// Output only. Name of the most recent operation modifying the function. If
    /// the function status is `DEPLOYING` or `DELETING`, then it points to the
    /// active operation.
    #[prost(string, tag = "8")]
    pub latest_operation: std::string::String,
    /// The name of the function (as defined in source code) that will be
    /// executed. Defaults to the resource name suffix, if not specified. For
    /// backward compatibility, if function with given name is not found, then the
    /// system will try to use function named "function".
    /// For Node.js this is name of a function exported by the module specified
    /// in `source_location`.
    #[prost(string, tag = "9")]
    pub entry_point: std::string::String,
    /// The runtime in which to run the function. Required when deploying a new
    /// function, optional when updating an existing function. For a complete
    /// list of possible choices, see the
    /// [`gcloud` command
    /// reference](/sdk/gcloud/reference/functions/deploy#--runtime).
    #[prost(string, tag = "23")]
    pub runtime: std::string::String,
    /// The function execution timeout. Execution is considered failed and
    /// can be terminated if the function is not completed at the end of the
    /// timeout period. Defaults to 60 seconds.
    #[prost(message, optional, tag = "10")]
    pub timeout: ::std::option::Option<::prost_types::Duration>,
    /// The amount of memory in MB available for a function.
    /// Defaults to 256MB.
    #[prost(int32, tag = "11")]
    pub available_memory_mb: i32,
    /// The email of the function's service account. If empty, defaults to
    /// `{project_id}@appspot.gserviceaccount.com`.
    #[prost(string, tag = "13")]
    pub service_account: std::string::String,
    /// Output only. The last update timestamp of a Cloud Function.
    #[prost(message, optional, tag = "15")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The version identifier of the Cloud Function. Each deployment attempt
    /// results in a new version of a function being created.
    #[prost(int64, tag = "20")]
    pub version_id: i64,
    /// Labels associated with this Cloud Function.
    #[prost(map = "string, string", tag = "21")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Environment variables that shall be available during function execution.
    #[prost(map = "string, string", tag = "22")]
    pub environment_variables:
        ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The limit on the maximum number of function instances that may coexist at a
    /// given time.
    #[prost(int32, tag = "24")]
    pub max_instances: i32,
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
    #[prost(string, tag = "25")]
    pub network: std::string::String,
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
    #[prost(string, tag = "26")]
    pub vpc_connector: std::string::String,
    /// The location of the function source code.
    #[prost(oneof = "cloud_function::SourceCode", tags = "14, 3, 18, 16")]
    pub source_code: ::std::option::Option<cloud_function::SourceCode>,
    /// An event that triggers the function.
    #[prost(oneof = "cloud_function::Trigger", tags = "6, 12")]
    pub trigger: ::std::option::Option<cloud_function::Trigger>,
}
pub mod cloud_function {
    /// The location of the function source code.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceCode {
        /// The Google Cloud Storage URL, starting with gs://, pointing to the zip
        /// archive which contains the function.
        #[prost(string, tag = "14")]
        SourceArchiveUrl(std::string::String),
        /// The hosted repository where the function is defined.
        #[prost(message, tag = "3")]
        SourceRepository(super::SourceRepository),
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
        #[prost(string, tag = "18")]
        SourceRepositoryUrl(std::string::String),
        /// The Google Cloud Storage signed URL used for source uploading, generated
        /// by [google.cloud.functions.v1beta2.GenerateUploadUrl][]
        #[prost(string, tag = "16")]
        SourceUploadUrl(std::string::String),
    }
    /// An event that triggers the function.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trigger {
        /// An HTTPS endpoint type of source that can be triggered via URL.
        #[prost(message, tag = "6")]
        HttpsTrigger(super::HttpsTrigger),
        /// A source that fires events in response to a condition in another service.
        #[prost(message, tag = "12")]
        EventTrigger(super::EventTrigger),
    }
}
/// Describes HTTPSTrigger, could be used to connect web hooks to function.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpsTrigger {
    /// Output only. The deployed url for the function.
    #[prost(string, tag = "1")]
    pub url: std::string::String,
}
/// Describes EventTrigger, used to request events be sent from another
/// service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTrigger {
    /// `event_type` names contain the service that is sending an event and the
    /// kind of event that was fired. Must be of the form
    /// `providers/*/eventTypes/*` e.g. Directly handle a Message published to
    /// Google Cloud Pub/Sub `providers/cloud.pubsub/eventTypes/topic.publish`.
    ///
    /// Handle an object changing in Google Cloud Storage:
    /// `providers/cloud.storage/eventTypes/object.change`
    ///
    /// Handle a write to the Firebase Realtime Database:
    /// `providers/google.firebase.database/eventTypes/ref.write`
    #[prost(string, tag = "1")]
    pub event_type: std::string::String,
    /// Which instance of the source's service should send events. E.g. for Pub/Sub
    /// this would be a Pub/Sub topic at `projects/*/topics/*`. For Google Cloud
    /// Storage this would be a bucket at `projects/*/buckets/*`. For any source
    /// that only supports one instance per-project, this should be the name of the
    /// project (`projects/*`)
    #[prost(string, tag = "2")]
    pub resource: std::string::String,
    /// The hostname of the service that should be observed.
    ///
    /// If no string is provided, the default service implementing the API will
    /// be used. For example, `storage.googleapis.com` is the default for all
    /// event types in the `google.storage` namespace.
    #[prost(string, tag = "6")]
    pub service: std::string::String,
    /// Specifies policy for failed executions.
    #[prost(message, optional, tag = "5")]
    pub failure_policy: ::std::option::Option<FailurePolicy>,
}
/// Describes the location of the function source in a remote repository.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceRepository {
    /// URL to the hosted repository where the function is defined. Only paths in
    /// https://source.developers.google.com domain are supported. The path should
    /// contain the name of the repository.
    #[prost(string, tag = "1")]
    pub repository_url: std::string::String,
    /// The path within the repository where the function is defined. The path
    /// should point to the directory where Cloud Functions files are located. Use
    /// "/" if the function is defined directly in the root directory of a
    /// repository.
    #[prost(string, tag = "2")]
    pub source_path: std::string::String,
    /// Output only. The id of the revision that was resolved at the moment of
    /// function creation or update. For example when a user deployed from a
    /// branch, it will be the revision id of the latest change on this branch at
    /// that time. If user deployed from revision then this value will be always
    /// equal to the revision specified by the user.
    #[prost(string, tag = "6")]
    pub deployed_revision: std::string::String,
    /// The version of a function. Defaults to the latest version of the master
    /// branch.
    #[prost(oneof = "source_repository::Version", tags = "3, 4, 5")]
    pub version: ::std::option::Option<source_repository::Version>,
}
pub mod source_repository {
    /// The version of a function. Defaults to the latest version of the master
    /// branch.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        /// The name of the branch from which the function should be fetched.
        #[prost(string, tag = "3")]
        Branch(std::string::String),
        /// The name of the tag that captures the state of the repository from
        /// which the function should be fetched.
        #[prost(string, tag = "4")]
        Tag(std::string::String),
        /// The id of the revision that captures the state of the repository from
        /// which the function should be fetched.
        #[prost(string, tag = "5")]
        Revision(std::string::String),
    }
}
/// Describes the policy in case of function's execution failure.
/// If empty, then defaults to ignoring failures (i.e. not retrying them).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailurePolicy {
    /// Defines the action taken in case of a function execution failure.
    #[prost(oneof = "failure_policy::Action", tags = "1")]
    pub action: ::std::option::Option<failure_policy::Action>,
}
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
    pub location: std::string::String,
    /// Required. Function to be created.
    #[prost(message, optional, tag = "2")]
    pub function: ::std::option::Option<CloudFunction>,
}
/// Request for the `UpdateFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFunctionRequest {
    /// Required. The name of the function to be updated.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. New version of the function.
    #[prost(message, optional, tag = "2")]
    pub function: ::std::option::Option<CloudFunction>,
}
/// Request for the `GetFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFunctionRequest {
    /// Required. The name of the function which details should be obtained.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for the `ListFunctions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsRequest {
    /// Required. The project and location from which the function should be listed,
    /// specified in the format `projects/*/locations/*`
    /// If you want to list functions in all locations, use "-" in place of a
    /// location. When listing functions in all locations, if one or more
    /// location(s) are unreachable, the response will contain functions from all
    /// reachable locations along with the names of any unreachable locations.
    #[prost(string, tag = "1")]
    pub location: std::string::String,
    /// Maximum number of functions to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last
    /// `ListFunctionsResponse`; indicates that
    /// this is a continuation of a prior `ListFunctions` call, and that the
    /// system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for the `ListFunctions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsResponse {
    /// The functions that match the request.
    #[prost(message, repeated, tag = "1")]
    pub functions: ::std::vec::Vec<CloudFunction>,
    /// If not empty, indicates that there may be more functions that match
    /// the request; this value should be passed in a new
    /// [google.cloud.functions.v1beta2.ListFunctionsRequest][google.cloud.functions.v1beta2.ListFunctionsRequest]
    /// to get more functions.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Locations that could not be reached. The response does not include any
    /// functions from these locations.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request for the `DeleteFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFunctionRequest {
    /// Required. The name of the function which should be deleted.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for the `CallFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallFunctionRequest {
    /// Required. The name of the function to be called.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Input to be passed to the function.
    #[prost(string, tag = "2")]
    pub data: std::string::String,
}
/// Response of `CallFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallFunctionResponse {
    /// Execution id of function invocation.
    #[prost(string, tag = "1")]
    pub execution_id: std::string::String,
    /// Result populated for successful execution of synchronous function. Will
    /// not be populated if function does not return a result through context.
    #[prost(string, tag = "2")]
    pub result: std::string::String,
    /// Either system or user-function generated error. Set if execution
    /// was not successful.
    #[prost(string, tag = "3")]
    pub error: std::string::String,
}
/// Request of `GenerateUploadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateUploadUrlRequest {
    /// The project and location in which the Google Cloud Storage signed URL
    /// should be generated, specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
}
/// Response of `GenerateUploadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateUploadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for a
    /// function source code upload. The uploaded file should be a zip archive
    /// which contains a function.
    #[prost(string, tag = "1")]
    pub upload_url: std::string::String,
}
/// Request of `GenerateDownloadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDownloadUrlRequest {
    /// The name of function for which source code Google Cloud Storage signed
    /// URL should be generated.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The optional version of function.
    #[prost(uint64, tag = "2")]
    pub version_id: u64,
}
/// Response of `GenerateDownloadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDownloadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for
    /// function source code download.
    #[prost(string, tag = "1")]
    pub download_url: std::string::String,
}
/// Describes the current stage of a deployment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CloudFunctionStatus {
    /// Status not specified.
    StatusUnspecified = 0,
    /// Successfully deployed.
    Ready = 1,
    /// Not deployed correctly - behavior is undefined. The item should be updated
    /// or deleted to move it out of this state.
    Failed = 2,
    /// Creation or update in progress.
    Deploying = 3,
    /// Deletion in progress.
    Deleting = 4,
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/ListFunctions",
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/GetFunction",
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/CreateFunction",
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/UpdateFunction",
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/DeleteFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Synchronously invokes a deployed Cloud Function. To be used for testing"]
        #[doc = " purposes as very limited traffic is allowed. For more information on"]
        #[doc = " the actual limits refer to [API Calls]("]
        #[doc = " https://cloud.google.com/functions/quotas#rate_limits)."]
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/CallFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a signed URL for uploading a function source code."]
        #[doc = " For more information about the signed URL usage see:"]
        #[doc = " https://cloud.google.com/storage/docs/access-control/signed-urls"]
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/GenerateUploadUrl",
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/GenerateDownloadUrl",
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
#[doc = r" Generated server implementations."]
pub mod cloud_functions_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CloudFunctionsServiceServer."]
    #[async_trait]
    pub trait CloudFunctionsService: Send + Sync + 'static {
        #[doc = " Returns a list of functions that belong to the requested project."]
        async fn list_functions(
            &self,
            request: tonic::Request<super::ListFunctionsRequest>,
        ) -> Result<tonic::Response<super::ListFunctionsResponse>, tonic::Status>;
        #[doc = " Returns a function with the given name from the requested project."]
        async fn get_function(
            &self,
            request: tonic::Request<super::GetFunctionRequest>,
        ) -> Result<tonic::Response<super::CloudFunction>, tonic::Status>;
        #[doc = " Creates a new function. If a function with the given name already exists in"]
        #[doc = " the specified project, the long running operation will return"]
        #[doc = " `ALREADY_EXISTS` error."]
        async fn create_function(
            &self,
            request: tonic::Request<super::CreateFunctionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Updates existing function."]
        async fn update_function(
            &self,
            request: tonic::Request<super::UpdateFunctionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes a function with the given name from the specified project. If the"]
        #[doc = " given function is used by some trigger, the trigger will be updated to"]
        #[doc = " remove this function."]
        async fn delete_function(
            &self,
            request: tonic::Request<super::DeleteFunctionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Synchronously invokes a deployed Cloud Function. To be used for testing"]
        #[doc = " purposes as very limited traffic is allowed. For more information on"]
        #[doc = " the actual limits refer to [API Calls]("]
        #[doc = " https://cloud.google.com/functions/quotas#rate_limits)."]
        async fn call_function(
            &self,
            request: tonic::Request<super::CallFunctionRequest>,
        ) -> Result<tonic::Response<super::CallFunctionResponse>, tonic::Status>;
        #[doc = " Returns a signed URL for uploading a function source code."]
        #[doc = " For more information about the signed URL usage see:"]
        #[doc = " https://cloud.google.com/storage/docs/access-control/signed-urls"]
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
        async fn generate_upload_url(
            &self,
            request: tonic::Request<super::GenerateUploadUrlRequest>,
        ) -> Result<tonic::Response<super::GenerateUploadUrlResponse>, tonic::Status>;
        #[doc = " Returns a signed URL for downloading deployed function source code."]
        #[doc = " The URL is only valid for a limited period and should be used within"]
        #[doc = " minutes after generation."]
        #[doc = " For more information about the signed URL usage see:"]
        #[doc = " https://cloud.google.com/storage/docs/access-control/signed-urls"]
        async fn generate_download_url(
            &self,
            request: tonic::Request<super::GenerateDownloadUrlRequest>,
        ) -> Result<tonic::Response<super::GenerateDownloadUrlResponse>, tonic::Status>;
    }
    #[doc = " A service that application uses to manipulate triggers and functions."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct CloudFunctionsServiceServer<T: CloudFunctionsService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CloudFunctionsService> CloudFunctionsServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for CloudFunctionsServiceServer<T>
    where
        T: CloudFunctionsService,
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/ListFunctions" => {
                    #[allow(non_camel_case_types)]
                    struct ListFunctionsSvc<T: CloudFunctionsService>(pub Arc<T>);
                    impl<T: CloudFunctionsService>
                        tonic::server::UnaryService<super::ListFunctionsRequest>
                        for ListFunctionsSvc<T>
                    {
                        type Response = super::ListFunctionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFunctionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_functions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListFunctionsSvc(inner);
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/GetFunction" => {
                    #[allow(non_camel_case_types)]
                    struct GetFunctionSvc<T: CloudFunctionsService>(pub Arc<T>);
                    impl<T: CloudFunctionsService>
                        tonic::server::UnaryService<super::GetFunctionRequest>
                        for GetFunctionSvc<T>
                    {
                        type Response = super::CloudFunction;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFunctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_function(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetFunctionSvc(inner);
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/CreateFunction" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFunctionSvc<T: CloudFunctionsService>(pub Arc<T>);
                    impl<T: CloudFunctionsService>
                        tonic::server::UnaryService<super::CreateFunctionRequest>
                        for CreateFunctionSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFunctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_function(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateFunctionSvc(inner);
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/UpdateFunction" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFunctionSvc<T: CloudFunctionsService>(pub Arc<T>);
                    impl<T: CloudFunctionsService>
                        tonic::server::UnaryService<super::UpdateFunctionRequest>
                        for UpdateFunctionSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateFunctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_function(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateFunctionSvc(inner);
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/DeleteFunction" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteFunctionSvc<T: CloudFunctionsService>(pub Arc<T>);
                    impl<T: CloudFunctionsService>
                        tonic::server::UnaryService<super::DeleteFunctionRequest>
                        for DeleteFunctionSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteFunctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_function(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteFunctionSvc(inner);
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/CallFunction" => {
                    #[allow(non_camel_case_types)]
                    struct CallFunctionSvc<T: CloudFunctionsService>(pub Arc<T>);
                    impl<T: CloudFunctionsService>
                        tonic::server::UnaryService<super::CallFunctionRequest>
                        for CallFunctionSvc<T>
                    {
                        type Response = super::CallFunctionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CallFunctionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.call_function(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CallFunctionSvc(inner);
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/GenerateUploadUrl" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateUploadUrlSvc<T: CloudFunctionsService>(pub Arc<T>);
                    impl<T: CloudFunctionsService>
                        tonic::server::UnaryService<super::GenerateUploadUrlRequest>
                        for GenerateUploadUrlSvc<T>
                    {
                        type Response = super::GenerateUploadUrlResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateUploadUrlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.generate_upload_url(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GenerateUploadUrlSvc(inner);
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
                "/google.cloud.functions.v1beta2.CloudFunctionsService/GenerateDownloadUrl" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateDownloadUrlSvc<T: CloudFunctionsService>(pub Arc<T>);
                    impl<T: CloudFunctionsService>
                        tonic::server::UnaryService<super::GenerateDownloadUrlRequest>
                        for GenerateDownloadUrlSvc<T>
                    {
                        type Response = super::GenerateDownloadUrlResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateDownloadUrlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.generate_download_url(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GenerateDownloadUrlSvc(inner);
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
    impl<T: CloudFunctionsService> Clone for CloudFunctionsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CloudFunctionsService> Clone for _Inner<T> {
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
/// Metadata describing an [Operation][google.longrunning.Operation]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadataV1Beta2 {
    /// Target of the operation - for example
    /// projects/project-1/locations/region-1/functions/function-1
    #[prost(string, tag = "1")]
    pub target: std::string::String,
    /// Type of operation.
    #[prost(enumeration = "OperationType", tag = "2")]
    pub r#type: i32,
    /// The original request that started the operation.
    #[prost(message, optional, tag = "3")]
    pub request: ::std::option::Option<::prost_types::Any>,
    /// Version id of the function created or updated by an API call.
    /// This field is only populated for Create and Update operations.
    #[prost(int64, tag = "4")]
    pub version_id: i64,
    /// The last update timestamp of the operation.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
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
