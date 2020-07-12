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
}
/// An instance of an analysis type that has been found on a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Occurrence {
    /// Output only. The name of the occurrence in the form of
    /// `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Immutable. The resource for which the occurrence applies.
    #[prost(message, optional, tag = "2")]
    pub resource: ::std::option::Option<Resource>,
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
    #[prost(oneof = "occurrence::Details", tags = "8, 9, 10, 11, 12, 13, 14")]
    pub details: ::std::option::Option<occurrence::Details>,
}
pub mod occurrence {
    /// Required. Immutable. Describes the details of the note kind found on this
    /// resource.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Describes a security vulnerability.
        #[prost(message, tag = "8")]
        Vulnerability(super::vulnerability::Details),
        /// Describes a verifiable build.
        #[prost(message, tag = "9")]
        Build(super::build::Details),
        /// Describes how this resource derives from the basis in the associated
        /// note.
        #[prost(message, tag = "10")]
        DerivedImage(super::image::Details),
        /// Describes the installation of a package on the linked resource.
        #[prost(message, tag = "11")]
        Installation(super::package::Details),
        /// Describes the deployment of an artifact on a runtime.
        #[prost(message, tag = "12")]
        Deployment(super::deployment::Details),
        /// Describes when a resource was discovered.
        #[prost(message, tag = "13")]
        Discovered(super::discovery::Details),
        /// Describes an attestation of an artifact.
        #[prost(message, tag = "14")]
        Attestation(super::attestation::Details),
    }
}
/// An entity that can have metadata. For example, a Docker image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The name of the resource. For example, the name of a Docker image -
    /// "Debian".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The unique URI of the resource. For example,
    /// `https://gcr.io/project/image@sha256:foo` for a Docker image.
    #[prost(string, tag = "2")]
    pub uri: std::string::String,
    /// The hash of the resource content. For example, the Docker digest.
    #[prost(message, optional, tag = "3")]
    pub content_hash: ::std::option::Option<provenance::Hash>,
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
    #[prost(oneof = "note::Type", tags = "10, 11, 12, 13, 14, 15, 16")]
    pub r#type: ::std::option::Option<note::Type>,
}
pub mod note {
    /// Required. Immutable. The type of analysis this note represents.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// A note describing a package vulnerability.
        #[prost(message, tag = "10")]
        Vulnerability(super::vulnerability::Vulnerability),
        /// A note describing build provenance for a verifiable build.
        #[prost(message, tag = "11")]
        Build(super::build::Build),
        /// A note describing a base image.
        #[prost(message, tag = "12")]
        BaseImage(super::image::Basis),
        /// A note describing a package hosted by various package managers.
        #[prost(message, tag = "13")]
        Package(super::package::Package),
        /// A note describing something that can be deployed.
        #[prost(message, tag = "14")]
        Deployable(super::deployment::Deployable),
        /// A note describing the initial analysis of a resource.
        #[prost(message, tag = "15")]
        Discovery(super::discovery::Discovery),
        /// A note describing an attestation role.
        #[prost(message, tag = "16")]
        AttestationAuthority(super::attestation::Authority),
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
    /// Number of occurrences to return in the list.
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
/// Request to delete a occurrence.
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
    /// Number of notes to return in the list.
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
    /// The notes to create.
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
    /// The occurrences to create.
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
/// Request to get a vulnerability summary for some set of occurrences.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVulnerabilityOccurrencesSummaryRequest {
    /// The name of the project to get a vulnerability summary for in the form of
    /// `projects/[PROJECT_ID]`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The filter expression.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
}
/// A summary of how many vulnerability occurrences there are per resource and
/// severity type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VulnerabilityOccurrencesSummary {
    /// A listing by resource of the number of fixable and total vulnerabilities.
    #[prost(message, repeated, tag = "1")]
    pub counts: ::std::vec::Vec<vulnerability_occurrences_summary::FixableTotalByDigest>,
}
pub mod vulnerability_occurrences_summary {
    /// Per resource and severity counts of fixable and total vulnerabilities.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FixableTotalByDigest {
        /// The affected resource.
        #[prost(message, optional, tag = "1")]
        pub resource: ::std::option::Option<super::Resource>,
        /// The severity for this count. SEVERITY_UNSPECIFIED indicates total across
        /// all severities.
        #[prost(enumeration = "super::vulnerability::Severity", tag = "2")]
        pub severity: i32,
        /// The number of fixable vulnerabilities associated with this resource.
        #[prost(int64, tag = "3")]
        pub fixable_count: i64,
        /// The total number of vulnerabilities associated with this resource.
        #[prost(int64, tag = "4")]
        pub total_count: i64,
    }
}
#[doc = r" Generated client implementations."]
pub mod grafeas_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " [Grafeas](grafeas.io) API."]
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
    pub struct GrafeasV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GrafeasV1Beta1Client<T>
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
            let path = http::uri::PathAndQuery::from_static(
                "/grafeas.v1beta1.GrafeasV1Beta1/GetOccurrence",
            );
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
            let path = http::uri::PathAndQuery::from_static(
                "/grafeas.v1beta1.GrafeasV1Beta1/ListOccurrences",
            );
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
            let path = http::uri::PathAndQuery::from_static(
                "/grafeas.v1beta1.GrafeasV1Beta1/DeleteOccurrence",
            );
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
            let path = http::uri::PathAndQuery::from_static(
                "/grafeas.v1beta1.GrafeasV1Beta1/CreateOccurrence",
            );
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
            let path = http::uri::PathAndQuery::from_static(
                "/grafeas.v1beta1.GrafeasV1Beta1/BatchCreateOccurrences",
            );
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
            let path = http::uri::PathAndQuery::from_static(
                "/grafeas.v1beta1.GrafeasV1Beta1/UpdateOccurrence",
            );
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
            let path = http::uri::PathAndQuery::from_static(
                "/grafeas.v1beta1.GrafeasV1Beta1/GetOccurrenceNote",
            );
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
            let path =
                http::uri::PathAndQuery::from_static("/grafeas.v1beta1.GrafeasV1Beta1/GetNote");
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
            let path =
                http::uri::PathAndQuery::from_static("/grafeas.v1beta1.GrafeasV1Beta1/ListNotes");
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
            let path =
                http::uri::PathAndQuery::from_static("/grafeas.v1beta1.GrafeasV1Beta1/DeleteNote");
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
            let path =
                http::uri::PathAndQuery::from_static("/grafeas.v1beta1.GrafeasV1Beta1/CreateNote");
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
            let path = http::uri::PathAndQuery::from_static(
                "/grafeas.v1beta1.GrafeasV1Beta1/BatchCreateNotes",
            );
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
            let path =
                http::uri::PathAndQuery::from_static("/grafeas.v1beta1.GrafeasV1Beta1/UpdateNote");
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
            let path = http::uri::PathAndQuery::from_static(
                "/grafeas.v1beta1.GrafeasV1Beta1/ListNoteOccurrences",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a summary of the number and severity of occurrences."]
        pub async fn get_vulnerability_occurrences_summary(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVulnerabilityOccurrencesSummaryRequest>,
        ) -> Result<tonic::Response<super::VulnerabilityOccurrencesSummary>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grafeas.v1beta1.GrafeasV1Beta1/GetVulnerabilityOccurrencesSummary",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GrafeasV1Beta1Client<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GrafeasV1Beta1Client<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GrafeasV1Beta1Client {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod grafeas_v1_beta1_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GrafeasV1Beta1Server."]
    #[async_trait]
    pub trait GrafeasV1Beta1: Send + Sync + 'static {
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
        #[doc = " Gets a summary of the number and severity of occurrences."]
        async fn get_vulnerability_occurrences_summary(
            &self,
            request: tonic::Request<super::GetVulnerabilityOccurrencesSummaryRequest>,
        ) -> Result<tonic::Response<super::VulnerabilityOccurrencesSummary>, tonic::Status>;
    }
    #[doc = " [Grafeas](grafeas.io) API."]
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
    pub struct GrafeasV1Beta1Server<T: GrafeasV1Beta1> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GrafeasV1Beta1> GrafeasV1Beta1Server<T> {
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
    impl<T, B> Service<http::Request<B>> for GrafeasV1Beta1Server<T>
    where
        T: GrafeasV1Beta1,
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
                "/grafeas.v1beta1.GrafeasV1Beta1/GetOccurrence" => {
                    #[allow(non_camel_case_types)]
                    struct GetOccurrenceSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1> tonic::server::UnaryService<super::GetOccurrenceRequest>
                        for GetOccurrenceSvc<T>
                    {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/ListOccurrences" => {
                    #[allow(non_camel_case_types)]
                    struct ListOccurrencesSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1>
                        tonic::server::UnaryService<super::ListOccurrencesRequest>
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
                "/grafeas.v1beta1.GrafeasV1Beta1/DeleteOccurrence" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOccurrenceSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1>
                        tonic::server::UnaryService<super::DeleteOccurrenceRequest>
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
                "/grafeas.v1beta1.GrafeasV1Beta1/CreateOccurrence" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOccurrenceSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1>
                        tonic::server::UnaryService<super::CreateOccurrenceRequest>
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
                "/grafeas.v1beta1.GrafeasV1Beta1/BatchCreateOccurrences" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateOccurrencesSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1>
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
                "/grafeas.v1beta1.GrafeasV1Beta1/UpdateOccurrence" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOccurrenceSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1>
                        tonic::server::UnaryService<super::UpdateOccurrenceRequest>
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
                "/grafeas.v1beta1.GrafeasV1Beta1/GetOccurrenceNote" => {
                    #[allow(non_camel_case_types)]
                    struct GetOccurrenceNoteSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1>
                        tonic::server::UnaryService<super::GetOccurrenceNoteRequest>
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
                "/grafeas.v1beta1.GrafeasV1Beta1/GetNote" => {
                    #[allow(non_camel_case_types)]
                    struct GetNoteSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1> tonic::server::UnaryService<super::GetNoteRequest> for GetNoteSvc<T> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/ListNotes" => {
                    #[allow(non_camel_case_types)]
                    struct ListNotesSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1> tonic::server::UnaryService<super::ListNotesRequest> for ListNotesSvc<T> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/DeleteNote" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteNoteSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1> tonic::server::UnaryService<super::DeleteNoteRequest> for DeleteNoteSvc<T> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/CreateNote" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNoteSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1> tonic::server::UnaryService<super::CreateNoteRequest> for CreateNoteSvc<T> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/BatchCreateNotes" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateNotesSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1>
                        tonic::server::UnaryService<super::BatchCreateNotesRequest>
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
                "/grafeas.v1beta1.GrafeasV1Beta1/UpdateNote" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateNoteSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1> tonic::server::UnaryService<super::UpdateNoteRequest> for UpdateNoteSvc<T> {
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
                "/grafeas.v1beta1.GrafeasV1Beta1/ListNoteOccurrences" => {
                    #[allow(non_camel_case_types)]
                    struct ListNoteOccurrencesSvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1>
                        tonic::server::UnaryService<super::ListNoteOccurrencesRequest>
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
                "/grafeas.v1beta1.GrafeasV1Beta1/GetVulnerabilityOccurrencesSummary" => {
                    #[allow(non_camel_case_types)]
                    struct GetVulnerabilityOccurrencesSummarySvc<T: GrafeasV1Beta1>(pub Arc<T>);
                    impl<T: GrafeasV1Beta1>
                        tonic::server::UnaryService<
                            super::GetVulnerabilityOccurrencesSummaryRequest,
                        > for GetVulnerabilityOccurrencesSummarySvc<T>
                    {
                        type Response = super::VulnerabilityOccurrencesSummary;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetVulnerabilityOccurrencesSummaryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                inner.get_vulnerability_occurrences_summary(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetVulnerabilityOccurrencesSummarySvc(inner);
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
    impl<T: GrafeasV1Beta1> Clone for GrafeasV1Beta1Server<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GrafeasV1Beta1> Clone for _Inner<T> {
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
