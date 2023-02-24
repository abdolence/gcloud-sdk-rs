/// A \[KeyRing][google.cloud.kms.v1.KeyRing\] is a toplevel logical grouping of
/// \[CryptoKeys][google.cloud.kms.v1.CryptoKey\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRing {
    /// Output only. The resource name for the
    /// \[KeyRing][google.cloud.kms.v1.KeyRing\] in the format
    /// `projects/*/locations/*/keyRings/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time at which this \[KeyRing][google.cloud.kms.v1.KeyRing\]
    /// was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A \[CryptoKey][google.cloud.kms.v1.CryptoKey\] represents a logical key that
/// can be used for cryptographic operations.
///
/// A \[CryptoKey][google.cloud.kms.v1.CryptoKey\] is made up of zero or more
/// \[versions][google.cloud.kms.v1.CryptoKeyVersion\], which represent the actual
/// key material used in cryptographic operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKey {
    /// Output only. The resource name for this
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] in the format
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. A copy of the "primary"
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] that will be used
    /// by \[Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt\] when this
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] is given in
    /// \[EncryptRequest.name][google.cloud.kms.v1.EncryptRequest.name\].
    ///
    /// The \[CryptoKey][google.cloud.kms.v1.CryptoKey\]'s primary version can be
    /// updated via
    /// \[UpdateCryptoKeyPrimaryVersion][google.cloud.kms.v1.KeyManagementService.UpdateCryptoKeyPrimaryVersion\].
    ///
    /// Keys with \[purpose][google.cloud.kms.v1.CryptoKey.purpose\]
    /// \[ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT\]
    /// may have a primary. For other keys, this field will be omitted.
    #[prost(message, optional, tag = "2")]
    pub primary: ::core::option::Option<CryptoKeyVersion>,
    /// Immutable. The immutable purpose of this
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\].
    #[prost(enumeration = "crypto_key::CryptoKeyPurpose", tag = "3")]
    pub purpose: i32,
    /// Output only. The time at which this
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// At \[next_rotation_time][google.cloud.kms.v1.CryptoKey.next_rotation_time\],
    /// the Key Management Service will automatically:
    ///
    /// 1. Create a new version of this \[CryptoKey][google.cloud.kms.v1.CryptoKey\].
    /// 2. Mark the new version as primary.
    ///
    /// Key rotations performed manually via
    /// \[CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion\]
    /// and
    /// \[UpdateCryptoKeyPrimaryVersion][google.cloud.kms.v1.KeyManagementService.UpdateCryptoKeyPrimaryVersion\]
    /// do not affect
    /// \[next_rotation_time][google.cloud.kms.v1.CryptoKey.next_rotation_time\].
    ///
    /// Keys with \[purpose][google.cloud.kms.v1.CryptoKey.purpose\]
    /// \[ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT\]
    /// support automatic rotation. For other keys, this field must be omitted.
    #[prost(message, optional, tag = "7")]
    pub next_rotation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A template describing settings for new
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] instances. The
    /// properties of new \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]
    /// instances created by either
    /// \[CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion\]
    /// or auto-rotation are controlled by this template.
    #[prost(message, optional, tag = "11")]
    pub version_template: ::core::option::Option<CryptoKeyVersionTemplate>,
    /// Labels with user-defined metadata. For more information, see
    /// [Labeling Keys](<https://cloud.google.com/kms/docs/labeling-keys>).
    #[prost(map = "string, string", tag = "10")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Immutable. Whether this key may contain imported versions only.
    #[prost(bool, tag = "13")]
    pub import_only: bool,
    /// Immutable. The period of time that versions of this key spend in the
    /// \[DESTROY_SCHEDULED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROY_SCHEDULED\]
    /// state before transitioning to
    /// \[DESTROYED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROYED\].
    /// If not specified at creation time, the default duration is 24 hours.
    #[prost(message, optional, tag = "14")]
    pub destroy_scheduled_duration: ::core::option::Option<::prost_types::Duration>,
    /// Immutable. The resource name of the backend environment where the key
    /// material for all \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\]
    /// associated with this \[CryptoKey][google.cloud.kms.v1.CryptoKey\] reside and
    /// where all related cryptographic operations are performed. Only applicable
    /// if \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\] have a
    /// \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] of
    /// \[EXTERNAL_VPC][CryptoKeyVersion.ProtectionLevel.EXTERNAL_VPC\], with the
    /// resource name in the format `projects/*/locations/*/ekmConnections/*`.
    /// Note, this list is non-exhaustive and may apply to additional
    /// \[ProtectionLevels][google.cloud.kms.v1.ProtectionLevel\] in the future.
    #[prost(string, tag = "15")]
    pub crypto_key_backend: ::prost::alloc::string::String,
    /// Controls the rate of automatic rotation.
    #[prost(oneof = "crypto_key::RotationSchedule", tags = "8")]
    pub rotation_schedule: ::core::option::Option<crypto_key::RotationSchedule>,
}
/// Nested message and enum types in `CryptoKey`.
pub mod crypto_key {
    /// \[CryptoKeyPurpose][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose\]
    /// describes the cryptographic capabilities of a
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\]. A given key can only be used
    /// for the operations allowed by its purpose. For more information, see [Key
    /// purposes](<https://cloud.google.com/kms/docs/algorithms#key_purposes>).
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CryptoKeyPurpose {
        /// Not specified.
        Unspecified = 0,
        /// \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] with this purpose may be used
        /// with \[Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt\] and
        /// \[Decrypt][google.cloud.kms.v1.KeyManagementService.Decrypt\].
        EncryptDecrypt = 1,
        /// \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] with this purpose may be used
        /// with
        /// \[AsymmetricSign][google.cloud.kms.v1.KeyManagementService.AsymmetricSign\]
        /// and
        /// \[GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey\].
        AsymmetricSign = 5,
        /// \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] with this purpose may be used
        /// with
        /// \[AsymmetricDecrypt][google.cloud.kms.v1.KeyManagementService.AsymmetricDecrypt\]
        /// and
        /// \[GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey\].
        AsymmetricDecrypt = 6,
        /// \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] with this purpose may be used
        /// with \[MacSign][google.cloud.kms.v1.KeyManagementService.MacSign\].
        Mac = 9,
    }
    impl CryptoKeyPurpose {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CryptoKeyPurpose::Unspecified => "CRYPTO_KEY_PURPOSE_UNSPECIFIED",
                CryptoKeyPurpose::EncryptDecrypt => "ENCRYPT_DECRYPT",
                CryptoKeyPurpose::AsymmetricSign => "ASYMMETRIC_SIGN",
                CryptoKeyPurpose::AsymmetricDecrypt => "ASYMMETRIC_DECRYPT",
                CryptoKeyPurpose::Mac => "MAC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CRYPTO_KEY_PURPOSE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENCRYPT_DECRYPT" => Some(Self::EncryptDecrypt),
                "ASYMMETRIC_SIGN" => Some(Self::AsymmetricSign),
                "ASYMMETRIC_DECRYPT" => Some(Self::AsymmetricDecrypt),
                "MAC" => Some(Self::Mac),
                _ => None,
            }
        }
    }
    /// Controls the rate of automatic rotation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RotationSchedule {
        /// \[next_rotation_time][google.cloud.kms.v1.CryptoKey.next_rotation_time\]
        /// will be advanced by this period when the service automatically rotates a
        /// key. Must be at least 24 hours and at most 876,000 hours.
        ///
        /// If \[rotation_period][google.cloud.kms.v1.CryptoKey.rotation_period\] is
        /// set,
        /// \[next_rotation_time][google.cloud.kms.v1.CryptoKey.next_rotation_time\]
        /// must also be set.
        ///
        /// Keys with \[purpose][google.cloud.kms.v1.CryptoKey.purpose\]
        /// \[ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT\]
        /// support automatic rotation. For other keys, this field must be omitted.
        #[prost(message, tag = "8")]
        RotationPeriod(::prost_types::Duration),
    }
}
/// A \[CryptoKeyVersionTemplate][google.cloud.kms.v1.CryptoKeyVersionTemplate\]
/// specifies the properties to use when creating a new
/// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\], either manually
/// with
/// \[CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion\]
/// or automatically as a result of auto-rotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKeyVersionTemplate {
    /// \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] to use when creating
    /// a \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] based on this
    /// template. Immutable. Defaults to
    /// \[SOFTWARE][google.cloud.kms.v1.ProtectionLevel.SOFTWARE\].
    #[prost(enumeration = "ProtectionLevel", tag = "1")]
    pub protection_level: i32,
    /// Required.
    /// \[Algorithm][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm\]
    /// to use when creating a
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] based on this
    /// template.
    ///
    /// For backwards compatibility, GOOGLE_SYMMETRIC_ENCRYPTION is implied if both
    /// this field is omitted and
    /// \[CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose\] is
    /// \[ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT\].
    #[prost(enumeration = "crypto_key_version::CryptoKeyVersionAlgorithm", tag = "3")]
    pub algorithm: i32,
}
/// Contains an HSM-generated attestation about a key operation. For more
/// information, see [Verifying attestations]
/// (<https://cloud.google.com/kms/docs/attest-key>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyOperationAttestation {
    /// Output only. The format of the attestation data.
    #[prost(enumeration = "key_operation_attestation::AttestationFormat", tag = "4")]
    pub format: i32,
    /// Output only. The attestation data provided by the HSM when the key
    /// operation was performed.
    #[prost(bytes = "vec", tag = "5")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    /// Output only. The certificate chains needed to validate the attestation
    #[prost(message, optional, tag = "6")]
    pub cert_chains: ::core::option::Option<
        key_operation_attestation::CertificateChains,
    >,
}
/// Nested message and enum types in `KeyOperationAttestation`.
pub mod key_operation_attestation {
    /// Certificate chains needed to verify the attestation.
    /// Certificates in chains are PEM-encoded and are ordered based on
    /// <https://tools.ietf.org/html/rfc5246#section-7.4.2.>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertificateChains {
        /// Cavium certificate chain corresponding to the attestation.
        #[prost(string, repeated, tag = "1")]
        pub cavium_certs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Google card certificate chain corresponding to the attestation.
        #[prost(string, repeated, tag = "2")]
        pub google_card_certs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Google partition certificate chain corresponding to the attestation.
        #[prost(string, repeated, tag = "3")]
        pub google_partition_certs: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
    }
    /// Attestation formats provided by the HSM.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum AttestationFormat {
        /// Not specified.
        Unspecified = 0,
        /// Cavium HSM attestation compressed with gzip. Note that this format is
        /// defined by Cavium and subject to change at any time.
        ///
        /// See
        /// <https://www.marvell.com/products/security-solutions/nitrox-hs-adapters/software-key-attestation.html.>
        CaviumV1Compressed = 3,
        /// Cavium HSM attestation V2 compressed with gzip. This is a new format
        /// introduced in Cavium's version 3.2-08.
        CaviumV2Compressed = 4,
    }
    impl AttestationFormat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AttestationFormat::Unspecified => "ATTESTATION_FORMAT_UNSPECIFIED",
                AttestationFormat::CaviumV1Compressed => "CAVIUM_V1_COMPRESSED",
                AttestationFormat::CaviumV2Compressed => "CAVIUM_V2_COMPRESSED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ATTESTATION_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
                "CAVIUM_V1_COMPRESSED" => Some(Self::CaviumV1Compressed),
                "CAVIUM_V2_COMPRESSED" => Some(Self::CaviumV2Compressed),
                _ => None,
            }
        }
    }
}
/// A \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] represents an
/// individual cryptographic key, and the associated key material.
///
/// An
/// \[ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED\]
/// version can be used for cryptographic operations.
///
/// For security reasons, the raw cryptographic key material represented by a
/// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] can never be viewed
/// or exported. It can only be used to encrypt, decrypt, or sign data when an
/// authorized user or application invokes Cloud KMS.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKeyVersion {
    /// Output only. The resource name for this
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] in the format
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The current state of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\].
    #[prost(enumeration = "crypto_key_version::CryptoKeyVersionState", tag = "3")]
    pub state: i32,
    /// Output only. The \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\]
    /// describing how crypto operations are performed with this
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\].
    #[prost(enumeration = "ProtectionLevel", tag = "7")]
    pub protection_level: i32,
    /// Output only. The
    /// \[CryptoKeyVersionAlgorithm][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm\]
    /// that this \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]
    /// supports.
    #[prost(enumeration = "crypto_key_version::CryptoKeyVersionAlgorithm", tag = "10")]
    pub algorithm: i32,
    /// Output only. Statement that was generated and signed by the HSM at key
    /// creation time. Use this statement to verify attributes of the key as stored
    /// on the HSM, independently of Google. Only provided for key versions with
    /// \[protection_level][google.cloud.kms.v1.CryptoKeyVersion.protection_level\]
    /// \[HSM][google.cloud.kms.v1.ProtectionLevel.HSM\].
    #[prost(message, optional, tag = "8")]
    pub attestation: ::core::option::Option<KeyOperationAttestation>,
    /// Output only. The time at which this
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]'s key material was
    /// generated.
    #[prost(message, optional, tag = "11")]
    pub generate_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]'s key material is
    /// scheduled for destruction. Only present if
    /// \[state][google.cloud.kms.v1.CryptoKeyVersion.state\] is
    /// \[DESTROY_SCHEDULED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROY_SCHEDULED\].
    #[prost(message, optional, tag = "5")]
    pub destroy_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this CryptoKeyVersion's key material was
    /// destroyed. Only present if
    /// \[state][google.cloud.kms.v1.CryptoKeyVersion.state\] is
    /// \[DESTROYED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROYED\].
    #[prost(message, optional, tag = "6")]
    pub destroy_event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The name of the \[ImportJob][google.cloud.kms.v1.ImportJob\]
    /// used in the most recent import of this
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]. Only present if
    /// the underlying key material was imported.
    #[prost(string, tag = "14")]
    pub import_job: ::prost::alloc::string::String,
    /// Output only. The time at which this
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]'s key material was
    /// most recently imported.
    #[prost(message, optional, tag = "15")]
    pub import_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The root cause of the most recent import failure. Only present
    /// if \[state][google.cloud.kms.v1.CryptoKeyVersion.state\] is
    /// \[IMPORT_FAILED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.IMPORT_FAILED\].
    #[prost(string, tag = "16")]
    pub import_failure_reason: ::prost::alloc::string::String,
    /// ExternalProtectionLevelOptions stores a group of additional fields for
    /// configuring a \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] that
    /// are specific to the
    /// \[EXTERNAL][google.cloud.kms.v1.ProtectionLevel.EXTERNAL\] protection level
    /// and \[EXTERNAL_VPC][google.cloud.kms.v1.ProtectionLevel.EXTERNAL_VPC\]
    /// protection levels.
    #[prost(message, optional, tag = "17")]
    pub external_protection_level_options: ::core::option::Option<
        ExternalProtectionLevelOptions,
    >,
    /// Output only. Whether or not this key version is eligible for reimport, by
    /// being specified as a target in
    /// \[ImportCryptoKeyVersionRequest.crypto_key_version][google.cloud.kms.v1.ImportCryptoKeyVersionRequest.crypto_key_version\].
    #[prost(bool, tag = "18")]
    pub reimport_eligible: bool,
}
/// Nested message and enum types in `CryptoKeyVersion`.
pub mod crypto_key_version {
    /// The algorithm of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\], indicating what
    /// parameters must be used for each cryptographic operation.
    ///
    /// The
    /// \[GOOGLE_SYMMETRIC_ENCRYPTION][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm.GOOGLE_SYMMETRIC_ENCRYPTION\]
    /// algorithm is usable with
    /// \[CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose\]
    /// \[ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT\].
    ///
    /// Algorithms beginning with "RSA_SIGN_" are usable with
    /// \[CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose\]
    /// \[ASYMMETRIC_SIGN][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_SIGN\].
    ///
    /// The fields in the name after "RSA_SIGN_" correspond to the following
    /// parameters: padding algorithm, modulus bit length, and digest algorithm.
    ///
    /// For PSS, the salt length used is equal to the length of digest
    /// algorithm. For example,
    /// \[RSA_SIGN_PSS_2048_SHA256][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm.RSA_SIGN_PSS_2048_SHA256\]
    /// will use PSS with a salt length of 256 bits or 32 bytes.
    ///
    /// Algorithms beginning with "RSA_DECRYPT_" are usable with
    /// \[CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose\]
    /// \[ASYMMETRIC_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_DECRYPT\].
    ///
    /// The fields in the name after "RSA_DECRYPT_" correspond to the following
    /// parameters: padding algorithm, modulus bit length, and digest algorithm.
    ///
    /// Algorithms beginning with "EC_SIGN_" are usable with
    /// \[CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose\]
    /// \[ASYMMETRIC_SIGN][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_SIGN\].
    ///
    /// The fields in the name after "EC_SIGN_" correspond to the following
    /// parameters: elliptic curve, digest algorithm.
    ///
    /// Algorithms beginning with "HMAC_" are usable with
    /// \[CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose\]
    /// \[MAC][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.MAC\].
    ///
    /// The suffix following "HMAC_" corresponds to the hash algorithm being used
    /// (eg. SHA256).
    ///
    /// For more information, see [Key purposes and algorithms]
    /// (<https://cloud.google.com/kms/docs/algorithms>).
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CryptoKeyVersionAlgorithm {
        /// Not specified.
        Unspecified = 0,
        /// Creates symmetric encryption keys.
        GoogleSymmetricEncryption = 1,
        /// RSASSA-PSS 2048 bit key with a SHA256 digest.
        RsaSignPss2048Sha256 = 2,
        /// RSASSA-PSS 3072 bit key with a SHA256 digest.
        RsaSignPss3072Sha256 = 3,
        /// RSASSA-PSS 4096 bit key with a SHA256 digest.
        RsaSignPss4096Sha256 = 4,
        /// RSASSA-PSS 4096 bit key with a SHA512 digest.
        RsaSignPss4096Sha512 = 15,
        /// RSASSA-PKCS1-v1_5 with a 2048 bit key and a SHA256 digest.
        RsaSignPkcs12048Sha256 = 5,
        /// RSASSA-PKCS1-v1_5 with a 3072 bit key and a SHA256 digest.
        RsaSignPkcs13072Sha256 = 6,
        /// RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA256 digest.
        RsaSignPkcs14096Sha256 = 7,
        /// RSASSA-PKCS1-v1_5 with a 4096 bit key and a SHA512 digest.
        RsaSignPkcs14096Sha512 = 16,
        /// RSASSA-PKCS1-v1_5 signing without encoding, with a 2048 bit key.
        RsaSignRawPkcs12048 = 28,
        /// RSASSA-PKCS1-v1_5 signing without encoding, with a 3072 bit key.
        RsaSignRawPkcs13072 = 29,
        /// RSASSA-PKCS1-v1_5 signing without encoding, with a 4096 bit key.
        RsaSignRawPkcs14096 = 30,
        /// RSAES-OAEP 2048 bit key with a SHA256 digest.
        RsaDecryptOaep2048Sha256 = 8,
        /// RSAES-OAEP 3072 bit key with a SHA256 digest.
        RsaDecryptOaep3072Sha256 = 9,
        /// RSAES-OAEP 4096 bit key with a SHA256 digest.
        RsaDecryptOaep4096Sha256 = 10,
        /// RSAES-OAEP 4096 bit key with a SHA512 digest.
        RsaDecryptOaep4096Sha512 = 17,
        /// RSAES-OAEP 2048 bit key with a SHA1 digest.
        RsaDecryptOaep2048Sha1 = 37,
        /// RSAES-OAEP 3072 bit key with a SHA1 digest.
        RsaDecryptOaep3072Sha1 = 38,
        /// RSAES-OAEP 4096 bit key with a SHA1 digest.
        RsaDecryptOaep4096Sha1 = 39,
        /// ECDSA on the NIST P-256 curve with a SHA256 digest.
        EcSignP256Sha256 = 12,
        /// ECDSA on the NIST P-384 curve with a SHA384 digest.
        EcSignP384Sha384 = 13,
        /// ECDSA on the non-NIST secp256k1 curve. This curve is only supported for
        /// HSM protection level.
        EcSignSecp256k1Sha256 = 31,
        /// HMAC-SHA256 signing with a 256 bit key.
        HmacSha256 = 32,
        /// HMAC-SHA1 signing with a 160 bit key.
        HmacSha1 = 33,
        /// HMAC-SHA384 signing with a 384 bit key.
        HmacSha384 = 34,
        /// HMAC-SHA512 signing with a 512 bit key.
        HmacSha512 = 35,
        /// HMAC-SHA224 signing with a 224 bit key.
        HmacSha224 = 36,
        /// Algorithm representing symmetric encryption by an external key manager.
        ExternalSymmetricEncryption = 18,
    }
    impl CryptoKeyVersionAlgorithm {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CryptoKeyVersionAlgorithm::Unspecified => {
                    "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED"
                }
                CryptoKeyVersionAlgorithm::GoogleSymmetricEncryption => {
                    "GOOGLE_SYMMETRIC_ENCRYPTION"
                }
                CryptoKeyVersionAlgorithm::RsaSignPss2048Sha256 => {
                    "RSA_SIGN_PSS_2048_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaSignPss3072Sha256 => {
                    "RSA_SIGN_PSS_3072_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaSignPss4096Sha256 => {
                    "RSA_SIGN_PSS_4096_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaSignPss4096Sha512 => {
                    "RSA_SIGN_PSS_4096_SHA512"
                }
                CryptoKeyVersionAlgorithm::RsaSignPkcs12048Sha256 => {
                    "RSA_SIGN_PKCS1_2048_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaSignPkcs13072Sha256 => {
                    "RSA_SIGN_PKCS1_3072_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha256 => {
                    "RSA_SIGN_PKCS1_4096_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaSignPkcs14096Sha512 => {
                    "RSA_SIGN_PKCS1_4096_SHA512"
                }
                CryptoKeyVersionAlgorithm::RsaSignRawPkcs12048 => {
                    "RSA_SIGN_RAW_PKCS1_2048"
                }
                CryptoKeyVersionAlgorithm::RsaSignRawPkcs13072 => {
                    "RSA_SIGN_RAW_PKCS1_3072"
                }
                CryptoKeyVersionAlgorithm::RsaSignRawPkcs14096 => {
                    "RSA_SIGN_RAW_PKCS1_4096"
                }
                CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha256 => {
                    "RSA_DECRYPT_OAEP_2048_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha256 => {
                    "RSA_DECRYPT_OAEP_3072_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha256 => {
                    "RSA_DECRYPT_OAEP_4096_SHA256"
                }
                CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha512 => {
                    "RSA_DECRYPT_OAEP_4096_SHA512"
                }
                CryptoKeyVersionAlgorithm::RsaDecryptOaep2048Sha1 => {
                    "RSA_DECRYPT_OAEP_2048_SHA1"
                }
                CryptoKeyVersionAlgorithm::RsaDecryptOaep3072Sha1 => {
                    "RSA_DECRYPT_OAEP_3072_SHA1"
                }
                CryptoKeyVersionAlgorithm::RsaDecryptOaep4096Sha1 => {
                    "RSA_DECRYPT_OAEP_4096_SHA1"
                }
                CryptoKeyVersionAlgorithm::EcSignP256Sha256 => "EC_SIGN_P256_SHA256",
                CryptoKeyVersionAlgorithm::EcSignP384Sha384 => "EC_SIGN_P384_SHA384",
                CryptoKeyVersionAlgorithm::EcSignSecp256k1Sha256 => {
                    "EC_SIGN_SECP256K1_SHA256"
                }
                CryptoKeyVersionAlgorithm::HmacSha256 => "HMAC_SHA256",
                CryptoKeyVersionAlgorithm::HmacSha1 => "HMAC_SHA1",
                CryptoKeyVersionAlgorithm::HmacSha384 => "HMAC_SHA384",
                CryptoKeyVersionAlgorithm::HmacSha512 => "HMAC_SHA512",
                CryptoKeyVersionAlgorithm::HmacSha224 => "HMAC_SHA224",
                CryptoKeyVersionAlgorithm::ExternalSymmetricEncryption => {
                    "EXTERNAL_SYMMETRIC_ENCRYPTION"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CRYPTO_KEY_VERSION_ALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
                "GOOGLE_SYMMETRIC_ENCRYPTION" => Some(Self::GoogleSymmetricEncryption),
                "RSA_SIGN_PSS_2048_SHA256" => Some(Self::RsaSignPss2048Sha256),
                "RSA_SIGN_PSS_3072_SHA256" => Some(Self::RsaSignPss3072Sha256),
                "RSA_SIGN_PSS_4096_SHA256" => Some(Self::RsaSignPss4096Sha256),
                "RSA_SIGN_PSS_4096_SHA512" => Some(Self::RsaSignPss4096Sha512),
                "RSA_SIGN_PKCS1_2048_SHA256" => Some(Self::RsaSignPkcs12048Sha256),
                "RSA_SIGN_PKCS1_3072_SHA256" => Some(Self::RsaSignPkcs13072Sha256),
                "RSA_SIGN_PKCS1_4096_SHA256" => Some(Self::RsaSignPkcs14096Sha256),
                "RSA_SIGN_PKCS1_4096_SHA512" => Some(Self::RsaSignPkcs14096Sha512),
                "RSA_SIGN_RAW_PKCS1_2048" => Some(Self::RsaSignRawPkcs12048),
                "RSA_SIGN_RAW_PKCS1_3072" => Some(Self::RsaSignRawPkcs13072),
                "RSA_SIGN_RAW_PKCS1_4096" => Some(Self::RsaSignRawPkcs14096),
                "RSA_DECRYPT_OAEP_2048_SHA256" => Some(Self::RsaDecryptOaep2048Sha256),
                "RSA_DECRYPT_OAEP_3072_SHA256" => Some(Self::RsaDecryptOaep3072Sha256),
                "RSA_DECRYPT_OAEP_4096_SHA256" => Some(Self::RsaDecryptOaep4096Sha256),
                "RSA_DECRYPT_OAEP_4096_SHA512" => Some(Self::RsaDecryptOaep4096Sha512),
                "RSA_DECRYPT_OAEP_2048_SHA1" => Some(Self::RsaDecryptOaep2048Sha1),
                "RSA_DECRYPT_OAEP_3072_SHA1" => Some(Self::RsaDecryptOaep3072Sha1),
                "RSA_DECRYPT_OAEP_4096_SHA1" => Some(Self::RsaDecryptOaep4096Sha1),
                "EC_SIGN_P256_SHA256" => Some(Self::EcSignP256Sha256),
                "EC_SIGN_P384_SHA384" => Some(Self::EcSignP384Sha384),
                "EC_SIGN_SECP256K1_SHA256" => Some(Self::EcSignSecp256k1Sha256),
                "HMAC_SHA256" => Some(Self::HmacSha256),
                "HMAC_SHA1" => Some(Self::HmacSha1),
                "HMAC_SHA384" => Some(Self::HmacSha384),
                "HMAC_SHA512" => Some(Self::HmacSha512),
                "HMAC_SHA224" => Some(Self::HmacSha224),
                "EXTERNAL_SYMMETRIC_ENCRYPTION" => {
                    Some(Self::ExternalSymmetricEncryption)
                }
                _ => None,
            }
        }
    }
    /// The state of a \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\],
    /// indicating if it can be used.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CryptoKeyVersionState {
        /// Not specified.
        Unspecified = 0,
        /// This version is still being generated. It may not be used, enabled,
        /// disabled, or destroyed yet. Cloud KMS will automatically mark this
        /// version
        /// \[ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED\]
        /// as soon as the version is ready.
        PendingGeneration = 5,
        /// This version may be used for cryptographic operations.
        Enabled = 1,
        /// This version may not be used, but the key material is still available,
        /// and the version can be placed back into the
        /// \[ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED\]
        /// state.
        Disabled = 2,
        /// This version is destroyed, and the key material is no longer stored.
        /// This version may only become
        /// \[ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED\]
        /// again if this version is
        /// \[reimport_eligible][google.cloud.kms.v1.CryptoKeyVersion.reimport_eligible\]
        /// and the original key material is reimported with a call to
        /// \[KeyManagementService.ImportCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.ImportCryptoKeyVersion\].
        Destroyed = 3,
        /// This version is scheduled for destruction, and will be destroyed soon.
        /// Call
        /// \[RestoreCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.RestoreCryptoKeyVersion\]
        /// to put it back into the
        /// \[DISABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DISABLED\]
        /// state.
        DestroyScheduled = 4,
        /// This version is still being imported. It may not be used, enabled,
        /// disabled, or destroyed yet. Cloud KMS will automatically mark this
        /// version
        /// \[ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED\]
        /// as soon as the version is ready.
        PendingImport = 6,
        /// This version was not imported successfully. It may not be used, enabled,
        /// disabled, or destroyed. The submitted key material has been discarded.
        /// Additional details can be found in
        /// \[CryptoKeyVersion.import_failure_reason][google.cloud.kms.v1.CryptoKeyVersion.import_failure_reason\].
        ImportFailed = 7,
    }
    impl CryptoKeyVersionState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CryptoKeyVersionState::Unspecified => {
                    "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED"
                }
                CryptoKeyVersionState::PendingGeneration => "PENDING_GENERATION",
                CryptoKeyVersionState::Enabled => "ENABLED",
                CryptoKeyVersionState::Disabled => "DISABLED",
                CryptoKeyVersionState::Destroyed => "DESTROYED",
                CryptoKeyVersionState::DestroyScheduled => "DESTROY_SCHEDULED",
                CryptoKeyVersionState::PendingImport => "PENDING_IMPORT",
                CryptoKeyVersionState::ImportFailed => "IMPORT_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CRYPTO_KEY_VERSION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING_GENERATION" => Some(Self::PendingGeneration),
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                "DESTROYED" => Some(Self::Destroyed),
                "DESTROY_SCHEDULED" => Some(Self::DestroyScheduled),
                "PENDING_IMPORT" => Some(Self::PendingImport),
                "IMPORT_FAILED" => Some(Self::ImportFailed),
                _ => None,
            }
        }
    }
    /// A view for \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]s.
    /// Controls the level of detail returned for
    /// \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\] in
    /// \[KeyManagementService.ListCryptoKeyVersions][google.cloud.kms.v1.KeyManagementService.ListCryptoKeyVersions\]
    /// and
    /// \[KeyManagementService.ListCryptoKeys][google.cloud.kms.v1.KeyManagementService.ListCryptoKeys\].
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CryptoKeyVersionView {
        /// Default view for each
        /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]. Does not
        /// include the
        /// \[attestation][google.cloud.kms.v1.CryptoKeyVersion.attestation\] field.
        Unspecified = 0,
        /// Provides all fields in each
        /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\], including the
        /// \[attestation][google.cloud.kms.v1.CryptoKeyVersion.attestation\].
        Full = 1,
    }
    impl CryptoKeyVersionView {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CryptoKeyVersionView::Unspecified => {
                    "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED"
                }
                CryptoKeyVersionView::Full => "FULL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CRYPTO_KEY_VERSION_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
                "FULL" => Some(Self::Full),
                _ => None,
            }
        }
    }
}
/// The public key for a given
/// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]. Obtained via
/// \[GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    /// The public key, encoded in PEM format. For more information, see the
    /// [RFC 7468](<https://tools.ietf.org/html/rfc7468>) sections for
    /// [General Considerations](<https://tools.ietf.org/html/rfc7468#section-2>) and
    /// [Textual Encoding of Subject Public Key Info]
    /// (<https://tools.ietf.org/html/rfc7468#section-13>).
    #[prost(string, tag = "1")]
    pub pem: ::prost::alloc::string::String,
    /// The
    /// \[Algorithm][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm\]
    /// associated with this key.
    #[prost(enumeration = "crypto_key_version::CryptoKeyVersionAlgorithm", tag = "2")]
    pub algorithm: i32,
    /// Integrity verification field. A CRC32C checksum of the returned
    /// \[PublicKey.pem][google.cloud.kms.v1.PublicKey.pem\]. An integrity check of
    /// \[PublicKey.pem][google.cloud.kms.v1.PublicKey.pem\] can be performed by
    /// computing the CRC32C checksum of
    /// \[PublicKey.pem][google.cloud.kms.v1.PublicKey.pem\] and comparing your
    /// results to this field. Discard the response in case of non-matching
    /// checksum values, and perform a limited number of retries. A persistent
    /// mismatch may indicate an issue in your computation of the CRC32C checksum.
    /// Note: This field is defined as int64 for reasons of compatibility across
    /// different languages. However, it is a non-negative integer, which will
    /// never exceed 2^32-1, and can be safely downconverted to uint32 in languages
    /// that support this type.
    ///
    /// NOTE: This field is in Beta.
    #[prost(message, optional, tag = "3")]
    pub pem_crc32c: ::core::option::Option<i64>,
    /// The \[name][google.cloud.kms.v1.CryptoKeyVersion.name\] of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] public key.
    /// Provided here for verification.
    ///
    /// NOTE: This field is in Beta.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// The \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] public key.
    #[prost(enumeration = "ProtectionLevel", tag = "5")]
    pub protection_level: i32,
}
/// An \[ImportJob][google.cloud.kms.v1.ImportJob\] can be used to create
/// \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] and
/// \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\] using pre-existing
/// key material, generated outside of Cloud KMS.
///
/// When an \[ImportJob][google.cloud.kms.v1.ImportJob\] is created, Cloud KMS will
/// generate a "wrapping key", which is a public/private key pair. You use the
/// wrapping key to encrypt (also known as wrap) the pre-existing key material to
/// protect it during the import process. The nature of the wrapping key depends
/// on the choice of
/// \[import_method][google.cloud.kms.v1.ImportJob.import_method\]. When the
/// wrapping key generation is complete, the
/// \[state][google.cloud.kms.v1.ImportJob.state\] will be set to
/// \[ACTIVE][google.cloud.kms.v1.ImportJob.ImportJobState.ACTIVE\] and the
/// \[public_key][google.cloud.kms.v1.ImportJob.public_key\] can be fetched. The
/// fetched public key can then be used to wrap your pre-existing key material.
///
/// Once the key material is wrapped, it can be imported into a new
/// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] in an existing
/// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] by calling
/// \[ImportCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.ImportCryptoKeyVersion\].
/// Multiple \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\] can be
/// imported with a single \[ImportJob][google.cloud.kms.v1.ImportJob\]. Cloud KMS
/// uses the private key portion of the wrapping key to unwrap the key material.
/// Only Cloud KMS has access to the private key.
///
/// An \[ImportJob][google.cloud.kms.v1.ImportJob\] expires 3 days after it is
/// created. Once expired, Cloud KMS will no longer be able to import or unwrap
/// any key material that was wrapped with the
/// \[ImportJob][google.cloud.kms.v1.ImportJob\]'s public key.
///
/// For more information, see
/// [Importing a key](<https://cloud.google.com/kms/docs/importing-a-key>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportJob {
    /// Output only. The resource name for this
    /// \[ImportJob][google.cloud.kms.v1.ImportJob\] in the format
    /// `projects/*/locations/*/keyRings/*/importJobs/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The wrapping method to be used for incoming key
    /// material.
    #[prost(enumeration = "import_job::ImportMethod", tag = "2")]
    pub import_method: i32,
    /// Required. Immutable. The protection level of the
    /// \[ImportJob][google.cloud.kms.v1.ImportJob\]. This must match the
    /// \[protection_level][google.cloud.kms.v1.CryptoKeyVersionTemplate.protection_level\]
    /// of the \[version_template][google.cloud.kms.v1.CryptoKey.version_template\]
    /// on the \[CryptoKey][google.cloud.kms.v1.CryptoKey\] you attempt to import
    /// into.
    #[prost(enumeration = "ProtectionLevel", tag = "9")]
    pub protection_level: i32,
    /// Output only. The time at which this
    /// \[ImportJob][google.cloud.kms.v1.ImportJob\] was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this \[ImportJob][google.cloud.kms.v1.ImportJob\]'s key
    /// material was generated.
    #[prost(message, optional, tag = "4")]
    pub generate_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this
    /// \[ImportJob][google.cloud.kms.v1.ImportJob\] is scheduled for expiration and
    /// can no longer be used to import key material.
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this \[ImportJob][google.cloud.kms.v1.ImportJob\]
    /// expired. Only present if \[state][google.cloud.kms.v1.ImportJob.state\] is
    /// \[EXPIRED][google.cloud.kms.v1.ImportJob.ImportJobState.EXPIRED\].
    #[prost(message, optional, tag = "10")]
    pub expire_event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the
    /// \[ImportJob][google.cloud.kms.v1.ImportJob\], indicating if it can be used.
    #[prost(enumeration = "import_job::ImportJobState", tag = "6")]
    pub state: i32,
    /// Output only. The public key with which to wrap key material prior to
    /// import. Only returned if \[state][google.cloud.kms.v1.ImportJob.state\] is
    /// \[ACTIVE][google.cloud.kms.v1.ImportJob.ImportJobState.ACTIVE\].
    #[prost(message, optional, tag = "7")]
    pub public_key: ::core::option::Option<import_job::WrappingPublicKey>,
    /// Output only. Statement that was generated and signed by the key creator
    /// (for example, an HSM) at key creation time. Use this statement to verify
    /// attributes of the key as stored on the HSM, independently of Google.
    /// Only present if the chosen
    /// \[ImportMethod][google.cloud.kms.v1.ImportJob.ImportMethod\] is one with a
    /// protection level of \[HSM][google.cloud.kms.v1.ProtectionLevel.HSM\].
    #[prost(message, optional, tag = "8")]
    pub attestation: ::core::option::Option<KeyOperationAttestation>,
}
/// Nested message and enum types in `ImportJob`.
pub mod import_job {
    /// The public key component of the wrapping key. For details of the type of
    /// key this public key corresponds to, see the
    /// \[ImportMethod][google.cloud.kms.v1.ImportJob.ImportMethod\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WrappingPublicKey {
        /// The public key, encoded in PEM format. For more information, see the [RFC
        /// 7468](<https://tools.ietf.org/html/rfc7468>) sections for [General
        /// Considerations](<https://tools.ietf.org/html/rfc7468#section-2>) and
        /// [Textual Encoding of Subject Public Key Info]
        /// (<https://tools.ietf.org/html/rfc7468#section-13>).
        #[prost(string, tag = "1")]
        pub pem: ::prost::alloc::string::String,
    }
    /// \[ImportMethod][google.cloud.kms.v1.ImportJob.ImportMethod\] describes the
    /// key wrapping method chosen for this
    /// \[ImportJob][google.cloud.kms.v1.ImportJob\].
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ImportMethod {
        /// Not specified.
        Unspecified = 0,
        /// This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping
        /// scheme defined in the PKCS #11 standard. In summary, this involves
        /// wrapping the raw key with an ephemeral AES key, and wrapping the
        /// ephemeral AES key with a 3072 bit RSA key. For more details, see
        /// [RSA AES key wrap
        /// mechanism](<http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908>).
        RsaOaep3072Sha1Aes256 = 1,
        /// This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping
        /// scheme defined in the PKCS #11 standard. In summary, this involves
        /// wrapping the raw key with an ephemeral AES key, and wrapping the
        /// ephemeral AES key with a 4096 bit RSA key. For more details, see
        /// [RSA AES key wrap
        /// mechanism](<http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908>).
        RsaOaep4096Sha1Aes256 = 2,
        /// This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping
        /// scheme defined in the PKCS #11 standard. In summary, this involves
        /// wrapping the raw key with an ephemeral AES key, and wrapping the
        /// ephemeral AES key with a 3072 bit RSA key. For more details, see
        /// [RSA AES key wrap
        /// mechanism](<http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908>).
        RsaOaep3072Sha256Aes256 = 3,
        /// This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping
        /// scheme defined in the PKCS #11 standard. In summary, this involves
        /// wrapping the raw key with an ephemeral AES key, and wrapping the
        /// ephemeral AES key with a 4096 bit RSA key. For more details, see
        /// [RSA AES key wrap
        /// mechanism](<http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908>).
        RsaOaep4096Sha256Aes256 = 4,
        /// This ImportMethod represents RSAES-OAEP with a 3072 bit RSA key. The
        /// key material to be imported is wrapped directly with the RSA key. Due
        /// to technical limitations of RSA wrapping, this method cannot be used to
        /// wrap RSA keys for import.
        RsaOaep3072Sha256 = 5,
        /// This ImportMethod represents RSAES-OAEP with a 4096 bit RSA key. The
        /// key material to be imported is wrapped directly with the RSA key. Due
        /// to technical limitations of RSA wrapping, this method cannot be used to
        /// wrap RSA keys for import.
        RsaOaep4096Sha256 = 6,
    }
    impl ImportMethod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ImportMethod::Unspecified => "IMPORT_METHOD_UNSPECIFIED",
                ImportMethod::RsaOaep3072Sha1Aes256 => "RSA_OAEP_3072_SHA1_AES_256",
                ImportMethod::RsaOaep4096Sha1Aes256 => "RSA_OAEP_4096_SHA1_AES_256",
                ImportMethod::RsaOaep3072Sha256Aes256 => "RSA_OAEP_3072_SHA256_AES_256",
                ImportMethod::RsaOaep4096Sha256Aes256 => "RSA_OAEP_4096_SHA256_AES_256",
                ImportMethod::RsaOaep3072Sha256 => "RSA_OAEP_3072_SHA256",
                ImportMethod::RsaOaep4096Sha256 => "RSA_OAEP_4096_SHA256",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IMPORT_METHOD_UNSPECIFIED" => Some(Self::Unspecified),
                "RSA_OAEP_3072_SHA1_AES_256" => Some(Self::RsaOaep3072Sha1Aes256),
                "RSA_OAEP_4096_SHA1_AES_256" => Some(Self::RsaOaep4096Sha1Aes256),
                "RSA_OAEP_3072_SHA256_AES_256" => Some(Self::RsaOaep3072Sha256Aes256),
                "RSA_OAEP_4096_SHA256_AES_256" => Some(Self::RsaOaep4096Sha256Aes256),
                "RSA_OAEP_3072_SHA256" => Some(Self::RsaOaep3072Sha256),
                "RSA_OAEP_4096_SHA256" => Some(Self::RsaOaep4096Sha256),
                _ => None,
            }
        }
    }
    /// The state of the \[ImportJob][google.cloud.kms.v1.ImportJob\], indicating if
    /// it can be used.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ImportJobState {
        /// Not specified.
        Unspecified = 0,
        /// The wrapping key for this job is still being generated. It may not be
        /// used. Cloud KMS will automatically mark this job as
        /// \[ACTIVE][google.cloud.kms.v1.ImportJob.ImportJobState.ACTIVE\] as soon as
        /// the wrapping key is generated.
        PendingGeneration = 1,
        /// This job may be used in
        /// \[CreateCryptoKey][google.cloud.kms.v1.KeyManagementService.CreateCryptoKey\]
        /// and
        /// \[CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion\]
        /// requests.
        Active = 2,
        /// This job can no longer be used and may not leave this state once entered.
        Expired = 3,
    }
    impl ImportJobState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ImportJobState::Unspecified => "IMPORT_JOB_STATE_UNSPECIFIED",
                ImportJobState::PendingGeneration => "PENDING_GENERATION",
                ImportJobState::Active => "ACTIVE",
                ImportJobState::Expired => "EXPIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "IMPORT_JOB_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING_GENERATION" => Some(Self::PendingGeneration),
                "ACTIVE" => Some(Self::Active),
                "EXPIRED" => Some(Self::Expired),
                _ => None,
            }
        }
    }
}
/// ExternalProtectionLevelOptions stores a group of additional fields for
/// configuring a \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] that
/// are specific to the \[EXTERNAL][google.cloud.kms.v1.ProtectionLevel.EXTERNAL\]
/// protection level and
/// \[EXTERNAL_VPC][google.cloud.kms.v1.ProtectionLevel.EXTERNAL_VPC\] protection
/// levels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalProtectionLevelOptions {
    /// The URI for an external resource that this
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] represents.
    #[prost(string, tag = "1")]
    pub external_key_uri: ::prost::alloc::string::String,
    /// The path to the external key material on the EKM when using
    /// \[EkmConnection][google.cloud.kms.v1.EkmConnection\] e.g., "v0/my/key". Set
    /// this field instead of external_key_uri when using an
    /// \[EkmConnection][google.cloud.kms.v1.EkmConnection\].
    #[prost(string, tag = "2")]
    pub ekm_connection_key_path: ::prost::alloc::string::String,
}
/// \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] specifies how
/// cryptographic operations are performed. For more information, see [Protection
/// levels] (<https://cloud.google.com/kms/docs/algorithms#protection_levels>).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtectionLevel {
    /// Not specified.
    Unspecified = 0,
    /// Crypto operations are performed in software.
    Software = 1,
    /// Crypto operations are performed in a Hardware Security Module.
    Hsm = 2,
    /// Crypto operations are performed by an external key manager.
    External = 3,
    /// Crypto operations are performed in an EKM-over-VPC backend.
    ExternalVpc = 4,
}
impl ProtectionLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProtectionLevel::Unspecified => "PROTECTION_LEVEL_UNSPECIFIED",
            ProtectionLevel::Software => "SOFTWARE",
            ProtectionLevel::Hsm => "HSM",
            ProtectionLevel::External => "EXTERNAL",
            ProtectionLevel::ExternalVpc => "EXTERNAL_VPC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROTECTION_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "SOFTWARE" => Some(Self::Software),
            "HSM" => Some(Self::Hsm),
            "EXTERNAL" => Some(Self::External),
            "EXTERNAL_VPC" => Some(Self::ExternalVpc),
            _ => None,
        }
    }
}
/// Request message for
/// \[EkmService.ListEkmConnections][google.cloud.kms.v1.EkmService.ListEkmConnections\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEkmConnectionsRequest {
    /// Required. The resource name of the location associated with the
    /// \[EkmConnections][google.cloud.kms.v1.EkmConnection\] to list, in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Optional limit on the number of
    /// \[EkmConnections][google.cloud.kms.v1.EkmConnection\] to include in the
    /// response. Further \[EkmConnections][google.cloud.kms.v1.EkmConnection\] can
    /// subsequently be obtained by including the
    /// \[ListEkmConnectionsResponse.next_page_token][google.cloud.kms.v1.ListEkmConnectionsResponse.next_page_token\]
    /// in a subsequent request. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Optional pagination token, returned earlier via
    /// \[ListEkmConnectionsResponse.next_page_token][google.cloud.kms.v1.ListEkmConnectionsResponse.next_page_token\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Only include resources that match the filter in the response. For
    /// more information, see
    /// [Sorting and filtering list
    /// results](<https://cloud.google.com/kms/docs/sorting-and-filtering>).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify how the results should be sorted. If not specified, the
    /// results will be sorted in the default order.  For more information, see
    /// [Sorting and filtering list
    /// results](<https://cloud.google.com/kms/docs/sorting-and-filtering>).
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// \[EkmService.ListEkmConnections][google.cloud.kms.v1.EkmService.ListEkmConnections\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEkmConnectionsResponse {
    /// The list of \[EkmConnections][google.cloud.kms.v1.EkmConnection\].
    #[prost(message, repeated, tag = "1")]
    pub ekm_connections: ::prost::alloc::vec::Vec<EkmConnection>,
    /// A token to retrieve next page of results. Pass this value in
    /// \[ListEkmConnectionsRequest.page_token][google.cloud.kms.v1.ListEkmConnectionsRequest.page_token\]
    /// to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of \[EkmConnections][google.cloud.kms.v1.EkmConnection\]
    /// that matched the query.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for
/// \[EkmService.GetEkmConnection][google.cloud.kms.v1.EkmService.GetEkmConnection\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEkmConnectionRequest {
    /// Required. The \[name][google.cloud.kms.v1.EkmConnection.name\] of the
    /// \[EkmConnection][google.cloud.kms.v1.EkmConnection\] to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[EkmService.CreateEkmConnection][google.cloud.kms.v1.EkmService.CreateEkmConnection\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEkmConnectionRequest {
    /// Required. The resource name of the location associated with the
    /// \[EkmConnection][google.cloud.kms.v1.EkmConnection\], in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. It must be unique within a location and match the regular
    /// expression `\[a-zA-Z0-9_-\]{1,63}`.
    #[prost(string, tag = "2")]
    pub ekm_connection_id: ::prost::alloc::string::String,
    /// Required. An \[EkmConnection][google.cloud.kms.v1.EkmConnection\] with
    /// initial field values.
    #[prost(message, optional, tag = "3")]
    pub ekm_connection: ::core::option::Option<EkmConnection>,
}
/// Request message for
/// \[EkmService.UpdateEkmConnection][google.cloud.kms.v1.EkmService.UpdateEkmConnection\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEkmConnectionRequest {
    /// Required. \[EkmConnection][google.cloud.kms.v1.EkmConnection\] with updated
    /// values.
    #[prost(message, optional, tag = "1")]
    pub ekm_connection: ::core::option::Option<EkmConnection>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A \[Certificate][google.cloud.kms.v1.Certificate\] represents an X.509
/// certificate used to authenticate HTTPS connections to EKM replicas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Certificate {
    /// Required. The raw certificate bytes in DER format.
    #[prost(bytes = "vec", tag = "1")]
    pub raw_der: ::prost::alloc::vec::Vec<u8>,
    /// Output only. True if the certificate was parsed successfully.
    #[prost(bool, tag = "2")]
    pub parsed: bool,
    /// Output only. The issuer distinguished name in RFC 2253 format. Only present
    /// if \[parsed][google.cloud.kms.v1.Certificate.parsed\] is true.
    #[prost(string, tag = "3")]
    pub issuer: ::prost::alloc::string::String,
    /// Output only. The subject distinguished name in RFC 2253 format. Only
    /// present if \[parsed][google.cloud.kms.v1.Certificate.parsed\] is true.
    #[prost(string, tag = "4")]
    pub subject: ::prost::alloc::string::String,
    /// Output only. The subject Alternative DNS names. Only present if
    /// \[parsed][google.cloud.kms.v1.Certificate.parsed\] is true.
    #[prost(string, repeated, tag = "5")]
    pub subject_alternative_dns_names: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Output only. The certificate is not valid before this time. Only present if
    /// \[parsed][google.cloud.kms.v1.Certificate.parsed\] is true.
    #[prost(message, optional, tag = "6")]
    pub not_before_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The certificate is not valid after this time. Only present if
    /// \[parsed][google.cloud.kms.v1.Certificate.parsed\] is true.
    #[prost(message, optional, tag = "7")]
    pub not_after_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The certificate serial number as a hex string. Only present if
    /// \[parsed][google.cloud.kms.v1.Certificate.parsed\] is true.
    #[prost(string, tag = "8")]
    pub serial_number: ::prost::alloc::string::String,
    /// Output only. The SHA-256 certificate fingerprint as a hex string. Only
    /// present if \[parsed][google.cloud.kms.v1.Certificate.parsed\] is true.
    #[prost(string, tag = "9")]
    pub sha256_fingerprint: ::prost::alloc::string::String,
}
/// An \[EkmConnection][google.cloud.kms.v1.EkmConnection\] represents an
/// individual EKM connection. It can be used for creating
/// \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] and
/// \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\] with a
/// \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] of
/// \[EXTERNAL_VPC][CryptoKeyVersion.ProtectionLevel.EXTERNAL_VPC\], as well as
/// performing cryptographic operations using keys created within the
/// \[EkmConnection][google.cloud.kms.v1.EkmConnection\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EkmConnection {
    /// Output only. The resource name for the
    /// \[EkmConnection][google.cloud.kms.v1.EkmConnection\] in the format
    /// `projects/*/locations/*/ekmConnections/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time at which the
    /// \[EkmConnection][google.cloud.kms.v1.EkmConnection\] was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A list of
    /// \[ServiceResolvers][google.cloud.kms.v1.EkmConnection.ServiceResolver\] where
    /// the EKM can be reached. There should be one ServiceResolver per EKM
    /// replica. Currently, only a single
    /// \[ServiceResolver][google.cloud.kms.v1.EkmConnection.ServiceResolver\] is
    /// supported.
    #[prost(message, repeated, tag = "3")]
    pub service_resolvers: ::prost::alloc::vec::Vec<ekm_connection::ServiceResolver>,
    /// Optional. Etag of the currently stored
    /// \[EkmConnection][google.cloud.kms.v1.EkmConnection\].
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EkmConnection`.
pub mod ekm_connection {
    /// A \[ServiceResolver][google.cloud.kms.v1.EkmConnection.ServiceResolver\]
    /// represents an EKM replica that can be reached within an
    /// \[EkmConnection][google.cloud.kms.v1.EkmConnection\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServiceResolver {
        /// Required. The resource name of the Service Directory service pointing to
        /// an EKM replica, in the format
        /// `projects/*/locations/*/namespaces/*/services/*`.
        #[prost(string, tag = "1")]
        pub service_directory_service: ::prost::alloc::string::String,
        /// Optional. The filter applied to the endpoints of the resolved service. If
        /// no filter is specified, all endpoints will be considered. An endpoint
        /// will be chosen arbitrarily from the filtered list for each request.
        ///
        /// For endpoint filter syntax and examples, see
        /// <https://cloud.google.com/service-directory/docs/reference/rpc/google.cloud.servicedirectory.v1#resolveservicerequest.>
        #[prost(string, tag = "2")]
        pub endpoint_filter: ::prost::alloc::string::String,
        /// Required. The hostname of the EKM replica used at TLS and HTTP layers.
        #[prost(string, tag = "3")]
        pub hostname: ::prost::alloc::string::String,
        /// Required. A list of leaf server certificates used to authenticate HTTPS
        /// connections to the EKM replica. Currently, a maximum of 10
        /// \[Certificate][google.cloud.kms.v1.Certificate\] is supported.
        #[prost(message, repeated, tag = "4")]
        pub server_certificates: ::prost::alloc::vec::Vec<super::Certificate>,
    }
}
/// Generated client implementations.
pub mod ekm_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Google Cloud Key Management EKM Service
    ///
    /// Manages external cryptographic keys and operations using those keys.
    /// Implements a REST model with the following objects:
    /// * [EkmConnection][google.cloud.kms.v1.EkmConnection]
    #[derive(Debug, Clone)]
    pub struct EkmServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EkmServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> EkmServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> EkmServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            EkmServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Lists [EkmConnections][google.cloud.kms.v1.EkmConnection].
        pub async fn list_ekm_connections(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEkmConnectionsRequest>,
        ) -> Result<tonic::Response<super::ListEkmConnectionsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.EkmService/ListEkmConnections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns metadata for a given
        /// [EkmConnection][google.cloud.kms.v1.EkmConnection].
        pub async fn get_ekm_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEkmConnectionRequest>,
        ) -> Result<tonic::Response<super::EkmConnection>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.EkmService/GetEkmConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new [EkmConnection][google.cloud.kms.v1.EkmConnection] in a given
        /// Project and Location.
        pub async fn create_ekm_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEkmConnectionRequest>,
        ) -> Result<tonic::Response<super::EkmConnection>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.EkmService/CreateEkmConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an [EkmConnection][google.cloud.kms.v1.EkmConnection]'s metadata.
        pub async fn update_ekm_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEkmConnectionRequest>,
        ) -> Result<tonic::Response<super::EkmConnection>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.EkmService/UpdateEkmConnection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// \[KeyManagementService.ListKeyRings][google.cloud.kms.v1.KeyManagementService.ListKeyRings\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeyRingsRequest {
    /// Required. The resource name of the location associated with the
    /// \[KeyRings][google.cloud.kms.v1.KeyRing\], in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Optional limit on the number of
    /// \[KeyRings][google.cloud.kms.v1.KeyRing\] to include in the response. Further
    /// \[KeyRings][google.cloud.kms.v1.KeyRing\] can subsequently be obtained by
    /// including the
    /// \[ListKeyRingsResponse.next_page_token][google.cloud.kms.v1.ListKeyRingsResponse.next_page_token\]
    /// in a subsequent request.  If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Optional pagination token, returned earlier via
    /// \[ListKeyRingsResponse.next_page_token][google.cloud.kms.v1.ListKeyRingsResponse.next_page_token\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Only include resources that match the filter in the response. For
    /// more information, see
    /// [Sorting and filtering list
    /// results](<https://cloud.google.com/kms/docs/sorting-and-filtering>).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify how the results should be sorted. If not specified, the
    /// results will be sorted in the default order.  For more information, see
    /// [Sorting and filtering list
    /// results](<https://cloud.google.com/kms/docs/sorting-and-filtering>).
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.ListCryptoKeys][google.cloud.kms.v1.KeyManagementService.ListCryptoKeys\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCryptoKeysRequest {
    /// Required. The resource name of the \[KeyRing][google.cloud.kms.v1.KeyRing\]
    /// to list, in the format `projects/*/locations/*/keyRings/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Optional limit on the number of
    /// \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] to include in the response.
    /// Further \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] can subsequently be
    /// obtained by including the
    /// \[ListCryptoKeysResponse.next_page_token][google.cloud.kms.v1.ListCryptoKeysResponse.next_page_token\]
    /// in a subsequent request.  If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Optional pagination token, returned earlier via
    /// \[ListCryptoKeysResponse.next_page_token][google.cloud.kms.v1.ListCryptoKeysResponse.next_page_token\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The fields of the primary version to include in the response.
    #[prost(enumeration = "crypto_key_version::CryptoKeyVersionView", tag = "4")]
    pub version_view: i32,
    /// Optional. Only include resources that match the filter in the response. For
    /// more information, see
    /// [Sorting and filtering list
    /// results](<https://cloud.google.com/kms/docs/sorting-and-filtering>).
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify how the results should be sorted. If not specified, the
    /// results will be sorted in the default order. For more information, see
    /// [Sorting and filtering list
    /// results](<https://cloud.google.com/kms/docs/sorting-and-filtering>).
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.ListCryptoKeyVersions][google.cloud.kms.v1.KeyManagementService.ListCryptoKeyVersions\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCryptoKeyVersionsRequest {
    /// Required. The resource name of the
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] to list, in the format
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Optional limit on the number of
    /// \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\] to include in the
    /// response. Further \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\]
    /// can subsequently be obtained by including the
    /// \[ListCryptoKeyVersionsResponse.next_page_token][google.cloud.kms.v1.ListCryptoKeyVersionsResponse.next_page_token\]
    /// in a subsequent request. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Optional pagination token, returned earlier via
    /// \[ListCryptoKeyVersionsResponse.next_page_token][google.cloud.kms.v1.ListCryptoKeyVersionsResponse.next_page_token\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The fields to include in the response.
    #[prost(enumeration = "crypto_key_version::CryptoKeyVersionView", tag = "4")]
    pub view: i32,
    /// Optional. Only include resources that match the filter in the response. For
    /// more information, see
    /// [Sorting and filtering list
    /// results](<https://cloud.google.com/kms/docs/sorting-and-filtering>).
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify how the results should be sorted. If not specified, the
    /// results will be sorted in the default order. For more information, see
    /// [Sorting and filtering list
    /// results](<https://cloud.google.com/kms/docs/sorting-and-filtering>).
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.ListImportJobs][google.cloud.kms.v1.KeyManagementService.ListImportJobs\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImportJobsRequest {
    /// Required. The resource name of the \[KeyRing][google.cloud.kms.v1.KeyRing\]
    /// to list, in the format `projects/*/locations/*/keyRings/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Optional limit on the number of
    /// \[ImportJobs][google.cloud.kms.v1.ImportJob\] to include in the response.
    /// Further \[ImportJobs][google.cloud.kms.v1.ImportJob\] can subsequently be
    /// obtained by including the
    /// \[ListImportJobsResponse.next_page_token][google.cloud.kms.v1.ListImportJobsResponse.next_page_token\]
    /// in a subsequent request. If unspecified, the server will pick an
    /// appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Optional pagination token, returned earlier via
    /// \[ListImportJobsResponse.next_page_token][google.cloud.kms.v1.ListImportJobsResponse.next_page_token\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Only include resources that match the filter in the response. For
    /// more information, see
    /// [Sorting and filtering list
    /// results](<https://cloud.google.com/kms/docs/sorting-and-filtering>).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specify how the results should be sorted. If not specified, the
    /// results will be sorted in the default order. For more information, see
    /// [Sorting and filtering list
    /// results](<https://cloud.google.com/kms/docs/sorting-and-filtering>).
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for
/// \[KeyManagementService.ListKeyRings][google.cloud.kms.v1.KeyManagementService.ListKeyRings\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeyRingsResponse {
    /// The list of \[KeyRings][google.cloud.kms.v1.KeyRing\].
    #[prost(message, repeated, tag = "1")]
    pub key_rings: ::prost::alloc::vec::Vec<KeyRing>,
    /// A token to retrieve next page of results. Pass this value in
    /// \[ListKeyRingsRequest.page_token][google.cloud.kms.v1.ListKeyRingsRequest.page_token\]
    /// to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of \[KeyRings][google.cloud.kms.v1.KeyRing\] that matched
    /// the query.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Response message for
/// \[KeyManagementService.ListCryptoKeys][google.cloud.kms.v1.KeyManagementService.ListCryptoKeys\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCryptoKeysResponse {
    /// The list of \[CryptoKeys][google.cloud.kms.v1.CryptoKey\].
    #[prost(message, repeated, tag = "1")]
    pub crypto_keys: ::prost::alloc::vec::Vec<CryptoKey>,
    /// A token to retrieve next page of results. Pass this value in
    /// \[ListCryptoKeysRequest.page_token][google.cloud.kms.v1.ListCryptoKeysRequest.page_token\]
    /// to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] that
    /// matched the query.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Response message for
/// \[KeyManagementService.ListCryptoKeyVersions][google.cloud.kms.v1.KeyManagementService.ListCryptoKeyVersions\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCryptoKeyVersionsResponse {
    /// The list of \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\].
    #[prost(message, repeated, tag = "1")]
    pub crypto_key_versions: ::prost::alloc::vec::Vec<CryptoKeyVersion>,
    /// A token to retrieve next page of results. Pass this value in
    /// \[ListCryptoKeyVersionsRequest.page_token][google.cloud.kms.v1.ListCryptoKeyVersionsRequest.page_token\]
    /// to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of
    /// \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\] that matched the
    /// query.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Response message for
/// \[KeyManagementService.ListImportJobs][google.cloud.kms.v1.KeyManagementService.ListImportJobs\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImportJobsResponse {
    /// The list of \[ImportJobs][google.cloud.kms.v1.ImportJob\].
    #[prost(message, repeated, tag = "1")]
    pub import_jobs: ::prost::alloc::vec::Vec<ImportJob>,
    /// A token to retrieve next page of results. Pass this value in
    /// \[ListImportJobsRequest.page_token][google.cloud.kms.v1.ListImportJobsRequest.page_token\]
    /// to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total number of \[ImportJobs][google.cloud.kms.v1.ImportJob\] that
    /// matched the query.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for
/// \[KeyManagementService.GetKeyRing][google.cloud.kms.v1.KeyManagementService.GetKeyRing\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyRingRequest {
    /// Required. The \[name][google.cloud.kms.v1.KeyRing.name\] of the
    /// \[KeyRing][google.cloud.kms.v1.KeyRing\] to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.GetCryptoKey][google.cloud.kms.v1.KeyManagementService.GetCryptoKey\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCryptoKeyRequest {
    /// Required. The \[name][google.cloud.kms.v1.CryptoKey.name\] of the
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.GetCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.GetCryptoKeyVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCryptoKeyVersionRequest {
    /// Required. The \[name][google.cloud.kms.v1.CryptoKeyVersion.name\] of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPublicKeyRequest {
    /// Required. The \[name][google.cloud.kms.v1.CryptoKeyVersion.name\] of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] public key to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.GetImportJob][google.cloud.kms.v1.KeyManagementService.GetImportJob\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImportJobRequest {
    /// Required. The \[name][google.cloud.kms.v1.ImportJob.name\] of the
    /// \[ImportJob][google.cloud.kms.v1.ImportJob\] to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.CreateKeyRing][google.cloud.kms.v1.KeyManagementService.CreateKeyRing\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyRingRequest {
    /// Required. The resource name of the location associated with the
    /// \[KeyRings][google.cloud.kms.v1.KeyRing\], in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. It must be unique within a location and match the regular
    /// expression `\[a-zA-Z0-9_-\]{1,63}`
    #[prost(string, tag = "2")]
    pub key_ring_id: ::prost::alloc::string::String,
    /// Required. A \[KeyRing][google.cloud.kms.v1.KeyRing\] with initial field
    /// values.
    #[prost(message, optional, tag = "3")]
    pub key_ring: ::core::option::Option<KeyRing>,
}
/// Request message for
/// \[KeyManagementService.CreateCryptoKey][google.cloud.kms.v1.KeyManagementService.CreateCryptoKey\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCryptoKeyRequest {
    /// Required. The \[name][google.cloud.kms.v1.KeyRing.name\] of the KeyRing
    /// associated with the \[CryptoKeys][google.cloud.kms.v1.CryptoKey\].
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. It must be unique within a KeyRing and match the regular
    /// expression `\[a-zA-Z0-9_-\]{1,63}`
    #[prost(string, tag = "2")]
    pub crypto_key_id: ::prost::alloc::string::String,
    /// Required. A \[CryptoKey][google.cloud.kms.v1.CryptoKey\] with initial field
    /// values.
    #[prost(message, optional, tag = "3")]
    pub crypto_key: ::core::option::Option<CryptoKey>,
    /// If set to true, the request will create a
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] without any
    /// \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\]. You must
    /// manually call
    /// \[CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion\]
    /// or
    /// \[ImportCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.ImportCryptoKeyVersion\]
    /// before you can use this \[CryptoKey][google.cloud.kms.v1.CryptoKey\].
    #[prost(bool, tag = "5")]
    pub skip_initial_version_creation: bool,
}
/// Request message for
/// \[KeyManagementService.CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCryptoKeyVersionRequest {
    /// Required. The \[name][google.cloud.kms.v1.CryptoKey.name\] of the
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] associated with the
    /// \[CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion\].
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] with
    /// initial field values.
    #[prost(message, optional, tag = "2")]
    pub crypto_key_version: ::core::option::Option<CryptoKeyVersion>,
}
/// Request message for
/// \[KeyManagementService.ImportCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.ImportCryptoKeyVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportCryptoKeyVersionRequest {
    /// Required. The \[name][google.cloud.kms.v1.CryptoKey.name\] of the
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] to be imported into.
    ///
    /// The create permission is only required on this key when creating a new
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\].
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The optional \[name][google.cloud.kms.v1.CryptoKeyVersion.name\] of
    /// an existing \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] to
    /// target for an import operation. If this field is not present, a new
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] containing the
    /// supplied key material is created.
    ///
    /// If this field is present, the supplied key material is imported into
    /// the existing \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]. To
    /// import into an existing
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\], the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] must be a child of
    /// \[ImportCryptoKeyVersionRequest.parent][google.cloud.kms.v1.ImportCryptoKeyVersionRequest.parent\],
    /// have been previously created via \[ImportCryptoKeyVersion][\], and be in
    /// \[DESTROYED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROYED\]
    /// or
    /// \[IMPORT_FAILED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.IMPORT_FAILED\]
    /// state. The key material and algorithm must match the previous
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] exactly if the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] has ever contained
    /// key material.
    #[prost(string, tag = "6")]
    pub crypto_key_version: ::prost::alloc::string::String,
    /// Required. The
    /// \[algorithm][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm\]
    /// of the key being imported. This does not need to match the
    /// \[version_template][google.cloud.kms.v1.CryptoKey.version_template\] of the
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] this version imports into.
    #[prost(enumeration = "crypto_key_version::CryptoKeyVersionAlgorithm", tag = "2")]
    pub algorithm: i32,
    /// Required. The \[name][google.cloud.kms.v1.ImportJob.name\] of the
    /// \[ImportJob][google.cloud.kms.v1.ImportJob\] that was used to wrap this key
    /// material.
    #[prost(string, tag = "4")]
    pub import_job: ::prost::alloc::string::String,
    /// Optional. The wrapped key material to import.
    ///
    /// Before wrapping, key material must be formatted. If importing symmetric key
    /// material, the expected key material format is plain bytes. If importing
    /// asymmetric key material, the expected key material format is PKCS#8-encoded
    /// DER (the PrivateKeyInfo structure from RFC 5208).
    ///
    /// When wrapping with import methods
    /// (\[RSA_OAEP_3072_SHA1_AES_256][google.cloud.kms.v1.ImportJob.ImportMethod.RSA_OAEP_3072_SHA1_AES_256\]
    /// or
    /// \[RSA_OAEP_4096_SHA1_AES_256][google.cloud.kms.v1.ImportJob.ImportMethod.RSA_OAEP_4096_SHA1_AES_256\]
    /// or
    /// \[RSA_OAEP_3072_SHA256_AES_256][google.cloud.kms.v1.ImportJob.ImportMethod.RSA_OAEP_3072_SHA256_AES_256\]
    /// or
    /// \[RSA_OAEP_4096_SHA256_AES_256][google.cloud.kms.v1.ImportJob.ImportMethod.RSA_OAEP_4096_SHA256_AES_256\]),
    ///
    /// this field must contain the concatenation of:
    /// <ol>
    ///    <li>An ephemeral AES-256 wrapping key wrapped with the
    ///        \[public_key][google.cloud.kms.v1.ImportJob.public_key\] using
    ///        RSAES-OAEP with SHA-1/SHA-256, MGF1 with SHA-1/SHA-256, and an empty
    ///        label.
    ///    </li>
    ///    <li>The formatted key to be imported, wrapped with the ephemeral AES-256
    ///        key using AES-KWP (RFC 5649).
    ///    </li>
    /// </ol>
    ///
    /// This format is the same as the format produced by PKCS#11 mechanism
    /// CKM_RSA_AES_KEY_WRAP.
    ///
    /// When wrapping with import methods
    /// (\[RSA_OAEP_3072_SHA256][google.cloud.kms.v1.ImportJob.ImportMethod.RSA_OAEP_3072_SHA256\]
    /// or
    /// \[RSA_OAEP_4096_SHA256][google.cloud.kms.v1.ImportJob.ImportMethod.RSA_OAEP_4096_SHA256\]),
    ///
    /// this field must contain the formatted key to be imported, wrapped with the
    /// \[public_key][google.cloud.kms.v1.ImportJob.public_key\] using RSAES-OAEP
    /// with SHA-256, MGF1 with SHA-256, and an empty label.
    #[prost(bytes = "vec", tag = "8")]
    pub wrapped_key: ::prost::alloc::vec::Vec<u8>,
    /// This field is legacy. Use the field
    /// \[wrapped_key][google.cloud.kms.v1.ImportCryptoKeyVersionRequest.wrapped_key\]
    /// instead.
    #[prost(oneof = "import_crypto_key_version_request::WrappedKeyMaterial", tags = "5")]
    pub wrapped_key_material: ::core::option::Option<
        import_crypto_key_version_request::WrappedKeyMaterial,
    >,
}
/// Nested message and enum types in `ImportCryptoKeyVersionRequest`.
pub mod import_crypto_key_version_request {
    /// This field is legacy. Use the field
    /// \[wrapped_key][google.cloud.kms.v1.ImportCryptoKeyVersionRequest.wrapped_key\]
    /// instead.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WrappedKeyMaterial {
        /// Optional. This field has the same meaning as
        /// \[wrapped_key][google.cloud.kms.v1.ImportCryptoKeyVersionRequest.wrapped_key\].
        /// Prefer to use that field in new work. Either that field or this field
        /// (but not both) must be specified.
        #[prost(bytes, tag = "5")]
        RsaAesWrappedKey(::prost::alloc::vec::Vec<u8>),
    }
}
/// Request message for
/// \[KeyManagementService.CreateImportJob][google.cloud.kms.v1.KeyManagementService.CreateImportJob\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateImportJobRequest {
    /// Required. The \[name][google.cloud.kms.v1.KeyRing.name\] of the
    /// \[KeyRing][google.cloud.kms.v1.KeyRing\] associated with the
    /// \[ImportJobs][google.cloud.kms.v1.ImportJob\].
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. It must be unique within a KeyRing and match the regular
    /// expression `\[a-zA-Z0-9_-\]{1,63}`
    #[prost(string, tag = "2")]
    pub import_job_id: ::prost::alloc::string::String,
    /// Required. An \[ImportJob][google.cloud.kms.v1.ImportJob\] with initial field
    /// values.
    #[prost(message, optional, tag = "3")]
    pub import_job: ::core::option::Option<ImportJob>,
}
/// Request message for
/// \[KeyManagementService.UpdateCryptoKey][google.cloud.kms.v1.KeyManagementService.UpdateCryptoKey\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCryptoKeyRequest {
    /// Required. \[CryptoKey][google.cloud.kms.v1.CryptoKey\] with updated values.
    #[prost(message, optional, tag = "1")]
    pub crypto_key: ::core::option::Option<CryptoKey>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[KeyManagementService.UpdateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.UpdateCryptoKeyVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCryptoKeyVersionRequest {
    /// Required. \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] with
    /// updated values.
    #[prost(message, optional, tag = "1")]
    pub crypto_key_version: ::core::option::Option<CryptoKeyVersion>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// \[KeyManagementService.UpdateCryptoKeyPrimaryVersion][google.cloud.kms.v1.KeyManagementService.UpdateCryptoKeyPrimaryVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCryptoKeyPrimaryVersionRequest {
    /// Required. The resource name of the
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] to update.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The id of the child
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] to use as primary.
    #[prost(string, tag = "2")]
    pub crypto_key_version_id: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.DestroyCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.DestroyCryptoKeyVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyCryptoKeyVersionRequest {
    /// Required. The resource name of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] to destroy.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.RestoreCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.RestoreCryptoKeyVersion\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreCryptoKeyVersionRequest {
    /// Required. The resource name of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] to restore.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[KeyManagementService.Decrypt][google.cloud.kms.v1.KeyManagementService.Decrypt\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptRequest {
    /// Required. The resource name of the
    /// \[CryptoKey][google.cloud.kms.v1.CryptoKey\] to use for decryption. The
    /// server will choose the appropriate version.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The encrypted data originally returned in
    /// \[EncryptResponse.ciphertext][google.cloud.kms.v1.EncryptResponse.ciphertext\].
    #[prost(bytes = "vec", tag = "2")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
    /// Optional. Optional data that must match the data originally supplied in
    /// \[EncryptRequest.additional_authenticated_data][google.cloud.kms.v1.EncryptRequest.additional_authenticated_data\].
    #[prost(bytes = "vec", tag = "3")]
    pub additional_authenticated_data: ::prost::alloc::vec::Vec<u8>,
    /// Optional. An optional CRC32C checksum of the
    /// \[DecryptRequest.ciphertext][google.cloud.kms.v1.DecryptRequest.ciphertext\].
    /// If specified,
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// verify the integrity of the received
    /// \[DecryptRequest.ciphertext][google.cloud.kms.v1.DecryptRequest.ciphertext\]
    /// using this checksum.
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// report an error if the checksum verification fails. If you receive a
    /// checksum error, your client should verify that
    /// CRC32C(\[DecryptRequest.ciphertext][google.cloud.kms.v1.DecryptRequest.ciphertext\])
    /// is equal to
    /// \[DecryptRequest.ciphertext_crc32c][google.cloud.kms.v1.DecryptRequest.ciphertext_crc32c\],
    /// and if so, perform a limited number of retries. A persistent mismatch may
    /// indicate an issue in your computation of the CRC32C checksum. Note: This
    /// field is defined as int64 for reasons of compatibility across different
    /// languages. However, it is a non-negative integer, which will never exceed
    /// 2^32-1, and can be safely downconverted to uint32 in languages that support
    /// this type.
    #[prost(message, optional, tag = "5")]
    pub ciphertext_crc32c: ::core::option::Option<i64>,
    /// Optional. An optional CRC32C checksum of the
    /// \[DecryptRequest.additional_authenticated_data][google.cloud.kms.v1.DecryptRequest.additional_authenticated_data\].
    /// If specified,
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// verify the integrity of the received
    /// \[DecryptRequest.additional_authenticated_data][google.cloud.kms.v1.DecryptRequest.additional_authenticated_data\]
    /// using this checksum.
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// report an error if the checksum verification fails. If you receive a
    /// checksum error, your client should verify that
    /// CRC32C(\[DecryptRequest.additional_authenticated_data][google.cloud.kms.v1.DecryptRequest.additional_authenticated_data\])
    /// is equal to
    /// \[DecryptRequest.additional_authenticated_data_crc32c][google.cloud.kms.v1.DecryptRequest.additional_authenticated_data_crc32c\],
    /// and if so, perform a limited number of retries. A persistent mismatch may
    /// indicate an issue in your computation of the CRC32C checksum. Note: This
    /// field is defined as int64 for reasons of compatibility across different
    /// languages. However, it is a non-negative integer, which will never exceed
    /// 2^32-1, and can be safely downconverted to uint32 in languages that support
    /// this type.
    #[prost(message, optional, tag = "6")]
    pub additional_authenticated_data_crc32c: ::core::option::Option<i64>,
}
/// Request message for
/// \[KeyManagementService.AsymmetricSign][google.cloud.kms.v1.KeyManagementService.AsymmetricSign\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsymmetricSignRequest {
    /// Required. The resource name of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] to use for
    /// signing.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The digest of the data to sign. The digest must be produced with
    /// the same digest algorithm as specified by the key version's
    /// \[algorithm][google.cloud.kms.v1.CryptoKeyVersion.algorithm\].
    ///
    /// This field may not be supplied if
    /// \[AsymmetricSignRequest.data][google.cloud.kms.v1.AsymmetricSignRequest.data\]
    /// is supplied.
    #[prost(message, optional, tag = "3")]
    pub digest: ::core::option::Option<Digest>,
    /// Optional. An optional CRC32C checksum of the
    /// \[AsymmetricSignRequest.digest][google.cloud.kms.v1.AsymmetricSignRequest.digest\].
    /// If specified,
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// verify the integrity of the received
    /// \[AsymmetricSignRequest.digest][google.cloud.kms.v1.AsymmetricSignRequest.digest\]
    /// using this checksum.
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// report an error if the checksum verification fails. If you receive a
    /// checksum error, your client should verify that
    /// CRC32C(\[AsymmetricSignRequest.digest][google.cloud.kms.v1.AsymmetricSignRequest.digest\])
    /// is equal to
    /// \[AsymmetricSignRequest.digest_crc32c][google.cloud.kms.v1.AsymmetricSignRequest.digest_crc32c\],
    /// and if so, perform a limited number of retries. A persistent mismatch may
    /// indicate an issue in your computation of the CRC32C checksum. Note: This
    /// field is defined as int64 for reasons of compatibility across different
    /// languages. However, it is a non-negative integer, which will never exceed
    /// 2^32-1, and can be safely downconverted to uint32 in languages that support
    /// this type.
    #[prost(message, optional, tag = "4")]
    pub digest_crc32c: ::core::option::Option<i64>,
    /// Optional. The data to sign.
    /// It can't be supplied if
    /// \[AsymmetricSignRequest.digest][google.cloud.kms.v1.AsymmetricSignRequest.digest\]
    /// is supplied.
    #[prost(bytes = "vec", tag = "6")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Optional. An optional CRC32C checksum of the
    /// \[AsymmetricSignRequest.data][google.cloud.kms.v1.AsymmetricSignRequest.data\].
    /// If specified,
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// verify the integrity of the received
    /// \[AsymmetricSignRequest.data][google.cloud.kms.v1.AsymmetricSignRequest.data\]
    /// using this checksum.
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// report an error if the checksum verification fails. If you receive a
    /// checksum error, your client should verify that
    /// CRC32C(\[AsymmetricSignRequest.data][google.cloud.kms.v1.AsymmetricSignRequest.data\])
    /// is equal to
    /// \[AsymmetricSignRequest.data_crc32c][google.cloud.kms.v1.AsymmetricSignRequest.data_crc32c\],
    /// and if so, perform a limited number of retries. A persistent mismatch may
    /// indicate an issue in your computation of the CRC32C checksum. Note: This
    /// field is defined as int64 for reasons of compatibility across different
    /// languages. However, it is a non-negative integer, which will never exceed
    /// 2^32-1, and can be safely downconverted to uint32 in languages that support
    /// this type.
    #[prost(message, optional, tag = "7")]
    pub data_crc32c: ::core::option::Option<i64>,
}
/// Request message for
/// \[KeyManagementService.AsymmetricDecrypt][google.cloud.kms.v1.KeyManagementService.AsymmetricDecrypt\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsymmetricDecryptRequest {
    /// Required. The resource name of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] to use for
    /// decryption.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The data encrypted with the named
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\]'s public key using
    /// OAEP.
    #[prost(bytes = "vec", tag = "3")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
    /// Optional. An optional CRC32C checksum of the
    /// \[AsymmetricDecryptRequest.ciphertext][google.cloud.kms.v1.AsymmetricDecryptRequest.ciphertext\].
    /// If specified,
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// verify the integrity of the received
    /// \[AsymmetricDecryptRequest.ciphertext][google.cloud.kms.v1.AsymmetricDecryptRequest.ciphertext\]
    /// using this checksum.
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// report an error if the checksum verification fails. If you receive a
    /// checksum error, your client should verify that
    /// CRC32C(\[AsymmetricDecryptRequest.ciphertext][google.cloud.kms.v1.AsymmetricDecryptRequest.ciphertext\])
    /// is equal to
    /// \[AsymmetricDecryptRequest.ciphertext_crc32c][google.cloud.kms.v1.AsymmetricDecryptRequest.ciphertext_crc32c\],
    /// and if so, perform a limited number of retries. A persistent mismatch may
    /// indicate an issue in your computation of the CRC32C checksum. Note: This
    /// field is defined as int64 for reasons of compatibility across different
    /// languages. However, it is a non-negative integer, which will never exceed
    /// 2^32-1, and can be safely downconverted to uint32 in languages that support
    /// this type.
    #[prost(message, optional, tag = "4")]
    pub ciphertext_crc32c: ::core::option::Option<i64>,
}
/// Request message for
/// \[KeyManagementService.MacSign][google.cloud.kms.v1.KeyManagementService.MacSign\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MacSignRequest {
    /// Required. The resource name of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] to use for
    /// signing.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The data to sign. The MAC tag is computed over this data field
    /// based on the specific algorithm.
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Optional. An optional CRC32C checksum of the
    /// \[MacSignRequest.data][google.cloud.kms.v1.MacSignRequest.data\]. If
    /// specified, \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]
    /// will verify the integrity of the received
    /// \[MacSignRequest.data][google.cloud.kms.v1.MacSignRequest.data\] using this
    /// checksum. \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]
    /// will report an error if the checksum verification fails. If you receive a
    /// checksum error, your client should verify that
    /// CRC32C(\[MacSignRequest.data][google.cloud.kms.v1.MacSignRequest.data\]) is
    /// equal to
    /// \[MacSignRequest.data_crc32c][google.cloud.kms.v1.MacSignRequest.data_crc32c\],
    /// and if so, perform a limited number of retries. A persistent mismatch may
    /// indicate an issue in your computation of the CRC32C checksum. Note: This
    /// field is defined as int64 for reasons of compatibility across different
    /// languages. However, it is a non-negative integer, which will never exceed
    /// 2^32-1, and can be safely downconverted to uint32 in languages that support
    /// this type.
    #[prost(message, optional, tag = "3")]
    pub data_crc32c: ::core::option::Option<i64>,
}
/// Request message for
/// \[KeyManagementService.MacVerify][google.cloud.kms.v1.KeyManagementService.MacVerify\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MacVerifyRequest {
    /// Required. The resource name of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] to use for
    /// verification.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The data used previously as a
    /// \[MacSignRequest.data][google.cloud.kms.v1.MacSignRequest.data\] to generate
    /// the MAC tag.
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Optional. An optional CRC32C checksum of the
    /// \[MacVerifyRequest.data][google.cloud.kms.v1.MacVerifyRequest.data\]. If
    /// specified, \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]
    /// will verify the integrity of the received
    /// \[MacVerifyRequest.data][google.cloud.kms.v1.MacVerifyRequest.data\] using
    /// this checksum.
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] will
    /// report an error if the checksum verification fails. If you receive a
    /// checksum error, your client should verify that
    /// CRC32C(\[MacVerifyRequest.data][google.cloud.kms.v1.MacVerifyRequest.data\])
    /// is equal to
    /// \[MacVerifyRequest.data_crc32c][google.cloud.kms.v1.MacVerifyRequest.data_crc32c\],
    /// and if so, perform a limited number of retries. A persistent mismatch may
    /// indicate an issue in your computation of the CRC32C checksum. Note: This
    /// field is defined as int64 for reasons of compatibility across different
    /// languages. However, it is a non-negative integer, which will never exceed
    /// 2^32-1, and can be safely downconverted to uint32 in languages that support
    /// this type.
    #[prost(message, optional, tag = "3")]
    pub data_crc32c: ::core::option::Option<i64>,
    /// Required. The signature to verify.
    #[prost(bytes = "vec", tag = "4")]
    pub mac: ::prost::alloc::vec::Vec<u8>,
    /// Optional. An optional CRC32C checksum of the
    /// \[MacVerifyRequest.mac][google.cloud.kms.v1.MacVerifyRequest.mac\]. If
    /// specified, \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]
    /// will verify the integrity of the received
    /// \[MacVerifyRequest.mac][google.cloud.kms.v1.MacVerifyRequest.mac\] using this
    /// checksum. \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]
    /// will report an error if the checksum verification fails. If you receive a
    /// checksum error, your client should verify that
    /// CRC32C(\[MacVerifyRequest.tag][\]) is equal to
    /// \[MacVerifyRequest.mac_crc32c][google.cloud.kms.v1.MacVerifyRequest.mac_crc32c\],
    /// and if so, perform a limited number of retries. A persistent mismatch may
    /// indicate an issue in your computation of the CRC32C checksum. Note: This
    /// field is defined as int64 for reasons of compatibility across different
    /// languages. However, it is a non-negative integer, which will never exceed
    /// 2^32-1, and can be safely downconverted to uint32 in languages that support
    /// this type.
    #[prost(message, optional, tag = "5")]
    pub mac_crc32c: ::core::option::Option<i64>,
}
/// Request message for
/// \[KeyManagementService.GenerateRandomBytes][google.cloud.kms.v1.KeyManagementService.GenerateRandomBytes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateRandomBytesRequest {
    /// The project-specific location in which to generate random bytes.
    /// For example, "projects/my-project/locations/us-central1".
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    /// The length in bytes of the amount of randomness to retrieve.  Minimum 8
    /// bytes, maximum 1024 bytes.
    #[prost(int32, tag = "2")]
    pub length_bytes: i32,
    /// The \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] to use when
    /// generating the random data. Currently, only
    /// \[HSM][google.cloud.kms.v1.ProtectionLevel.HSM\] protection level is
    /// supported.
    #[prost(enumeration = "ProtectionLevel", tag = "3")]
    pub protection_level: i32,
}
/// Response message for
/// \[KeyManagementService.Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptResponse {
    /// The resource name of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] used in
    /// encryption. Check this field to verify that the intended resource was used
    /// for encryption.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The encrypted data.
    #[prost(bytes = "vec", tag = "2")]
    pub ciphertext: ::prost::alloc::vec::Vec<u8>,
    /// Integrity verification field. A CRC32C checksum of the returned
    /// \[EncryptResponse.ciphertext][google.cloud.kms.v1.EncryptResponse.ciphertext\].
    /// An integrity check of
    /// \[EncryptResponse.ciphertext][google.cloud.kms.v1.EncryptResponse.ciphertext\]
    /// can be performed by computing the CRC32C checksum of
    /// \[EncryptResponse.ciphertext][google.cloud.kms.v1.EncryptResponse.ciphertext\]
    /// and comparing your results to this field. Discard the response in case of
    /// non-matching checksum values, and perform a limited number of retries. A
    /// persistent mismatch may indicate an issue in your computation of the CRC32C
    /// checksum. Note: This field is defined as int64 for reasons of compatibility
    /// across different languages. However, it is a non-negative integer, which
    /// will never exceed 2^32-1, and can be safely downconverted to uint32 in
    /// languages that support this type.
    #[prost(message, optional, tag = "4")]
    pub ciphertext_crc32c: ::core::option::Option<i64>,
    /// Integrity verification field. A flag indicating whether
    /// \[EncryptRequest.plaintext_crc32c][google.cloud.kms.v1.EncryptRequest.plaintext_crc32c\]
    /// was received by
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] and used
    /// for the integrity verification of the
    /// \[plaintext][google.cloud.kms.v1.EncryptRequest.plaintext\]. A false value of
    /// this field indicates either that
    /// \[EncryptRequest.plaintext_crc32c][google.cloud.kms.v1.EncryptRequest.plaintext_crc32c\]
    /// was left unset or that it was not delivered to
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]. If you've
    /// set
    /// \[EncryptRequest.plaintext_crc32c][google.cloud.kms.v1.EncryptRequest.plaintext_crc32c\]
    /// but this field is still false, discard the response and perform a limited
    /// number of retries.
    #[prost(bool, tag = "5")]
    pub verified_plaintext_crc32c: bool,
    /// Integrity verification field. A flag indicating whether
    /// \[EncryptRequest.additional_authenticated_data_crc32c][google.cloud.kms.v1.EncryptRequest.additional_authenticated_data_crc32c\]
    /// was received by
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] and used
    /// for the integrity verification of the
    /// \[AAD][google.cloud.kms.v1.EncryptRequest.additional_authenticated_data\]. A
    /// false value of this field indicates either that
    /// \[EncryptRequest.additional_authenticated_data_crc32c][google.cloud.kms.v1.EncryptRequest.additional_authenticated_data_crc32c\]
    /// was left unset or that it was not delivered to
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]. If you've
    /// set
    /// \[EncryptRequest.additional_authenticated_data_crc32c][google.cloud.kms.v1.EncryptRequest.additional_authenticated_data_crc32c\]
    /// but this field is still false, discard the response and perform a limited
    /// number of retries.
    #[prost(bool, tag = "6")]
    pub verified_additional_authenticated_data_crc32c: bool,
    /// The \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] used in
    /// encryption.
    #[prost(enumeration = "ProtectionLevel", tag = "7")]
    pub protection_level: i32,
}
/// Response message for
/// \[KeyManagementService.AsymmetricSign][google.cloud.kms.v1.KeyManagementService.AsymmetricSign\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsymmetricSignResponse {
    /// The created signature.
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    /// Integrity verification field. A CRC32C checksum of the returned
    /// \[AsymmetricSignResponse.signature][google.cloud.kms.v1.AsymmetricSignResponse.signature\].
    /// An integrity check of
    /// \[AsymmetricSignResponse.signature][google.cloud.kms.v1.AsymmetricSignResponse.signature\]
    /// can be performed by computing the CRC32C checksum of
    /// \[AsymmetricSignResponse.signature][google.cloud.kms.v1.AsymmetricSignResponse.signature\]
    /// and comparing your results to this field. Discard the response in case of
    /// non-matching checksum values, and perform a limited number of retries. A
    /// persistent mismatch may indicate an issue in your computation of the CRC32C
    /// checksum. Note: This field is defined as int64 for reasons of compatibility
    /// across different languages. However, it is a non-negative integer, which
    /// will never exceed 2^32-1, and can be safely downconverted to uint32 in
    /// languages that support this type.
    #[prost(message, optional, tag = "2")]
    pub signature_crc32c: ::core::option::Option<i64>,
    /// Integrity verification field. A flag indicating whether
    /// \[AsymmetricSignRequest.digest_crc32c][google.cloud.kms.v1.AsymmetricSignRequest.digest_crc32c\]
    /// was received by
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] and used
    /// for the integrity verification of the
    /// \[digest][google.cloud.kms.v1.AsymmetricSignRequest.digest\]. A false value
    /// of this field indicates either that
    /// \[AsymmetricSignRequest.digest_crc32c][google.cloud.kms.v1.AsymmetricSignRequest.digest_crc32c\]
    /// was left unset or that it was not delivered to
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]. If you've
    /// set
    /// \[AsymmetricSignRequest.digest_crc32c][google.cloud.kms.v1.AsymmetricSignRequest.digest_crc32c\]
    /// but this field is still false, discard the response and perform a limited
    /// number of retries.
    #[prost(bool, tag = "3")]
    pub verified_digest_crc32c: bool,
    /// The resource name of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] used for signing.
    /// Check this field to verify that the intended resource was used for signing.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Integrity verification field. A flag indicating whether
    /// \[AsymmetricSignRequest.data_crc32c][google.cloud.kms.v1.AsymmetricSignRequest.data_crc32c\]
    /// was received by
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] and used
    /// for the integrity verification of the
    /// \[data][google.cloud.kms.v1.AsymmetricSignRequest.data\]. A false value of
    /// this field indicates either that
    /// \[AsymmetricSignRequest.data_crc32c][google.cloud.kms.v1.AsymmetricSignRequest.data_crc32c\]
    /// was left unset or that it was not delivered to
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]. If you've
    /// set
    /// \[AsymmetricSignRequest.data_crc32c][google.cloud.kms.v1.AsymmetricSignRequest.data_crc32c\]
    /// but this field is still false, discard the response and perform a limited
    /// number of retries.
    #[prost(bool, tag = "5")]
    pub verified_data_crc32c: bool,
    /// The \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] used for signing.
    #[prost(enumeration = "ProtectionLevel", tag = "6")]
    pub protection_level: i32,
}
/// Response message for
/// \[KeyManagementService.AsymmetricDecrypt][google.cloud.kms.v1.KeyManagementService.AsymmetricDecrypt\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsymmetricDecryptResponse {
    /// The decrypted data originally encrypted with the matching public key.
    #[prost(bytes = "vec", tag = "1")]
    pub plaintext: ::prost::alloc::vec::Vec<u8>,
    /// Integrity verification field. A CRC32C checksum of the returned
    /// \[AsymmetricDecryptResponse.plaintext][google.cloud.kms.v1.AsymmetricDecryptResponse.plaintext\].
    /// An integrity check of
    /// \[AsymmetricDecryptResponse.plaintext][google.cloud.kms.v1.AsymmetricDecryptResponse.plaintext\]
    /// can be performed by computing the CRC32C checksum of
    /// \[AsymmetricDecryptResponse.plaintext][google.cloud.kms.v1.AsymmetricDecryptResponse.plaintext\]
    /// and comparing your results to this field. Discard the response in case of
    /// non-matching checksum values, and perform a limited number of retries. A
    /// persistent mismatch may indicate an issue in your computation of the CRC32C
    /// checksum. Note: This field is defined as int64 for reasons of compatibility
    /// across different languages. However, it is a non-negative integer, which
    /// will never exceed 2^32-1, and can be safely downconverted to uint32 in
    /// languages that support this type.
    #[prost(message, optional, tag = "2")]
    pub plaintext_crc32c: ::core::option::Option<i64>,
    /// Integrity verification field. A flag indicating whether
    /// \[AsymmetricDecryptRequest.ciphertext_crc32c][google.cloud.kms.v1.AsymmetricDecryptRequest.ciphertext_crc32c\]
    /// was received by
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] and used
    /// for the integrity verification of the
    /// \[ciphertext][google.cloud.kms.v1.AsymmetricDecryptRequest.ciphertext\]. A
    /// false value of this field indicates either that
    /// \[AsymmetricDecryptRequest.ciphertext_crc32c][google.cloud.kms.v1.AsymmetricDecryptRequest.ciphertext_crc32c\]
    /// was left unset or that it was not delivered to
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]. If you've
    /// set
    /// \[AsymmetricDecryptRequest.ciphertext_crc32c][google.cloud.kms.v1.AsymmetricDecryptRequest.ciphertext_crc32c\]
    /// but this field is still false, discard the response and perform a limited
    /// number of retries.
    #[prost(bool, tag = "3")]
    pub verified_ciphertext_crc32c: bool,
    /// The \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] used in
    /// decryption.
    #[prost(enumeration = "ProtectionLevel", tag = "4")]
    pub protection_level: i32,
}
/// Response message for
/// \[KeyManagementService.MacSign][google.cloud.kms.v1.KeyManagementService.MacSign\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MacSignResponse {
    /// The resource name of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] used for signing.
    /// Check this field to verify that the intended resource was used for signing.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The created signature.
    #[prost(bytes = "vec", tag = "2")]
    pub mac: ::prost::alloc::vec::Vec<u8>,
    /// Integrity verification field. A CRC32C checksum of the returned
    /// \[MacSignResponse.mac][google.cloud.kms.v1.MacSignResponse.mac\]. An
    /// integrity check of
    /// \[MacSignResponse.mac][google.cloud.kms.v1.MacSignResponse.mac\] can be
    /// performed by computing the CRC32C checksum of
    /// \[MacSignResponse.mac][google.cloud.kms.v1.MacSignResponse.mac\] and
    /// comparing your results to this field. Discard the response in case of
    /// non-matching checksum values, and perform a limited number of retries. A
    /// persistent mismatch may indicate an issue in your computation of the CRC32C
    /// checksum. Note: This field is defined as int64 for reasons of compatibility
    /// across different languages. However, it is a non-negative integer, which
    /// will never exceed 2^32-1, and can be safely downconverted to uint32 in
    /// languages that support this type.
    #[prost(message, optional, tag = "3")]
    pub mac_crc32c: ::core::option::Option<i64>,
    /// Integrity verification field. A flag indicating whether
    /// \[MacSignRequest.data_crc32c][google.cloud.kms.v1.MacSignRequest.data_crc32c\]
    /// was received by
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] and used
    /// for the integrity verification of the
    /// \[data][google.cloud.kms.v1.MacSignRequest.data\]. A false value of this
    /// field indicates either that
    /// \[MacSignRequest.data_crc32c][google.cloud.kms.v1.MacSignRequest.data_crc32c\]
    /// was left unset or that it was not delivered to
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]. If you've
    /// set
    /// \[MacSignRequest.data_crc32c][google.cloud.kms.v1.MacSignRequest.data_crc32c\]
    /// but this field is still false, discard the response and perform a limited
    /// number of retries.
    #[prost(bool, tag = "4")]
    pub verified_data_crc32c: bool,
    /// The \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] used for signing.
    #[prost(enumeration = "ProtectionLevel", tag = "5")]
    pub protection_level: i32,
}
/// Response message for
/// \[KeyManagementService.MacVerify][google.cloud.kms.v1.KeyManagementService.MacVerify\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MacVerifyResponse {
    /// The resource name of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] used for
    /// verification. Check this field to verify that the intended resource was
    /// used for verification.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// This field indicates whether or not the verification operation for
    /// \[MacVerifyRequest.mac][google.cloud.kms.v1.MacVerifyRequest.mac\] over
    /// \[MacVerifyRequest.data][google.cloud.kms.v1.MacVerifyRequest.data\] was
    /// successful.
    #[prost(bool, tag = "2")]
    pub success: bool,
    /// Integrity verification field. A flag indicating whether
    /// \[MacVerifyRequest.data_crc32c][google.cloud.kms.v1.MacVerifyRequest.data_crc32c\]
    /// was received by
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] and used
    /// for the integrity verification of the
    /// \[data][google.cloud.kms.v1.MacVerifyRequest.data\]. A false value of this
    /// field indicates either that
    /// \[MacVerifyRequest.data_crc32c][google.cloud.kms.v1.MacVerifyRequest.data_crc32c\]
    /// was left unset or that it was not delivered to
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]. If you've
    /// set
    /// \[MacVerifyRequest.data_crc32c][google.cloud.kms.v1.MacVerifyRequest.data_crc32c\]
    /// but this field is still false, discard the response and perform a limited
    /// number of retries.
    #[prost(bool, tag = "3")]
    pub verified_data_crc32c: bool,
    /// Integrity verification field. A flag indicating whether
    /// \[MacVerifyRequest.mac_crc32c][google.cloud.kms.v1.MacVerifyRequest.mac_crc32c\]
    /// was received by
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\] and used
    /// for the integrity verification of the
    /// \[data][google.cloud.kms.v1.MacVerifyRequest.mac\]. A false value of this
    /// field indicates either that
    /// \[MacVerifyRequest.mac_crc32c][google.cloud.kms.v1.MacVerifyRequest.mac_crc32c\]
    /// was left unset or that it was not delivered to
    /// \[KeyManagementService][google.cloud.kms.v1.KeyManagementService\]. If you've
    /// set
    /// \[MacVerifyRequest.mac_crc32c][google.cloud.kms.v1.MacVerifyRequest.mac_crc32c\]
    /// but this field is still false, discard the response and perform a limited
    /// number of retries.
    #[prost(bool, tag = "4")]
    pub verified_mac_crc32c: bool,
    /// Integrity verification field. This value is used for the integrity
    /// verification of \[MacVerifyResponse.success\]. If the value of this field
    /// contradicts the value of \[MacVerifyResponse.success\], discard the response
    /// and perform a limited number of retries.
    #[prost(bool, tag = "5")]
    pub verified_success_integrity: bool,
    /// The \[ProtectionLevel][google.cloud.kms.v1.ProtectionLevel\] of the
    /// \[CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion\] used for
    /// verification.
    #[prost(enumeration = "ProtectionLevel", tag = "6")]
    pub protection_level: i32,
}
/// Response message for
/// \[KeyManagementService.GenerateRandomBytes][google.cloud.kms.v1.KeyManagementService.GenerateRandomBytes\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateRandomBytesResponse {
    /// The generated data.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Integrity verification field. A CRC32C checksum of the returned
    /// \[GenerateRandomBytesResponse.data][google.cloud.kms.v1.GenerateRandomBytesResponse.data\].
    /// An integrity check of
    /// \[GenerateRandomBytesResponse.data][google.cloud.kms.v1.GenerateRandomBytesResponse.data\]
    /// can be performed by computing the CRC32C checksum of
    /// \[GenerateRandomBytesResponse.data][google.cloud.kms.v1.GenerateRandomBytesResponse.data\]
    /// and comparing your results to this field. Discard the response in case of
    /// non-matching checksum values, and perform a limited number of retries. A
    /// persistent mismatch may indicate an issue in your computation of the CRC32C
    /// checksum. Note: This field is defined as int64 for reasons of compatibility
    /// across different languages. However, it is a non-negative integer, which
    /// will never exceed 2^32-1, and can be safely downconverted to uint32 in
    /// languages that support this type.
    #[prost(message, optional, tag = "3")]
    pub data_crc32c: ::core::option::Option<i64>,
}
/// A \[Digest][google.cloud.kms.v1.Digest\] holds a cryptographic message digest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Digest {
    /// Required. The message digest.
    #[prost(oneof = "digest::Digest", tags = "1, 2, 3")]
    pub digest: ::core::option::Option<digest::Digest>,
}
/// Nested message and enum types in `Digest`.
pub mod digest {
    /// Required. The message digest.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Digest {
        /// A message digest produced with the SHA-256 algorithm.
        #[prost(bytes, tag = "1")]
        Sha256(::prost::alloc::vec::Vec<u8>),
        /// A message digest produced with the SHA-384 algorithm.
        #[prost(bytes, tag = "2")]
        Sha384(::prost::alloc::vec::Vec<u8>),
        /// A message digest produced with the SHA-512 algorithm.
        #[prost(bytes, tag = "3")]
        Sha512(::prost::alloc::vec::Vec<u8>),
    }
}
/// Cloud KMS metadata for the given
/// \[google.cloud.location.Location][google.cloud.location.Location\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// Indicates whether \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] with
    /// \[protection_level][google.cloud.kms.v1.CryptoKeyVersionTemplate.protection_level\]
    /// \[HSM][google.cloud.kms.v1.ProtectionLevel.HSM\] can be created in this
    /// location.
    #[prost(bool, tag = "1")]
    pub hsm_available: bool,
    /// Indicates whether \[CryptoKeys][google.cloud.kms.v1.CryptoKey\] with
    /// \[protection_level][google.cloud.kms.v1.CryptoKeyVersionTemplate.protection_level\]
    /// \[EXTERNAL][google.cloud.kms.v1.ProtectionLevel.EXTERNAL\] can be created in
    /// this location.
    #[prost(bool, tag = "2")]
    pub ekm_available: bool,
}
/// Generated client implementations.
pub mod key_management_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Google Cloud Key Management Service
    ///
    /// Manages cryptographic keys and operations using those keys. Implements a REST
    /// model with the following objects:
    ///
    /// * [KeyRing][google.cloud.kms.v1.KeyRing]
    /// * [CryptoKey][google.cloud.kms.v1.CryptoKey]
    /// * [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]
    /// * [ImportJob][google.cloud.kms.v1.ImportJob]
    ///
    /// If you are using manual gRPC libraries, see
    /// [Using gRPC with Cloud KMS](https://cloud.google.com/kms/docs/grpc).
    #[derive(Debug, Clone)]
    pub struct KeyManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl KeyManagementServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> KeyManagementServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> KeyManagementServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            KeyManagementServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Lists [KeyRings][google.cloud.kms.v1.KeyRing].
        pub async fn list_key_rings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKeyRingsRequest>,
        ) -> Result<tonic::Response<super::ListKeyRingsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/ListKeyRings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists [CryptoKeys][google.cloud.kms.v1.CryptoKey].
        pub async fn list_crypto_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCryptoKeysRequest>,
        ) -> Result<tonic::Response<super::ListCryptoKeysResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/ListCryptoKeys",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion].
        pub async fn list_crypto_key_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCryptoKeyVersionsRequest>,
        ) -> Result<
            tonic::Response<super::ListCryptoKeyVersionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/ListCryptoKeyVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists [ImportJobs][google.cloud.kms.v1.ImportJob].
        pub async fn list_import_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListImportJobsRequest>,
        ) -> Result<tonic::Response<super::ListImportJobsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/ListImportJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns metadata for a given [KeyRing][google.cloud.kms.v1.KeyRing].
        pub async fn get_key_ring(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeyRingRequest>,
        ) -> Result<tonic::Response<super::KeyRing>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/GetKeyRing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns metadata for a given [CryptoKey][google.cloud.kms.v1.CryptoKey], as
        /// well as its [primary][google.cloud.kms.v1.CryptoKey.primary]
        /// [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion].
        pub async fn get_crypto_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCryptoKeyRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/GetCryptoKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns metadata for a given
        /// [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion].
        pub async fn get_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/GetCryptoKeyVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the public key for the given
        /// [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]. The
        /// [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] must be
        /// [ASYMMETRIC_SIGN][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_SIGN]
        /// or
        /// [ASYMMETRIC_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_DECRYPT].
        pub async fn get_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPublicKeyRequest>,
        ) -> Result<tonic::Response<super::PublicKey>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/GetPublicKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns metadata for a given [ImportJob][google.cloud.kms.v1.ImportJob].
        pub async fn get_import_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetImportJobRequest>,
        ) -> Result<tonic::Response<super::ImportJob>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/GetImportJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a new [KeyRing][google.cloud.kms.v1.KeyRing] in a given Project and
        /// Location.
        pub async fn create_key_ring(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKeyRingRequest>,
        ) -> Result<tonic::Response<super::KeyRing>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/CreateKeyRing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a new [CryptoKey][google.cloud.kms.v1.CryptoKey] within a
        /// [KeyRing][google.cloud.kms.v1.KeyRing].
        ///
        /// [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] and
        /// [CryptoKey.version_template.algorithm][google.cloud.kms.v1.CryptoKeyVersionTemplate.algorithm]
        /// are required.
        pub async fn create_crypto_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCryptoKeyRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/CreateCryptoKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a new [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] in a
        /// [CryptoKey][google.cloud.kms.v1.CryptoKey].
        ///
        /// The server will assign the next sequential id. If unset,
        /// [state][google.cloud.kms.v1.CryptoKeyVersion.state] will be set to
        /// [ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED].
        pub async fn create_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/CreateCryptoKeyVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Import wrapped key material into a
        /// [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion].
        ///
        /// All requests must specify a [CryptoKey][google.cloud.kms.v1.CryptoKey]. If
        /// a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] is additionally
        /// specified in the request, key material will be reimported into that
        /// version. Otherwise, a new version will be created, and will be assigned the
        /// next sequential id within the [CryptoKey][google.cloud.kms.v1.CryptoKey].
        pub async fn import_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/ImportCryptoKeyVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a new [ImportJob][google.cloud.kms.v1.ImportJob] within a
        /// [KeyRing][google.cloud.kms.v1.KeyRing].
        ///
        /// [ImportJob.import_method][google.cloud.kms.v1.ImportJob.import_method] is
        /// required.
        pub async fn create_import_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateImportJobRequest>,
        ) -> Result<tonic::Response<super::ImportJob>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/CreateImportJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update a [CryptoKey][google.cloud.kms.v1.CryptoKey].
        pub async fn update_crypto_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCryptoKeyRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/UpdateCryptoKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]'s
        /// metadata.
        ///
        /// [state][google.cloud.kms.v1.CryptoKeyVersion.state] may be changed between
        /// [ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED]
        /// and
        /// [DISABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DISABLED]
        /// using this method. See
        /// [DestroyCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.DestroyCryptoKeyVersion]
        /// and
        /// [RestoreCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.RestoreCryptoKeyVersion]
        /// to move between other states.
        pub async fn update_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/UpdateCryptoKeyVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update the version of a [CryptoKey][google.cloud.kms.v1.CryptoKey] that
        /// will be used in
        /// [Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt].
        ///
        /// Returns an error if called on a key whose purpose is not
        /// [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT].
        pub async fn update_crypto_key_primary_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCryptoKeyPrimaryVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/UpdateCryptoKeyPrimaryVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Schedule a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] for
        /// destruction.
        ///
        /// Upon calling this method,
        /// [CryptoKeyVersion.state][google.cloud.kms.v1.CryptoKeyVersion.state] will
        /// be set to
        /// [DESTROY_SCHEDULED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROY_SCHEDULED],
        /// and [destroy_time][google.cloud.kms.v1.CryptoKeyVersion.destroy_time] will
        /// be set to the time
        /// [destroy_scheduled_duration][google.cloud.kms.v1.CryptoKey.destroy_scheduled_duration]
        /// in the future. At that time, the
        /// [state][google.cloud.kms.v1.CryptoKeyVersion.state] will automatically
        /// change to
        /// [DESTROYED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROYED],
        /// and the key material will be irrevocably destroyed.
        ///
        /// Before the
        /// [destroy_time][google.cloud.kms.v1.CryptoKeyVersion.destroy_time] is
        /// reached,
        /// [RestoreCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.RestoreCryptoKeyVersion]
        /// may be called to reverse the process.
        pub async fn destroy_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/DestroyCryptoKeyVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Restore a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] in the
        /// [DESTROY_SCHEDULED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROY_SCHEDULED]
        /// state.
        ///
        /// Upon restoration of the CryptoKeyVersion,
        /// [state][google.cloud.kms.v1.CryptoKeyVersion.state] will be set to
        /// [DISABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DISABLED],
        /// and [destroy_time][google.cloud.kms.v1.CryptoKeyVersion.destroy_time] will
        /// be cleared.
        pub async fn restore_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/RestoreCryptoKeyVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Encrypts data, so that it can only be recovered by a call to
        /// [Decrypt][google.cloud.kms.v1.KeyManagementService.Decrypt]. The
        /// [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] must be
        /// [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT].
        pub async fn encrypt(
            &mut self,
            request: impl tonic::IntoRequest<crate::proto_ext::kms::EncryptRequest>,
        ) -> Result<tonic::Response<super::EncryptResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/Encrypt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Decrypts data that was protected by
        /// [Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt]. The
        /// [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] must be
        /// [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT].
        pub async fn decrypt(
            &mut self,
            request: impl tonic::IntoRequest<super::DecryptRequest>,
        ) -> Result<
            tonic::Response<crate::proto_ext::kms::DecryptResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/Decrypt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Signs data using a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]
        /// with [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose]
        /// ASYMMETRIC_SIGN, producing a signature that can be verified with the public
        /// key retrieved from
        /// [GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey].
        pub async fn asymmetric_sign(
            &mut self,
            request: impl tonic::IntoRequest<super::AsymmetricSignRequest>,
        ) -> Result<tonic::Response<super::AsymmetricSignResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/AsymmetricSign",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Decrypts data that was encrypted with a public key retrieved from
        /// [GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey]
        /// corresponding to a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]
        /// with [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose]
        /// ASYMMETRIC_DECRYPT.
        pub async fn asymmetric_decrypt(
            &mut self,
            request: impl tonic::IntoRequest<super::AsymmetricDecryptRequest>,
        ) -> Result<tonic::Response<super::AsymmetricDecryptResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/AsymmetricDecrypt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Signs data using a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]
        /// with [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] MAC,
        /// producing a tag that can be verified by another source with the same key.
        pub async fn mac_sign(
            &mut self,
            request: impl tonic::IntoRequest<super::MacSignRequest>,
        ) -> Result<tonic::Response<super::MacSignResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/MacSign",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Verifies MAC tag using a
        /// [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] with
        /// [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] MAC, and returns
        /// a response that indicates whether or not the verification was successful.
        pub async fn mac_verify(
            &mut self,
            request: impl tonic::IntoRequest<super::MacVerifyRequest>,
        ) -> Result<tonic::Response<super::MacVerifyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/MacVerify",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Generate random bytes using the Cloud KMS randomness source in the provided
        /// location.
        pub async fn generate_random_bytes(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateRandomBytesRequest>,
        ) -> Result<tonic::Response<super::GenerateRandomBytesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.kms.v1.KeyManagementService/GenerateRandomBytes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
