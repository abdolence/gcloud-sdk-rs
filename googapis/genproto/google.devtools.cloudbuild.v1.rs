/// Specifies a build to retry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryBuildRequest {
    /// The name of the `Build` to retry.
    /// Format: `projects/{project}/locations/{location}/builds/{build}`
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Build ID of the original build.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// Specifies a build trigger to run and the source to use.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunBuildTriggerRequest {
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. ID of the trigger.
    #[prost(string, tag = "2")]
    pub trigger_id: ::prost::alloc::string::String,
    /// Source to build against this trigger.
    #[prost(message, optional, tag = "3")]
    pub source: ::core::option::Option<RepoSource>,
}
/// Location of the source in an archive file in Google Cloud Storage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageSource {
    /// Google Cloud Storage bucket containing the source (see
    /// [Bucket Name
    /// Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
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
    #[prost(string, tag = "7")]
    pub dir: ::prost::alloc::string::String,
    /// Only trigger a build if the revision regex does NOT match the revision
    /// regex.
    #[prost(bool, tag = "8")]
    pub invert_regex: bool,
    /// Substitutions to use in a triggered build.
    /// Should only be used with RunBuildTrigger
    #[prost(map = "string, string", tag = "9")]
    pub substitutions:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
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
        /// RE2 and described at https://github.com/google/re2/wiki/Syntax
        #[prost(string, tag = "3")]
        BranchName(::prost::alloc::string::String),
        /// Regex matching tags to build.
        ///
        /// The syntax of the regular expressions accepted is the syntax accepted by
        /// RE2 and described at https://github.com/google/re2/wiki/Syntax
        #[prost(string, tag = "4")]
        TagName(::prost::alloc::string::String),
        /// Explicit commit SHA to build.
        #[prost(string, tag = "5")]
        CommitSha(::prost::alloc::string::String),
    }
}
/// Location of the source in a supported storage service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// Location of source.
    #[prost(oneof = "source::Source", tags = "2, 3")]
    pub source: ::core::option::Option<source::Source>,
}
/// Nested message and enum types in `Source`.
pub mod source {
    /// Location of source.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// If provided, get the source from this location in Google Cloud Storage.
        #[prost(message, tag = "2")]
        StorageSource(super::StorageSource),
        /// If provided, get the source from this location in a Cloud Source
        /// Repository.
        #[prost(message, tag = "3")]
        RepoSource(super::RepoSource),
    }
}
/// An image built by the pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuiltImage {
    /// Name used to push the container image to Google Container Registry, as
    /// presented to `docker push`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Docker Registry 2.0 digest.
    #[prost(string, tag = "3")]
    pub digest: ::prost::alloc::string::String,
    /// Output only. Stores timing information for pushing the specified image.
    #[prost(message, optional, tag = "4")]
    pub push_timing: ::core::option::Option<TimeSpan>,
}
/// A step in the build pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildStep {
    /// Required. The name of the container image that will run this particular
    /// build step.
    ///
    /// If the image is available in the host's Docker daemon's cache, it
    /// will be run directly. If not, the host will attempt to pull the image
    /// first, using the builder service account's credentials if necessary.
    ///
    /// The Docker daemon's cache will already have the latest versions of all of
    /// the officially supported build steps
    /// ([https://github.com/GoogleCloudPlatform/cloud-builders](https://github.com/GoogleCloudPlatform/cloud-builders)).
    /// The Docker daemon will also have cached many of the layers for some popular
    /// images, like "ubuntu", "debian", but they will be refreshed at the time you
    /// attempt to use them.
    ///
    /// If you built an image in a previous build step, it will be stored in the
    /// host's Docker daemon's cache and is available to use as the name for a
    /// later build step.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of environment variable definitions to be used when running a step.
    ///
    /// The elements are of the form "KEY=VALUE" for the environment variable "KEY"
    /// being given the value "VALUE".
    #[prost(string, repeated, tag = "2")]
    pub env: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of arguments that will be presented to the step when it is started.
    ///
    /// If the image used to run the step's container has an entrypoint, the `args`
    /// are used as arguments to that entrypoint. If the image does not define
    /// an entrypoint, the first element in args is used as the entrypoint,
    /// and the remainder will be used as arguments.
    #[prost(string, repeated, tag = "3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Working directory to use when running this step's container.
    ///
    /// If this value is a relative path, it is relative to the build's working
    /// directory. If this value is absolute, it may be outside the build's working
    /// directory, in which case the contents of the path may not be persisted
    /// across build step executions, unless a `volume` for that path is specified.
    ///
    /// If the build specifies a `RepoSource` with `dir` and a step with a `dir`,
    /// which specifies an absolute path, the `RepoSource` `dir` is ignored for
    /// the step's execution.
    #[prost(string, tag = "4")]
    pub dir: ::prost::alloc::string::String,
    /// Unique identifier for this build step, used in `wait_for` to
    /// reference this build step as a dependency.
    #[prost(string, tag = "5")]
    pub id: ::prost::alloc::string::String,
    /// The ID(s) of the step(s) that this build step depends on.
    /// This build step will not start until all the build steps in `wait_for`
    /// have completed successfully. If `wait_for` is empty, this build step will
    /// start when all previous build steps in the `Build.Steps` list have
    /// completed successfully.
    #[prost(string, repeated, tag = "6")]
    pub wait_for: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Entrypoint to be used instead of the build step image's default entrypoint.
    /// If unset, the image's default entrypoint is used.
    #[prost(string, tag = "7")]
    pub entrypoint: ::prost::alloc::string::String,
    /// A list of environment variables which are encrypted using a Cloud Key
    /// Management Service crypto key. These values must be specified in the
    /// build's `Secret`.
    #[prost(string, repeated, tag = "8")]
    pub secret_env: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of volumes to mount into the build step.
    ///
    /// Each volume is created as an empty volume prior to execution of the
    /// build step. Upon completion of the build, volumes and their contents are
    /// discarded.
    ///
    /// Using a named volume in only one step is not valid as it is indicative
    /// of a build request with an incorrect configuration.
    #[prost(message, repeated, tag = "9")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// Output only. Stores timing information for executing this build step.
    #[prost(message, optional, tag = "10")]
    pub timing: ::core::option::Option<TimeSpan>,
    /// Output only. Stores timing information for pulling this build step's
    /// builder image only.
    #[prost(message, optional, tag = "13")]
    pub pull_timing: ::core::option::Option<TimeSpan>,
    /// Time limit for executing this build step. If not defined, the step has no
    /// time limit and will be allowed to continue to run until either it completes
    /// or the build itself times out.
    #[prost(message, optional, tag = "11")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Output only. Status of the build step. At this time, build step status is
    /// only updated on build completion; step status is not updated in real-time
    /// as the build progresses.
    #[prost(enumeration = "build::Status", tag = "12")]
    pub status: i32,
}
/// Volume describes a Docker container volume which is mounted into build steps
/// in order to persist files across build step execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// Name of the volume to mount.
    ///
    /// Volume names must be unique per build step and must be valid names for
    /// Docker volumes. Each named volume must be used by at least two build steps.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Path at which to mount the volume.
    ///
    /// Paths must be absolute and cannot conflict with other volume paths on the
    /// same build step or with certain reserved volume paths.
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
/// Artifacts created by the build pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Results {
    /// Container images that were built as a part of the build.
    #[prost(message, repeated, tag = "2")]
    pub images: ::prost::alloc::vec::Vec<BuiltImage>,
    /// List of build step digests, in the order corresponding to build step
    /// indices.
    #[prost(string, repeated, tag = "3")]
    pub build_step_images: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Path to the artifact manifest. Only populated when artifacts are uploaded.
    #[prost(string, tag = "4")]
    pub artifact_manifest: ::prost::alloc::string::String,
    /// Number of artifacts uploaded. Only populated when artifacts are uploaded.
    #[prost(int64, tag = "5")]
    pub num_artifacts: i64,
    /// List of build step outputs, produced by builder images, in the order
    /// corresponding to build step indices.
    ///
    /// [Cloud Builders](https://cloud.google.com/cloud-build/docs/cloud-builders)
    /// can produce this output by writing to `$BUILDER_OUTPUT/output`.
    /// Only the first 4KB of data is stored.
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub build_step_outputs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Time to push all non-container artifacts.
    #[prost(message, optional, tag = "7")]
    pub artifact_timing: ::core::option::Option<TimeSpan>,
}
/// An artifact that was uploaded during a build. This
/// is a single record in the artifact manifest JSON file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactResult {
    /// The path of an artifact in a Google Cloud Storage bucket, with the
    /// generation number. For example,
    /// `gs://mybucket/path/to/output.jar#generation`.
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    /// The file hash of the artifact.
    #[prost(message, repeated, tag = "2")]
    pub file_hash: ::prost::alloc::vec::Vec<FileHashes>,
}
/// A build resource in the Cloud Build API.
///
/// At a high level, a `Build` describes where to find source code, how to build
/// it (for example, the builder image to run on the source), and where to store
/// the built artifacts.
///
/// Fields can include the following variables, which will be expanded when the
/// build is created:
///
/// - $PROJECT_ID: the project ID of the build.
/// - $PROJECT_NUMBER: the project number of the build.
/// - $BUILD_ID: the autogenerated ID of the build.
/// - $REPO_NAME: the source repository name specified by RepoSource.
/// - $BRANCH_NAME: the branch name specified by RepoSource.
/// - $TAG_NAME: the tag name specified by RepoSource.
/// - $REVISION_ID or $COMMIT_SHA: the commit SHA specified by RepoSource or
///   resolved from the specified branch or tag.
/// - $SHORT_SHA: first 7 characters of $REVISION_ID or $COMMIT_SHA.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Build {
    /// Output only. The 'Build' name with format:
    /// `projects/{project}/locations/{location}/builds/{build}`, where {build}
    /// is a unique identifier generated by the service.
    #[prost(string, tag = "45")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the build.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. ID of the project.
    #[prost(string, tag = "16")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. Status of the build.
    #[prost(enumeration = "build::Status", tag = "2")]
    pub status: i32,
    /// Output only. Customer-readable message about the current status.
    #[prost(string, tag = "24")]
    pub status_detail: ::prost::alloc::string::String,
    /// The location of the source files to build.
    #[prost(message, optional, tag = "3")]
    pub source: ::core::option::Option<Source>,
    /// Required. The operations to be performed on the workspace.
    #[prost(message, repeated, tag = "11")]
    pub steps: ::prost::alloc::vec::Vec<BuildStep>,
    /// Output only. Results of the build.
    #[prost(message, optional, tag = "10")]
    pub results: ::core::option::Option<Results>,
    /// Output only. Time at which the request to create the build was received.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which execution of the build was started.
    #[prost(message, optional, tag = "7")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which execution of the build was finished.
    ///
    /// The difference between finish_time and start_time is the duration of the
    /// build's execution.
    #[prost(message, optional, tag = "8")]
    pub finish_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Amount of time that this build should be allowed to run, to second
    /// granularity. If this amount of time elapses, work on the build will cease
    /// and the build status will be `TIMEOUT`.
    ///
    /// `timeout` starts ticking from `startTime`.
    ///
    /// Default time is ten minutes.
    #[prost(message, optional, tag = "12")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// A list of images to be pushed upon the successful completion of all build
    /// steps.
    ///
    /// The images are pushed using the builder service account's credentials.
    ///
    /// The digests of the pushed images will be stored in the `Build` resource's
    /// results field.
    ///
    /// If any of the images fail to be pushed, the build status is marked
    /// `FAILURE`.
    #[prost(string, repeated, tag = "13")]
    pub images: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// TTL in queue for this build. If provided and the build is enqueued longer
    /// than this value, the build will expire and the build status will be
    /// `EXPIRED`.
    ///
    /// The TTL starts ticking from create_time.
    #[prost(message, optional, tag = "40")]
    pub queue_ttl: ::core::option::Option<::prost_types::Duration>,
    /// Artifacts produced by the build that should be uploaded upon
    /// successful completion of all build steps.
    #[prost(message, optional, tag = "37")]
    pub artifacts: ::core::option::Option<Artifacts>,
    /// Google Cloud Storage bucket where logs should be written (see
    /// [Bucket Name
    /// Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
    /// Logs file names will be of the format `${logs_bucket}/log-${build_id}.txt`.
    #[prost(string, tag = "19")]
    pub logs_bucket: ::prost::alloc::string::String,
    /// Output only. A permanent fixed identifier for source.
    #[prost(message, optional, tag = "21")]
    pub source_provenance: ::core::option::Option<SourceProvenance>,
    /// Output only. The ID of the `BuildTrigger` that triggered this build, if it
    /// was triggered automatically.
    #[prost(string, tag = "22")]
    pub build_trigger_id: ::prost::alloc::string::String,
    /// Special options for this build.
    #[prost(message, optional, tag = "23")]
    pub options: ::core::option::Option<BuildOptions>,
    /// Output only. URL to logs for this build in Google Cloud Console.
    #[prost(string, tag = "25")]
    pub log_url: ::prost::alloc::string::String,
    /// Substitutions data for `Build` resource.
    #[prost(map = "string, string", tag = "29")]
    pub substitutions:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Tags for annotation of a `Build`. These are not docker tags.
    #[prost(string, repeated, tag = "31")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Secrets to decrypt using Cloud Key Management Service.
    /// Note: Secret Manager is the recommended technique
    /// for managing sensitive data with Cloud Build. Use `available_secrets` to
    /// configure builds to access secrets from Secret Manager. For instructions,
    /// see: https://cloud.google.com/cloud-build/docs/securing-builds/use-secrets
    #[prost(message, repeated, tag = "32")]
    pub secrets: ::prost::alloc::vec::Vec<Secret>,
    /// Output only. Stores timing information for phases of the build. Valid keys
    /// are:
    ///
    /// * BUILD: time to execute all build steps
    /// * PUSH: time to push all specified images.
    /// * FETCHSOURCE: time to fetch source.
    ///
    /// If the build does not specify source or images,
    /// these keys will not be included.
    #[prost(map = "string, message", tag = "33")]
    pub timing: ::std::collections::HashMap<::prost::alloc::string::String, TimeSpan>,
    /// IAM service account whose credentials will be used at build runtime.
    /// Must be of the format `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// ACCOUNT can be email address or uniqueId of the service account.
    ///
    /// This field is in beta.
    #[prost(string, tag = "42")]
    pub service_account: ::prost::alloc::string::String,
    /// Secrets and secret environment variables.
    #[prost(message, optional, tag = "47")]
    pub available_secrets: ::core::option::Option<Secrets>,
}
/// Nested message and enum types in `Build`.
pub mod build {
    /// Possible status of a build or build step.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Status of the build is unknown.
        Unknown = 0,
        /// Build or step is queued; work has not yet begun.
        Queued = 1,
        /// Build or step is being executed.
        Working = 2,
        /// Build or step finished successfully.
        Success = 3,
        /// Build or step failed to complete successfully.
        Failure = 4,
        /// Build or step failed due to an internal cause.
        InternalError = 5,
        /// Build or step took longer than was allowed.
        Timeout = 6,
        /// Build or step was canceled by a user.
        Cancelled = 7,
        /// Build was enqueued for longer than the value of `queue_ttl`.
        Expired = 9,
    }
}
/// Artifacts produced by a build that should be uploaded upon
/// successful completion of all build steps.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Artifacts {
    /// A list of images to be pushed upon the successful completion of all build
    /// steps.
    ///
    /// The images will be pushed using the builder service account's credentials.
    ///
    /// The digests of the pushed images will be stored in the Build resource's
    /// results field.
    ///
    /// If any of the images fail to be pushed, the build is marked FAILURE.
    #[prost(string, repeated, tag = "1")]
    pub images: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of objects to be uploaded to Cloud Storage upon successful
    /// completion of all build steps.
    ///
    /// Files in the workspace matching specified paths globs will be uploaded to
    /// the specified Cloud Storage location using the builder service account's
    /// credentials.
    ///
    /// The location and generation of the uploaded objects will be stored in the
    /// Build resource's results field.
    ///
    /// If any objects fail to be pushed, the build is marked FAILURE.
    #[prost(message, optional, tag = "2")]
    pub objects: ::core::option::Option<artifacts::ArtifactObjects>,
}
/// Nested message and enum types in `Artifacts`.
pub mod artifacts {
    /// Files in the workspace to upload to Cloud Storage upon successful
    /// completion of all build steps.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ArtifactObjects {
        /// Cloud Storage bucket and optional object path, in the form
        /// "gs://bucket/path/to/somewhere/". (see [Bucket Name
        /// Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
        ///
        /// Files in the workspace matching any path pattern will be uploaded to
        /// Cloud Storage with this location as a prefix.
        #[prost(string, tag = "1")]
        pub location: ::prost::alloc::string::String,
        /// Path globs used to match files in the build's workspace.
        #[prost(string, repeated, tag = "2")]
        pub paths: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Output only. Stores timing information for pushing all artifact objects.
        #[prost(message, optional, tag = "3")]
        pub timing: ::core::option::Option<super::TimeSpan>,
    }
}
/// Start and end times for a build execution phase.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSpan {
    /// Start of time span.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End of time span.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata for build operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildOperationMetadata {
    /// The build that the operation is tracking.
    #[prost(message, optional, tag = "1")]
    pub build: ::core::option::Option<Build>,
}
/// Provenance of the source. Ways to find the original source, or verify that
/// some source was used for this build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceProvenance {
    /// A copy of the build's `source.storage_source`, if exists, with any
    /// generations resolved.
    #[prost(message, optional, tag = "3")]
    pub resolved_storage_source: ::core::option::Option<StorageSource>,
    /// A copy of the build's `source.repo_source`, if exists, with any
    /// revisions resolved.
    #[prost(message, optional, tag = "6")]
    pub resolved_repo_source: ::core::option::Option<RepoSource>,
    /// Output only. Hash(es) of the build source, which can be used to verify that
    /// the original source integrity was maintained in the build. Note that
    /// `FileHashes` will only be populated if `BuildOptions` has requested a
    /// `SourceProvenanceHash`.
    ///
    /// The keys to this map are file paths used as build source and the values
    /// contain the hash values for those files.
    ///
    /// If the build source came in a single package such as a gzipped tarfile
    /// (`.tar.gz`), the `FileHash` will be for the single path to that file.
    #[prost(map = "string, message", tag = "4")]
    pub file_hashes: ::std::collections::HashMap<::prost::alloc::string::String, FileHashes>,
}
/// Container message for hashes of byte content of files, used in
/// SourceProvenance messages to verify integrity of source input to the build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHashes {
    /// Collection of file hashes.
    #[prost(message, repeated, tag = "1")]
    pub file_hash: ::prost::alloc::vec::Vec<Hash>,
}
/// Container message for hash values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    /// The type of hash that was performed.
    #[prost(enumeration = "hash::HashType", tag = "1")]
    pub r#type: i32,
    /// The hash value.
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Hash`.
pub mod hash {
    /// Specifies the hash algorithm, if any.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HashType {
        /// No hash requested.
        None = 0,
        /// Use a sha256 hash.
        Sha256 = 1,
        /// Use a md5 hash.
        Md5 = 2,
    }
}
/// Secrets and secret environment variables.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secrets {
    /// Secrets in Secret Manager and associated secret environment variable.
    #[prost(message, repeated, tag = "1")]
    pub secret_manager: ::prost::alloc::vec::Vec<SecretManagerSecret>,
    /// Secrets encrypted with KMS key and the associated secret environment
    /// variable.
    #[prost(message, repeated, tag = "2")]
    pub inline: ::prost::alloc::vec::Vec<InlineSecret>,
}
/// Pairs a set of secret environment variables mapped to encrypted
/// values with the Cloud KMS key to use to decrypt the value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InlineSecret {
    /// Resource name of Cloud KMS crypto key to decrypt the encrypted value.
    /// In format: projects/*/locations/*/keyRings/*/cryptoKeys/*
    #[prost(string, tag = "1")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Map of environment variable name to its encrypted value.
    ///
    /// Secret environment variables must be unique across all of a build's
    /// secrets, and must be used by at least one build step. Values can be at most
    /// 64 KB in size. There can be at most 100 secret values across all of a
    /// build's secrets.
    #[prost(map = "string, bytes", tag = "2")]
    pub env_map:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
/// Pairs a secret environment variable with a SecretVersion in Secret Manager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretManagerSecret {
    /// Resource name of the SecretVersion. In format:
    /// projects/*/secrets/*/versions/*
    #[prost(string, tag = "1")]
    pub version_name: ::prost::alloc::string::String,
    /// Environment variable name to associate with the secret.
    /// Secret environment variables must be unique across all of a build's
    /// secrets, and must be used by at least one build step.
    #[prost(string, tag = "2")]
    pub env: ::prost::alloc::string::String,
}
/// Pairs a set of secret environment variables containing encrypted
/// values with the Cloud KMS key to use to decrypt the value.
/// Note: Use `kmsKeyName` with  `available_secrets` instead of using
/// `kmsKeyName` with `secret`. For instructions see:
/// https://cloud.google.com/cloud-build/docs/securing-builds/use-encrypted-credentials.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    /// Cloud KMS key name to use to decrypt these envs.
    #[prost(string, tag = "1")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// Map of environment variable name to its encrypted value.
    ///
    /// Secret environment variables must be unique across all of a build's
    /// secrets, and must be used by at least one build step. Values can be at most
    /// 64 KB in size. There can be at most 100 secret values across all of a
    /// build's secrets.
    #[prost(map = "string, bytes", tag = "3")]
    pub secret_env:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
}
/// Request to create a new build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBuildRequest {
    /// The parent resource where this build will be created.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Build resource to create.
    #[prost(message, optional, tag = "2")]
    pub build: ::core::option::Option<Build>,
}
/// Request to get a build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuildRequest {
    /// The name of the `Build` to retrieve.
    /// Format: `projects/{project}/locations/{location}/builds/{build}`
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. ID of the build.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// Request to list builds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildsRequest {
    /// The parent of the collection of `Builds`.
    /// Format: `projects/{project}/locations/location`
    #[prost(string, tag = "9")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Number of results to return in the list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token for the next page of Builds.
    ///
    /// If unspecified, the first page of results is returned.
    ///
    /// If the token is rejected for any reason, INVALID_ARGUMENT will be thrown.
    /// In this case, the token should be discarded, and pagination should be
    /// restarted from the first page of results.
    ///
    /// See https://google.aip.dev/158 for more.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The raw filter text to constrain the results.
    #[prost(string, tag = "8")]
    pub filter: ::prost::alloc::string::String,
}
/// Response including listed builds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildsResponse {
    /// Builds will be sorted by `create_time`, descending.
    #[prost(message, repeated, tag = "1")]
    pub builds: ::prost::alloc::vec::Vec<Build>,
    /// Token to receive the next page of results.
    /// This will be absent if the end of the response list has been reached.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to cancel an ongoing build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelBuildRequest {
    /// The name of the `Build` to cancel.
    /// Format: `projects/{project}/locations/{location}/builds/{build}`
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. ID of the build.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// Configuration for an automated build in response to source repository
/// changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildTrigger {
    /// Output only. Unique identifier of the trigger.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Human-readable description of this trigger.
    #[prost(string, tag = "10")]
    pub description: ::prost::alloc::string::String,
    /// User-assigned name of the trigger. Must be unique within the project.
    /// Trigger names must meet the following requirements:
    ///
    /// + They must contain only alphanumeric characters and dashes.
    /// + They can be 1-64 characters long.
    /// + They must begin and end with an alphanumeric character.
    #[prost(string, tag = "21")]
    pub name: ::prost::alloc::string::String,
    /// Tags for annotation of a `BuildTrigger`
    #[prost(string, repeated, tag = "19")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Template describing the types of source changes to trigger a build.
    ///
    /// Branch and tag names in trigger templates are interpreted as regular
    /// expressions. Any branch or tag change that matches that regular expression
    /// will trigger a build.
    ///
    /// Mutually exclusive with `github`.
    #[prost(message, optional, tag = "7")]
    pub trigger_template: ::core::option::Option<RepoSource>,
    /// GitHubEventsConfig describes the configuration of a trigger that creates
    /// a build whenever a GitHub event is received.
    ///
    /// Mutually exclusive with `trigger_template`.
    #[prost(message, optional, tag = "13")]
    pub github: ::core::option::Option<GitHubEventsConfig>,
    /// Output only. Time when the trigger was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If true, the trigger will never automatically execute a build.
    #[prost(bool, tag = "9")]
    pub disabled: bool,
    /// Substitutions for Build resource. The keys must match the following
    /// regular expression: `^_[A-Z0-9_]+$`.
    #[prost(map = "string, string", tag = "11")]
    pub substitutions:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// ignored_files and included_files are file glob matches using
    /// https://golang.org/pkg/path/filepath/#Match extended with support for "**".
    ///
    /// If ignored_files and changed files are both empty, then they are
    /// not used to determine whether or not to trigger a build.
    ///
    /// If ignored_files is not empty, then we ignore any files that match
    /// any of the ignored_file globs. If the change has no files that are
    /// outside of the ignored_files globs, then we do not trigger a build.
    #[prost(string, repeated, tag = "15")]
    pub ignored_files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If any of the files altered in the commit pass the ignored_files
    /// filter and included_files is empty, then as far as this filter is
    /// concerned, we should trigger the build.
    ///
    /// If any of the files altered in the commit pass the ignored_files
    /// filter and included_files is not empty, then we make sure that at
    /// least one of those files matches a included_files glob. If not,
    /// then we do not trigger a build.
    #[prost(string, repeated, tag = "16")]
    pub included_files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Template describing the Build request to make when the trigger is matched.
    #[prost(oneof = "build_trigger::BuildTemplate", tags = "4, 8")]
    pub build_template: ::core::option::Option<build_trigger::BuildTemplate>,
}
/// Nested message and enum types in `BuildTrigger`.
pub mod build_trigger {
    /// Template describing the Build request to make when the trigger is matched.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BuildTemplate {
        /// Contents of the build template.
        #[prost(message, tag = "4")]
        Build(super::Build),
        /// Path, from the source root, to the build configuration file
        /// (i.e. cloudbuild.yaml).
        #[prost(string, tag = "8")]
        Filename(::prost::alloc::string::String),
    }
}
/// GitHubEventsConfig describes the configuration of a trigger that creates a
/// build whenever a GitHub event is received.
///
/// This message is experimental.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitHubEventsConfig {
    /// The installationID that emits the GitHub event.
    #[deprecated]
    #[prost(int64, tag = "1")]
    pub installation_id: i64,
    /// Owner of the repository. For example: The owner for
    /// https://github.com/googlecloudplatform/cloud-builders is
    /// "googlecloudplatform".
    #[prost(string, tag = "6")]
    pub owner: ::prost::alloc::string::String,
    /// Name of the repository. For example: The name for
    /// https://github.com/googlecloudplatform/cloud-builders is "cloud-builders".
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    /// Filter describing the types of events to trigger a build.
    /// Currently supported event types: push, pull_request.
    #[prost(oneof = "git_hub_events_config::Event", tags = "4, 5")]
    pub event: ::core::option::Option<git_hub_events_config::Event>,
}
/// Nested message and enum types in `GitHubEventsConfig`.
pub mod git_hub_events_config {
    /// Filter describing the types of events to trigger a build.
    /// Currently supported event types: push, pull_request.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// filter to match changes in pull requests.
        #[prost(message, tag = "4")]
        PullRequest(super::PullRequestFilter),
        /// filter to match changes in refs like branches, tags.
        #[prost(message, tag = "5")]
        Push(super::PushFilter),
    }
}
/// PullRequestFilter contains filter properties for matching GitHub Pull
/// Requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullRequestFilter {
    /// Configure builds to run whether a repository owner or collaborator need to
    /// comment `/gcbrun`.
    #[prost(enumeration = "pull_request_filter::CommentControl", tag = "5")]
    pub comment_control: i32,
    /// If true, branches that do NOT match the git_ref will trigger a build.
    #[prost(bool, tag = "6")]
    pub invert_regex: bool,
    /// Target refs to match.
    /// A target ref is the git reference where the pull request will be applied.
    #[prost(oneof = "pull_request_filter::GitRef", tags = "2")]
    pub git_ref: ::core::option::Option<pull_request_filter::GitRef>,
}
/// Nested message and enum types in `PullRequestFilter`.
pub mod pull_request_filter {
    /// Controls behavior of Pull Request comments.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CommentControl {
        /// Do not require comments on Pull Requests before builds are triggered.
        CommentsDisabled = 0,
        /// Enforce that repository owners or collaborators must comment on Pull
        /// Requests before builds are triggered.
        CommentsEnabled = 1,
        /// Enforce that repository owners or collaborators must comment on external
        /// contributors' Pull Requests before builds are triggered.
        CommentsEnabledForExternalContributorsOnly = 2,
    }
    /// Target refs to match.
    /// A target ref is the git reference where the pull request will be applied.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GitRef {
        /// Regex of branches to match.
        ///
        /// The syntax of the regular expressions accepted is the syntax accepted by
        /// RE2 and described at https://github.com/google/re2/wiki/Syntax
        #[prost(string, tag = "2")]
        Branch(::prost::alloc::string::String),
    }
}
/// Push contains filter properties for matching GitHub git pushes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushFilter {
    /// When true, only trigger a build if the revision regex does NOT match the
    /// git_ref regex.
    #[prost(bool, tag = "4")]
    pub invert_regex: bool,
    /// Modified refs to match.
    /// A modified refs are the refs modified by a git push operation.
    #[prost(oneof = "push_filter::GitRef", tags = "2, 3")]
    pub git_ref: ::core::option::Option<push_filter::GitRef>,
}
/// Nested message and enum types in `PushFilter`.
pub mod push_filter {
    /// Modified refs to match.
    /// A modified refs are the refs modified by a git push operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GitRef {
        /// Regexes matching branches to build.
        ///
        /// The syntax of the regular expressions accepted is the syntax accepted by
        /// RE2 and described at https://github.com/google/re2/wiki/Syntax
        #[prost(string, tag = "2")]
        Branch(::prost::alloc::string::String),
        /// Regexes matching tags to build.
        ///
        /// The syntax of the regular expressions accepted is the syntax accepted by
        /// RE2 and described at https://github.com/google/re2/wiki/Syntax
        #[prost(string, tag = "3")]
        Tag(::prost::alloc::string::String),
    }
}
/// Request to create a new `BuildTrigger`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBuildTriggerRequest {
    /// Required. ID of the project for which to configure automatic builds.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. `BuildTrigger` to create.
    #[prost(message, optional, tag = "2")]
    pub trigger: ::core::option::Option<BuildTrigger>,
}
/// Returns the `BuildTrigger` with the specified ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuildTriggerRequest {
    /// Required. ID of the project that owns the trigger.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Identifier (`id` or `name`) of the `BuildTrigger` to get.
    #[prost(string, tag = "2")]
    pub trigger_id: ::prost::alloc::string::String,
}
/// Request to list existing `BuildTriggers`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildTriggersRequest {
    /// Required. ID of the project for which to list BuildTriggers.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Number of results to return in the list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response containing existing `BuildTriggers`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildTriggersResponse {
    /// `BuildTriggers` for the project, sorted by `create_time` descending.
    #[prost(message, repeated, tag = "1")]
    pub triggers: ::prost::alloc::vec::Vec<BuildTrigger>,
    /// Token to receive the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to delete a `BuildTrigger`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBuildTriggerRequest {
    /// Required. ID of the project that owns the trigger.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. ID of the `BuildTrigger` to delete.
    #[prost(string, tag = "2")]
    pub trigger_id: ::prost::alloc::string::String,
}
/// Request to update an existing `BuildTrigger`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBuildTriggerRequest {
    /// Required. ID of the project that owns the trigger.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. ID of the `BuildTrigger` to update.
    #[prost(string, tag = "2")]
    pub trigger_id: ::prost::alloc::string::String,
    /// Required. `BuildTrigger` to update.
    #[prost(message, optional, tag = "3")]
    pub trigger: ::core::option::Option<BuildTrigger>,
}
/// Optional arguments to enable specific features of builds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildOptions {
    /// Requested hash for SourceProvenance.
    #[prost(enumeration = "hash::HashType", repeated, tag = "1")]
    pub source_provenance_hash: ::prost::alloc::vec::Vec<i32>,
    /// Requested verifiability options.
    #[prost(enumeration = "build_options::VerifyOption", tag = "2")]
    pub requested_verify_option: i32,
    /// Compute Engine machine type on which to run the build.
    #[prost(enumeration = "build_options::MachineType", tag = "3")]
    pub machine_type: i32,
    /// Requested disk size for the VM that runs the build. Note that this is *NOT*
    /// "disk free"; some of the space will be used by the operating system and
    /// build utilities. Also note that this is the minimum disk size that will be
    /// allocated for the build -- the build may run with a larger disk than
    /// requested. At present, the maximum disk size is 1000GB; builds that request
    /// more than the maximum are rejected with an error.
    #[prost(int64, tag = "6")]
    pub disk_size_gb: i64,
    /// Option to specify behavior when there is an error in the substitution
    /// checks.
    ///
    /// NOTE: this is always set to ALLOW_LOOSE for triggered builds and cannot
    /// be overridden in the build configuration file.
    #[prost(enumeration = "build_options::SubstitutionOption", tag = "4")]
    pub substitution_option: i32,
    /// Option to specify whether or not to apply bash style string
    /// operations to the substitutions.
    ///
    /// NOTE: this is always enabled for triggered builds and cannot be
    /// overridden in the build configuration file.
    #[prost(bool, tag = "17")]
    pub dynamic_substitutions: bool,
    /// Option to define build log streaming behavior to Google Cloud
    /// Storage.
    #[prost(enumeration = "build_options::LogStreamingOption", tag = "5")]
    pub log_streaming_option: i32,
    /// Option to specify a `WorkerPool` for the build.
    /// Format: projects/{project}/locations/{location}/workerPools/{workerPool}
    ///
    /// This field is experimental.
    #[prost(string, tag = "7")]
    pub worker_pool: ::prost::alloc::string::String,
    /// Option to specify the logging mode, which determines if and where build
    /// logs are stored.
    #[prost(enumeration = "build_options::LoggingMode", tag = "11")]
    pub logging: i32,
    /// A list of global environment variable definitions that will exist for all
    /// build steps in this build. If a variable is defined in both globally and in
    /// a build step, the variable will use the build step value.
    ///
    /// The elements are of the form "KEY=VALUE" for the environment variable "KEY"
    /// being given the value "VALUE".
    #[prost(string, repeated, tag = "12")]
    pub env: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of global environment variables, which are encrypted using a Cloud
    /// Key Management Service crypto key. These values must be specified in the
    /// build's `Secret`. These variables will be available to all build steps
    /// in this build.
    #[prost(string, repeated, tag = "13")]
    pub secret_env: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Global list of volumes to mount for ALL build steps
    ///
    /// Each volume is created as an empty volume prior to starting the build
    /// process. Upon completion of the build, volumes and their contents are
    /// discarded. Global volume names and paths cannot conflict with the volumes
    /// defined a build step.
    ///
    /// Using a global volume in a build with only one step is not valid as
    /// it is indicative of a build request with an incorrect configuration.
    #[prost(message, repeated, tag = "14")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
}
/// Nested message and enum types in `BuildOptions`.
pub mod build_options {
    /// Specifies the manner in which the build should be verified, if at all.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VerifyOption {
        /// Not a verifiable build. (default)
        NotVerified = 0,
        /// Verified build.
        Verified = 1,
    }
    /// Supported Compute Engine machine types.
    /// For more information, see [Machine
    /// types](https://cloud.google.com/compute/docs/machine-types).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MachineType {
        /// Standard machine type.
        Unspecified = 0,
        /// Highcpu machine with 8 CPUs.
        N1Highcpu8 = 1,
        /// Highcpu machine with 32 CPUs.
        N1Highcpu32 = 2,
        /// Highcpu e2 machine with 8 CPUs.
        E2Highcpu8 = 5,
        /// Highcpu e2 machine with 32 CPUs.
        E2Highcpu32 = 6,
    }
    /// Specifies the behavior when there is an error in the substitution checks.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SubstitutionOption {
        /// Fails the build if error in substitutions checks, like missing
        /// a substitution in the template or in the map.
        MustMatch = 0,
        /// Do not fail the build if error in substitutions checks.
        AllowLoose = 1,
    }
    /// Specifies the behavior when writing build logs to Google Cloud Storage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LogStreamingOption {
        /// Service may automatically determine build log streaming behavior.
        StreamDefault = 0,
        /// Build logs should be streamed to Google Cloud Storage.
        StreamOn = 1,
        /// Build logs should not be streamed to Google Cloud Storage; they will be
        /// written when the build is completed.
        StreamOff = 2,
    }
    /// Specifies the logging mode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LoggingMode {
        /// The service determines the logging mode. The default is `LEGACY`. Do not
        /// rely on the default logging behavior as it may change in the future.
        LoggingUnspecified = 0,
        /// Cloud Logging and Cloud Storage logging are enabled.
        Legacy = 1,
        /// Only Cloud Storage logging is enabled.
        GcsOnly = 2,
        /// This option is the same as CLOUD_LOGGING_ONLY.
        StackdriverOnly = 3,
        /// Only Cloud Logging is enabled. Note that logs for both the Cloud Console
        /// UI and Cloud SDK are based on Cloud Storage logs, so neither will provide
        /// logs if this option is chosen.
        CloudLoggingOnly = 5,
        /// Turn off all logging. No build logs will be captured.
        None = 4,
    }
}
/// ReceiveTriggerWebhookRequest [Experimental] is the request object accepted by
/// the ReceiveTriggerWebhook method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveTriggerWebhookRequest {
    /// HTTP request body.
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<super::super::super::api::HttpBody>,
    /// Project in which the specified trigger lives
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Name of the trigger to run the payload against
    #[prost(string, tag = "3")]
    pub trigger: ::prost::alloc::string::String,
    /// Secret token used for authorization if an OAuth token isn't provided.
    #[prost(string, tag = "4")]
    pub secret: ::prost::alloc::string::String,
}
/// ReceiveTriggerWebhookResponse [Experimental] is the response object for the
/// ReceiveTriggerWebhook method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveTriggerWebhookResponse {}
/// Configuration for a WorkerPool to run the builds.
///
/// Workers are machines that Cloud Build uses to run your builds. By default,
/// all workers run in a project owned by Cloud Build. To have full control over
/// the workers that execute your builds -- such as enabling them to access
/// private resources on your private network -- you can request Cloud Build to
/// run the workers in your own project by creating a custom workers pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerPool {
    /// User-defined name of the `WorkerPool`.
    #[prost(string, tag = "14")]
    pub name: ::prost::alloc::string::String,
    /// The project ID of the GCP project for which the `WorkerPool` is created.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. The service account used to manage the `WorkerPool`. The
    /// service account must have the Compute Instance Admin (Beta) permission at
    /// the project level.
    #[prost(string, tag = "3")]
    pub service_account_email: ::prost::alloc::string::String,
    /// Total number of workers to be created across all requested regions.
    #[prost(int64, tag = "4")]
    pub worker_count: i64,
    /// Configuration to be used for a creating workers in the `WorkerPool`.
    #[prost(message, optional, tag = "16")]
    pub worker_config: ::core::option::Option<WorkerConfig>,
    /// List of regions to create the `WorkerPool`. Regions can't be empty.
    /// If Cloud Build adds a new GCP region in the future, the existing
    /// `WorkerPool` will not be enabled in the new region automatically;
    /// you must add the new region to the `regions` field to enable the
    /// `WorkerPool` in that region.
    #[prost(enumeration = "worker_pool::Region", repeated, tag = "9")]
    pub regions: ::prost::alloc::vec::Vec<i32>,
    /// Output only. Time at which the request to create the `WorkerPool` was
    /// received.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the request to update the `WorkerPool` was
    /// received.
    #[prost(message, optional, tag = "17")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the request to delete the `WorkerPool` was
    /// received.
    #[prost(message, optional, tag = "12")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. WorkerPool Status.
    #[prost(enumeration = "worker_pool::Status", tag = "13")]
    pub status: i32,
}
/// Nested message and enum types in `WorkerPool`.
pub mod worker_pool {
    /// Supported GCP regions to create the `WorkerPool`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Region {
        /// no region
        Unspecified = 0,
        /// us-central1 region
        UsCentral1 = 1,
        /// us-west1 region
        UsWest1 = 2,
        /// us-east1 region
        UsEast1 = 3,
        /// us-east4 region
        UsEast4 = 4,
    }
    /// `WorkerPool` status
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Status of the `WorkerPool` is unknown.
        Unspecified = 0,
        /// `WorkerPool` is being created.
        Creating = 1,
        /// `WorkerPool` is running.
        Running = 2,
        /// `WorkerPool` is being deleted: cancelling builds and draining workers.
        Deleting = 3,
        /// `WorkerPool` is deleted.
        Deleted = 4,
    }
}
/// WorkerConfig defines the configuration to be used for a creating workers in
/// the pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerConfig {
    /// Machine Type of the worker, such as n1-standard-1.
    /// See https://cloud.google.com/compute/docs/machine-types.
    /// If left blank, Cloud Build will use a standard unspecified machine to
    /// create the worker pool.
    /// `machine_type` is overridden if you specify a different machine type in
    /// `build_options`. In this case, the VM specified in the `build_options`
    /// will be created on demand at build time. For more information see
    /// https://cloud.google.com/cloud-build/docs/speeding-up-builds#using_custom_virtual_machine_sizes
    #[prost(string, tag = "1")]
    pub machine_type: ::prost::alloc::string::String,
    /// Size of the disk attached to the worker, in GB.
    /// See https://cloud.google.com/compute/docs/disks/
    /// If `0` is specified, Cloud Build will use a standard disk size.
    /// `disk_size` is overridden if you specify a different disk size in
    /// `build_options`. In this case, a VM with a disk size specified in the
    /// `build_options` will be created on demand at build time. For more
    /// information see
    /// https://cloud.google.com/cloud-build/docs/api/reference/rest/v1/projects.builds#buildoptions
    #[prost(int64, tag = "2")]
    pub disk_size_gb: i64,
    /// The network definition used to create the worker.
    /// If this section is left empty, the workers will be created in
    /// WorkerPool.project_id on the default network.
    #[prost(message, optional, tag = "3")]
    pub network: ::core::option::Option<Network>,
    /// The tag applied to the worker, and the same tag used by the firewall rule.
    /// It is used to identify the Cloud Build workers among other VMs.
    /// The default value for tag is `worker`.
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
}
/// Network describes the GCP network used to create workers in.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// Project id containing the defined network and subnetwork. For a peered VPC,
    /// this will be the same as the project_id in which the workers are created.
    /// For a shared VPC, this will be the project sharing the network with the
    /// project_id project in which workers will be created. For custom workers
    /// with no VPC, this will be the same as project_id.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Network on which the workers are created.
    /// "default" network is used if empty.
    #[prost(string, tag = "2")]
    pub network: ::prost::alloc::string::String,
    /// Subnetwork on which the workers are created.
    /// "default" subnetwork is used if empty.
    #[prost(string, tag = "3")]
    pub subnetwork: ::prost::alloc::string::String,
}
/// Request to create a new `WorkerPool`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkerPoolRequest {
    /// ID of the parent project.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// `WorkerPool` resource to create.
    #[prost(message, optional, tag = "2")]
    pub worker_pool: ::core::option::Option<WorkerPool>,
}
/// Request to get a `WorkerPool` with the specified name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkerPoolRequest {
    /// The field will contain name of the resource requested, for example:
    /// "projects/project-1/workerPools/workerpool-name"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to delete a `WorkerPool`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkerPoolRequest {
    /// The field will contain name of the resource requested, for example:
    /// "projects/project-1/workerPools/workerpool-name"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to update a `WorkerPool`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkerPoolRequest {
    /// The field will contain name of the resource requested, for example:
    /// "projects/project-1/workerPools/workerpool-name"
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// `WorkerPool` resource to update.
    #[prost(message, optional, tag = "3")]
    pub worker_pool: ::core::option::Option<WorkerPool>,
}
/// Request to list `WorkerPools`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkerPoolsRequest {
    /// ID of the parent project.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response containing existing `WorkerPools`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkerPoolsResponse {
    /// `WorkerPools` for the project.
    #[prost(message, repeated, tag = "1")]
    pub worker_pools: ::prost::alloc::vec::Vec<WorkerPool>,
}
#[doc = r" Generated client implementations."]
pub mod cloud_build_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Creates and manages builds on Google Cloud Platform."]
    #[doc = ""]
    #[doc = " The main concept used by this API is a `Build`, which describes the location"]
    #[doc = " of the source to build, how to build the source, and where to store the"]
    #[doc = " built artifacts, if any."]
    #[doc = ""]
    #[doc = " A user can list previously-requested builds or get builds by their ID to"]
    #[doc = " determine the status of the build."]
    pub struct CloudBuildClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudBuildClient<T>
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
        #[doc = " Starts a build with the specified configuration."]
        #[doc = ""]
        #[doc = " This method returns a long-running `Operation`, which includes the build"]
        #[doc = " ID. Pass the build ID to `GetBuild` to determine the build status (such as"]
        #[doc = " `SUCCESS` or `FAILURE`)."]
        pub async fn create_build(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBuildRequest>,
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
                "/google.devtools.cloudbuild.v1.CloudBuild/CreateBuild",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information about a previously requested build."]
        #[doc = ""]
        #[doc = " The `Build` that is returned includes its status (such as `SUCCESS`,"]
        #[doc = " `FAILURE`, or `WORKING`), and timing information."]
        pub async fn get_build(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBuildRequest>,
        ) -> Result<tonic::Response<super::Build>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/GetBuild",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists previously requested builds."]
        #[doc = ""]
        #[doc = " Previously requested builds may still be in-progress, or may have finished"]
        #[doc = " successfully or unsuccessfully."]
        pub async fn list_builds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBuildsRequest>,
        ) -> Result<tonic::Response<super::ListBuildsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/ListBuilds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels a build in progress."]
        pub async fn cancel_build(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelBuildRequest>,
        ) -> Result<tonic::Response<super::Build>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/CancelBuild",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new build based on the specified build."]
        #[doc = ""]
        #[doc = " This method creates a new build using the original build request, which may"]
        #[doc = " or may not result in an identical build."]
        #[doc = ""]
        #[doc = " For triggered builds:"]
        #[doc = ""]
        #[doc = " * Triggered builds resolve to a precise revision; therefore a retry of a"]
        #[doc = " triggered build will result in a build that uses the same revision."]
        #[doc = ""]
        #[doc = " For non-triggered builds that specify `RepoSource`:"]
        #[doc = ""]
        #[doc = " * If the original build built from the tip of a branch, the retried build"]
        #[doc = " will build from the tip of that branch, which may not be the same revision"]
        #[doc = " as the original build."]
        #[doc = " * If the original build specified a commit sha or revision ID, the retried"]
        #[doc = " build will use the identical source."]
        #[doc = ""]
        #[doc = " For builds that specify `StorageSource`:"]
        #[doc = ""]
        #[doc = " * If the original build pulled source from Google Cloud Storage without"]
        #[doc = " specifying the generation of the object, the new build will use the current"]
        #[doc = " object, which may be different from the original build source."]
        #[doc = " * If the original build pulled source from Cloud Storage and specified the"]
        #[doc = " generation of the object, the new build will attempt to use the same"]
        #[doc = " object, which may or may not be available depending on the bucket's"]
        #[doc = " lifecycle management settings."]
        pub async fn retry_build(
            &mut self,
            request: impl tonic::IntoRequest<super::RetryBuildRequest>,
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
                "/google.devtools.cloudbuild.v1.CloudBuild/RetryBuild",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new `BuildTrigger`."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        pub async fn create_build_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBuildTriggerRequest>,
        ) -> Result<tonic::Response<super::BuildTrigger>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/CreateBuildTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information about a `BuildTrigger`."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        pub async fn get_build_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBuildTriggerRequest>,
        ) -> Result<tonic::Response<super::BuildTrigger>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/GetBuildTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists existing `BuildTrigger`s."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        pub async fn list_build_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBuildTriggersRequest>,
        ) -> Result<tonic::Response<super::ListBuildTriggersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/ListBuildTriggers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a `BuildTrigger` by its project ID and trigger ID."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        pub async fn delete_build_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBuildTriggerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/DeleteBuildTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a `BuildTrigger` by its project ID and trigger ID."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        pub async fn update_build_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBuildTriggerRequest>,
        ) -> Result<tonic::Response<super::BuildTrigger>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/UpdateBuildTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Runs a `BuildTrigger` at a particular source revision."]
        pub async fn run_build_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::RunBuildTriggerRequest>,
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
                "/google.devtools.cloudbuild.v1.CloudBuild/RunBuildTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ReceiveTriggerWebhook [Experimental] is called when the API receives a"]
        #[doc = " webhook request targeted at a specific trigger."]
        pub async fn receive_trigger_webhook(
            &mut self,
            request: impl tonic::IntoRequest<super::ReceiveTriggerWebhookRequest>,
        ) -> Result<tonic::Response<super::ReceiveTriggerWebhookResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/ReceiveTriggerWebhook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a `WorkerPool` to run the builds, and returns the new worker pool."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        pub async fn create_worker_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkerPoolRequest>,
        ) -> Result<tonic::Response<super::WorkerPool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/CreateWorkerPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information about a `WorkerPool`."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        pub async fn get_worker_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkerPoolRequest>,
        ) -> Result<tonic::Response<super::WorkerPool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/GetWorkerPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a `WorkerPool` by its project ID and WorkerPool name."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        pub async fn delete_worker_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkerPoolRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/DeleteWorkerPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a `WorkerPool`."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        pub async fn update_worker_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkerPoolRequest>,
        ) -> Result<tonic::Response<super::WorkerPool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/UpdateWorkerPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List project's `WorkerPools`."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        pub async fn list_worker_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkerPoolsRequest>,
        ) -> Result<tonic::Response<super::ListWorkerPoolsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudbuild.v1.CloudBuild/ListWorkerPools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CloudBuildClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CloudBuildClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CloudBuildClient {{ ... }}")
        }
    }
}
