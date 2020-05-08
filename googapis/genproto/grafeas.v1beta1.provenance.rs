/// Provenance of a build. Contains all information needed to verify the full
/// details about the build from source to completion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildProvenance {
    /// Required. Unique identifier of the build.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// ID of the project.
    #[prost(string, tag = "2")]
    pub project_id: std::string::String,
    /// Commands requested by the build.
    #[prost(message, repeated, tag = "3")]
    pub commands: ::std::vec::Vec<Command>,
    /// Output of the build.
    #[prost(message, repeated, tag = "4")]
    pub built_artifacts: ::std::vec::Vec<Artifact>,
    /// Time at which the build was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Time at which execution of the build was started.
    #[prost(message, optional, tag = "6")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Time at which execution of the build was finished.
    #[prost(message, optional, tag = "7")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// E-mail address of the user who initiated this build. Note that this was the
    /// user's e-mail address at the time the build was initiated; this address may
    /// not represent the same end-user for all time.
    #[prost(string, tag = "8")]
    pub creator: std::string::String,
    /// URI where any logs for this provenance were written.
    #[prost(string, tag = "9")]
    pub logs_uri: std::string::String,
    /// Details of the Source input to the build.
    #[prost(message, optional, tag = "10")]
    pub source_provenance: ::std::option::Option<Source>,
    /// Trigger identifier if the build was triggered automatically; empty if not.
    #[prost(string, tag = "11")]
    pub trigger_id: std::string::String,
    /// Special options applied to this build. This is a catch-all field where
    /// build providers can enter any desired additional details.
    #[prost(map = "string, string", tag = "12")]
    pub build_options: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Version string of the builder at the time this build was executed.
    #[prost(string, tag = "13")]
    pub builder_version: std::string::String,
}
/// Source describes the location of the source used for the build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// If provided, the input binary artifacts for the build came from this
    /// location.
    #[prost(string, tag = "1")]
    pub artifact_storage_source_uri: std::string::String,
    /// Hash(es) of the build source, which can be used to verify that the original
    /// source integrity was maintained in the build.
    ///
    /// The keys to this map are file paths used as build source and the values
    /// contain the hash values for those files.
    ///
    /// If the build source came in a single package such as a gzipped tarfile
    /// (.tar.gz), the FileHash will be for the single path to that file.
    #[prost(map = "string, message", tag = "2")]
    pub file_hashes: ::std::collections::HashMap<std::string::String, FileHashes>,
    /// If provided, the source code used for the build came from this location.
    #[prost(message, optional, tag = "3")]
    pub context: ::std::option::Option<super::source::SourceContext>,
    /// If provided, some of the source code used for the build may be found in
    /// these locations, in the case where the source repository had multiple
    /// remotes or submodules. This list will not include the context specified in
    /// the context field.
    #[prost(message, repeated, tag = "4")]
    pub additional_contexts: ::std::vec::Vec<super::source::SourceContext>,
}
/// Container message for hashes of byte content of files, used in source
/// messages to verify integrity of source input to the build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileHashes {
    /// Required. Collection of file hashes.
    #[prost(message, repeated, tag = "1")]
    pub file_hash: ::std::vec::Vec<Hash>,
}
/// Container message for hash values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hash {
    /// Required. The type of hash that was performed.
    #[prost(enumeration = "hash::HashType", tag = "1")]
    pub r#type: i32,
    /// Required. The hash value.
    #[prost(bytes, tag = "2")]
    pub value: std::vec::Vec<u8>,
}
pub mod hash {
    /// Specifies the hash algorithm.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HashType {
        /// Unknown.
        Unspecified = 0,
        /// A SHA-256 hash.
        Sha256 = 1,
    }
}
/// Command describes a step performed as part of the build pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    /// Required. Name of the command, as presented on the command line, or if the
    /// command is packaged as a Docker container, as presented to `docker pull`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Environment variables set before running this command.
    #[prost(string, repeated, tag = "2")]
    pub env: ::std::vec::Vec<std::string::String>,
    /// Command-line arguments used when executing this command.
    #[prost(string, repeated, tag = "3")]
    pub args: ::std::vec::Vec<std::string::String>,
    /// Working directory (relative to project source root) used when running this
    /// command.
    #[prost(string, tag = "4")]
    pub dir: std::string::String,
    /// Optional unique identifier for this command, used in wait_for to reference
    /// this command as a dependency.
    #[prost(string, tag = "5")]
    pub id: std::string::String,
    /// The ID(s) of the command(s) that this command depends on.
    #[prost(string, repeated, tag = "6")]
    pub wait_for: ::std::vec::Vec<std::string::String>,
}
/// Artifact describes a build product.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Artifact {
    /// Hash or checksum value of a binary, or Docker Registry 2.0 digest of a
    /// container.
    #[prost(string, tag = "1")]
    pub checksum: std::string::String,
    /// Artifact ID, if any; for container images, this will be a URL by digest
    /// like `gcr.io/projectID/imagename@sha256:123456`.
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// Related artifact names. This may be the path to a binary or jar file, or in
    /// the case of a container build, the name used to push the container image to
    /// Google Container Registry, as presented to `docker push`. Note that a
    /// single Artifact ID can have multiple names, for example if two tags are
    /// applied to one image.
    #[prost(string, repeated, tag = "3")]
    pub names: ::std::vec::Vec<std::string::String>,
}
