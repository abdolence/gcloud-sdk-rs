/// Specifies a build to retry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryBuildRequest {
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Build ID of the original build.
    #[prost(string, tag = "2")]
    pub id: std::string::String,
}
/// Specifies a build trigger to run and the source to use.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunBuildTriggerRequest {
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. ID of the trigger.
    #[prost(string, tag = "2")]
    pub trigger_id: std::string::String,
    /// Required. Source to build against this trigger.
    #[prost(message, optional, tag = "3")]
    pub source: ::std::option::Option<RepoSource>,
}
/// Location of the source in an archive file in Google Cloud Storage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageSource {
    /// Google Cloud Storage bucket containing the source (see
    /// [Bucket Name
    /// Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
    #[prost(string, tag = "1")]
    pub bucket: std::string::String,
    /// Google Cloud Storage object containing the source.
    ///
    /// This object must be a gzipped archive file (`.tar.gz`) containing source to
    /// build.
    #[prost(string, tag = "2")]
    pub object: std::string::String,
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
    pub project_id: std::string::String,
    /// Required. Name of the Cloud Source Repository.
    #[prost(string, tag = "2")]
    pub repo_name: std::string::String,
    /// Directory, relative to the source root, in which to run the build.
    ///
    /// This must be a relative path. If a step's `dir` is specified and is an
    /// absolute path, this value is ignored for that step's execution.
    #[prost(string, tag = "7")]
    pub dir: std::string::String,
    /// Only trigger a build if the revision regex does NOT match the revision
    /// regex.
    #[prost(bool, tag = "8")]
    pub invert_regex: bool,
    /// Substitutions to use in a triggered build.
    /// Should only be used with RunBuildTrigger
    #[prost(map = "string, string", tag = "9")]
    pub substitutions: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// A revision within the Cloud Source Repository must be specified in
    /// one of these ways.
    #[prost(oneof = "repo_source::Revision", tags = "3, 4, 5")]
    pub revision: ::std::option::Option<repo_source::Revision>,
}
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
        BranchName(std::string::String),
        /// Regex matching tags to build.
        ///
        /// The syntax of the regular expressions accepted is the syntax accepted by
        /// RE2 and described at https://github.com/google/re2/wiki/Syntax
        #[prost(string, tag = "4")]
        TagName(std::string::String),
        /// Explicit commit SHA to build.
        #[prost(string, tag = "5")]
        CommitSha(std::string::String),
    }
}
/// Location of the source in a supported storage service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// Location of source.
    #[prost(oneof = "source::Source", tags = "2, 3")]
    pub source: ::std::option::Option<source::Source>,
}
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
    pub name: std::string::String,
    /// Docker Registry 2.0 digest.
    #[prost(string, tag = "3")]
    pub digest: std::string::String,
    /// Output only. Stores timing information for pushing the specified image.
    #[prost(message, optional, tag = "4")]
    pub push_timing: ::std::option::Option<TimeSpan>,
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
    pub name: std::string::String,
    /// A list of environment variable definitions to be used when running a step.
    ///
    /// The elements are of the form "KEY=VALUE" for the environment variable "KEY"
    /// being given the value "VALUE".
    #[prost(string, repeated, tag = "2")]
    pub env: ::std::vec::Vec<std::string::String>,
    /// A list of arguments that will be presented to the step when it is started.
    ///
    /// If the image used to run the step's container has an entrypoint, the `args`
    /// are used as arguments to that entrypoint. If the image does not define
    /// an entrypoint, the first element in args is used as the entrypoint,
    /// and the remainder will be used as arguments.
    #[prost(string, repeated, tag = "3")]
    pub args: ::std::vec::Vec<std::string::String>,
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
    pub dir: std::string::String,
    /// Unique identifier for this build step, used in `wait_for` to
    /// reference this build step as a dependency.
    #[prost(string, tag = "5")]
    pub id: std::string::String,
    /// The ID(s) of the step(s) that this build step depends on.
    /// This build step will not start until all the build steps in `wait_for`
    /// have completed successfully. If `wait_for` is empty, this build step will
    /// start when all previous build steps in the `Build.Steps` list have
    /// completed successfully.
    #[prost(string, repeated, tag = "6")]
    pub wait_for: ::std::vec::Vec<std::string::String>,
    /// Entrypoint to be used instead of the build step image's default entrypoint.
    /// If unset, the image's default entrypoint is used.
    #[prost(string, tag = "7")]
    pub entrypoint: std::string::String,
    /// A list of environment variables which are encrypted using a Cloud Key
    /// Management Service crypto key. These values must be specified in the
    /// build's `Secret`.
    #[prost(string, repeated, tag = "8")]
    pub secret_env: ::std::vec::Vec<std::string::String>,
    /// List of volumes to mount into the build step.
    ///
    /// Each volume is created as an empty volume prior to execution of the
    /// build step. Upon completion of the build, volumes and their contents are
    /// discarded.
    ///
    /// Using a named volume in only one step is not valid as it is indicative
    /// of a build request with an incorrect configuration.
    #[prost(message, repeated, tag = "9")]
    pub volumes: ::std::vec::Vec<Volume>,
    /// Output only. Stores timing information for executing this build step.
    #[prost(message, optional, tag = "10")]
    pub timing: ::std::option::Option<TimeSpan>,
    /// Output only. Stores timing information for pulling this build step's
    /// builder image only.
    #[prost(message, optional, tag = "13")]
    pub pull_timing: ::std::option::Option<TimeSpan>,
    /// Time limit for executing this build step. If not defined, the step has no
    /// time limit and will be allowed to continue to run until either it completes
    /// or the build itself times out.
    #[prost(message, optional, tag = "11")]
    pub timeout: ::std::option::Option<::prost_types::Duration>,
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
    pub name: std::string::String,
    /// Path at which to mount the volume.
    ///
    /// Paths must be absolute and cannot conflict with other volume paths on the
    /// same build step or with certain reserved volume paths.
    #[prost(string, tag = "2")]
    pub path: std::string::String,
}
/// Artifacts created by the build pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Results {
    /// Container images that were built as a part of the build.
    #[prost(message, repeated, tag = "2")]
    pub images: ::std::vec::Vec<BuiltImage>,
    /// List of build step digests, in the order corresponding to build step
    /// indices.
    #[prost(string, repeated, tag = "3")]
    pub build_step_images: ::std::vec::Vec<std::string::String>,
    /// Path to the artifact manifest. Only populated when artifacts are uploaded.
    #[prost(string, tag = "4")]
    pub artifact_manifest: std::string::String,
    /// Number of artifacts uploaded. Only populated when artifacts are uploaded.
    #[prost(int64, tag = "5")]
    pub num_artifacts: i64,
    /// List of build step outputs, produced by builder images, in the order
    /// corresponding to build step indices.
    ///
    /// [Cloud Builders](https://cloud.google.com/cloud-build/docs/cloud-builders)
    /// can produce this output by writing to `$BUILDER_OUTPUT/output`.
    /// Only the first 4KB of data is stored.
    #[prost(bytes, repeated, tag = "6")]
    pub build_step_outputs: ::std::vec::Vec<std::vec::Vec<u8>>,
    /// Time to push all non-container artifacts.
    #[prost(message, optional, tag = "7")]
    pub artifact_timing: ::std::option::Option<TimeSpan>,
}
/// An artifact that was uploaded during a build. This
/// is a single record in the artifact manifest JSON file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArtifactResult {
    /// The path of an artifact in a Google Cloud Storage bucket, with the
    /// generation number. For example,
    /// `gs://mybucket/path/to/output.jar#generation`.
    #[prost(string, tag = "1")]
    pub location: std::string::String,
    /// The file hash of the artifact.
    #[prost(message, repeated, tag = "2")]
    pub file_hash: ::std::vec::Vec<FileHashes>,
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
/// - $BUILD_ID: the autogenerated ID of the build.
/// - $REPO_NAME: the source repository name specified by RepoSource.
/// - $BRANCH_NAME: the branch name specified by RepoSource.
/// - $TAG_NAME: the tag name specified by RepoSource.
/// - $REVISION_ID or $COMMIT_SHA: the commit SHA specified by RepoSource or
///   resolved from the specified branch or tag.
/// - $SHORT_SHA: first 7 characters of $REVISION_ID or $COMMIT_SHA.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Build {
    /// Output only. Unique identifier of the build.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// Output only. ID of the project.
    #[prost(string, tag = "16")]
    pub project_id: std::string::String,
    /// Output only. Status of the build.
    #[prost(enumeration = "build::Status", tag = "2")]
    pub status: i32,
    /// Output only. Customer-readable message about the current status.
    #[prost(string, tag = "24")]
    pub status_detail: std::string::String,
    /// The location of the source files to build.
    #[prost(message, optional, tag = "3")]
    pub source: ::std::option::Option<Source>,
    /// Required. The operations to be performed on the workspace.
    #[prost(message, repeated, tag = "11")]
    pub steps: ::std::vec::Vec<BuildStep>,
    /// Output only. Results of the build.
    #[prost(message, optional, tag = "10")]
    pub results: ::std::option::Option<Results>,
    /// Output only. Time at which the request to create the build was received.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which execution of the build was started.
    #[prost(message, optional, tag = "7")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which execution of the build was finished.
    ///
    /// The difference between finish_time and start_time is the duration of the
    /// build's execution.
    #[prost(message, optional, tag = "8")]
    pub finish_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Amount of time that this build should be allowed to run, to second
    /// granularity. If this amount of time elapses, work on the build will cease
    /// and the build status will be `TIMEOUT`.
    ///
    /// Default time is ten minutes.
    #[prost(message, optional, tag = "12")]
    pub timeout: ::std::option::Option<::prost_types::Duration>,
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
    pub images: ::std::vec::Vec<std::string::String>,
    /// TTL in queue for this build. If provided and the build is enqueued longer
    /// than this value, the build will expire and the build status will be
    /// `EXPIRED`.
    ///
    /// The TTL starts ticking from create_time.
    #[prost(message, optional, tag = "40")]
    pub queue_ttl: ::std::option::Option<::prost_types::Duration>,
    /// Artifacts produced by the build that should be uploaded upon
    /// successful completion of all build steps.
    #[prost(message, optional, tag = "37")]
    pub artifacts: ::std::option::Option<Artifacts>,
    /// Google Cloud Storage bucket where logs should be written (see
    /// [Bucket Name
    /// Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
    /// Logs file names will be of the format `${logs_bucket}/log-${build_id}.txt`.
    #[prost(string, tag = "19")]
    pub logs_bucket: std::string::String,
    /// Output only. A permanent fixed identifier for source.
    #[prost(message, optional, tag = "21")]
    pub source_provenance: ::std::option::Option<SourceProvenance>,
    /// Output only. The ID of the `BuildTrigger` that triggered this build, if it
    /// was triggered automatically.
    #[prost(string, tag = "22")]
    pub build_trigger_id: std::string::String,
    /// Special options for this build.
    #[prost(message, optional, tag = "23")]
    pub options: ::std::option::Option<BuildOptions>,
    /// Output only. URL to logs for this build in Google Cloud Console.
    #[prost(string, tag = "25")]
    pub log_url: std::string::String,
    /// Substitutions data for `Build` resource.
    #[prost(map = "string, string", tag = "29")]
    pub substitutions: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Tags for annotation of a `Build`. These are not docker tags.
    #[prost(string, repeated, tag = "31")]
    pub tags: ::std::vec::Vec<std::string::String>,
    /// Secrets to decrypt using Cloud Key Management Service.
    #[prost(message, repeated, tag = "32")]
    pub secrets: ::std::vec::Vec<Secret>,
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
    pub timing: ::std::collections::HashMap<std::string::String, TimeSpan>,
}
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
    pub images: ::std::vec::Vec<std::string::String>,
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
    pub objects: ::std::option::Option<artifacts::ArtifactObjects>,
}
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
        pub location: std::string::String,
        /// Path globs used to match files in the build's workspace.
        #[prost(string, repeated, tag = "2")]
        pub paths: ::std::vec::Vec<std::string::String>,
        /// Output only. Stores timing information for pushing all artifact objects.
        #[prost(message, optional, tag = "3")]
        pub timing: ::std::option::Option<super::TimeSpan>,
    }
}
/// Start and end times for a build execution phase.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSpan {
    /// Start of time span.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// End of time span.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Metadata for build operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildOperationMetadata {
    /// The build that the operation is tracking.
    #[prost(message, optional, tag = "1")]
    pub build: ::std::option::Option<Build>,
}
/// Provenance of the source. Ways to find the original source, or verify that
/// some source was used for this build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceProvenance {
    /// A copy of the build's `source.storage_source`, if exists, with any
    /// generations resolved.
    #[prost(message, optional, tag = "3")]
    pub resolved_storage_source: ::std::option::Option<StorageSource>,
    /// A copy of the build's `source.repo_source`, if exists, with any
    /// revisions resolved.
    #[prost(message, optional, tag = "6")]
    pub resolved_repo_source: ::std::option::Option<RepoSource>,
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
    pub file_hashes: ::std::collections::HashMap<std::string::String, FileHashes>,
}
/// Container message for hashes of byte content of files, used in
/// SourceProvenance messages to verify integrity of source input to the build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHashes {
    /// Collection of file hashes.
    #[prost(message, repeated, tag = "1")]
    pub file_hash: ::std::vec::Vec<Hash>,
}
/// Container message for hash values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    /// The type of hash that was performed.
    #[prost(enumeration = "hash::HashType", tag = "1")]
    pub r#type: i32,
    /// The hash value.
    #[prost(bytes, tag = "2")]
    pub value: std::vec::Vec<u8>,
}
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
/// Pairs a set of secret environment variables containing encrypted
/// values with the Cloud KMS key to use to decrypt the value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    /// Cloud KMS key name to use to decrypt these envs.
    #[prost(string, tag = "1")]
    pub kms_key_name: std::string::String,
    /// Map of environment variable name to its encrypted value.
    ///
    /// Secret environment variables must be unique across all of a build's
    /// secrets, and must be used by at least one build step. Values can be at most
    /// 64 KB in size. There can be at most 100 secret values across all of a
    /// build's secrets.
    #[prost(map = "string, bytes", tag = "3")]
    pub secret_env: ::std::collections::HashMap<std::string::String, std::vec::Vec<u8>>,
}
/// Request to create a new build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBuildRequest {
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Build resource to create.
    #[prost(message, optional, tag = "2")]
    pub build: ::std::option::Option<Build>,
}
/// Request to get a build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuildRequest {
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. ID of the build.
    #[prost(string, tag = "2")]
    pub id: std::string::String,
}
/// Request to list builds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildsRequest {
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Number of results to return in the list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// The raw filter text to constrain the results.
    #[prost(string, tag = "8")]
    pub filter: std::string::String,
}
/// Response including listed builds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildsResponse {
    /// Builds will be sorted by `create_time`, descending.
    #[prost(message, repeated, tag = "1")]
    pub builds: ::std::vec::Vec<Build>,
    /// Token to receive the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request to cancel an ongoing build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelBuildRequest {
    /// Required. ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. ID of the build.
    #[prost(string, tag = "2")]
    pub id: std::string::String,
}
/// Configuration for an automated build in response to source repository
/// changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildTrigger {
    /// Output only. Unique identifier of the trigger.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// Human-readable description of this trigger.
    #[prost(string, tag = "10")]
    pub description: std::string::String,
    /// User-assigned name of the trigger. Must be unique within the project.
    /// Trigger names must meet the following requirements:
    ///
    /// + They must contain only alphanumeric characters and dashes.
    /// + They can be 1-64 characters long.
    /// + They must begin and end with an alphanumeric character.
    #[prost(string, tag = "21")]
    pub name: std::string::String,
    /// Tags for annotation of a `BuildTrigger`
    #[prost(string, repeated, tag = "19")]
    pub tags: ::std::vec::Vec<std::string::String>,
    /// Template describing the types of source changes to trigger a build.
    ///
    /// Branch and tag names in trigger templates are interpreted as regular
    /// expressions. Any branch or tag change that matches that regular expression
    /// will trigger a build.
    ///
    /// Mutually exclusive with `github`.
    #[prost(message, optional, tag = "7")]
    pub trigger_template: ::std::option::Option<RepoSource>,
    /// GitHubEventsConfig describes the configuration of a trigger that creates
    /// a build whenever a GitHub event is received.
    ///
    /// Mutually exclusive with `trigger_template`.
    #[prost(message, optional, tag = "13")]
    pub github: ::std::option::Option<GitHubEventsConfig>,
    /// Output only. Time when the trigger was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// If true, the trigger will never result in a build.
    #[prost(bool, tag = "9")]
    pub disabled: bool,
    /// Substitutions for Build resource. The keys must match the following
    /// regular expression: `^_[A-Z0-9_]+$`.The keys cannot conflict with the
    /// keys in bindings.
    #[prost(map = "string, string", tag = "11")]
    pub substitutions: ::std::collections::HashMap<std::string::String, std::string::String>,
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
    pub ignored_files: ::std::vec::Vec<std::string::String>,
    /// If any of the files altered in the commit pass the ignored_files
    /// filter and included_files is empty, then as far as this filter is
    /// concerned, we should trigger the build.
    ///
    /// If any of the files altered in the commit pass the ignored_files
    /// filter and included_files is not empty, then we make sure that at
    /// least one of those files matches a included_files glob. If not,
    /// then we do not trigger a build.
    #[prost(string, repeated, tag = "16")]
    pub included_files: ::std::vec::Vec<std::string::String>,
    /// Template describing the Build request to make when the trigger is matched.
    #[prost(oneof = "build_trigger::BuildTemplate", tags = "4, 8")]
    pub build_template: ::std::option::Option<build_trigger::BuildTemplate>,
}
pub mod build_trigger {
    /// Template describing the Build request to make when the trigger is matched.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BuildTemplate {
        /// Contents of the build template.
        #[prost(message, tag = "4")]
        Build(super::Build),
        /// Path, from the source root, to a file whose contents is used for the
        /// template.
        #[prost(string, tag = "8")]
        Filename(std::string::String),
    }
}
/// GitHubEventsConfig describes the configuration of a trigger that creates a
/// build whenever a GitHub event is received.
///
/// This message is experimental.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitHubEventsConfig {
    /// The installationID that emits the GitHub event.
    #[prost(int64, tag = "1")]
    pub installation_id: i64,
    /// Owner of the repository. For example: The owner for
    /// https://github.com/googlecloudplatform/cloud-builders is
    /// "googlecloudplatform".
    #[prost(string, tag = "6")]
    pub owner: std::string::String,
    /// Name of the repository. For example: The name for
    /// https://github.com/googlecloudplatform/cloud-builders is "cloud-builders".
    #[prost(string, tag = "7")]
    pub name: std::string::String,
    /// Filter describing the types of events to trigger a build.
    /// Currently supported event types: push, pull_request.
    #[prost(oneof = "git_hub_events_config::Event", tags = "4, 5")]
    pub event: ::std::option::Option<git_hub_events_config::Event>,
}
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
    /// Whether to block builds on a "/gcbrun" comment from a repository admin or
    /// collaborator.
    #[prost(enumeration = "pull_request_filter::CommentControl", tag = "5")]
    pub comment_control: i32,
    /// If true, branches that do NOT match the git_ref will trigger a build.
    #[prost(bool, tag = "6")]
    pub invert_regex: bool,
    /// Target refs to match.
    /// A target ref is the git reference where the pull request will be applied.
    #[prost(oneof = "pull_request_filter::GitRef", tags = "2")]
    pub git_ref: ::std::option::Option<pull_request_filter::GitRef>,
}
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
        Branch(std::string::String),
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
    pub git_ref: ::std::option::Option<push_filter::GitRef>,
}
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
        Branch(std::string::String),
        /// Regexes matching tags to build.
        ///
        /// The syntax of the regular expressions accepted is the syntax accepted by
        /// RE2 and described at https://github.com/google/re2/wiki/Syntax
        #[prost(string, tag = "3")]
        Tag(std::string::String),
    }
}
/// Request to create a new `BuildTrigger`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBuildTriggerRequest {
    /// Required. ID of the project for which to configure automatic builds.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. `BuildTrigger` to create.
    #[prost(message, optional, tag = "2")]
    pub trigger: ::std::option::Option<BuildTrigger>,
}
/// Returns the `BuildTrigger` with the specified ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBuildTriggerRequest {
    /// Required. ID of the project that owns the trigger.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Identifier (`id` or `name`) of the `BuildTrigger` to get.
    #[prost(string, tag = "2")]
    pub trigger_id: std::string::String,
}
/// Request to list existing `BuildTriggers`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildTriggersRequest {
    /// Required. ID of the project for which to list BuildTriggers.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Number of results to return in the list.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response containing existing `BuildTriggers`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildTriggersResponse {
    /// `BuildTriggers` for the project, sorted by `create_time` descending.
    #[prost(message, repeated, tag = "1")]
    pub triggers: ::std::vec::Vec<BuildTrigger>,
    /// Token to receive the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request to delete a `BuildTrigger`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBuildTriggerRequest {
    /// Required. ID of the project that owns the trigger.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. ID of the `BuildTrigger` to delete.
    #[prost(string, tag = "2")]
    pub trigger_id: std::string::String,
}
/// Request to update an existing `BuildTrigger`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBuildTriggerRequest {
    /// Required. ID of the project that owns the trigger.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. ID of the `BuildTrigger` to update.
    #[prost(string, tag = "2")]
    pub trigger_id: std::string::String,
    /// Required. `BuildTrigger` to update.
    #[prost(message, optional, tag = "3")]
    pub trigger: ::std::option::Option<BuildTrigger>,
}
/// Optional arguments to enable specific features of builds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildOptions {
    /// Requested hash for SourceProvenance.
    #[prost(enumeration = "hash::HashType", repeated, tag = "1")]
    pub source_provenance_hash: ::std::vec::Vec<i32>,
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
    #[prost(enumeration = "build_options::SubstitutionOption", tag = "4")]
    pub substitution_option: i32,
    /// Option to define build log streaming behavior to Google Cloud
    /// Storage.
    #[prost(enumeration = "build_options::LogStreamingOption", tag = "5")]
    pub log_streaming_option: i32,
    /// Option to specify a `WorkerPool` for the build.
    /// Format: projects/{project}/workerPools/{workerPool}
    ///
    /// This field is experimental.
    #[prost(string, tag = "7")]
    pub worker_pool: std::string::String,
    /// Option to specify the logging mode, which determines where the logs are
    /// stored.
    #[prost(enumeration = "build_options::LoggingMode", tag = "11")]
    pub logging: i32,
    /// A list of global environment variable definitions that will exist for all
    /// build steps in this build. If a variable is defined in both globally and in
    /// a build step, the variable will use the build step value.
    ///
    /// The elements are of the form "KEY=VALUE" for the environment variable "KEY"
    /// being given the value "VALUE".
    #[prost(string, repeated, tag = "12")]
    pub env: ::std::vec::Vec<std::string::String>,
    /// A list of global environment variables, which are encrypted using a Cloud
    /// Key Management Service crypto key. These values must be specified in the
    /// build's `Secret`. These variables will be available to all build steps
    /// in this build.
    #[prost(string, repeated, tag = "13")]
    pub secret_env: ::std::vec::Vec<std::string::String>,
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
    pub volumes: ::std::vec::Vec<Volume>,
}
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
    /// Supported VM sizes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MachineType {
        /// Standard machine type.
        Unspecified = 0,
        /// Highcpu machine with 8 CPUs.
        N1Highcpu8 = 1,
        /// Highcpu machine with 32 CPUs.
        N1Highcpu32 = 2,
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
        /// Stackdriver logging and Cloud Storage logging are enabled.
        Legacy = 1,
        /// Only Cloud Storage logging is enabled.
        GcsOnly = 2,
    }
}
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
    pub name: std::string::String,
    /// The project ID of the GCP project for which the `WorkerPool` is created.
    #[prost(string, tag = "2")]
    pub project_id: std::string::String,
    /// Output only. The service account used to manage the `WorkerPool`. The
    /// service account must have the Compute Instance Admin (Beta) permission at
    /// the project level.
    #[prost(string, tag = "3")]
    pub service_account_email: std::string::String,
    /// Total number of workers to be created across all requested regions.
    #[prost(int64, tag = "4")]
    pub worker_count: i64,
    /// Configuration to be used for a creating workers in the `WorkerPool`.
    #[prost(message, optional, tag = "16")]
    pub worker_config: ::std::option::Option<WorkerConfig>,
    /// List of regions to create the `WorkerPool`. Regions can't be empty.
    /// If Cloud Build adds a new GCP region in the future, the existing
    /// `WorkerPool` will not be enabled in the new region automatically;
    /// you must add the new region to the `regions` field to enable the
    /// `WorkerPool` in that region.
    #[prost(enumeration = "worker_pool::Region", repeated, tag = "9")]
    pub regions: ::std::vec::Vec<i32>,
    /// Output only. Time at which the request to create the `WorkerPool` was
    /// received.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the request to update the `WorkerPool` was
    /// received.
    #[prost(message, optional, tag = "17")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the request to delete the `WorkerPool` was
    /// received.
    #[prost(message, optional, tag = "12")]
    pub delete_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. WorkerPool Status.
    #[prost(enumeration = "worker_pool::Status", tag = "13")]
    pub status: i32,
}
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
    pub machine_type: std::string::String,
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
    pub network: ::std::option::Option<Network>,
    /// The tag applied to the worker, and the same tag used by the firewall rule.
    /// It is used to identify the Cloud Build workers among other VMs.
    /// The default value for tag is `worker`.
    #[prost(string, tag = "4")]
    pub tag: std::string::String,
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
    pub project_id: std::string::String,
    /// Network on which the workers are created.
    /// "default" network is used if empty.
    #[prost(string, tag = "2")]
    pub network: std::string::String,
    /// Subnetwork on which the workers are created.
    /// "default" subnetwork is used if empty.
    #[prost(string, tag = "3")]
    pub subnetwork: std::string::String,
}
/// Request to create a new `WorkerPool`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkerPoolRequest {
    /// ID of the parent project.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// `WorkerPool` resource to create.
    #[prost(message, optional, tag = "2")]
    pub worker_pool: ::std::option::Option<WorkerPool>,
}
/// Request to get a `WorkerPool` with the specified name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkerPoolRequest {
    /// The field will contain name of the resource requested, for example:
    /// "projects/project-1/workerPools/workerpool-name"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request to delete a `WorkerPool`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkerPoolRequest {
    /// The field will contain name of the resource requested, for example:
    /// "projects/project-1/workerPools/workerpool-name"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request to update a `WorkerPool`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkerPoolRequest {
    /// The field will contain name of the resource requested, for example:
    /// "projects/project-1/workerPools/workerpool-name"
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    /// `WorkerPool` resource to update.
    #[prost(message, optional, tag = "3")]
    pub worker_pool: ::std::option::Option<WorkerPool>,
}
/// Request to list `WorkerPool`s.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkerPoolsRequest {
    /// ID of the parent project.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
}
/// Response containing existing `WorkerPools`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkerPoolsResponse {
    /// `WorkerPools` for the project.
    #[prost(message, repeated, tag = "1")]
    pub worker_pools: ::std::vec::Vec<WorkerPool>,
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
        #[doc = " List project's `WorkerPool`s."]
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
#[doc = r" Generated server implementations."]
pub mod cloud_build_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CloudBuildServer."]
    #[async_trait]
    pub trait CloudBuild: Send + Sync + 'static {
        #[doc = " Starts a build with the specified configuration."]
        #[doc = ""]
        #[doc = " This method returns a long-running `Operation`, which includes the build"]
        #[doc = " ID. Pass the build ID to `GetBuild` to determine the build status (such as"]
        #[doc = " `SUCCESS` or `FAILURE`)."]
        async fn create_build(
            &self,
            request: tonic::Request<super::CreateBuildRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Returns information about a previously requested build."]
        #[doc = ""]
        #[doc = " The `Build` that is returned includes its status (such as `SUCCESS`,"]
        #[doc = " `FAILURE`, or `WORKING`), and timing information."]
        async fn get_build(
            &self,
            request: tonic::Request<super::GetBuildRequest>,
        ) -> Result<tonic::Response<super::Build>, tonic::Status>;
        #[doc = " Lists previously requested builds."]
        #[doc = ""]
        #[doc = " Previously requested builds may still be in-progress, or may have finished"]
        #[doc = " successfully or unsuccessfully."]
        async fn list_builds(
            &self,
            request: tonic::Request<super::ListBuildsRequest>,
        ) -> Result<tonic::Response<super::ListBuildsResponse>, tonic::Status>;
        #[doc = " Cancels a build in progress."]
        async fn cancel_build(
            &self,
            request: tonic::Request<super::CancelBuildRequest>,
        ) -> Result<tonic::Response<super::Build>, tonic::Status>;
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
        async fn retry_build(
            &self,
            request: tonic::Request<super::RetryBuildRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Creates a new `BuildTrigger`."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        async fn create_build_trigger(
            &self,
            request: tonic::Request<super::CreateBuildTriggerRequest>,
        ) -> Result<tonic::Response<super::BuildTrigger>, tonic::Status>;
        #[doc = " Returns information about a `BuildTrigger`."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        async fn get_build_trigger(
            &self,
            request: tonic::Request<super::GetBuildTriggerRequest>,
        ) -> Result<tonic::Response<super::BuildTrigger>, tonic::Status>;
        #[doc = " Lists existing `BuildTrigger`s."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        async fn list_build_triggers(
            &self,
            request: tonic::Request<super::ListBuildTriggersRequest>,
        ) -> Result<tonic::Response<super::ListBuildTriggersResponse>, tonic::Status>;
        #[doc = " Deletes a `BuildTrigger` by its project ID and trigger ID."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        async fn delete_build_trigger(
            &self,
            request: tonic::Request<super::DeleteBuildTriggerRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates a `BuildTrigger` by its project ID and trigger ID."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        async fn update_build_trigger(
            &self,
            request: tonic::Request<super::UpdateBuildTriggerRequest>,
        ) -> Result<tonic::Response<super::BuildTrigger>, tonic::Status>;
        #[doc = " Runs a `BuildTrigger` at a particular source revision."]
        async fn run_build_trigger(
            &self,
            request: tonic::Request<super::RunBuildTriggerRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Creates a `WorkerPool` to run the builds, and returns the new worker pool."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        async fn create_worker_pool(
            &self,
            request: tonic::Request<super::CreateWorkerPoolRequest>,
        ) -> Result<tonic::Response<super::WorkerPool>, tonic::Status>;
        #[doc = " Returns information about a `WorkerPool`."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        async fn get_worker_pool(
            &self,
            request: tonic::Request<super::GetWorkerPoolRequest>,
        ) -> Result<tonic::Response<super::WorkerPool>, tonic::Status>;
        #[doc = " Deletes a `WorkerPool` by its project ID and WorkerPool name."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        async fn delete_worker_pool(
            &self,
            request: tonic::Request<super::DeleteWorkerPoolRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Update a `WorkerPool`."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        async fn update_worker_pool(
            &self,
            request: tonic::Request<super::UpdateWorkerPoolRequest>,
        ) -> Result<tonic::Response<super::WorkerPool>, tonic::Status>;
        #[doc = " List project's `WorkerPool`s."]
        #[doc = ""]
        #[doc = " This API is experimental."]
        async fn list_worker_pools(
            &self,
            request: tonic::Request<super::ListWorkerPoolsRequest>,
        ) -> Result<tonic::Response<super::ListWorkerPoolsResponse>, tonic::Status>;
    }
    #[doc = " Creates and manages builds on Google Cloud Platform."]
    #[doc = ""]
    #[doc = " The main concept used by this API is a `Build`, which describes the location"]
    #[doc = " of the source to build, how to build the source, and where to store the"]
    #[doc = " built artifacts, if any."]
    #[doc = ""]
    #[doc = " A user can list previously-requested builds or get builds by their ID to"]
    #[doc = " determine the status of the build."]
    #[derive(Debug)]
    pub struct CloudBuildServer<T: CloudBuild> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CloudBuild> CloudBuildServer<T> {
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
    impl<T, B> Service<http::Request<B>> for CloudBuildServer<T>
    where
        T: CloudBuild,
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
                "/google.devtools.cloudbuild.v1.CloudBuild/CreateBuild" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBuildSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::CreateBuildRequest> for CreateBuildSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBuildRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_build(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateBuildSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/GetBuild" => {
                    #[allow(non_camel_case_types)]
                    struct GetBuildSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::GetBuildRequest> for GetBuildSvc<T> {
                        type Response = super::Build;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBuildRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_build(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBuildSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/ListBuilds" => {
                    #[allow(non_camel_case_types)]
                    struct ListBuildsSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::ListBuildsRequest> for ListBuildsSvc<T> {
                        type Response = super::ListBuildsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBuildsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_builds(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListBuildsSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/CancelBuild" => {
                    #[allow(non_camel_case_types)]
                    struct CancelBuildSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::CancelBuildRequest> for CancelBuildSvc<T> {
                        type Response = super::Build;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelBuildRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel_build(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CancelBuildSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/RetryBuild" => {
                    #[allow(non_camel_case_types)]
                    struct RetryBuildSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::RetryBuildRequest> for RetryBuildSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RetryBuildRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).retry_build(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RetryBuildSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/CreateBuildTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBuildTriggerSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild>
                        tonic::server::UnaryService<super::CreateBuildTriggerRequest>
                        for CreateBuildTriggerSvc<T>
                    {
                        type Response = super::BuildTrigger;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBuildTriggerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_build_trigger(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateBuildTriggerSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/GetBuildTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct GetBuildTriggerSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::GetBuildTriggerRequest>
                        for GetBuildTriggerSvc<T>
                    {
                        type Response = super::BuildTrigger;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBuildTriggerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_build_trigger(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBuildTriggerSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/ListBuildTriggers" => {
                    #[allow(non_camel_case_types)]
                    struct ListBuildTriggersSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::ListBuildTriggersRequest>
                        for ListBuildTriggersSvc<T>
                    {
                        type Response = super::ListBuildTriggersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBuildTriggersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_build_triggers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListBuildTriggersSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/DeleteBuildTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBuildTriggerSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild>
                        tonic::server::UnaryService<super::DeleteBuildTriggerRequest>
                        for DeleteBuildTriggerSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBuildTriggerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_build_trigger(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteBuildTriggerSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/UpdateBuildTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBuildTriggerSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild>
                        tonic::server::UnaryService<super::UpdateBuildTriggerRequest>
                        for UpdateBuildTriggerSvc<T>
                    {
                        type Response = super::BuildTrigger;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateBuildTriggerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_build_trigger(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateBuildTriggerSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/RunBuildTrigger" => {
                    #[allow(non_camel_case_types)]
                    struct RunBuildTriggerSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::RunBuildTriggerRequest>
                        for RunBuildTriggerSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RunBuildTriggerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).run_build_trigger(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RunBuildTriggerSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/CreateWorkerPool" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWorkerPoolSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::CreateWorkerPoolRequest>
                        for CreateWorkerPoolSvc<T>
                    {
                        type Response = super::WorkerPool;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWorkerPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_worker_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateWorkerPoolSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/GetWorkerPool" => {
                    #[allow(non_camel_case_types)]
                    struct GetWorkerPoolSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::GetWorkerPoolRequest>
                        for GetWorkerPoolSvc<T>
                    {
                        type Response = super::WorkerPool;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWorkerPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_worker_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetWorkerPoolSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/DeleteWorkerPool" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteWorkerPoolSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::DeleteWorkerPoolRequest>
                        for DeleteWorkerPoolSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteWorkerPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_worker_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteWorkerPoolSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/UpdateWorkerPool" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateWorkerPoolSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::UpdateWorkerPoolRequest>
                        for UpdateWorkerPoolSvc<T>
                    {
                        type Response = super::WorkerPool;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateWorkerPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_worker_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateWorkerPoolSvc(inner);
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
                "/google.devtools.cloudbuild.v1.CloudBuild/ListWorkerPools" => {
                    #[allow(non_camel_case_types)]
                    struct ListWorkerPoolsSvc<T: CloudBuild>(pub Arc<T>);
                    impl<T: CloudBuild> tonic::server::UnaryService<super::ListWorkerPoolsRequest>
                        for ListWorkerPoolsSvc<T>
                    {
                        type Response = super::ListWorkerPoolsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWorkerPoolsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_worker_pools(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListWorkerPoolsSvc(inner);
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
    impl<T: CloudBuild> Clone for CloudBuildServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CloudBuild> Clone for _Inner<T> {
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
