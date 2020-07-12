/// Metadata for any related URL information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedUrl {
    /// Specific URL associated with the resource.
    #[prost(string, tag = "1")]
    pub url: std::string::String,
    /// Label to describe usage of the URL.
    #[prost(string, tag = "2")]
    pub label: std::string::String,
}
/// Verifiers (e.g. Kritis implementations) MUST verify signatures
/// with respect to the trust anchors defined in policy (e.g. a Kritis policy).
/// Typically this means that the verifier has been configured with a map from
/// `public_key_id` to public key material (and any required parameters, e.g.
/// signing algorithm).
///
/// In particular, verification implementations MUST NOT treat the signature
/// `public_key_id` as anything more than a key lookup hint. The `public_key_id`
/// DOES NOT validate or authenticate a public key; it only provides a mechanism
/// for quickly selecting a public key ALREADY CONFIGURED on the verifier through
/// a trusted channel. Verification implementations MUST reject signatures in any
/// of the following circumstances:
///   * The `public_key_id` is not recognized by the verifier.
///   * The public key that `public_key_id` refers to does not verify the
///     signature with respect to the payload.
///
/// The `signature` contents SHOULD NOT be "attached" (where the payload is
/// included with the serialized `signature` bytes). Verifiers MUST ignore any
/// "attached" payload and only verify signatures with respect to explicitly
/// provided payload (e.g. a `payload` field on the proto message that holds
/// this Signature, or the canonical serialization of the proto message that
/// holds this signature).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    /// The content of the signature, an opaque bytestring.
    /// The payload that this signature verifies MUST be unambiguously provided
    /// with the Signature during verification. A wrapper message might provide
    /// the payload explicitly. Alternatively, a message might have a canonical
    /// serialization that can always be unambiguously computed to derive the
    /// payload.
    #[prost(bytes, tag = "1")]
    pub signature: std::vec::Vec<u8>,
    /// The identifier for the public key that verifies this signature.
    ///   * The `public_key_id` is required.
    ///   * The `public_key_id` MUST be an RFC3986 conformant URI.
    ///   * When possible, the `public_key_id` SHOULD be an immutable reference,
    ///     such as a cryptographic digest.
    ///
    /// Examples of valid `public_key_id`s:
    ///
    /// OpenPGP V4 public key fingerprint:
    ///   * "openpgp4fpr:74FAF3B861BDA0870C7B6DEF607E48D2A663AEEA"
    /// See https://www.iana.org/assignments/uri-schemes/prov/openpgp4fpr for more
    /// details on this scheme.
    ///
    /// RFC6920 digest-named SubjectPublicKeyInfo (digest of the DER
    /// serialization):
    ///   * "ni:///sha-256;cD9o9Cq6LG3jD0iKXqEi_vdjJGecm_iXkbqVoScViaU"
    ///   * "nih:///sha-256;703f68f42aba2c6de30f488a5ea122fef76324679c9bf89791ba95a1271589a5"
    #[prost(string, tag = "2")]
    pub public_key_id: std::string::String,
}
/// Kind represents the kinds of notes supported.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NoteKind {
    /// Unknown.
    Unspecified = 0,
    /// The note and occurrence represent a package vulnerability.
    Vulnerability = 1,
    /// The note and occurrence assert build provenance.
    Build = 2,
    /// This represents an image basis relationship.
    Image = 3,
    /// This represents a package installed via a package manager.
    Package = 4,
    /// The note and occurrence track deployment events.
    Deployment = 5,
    /// The note and occurrence track the initial discovery status of a resource.
    Discovery = 6,
    /// This represents a logical "role" that can attest to artifacts.
    Attestation = 7,
    /// This represents an available package upgrade.
    Upgrade = 8,
}
// An attestation wrapper with a PGP-compatible signature. This message only
// supports `ATTACHED` signatures, where the payload that is signed is included
// alongside the signature itself in the same file.

/// Note kind that represents a logical attestation "role" or "authority". For
/// example, an organization might have one `Authority` for "QA" and one for
/// "build". This note is intended to act strictly as a grouping mechanism for
/// the attached occurrences (Attestations). This grouping mechanism also
/// provides a security boundary, since IAM ACLs gate the ability for a principle
/// to attach an occurrence to a given note. It also provides a single point of
/// lookup to find all attached attestation occurrences, even if they don't all
/// live in the same project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestationNote {
    /// Hint hints at the purpose of the attestation authority.
    #[prost(message, optional, tag = "1")]
    pub hint: ::std::option::Option<attestation_note::Hint>,
}
pub mod attestation_note {
    /// This submessage provides human-readable hints about the purpose of the
    /// authority. Because the name of a note acts as its resource reference, it is
    /// important to disambiguate the canonical name of the Note (which might be a
    /// UUID for security purposes) from "readable" names more suitable for debug
    /// output. Note that these hints should not be used to look up authorities in
    /// security sensitive contexts, such as when looking up attestations to
    /// verify.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Hint {
        /// Required. The human readable name of this attestation authority, for
        /// example "qa".
        #[prost(string, tag = "1")]
        pub human_readable_name: std::string::String,
    }
}
/// Occurrence that represents a single "attestation". The authenticity of an
/// attestation can be verified using the attached signature. If the verifier
/// trusts the public key of the signer, then verifying the signature is
/// sufficient to establish trust. In this circumstance, the authority to which
/// this attestation is attached is primarily useful for lookup (how to find
/// this attestation if you already know the authority and artifact to be
/// verified) and intent (for which authority this attestation was intended to
/// sign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestationOccurrence {
    /// Required. The serialized payload that is verified by one or more
    /// `signatures`.
    #[prost(bytes, tag = "1")]
    pub serialized_payload: std::vec::Vec<u8>,
    /// One or more signatures over `serialized_payload`.  Verifier implementations
    /// should consider this attestation message verified if at least one
    /// `signature` verifies `serialized_payload`.  See `Signature` in common.proto
    /// for more details on signature structure and verification.
    #[prost(message, repeated, tag = "2")]
    pub signatures: ::std::vec::Vec<Signature>,
}
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
    pub context: ::std::option::Option<SourceContext>,
    /// If provided, some of the source code used for the build may be found in
    /// these locations, in the case where the source repository had multiple
    /// remotes or submodules. This list will not include the context specified in
    /// the context field.
    #[prost(message, repeated, tag = "4")]
    pub additional_contexts: ::std::vec::Vec<SourceContext>,
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
    /// Required. The type of hash that was performed, e.g. "SHA-256".
    #[prost(string, tag = "1")]
    pub r#type: std::string::String,
    /// Required. The hash value.
    #[prost(bytes, tag = "2")]
    pub value: std::vec::Vec<u8>,
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
/// A SourceContext is a reference to a tree of files. A SourceContext together
/// with a path point to a unique revision of a single file or directory.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceContext {
    /// Labels with user defined metadata.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// A SourceContext can refer any one of the following types of repositories.
    #[prost(oneof = "source_context::Context", tags = "1, 2, 3")]
    pub context: ::std::option::Option<source_context::Context>,
}
pub mod source_context {
    /// A SourceContext can refer any one of the following types of repositories.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Context {
        /// A SourceContext referring to a revision in a Google Cloud Source Repo.
        #[prost(message, tag = "1")]
        CloudRepo(super::CloudRepoSourceContext),
        /// A SourceContext referring to a Gerrit project.
        #[prost(message, tag = "2")]
        Gerrit(super::GerritSourceContext),
        /// A SourceContext referring to any third party Git repo (e.g., GitHub).
        #[prost(message, tag = "3")]
        Git(super::GitSourceContext),
    }
}
/// An alias to a repo revision.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AliasContext {
    /// The alias kind.
    #[prost(enumeration = "alias_context::Kind", tag = "1")]
    pub kind: i32,
    /// The alias name.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
pub mod alias_context {
    /// The type of an alias.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        /// Unknown.
        Unspecified = 0,
        /// Git tag.
        Fixed = 1,
        /// Git branch.
        Movable = 2,
        /// Used to specify non-standard aliases. For example, if a Git repo has a
        /// ref named "refs/foo/bar".
        Other = 4,
    }
}
/// A CloudRepoSourceContext denotes a particular revision in a Google Cloud
/// Source Repo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRepoSourceContext {
    /// The ID of the repo.
    #[prost(message, optional, tag = "1")]
    pub repo_id: ::std::option::Option<RepoId>,
    /// A revision in a Cloud Repo can be identified by either its revision ID or
    /// its alias.
    #[prost(oneof = "cloud_repo_source_context::Revision", tags = "2, 3")]
    pub revision: ::std::option::Option<cloud_repo_source_context::Revision>,
}
pub mod cloud_repo_source_context {
    /// A revision in a Cloud Repo can be identified by either its revision ID or
    /// its alias.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Revision {
        /// A revision ID.
        #[prost(string, tag = "2")]
        RevisionId(std::string::String),
        /// An alias, which may be a branch or tag.
        #[prost(message, tag = "3")]
        AliasContext(super::AliasContext),
    }
}
/// A SourceContext referring to a Gerrit project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GerritSourceContext {
    /// The URI of a running Gerrit instance.
    #[prost(string, tag = "1")]
    pub host_uri: std::string::String,
    /// The full project name within the host. Projects may be nested, so
    /// "project/subproject" is a valid project name. The "repo name" is the
    /// hostURI/project.
    #[prost(string, tag = "2")]
    pub gerrit_project: std::string::String,
    /// A revision in a Gerrit project can be identified by either its revision ID
    /// or its alias.
    #[prost(oneof = "gerrit_source_context::Revision", tags = "3, 4")]
    pub revision: ::std::option::Option<gerrit_source_context::Revision>,
}
pub mod gerrit_source_context {
    /// A revision in a Gerrit project can be identified by either its revision ID
    /// or its alias.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Revision {
        /// A revision (commit) ID.
        #[prost(string, tag = "3")]
        RevisionId(std::string::String),
        /// An alias, which may be a branch or tag.
        #[prost(message, tag = "4")]
        AliasContext(super::AliasContext),
    }
}
/// A GitSourceContext denotes a particular revision in a third party Git
/// repository (e.g., GitHub).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitSourceContext {
    /// Git repository URL.
    #[prost(string, tag = "1")]
    pub url: std::string::String,
    /// Git commit hash.
    #[prost(string, tag = "2")]
    pub revision_id: std::string::String,
}
/// A unique identifier for a Cloud Repo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepoId {
    /// A cloud repo can be identified by either its project ID and repository name
    /// combination, or its globally unique identifier.
    #[prost(oneof = "repo_id::Id", tags = "1, 2")]
    pub id: ::std::option::Option<repo_id::Id>,
}
pub mod repo_id {
    /// A cloud repo can be identified by either its project ID and repository name
    /// combination, or its globally unique identifier.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        /// A combination of a project ID and a repo name.
        #[prost(message, tag = "1")]
        ProjectRepoId(super::ProjectRepoId),
        /// A server-assigned, globally unique identifier.
        #[prost(string, tag = "2")]
        Uid(std::string::String),
    }
}
/// Selects a repo using a Google Cloud Platform project ID (e.g.,
/// winged-cargo-31) and a repo name within that project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectRepoId {
    /// The ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// The name of the repo. Leave empty for the default repo.
    #[prost(string, tag = "2")]
    pub repo_name: std::string::String,
}
/// Note holding the version of the provider's builder and the signature of the
/// provenance message in the build details occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildNote {
    /// Required. Immutable. Version of the builder which produced this build.
    #[prost(string, tag = "1")]
    pub builder_version: std::string::String,
}
/// Details of a build occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildOccurrence {
    /// Required. The actual provenance for the build.
    #[prost(message, optional, tag = "1")]
    pub provenance: ::std::option::Option<BuildProvenance>,
    /// Serialized JSON representation of the provenance, used in generating the
    /// build signature in the corresponding build note. After verifying the
    /// signature, `provenance_bytes` can be unmarshalled and compared to the
    /// provenance to confirm that it is unchanged. A base64-encoded string
    /// representation of the provenance bytes is used for the signature in order
    /// to interoperate with openssl which expects this format for signature
    /// verification.
    ///
    /// The serialized form is captured both to avoid ambiguity in how the
    /// provenance is marshalled to json as well to prevent incompatibilities with
    /// future changes.
    #[prost(string, tag = "2")]
    pub provenance_bytes: std::string::String,
}
/// Common Vulnerability Scoring System version 3.
/// For details, see https://www.first.org/cvss/specification-document
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CvsSv3 {
    /// The base score is a function of the base metric scores.
    #[prost(float, tag = "1")]
    pub base_score: f32,
    #[prost(float, tag = "2")]
    pub exploitability_score: f32,
    #[prost(float, tag = "3")]
    pub impact_score: f32,
    /// Base Metrics
    /// Represents the intrinsic characteristics of a vulnerability that are
    /// constant over time and across user environments.
    #[prost(enumeration = "cvs_sv3::AttackVector", tag = "5")]
    pub attack_vector: i32,
    #[prost(enumeration = "cvs_sv3::AttackComplexity", tag = "6")]
    pub attack_complexity: i32,
    #[prost(enumeration = "cvs_sv3::PrivilegesRequired", tag = "7")]
    pub privileges_required: i32,
    #[prost(enumeration = "cvs_sv3::UserInteraction", tag = "8")]
    pub user_interaction: i32,
    #[prost(enumeration = "cvs_sv3::Scope", tag = "9")]
    pub scope: i32,
    #[prost(enumeration = "cvs_sv3::Impact", tag = "10")]
    pub confidentiality_impact: i32,
    #[prost(enumeration = "cvs_sv3::Impact", tag = "11")]
    pub integrity_impact: i32,
    #[prost(enumeration = "cvs_sv3::Impact", tag = "12")]
    pub availability_impact: i32,
}
pub mod cvs_sv3 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AttackVector {
        Unspecified = 0,
        Network = 1,
        Adjacent = 2,
        Local = 3,
        Physical = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AttackComplexity {
        Unspecified = 0,
        Low = 1,
        High = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PrivilegesRequired {
        Unspecified = 0,
        None = 1,
        Low = 2,
        High = 3,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserInteraction {
        Unspecified = 0,
        None = 1,
        Required = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Scope {
        Unspecified = 0,
        Unchanged = 1,
        Changed = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Impact {
        Unspecified = 0,
        High = 1,
        Low = 2,
        None = 3,
    }
}
/// An artifact that can be deployed in some runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentNote {
    /// Required. Resource URI for the artifact being deployed.
    #[prost(string, repeated, tag = "1")]
    pub resource_uri: ::std::vec::Vec<std::string::String>,
}
/// The period during which some deployable was active in a runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentOccurrence {
    /// Identity of the user that triggered this deployment.
    #[prost(string, tag = "1")]
    pub user_email: std::string::String,
    /// Required. Beginning of the lifetime of this deployment.
    #[prost(message, optional, tag = "2")]
    pub deploy_time: ::std::option::Option<::prost_types::Timestamp>,
    /// End of the lifetime of this deployment.
    #[prost(message, optional, tag = "3")]
    pub undeploy_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Configuration used to create this deployment.
    #[prost(string, tag = "4")]
    pub config: std::string::String,
    /// Address of the runtime element hosting this deployment.
    #[prost(string, tag = "5")]
    pub address: std::string::String,
    /// Output only. Resource URI for the artifact being deployed taken from
    /// the deployable field with the same name.
    #[prost(string, repeated, tag = "6")]
    pub resource_uri: ::std::vec::Vec<std::string::String>,
    /// Platform hosting this deployment.
    #[prost(enumeration = "deployment_occurrence::Platform", tag = "7")]
    pub platform: i32,
}
pub mod deployment_occurrence {
    /// Types of platforms.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Platform {
        /// Unknown.
        Unspecified = 0,
        /// Google Container Engine.
        Gke = 1,
        /// Google App Engine: Flexible Environment.
        Flex = 2,
        /// Custom user-defined platform.
        Custom = 3,
    }
}
/// A note that indicates a type of analysis a provider would perform. This note
/// exists in a provider's project. A `Discovery` occurrence is created in a
/// consumer's project at the start of analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryNote {
    /// Required. Immutable. The kind of analysis that is handled by this
    /// discovery.
    #[prost(enumeration = "NoteKind", tag = "1")]
    pub analysis_kind: i32,
}
/// Provides information about the analysis status of a discovered resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryOccurrence {
    /// Whether the resource is continuously analyzed.
    #[prost(enumeration = "discovery_occurrence::ContinuousAnalysis", tag = "1")]
    pub continuous_analysis: i32,
    /// The status of discovery for the resource.
    #[prost(enumeration = "discovery_occurrence::AnalysisStatus", tag = "2")]
    pub analysis_status: i32,
    /// When an error is encountered this will contain a LocalizedMessage under
    /// details to show to the user. The LocalizedMessage is output only and
    /// populated by the API.
    #[prost(message, optional, tag = "3")]
    pub analysis_status_error: ::std::option::Option<super::super::google::rpc::Status>,
    /// The CPE of the resource being scanned.
    #[prost(string, tag = "4")]
    pub cpe: std::string::String,
    /// The last time this resource was scanned.
    #[prost(message, optional, tag = "5")]
    pub last_scan_time: ::std::option::Option<::prost_types::Timestamp>,
}
pub mod discovery_occurrence {
    /// Whether the resource is continuously analyzed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ContinuousAnalysis {
        /// Unknown.
        Unspecified = 0,
        /// The resource is continuously analyzed.
        Active = 1,
        /// The resource is ignored for continuous analysis.
        Inactive = 2,
    }
    /// Analysis status for a resource. Currently for initial analysis only (not
    /// updated in continuous analysis).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AnalysisStatus {
        /// Unknown.
        Unspecified = 0,
        /// Resource is known but no action has been taken yet.
        Pending = 1,
        /// Resource is being analyzed.
        Scanning = 2,
        /// Analysis has finished successfully.
        FinishedSuccess = 3,
        /// Analysis has finished unsuccessfully, the analysis itself is in a bad
        /// state.
        FinishedFailed = 4,
        /// The resource is known not to be supported
        FinishedUnsupported = 5,
    }
}
/// Layer holds metadata specific to a layer of a Docker image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Layer {
    /// Required. The recovered Dockerfile directive used to construct this layer.
    /// See https://docs.docker.com/engine/reference/builder/ for more information.
    #[prost(string, tag = "1")]
    pub directive: std::string::String,
    /// The recovered arguments to the Dockerfile directive.
    #[prost(string, tag = "2")]
    pub arguments: std::string::String,
}
/// A set of properties that uniquely identify a given Docker image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fingerprint {
    /// Required. The layer ID of the final layer in the Docker image's v1
    /// representation.
    #[prost(string, tag = "1")]
    pub v1_name: std::string::String,
    /// Required. The ordered list of v2 blobs that represent a given image.
    #[prost(string, repeated, tag = "2")]
    pub v2_blob: ::std::vec::Vec<std::string::String>,
    /// Output only. The name of the image's v2 blobs computed via:
    ///   [bottom] := v2_blob[bottom]
    ///   [N] := sha256(v2_blob[N] + " " + v2_name[N+1])
    /// Only the name of the final blob is kept.
    #[prost(string, tag = "3")]
    pub v2_name: std::string::String,
}
/// Basis describes the base image portion (Note) of the DockerImage
/// relationship. Linked occurrences are derived from this or an equivalent image
/// via:
///   FROM <Basis.resource_url>
/// Or an equivalent reference, e.g., a tag of the resource_url.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageNote {
    /// Required. Immutable. The resource_url for the resource representing the
    /// basis of associated occurrence images.
    #[prost(string, tag = "1")]
    pub resource_url: std::string::String,
    /// Required. Immutable. The fingerprint of the base image.
    #[prost(message, optional, tag = "2")]
    pub fingerprint: ::std::option::Option<Fingerprint>,
}
/// Details of the derived image portion of the DockerImage relationship. This
/// image would be produced from a Dockerfile with FROM <DockerImage.Basis in
/// attached Note>.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageOccurrence {
    /// Required. The fingerprint of the derived image.
    #[prost(message, optional, tag = "1")]
    pub fingerprint: ::std::option::Option<Fingerprint>,
    /// Output only. The number of layers by which this image differs from the
    /// associated image basis.
    #[prost(int32, tag = "2")]
    pub distance: i32,
    /// This contains layer-specific metadata, if populated it has length
    /// "distance" and is ordered with [distance] being the layer immediately
    /// following the base image and [1] being the final layer.
    #[prost(message, repeated, tag = "3")]
    pub layer_info: ::std::vec::Vec<Layer>,
    /// Output only. This contains the base image URL for the derived image
    /// occurrence.
    #[prost(string, tag = "4")]
    pub base_resource_url: std::string::String,
}
/// This represents a particular channel of distribution for a given package.
/// E.g., Debian's jessie-backports dpkg mirror.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Distribution {
    /// Required. The cpe_uri in [CPE format](https://cpe.mitre.org/specification/)
    /// denoting the package manager version distributing a package.
    #[prost(string, tag = "1")]
    pub cpe_uri: std::string::String,
    /// The CPU architecture for which packages in this distribution channel were
    /// built.
    #[prost(enumeration = "Architecture", tag = "2")]
    pub architecture: i32,
    /// The latest available version of this package in this distribution channel.
    #[prost(message, optional, tag = "3")]
    pub latest_version: ::std::option::Option<Version>,
    /// A freeform string denoting the maintainer of this package.
    #[prost(string, tag = "4")]
    pub maintainer: std::string::String,
    /// The distribution channel-specific homepage for this package.
    #[prost(string, tag = "5")]
    pub url: std::string::String,
    /// The distribution channel-specific description of this package.
    #[prost(string, tag = "6")]
    pub description: std::string::String,
}
/// An occurrence of a particular package installation found within a system's
/// filesystem. E.g., glibc was found in `/var/lib/dpkg/status`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Required. The CPE URI in [CPE format](https://cpe.mitre.org/specification/)
    /// denoting the package manager version distributing a package.
    #[prost(string, tag = "1")]
    pub cpe_uri: std::string::String,
    /// The version installed at this location.
    #[prost(message, optional, tag = "2")]
    pub version: ::std::option::Option<Version>,
    /// The path from which we gathered that this package/version is installed.
    #[prost(string, tag = "3")]
    pub path: std::string::String,
}
/// This represents a particular package that is distributed over various
/// channels. E.g., glibc (aka libc6) is distributed by many, at various
/// versions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageNote {
    /// Required. Immutable. The name of the package.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The various channels by which a package is distributed.
    #[prost(message, repeated, tag = "10")]
    pub distribution: ::std::vec::Vec<Distribution>,
}
/// Details on how a particular software package was installed on a system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageOccurrence {
    /// Output only. The name of the installed package.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. All of the places within the filesystem versions of this package
    /// have been found.
    #[prost(message, repeated, tag = "2")]
    pub location: ::std::vec::Vec<Location>,
}
/// Version contains structured information about the version of a package.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Used to correct mistakes in the version numbering scheme.
    #[prost(int32, tag = "1")]
    pub epoch: i32,
    /// Required only when version kind is NORMAL. The main part of the version
    /// name.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    /// The iteration of the package build from the above version.
    #[prost(string, tag = "3")]
    pub revision: std::string::String,
    /// Required. Distinguishes between sentinel MIN/MAX versions and normal
    /// versions.
    #[prost(enumeration = "version::VersionKind", tag = "4")]
    pub kind: i32,
    /// Human readable version string. This string is of the form
    /// <epoch>:<name>-<revision> and is only set when kind is NORMAL.
    #[prost(string, tag = "5")]
    pub full_name: std::string::String,
}
pub mod version {
    /// Whether this is an ordinary package version or a sentinel MIN/MAX version.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VersionKind {
        /// Unknown.
        Unspecified = 0,
        /// A standard package version.
        Normal = 1,
        /// A special version representing negative infinity.
        Minimum = 2,
        /// A special version representing positive infinity.
        Maximum = 3,
    }
}
/// Instruction set architectures supported by various package managers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Architecture {
    /// Unknown architecture.
    Unspecified = 0,
    /// X86 architecture.
    X86 = 1,
    /// X64 architecture.
    X64 = 2,
}
/// An Upgrade Note represents a potential upgrade of a package to a given
/// version. For each package version combination (i.e. bash 4.0, bash 4.1,
/// bash 4.1.2), there will be an Upgrade Note. For Windows, windows_update field
/// represents the information related to the update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeNote {
    /// Required for non-Windows OS. The package this Upgrade is for.
    #[prost(string, tag = "1")]
    pub package: std::string::String,
    /// Required for non-Windows OS. The version of the package in machine + human
    /// readable form.
    #[prost(message, optional, tag = "2")]
    pub version: ::std::option::Option<Version>,
    /// Metadata about the upgrade for each specific operating system.
    #[prost(message, repeated, tag = "3")]
    pub distributions: ::std::vec::Vec<UpgradeDistribution>,
    /// Required for Windows OS. Represents the metadata about the Windows update.
    #[prost(message, optional, tag = "4")]
    pub windows_update: ::std::option::Option<WindowsUpdate>,
}
/// The Upgrade Distribution represents metadata about the Upgrade for each
/// operating system (CPE). Some distributions have additional metadata around
/// updates, classifying them into various categories and severities.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeDistribution {
    /// Required - The specific operating system this metadata applies to. See
    /// https://cpe.mitre.org/specification/.
    #[prost(string, tag = "1")]
    pub cpe_uri: std::string::String,
    /// The operating system classification of this Upgrade, as specified by the
    /// upstream operating system upgrade feed. For Windows the classification is
    /// one of the category_ids listed at
    /// https://docs.microsoft.com/en-us/previous-versions/windows/desktop/ff357803(v=vs.85)
    #[prost(string, tag = "2")]
    pub classification: std::string::String,
    /// The severity as specified by the upstream operating system.
    #[prost(string, tag = "3")]
    pub severity: std::string::String,
    /// The cve tied to this Upgrade.
    #[prost(string, repeated, tag = "4")]
    pub cve: ::std::vec::Vec<std::string::String>,
}
/// Windows Update represents the metadata about the update for the Windows
/// operating system. The fields in this message come from the Windows Update API
/// documented at
/// https://docs.microsoft.com/en-us/windows/win32/api/wuapi/nn-wuapi-iupdate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowsUpdate {
    /// Required - The unique identifier for the update.
    #[prost(message, optional, tag = "1")]
    pub identity: ::std::option::Option<windows_update::Identity>,
    /// The localized title of the update.
    #[prost(string, tag = "2")]
    pub title: std::string::String,
    /// The localized description of the update.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// The list of categories to which the update belongs.
    #[prost(message, repeated, tag = "4")]
    pub categories: ::std::vec::Vec<windows_update::Category>,
    /// The Microsoft Knowledge Base article IDs that are associated with the
    /// update.
    #[prost(string, repeated, tag = "5")]
    pub kb_article_ids: ::std::vec::Vec<std::string::String>,
    /// The hyperlink to the support information for the update.
    #[prost(string, tag = "6")]
    pub support_url: std::string::String,
    /// The last published timestamp of the update.
    #[prost(message, optional, tag = "7")]
    pub last_published_timestamp: ::std::option::Option<::prost_types::Timestamp>,
}
pub mod windows_update {
    /// The unique identifier of the update.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identity {
        /// The revision independent identifier of the update.
        #[prost(string, tag = "1")]
        pub update_id: std::string::String,
        /// The revision number of the update.
        #[prost(int32, tag = "2")]
        pub revision: i32,
    }
    /// The category to which the update belongs.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Category {
        /// The identifier of the category.
        #[prost(string, tag = "1")]
        pub category_id: std::string::String,
        /// The localized name of the category.
        #[prost(string, tag = "2")]
        pub name: std::string::String,
    }
}
/// An Upgrade Occurrence represents that a specific resource_url could install a
/// specific upgrade. This presence is supplied via local sources (i.e. it is
/// present in the mirror and the running system has noticed its availability).
/// For Windows, both distribution and windows_update contain information for the
/// Windows update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeOccurrence {
    /// Required for non-Windows OS. The package this Upgrade is for.
    #[prost(string, tag = "1")]
    pub package: std::string::String,
    /// Required for non-Windows OS. The version of the package in a machine +
    /// human readable form.
    #[prost(message, optional, tag = "3")]
    pub parsed_version: ::std::option::Option<Version>,
    /// Metadata about the upgrade for available for the specific operating system
    /// for the resource_url. This allows efficient filtering, as well as
    /// making it easier to use the occurrence.
    #[prost(message, optional, tag = "4")]
    pub distribution: ::std::option::Option<UpgradeDistribution>,
    /// Required for Windows OS. Represents the metadata about the Windows update.
    #[prost(message, optional, tag = "5")]
    pub windows_update: ::std::option::Option<WindowsUpdate>,
}
/// A security vulnerability that can be found in resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerabilityNote {
    /// The CVSS score of this vulnerability. CVSS score is on a scale of 0 - 10
    /// where 0 indicates low severity and 10 indicates high severity.
    #[prost(float, tag = "1")]
    pub cvss_score: f32,
    /// The note provider assigned severity of this vulnerability.
    #[prost(enumeration = "Severity", tag = "2")]
    pub severity: i32,
    /// Details of all known distros and packages affected by this vulnerability.
    #[prost(message, repeated, tag = "3")]
    pub details: ::std::vec::Vec<vulnerability_note::Detail>,
    /// The full description of the CVSSv3 for this vulnerability.
    #[prost(message, optional, tag = "4")]
    pub cvss_v3: ::std::option::Option<CvsSv3>,
    /// Windows details get their own format because the information format and
    /// model don't match a normal detail. Specifically Windows updates are done as
    /// patches, thus Windows vulnerabilities really are a missing package, rather
    /// than a package being at an incorrect version.
    #[prost(message, repeated, tag = "5")]
    pub windows_details: ::std::vec::Vec<vulnerability_note::WindowsDetail>,
    /// The time this information was last changed at the source. This is an
    /// upstream timestamp from the underlying information source - e.g. Ubuntu
    /// security tracker.
    #[prost(message, optional, tag = "6")]
    pub source_update_time: ::std::option::Option<::prost_types::Timestamp>,
}
pub mod vulnerability_note {
    /// A detail for a distro and package affected by this vulnerability and its
    /// associated fix (if one is available).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Detail {
        /// The distro assigned severity of this vulnerability.
        #[prost(string, tag = "1")]
        pub severity_name: std::string::String,
        /// A vendor-specific description of this vulnerability.
        #[prost(string, tag = "2")]
        pub description: std::string::String,
        /// The type of package; whether native or non native (e.g., ruby gems,
        /// node.js packages, etc.).
        #[prost(string, tag = "3")]
        pub package_type: std::string::String,
        /// Required. The [CPE URI](https://cpe.mitre.org/specification/) this
        /// vulnerability affects.
        #[prost(string, tag = "4")]
        pub affected_cpe_uri: std::string::String,
        /// Required. The package this vulnerability affects.
        #[prost(string, tag = "5")]
        pub affected_package: std::string::String,
        /// The version number at the start of an interval in which this
        /// vulnerability exists. A vulnerability can affect a package between
        /// version numbers that are disjoint sets of intervals (example:
        /// [1.0.0-1.1.0], [2.4.6-2.4.8] and [4.5.6-4.6.8]) each of which will be
        /// represented in its own Detail. If a specific affected version is provided
        /// by a vulnerability database, affected_version_start and
        /// affected_version_end will be the same in that Detail.
        #[prost(message, optional, tag = "6")]
        pub affected_version_start: ::std::option::Option<super::Version>,
        /// The version number at the end of an interval in which this vulnerability
        /// exists. A vulnerability can affect a package between version numbers
        /// that are disjoint sets of intervals (example: [1.0.0-1.1.0],
        /// [2.4.6-2.4.8] and [4.5.6-4.6.8]) each of which will be represented in its
        /// own Detail. If a specific affected version is provided by a vulnerability
        /// database, affected_version_start and affected_version_end will be the
        /// same in that Detail.
        #[prost(message, optional, tag = "7")]
        pub affected_version_end: ::std::option::Option<super::Version>,
        /// The distro recommended [CPE URI](https://cpe.mitre.org/specification/)
        /// to update to that contains a fix for this vulnerability. It is possible
        /// for this to be different from the affected_cpe_uri.
        #[prost(string, tag = "8")]
        pub fixed_cpe_uri: std::string::String,
        /// The distro recommended package to update to that contains a fix for this
        /// vulnerability. It is possible for this to be different from the
        /// affected_package.
        #[prost(string, tag = "9")]
        pub fixed_package: std::string::String,
        /// The distro recommended version to update to that contains a
        /// fix for this vulnerability. Setting this to VersionKind.MAXIMUM means no
        /// such version is yet available.
        #[prost(message, optional, tag = "10")]
        pub fixed_version: ::std::option::Option<super::Version>,
        /// Whether this detail is obsolete. Occurrences are expected not to point to
        /// obsolete details.
        #[prost(bool, tag = "11")]
        pub is_obsolete: bool,
        /// The time this information was last changed at the source. This is an
        /// upstream timestamp from the underlying information source - e.g. Ubuntu
        /// security tracker.
        #[prost(message, optional, tag = "12")]
        pub source_update_time: ::std::option::Option<::prost_types::Timestamp>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsDetail {
        /// Required. The [CPE URI](https://cpe.mitre.org/specification/) this
        /// vulnerability affects.
        #[prost(string, tag = "1")]
        pub cpe_uri: std::string::String,
        /// Required. The name of this vulnerability.
        #[prost(string, tag = "2")]
        pub name: std::string::String,
        /// The description of this vulnerability.
        #[prost(string, tag = "3")]
        pub description: std::string::String,
        /// Required. The names of the KBs which have hotfixes to mitigate this
        /// vulnerability. Note that there may be multiple hotfixes (and thus
        /// multiple KBs) that mitigate a given vulnerability. Currently any listed
        /// KBs presence is considered a fix.
        #[prost(message, repeated, tag = "4")]
        pub fixing_kbs: ::std::vec::Vec<windows_detail::KnowledgeBase>,
    }
    pub mod windows_detail {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct KnowledgeBase {
            /// The KB name (generally of the form KB[0-9]+ (e.g., KB123456)).
            #[prost(string, tag = "1")]
            pub name: std::string::String,
            /// A link to the KB in the [Windows update catalog]
            /// (https://www.catalog.update.microsoft.com/).
            #[prost(string, tag = "2")]
            pub url: std::string::String,
        }
    }
}
/// An occurrence of a severity vulnerability on a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerabilityOccurrence {
    /// The type of package; whether native or non native (e.g., ruby gems, node.js
    /// packages, etc.).
    #[prost(string, tag = "1")]
    pub r#type: std::string::String,
    /// Output only. The note provider assigned severity of this vulnerability.
    #[prost(enumeration = "Severity", tag = "2")]
    pub severity: i32,
    /// Output only. The CVSS score of this vulnerability. CVSS score is on a
    /// scale of 0 - 10 where 0 indicates low severity and 10 indicates high
    /// severity.
    #[prost(float, tag = "3")]
    pub cvss_score: f32,
    /// Required. The set of affected locations and their fixes (if available)
    /// within the associated resource.
    #[prost(message, repeated, tag = "4")]
    pub package_issue: ::std::vec::Vec<vulnerability_occurrence::PackageIssue>,
    /// Output only. A one sentence description of this vulnerability.
    #[prost(string, tag = "5")]
    pub short_description: std::string::String,
    /// Output only. A detailed description of this vulnerability.
    #[prost(string, tag = "6")]
    pub long_description: std::string::String,
    /// Output only. URLs related to this vulnerability.
    #[prost(message, repeated, tag = "7")]
    pub related_urls: ::std::vec::Vec<RelatedUrl>,
    /// The distro assigned severity for this vulnerability when it is available,
    /// otherwise this is the note provider assigned severity.
    #[prost(enumeration = "Severity", tag = "8")]
    pub effective_severity: i32,
    /// Output only. Whether at least one of the affected packages has a fix
    /// available.
    #[prost(bool, tag = "9")]
    pub fix_available: bool,
}
pub mod vulnerability_occurrence {
    /// A detail for a distro and package this vulnerability occurrence was found
    /// in and its associated fix (if one is available).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PackageIssue {
        /// Required. The [CPE URI](https://cpe.mitre.org/specification/) this
        /// vulnerability was found in.
        #[prost(string, tag = "1")]
        pub affected_cpe_uri: std::string::String,
        /// Required. The package this vulnerability was found in.
        #[prost(string, tag = "2")]
        pub affected_package: std::string::String,
        /// Required. The version of the package that is installed on the resource
        /// affected by this vulnerability.
        #[prost(message, optional, tag = "3")]
        pub affected_version: ::std::option::Option<super::Version>,
        /// The [CPE URI](https://cpe.mitre.org/specification/) this vulnerability
        /// was fixed in. It is possible for this to be different from the
        /// affected_cpe_uri.
        #[prost(string, tag = "4")]
        pub fixed_cpe_uri: std::string::String,
        /// The package this vulnerability was fixed in. It is possible for this to
        /// be different from the affected_package.
        #[prost(string, tag = "5")]
        pub fixed_package: std::string::String,
        /// Required. The version of the package this vulnerability was fixed in.
        /// Setting this to VersionKind.MAXIMUM means no fix is yet available.
        #[prost(message, optional, tag = "6")]
        pub fixed_version: ::std::option::Option<super::Version>,
        /// Output only. Whether a fix is available for this package.
        #[prost(bool, tag = "7")]
        pub fix_available: bool,
    }
}
/// Note provider assigned severity/impact ranking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Severity {
    /// Unknown.
    Unspecified = 0,
    /// Minimal severity.
    Minimal = 1,
    /// Low severity.
    Low = 2,
    /// Medium severity.
    Medium = 3,
    /// High severity.
    High = 4,
    /// Critical severity.
    Critical = 5,
}
/// An instance of an analysis type that has been found on a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Occurrence {
    /// Output only. The name of the occurrence in the form of
    /// `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Immutable. A URI that represents the resource for which the
    /// occurrence applies. For example,
    /// `https://gcr.io/project/image@sha256:123abc` for a Docker image.
    #[prost(string, tag = "2")]
    pub resource_uri: std::string::String,
    /// Required. Immutable. The analysis note associated with this occurrence, in
    /// the form of `projects/[PROVIDER_ID]/notes/[NOTE_ID]`. This field can be
    /// used as a filter in list requests.
    #[prost(string, tag = "3")]
    pub note_name: std::string::String,
    /// Output only. This explicitly denotes which of the occurrence details are
    /// specified. This field can be used as a filter in list requests.
    #[prost(enumeration = "NoteKind", tag = "4")]
    pub kind: i32,
    /// A description of actions that can be taken to remedy the note.
    #[prost(string, tag = "5")]
    pub remediation: std::string::String,
    /// Output only. The time this occurrence was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this occurrence was last updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Required. Immutable. Describes the details of the note kind found on this
    /// resource.
    #[prost(oneof = "occurrence::Details", tags = "8, 9, 10, 11, 12, 13, 14, 15")]
    pub details: ::std::option::Option<occurrence::Details>,
}
pub mod occurrence {
    /// Required. Immutable. Describes the details of the note kind found on this
    /// resource.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Describes a security vulnerability.
        #[prost(message, tag = "8")]
        Vulnerability(super::VulnerabilityOccurrence),
        /// Describes a verifiable build.
        #[prost(message, tag = "9")]
        Build(super::BuildOccurrence),
        /// Describes how this resource derives from the basis in the associated
        /// note.
        #[prost(message, tag = "10")]
        Image(super::ImageOccurrence),
        /// Describes the installation of a package on the linked resource.
        #[prost(message, tag = "11")]
        Package(super::PackageOccurrence),
        /// Describes the deployment of an artifact on a runtime.
        #[prost(message, tag = "12")]
        Deployment(super::DeploymentOccurrence),
        /// Describes when a resource was discovered.
        #[prost(message, tag = "13")]
        Discovery(super::DiscoveryOccurrence),
        /// Describes an attestation of an artifact.
        #[prost(message, tag = "14")]
        Attestation(super::AttestationOccurrence),
        /// Describes an available package upgrade on the linked resource.
        #[prost(message, tag = "15")]
        Upgrade(super::UpgradeOccurrence),
    }
}
/// A type of analysis that can be done for a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Note {
    /// Output only. The name of the note in the form of
    /// `projects/[PROVIDER_ID]/notes/[NOTE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A one sentence description of this note.
    #[prost(string, tag = "2")]
    pub short_description: std::string::String,
    /// A detailed description of this note.
    #[prost(string, tag = "3")]
    pub long_description: std::string::String,
    /// Output only. The type of analysis. This field can be used as a filter in
    /// list requests.
    #[prost(enumeration = "NoteKind", tag = "4")]
    pub kind: i32,
    /// URLs associated with this note.
    #[prost(message, repeated, tag = "5")]
    pub related_url: ::std::vec::Vec<RelatedUrl>,
    /// Time of expiration for this note. Empty if note does not expire.
    #[prost(message, optional, tag = "6")]
    pub expiration_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this note was created. This field can be used as a
    /// filter in list requests.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this note was last updated. This field can be used as
    /// a filter in list requests.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Other notes related to this note.
    #[prost(string, repeated, tag = "9")]
    pub related_note_names: ::std::vec::Vec<std::string::String>,
    /// Required. Immutable. The type of analysis this note represents.
    #[prost(oneof = "note::Type", tags = "10, 11, 12, 13, 14, 15, 16, 17")]
    pub r#type: ::std::option::Option<note::Type>,
}
pub mod note {
    /// Required. Immutable. The type of analysis this note represents.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// A note describing a package vulnerability.
        #[prost(message, tag = "10")]
        Vulnerability(super::VulnerabilityNote),
        /// A note describing build provenance for a verifiable build.
        #[prost(message, tag = "11")]
        Build(super::BuildNote),
        /// A note describing a base image.
        #[prost(message, tag = "12")]
        Image(super::ImageNote),
        /// A note describing a package hosted by various package managers.
        #[prost(message, tag = "13")]
        Package(super::PackageNote),
        /// A note describing something that can be deployed.
        #[prost(message, tag = "14")]
        Deployment(super::DeploymentNote),
        /// A note describing the initial analysis of a resource.
        #[prost(message, tag = "15")]
        Discovery(super::DiscoveryNote),
        /// A note describing an attestation role.
        #[prost(message, tag = "16")]
        Attestation(super::AttestationNote),
        /// A note describing available package upgrades.
        #[prost(message, tag = "17")]
        Upgrade(super::UpgradeNote),
    }
}
/// Request to get an occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOccurrenceRequest {
    /// The name of the occurrence in the form of
    /// `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request to list occurrences.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOccurrencesRequest {
    /// The name of the project to list occurrences for in the form of
    /// `projects/[PROJECT_ID]`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The filter expression.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Number of occurrences to return in the list. Must be positive. Max allowed
    /// page size is 1000. If not specified, page size defaults to 20.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// Response for listing occurrences.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOccurrencesResponse {
    /// The occurrences requested.
    #[prost(message, repeated, tag = "1")]
    pub occurrences: ::std::vec::Vec<Occurrence>,
    /// The next pagination token in the list response. It should be used as
    /// `page_token` for the following request. An empty value means no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request to delete an occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOccurrenceRequest {
    /// The name of the occurrence in the form of
    /// `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request to create a new occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOccurrenceRequest {
    /// The name of the project in the form of `projects/[PROJECT_ID]`, under which
    /// the occurrence is to be created.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The occurrence to create.
    #[prost(message, optional, tag = "2")]
    pub occurrence: ::std::option::Option<Occurrence>,
}
/// Request to update an occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOccurrenceRequest {
    /// The name of the occurrence in the form of
    /// `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The updated occurrence.
    #[prost(message, optional, tag = "2")]
    pub occurrence: ::std::option::Option<Occurrence>,
    /// The fields to update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request to get a note.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNoteRequest {
    /// The name of the note in the form of
    /// `projects/[PROVIDER_ID]/notes/[NOTE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request to get the note to which the specified occurrence is attached.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOccurrenceNoteRequest {
    /// The name of the occurrence in the form of
    /// `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request to list notes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotesRequest {
    /// The name of the project to list notes for in the form of
    /// `projects/[PROJECT_ID]`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The filter expression.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Number of notes to return in the list. Must be positive. Max allowed page
    /// size is 1000. If not specified, page size defaults to 20.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// Response for listing notes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotesResponse {
    /// The notes requested.
    #[prost(message, repeated, tag = "1")]
    pub notes: ::std::vec::Vec<Note>,
    /// The next pagination token in the list response. It should be used as
    /// `page_token` for the following request. An empty value means no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request to delete a note.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNoteRequest {
    /// The name of the note in the form of
    /// `projects/[PROVIDER_ID]/notes/[NOTE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request to create a new note.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNoteRequest {
    /// The name of the project in the form of `projects/[PROJECT_ID]`, under which
    /// the note is to be created.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The ID to use for this note.
    #[prost(string, tag = "2")]
    pub note_id: std::string::String,
    /// The note to create.
    #[prost(message, optional, tag = "3")]
    pub note: ::std::option::Option<Note>,
}
/// Request to update a note.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNoteRequest {
    /// The name of the note in the form of
    /// `projects/[PROVIDER_ID]/notes/[NOTE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The updated note.
    #[prost(message, optional, tag = "2")]
    pub note: ::std::option::Option<Note>,
    /// The fields to update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request to list occurrences for a note.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNoteOccurrencesRequest {
    /// The name of the note to list occurrences for in the form of
    /// `projects/[PROVIDER_ID]/notes/[NOTE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The filter expression.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Number of occurrences to return in the list.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// Response for listing occurrences for a note.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNoteOccurrencesResponse {
    /// The occurrences attached to the specified note.
    #[prost(message, repeated, tag = "1")]
    pub occurrences: ::std::vec::Vec<Occurrence>,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request to create notes in batch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateNotesRequest {
    /// The name of the project in the form of `projects/[PROJECT_ID]`, under which
    /// the notes are to be created.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The notes to create. Max allowed length is 1000.
    #[prost(map = "string, message", tag = "2")]
    pub notes: ::std::collections::HashMap<std::string::String, Note>,
}
/// Response for creating notes in batch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateNotesResponse {
    /// The notes that were created.
    #[prost(message, repeated, tag = "1")]
    pub notes: ::std::vec::Vec<Note>,
}
/// Request to create occurrences in batch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateOccurrencesRequest {
    /// The name of the project in the form of `projects/[PROJECT_ID]`, under which
    /// the occurrences are to be created.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The occurrences to create. Max allowed length is 1000.
    #[prost(message, repeated, tag = "2")]
    pub occurrences: ::std::vec::Vec<Occurrence>,
}
/// Response for creating occurrences in batch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateOccurrencesResponse {
    /// The occurrences that were created.
    #[prost(message, repeated, tag = "1")]
    pub occurrences: ::std::vec::Vec<Occurrence>,
}
#[doc = r" Generated client implementations."]
pub mod grafeas_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " [Grafeas](https://grafeas.io) API."]
    #[doc = ""]
    #[doc = " Retrieves analysis results of Cloud components such as Docker container"]
    #[doc = " images."]
    #[doc = ""]
    #[doc = " Analysis results are stored as a series of occurrences. An `Occurrence`"]
    #[doc = " contains information about a specific analysis instance on a resource. An"]
    #[doc = " occurrence refers to a `Note`. A note contains details describing the"]
    #[doc = " analysis and is generally stored in a separate project, called a `Provider`."]
    #[doc = " Multiple occurrences can refer to the same note."]
    #[doc = ""]
    #[doc = " For example, an SSL vulnerability could affect multiple images. In this case,"]
    #[doc = " there would be one note for the vulnerability and an occurrence for each"]
    #[doc = " image with the vulnerability referring to that note."]
    pub struct GrafeasClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GrafeasClient<T>
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
        #[doc = " Gets the specified occurrence."]
        pub async fn get_occurrence(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOccurrenceRequest>,
        ) -> Result<tonic::Response<super::Occurrence>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/GetOccurrence");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists occurrences for the specified project."]
        pub async fn list_occurrences(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOccurrencesRequest>,
        ) -> Result<tonic::Response<super::ListOccurrencesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/ListOccurrences");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified occurrence. For example, use this method to delete an"]
        #[doc = " occurrence when the occurrence is no longer applicable for the given"]
        #[doc = " resource."]
        pub async fn delete_occurrence(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOccurrenceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/DeleteOccurrence");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new occurrence."]
        pub async fn create_occurrence(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOccurrenceRequest>,
        ) -> Result<tonic::Response<super::Occurrence>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/CreateOccurrence");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates new occurrences in batch."]
        pub async fn batch_create_occurrences(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateOccurrencesRequest>,
        ) -> Result<tonic::Response<super::BatchCreateOccurrencesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/BatchCreateOccurrences");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified occurrence."]
        pub async fn update_occurrence(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOccurrenceRequest>,
        ) -> Result<tonic::Response<super::Occurrence>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/UpdateOccurrence");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the note attached to the specified occurrence. Consumer projects can"]
        #[doc = " use this method to get a note that belongs to a provider project."]
        pub async fn get_occurrence_note(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOccurrenceNoteRequest>,
        ) -> Result<tonic::Response<super::Note>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/GetOccurrenceNote");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the specified note."]
        pub async fn get_note(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNoteRequest>,
        ) -> Result<tonic::Response<super::Note>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/GetNote");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists notes for the specified project."]
        pub async fn list_notes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotesRequest>,
        ) -> Result<tonic::Response<super::ListNotesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/ListNotes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified note."]
        pub async fn delete_note(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNoteRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/DeleteNote");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new note."]
        pub async fn create_note(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNoteRequest>,
        ) -> Result<tonic::Response<super::Note>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/CreateNote");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates new notes in batch."]
        pub async fn batch_create_notes(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateNotesRequest>,
        ) -> Result<tonic::Response<super::BatchCreateNotesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/BatchCreateNotes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified note."]
        pub async fn update_note(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNoteRequest>,
        ) -> Result<tonic::Response<super::Note>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/UpdateNote");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists occurrences referencing the specified note. Provider projects can use"]
        #[doc = " this method to get all occurrences across consumer projects referencing the"]
        #[doc = " specified note."]
        pub async fn list_note_occurrences(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNoteOccurrencesRequest>,
        ) -> Result<tonic::Response<super::ListNoteOccurrencesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/grafeas.v1.Grafeas/ListNoteOccurrences");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GrafeasClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GrafeasClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GrafeasClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod grafeas_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GrafeasServer."]
    #[async_trait]
    pub trait Grafeas: Send + Sync + 'static {
        #[doc = " Gets the specified occurrence."]
        async fn get_occurrence(
            &self,
            request: tonic::Request<super::GetOccurrenceRequest>,
        ) -> Result<tonic::Response<super::Occurrence>, tonic::Status>;
        #[doc = " Lists occurrences for the specified project."]
        async fn list_occurrences(
            &self,
            request: tonic::Request<super::ListOccurrencesRequest>,
        ) -> Result<tonic::Response<super::ListOccurrencesResponse>, tonic::Status>;
        #[doc = " Deletes the specified occurrence. For example, use this method to delete an"]
        #[doc = " occurrence when the occurrence is no longer applicable for the given"]
        #[doc = " resource."]
        async fn delete_occurrence(
            &self,
            request: tonic::Request<super::DeleteOccurrenceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Creates a new occurrence."]
        async fn create_occurrence(
            &self,
            request: tonic::Request<super::CreateOccurrenceRequest>,
        ) -> Result<tonic::Response<super::Occurrence>, tonic::Status>;
        #[doc = " Creates new occurrences in batch."]
        async fn batch_create_occurrences(
            &self,
            request: tonic::Request<super::BatchCreateOccurrencesRequest>,
        ) -> Result<tonic::Response<super::BatchCreateOccurrencesResponse>, tonic::Status>;
        #[doc = " Updates the specified occurrence."]
        async fn update_occurrence(
            &self,
            request: tonic::Request<super::UpdateOccurrenceRequest>,
        ) -> Result<tonic::Response<super::Occurrence>, tonic::Status>;
        #[doc = " Gets the note attached to the specified occurrence. Consumer projects can"]
        #[doc = " use this method to get a note that belongs to a provider project."]
        async fn get_occurrence_note(
            &self,
            request: tonic::Request<super::GetOccurrenceNoteRequest>,
        ) -> Result<tonic::Response<super::Note>, tonic::Status>;
        #[doc = " Gets the specified note."]
        async fn get_note(
            &self,
            request: tonic::Request<super::GetNoteRequest>,
        ) -> Result<tonic::Response<super::Note>, tonic::Status>;
        #[doc = " Lists notes for the specified project."]
        async fn list_notes(
            &self,
            request: tonic::Request<super::ListNotesRequest>,
        ) -> Result<tonic::Response<super::ListNotesResponse>, tonic::Status>;
        #[doc = " Deletes the specified note."]
        async fn delete_note(
            &self,
            request: tonic::Request<super::DeleteNoteRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Creates a new note."]
        async fn create_note(
            &self,
            request: tonic::Request<super::CreateNoteRequest>,
        ) -> Result<tonic::Response<super::Note>, tonic::Status>;
        #[doc = " Creates new notes in batch."]
        async fn batch_create_notes(
            &self,
            request: tonic::Request<super::BatchCreateNotesRequest>,
        ) -> Result<tonic::Response<super::BatchCreateNotesResponse>, tonic::Status>;
        #[doc = " Updates the specified note."]
        async fn update_note(
            &self,
            request: tonic::Request<super::UpdateNoteRequest>,
        ) -> Result<tonic::Response<super::Note>, tonic::Status>;
        #[doc = " Lists occurrences referencing the specified note. Provider projects can use"]
        #[doc = " this method to get all occurrences across consumer projects referencing the"]
        #[doc = " specified note."]
        async fn list_note_occurrences(
            &self,
            request: tonic::Request<super::ListNoteOccurrencesRequest>,
        ) -> Result<tonic::Response<super::ListNoteOccurrencesResponse>, tonic::Status>;
    }
    #[doc = " [Grafeas](https://grafeas.io) API."]
    #[doc = ""]
    #[doc = " Retrieves analysis results of Cloud components such as Docker container"]
    #[doc = " images."]
    #[doc = ""]
    #[doc = " Analysis results are stored as a series of occurrences. An `Occurrence`"]
    #[doc = " contains information about a specific analysis instance on a resource. An"]
    #[doc = " occurrence refers to a `Note`. A note contains details describing the"]
    #[doc = " analysis and is generally stored in a separate project, called a `Provider`."]
    #[doc = " Multiple occurrences can refer to the same note."]
    #[doc = ""]
    #[doc = " For example, an SSL vulnerability could affect multiple images. In this case,"]
    #[doc = " there would be one note for the vulnerability and an occurrence for each"]
    #[doc = " image with the vulnerability referring to that note."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct GrafeasServer<T: Grafeas> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Grafeas> GrafeasServer<T> {
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
    impl<T, B> Service<http::Request<B>> for GrafeasServer<T>
    where
        T: Grafeas,
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
                "/grafeas.v1.Grafeas/GetOccurrence" => {
                    #[allow(non_camel_case_types)]
                    struct GetOccurrenceSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::GetOccurrenceRequest> for GetOccurrenceSvc<T> {
                        type Response = super::Occurrence;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOccurrenceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_occurrence(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetOccurrenceSvc(inner);
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
                "/grafeas.v1.Grafeas/ListOccurrences" => {
                    #[allow(non_camel_case_types)]
                    struct ListOccurrencesSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::ListOccurrencesRequest>
                        for ListOccurrencesSvc<T>
                    {
                        type Response = super::ListOccurrencesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListOccurrencesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_occurrences(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListOccurrencesSvc(inner);
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
                "/grafeas.v1.Grafeas/DeleteOccurrence" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOccurrenceSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::DeleteOccurrenceRequest>
                        for DeleteOccurrenceSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOccurrenceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_occurrence(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteOccurrenceSvc(inner);
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
                "/grafeas.v1.Grafeas/CreateOccurrence" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOccurrenceSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::CreateOccurrenceRequest>
                        for CreateOccurrenceSvc<T>
                    {
                        type Response = super::Occurrence;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOccurrenceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_occurrence(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateOccurrenceSvc(inner);
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
                "/grafeas.v1.Grafeas/BatchCreateOccurrences" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateOccurrencesSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas>
                        tonic::server::UnaryService<super::BatchCreateOccurrencesRequest>
                        for BatchCreateOccurrencesSvc<T>
                    {
                        type Response = super::BatchCreateOccurrencesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchCreateOccurrencesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_create_occurrences(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchCreateOccurrencesSvc(inner);
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
                "/grafeas.v1.Grafeas/UpdateOccurrence" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOccurrenceSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::UpdateOccurrenceRequest>
                        for UpdateOccurrenceSvc<T>
                    {
                        type Response = super::Occurrence;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateOccurrenceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_occurrence(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateOccurrenceSvc(inner);
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
                "/grafeas.v1.Grafeas/GetOccurrenceNote" => {
                    #[allow(non_camel_case_types)]
                    struct GetOccurrenceNoteSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::GetOccurrenceNoteRequest>
                        for GetOccurrenceNoteSvc<T>
                    {
                        type Response = super::Note;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOccurrenceNoteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_occurrence_note(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetOccurrenceNoteSvc(inner);
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
                "/grafeas.v1.Grafeas/GetNote" => {
                    #[allow(non_camel_case_types)]
                    struct GetNoteSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::GetNoteRequest> for GetNoteSvc<T> {
                        type Response = super::Note;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNoteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_note(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetNoteSvc(inner);
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
                "/grafeas.v1.Grafeas/ListNotes" => {
                    #[allow(non_camel_case_types)]
                    struct ListNotesSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::ListNotesRequest> for ListNotesSvc<T> {
                        type Response = super::ListNotesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListNotesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_notes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListNotesSvc(inner);
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
                "/grafeas.v1.Grafeas/DeleteNote" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteNoteSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::DeleteNoteRequest> for DeleteNoteSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteNoteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_note(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteNoteSvc(inner);
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
                "/grafeas.v1.Grafeas/CreateNote" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNoteSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::CreateNoteRequest> for CreateNoteSvc<T> {
                        type Response = super::Note;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateNoteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_note(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateNoteSvc(inner);
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
                "/grafeas.v1.Grafeas/BatchCreateNotes" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateNotesSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::BatchCreateNotesRequest>
                        for BatchCreateNotesSvc<T>
                    {
                        type Response = super::BatchCreateNotesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchCreateNotesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_create_notes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchCreateNotesSvc(inner);
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
                "/grafeas.v1.Grafeas/UpdateNote" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateNoteSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::UpdateNoteRequest> for UpdateNoteSvc<T> {
                        type Response = super::Note;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateNoteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_note(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateNoteSvc(inner);
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
                "/grafeas.v1.Grafeas/ListNoteOccurrences" => {
                    #[allow(non_camel_case_types)]
                    struct ListNoteOccurrencesSvc<T: Grafeas>(pub Arc<T>);
                    impl<T: Grafeas> tonic::server::UnaryService<super::ListNoteOccurrencesRequest>
                        for ListNoteOccurrencesSvc<T>
                    {
                        type Response = super::ListNoteOccurrencesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListNoteOccurrencesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_note_occurrences(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListNoteOccurrencesSvc(inner);
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
    impl<T: Grafeas> Clone for GrafeasServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Grafeas> Clone for _Inner<T> {
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
