/// A \[policy][google.cloud.binaryauthorization.v1.Policy\] for container image binary authorization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    /// Output only. The resource name, in the format `projects/*/policy`. There is
    /// at most one policy per project.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A descriptive comment.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Controls the evaluation of a Google-maintained global admission
    /// policy for common system-level images. Images not covered by the global
    /// policy will be subject to the project admission policy. This setting
    /// has no effect when specified inside a global admission policy.
    #[prost(enumeration = "policy::GlobalPolicyEvaluationMode", tag = "7")]
    pub global_policy_evaluation_mode: i32,
    /// Optional. Admission policy allowlisting. A matching admission request will
    /// always be permitted. This feature is typically used to exclude Google or
    /// third-party infrastructure images from Binary Authorization policies.
    #[prost(message, repeated, tag = "2")]
    pub admission_whitelist_patterns: ::prost::alloc::vec::Vec<AdmissionWhitelistPattern>,
    /// Optional. Per-cluster admission rules. Cluster spec format:
    /// `location.clusterId`. There can be at most one admission rule per cluster
    /// spec.
    /// A `location` is either a compute zone (e.g. us-central1-a) or a region
    /// (e.g. us-central1).
    /// For `clusterId` syntax restrictions see
    /// <https://cloud.google.com/container-engine/reference/rest/v1/projects.zones.clusters.>
    #[prost(map = "string, message", tag = "3")]
    pub cluster_admission_rules:
        ::std::collections::HashMap<::prost::alloc::string::String, AdmissionRule>,
    /// Optional. Per-kubernetes-namespace admission rules. K8s namespace spec format:
    /// \[a-z.-\]+, e.g. 'some-namespace'
    #[prost(map = "string, message", tag = "10")]
    pub kubernetes_namespace_admission_rules:
        ::std::collections::HashMap<::prost::alloc::string::String, AdmissionRule>,
    /// Optional. Per-kubernetes-service-account admission rules. Service account
    /// spec format: `namespace:serviceaccount`. e.g. 'test-ns:default'
    #[prost(map = "string, message", tag = "8")]
    pub kubernetes_service_account_admission_rules:
        ::std::collections::HashMap<::prost::alloc::string::String, AdmissionRule>,
    /// Optional. Per-istio-service-identity admission rules. Istio service
    /// identity spec format:
    /// spiffe://<domain>/ns/<namespace>/sa/<serviceaccount> or
    /// <domain>/ns/<namespace>/sa/<serviceaccount>
    /// e.g. spiffe://example.com/ns/test-ns/sa/default
    #[prost(map = "string, message", tag = "9")]
    pub istio_service_identity_admission_rules:
        ::std::collections::HashMap<::prost::alloc::string::String, AdmissionRule>,
    /// Required. Default admission rule for a cluster without a per-cluster, per-
    /// kubernetes-service-account, or per-istio-service-identity admission rule.
    #[prost(message, optional, tag = "4")]
    pub default_admission_rule: ::core::option::Option<AdmissionRule>,
    /// Output only. Time when the policy was last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Policy`.
pub mod policy {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GlobalPolicyEvaluationMode {
        /// Not specified: DISABLE is assumed.
        Unspecified = 0,
        /// Enables system policy evaluation.
        Enable = 1,
        /// Disables system policy evaluation.
        Disable = 2,
    }
}
/// An [admission allowlist pattern]\[google.cloud.binaryauthorization.v1.AdmissionWhitelistPattern\] exempts images
/// from checks by [admission rules]\[google.cloud.binaryauthorization.v1.AdmissionRule\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdmissionWhitelistPattern {
    /// An image name pattern to allowlist, in the form `registry/path/to/image`.
    /// This supports a trailing `*` wildcard, but this is allowed only in
    /// text after the `registry/` part. This also supports a trailing `**`
    /// wildcard which matches subdirectories of a given entry.
    #[prost(string, tag = "1")]
    pub name_pattern: ::prost::alloc::string::String,
}
/// An [admission rule]\[google.cloud.binaryauthorization.v1.AdmissionRule\] specifies either that all container images
/// used in a pod creation request must be attested to by one or more
/// \[attestors][google.cloud.binaryauthorization.v1.Attestor\], that all pod creations will be allowed, or that all
/// pod creations will be denied.
///
/// Images matching an [admission allowlist pattern]\[google.cloud.binaryauthorization.v1.AdmissionWhitelistPattern\]
/// are exempted from admission rules and will never block a pod creation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdmissionRule {
    /// Required. How this admission rule will be evaluated.
    #[prost(enumeration = "admission_rule::EvaluationMode", tag = "1")]
    pub evaluation_mode: i32,
    /// Optional. The resource names of the attestors that must attest to
    /// a container image, in the format `projects/*/attestors/*`. Each
    /// attestor must exist before a policy can reference it.  To add an attestor
    /// to a policy the principal issuing the policy change request must be able
    /// to read the attestor resource.
    ///
    /// Note: this field must be non-empty when the evaluation_mode field specifies
    /// REQUIRE_ATTESTATION, otherwise it must be empty.
    #[prost(string, repeated, tag = "2")]
    pub require_attestations_by: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The action when a pod creation is denied by the admission rule.
    #[prost(enumeration = "admission_rule::EnforcementMode", tag = "3")]
    pub enforcement_mode: i32,
}
/// Nested message and enum types in `AdmissionRule`.
pub mod admission_rule {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EvaluationMode {
        /// Do not use.
        Unspecified = 0,
        /// This rule allows all all pod creations.
        AlwaysAllow = 1,
        /// This rule allows a pod creation if all the attestors listed in
        /// 'require_attestations_by' have valid attestations for all of the
        /// images in the pod spec.
        RequireAttestation = 2,
        /// This rule denies all pod creations.
        AlwaysDeny = 3,
    }
    /// Defines the possible actions when a pod creation is denied by an admission
    /// rule.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EnforcementMode {
        /// Do not use.
        Unspecified = 0,
        /// Enforce the admission rule by blocking the pod creation.
        EnforcedBlockAndAuditLog = 1,
        /// Dryrun mode: Audit logging only.  This will allow the pod creation as if
        /// the admission request had specified break-glass.
        DryrunAuditLogOnly = 2,
    }
}
/// An \[attestor][google.cloud.binaryauthorization.v1.Attestor\] that attests to container image
/// artifacts. An existing attestor cannot be modified except where
/// indicated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attestor {
    /// Required. The resource name, in the format:
    /// `projects/*/attestors/*`. This field may not be updated.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A descriptive comment.  This field may be updated.
    /// The field may be displayed in chooser dialogs.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time when the attestor was last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "attestor::AttestorType", tags = "3")]
    pub attestor_type: ::core::option::Option<attestor::AttestorType>,
}
/// Nested message and enum types in `Attestor`.
pub mod attestor {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AttestorType {
        /// This specifies how an attestation will be read, and how it will be used
        /// during policy enforcement.
        #[prost(message, tag = "3")]
        UserOwnedGrafeasNote(super::UserOwnedGrafeasNote),
    }
}
/// An [user owned Grafeas note]\[google.cloud.binaryauthorization.v1.UserOwnedGrafeasNote\] references a Grafeas
/// Attestation.Authority Note created by the user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserOwnedGrafeasNote {
    /// Required. The Grafeas resource name of a Attestation.Authority Note,
    /// created by the user, in the format: `projects/*/notes/*`. This field may
    /// not be updated.
    ///
    /// An attestation by this attestor is stored as a Grafeas
    /// Attestation.Authority Occurrence that names a container image and that
    /// links to this Note. Grafeas is an external dependency.
    #[prost(string, tag = "1")]
    pub note_reference: ::prost::alloc::string::String,
    /// Optional. Public keys that verify attestations signed by this
    /// attestor.  This field may be updated.
    ///
    /// If this field is non-empty, one of the specified public keys must
    /// verify that an attestation was signed by this attestor for the
    /// image specified in the admission request.
    ///
    /// If this field is empty, this attestor always returns that no
    /// valid attestations exist.
    #[prost(message, repeated, tag = "2")]
    pub public_keys: ::prost::alloc::vec::Vec<AttestorPublicKey>,
    /// Output only. This field will contain the service account email address
    /// that this Attestor will use as the principal when querying Container
    /// Analysis. Attestor administrators must grant this service account the
    /// IAM role needed to read attestations from the \[note_reference][Note\] in
    /// Container Analysis (`containeranalysis.notes.occurrences.viewer`).
    ///
    /// This email address is fixed for the lifetime of the Attestor, but callers
    /// should not make any other assumptions about the service account email;
    /// future versions may use an email based on a different naming pattern.
    #[prost(string, tag = "3")]
    pub delegation_service_account_email: ::prost::alloc::string::String,
}
/// A public key in the PkixPublicKey format (see
/// <https://tools.ietf.org/html/rfc5280#section-4.1.2.7> for details).
/// Public keys of this type are typically textually encoded using the PEM
/// format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PkixPublicKey {
    /// A PEM-encoded public key, as described in
    /// <https://tools.ietf.org/html/rfc7468#section-13>
    #[prost(string, tag = "1")]
    pub public_key_pem: ::prost::alloc::string::String,
    /// The signature algorithm used to verify a message against a signature using
    /// this key.
    /// These signature algorithm must match the structure and any object
    /// identifiers encoded in `public_key_pem` (i.e. this algorithm must match
    /// that of the public key).
    #[prost(enumeration = "pkix_public_key::SignatureAlgorithm", tag = "2")]
    pub signature_algorithm: i32,
}
/// Nested message and enum types in `PkixPublicKey`.
pub mod pkix_public_key {
    /// Represents a signature algorithm and other information necessary to verify
    /// signatures with a given public key.
    /// This is based primarily on the public key types supported by Tink's
    /// PemKeyType, which is in turn based on KMS's supported signing algorithms.
    /// See <https://cloud.google.com/kms/docs/algorithms.> In the future, BinAuthz
    /// might support additional public key types independently of Tink and/or KMS.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SignatureAlgorithm {
        /// Not specified.
        Unspecified = 0,
        /// RSASSA-PSS 2048 bit key with a SHA256 digest.
        RsaPss2048Sha256 = 1,
        /// RSASSA-PSS 3072 bit key with a SHA256 digest.
        RsaPss3072Sha256 = 2,
        /// RSASSA-PSS 4096 bit key with a SHA256 digest.
        RsaPss4096Sha256 = 3,
        /// RSASSA-PSS 4096 bit key with a SHA512 digest.
        RsaPss4096Sha512 = 4,
        /// RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest.
        RsaSignPkcs12048Sha256 = 5,
        /// RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest.
        RsaSignPkcs13072Sha256 = 6,
        /// RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest.
        RsaSignPkcs14096Sha256 = 7,
        /// RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest.
        RsaSignPkcs14096Sha512 = 8,
        /// ECDSA on the NIST P-256 curve with a SHA256 digest.
        EcdsaP256Sha256 = 9,
        /// ECDSA on the NIST P-384 curve with a SHA384 digest.
        EcdsaP384Sha384 = 10,
        /// ECDSA on the NIST P-521 curve with a SHA512 digest.
        EcdsaP521Sha512 = 11,
    }
}
/// An [attestor public key]\[google.cloud.binaryauthorization.v1.AttestorPublicKey\] that will be used to verify
/// attestations signed by this attestor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestorPublicKey {
    /// Optional. A descriptive comment. This field may be updated.
    #[prost(string, tag = "1")]
    pub comment: ::prost::alloc::string::String,
    /// The ID of this public key.
    /// Signatures verified by BinAuthz must include the ID of the public key that
    /// can be used to verify them, and that ID must match the contents of this
    /// field exactly.
    /// Additional restrictions on this field can be imposed based on which public
    /// key type is encapsulated. See the documentation on `public_key` cases below
    /// for details.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(oneof = "attestor_public_key::PublicKey", tags = "3, 5")]
    pub public_key: ::core::option::Option<attestor_public_key::PublicKey>,
}
/// Nested message and enum types in `AttestorPublicKey`.
pub mod attestor_public_key {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PublicKey {
        /// ASCII-armored representation of a PGP public key, as the entire output by
        /// the command `gpg --export --armor foo@example.com` (either LF or CRLF
        /// line endings).
        /// When using this field, `id` should be left blank.  The BinAuthz API
        /// handlers will calculate the ID and fill it in automatically.  BinAuthz
        /// computes this ID as the OpenPGP RFC4880 V4 fingerprint, represented as
        /// upper-case hex.  If `id` is provided by the caller, it will be
        /// overwritten by the API-calculated ID.
        #[prost(string, tag = "3")]
        AsciiArmoredPgpPublicKey(::prost::alloc::string::String),
        /// A raw PKIX SubjectPublicKeyInfo format public key.
        ///
        /// NOTE: `id` may be explicitly provided by the caller when using this
        /// type of public key, but it MUST be a valid RFC3986 URI. If `id` is left
        /// blank, a default one will be computed based on the digest of the DER
        /// encoding of the public key.
        #[prost(message, tag = "5")]
        PkixPublicKey(super::PkixPublicKey),
    }
}
/// Request message for \[BinauthzManagementService.GetPolicy][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyRequest {
    /// Required. The resource name of the \[policy][google.cloud.binaryauthorization.v1.Policy\] to retrieve,
    /// in the format `projects/*/policy`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[BinauthzManagementService.UpdatePolicy][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyRequest {
    /// Required. A new or updated \[policy][google.cloud.binaryauthorization.v1.Policy\] value. The service will
    /// overwrite the [policy name]\[google.cloud.binaryauthorization.v1.Policy.name\] field with the resource name in
    /// the request URL, in the format `projects/*/policy`.
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<Policy>,
}
/// Request message for \[BinauthzManagementService.CreateAttestor][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAttestorRequest {
    /// Required. The parent of this \[attestor][google.cloud.binaryauthorization.v1.Attestor\].
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The \[attestors][google.cloud.binaryauthorization.v1.Attestor\] ID.
    #[prost(string, tag = "2")]
    pub attestor_id: ::prost::alloc::string::String,
    /// Required. The initial \[attestor][google.cloud.binaryauthorization.v1.Attestor\] value. The service will
    /// overwrite the [attestor name]\[google.cloud.binaryauthorization.v1.Attestor.name\] field with the resource name,
    /// in the format `projects/*/attestors/*`.
    #[prost(message, optional, tag = "3")]
    pub attestor: ::core::option::Option<Attestor>,
}
/// Request message for \[BinauthzManagementService.GetAttestor][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAttestorRequest {
    /// Required. The name of the \[attestor][google.cloud.binaryauthorization.v1.Attestor\] to retrieve, in the format
    /// `projects/*/attestors/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for \[BinauthzManagementService.UpdateAttestor][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAttestorRequest {
    /// Required. The updated \[attestor][google.cloud.binaryauthorization.v1.Attestor\] value. The service will
    /// overwrite the [attestor name]\[google.cloud.binaryauthorization.v1.Attestor.name\] field with the resource name
    /// in the request URL, in the format `projects/*/attestors/*`.
    #[prost(message, optional, tag = "1")]
    pub attestor: ::core::option::Option<Attestor>,
}
/// Request message for \[BinauthzManagementService.ListAttestors][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttestorsRequest {
    /// Required. The resource name of the project associated with the
    /// \[attestors][google.cloud.binaryauthorization.v1.Attestor\], in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. The server may return fewer results than requested. If
    /// unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return. Typically,
    /// this is the value of \[ListAttestorsResponse.next_page_token][google.cloud.binaryauthorization.v1.ListAttestorsResponse.next_page_token\] returned
    /// from the previous call to the `ListAttestors` method.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for \[BinauthzManagementService.ListAttestors][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAttestorsResponse {
    /// The list of \[attestors][google.cloud.binaryauthorization.v1.Attestor\].
    #[prost(message, repeated, tag = "1")]
    pub attestors: ::prost::alloc::vec::Vec<Attestor>,
    /// A token to retrieve the next page of results. Pass this value in the
    /// \[ListAttestorsRequest.page_token][google.cloud.binaryauthorization.v1.ListAttestorsRequest.page_token\] field in the subsequent call to the
    /// `ListAttestors` method to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for \[BinauthzManagementService.DeleteAttestor][\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAttestorRequest {
    /// Required. The name of the \[attestors][google.cloud.binaryauthorization.v1.Attestor\] to delete, in the format
    /// `projects/*/attestors/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to read the current system policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSystemPolicyRequest {
    /// Required. The resource name, in the format `locations/*/policy`.
    /// Note that the system policy is not associated with a project.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[ValidationHelperV1.ValidateAttestationOccurrence][google.cloud.binaryauthorization.v1.ValidationHelperV1.ValidateAttestationOccurrence\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateAttestationOccurrenceRequest {
    /// Required. The resource name of the \[Attestor][google.cloud.binaryauthorization.v1.Attestor\] of the
    /// \[occurrence][grafeas.v1.Occurrence\], in the format
    /// `projects/*/attestors/*`.
    #[prost(string, tag = "1")]
    pub attestor: ::prost::alloc::string::String,
    /// Required. An \[AttestationOccurrence][grafeas.v1.AttestationOccurrence\] to
    /// be checked that it can be verified by the Attestor. It does not have to be
    /// an existing entity in Container Analysis. It must otherwise be a valid
    /// AttestationOccurrence.
    #[prost(message, optional, tag = "2")]
    pub attestation:
        ::core::option::Option<super::super::super::super::grafeas::v1::AttestationOccurrence>,
    /// Required. The resource name of the \[Note][grafeas.v1.Note\] to which the
    /// containing \[Occurrence][grafeas.v1.Occurrence\] is associated.
    #[prost(string, tag = "3")]
    pub occurrence_note: ::prost::alloc::string::String,
    /// Required. The URI of the artifact (e.g. container image) that is the
    /// subject of the containing \[Occurrence][grafeas.v1.Occurrence\].
    #[prost(string, tag = "4")]
    pub occurrence_resource_uri: ::prost::alloc::string::String,
}
/// Response message for
/// \[ValidationHelperV1.ValidateAttestationOccurrence][google.cloud.binaryauthorization.v1.ValidationHelperV1.ValidateAttestationOccurrence\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateAttestationOccurrenceResponse {
    /// The result of the Attestation validation.
    #[prost(
        enumeration = "validate_attestation_occurrence_response::Result",
        tag = "1"
    )]
    pub result: i32,
    /// The reason for denial if the Attestation couldn't be validated.
    #[prost(string, tag = "2")]
    pub denial_reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ValidateAttestationOccurrenceResponse`.
pub mod validate_attestation_occurrence_response {
    /// The enum returned in the "result" field.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// Unspecified.
        Unspecified = 0,
        /// The Attestation was able to verified by the Attestor.
        Verified = 1,
        /// The Attestation was not able to verified by the Attestor.
        AttestationNotVerifiable = 2,
    }
}
#[doc = r" Generated client implementations."]
pub mod binauthz_management_service_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Google Cloud Management Service for Binary Authorization admission policies"]
    #[doc = " and attestation authorities."]
    #[doc = ""]
    #[doc = " This API implements a REST model with the following objects:"]
    #[doc = ""]
    #[doc = " * [Policy][google.cloud.binaryauthorization.v1.Policy]"]
    #[doc = " * [Attestor][google.cloud.binaryauthorization.v1.Attestor]"]
    #[derive(Debug, Clone)]
    pub struct BinauthzManagementServiceV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BinauthzManagementServiceV1Client<T>
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
        ) -> BinauthzManagementServiceV1Client<InterceptedService<T, F>>
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
            BinauthzManagementServiceV1Client::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " A [policy][google.cloud.binaryauthorization.v1.Policy] specifies the [attestors][google.cloud.binaryauthorization.v1.Attestor] that must attest to"]
        #[doc = " a container image, before the project is allowed to deploy that"]
        #[doc = " image. There is at most one policy per project. All image admission"]
        #[doc = " requests are permitted if a project has no policy."]
        #[doc = ""]
        #[doc = " Gets the [policy][google.cloud.binaryauthorization.v1.Policy] for this project. Returns a default"]
        #[doc = " [policy][google.cloud.binaryauthorization.v1.Policy] if the project does not have one."]
        pub async fn get_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPolicyRequest>,
        ) -> Result<tonic::Response<super::Policy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/GetPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or updates a project's [policy][google.cloud.binaryauthorization.v1.Policy], and returns a copy of the"]
        #[doc = " new [policy][google.cloud.binaryauthorization.v1.Policy]. A policy is always updated as a whole, to avoid race"]
        #[doc = " conditions with concurrent policy enforcement (or management!)"]
        #[doc = " requests. Returns NOT_FOUND if the project does not exist, INVALID_ARGUMENT"]
        #[doc = " if the request is malformed."]
        pub async fn update_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePolicyRequest>,
        ) -> Result<tonic::Response<super::Policy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/UpdatePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an [attestor][google.cloud.binaryauthorization.v1.Attestor], and returns a copy of the new"]
        #[doc = " [attestor][google.cloud.binaryauthorization.v1.Attestor]. Returns NOT_FOUND if the project does not exist,"]
        #[doc = " INVALID_ARGUMENT if the request is malformed, ALREADY_EXISTS if the"]
        #[doc = " [attestor][google.cloud.binaryauthorization.v1.Attestor] already exists."]
        pub async fn create_attestor(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAttestorRequest>,
        ) -> Result<tonic::Response<super::Attestor>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/CreateAttestor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets an [attestor][google.cloud.binaryauthorization.v1.Attestor]."]
        #[doc = " Returns NOT_FOUND if the [attestor][google.cloud.binaryauthorization.v1.Attestor] does not exist."]
        pub async fn get_attestor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAttestorRequest>,
        ) -> Result<tonic::Response<super::Attestor>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/GetAttestor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an [attestor][google.cloud.binaryauthorization.v1.Attestor]."]
        #[doc = " Returns NOT_FOUND if the [attestor][google.cloud.binaryauthorization.v1.Attestor] does not exist."]
        pub async fn update_attestor(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAttestorRequest>,
        ) -> Result<tonic::Response<super::Attestor>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/UpdateAttestor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists [attestors][google.cloud.binaryauthorization.v1.Attestor]."]
        #[doc = " Returns INVALID_ARGUMENT if the project does not exist."]
        pub async fn list_attestors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAttestorsRequest>,
        ) -> Result<tonic::Response<super::ListAttestorsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/ListAttestors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an [attestor][google.cloud.binaryauthorization.v1.Attestor]. Returns NOT_FOUND if the"]
        #[doc = " [attestor][google.cloud.binaryauthorization.v1.Attestor] does not exist."]
        pub async fn delete_attestor(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAttestorRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.binaryauthorization.v1.BinauthzManagementServiceV1/DeleteAttestor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod system_policy_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " API for working with the system policy."]
    #[derive(Debug, Clone)]
    pub struct SystemPolicyV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SystemPolicyV1Client<T>
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
        ) -> SystemPolicyV1Client<InterceptedService<T, F>>
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
            SystemPolicyV1Client::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Gets the current system policy in the specified location."]
        pub async fn get_system_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSystemPolicyRequest>,
        ) -> Result<tonic::Response<super::Policy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.binaryauthorization.v1.SystemPolicyV1/GetSystemPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod validation_helper_v1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " BinAuthz Attestor verification"]
    #[derive(Debug, Clone)]
    pub struct ValidationHelperV1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ValidationHelperV1Client<T>
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
        ) -> ValidationHelperV1Client<InterceptedService<T, F>>
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
            ValidationHelperV1Client::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns whether the given Attestation for the given image URI"]
        #[doc = " was signed by the given Attestor"]
        pub async fn validate_attestation_occurrence(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateAttestationOccurrenceRequest>,
        ) -> Result<tonic::Response<super::ValidateAttestationOccurrenceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.binaryauthorization.v1.ValidationHelperV1/ValidateAttestationOccurrence") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
