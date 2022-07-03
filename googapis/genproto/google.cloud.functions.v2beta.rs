/// Describes a Cloud Function that contains user computation executed in
/// response to an event. It encapsulates function and trigger configurations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Function {
    /// A user-defined name of the function. Function names must be unique
    /// globally and match pattern `projects/*/locations/*/functions/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Describe whether the function is gen1 or gen2.
    #[prost(enumeration = "Environment", tag = "10")]
    pub environment: i32,
    /// User-provided description of a function.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Describes the Build step of the function that builds a container from the
    /// given source.
    #[prost(message, optional, tag = "3")]
    pub build_config: ::core::option::Option<BuildConfig>,
    /// Describes the Service being deployed. Currently deploys services to Cloud
    /// Run (fully managed).
    #[prost(message, optional, tag = "4")]
    pub service_config: ::core::option::Option<ServiceConfig>,
    /// An Eventarc trigger managed by Google Cloud Functions that fires events in
    /// response to a condition in another service.
    #[prost(message, optional, tag = "5")]
    pub event_trigger: ::core::option::Option<EventTrigger>,
    /// Output only. State of the function.
    #[prost(enumeration = "function::State", tag = "6")]
    pub state: i32,
    /// Output only. The last update timestamp of a Cloud Function.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this Cloud Function.
    #[prost(map = "string, string", tag = "8")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. State Messages for this Cloud Function.
    #[prost(message, repeated, tag = "9")]
    pub state_messages: ::prost::alloc::vec::Vec<StateMessage>,
}
/// Nested message and enum types in `Function`.
pub mod function {
    /// Describes the current state of the function.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified. Invalid state.
        Unspecified = 0,
        /// Function has been successfully deployed and is serving.
        Active = 1,
        /// Function deployment failed and the function is not serving.
        Failed = 2,
        /// Function is being created or updated.
        Deploying = 3,
        /// Function is being deleted.
        Deleting = 4,
        /// Function deployment failed and the function serving state is undefined.
        /// The function should be updated or deleted to move it out of this state.
        Unknown = 5,
    }
}
/// Informational messages about the state of the Cloud Function or Operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateMessage {
    /// Severity of the state message.
    #[prost(enumeration = "state_message::Severity", tag = "1")]
    pub severity: i32,
    /// One-word CamelCase type of the state message.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// The message.
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StateMessage`.
pub mod state_message {
    /// Severity of the state message.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// Not specified. Invalid severity.
        Unspecified = 0,
        /// ERROR-level severity.
        Error = 1,
        /// WARNING-level severity.
        Warning = 2,
        /// INFO-level severity.
        Info = 3,
    }
}
/// Location of the source in an archive file in Google Cloud Storage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageSource {
    /// Google Cloud Storage bucket containing the source (see
    /// [Bucket Name
    /// Requirements](<https://cloud.google.com/storage/docs/bucket-naming#requirements>)).
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// Google Cloud Storage object containing the source.
    ///
    /// This object must be a gzipped archive file (`.tar.gz`) containing source to
    /// build.
    #[prost(string, tag = "2")]
    pub object: ::prost::alloc::string::String,
    /// Google Cloud Storage generation for the object. If the generation is
    /// omitted, the latest generation will be used.
    #[prost(int64, tag = "3")]
    pub generation: i64,
}
/// Location of the source in a Google Cloud Source Repository.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepoSource {
    /// ID of the project that owns the Cloud Source Repository. If omitted, the
    /// project ID requesting the build is assumed.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Name of the Cloud Source Repository.
    #[prost(string, tag = "2")]
    pub repo_name: ::prost::alloc::string::String,
    /// Directory, relative to the source root, in which to run the build.
    ///
    /// This must be a relative path. If a step's `dir` is specified and is an
    /// absolute path, this value is ignored for that step's execution.
    /// eg. helloworld (no leading slash allowed)
    #[prost(string, tag = "6")]
    pub dir: ::prost::alloc::string::String,
    /// Only trigger a build if the revision regex does NOT match the revision
    /// regex.
    #[prost(bool, tag = "7")]
    pub invert_regex: bool,
    /// A revision within the Cloud Source Repository must be specified in
    /// one of these ways.
    #[prost(oneof = "repo_source::Revision", tags = "3, 4, 5")]
    pub revision: ::core::option::Option<repo_source::Revision>,
}
/// Nested message and enum types in `RepoSource`.
pub mod repo_source {
    /// A revision within the Cloud Source Repository must be specified in
    /// one of these ways.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Revision {
        /// Regex matching branches to build.
        ///
        /// The syntax of the regular expressions accepted is the syntax accepted by
        /// RE2 and described at <https://github.com/google/re2/wiki/Syntax>
        #[prost(string, tag = "3")]
        BranchName(::prost::alloc::string::String),
        /// Regex matching tags to build.
        ///
        /// The syntax of the regular expressions accepted is the syntax accepted by
        /// RE2 and described at <https://github.com/google/re2/wiki/Syntax>
        #[prost(string, tag = "4")]
        TagName(::prost::alloc::string::String),
        /// Explicit commit SHA to build.
        #[prost(string, tag = "5")]
        CommitSha(::prost::alloc::string::String),
    }
}
/// The location of the function source code.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// Location of the source.
    /// At least one source needs to be provided for the deployment to succeed.
    #[prost(oneof = "source::Source", tags = "1, 2")]
    pub source: ::core::option::Option<source::Source>,
}
/// Nested message and enum types in `Source`.
pub mod source {
    /// Location of the source.
    /// At least one source needs to be provided for the deployment to succeed.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// If provided, get the source from this location in Google Cloud Storage.
        #[prost(message, tag = "1")]
        StorageSource(super::StorageSource),
        /// If provided, get the source from this location in a Cloud Source
        /// Repository.
        #[prost(message, tag = "2")]
        RepoSource(super::RepoSource),
    }
}
/// Provenance of the source. Ways to find the original source, or verify that
/// some source was used for this build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceProvenance {
    /// A copy of the build's `source.storage_source`, if exists, with any
    /// generations resolved.
    #[prost(message, optional, tag = "1")]
    pub resolved_storage_source: ::core::option::Option<StorageSource>,
    /// A copy of the build's `source.repo_source`, if exists, with any
    /// revisions resolved.
    #[prost(message, optional, tag = "2")]
    pub resolved_repo_source: ::core::option::Option<RepoSource>,
}
/// Describes the Build step of the function that builds a container from the
/// given source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildConfig {
    /// Output only. The Cloud Build name of the latest successful deployment of the
    /// function.
    #[prost(string, tag = "1")]
    pub build: ::prost::alloc::string::String,
    /// The runtime in which to run the function. Required when deploying a new
    /// function, optional when updating an existing function. For a complete
    /// list of possible choices, see the
    /// [`gcloud` command
    /// reference](<https://cloud.google.com/sdk/gcloud/reference/functions/deploy#--runtime>).
    #[prost(string, tag = "2")]
    pub runtime: ::prost::alloc::string::String,
    /// The name of the function (as defined in source code) that will be
    /// executed. Defaults to the resource name suffix, if not specified. For
    /// backward compatibility, if function with given name is not found, then the
    /// system will try to use function named "function".
    /// For Node.js this is name of a function exported by the module specified
    /// in `source_location`.
    #[prost(string, tag = "3")]
    pub entry_point: ::prost::alloc::string::String,
    /// The location of the function source code.
    #[prost(message, optional, tag = "4")]
    pub source: ::core::option::Option<Source>,
    /// Output only. A permanent fixed identifier for source.
    #[prost(message, optional, tag = "8")]
    pub source_provenance: ::core::option::Option<SourceProvenance>,
    /// Name of the Cloud Build Custom Worker Pool that should be used to build the
    /// function. The format of this field is
    /// `projects/{project}/locations/{region}/workerPools/{workerPool}` where
    /// {project} and {region} are the project id and region respectively where the
    /// worker pool is defined and {workerPool} is the short name of the worker
    /// pool.
    ///
    /// If the project id is not the same as the function, then the Cloud
    /// Functions Service Agent
    /// (service-<project_number>@gcf-admin-robot.iam.gserviceaccount.com) must be
    /// granted the role Cloud Build Custom Workers Builder
    /// (roles/cloudbuild.customworkers.builder) in the project.
    #[prost(string, tag = "5")]
    pub worker_pool: ::prost::alloc::string::String,
    /// User-provided build-time environment variables for the function
    #[prost(map = "string, string", tag = "6")]
    pub environment_variables:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. User managed repository created in Artifact Registry optionally with a
    /// customer managed encryption key. This is the repository to which the
    /// function docker image will be pushed after it is built by Cloud Build.
    /// If unspecified, GCF will create and use a repository named 'gcf-artifacts'
    /// for every deployed region.
    ///
    /// It must match the pattern
    /// `projects/{project}/locations/{location}/repositories/{repository}`.
    ///
    /// Cross-project repositories are not supported.
    /// Cross-location repositories are not supported.
    /// Repository format must be 'DOCKER'.
    #[prost(string, tag = "7")]
    pub docker_repository: ::prost::alloc::string::String,
}
/// Describes the Service being deployed.
/// Currently Supported : Cloud Run (fully managed).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceConfig {
    /// Output only. Name of the service associated with a Function.
    /// The format of this field is
    /// `projects/{project}/locations/{region}/services/{service}`
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// The function execution timeout. Execution is considered failed and
    /// can be terminated if the function is not completed at the end of the
    /// timeout period. Defaults to 60 seconds.
    #[prost(int32, tag = "2")]
    pub timeout_seconds: i32,
    /// The amount of memory available for a function.
    /// Defaults to 256M. Supported units are k, M, G, Mi, Gi. If no unit is
    /// supplied the value is interpreted as bytes.
    /// See
    /// <https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go>
    /// a full description.
    #[prost(string, tag = "13")]
    pub available_memory: ::prost::alloc::string::String,
    /// Environment variables that shall be available during function execution.
    #[prost(map = "string, string", tag = "4")]
    pub environment_variables:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The limit on the maximum number of function instances that may coexist at a
    /// given time.
    ///
    /// In some cases, such as rapid traffic surges, Cloud Functions may, for a
    /// short period of time, create more instances than the specified max
    /// instances limit. If your function cannot tolerate this temporary behavior,
    /// you may want to factor in a safety margin and set a lower max instances
    /// value than your function can tolerate.
    ///
    /// See the [Max
    /// Instances](<https://cloud.google.com/functions/docs/max-instances>) Guide for
    /// more details.
    #[prost(int32, tag = "5")]
    pub max_instance_count: i32,
    /// The limit on the minimum number of function instances that may coexist at a
    /// given time.
    ///
    /// Function instances are kept in idle state for a short period after they
    /// finished executing the request to reduce cold start time for subsequent
    /// requests. Setting a minimum instance count will ensure that the given
    /// number of instances are kept running in idle state always. This can help
    /// with cold start times when jump in incoming request count occurs after the
    /// idle instance would have been stopped in the default case.
    #[prost(int32, tag = "12")]
    pub min_instance_count: i32,
    /// The Serverless VPC Access connector that this cloud function can connect
    /// to. The format of this field is `projects/*/locations/*/connectors/*`.
    #[prost(string, tag = "6")]
    pub vpc_connector: ::prost::alloc::string::String,
    /// The egress settings for the connector, controlling what traffic is diverted
    /// through it.
    #[prost(enumeration = "service_config::VpcConnectorEgressSettings", tag = "7")]
    pub vpc_connector_egress_settings: i32,
    /// The ingress settings for the function, controlling what traffic can reach
    /// it.
    #[prost(enumeration = "service_config::IngressSettings", tag = "8")]
    pub ingress_settings: i32,
    /// Output only. URI of the Service deployed.
    #[prost(string, tag = "9")]
    pub uri: ::prost::alloc::string::String,
    /// The email of the service's service account. If empty, defaults to
    /// `{project_number}-compute@developer.gserviceaccount.com`.
    #[prost(string, tag = "10")]
    pub service_account_email: ::prost::alloc::string::String,
    /// Whether 100% of traffic is routed to the latest revision.
    /// On CreateFunction and UpdateFunction, when set to true, the revision being
    /// deployed will serve 100% of traffic, ignoring any traffic split settings,
    /// if any. On GetFunction, true will be returned if the latest revision is
    /// serving 100% of traffic.
    #[prost(bool, tag = "16")]
    pub all_traffic_on_latest_revision: bool,
    /// Secret environment variables configuration.
    #[prost(message, repeated, tag = "17")]
    pub secret_environment_variables: ::prost::alloc::vec::Vec<SecretEnvVar>,
    /// Secret volumes configuration.
    #[prost(message, repeated, tag = "19")]
    pub secret_volumes: ::prost::alloc::vec::Vec<SecretVolume>,
    /// Output only. The name of service revision.
    #[prost(string, tag = "18")]
    pub revision: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ServiceConfig`.
pub mod service_config {
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
}
/// Configuration for a secret environment variable. It has the information
/// necessary to fetch the secret value from secret manager and expose it as an
/// environment variable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretEnvVar {
    /// Name of the environment variable.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Project identifier (preferably project number but can also be the
    /// project ID) of the project that contains the secret. If not set, it is
    /// assumed that the secret is in the same project as the function.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Name of the secret in secret manager (not the full resource name).
    #[prost(string, tag = "3")]
    pub secret: ::prost::alloc::string::String,
    /// Version of the secret (version number or the string 'latest'). It is
    /// recommended to use a numeric version for secret environment variables as
    /// any updates to the secret value is not reflected until new instances
    /// start.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
}
/// Configuration for a secret volume. It has the information necessary to fetch
/// the secret value from secret manager and make it available as files mounted
/// at the requested paths within the application container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretVolume {
    /// The path within the container to mount the secret volume. For example,
    /// setting the mount_path as `/etc/secrets` would mount the secret value files
    /// under the `/etc/secrets` directory. This directory will also be completely
    /// shadowed and unavailable to mount any other secrets.
    /// Recommended mount path: /etc/secrets
    #[prost(string, tag = "1")]
    pub mount_path: ::prost::alloc::string::String,
    /// Project identifier (preferably project number but can also be the project
    /// ID) of the project that contains the secret. If not set, it is
    /// assumed that the secret is in the same project as the function.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Name of the secret in secret manager (not the full resource name).
    #[prost(string, tag = "3")]
    pub secret: ::prost::alloc::string::String,
    /// List of secret versions to mount for this secret. If empty, the `latest`
    /// version of the secret will be made available in a file named after the
    /// secret under the mount point.
    #[prost(message, repeated, tag = "4")]
    pub versions: ::prost::alloc::vec::Vec<secret_volume::SecretVersion>,
}
/// Nested message and enum types in `SecretVolume`.
pub mod secret_volume {
    /// Configuration for a single version.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecretVersion {
        /// Version of the secret (version number or the string 'latest'). It is
        /// preferable to use `latest` version with secret volumes as secret value
        /// changes are reflected immediately.
        #[prost(string, tag = "1")]
        pub version: ::prost::alloc::string::String,
        /// Relative path of the file under the mount path where the secret value for
        /// this version will be fetched and made available. For example, setting the
        /// mount_path as '/etc/secrets' and path as `secret_foo` would mount the
        /// secret value file at `/etc/secrets/secret_foo`.
        #[prost(string, tag = "2")]
        pub path: ::prost::alloc::string::String,
    }
}
/// Describes EventTrigger, used to request events to be sent from another
/// service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTrigger {
    /// Output only. The resource name of the Eventarc trigger. The format of this field is
    /// `projects/{project}/locations/{region}/triggers/{trigger}`.
    #[prost(string, tag = "1")]
    pub trigger: ::prost::alloc::string::String,
    /// The region that the trigger will be in. The trigger will only receive
    /// events originating in this region. It can be the same
    /// region as the function, a different region or multi-region, or the global
    /// region. If not provided, defaults to the same region as the function.
    #[prost(string, tag = "2")]
    pub trigger_region: ::prost::alloc::string::String,
    /// Required. The type of event to observe. For example:
    /// `google.cloud.audit.log.v1.written` or
    /// `google.cloud.pubsub.topic.v1.messagePublished`.
    #[prost(string, tag = "3")]
    pub event_type: ::prost::alloc::string::String,
    /// Criteria used to filter events.
    #[prost(message, repeated, tag = "4")]
    pub event_filters: ::prost::alloc::vec::Vec<EventFilter>,
    /// Optional. The name of a Pub/Sub topic in the same project that will be used
    /// as the transport topic for the event delivery. Format:
    /// `projects/{project}/topics/{topic}`.
    ///
    /// This is only valid for events of type
    /// `google.cloud.pubsub.topic.v1.messagePublished`. The topic provided here
    /// will not be deleted at function deletion.
    #[prost(string, tag = "5")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// Optional. The email of the trigger's service account. The service account must have
    /// permission to invoke Cloud Run services, the permission is
    /// `run.routes.invoke`.
    /// If empty, defaults to the Compute Engine default service account:
    /// `{project_number}-compute@developer.gserviceaccount.com`.
    #[prost(string, tag = "6")]
    pub service_account_email: ::prost::alloc::string::String,
    /// Optional. If unset, then defaults to ignoring failures (i.e. not retrying them).
    #[prost(enumeration = "event_trigger::RetryPolicy", tag = "7")]
    pub retry_policy: i32,
    /// Optional. The name of the channel associated with the trigger in
    /// `projects/{project}/locations/{location}/channels/{channel}` format.
    /// You must provide a channel to receive events from Eventarc SaaS partners.
    #[prost(string, tag = "8")]
    pub channel: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EventTrigger`.
pub mod event_trigger {
    /// Describes the retry policy in case of function's execution failure.
    /// Retried execution is charged as any other execution.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RetryPolicy {
        /// Not specified.
        Unspecified = 0,
        /// Do not retry.
        DoNotRetry = 1,
        /// Retry on any failure, retry up to 7 days with an exponential backoff
        /// (capped at 10 seconds).
        Retry = 2,
    }
}
/// Filters events based on exact matches on the CloudEvents attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFilter {
    /// Required. The name of a CloudEvents attribute.
    #[prost(string, tag = "1")]
    pub attribute: ::prost::alloc::string::String,
    /// Required. The value for the attribute.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// Optional. The operator used for matching the events with the value of the
    /// filter. If not specified, only events that have an exact key-value pair
    /// specified in the filter are matched. The only allowed value is
    /// `match-path-pattern`.
    #[prost(string, tag = "3")]
    pub operator: ::prost::alloc::string::String,
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
    /// Required. The project and location from which the function should be listed,
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
    /// The filter for Functions that match the filter expression,
    /// following the syntax outlined in <https://google.aip.dev/160.>
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// The sorting order of the resources returned. Value should be a comma
    /// separated list of fields. The default sorting oder is ascending.
    /// See <https://google.aip.dev/132#ordering.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the `ListFunctions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFunctionsResponse {
    /// The functions that match the request.
    #[prost(message, repeated, tag = "1")]
    pub functions: ::prost::alloc::vec::Vec<Function>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached. The response does not include any
    /// functions from these locations.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the `CreateFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFunctionRequest {
    /// Required. The project and location in which the function should be created, specified
    /// in the format `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Function to be created.
    #[prost(message, optional, tag = "2")]
    pub function: ::core::option::Option<Function>,
    /// The ID to use for the function, which will become the final component of
    /// the function's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are /\[a-z][0-9\]-/.
    #[prost(string, tag = "3")]
    pub function_id: ::prost::alloc::string::String,
}
/// Request for the `UpdateFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFunctionRequest {
    /// Required. New version of the function.
    #[prost(message, optional, tag = "1")]
    pub function: ::core::option::Option<Function>,
    /// The list of fields to be updated.
    /// If no field mask is provided, all provided fields in the request will be
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for the `DeleteFunction` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFunctionRequest {
    /// Required. The name of the function which should be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request of `GenerateSourceUploadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateUploadUrlRequest {
    /// Required. The project and location in which the Google Cloud Storage signed URL
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
    /// The location of the source code in the upload bucket.
    ///
    /// Once the archive is uploaded using the `upload_url` use this field to
    /// set the `function.build_config.source.storage_source`
    /// during CreateFunction and UpdateFunction.
    ///
    /// Generation defaults to 0, as Cloud Storage provides a new generation only
    /// upon uploading a new object or version of an object.
    #[prost(message, optional, tag = "2")]
    pub storage_source: ::core::option::Option<StorageSource>,
}
/// Request of `GenerateDownloadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDownloadUrlRequest {
    /// Required. The name of function for which source code Google Cloud Storage signed
    /// URL should be generated.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response of `GenerateDownloadUrl` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDownloadUrlResponse {
    /// The generated Google Cloud Storage signed URL that should be used for
    /// function source code download.
    #[prost(string, tag = "1")]
    pub download_url: ::prost::alloc::string::String,
}
/// Request for the `ListRuntimes` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimesRequest {
    /// Required. The project and location from which the runtimes should be listed,
    /// specified in the format `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The filter for Runtimes that match the filter expression,
    /// following the syntax outlined in <https://google.aip.dev/160.>
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
}
/// Response for the `ListRuntimes` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuntimesResponse {
    /// The runtimes that match the request.
    #[prost(message, repeated, tag = "1")]
    pub runtimes: ::prost::alloc::vec::Vec<list_runtimes_response::Runtime>,
}
/// Nested message and enum types in `ListRuntimesResponse`.
pub mod list_runtimes_response {
    /// Describes a runtime and any special information (e.g., deprecation status)
    /// related to it.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Runtime {
        /// The name of the runtime, e.g., 'go113', 'nodejs12', etc.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The user facing name, eg 'Go 1.13', 'Node.js 12', etc.
        #[prost(string, tag = "5")]
        pub display_name: ::prost::alloc::string::String,
        /// The stage of life this runtime is in, e.g., BETA, GA, etc.
        #[prost(enumeration = "RuntimeStage", tag = "2")]
        pub stage: i32,
        /// Warning messages, e.g., a deprecation warning.
        #[prost(string, repeated, tag = "3")]
        pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The environment for the runtime.
        #[prost(enumeration = "super::Environment", tag = "4")]
        pub environment: i32,
    }
    /// The various stages that a runtime can be in.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RuntimeStage {
        /// Not specified.
        Unspecified = 0,
        /// The runtime is in development.
        Development = 1,
        /// The runtime is in the Alpha stage.
        Alpha = 2,
        /// The runtime is in the Beta stage.
        Beta = 3,
        /// The runtime is generally available.
        Ga = 4,
        /// The runtime is deprecated.
        Deprecated = 5,
        /// The runtime is no longer supported.
        Decommissioned = 6,
    }
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// The original request that started the operation.
    #[prost(message, optional, tag = "8")]
    pub request_resource: ::core::option::Option<::prost_types::Any>,
    /// Mechanism for reporting in-progress stages
    #[prost(message, repeated, tag = "9")]
    pub stages: ::prost::alloc::vec::Vec<Stage>,
}
/// Each Stage of the deployment process
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stage {
    /// Name of the Stage. This will be unique for each Stage.
    #[prost(enumeration = "stage::Name", tag = "1")]
    pub name: i32,
    /// Message describing the Stage
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// Current state of the Stage
    #[prost(enumeration = "stage::State", tag = "3")]
    pub state: i32,
    /// Resource of the Stage
    #[prost(string, tag = "4")]
    pub resource: ::prost::alloc::string::String,
    /// Link to the current Stage resource
    #[prost(string, tag = "5")]
    pub resource_uri: ::prost::alloc::string::String,
    /// State messages from the current Stage.
    #[prost(message, repeated, tag = "6")]
    pub state_messages: ::prost::alloc::vec::Vec<StateMessage>,
}
/// Nested message and enum types in `Stage`.
pub mod stage {
    /// Possible names for a Stage
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Name {
        /// Not specified. Invalid name.
        Unspecified = 0,
        /// Artifact Regsitry Stage
        ArtifactRegistry = 1,
        /// Build Stage
        Build = 2,
        /// Service Stage
        Service = 3,
        /// Trigger Stage
        Trigger = 4,
        /// Service Rollback Stage
        ServiceRollback = 5,
        /// Trigger Rollback Stage
        TriggerRollback = 6,
    }
    /// Possible states for a Stage
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified. Invalid state.
        Unspecified = 0,
        /// Stage has not started.
        NotStarted = 1,
        /// Stage is in progress.
        InProgress = 2,
        /// Stage has completed.
        Complete = 3,
    }
}
/// The environment the function is hosted on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Environment {
    /// Unspecified
    Unspecified = 0,
    /// Gen 1
    Gen1 = 1,
    /// Gen 2
    Gen2 = 2,
}
#[doc = r" Generated client implementations."]
pub mod function_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Google Cloud Functions is used to deploy functions that are executed by"]
    #[doc = " Google in response to various events. Data connected with that event is"]
    #[doc = " passed to a function as the input data."]
    #[doc = ""]
    #[doc = " A **function** is a resource which describes a function that should be"]
    #[doc = " executed and how it is triggered."]
    #[derive(Debug, Clone)]
    pub struct FunctionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FunctionServiceClient<T>
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
        ) -> FunctionServiceClient<InterceptedService<T, F>>
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
            FunctionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns a function with the given name from the requested project."]
        pub async fn get_function(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFunctionRequest>,
        ) -> Result<tonic::Response<super::Function>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.functions.v2beta.FunctionService/GetFunction",
            );
            self.inner.unary(request.into_request(), path, codec).await
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
                "/google.cloud.functions.v2beta.FunctionService/ListFunctions",
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
                "/google.cloud.functions.v2beta.FunctionService/CreateFunction",
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
                "/google.cloud.functions.v2beta.FunctionService/UpdateFunction",
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
                "/google.cloud.functions.v2beta.FunctionService/DeleteFunction",
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
        #[doc = " * No credentials should be attached - the signed URLs provide access to the"]
        #[doc = "   target bucket using internal service identity; if credentials were"]
        #[doc = "   attached, the identity from the credentials would be used, but that"]
        #[doc = "   identity does not have permissions to upload files to the URL."]
        #[doc = ""]
        #[doc = " When making a HTTP PUT request, these two headers need to be specified:"]
        #[doc = ""]
        #[doc = " * `content-type: application/zip`"]
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
                "/google.cloud.functions.v2beta.FunctionService/GenerateUploadUrl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a signed URL for downloading deployed function source code."]
        #[doc = " The URL is only valid for a limited period and should be used within"]
        #[doc = " 30 minutes of generation."]
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
                "/google.cloud.functions.v2beta.FunctionService/GenerateDownloadUrl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a list of runtimes that are supported for the requested project."]
        pub async fn list_runtimes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuntimesRequest>,
        ) -> Result<tonic::Response<super::ListRuntimesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.functions.v2beta.FunctionService/ListRuntimes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
