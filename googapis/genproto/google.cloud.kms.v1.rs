/// A [KeyRing][google.cloud.kms.v1.KeyRing] is a toplevel logical grouping of [CryptoKeys][google.cloud.kms.v1.CryptoKey].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRing {
    /// Output only. The resource name for the [KeyRing][google.cloud.kms.v1.KeyRing] in the format
    /// `projects/*/locations/*/keyRings/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The time at which this [KeyRing][google.cloud.kms.v1.KeyRing] was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// A [CryptoKey][google.cloud.kms.v1.CryptoKey] represents a logical key that can be used for cryptographic
/// operations.
///
/// A [CryptoKey][google.cloud.kms.v1.CryptoKey] is made up of one or more [versions][google.cloud.kms.v1.CryptoKeyVersion], which
/// represent the actual key material used in cryptographic operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKey {
    /// Output only. The resource name for this [CryptoKey][google.cloud.kms.v1.CryptoKey] in the format
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. A copy of the "primary" [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] that will be used
    /// by [Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt] when this [CryptoKey][google.cloud.kms.v1.CryptoKey] is given
    /// in [EncryptRequest.name][google.cloud.kms.v1.EncryptRequest.name].
    ///
    /// The [CryptoKey][google.cloud.kms.v1.CryptoKey]'s primary version can be updated via
    /// [UpdateCryptoKeyPrimaryVersion][google.cloud.kms.v1.KeyManagementService.UpdateCryptoKeyPrimaryVersion].
    ///
    /// Keys with [purpose][google.cloud.kms.v1.CryptoKey.purpose]
    /// [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT] may have a
    /// primary. For other keys, this field will be omitted.
    #[prost(message, optional, tag = "2")]
    pub primary: ::std::option::Option<CryptoKeyVersion>,
    /// Immutable. The immutable purpose of this [CryptoKey][google.cloud.kms.v1.CryptoKey].
    #[prost(enumeration = "crypto_key::CryptoKeyPurpose", tag = "3")]
    pub purpose: i32,
    /// Output only. The time at which this [CryptoKey][google.cloud.kms.v1.CryptoKey] was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// At [next_rotation_time][google.cloud.kms.v1.CryptoKey.next_rotation_time], the Key Management Service will automatically:
    ///
    /// 1. Create a new version of this [CryptoKey][google.cloud.kms.v1.CryptoKey].
    /// 2. Mark the new version as primary.
    ///
    /// Key rotations performed manually via
    /// [CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion] and
    /// [UpdateCryptoKeyPrimaryVersion][google.cloud.kms.v1.KeyManagementService.UpdateCryptoKeyPrimaryVersion]
    /// do not affect [next_rotation_time][google.cloud.kms.v1.CryptoKey.next_rotation_time].
    ///
    /// Keys with [purpose][google.cloud.kms.v1.CryptoKey.purpose]
    /// [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT] support
    /// automatic rotation. For other keys, this field must be omitted.
    #[prost(message, optional, tag = "7")]
    pub next_rotation_time: ::std::option::Option<::prost_types::Timestamp>,
    /// A template describing settings for new [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] instances.
    /// The properties of new [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] instances created by either
    /// [CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion] or
    /// auto-rotation are controlled by this template.
    #[prost(message, optional, tag = "11")]
    pub version_template: ::std::option::Option<CryptoKeyVersionTemplate>,
    /// Labels with user-defined metadata. For more information, see
    /// [Labeling Keys](https://cloud.google.com/kms/docs/labeling-keys).
    #[prost(map = "string, string", tag = "10")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Controls the rate of automatic rotation.
    #[prost(oneof = "crypto_key::RotationSchedule", tags = "8")]
    pub rotation_schedule: ::std::option::Option<crypto_key::RotationSchedule>,
}
pub mod crypto_key {
    /// [CryptoKeyPurpose][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose] describes the cryptographic capabilities of a
    /// [CryptoKey][google.cloud.kms.v1.CryptoKey]. A given key can only be used for the operations allowed by
    /// its purpose. For more information, see
    /// [Key purposes](https://cloud.google.com/kms/docs/algorithms#key_purposes).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CryptoKeyPurpose {
        /// Not specified.
        Unspecified = 0,
        /// [CryptoKeys][google.cloud.kms.v1.CryptoKey] with this purpose may be used with
        /// [Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt] and
        /// [Decrypt][google.cloud.kms.v1.KeyManagementService.Decrypt].
        EncryptDecrypt = 1,
        /// [CryptoKeys][google.cloud.kms.v1.CryptoKey] with this purpose may be used with
        /// [AsymmetricSign][google.cloud.kms.v1.KeyManagementService.AsymmetricSign] and
        /// [GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey].
        AsymmetricSign = 5,
        /// [CryptoKeys][google.cloud.kms.v1.CryptoKey] with this purpose may be used with
        /// [AsymmetricDecrypt][google.cloud.kms.v1.KeyManagementService.AsymmetricDecrypt] and
        /// [GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey].
        AsymmetricDecrypt = 6,
    }
    /// Controls the rate of automatic rotation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RotationSchedule {
        /// [next_rotation_time][google.cloud.kms.v1.CryptoKey.next_rotation_time] will be advanced by this period when the service
        /// automatically rotates a key. Must be at least 24 hours and at most
        /// 876,000 hours.
        ///
        /// If [rotation_period][google.cloud.kms.v1.CryptoKey.rotation_period] is set, [next_rotation_time][google.cloud.kms.v1.CryptoKey.next_rotation_time] must also be set.
        ///
        /// Keys with [purpose][google.cloud.kms.v1.CryptoKey.purpose]
        /// [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT] support
        /// automatic rotation. For other keys, this field must be omitted.
        #[prost(message, tag = "8")]
        RotationPeriod(::prost_types::Duration),
    }
}
/// A [CryptoKeyVersionTemplate][google.cloud.kms.v1.CryptoKeyVersionTemplate] specifies the properties to use when creating
/// a new [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion], either manually with
/// [CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion] or
/// automatically as a result of auto-rotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKeyVersionTemplate {
    /// [ProtectionLevel][google.cloud.kms.v1.ProtectionLevel] to use when creating a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] based on
    /// this template. Immutable. Defaults to [SOFTWARE][google.cloud.kms.v1.ProtectionLevel.SOFTWARE].
    #[prost(enumeration = "ProtectionLevel", tag = "1")]
    pub protection_level: i32,
    /// Required. [Algorithm][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm] to use
    /// when creating a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] based on this template.
    ///
    /// For backwards compatibility, GOOGLE_SYMMETRIC_ENCRYPTION is implied if both
    /// this field is omitted and [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] is
    /// [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT].
    #[prost(
        enumeration = "crypto_key_version::CryptoKeyVersionAlgorithm",
        tag = "3"
    )]
    pub algorithm: i32,
}
/// Contains an HSM-generated attestation about a key operation. For more
/// information, see [Verifying attestations]
/// (https://cloud.google.com/kms/docs/attest-key).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyOperationAttestation {
    /// Output only. The format of the attestation data.
    #[prost(
        enumeration = "key_operation_attestation::AttestationFormat",
        tag = "4"
    )]
    pub format: i32,
    /// Output only. The attestation data provided by the HSM when the key
    /// operation was performed.
    #[prost(bytes, tag = "5")]
    pub content: std::vec::Vec<u8>,
}
pub mod key_operation_attestation {
    /// Attestation formats provided by the HSM.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AttestationFormat {
        /// Not specified.
        Unspecified = 0,
        /// Cavium HSM attestation compressed with gzip. Note that this format is
        /// defined by Cavium and subject to change at any time.
        CaviumV1Compressed = 3,
        /// Cavium HSM attestation V2 compressed with gzip. This is a new format
        /// introduced in Cavium's version 3.2-08.
        CaviumV2Compressed = 4,
    }
}
/// A [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] represents an individual cryptographic key, and the
/// associated key material.
///
/// An [ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED] version can be
/// used for cryptographic operations.
///
/// For security reasons, the raw cryptographic key material represented by a
/// [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] can never be viewed or exported. It can only be used to
/// encrypt, decrypt, or sign data when an authorized user or application invokes
/// Cloud KMS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKeyVersion {
    /// Output only. The resource name for this [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] in the format
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The current state of the [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion].
    #[prost(enumeration = "crypto_key_version::CryptoKeyVersionState", tag = "3")]
    pub state: i32,
    /// Output only. The [ProtectionLevel][google.cloud.kms.v1.ProtectionLevel] describing how crypto operations are
    /// performed with this [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion].
    #[prost(enumeration = "ProtectionLevel", tag = "7")]
    pub protection_level: i32,
    /// Output only. The [CryptoKeyVersionAlgorithm][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm] that this
    /// [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] supports.
    #[prost(
        enumeration = "crypto_key_version::CryptoKeyVersionAlgorithm",
        tag = "10"
    )]
    pub algorithm: i32,
    /// Output only. Statement that was generated and signed by the HSM at key
    /// creation time. Use this statement to verify attributes of the key as stored
    /// on the HSM, independently of Google. Only provided for key versions with
    /// [protection_level][google.cloud.kms.v1.CryptoKeyVersion.protection_level] [HSM][google.cloud.kms.v1.ProtectionLevel.HSM].
    #[prost(message, optional, tag = "8")]
    pub attestation: ::std::option::Option<KeyOperationAttestation>,
    /// Output only. The time at which this [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]'s key material was
    /// generated.
    #[prost(message, optional, tag = "11")]
    pub generate_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]'s key material is scheduled
    /// for destruction. Only present if [state][google.cloud.kms.v1.CryptoKeyVersion.state] is
    /// [DESTROY_SCHEDULED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROY_SCHEDULED].
    #[prost(message, optional, tag = "5")]
    pub destroy_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this CryptoKeyVersion's key material was
    /// destroyed. Only present if [state][google.cloud.kms.v1.CryptoKeyVersion.state] is
    /// [DESTROYED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROYED].
    #[prost(message, optional, tag = "6")]
    pub destroy_event_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The name of the [ImportJob][google.cloud.kms.v1.ImportJob] used to import this
    /// [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]. Only present if the underlying key material was
    /// imported.
    #[prost(string, tag = "14")]
    pub import_job: std::string::String,
    /// Output only. The time at which this [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]'s key material
    /// was imported.
    #[prost(message, optional, tag = "15")]
    pub import_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The root cause of an import failure. Only present if
    /// [state][google.cloud.kms.v1.CryptoKeyVersion.state] is
    /// [IMPORT_FAILED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.IMPORT_FAILED].
    #[prost(string, tag = "16")]
    pub import_failure_reason: std::string::String,
    /// ExternalProtectionLevelOptions stores a group of additional fields for
    /// configuring a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] that are specific to the
    /// [EXTERNAL][google.cloud.kms.v1.ProtectionLevel.EXTERNAL] protection level.
    #[prost(message, optional, tag = "17")]
    pub external_protection_level_options: ::std::option::Option<ExternalProtectionLevelOptions>,
}
pub mod crypto_key_version {
    /// The algorithm of the [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion], indicating what
    /// parameters must be used for each cryptographic operation.
    ///
    /// The
    /// [GOOGLE_SYMMETRIC_ENCRYPTION][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm.GOOGLE_SYMMETRIC_ENCRYPTION]
    /// algorithm is usable with [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose]
    /// [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT].
    ///
    /// Algorithms beginning with "RSA_SIGN_" are usable with [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose]
    /// [ASYMMETRIC_SIGN][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_SIGN].
    ///
    /// The fields in the name after "RSA_SIGN_" correspond to the following
    /// parameters: padding algorithm, modulus bit length, and digest algorithm.
    ///
    /// For PSS, the salt length used is equal to the length of digest
    /// algorithm. For example,
    /// [RSA_SIGN_PSS_2048_SHA256][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm.RSA_SIGN_PSS_2048_SHA256]
    /// will use PSS with a salt length of 256 bits or 32 bytes.
    ///
    /// Algorithms beginning with "RSA_DECRYPT_" are usable with
    /// [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose]
    /// [ASYMMETRIC_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_DECRYPT].
    ///
    /// The fields in the name after "RSA_DECRYPT_" correspond to the following
    /// parameters: padding algorithm, modulus bit length, and digest algorithm.
    ///
    /// Algorithms beginning with "EC_SIGN_" are usable with [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose]
    /// [ASYMMETRIC_SIGN][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_SIGN].
    ///
    /// The fields in the name after "EC_SIGN_" correspond to the following
    /// parameters: elliptic curve, digest algorithm.
    ///
    /// For more information, see [Key purposes and algorithms]
    /// (https://cloud.google.com/kms/docs/algorithms).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
        /// RSAES-OAEP 2048 bit key with a SHA256 digest.
        RsaDecryptOaep2048Sha256 = 8,
        /// RSAES-OAEP 3072 bit key with a SHA256 digest.
        RsaDecryptOaep3072Sha256 = 9,
        /// RSAES-OAEP 4096 bit key with a SHA256 digest.
        RsaDecryptOaep4096Sha256 = 10,
        /// RSAES-OAEP 4096 bit key with a SHA512 digest.
        RsaDecryptOaep4096Sha512 = 17,
        /// ECDSA on the NIST P-256 curve with a SHA256 digest.
        EcSignP256Sha256 = 12,
        /// ECDSA on the NIST P-384 curve with a SHA384 digest.
        EcSignP384Sha384 = 13,
        /// Algorithm representing symmetric encryption by an external key manager.
        ExternalSymmetricEncryption = 18,
    }
    /// The state of a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion], indicating if it can be used.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CryptoKeyVersionState {
        /// Not specified.
        Unspecified = 0,
        /// This version is still being generated. It may not be used, enabled,
        /// disabled, or destroyed yet. Cloud KMS will automatically mark this
        /// version [ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED] as soon as the version is ready.
        PendingGeneration = 5,
        /// This version may be used for cryptographic operations.
        Enabled = 1,
        /// This version may not be used, but the key material is still available,
        /// and the version can be placed back into the [ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED] state.
        Disabled = 2,
        /// This version is destroyed, and the key material is no longer stored.
        /// A version may not leave this state once entered.
        Destroyed = 3,
        /// This version is scheduled for destruction, and will be destroyed soon.
        /// Call
        /// [RestoreCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.RestoreCryptoKeyVersion]
        /// to put it back into the [DISABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DISABLED] state.
        DestroyScheduled = 4,
        /// This version is still being imported. It may not be used, enabled,
        /// disabled, or destroyed yet. Cloud KMS will automatically mark this
        /// version [ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED] as soon as the version is ready.
        PendingImport = 6,
        /// This version was not imported successfully. It may not be used, enabled,
        /// disabled, or destroyed. The submitted key material has been discarded.
        /// Additional details can be found in
        /// [CryptoKeyVersion.import_failure_reason][google.cloud.kms.v1.CryptoKeyVersion.import_failure_reason].
        ImportFailed = 7,
    }
    /// A view for [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]s. Controls the level of detail returned
    /// for [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion] in
    /// [KeyManagementService.ListCryptoKeyVersions][google.cloud.kms.v1.KeyManagementService.ListCryptoKeyVersions] and
    /// [KeyManagementService.ListCryptoKeys][google.cloud.kms.v1.KeyManagementService.ListCryptoKeys].
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CryptoKeyVersionView {
        /// Default view for each [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]. Does not include
        /// the [attestation][google.cloud.kms.v1.CryptoKeyVersion.attestation] field.
        Unspecified = 0,
        /// Provides all fields in each [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion], including the
        /// [attestation][google.cloud.kms.v1.CryptoKeyVersion.attestation].
        Full = 1,
    }
}
/// The public key for a given [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]. Obtained via
/// [GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    /// The public key, encoded in PEM format. For more information, see the
    /// [RFC 7468](https://tools.ietf.org/html/rfc7468) sections for
    /// [General Considerations](https://tools.ietf.org/html/rfc7468#section-2) and
    /// [Textual Encoding of Subject Public Key Info]
    /// (https://tools.ietf.org/html/rfc7468#section-13).
    #[prost(string, tag = "1")]
    pub pem: std::string::String,
    /// The [Algorithm][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm] associated
    /// with this key.
    #[prost(
        enumeration = "crypto_key_version::CryptoKeyVersionAlgorithm",
        tag = "2"
    )]
    pub algorithm: i32,
}
/// An [ImportJob][google.cloud.kms.v1.ImportJob] can be used to create [CryptoKeys][google.cloud.kms.v1.CryptoKey] and
/// [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion] using pre-existing key material,
/// generated outside of Cloud KMS.
///
/// When an [ImportJob][google.cloud.kms.v1.ImportJob] is created, Cloud KMS will generate a "wrapping key",
/// which is a public/private key pair. You use the wrapping key to encrypt (also
/// known as wrap) the pre-existing key material to protect it during the import
/// process. The nature of the wrapping key depends on the choice of
/// [import_method][google.cloud.kms.v1.ImportJob.import_method]. When the wrapping key generation
/// is complete, the [state][google.cloud.kms.v1.ImportJob.state] will be set to
/// [ACTIVE][google.cloud.kms.v1.ImportJob.ImportJobState.ACTIVE] and the [public_key][google.cloud.kms.v1.ImportJob.public_key]
/// can be fetched. The fetched public key can then be used to wrap your
/// pre-existing key material.
///
/// Once the key material is wrapped, it can be imported into a new
/// [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] in an existing [CryptoKey][google.cloud.kms.v1.CryptoKey] by calling
/// [ImportCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.ImportCryptoKeyVersion].
/// Multiple [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion] can be imported with a single
/// [ImportJob][google.cloud.kms.v1.ImportJob]. Cloud KMS uses the private key portion of the wrapping key to
/// unwrap the key material. Only Cloud KMS has access to the private key.
///
/// An [ImportJob][google.cloud.kms.v1.ImportJob] expires 3 days after it is created. Once expired, Cloud KMS
/// will no longer be able to import or unwrap any key material that was wrapped
/// with the [ImportJob][google.cloud.kms.v1.ImportJob]'s public key.
///
/// For more information, see
/// [Importing a key](https://cloud.google.com/kms/docs/importing-a-key).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportJob {
    /// Output only. The resource name for this [ImportJob][google.cloud.kms.v1.ImportJob] in the format
    /// `projects/*/locations/*/keyRings/*/importJobs/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Immutable. The wrapping method to be used for incoming key material.
    #[prost(enumeration = "import_job::ImportMethod", tag = "2")]
    pub import_method: i32,
    /// Required. Immutable. The protection level of the [ImportJob][google.cloud.kms.v1.ImportJob]. This must match the
    /// [protection_level][google.cloud.kms.v1.CryptoKeyVersionTemplate.protection_level] of the
    /// [version_template][google.cloud.kms.v1.CryptoKey.version_template] on the [CryptoKey][google.cloud.kms.v1.CryptoKey] you
    /// attempt to import into.
    #[prost(enumeration = "ProtectionLevel", tag = "9")]
    pub protection_level: i32,
    /// Output only. The time at which this [ImportJob][google.cloud.kms.v1.ImportJob] was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this [ImportJob][google.cloud.kms.v1.ImportJob]'s key material was generated.
    #[prost(message, optional, tag = "4")]
    pub generate_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this [ImportJob][google.cloud.kms.v1.ImportJob] is scheduled for
    /// expiration and can no longer be used to import key material.
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this [ImportJob][google.cloud.kms.v1.ImportJob] expired. Only present if
    /// [state][google.cloud.kms.v1.ImportJob.state] is [EXPIRED][google.cloud.kms.v1.ImportJob.ImportJobState.EXPIRED].
    #[prost(message, optional, tag = "10")]
    pub expire_event_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the [ImportJob][google.cloud.kms.v1.ImportJob], indicating if it can
    /// be used.
    #[prost(enumeration = "import_job::ImportJobState", tag = "6")]
    pub state: i32,
    /// Output only. The public key with which to wrap key material prior to
    /// import. Only returned if [state][google.cloud.kms.v1.ImportJob.state] is
    /// [ACTIVE][google.cloud.kms.v1.ImportJob.ImportJobState.ACTIVE].
    #[prost(message, optional, tag = "7")]
    pub public_key: ::std::option::Option<import_job::WrappingPublicKey>,
    /// Output only. Statement that was generated and signed by the key creator
    /// (for example, an HSM) at key creation time. Use this statement to verify
    /// attributes of the key as stored on the HSM, independently of Google.
    /// Only present if the chosen [ImportMethod][google.cloud.kms.v1.ImportJob.ImportMethod] is one with a protection
    /// level of [HSM][google.cloud.kms.v1.ProtectionLevel.HSM].
    #[prost(message, optional, tag = "8")]
    pub attestation: ::std::option::Option<KeyOperationAttestation>,
}
pub mod import_job {
    /// The public key component of the wrapping key. For details of the type of
    /// key this public key corresponds to, see the [ImportMethod][google.cloud.kms.v1.ImportJob.ImportMethod].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WrappingPublicKey {
        /// The public key, encoded in PEM format. For more information, see the [RFC
        /// 7468](https://tools.ietf.org/html/rfc7468) sections for [General
        /// Considerations](https://tools.ietf.org/html/rfc7468#section-2) and
        /// [Textual Encoding of Subject Public Key Info]
        /// (https://tools.ietf.org/html/rfc7468#section-13).
        #[prost(string, tag = "1")]
        pub pem: std::string::String,
    }
    /// [ImportMethod][google.cloud.kms.v1.ImportJob.ImportMethod] describes the key wrapping method chosen for this
    /// [ImportJob][google.cloud.kms.v1.ImportJob].
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ImportMethod {
        /// Not specified.
        Unspecified = 0,
        /// This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping
        /// scheme defined in the PKCS #11 standard. In summary, this involves
        /// wrapping the raw key with an ephemeral AES key, and wrapping the
        /// ephemeral AES key with a 3072 bit RSA key. For more details, see
        /// [RSA AES key wrap
        /// mechanism](http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908).
        RsaOaep3072Sha1Aes256 = 1,
        /// This ImportMethod represents the CKM_RSA_AES_KEY_WRAP key wrapping
        /// scheme defined in the PKCS #11 standard. In summary, this involves
        /// wrapping the raw key with an ephemeral AES key, and wrapping the
        /// ephemeral AES key with a 4096 bit RSA key. For more details, see
        /// [RSA AES key wrap
        /// mechanism](http://docs.oasis-open.org/pkcs11/pkcs11-curr/v2.40/cos01/pkcs11-curr-v2.40-cos01.html#_Toc408226908).
        RsaOaep4096Sha1Aes256 = 2,
    }
    /// The state of the [ImportJob][google.cloud.kms.v1.ImportJob], indicating if it can be used.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ImportJobState {
        /// Not specified.
        Unspecified = 0,
        /// The wrapping key for this job is still being generated. It may not be
        /// used. Cloud KMS will automatically mark this job as
        /// [ACTIVE][google.cloud.kms.v1.ImportJob.ImportJobState.ACTIVE] as soon as the wrapping key is generated.
        PendingGeneration = 1,
        /// This job may be used in
        /// [CreateCryptoKey][google.cloud.kms.v1.KeyManagementService.CreateCryptoKey] and
        /// [CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion]
        /// requests.
        Active = 2,
        /// This job can no longer be used and may not leave this state once entered.
        Expired = 3,
    }
}
/// ExternalProtectionLevelOptions stores a group of additional fields for
/// configuring a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] that are specific to the
/// [EXTERNAL][google.cloud.kms.v1.ProtectionLevel.EXTERNAL] protection level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalProtectionLevelOptions {
    /// The URI for an external resource that this [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] represents.
    #[prost(string, tag = "1")]
    pub external_key_uri: std::string::String,
}
/// [ProtectionLevel][google.cloud.kms.v1.ProtectionLevel] specifies how cryptographic operations are performed.
/// For more information, see [Protection levels]
/// (https://cloud.google.com/kms/docs/algorithms#protection_levels).
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
}
/// Request message for [KeyManagementService.ListKeyRings][google.cloud.kms.v1.KeyManagementService.ListKeyRings].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeyRingsRequest {
    /// Required. The resource name of the location associated with the
    /// [KeyRings][google.cloud.kms.v1.KeyRing], in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Optional limit on the number of [KeyRings][google.cloud.kms.v1.KeyRing] to include in the
    /// response.  Further [KeyRings][google.cloud.kms.v1.KeyRing] can subsequently be obtained by
    /// including the [ListKeyRingsResponse.next_page_token][google.cloud.kms.v1.ListKeyRingsResponse.next_page_token] in a subsequent
    /// request.  If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Optional pagination token, returned earlier via
    /// [ListKeyRingsResponse.next_page_token][google.cloud.kms.v1.ListKeyRingsResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. Only include resources that match the filter in the response. For
    /// more information, see
    /// [Sorting and filtering list
    /// results](https://cloud.google.com/kms/docs/sorting-and-filtering).
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specify how the results should be sorted. If not specified, the
    /// results will be sorted in the default order.  For more information, see
    /// [Sorting and filtering list
    /// results](https://cloud.google.com/kms/docs/sorting-and-filtering).
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Request message for [KeyManagementService.ListCryptoKeys][google.cloud.kms.v1.KeyManagementService.ListCryptoKeys].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCryptoKeysRequest {
    /// Required. The resource name of the [KeyRing][google.cloud.kms.v1.KeyRing] to list, in the format
    /// `projects/*/locations/*/keyRings/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Optional limit on the number of [CryptoKeys][google.cloud.kms.v1.CryptoKey] to include in the
    /// response.  Further [CryptoKeys][google.cloud.kms.v1.CryptoKey] can subsequently be obtained by
    /// including the [ListCryptoKeysResponse.next_page_token][google.cloud.kms.v1.ListCryptoKeysResponse.next_page_token] in a subsequent
    /// request.  If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Optional pagination token, returned earlier via
    /// [ListCryptoKeysResponse.next_page_token][google.cloud.kms.v1.ListCryptoKeysResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// The fields of the primary version to include in the response.
    #[prost(enumeration = "crypto_key_version::CryptoKeyVersionView", tag = "4")]
    pub version_view: i32,
    /// Optional. Only include resources that match the filter in the response. For
    /// more information, see
    /// [Sorting and filtering list
    /// results](https://cloud.google.com/kms/docs/sorting-and-filtering).
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// Optional. Specify how the results should be sorted. If not specified, the
    /// results will be sorted in the default order. For more information, see
    /// [Sorting and filtering list
    /// results](https://cloud.google.com/kms/docs/sorting-and-filtering).
    #[prost(string, tag = "6")]
    pub order_by: std::string::String,
}
/// Request message for [KeyManagementService.ListCryptoKeyVersions][google.cloud.kms.v1.KeyManagementService.ListCryptoKeyVersions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCryptoKeyVersionsRequest {
    /// Required. The resource name of the [CryptoKey][google.cloud.kms.v1.CryptoKey] to list, in the format
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Optional limit on the number of [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion] to
    /// include in the response. Further [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion] can
    /// subsequently be obtained by including the
    /// [ListCryptoKeyVersionsResponse.next_page_token][google.cloud.kms.v1.ListCryptoKeyVersionsResponse.next_page_token] in a subsequent request.
    /// If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Optional pagination token, returned earlier via
    /// [ListCryptoKeyVersionsResponse.next_page_token][google.cloud.kms.v1.ListCryptoKeyVersionsResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// The fields to include in the response.
    #[prost(enumeration = "crypto_key_version::CryptoKeyVersionView", tag = "4")]
    pub view: i32,
    /// Optional. Only include resources that match the filter in the response. For
    /// more information, see
    /// [Sorting and filtering list
    /// results](https://cloud.google.com/kms/docs/sorting-and-filtering).
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// Optional. Specify how the results should be sorted. If not specified, the
    /// results will be sorted in the default order. For more information, see
    /// [Sorting and filtering list
    /// results](https://cloud.google.com/kms/docs/sorting-and-filtering).
    #[prost(string, tag = "6")]
    pub order_by: std::string::String,
}
/// Request message for [KeyManagementService.ListImportJobs][google.cloud.kms.v1.KeyManagementService.ListImportJobs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImportJobsRequest {
    /// Required. The resource name of the [KeyRing][google.cloud.kms.v1.KeyRing] to list, in the format
    /// `projects/*/locations/*/keyRings/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Optional limit on the number of [ImportJobs][google.cloud.kms.v1.ImportJob] to include in the
    /// response. Further [ImportJobs][google.cloud.kms.v1.ImportJob] can subsequently be obtained by
    /// including the [ListImportJobsResponse.next_page_token][google.cloud.kms.v1.ListImportJobsResponse.next_page_token] in a subsequent
    /// request. If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Optional pagination token, returned earlier via
    /// [ListImportJobsResponse.next_page_token][google.cloud.kms.v1.ListImportJobsResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. Only include resources that match the filter in the response. For
    /// more information, see
    /// [Sorting and filtering list
    /// results](https://cloud.google.com/kms/docs/sorting-and-filtering).
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specify how the results should be sorted. If not specified, the
    /// results will be sorted in the default order. For more information, see
    /// [Sorting and filtering list
    /// results](https://cloud.google.com/kms/docs/sorting-and-filtering).
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Response message for [KeyManagementService.ListKeyRings][google.cloud.kms.v1.KeyManagementService.ListKeyRings].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeyRingsResponse {
    /// The list of [KeyRings][google.cloud.kms.v1.KeyRing].
    #[prost(message, repeated, tag = "1")]
    pub key_rings: ::std::vec::Vec<KeyRing>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListKeyRingsRequest.page_token][google.cloud.kms.v1.ListKeyRingsRequest.page_token] to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// The total number of [KeyRings][google.cloud.kms.v1.KeyRing] that matched the query.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Response message for [KeyManagementService.ListCryptoKeys][google.cloud.kms.v1.KeyManagementService.ListCryptoKeys].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCryptoKeysResponse {
    /// The list of [CryptoKeys][google.cloud.kms.v1.CryptoKey].
    #[prost(message, repeated, tag = "1")]
    pub crypto_keys: ::std::vec::Vec<CryptoKey>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListCryptoKeysRequest.page_token][google.cloud.kms.v1.ListCryptoKeysRequest.page_token] to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// The total number of [CryptoKeys][google.cloud.kms.v1.CryptoKey] that matched the query.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Response message for [KeyManagementService.ListCryptoKeyVersions][google.cloud.kms.v1.KeyManagementService.ListCryptoKeyVersions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCryptoKeyVersionsResponse {
    /// The list of [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion].
    #[prost(message, repeated, tag = "1")]
    pub crypto_key_versions: ::std::vec::Vec<CryptoKeyVersion>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListCryptoKeyVersionsRequest.page_token][google.cloud.kms.v1.ListCryptoKeyVersionsRequest.page_token] to retrieve the next page of
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// The total number of [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion] that matched the
    /// query.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Response message for [KeyManagementService.ListImportJobs][google.cloud.kms.v1.KeyManagementService.ListImportJobs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImportJobsResponse {
    /// The list of [ImportJobs][google.cloud.kms.v1.ImportJob].
    #[prost(message, repeated, tag = "1")]
    pub import_jobs: ::std::vec::Vec<ImportJob>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListImportJobsRequest.page_token][google.cloud.kms.v1.ListImportJobsRequest.page_token] to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// The total number of [ImportJobs][google.cloud.kms.v1.ImportJob] that matched the query.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for [KeyManagementService.GetKeyRing][google.cloud.kms.v1.KeyManagementService.GetKeyRing].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyRingRequest {
    /// Required. The [name][google.cloud.kms.v1.KeyRing.name] of the [KeyRing][google.cloud.kms.v1.KeyRing] to get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [KeyManagementService.GetCryptoKey][google.cloud.kms.v1.KeyManagementService.GetCryptoKey].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCryptoKeyRequest {
    /// Required. The [name][google.cloud.kms.v1.CryptoKey.name] of the [CryptoKey][google.cloud.kms.v1.CryptoKey] to get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [KeyManagementService.GetCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.GetCryptoKeyVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCryptoKeyVersionRequest {
    /// Required. The [name][google.cloud.kms.v1.CryptoKeyVersion.name] of the [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] to get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [KeyManagementService.GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPublicKeyRequest {
    /// Required. The [name][google.cloud.kms.v1.CryptoKeyVersion.name] of the [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] public key to
    /// get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [KeyManagementService.GetImportJob][google.cloud.kms.v1.KeyManagementService.GetImportJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImportJobRequest {
    /// Required. The [name][google.cloud.kms.v1.ImportJob.name] of the [ImportJob][google.cloud.kms.v1.ImportJob] to get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [KeyManagementService.CreateKeyRing][google.cloud.kms.v1.KeyManagementService.CreateKeyRing].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyRingRequest {
    /// Required. The resource name of the location associated with the
    /// [KeyRings][google.cloud.kms.v1.KeyRing], in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. It must be unique within a location and match the regular
    /// expression `[a-zA-Z0-9_-]{1,63}`
    #[prost(string, tag = "2")]
    pub key_ring_id: std::string::String,
    /// Required. A [KeyRing][google.cloud.kms.v1.KeyRing] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub key_ring: ::std::option::Option<KeyRing>,
}
/// Request message for [KeyManagementService.CreateCryptoKey][google.cloud.kms.v1.KeyManagementService.CreateCryptoKey].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCryptoKeyRequest {
    /// Required. The [name][google.cloud.kms.v1.KeyRing.name] of the KeyRing associated with the
    /// [CryptoKeys][google.cloud.kms.v1.CryptoKey].
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. It must be unique within a KeyRing and match the regular
    /// expression `[a-zA-Z0-9_-]{1,63}`
    #[prost(string, tag = "2")]
    pub crypto_key_id: std::string::String,
    /// Required. A [CryptoKey][google.cloud.kms.v1.CryptoKey] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub crypto_key: ::std::option::Option<CryptoKey>,
    /// If set to true, the request will create a [CryptoKey][google.cloud.kms.v1.CryptoKey] without any
    /// [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion]. You must manually call
    /// [CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion] or
    /// [ImportCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.ImportCryptoKeyVersion]
    /// before you can use this [CryptoKey][google.cloud.kms.v1.CryptoKey].
    #[prost(bool, tag = "5")]
    pub skip_initial_version_creation: bool,
}
/// Request message for [KeyManagementService.CreateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.CreateCryptoKeyVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCryptoKeyVersionRequest {
    /// Required. The [name][google.cloud.kms.v1.CryptoKey.name] of the [CryptoKey][google.cloud.kms.v1.CryptoKey] associated with
    /// the [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion].
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. A [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] with initial field values.
    #[prost(message, optional, tag = "2")]
    pub crypto_key_version: ::std::option::Option<CryptoKeyVersion>,
}
/// Request message for [KeyManagementService.ImportCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.ImportCryptoKeyVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportCryptoKeyVersionRequest {
    /// Required. The [name][google.cloud.kms.v1.CryptoKey.name] of the [CryptoKey][google.cloud.kms.v1.CryptoKey] to
    /// be imported into.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The [algorithm][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionAlgorithm] of
    /// the key being imported. This does not need to match the
    /// [version_template][google.cloud.kms.v1.CryptoKey.version_template] of the [CryptoKey][google.cloud.kms.v1.CryptoKey] this
    /// version imports into.
    #[prost(
        enumeration = "crypto_key_version::CryptoKeyVersionAlgorithm",
        tag = "2"
    )]
    pub algorithm: i32,
    /// Required. The [name][google.cloud.kms.v1.ImportJob.name] of the [ImportJob][google.cloud.kms.v1.ImportJob] that was used to
    /// wrap this key material.
    #[prost(string, tag = "4")]
    pub import_job: std::string::String,
    /// Required. The incoming wrapped key material that is to be imported.
    #[prost(
        oneof = "import_crypto_key_version_request::WrappedKeyMaterial",
        tags = "5"
    )]
    pub wrapped_key_material:
        ::std::option::Option<import_crypto_key_version_request::WrappedKeyMaterial>,
}
pub mod import_crypto_key_version_request {
    /// Required. The incoming wrapped key material that is to be imported.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WrappedKeyMaterial {
        /// Wrapped key material produced with
        /// [RSA_OAEP_3072_SHA1_AES_256][google.cloud.kms.v1.ImportJob.ImportMethod.RSA_OAEP_3072_SHA1_AES_256]
        /// or
        /// [RSA_OAEP_4096_SHA1_AES_256][google.cloud.kms.v1.ImportJob.ImportMethod.RSA_OAEP_4096_SHA1_AES_256].
        ///
        /// This field contains the concatenation of two wrapped keys:
        /// <ol>
        ///   <li>An ephemeral AES-256 wrapping key wrapped with the
        ///       [public_key][google.cloud.kms.v1.ImportJob.public_key] using RSAES-OAEP with SHA-1,
        ///       MGF1 with SHA-1, and an empty label.
        ///   </li>
        ///   <li>The key to be imported, wrapped with the ephemeral AES-256 key
        ///       using AES-KWP (RFC 5649).
        ///   </li>
        /// </ol>
        ///
        /// If importing symmetric key material, it is expected that the unwrapped
        /// key contains plain bytes. If importing asymmetric key material, it is
        /// expected that the unwrapped key is in PKCS#8-encoded DER format (the
        /// PrivateKeyInfo structure from RFC 5208).
        ///
        /// This format is the same as the format produced by PKCS#11 mechanism
        /// CKM_RSA_AES_KEY_WRAP.
        #[prost(bytes, tag = "5")]
        RsaAesWrappedKey(std::vec::Vec<u8>),
    }
}
/// Request message for [KeyManagementService.CreateImportJob][google.cloud.kms.v1.KeyManagementService.CreateImportJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateImportJobRequest {
    /// Required. The [name][google.cloud.kms.v1.KeyRing.name] of the [KeyRing][google.cloud.kms.v1.KeyRing] associated with the
    /// [ImportJobs][google.cloud.kms.v1.ImportJob].
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. It must be unique within a KeyRing and match the regular
    /// expression `[a-zA-Z0-9_-]{1,63}`
    #[prost(string, tag = "2")]
    pub import_job_id: std::string::String,
    /// Required. An [ImportJob][google.cloud.kms.v1.ImportJob] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub import_job: ::std::option::Option<ImportJob>,
}
/// Request message for [KeyManagementService.UpdateCryptoKey][google.cloud.kms.v1.KeyManagementService.UpdateCryptoKey].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCryptoKeyRequest {
    /// Required. [CryptoKey][google.cloud.kms.v1.CryptoKey] with updated values.
    #[prost(message, optional, tag = "1")]
    pub crypto_key: ::std::option::Option<CryptoKey>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for [KeyManagementService.UpdateCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.UpdateCryptoKeyVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCryptoKeyVersionRequest {
    /// Required. [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] with updated values.
    #[prost(message, optional, tag = "1")]
    pub crypto_key_version: ::std::option::Option<CryptoKeyVersion>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for [KeyManagementService.Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptRequest {
    /// Required. The resource name of the [CryptoKey][google.cloud.kms.v1.CryptoKey] or [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]
    /// to use for encryption.
    ///
    /// If a [CryptoKey][google.cloud.kms.v1.CryptoKey] is specified, the server will use its
    /// [primary version][google.cloud.kms.v1.CryptoKey.primary].
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The data to encrypt. Must be no larger than 64KiB.
    ///
    /// The maximum size depends on the key version's
    /// [protection_level][google.cloud.kms.v1.CryptoKeyVersionTemplate.protection_level]. For
    /// [SOFTWARE][google.cloud.kms.v1.ProtectionLevel.SOFTWARE] keys, the plaintext must be no larger
    /// than 64KiB. For [HSM][google.cloud.kms.v1.ProtectionLevel.HSM] keys, the combined length of the
    /// plaintext and additional_authenticated_data fields must be no larger than
    /// 8KiB.
    #[prost(bytes, tag = "2")]
    pub plaintext: std::vec::Vec<u8>,
    /// Optional. Optional data that, if specified, must also be provided during decryption
    /// through [DecryptRequest.additional_authenticated_data][google.cloud.kms.v1.DecryptRequest.additional_authenticated_data].
    ///
    /// The maximum size depends on the key version's
    /// [protection_level][google.cloud.kms.v1.CryptoKeyVersionTemplate.protection_level]. For
    /// [SOFTWARE][google.cloud.kms.v1.ProtectionLevel.SOFTWARE] keys, the AAD must be no larger than
    /// 64KiB. For [HSM][google.cloud.kms.v1.ProtectionLevel.HSM] keys, the combined length of the
    /// plaintext and additional_authenticated_data fields must be no larger than
    /// 8KiB.
    #[prost(bytes, tag = "3")]
    pub additional_authenticated_data: std::vec::Vec<u8>,
}
/// Request message for [KeyManagementService.Decrypt][google.cloud.kms.v1.KeyManagementService.Decrypt].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptRequest {
    /// Required. The resource name of the [CryptoKey][google.cloud.kms.v1.CryptoKey] to use for decryption.
    /// The server will choose the appropriate version.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The encrypted data originally returned in
    /// [EncryptResponse.ciphertext][google.cloud.kms.v1.EncryptResponse.ciphertext].
    #[prost(bytes, tag = "2")]
    pub ciphertext: std::vec::Vec<u8>,
    /// Optional. Optional data that must match the data originally supplied in
    /// [EncryptRequest.additional_authenticated_data][google.cloud.kms.v1.EncryptRequest.additional_authenticated_data].
    #[prost(bytes, tag = "3")]
    pub additional_authenticated_data: std::vec::Vec<u8>,
}
/// Request message for [KeyManagementService.AsymmetricSign][google.cloud.kms.v1.KeyManagementService.AsymmetricSign].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsymmetricSignRequest {
    /// Required. The resource name of the [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] to use for signing.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The digest of the data to sign. The digest must be produced with
    /// the same digest algorithm as specified by the key version's
    /// [algorithm][google.cloud.kms.v1.CryptoKeyVersion.algorithm].
    #[prost(message, optional, tag = "3")]
    pub digest: ::std::option::Option<Digest>,
}
/// Request message for [KeyManagementService.AsymmetricDecrypt][google.cloud.kms.v1.KeyManagementService.AsymmetricDecrypt].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsymmetricDecryptRequest {
    /// Required. The resource name of the [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] to use for
    /// decryption.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The data encrypted with the named [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]'s public
    /// key using OAEP.
    #[prost(bytes, tag = "3")]
    pub ciphertext: std::vec::Vec<u8>,
}
/// Response message for [KeyManagementService.Decrypt][google.cloud.kms.v1.KeyManagementService.Decrypt].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecryptResponse {
    /// The decrypted data originally supplied in [EncryptRequest.plaintext][google.cloud.kms.v1.EncryptRequest.plaintext].
    #[prost(bytes, tag = "1")]
    pub plaintext: std::vec::Vec<u8>,
}
/// Response message for [KeyManagementService.Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptResponse {
    /// The resource name of the [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] used in encryption. Check
    /// this field to verify that the intended resource was used for encryption.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The encrypted data.
    #[prost(bytes, tag = "2")]
    pub ciphertext: std::vec::Vec<u8>,
}
/// Response message for [KeyManagementService.AsymmetricSign][google.cloud.kms.v1.KeyManagementService.AsymmetricSign].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsymmetricSignResponse {
    /// The created signature.
    #[prost(bytes, tag = "1")]
    pub signature: std::vec::Vec<u8>,
}
/// Response message for [KeyManagementService.AsymmetricDecrypt][google.cloud.kms.v1.KeyManagementService.AsymmetricDecrypt].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsymmetricDecryptResponse {
    /// The decrypted data originally encrypted with the matching public key.
    #[prost(bytes, tag = "1")]
    pub plaintext: std::vec::Vec<u8>,
}
/// Request message for [KeyManagementService.UpdateCryptoKeyPrimaryVersion][google.cloud.kms.v1.KeyManagementService.UpdateCryptoKeyPrimaryVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCryptoKeyPrimaryVersionRequest {
    /// Required. The resource name of the [CryptoKey][google.cloud.kms.v1.CryptoKey] to update.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The id of the child [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] to use as primary.
    #[prost(string, tag = "2")]
    pub crypto_key_version_id: std::string::String,
}
/// Request message for [KeyManagementService.DestroyCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.DestroyCryptoKeyVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroyCryptoKeyVersionRequest {
    /// Required. The resource name of the [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] to destroy.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [KeyManagementService.RestoreCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.RestoreCryptoKeyVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreCryptoKeyVersionRequest {
    /// Required. The resource name of the [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] to restore.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A [Digest][google.cloud.kms.v1.Digest] holds a cryptographic message digest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Digest {
    /// Required. The message digest.
    #[prost(oneof = "digest::Digest", tags = "1, 2, 3")]
    pub digest: ::std::option::Option<digest::Digest>,
}
pub mod digest {
    /// Required. The message digest.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Digest {
        /// A message digest produced with the SHA-256 algorithm.
        #[prost(bytes, tag = "1")]
        Sha256(std::vec::Vec<u8>),
        /// A message digest produced with the SHA-384 algorithm.
        #[prost(bytes, tag = "2")]
        Sha384(std::vec::Vec<u8>),
        /// A message digest produced with the SHA-512 algorithm.
        #[prost(bytes, tag = "3")]
        Sha512(std::vec::Vec<u8>),
    }
}
/// Cloud KMS metadata for the given [google.cloud.location.Location][google.cloud.location.Location].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// Indicates whether [CryptoKeys][google.cloud.kms.v1.CryptoKey] with
    /// [protection_level][google.cloud.kms.v1.CryptoKeyVersionTemplate.protection_level]
    /// [HSM][google.cloud.kms.v1.ProtectionLevel.HSM] can be created in this location.
    #[prost(bool, tag = "1")]
    pub hsm_available: bool,
    /// Indicates whether [CryptoKeys][google.cloud.kms.v1.CryptoKey] with
    /// [protection_level][google.cloud.kms.v1.CryptoKeyVersionTemplate.protection_level]
    /// [EXTERNAL][google.cloud.kms.v1.ProtectionLevel.EXTERNAL] can be created in this location.
    #[prost(bool, tag = "2")]
    pub ekm_available: bool,
}
#[doc = r" Generated client implementations."]
pub mod key_management_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Google Cloud Key Management Service"]
    #[doc = ""]
    #[doc = " Manages cryptographic keys and operations using those keys. Implements a REST"]
    #[doc = " model with the following objects:"]
    #[doc = ""]
    #[doc = " * [KeyRing][google.cloud.kms.v1.KeyRing]"]
    #[doc = " * [CryptoKey][google.cloud.kms.v1.CryptoKey]"]
    #[doc = " * [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]"]
    #[doc = " * [ImportJob][google.cloud.kms.v1.ImportJob]"]
    #[doc = ""]
    #[doc = " If you are using manual gRPC libraries, see"]
    #[doc = " [Using gRPC with Cloud KMS](https://cloud.google.com/kms/docs/grpc)."]
    pub struct KeyManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeyManagementServiceClient<T>
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
        #[doc = " Lists [KeyRings][google.cloud.kms.v1.KeyRing]."]
        pub async fn list_key_rings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKeyRingsRequest>,
        ) -> Result<tonic::Response<super::ListKeyRingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Lists [CryptoKeys][google.cloud.kms.v1.CryptoKey]."]
        pub async fn list_crypto_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCryptoKeysRequest>,
        ) -> Result<tonic::Response<super::ListCryptoKeysResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Lists [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion]."]
        pub async fn list_crypto_key_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCryptoKeyVersionsRequest>,
        ) -> Result<tonic::Response<super::ListCryptoKeyVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Lists [ImportJobs][google.cloud.kms.v1.ImportJob]."]
        pub async fn list_import_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListImportJobsRequest>,
        ) -> Result<tonic::Response<super::ListImportJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Returns metadata for a given [KeyRing][google.cloud.kms.v1.KeyRing]."]
        pub async fn get_key_ring(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeyRingRequest>,
        ) -> Result<tonic::Response<super::KeyRing>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Returns metadata for a given [CryptoKey][google.cloud.kms.v1.CryptoKey], as well as its"]
        #[doc = " [primary][google.cloud.kms.v1.CryptoKey.primary] [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]."]
        pub async fn get_crypto_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCryptoKeyRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Returns metadata for a given [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]."]
        pub async fn get_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Returns the public key for the given [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]. The"]
        #[doc = " [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] must be"]
        #[doc = " [ASYMMETRIC_SIGN][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_SIGN] or"]
        #[doc = " [ASYMMETRIC_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_DECRYPT]."]
        pub async fn get_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPublicKeyRequest>,
        ) -> Result<tonic::Response<super::PublicKey>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Returns metadata for a given [ImportJob][google.cloud.kms.v1.ImportJob]."]
        pub async fn get_import_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetImportJobRequest>,
        ) -> Result<tonic::Response<super::ImportJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Create a new [KeyRing][google.cloud.kms.v1.KeyRing] in a given Project and Location."]
        pub async fn create_key_ring(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKeyRingRequest>,
        ) -> Result<tonic::Response<super::KeyRing>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Create a new [CryptoKey][google.cloud.kms.v1.CryptoKey] within a [KeyRing][google.cloud.kms.v1.KeyRing]."]
        #[doc = ""]
        #[doc = " [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] and"]
        #[doc = " [CryptoKey.version_template.algorithm][google.cloud.kms.v1.CryptoKeyVersionTemplate.algorithm]"]
        #[doc = " are required."]
        pub async fn create_crypto_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCryptoKeyRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Create a new [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] in a [CryptoKey][google.cloud.kms.v1.CryptoKey]."]
        #[doc = ""]
        #[doc = " The server will assign the next sequential id. If unset,"]
        #[doc = " [state][google.cloud.kms.v1.CryptoKeyVersion.state] will be set to"]
        #[doc = " [ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED]."]
        pub async fn create_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Imports a new [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] into an existing [CryptoKey][google.cloud.kms.v1.CryptoKey] using the"]
        #[doc = " wrapped key material provided in the request."]
        #[doc = ""]
        #[doc = " The version ID will be assigned the next sequential id within the"]
        #[doc = " [CryptoKey][google.cloud.kms.v1.CryptoKey]."]
        pub async fn import_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Create a new [ImportJob][google.cloud.kms.v1.ImportJob] within a [KeyRing][google.cloud.kms.v1.KeyRing]."]
        #[doc = ""]
        #[doc = " [ImportJob.import_method][google.cloud.kms.v1.ImportJob.import_method] is required."]
        pub async fn create_import_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateImportJobRequest>,
        ) -> Result<tonic::Response<super::ImportJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Update a [CryptoKey][google.cloud.kms.v1.CryptoKey]."]
        pub async fn update_crypto_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCryptoKeyRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Update a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]'s metadata."]
        #[doc = ""]
        #[doc = " [state][google.cloud.kms.v1.CryptoKeyVersion.state] may be changed between"]
        #[doc = " [ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED] and"]
        #[doc = " [DISABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DISABLED] using this"]
        #[doc = " method. See [DestroyCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.DestroyCryptoKeyVersion] and [RestoreCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.RestoreCryptoKeyVersion] to"]
        #[doc = " move between other states."]
        pub async fn update_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Encrypts data, so that it can only be recovered by a call to [Decrypt][google.cloud.kms.v1.KeyManagementService.Decrypt]."]
        #[doc = " The [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] must be"]
        #[doc = " [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT]."]
        pub async fn encrypt(
            &mut self,
            request: impl tonic::IntoRequest<super::EncryptRequest>,
        ) -> Result<tonic::Response<super::EncryptResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Decrypts data that was protected by [Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt]. The [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose]"]
        #[doc = " must be [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT]."]
        pub async fn decrypt(
            &mut self,
            request: impl tonic::IntoRequest<super::DecryptRequest>,
        ) -> Result<tonic::Response<super::DecryptResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Signs data using a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] with [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose]"]
        #[doc = " ASYMMETRIC_SIGN, producing a signature that can be verified with the public"]
        #[doc = " key retrieved from [GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey]."]
        pub async fn asymmetric_sign(
            &mut self,
            request: impl tonic::IntoRequest<super::AsymmetricSignRequest>,
        ) -> Result<tonic::Response<super::AsymmetricSignResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Decrypts data that was encrypted with a public key retrieved from"]
        #[doc = " [GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey] corresponding to a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] with"]
        #[doc = " [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] ASYMMETRIC_DECRYPT."]
        pub async fn asymmetric_decrypt(
            &mut self,
            request: impl tonic::IntoRequest<super::AsymmetricDecryptRequest>,
        ) -> Result<tonic::Response<super::AsymmetricDecryptResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Update the version of a [CryptoKey][google.cloud.kms.v1.CryptoKey] that will be used in [Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt]."]
        #[doc = ""]
        #[doc = " Returns an error if called on an asymmetric key."]
        pub async fn update_crypto_key_primary_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCryptoKeyPrimaryVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Schedule a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] for destruction."]
        #[doc = ""]
        #[doc = " Upon calling this method, [CryptoKeyVersion.state][google.cloud.kms.v1.CryptoKeyVersion.state] will be set to"]
        #[doc = " [DESTROY_SCHEDULED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROY_SCHEDULED]"]
        #[doc = " and [destroy_time][google.cloud.kms.v1.CryptoKeyVersion.destroy_time] will be set to a time 24"]
        #[doc = " hours in the future, at which point the [state][google.cloud.kms.v1.CryptoKeyVersion.state]"]
        #[doc = " will be changed to"]
        #[doc = " [DESTROYED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROYED], and the key"]
        #[doc = " material will be irrevocably destroyed."]
        #[doc = ""]
        #[doc = " Before the [destroy_time][google.cloud.kms.v1.CryptoKeyVersion.destroy_time] is reached,"]
        #[doc = " [RestoreCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.RestoreCryptoKeyVersion] may be called to reverse the process."]
        pub async fn destroy_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroyCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Restore a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] in the"]
        #[doc = " [DESTROY_SCHEDULED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROY_SCHEDULED]"]
        #[doc = " state."]
        #[doc = ""]
        #[doc = " Upon restoration of the CryptoKeyVersion, [state][google.cloud.kms.v1.CryptoKeyVersion.state]"]
        #[doc = " will be set to [DISABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DISABLED],"]
        #[doc = " and [destroy_time][google.cloud.kms.v1.CryptoKeyVersion.destroy_time] will be cleared."]
        pub async fn restore_crypto_key_version(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
    }
    impl<T: Clone> Clone for KeyManagementServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for KeyManagementServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "KeyManagementServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod key_management_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with KeyManagementServiceServer."]
    #[async_trait]
    pub trait KeyManagementService: Send + Sync + 'static {
        #[doc = " Lists [KeyRings][google.cloud.kms.v1.KeyRing]."]
        async fn list_key_rings(
            &self,
            request: tonic::Request<super::ListKeyRingsRequest>,
        ) -> Result<tonic::Response<super::ListKeyRingsResponse>, tonic::Status>;
        #[doc = " Lists [CryptoKeys][google.cloud.kms.v1.CryptoKey]."]
        async fn list_crypto_keys(
            &self,
            request: tonic::Request<super::ListCryptoKeysRequest>,
        ) -> Result<tonic::Response<super::ListCryptoKeysResponse>, tonic::Status>;
        #[doc = " Lists [CryptoKeyVersions][google.cloud.kms.v1.CryptoKeyVersion]."]
        async fn list_crypto_key_versions(
            &self,
            request: tonic::Request<super::ListCryptoKeyVersionsRequest>,
        ) -> Result<tonic::Response<super::ListCryptoKeyVersionsResponse>, tonic::Status>;
        #[doc = " Lists [ImportJobs][google.cloud.kms.v1.ImportJob]."]
        async fn list_import_jobs(
            &self,
            request: tonic::Request<super::ListImportJobsRequest>,
        ) -> Result<tonic::Response<super::ListImportJobsResponse>, tonic::Status>;
        #[doc = " Returns metadata for a given [KeyRing][google.cloud.kms.v1.KeyRing]."]
        async fn get_key_ring(
            &self,
            request: tonic::Request<super::GetKeyRingRequest>,
        ) -> Result<tonic::Response<super::KeyRing>, tonic::Status>;
        #[doc = " Returns metadata for a given [CryptoKey][google.cloud.kms.v1.CryptoKey], as well as its"]
        #[doc = " [primary][google.cloud.kms.v1.CryptoKey.primary] [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]."]
        async fn get_crypto_key(
            &self,
            request: tonic::Request<super::GetCryptoKeyRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status>;
        #[doc = " Returns metadata for a given [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]."]
        async fn get_crypto_key_version(
            &self,
            request: tonic::Request<super::GetCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status>;
        #[doc = " Returns the public key for the given [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]. The"]
        #[doc = " [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] must be"]
        #[doc = " [ASYMMETRIC_SIGN][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_SIGN] or"]
        #[doc = " [ASYMMETRIC_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ASYMMETRIC_DECRYPT]."]
        async fn get_public_key(
            &self,
            request: tonic::Request<super::GetPublicKeyRequest>,
        ) -> Result<tonic::Response<super::PublicKey>, tonic::Status>;
        #[doc = " Returns metadata for a given [ImportJob][google.cloud.kms.v1.ImportJob]."]
        async fn get_import_job(
            &self,
            request: tonic::Request<super::GetImportJobRequest>,
        ) -> Result<tonic::Response<super::ImportJob>, tonic::Status>;
        #[doc = " Create a new [KeyRing][google.cloud.kms.v1.KeyRing] in a given Project and Location."]
        async fn create_key_ring(
            &self,
            request: tonic::Request<super::CreateKeyRingRequest>,
        ) -> Result<tonic::Response<super::KeyRing>, tonic::Status>;
        #[doc = " Create a new [CryptoKey][google.cloud.kms.v1.CryptoKey] within a [KeyRing][google.cloud.kms.v1.KeyRing]."]
        #[doc = ""]
        #[doc = " [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] and"]
        #[doc = " [CryptoKey.version_template.algorithm][google.cloud.kms.v1.CryptoKeyVersionTemplate.algorithm]"]
        #[doc = " are required."]
        async fn create_crypto_key(
            &self,
            request: tonic::Request<super::CreateCryptoKeyRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status>;
        #[doc = " Create a new [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] in a [CryptoKey][google.cloud.kms.v1.CryptoKey]."]
        #[doc = ""]
        #[doc = " The server will assign the next sequential id. If unset,"]
        #[doc = " [state][google.cloud.kms.v1.CryptoKeyVersion.state] will be set to"]
        #[doc = " [ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED]."]
        async fn create_crypto_key_version(
            &self,
            request: tonic::Request<super::CreateCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status>;
        #[doc = " Imports a new [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] into an existing [CryptoKey][google.cloud.kms.v1.CryptoKey] using the"]
        #[doc = " wrapped key material provided in the request."]
        #[doc = ""]
        #[doc = " The version ID will be assigned the next sequential id within the"]
        #[doc = " [CryptoKey][google.cloud.kms.v1.CryptoKey]."]
        async fn import_crypto_key_version(
            &self,
            request: tonic::Request<super::ImportCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status>;
        #[doc = " Create a new [ImportJob][google.cloud.kms.v1.ImportJob] within a [KeyRing][google.cloud.kms.v1.KeyRing]."]
        #[doc = ""]
        #[doc = " [ImportJob.import_method][google.cloud.kms.v1.ImportJob.import_method] is required."]
        async fn create_import_job(
            &self,
            request: tonic::Request<super::CreateImportJobRequest>,
        ) -> Result<tonic::Response<super::ImportJob>, tonic::Status>;
        #[doc = " Update a [CryptoKey][google.cloud.kms.v1.CryptoKey]."]
        async fn update_crypto_key(
            &self,
            request: tonic::Request<super::UpdateCryptoKeyRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status>;
        #[doc = " Update a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]'s metadata."]
        #[doc = ""]
        #[doc = " [state][google.cloud.kms.v1.CryptoKeyVersion.state] may be changed between"]
        #[doc = " [ENABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.ENABLED] and"]
        #[doc = " [DISABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DISABLED] using this"]
        #[doc = " method. See [DestroyCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.DestroyCryptoKeyVersion] and [RestoreCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.RestoreCryptoKeyVersion] to"]
        #[doc = " move between other states."]
        async fn update_crypto_key_version(
            &self,
            request: tonic::Request<super::UpdateCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status>;
        #[doc = " Encrypts data, so that it can only be recovered by a call to [Decrypt][google.cloud.kms.v1.KeyManagementService.Decrypt]."]
        #[doc = " The [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] must be"]
        #[doc = " [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT]."]
        async fn encrypt(
            &self,
            request: tonic::Request<super::EncryptRequest>,
        ) -> Result<tonic::Response<super::EncryptResponse>, tonic::Status>;
        #[doc = " Decrypts data that was protected by [Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt]. The [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose]"]
        #[doc = " must be [ENCRYPT_DECRYPT][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose.ENCRYPT_DECRYPT]."]
        async fn decrypt(
            &self,
            request: tonic::Request<super::DecryptRequest>,
        ) -> Result<tonic::Response<super::DecryptResponse>, tonic::Status>;
        #[doc = " Signs data using a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] with [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose]"]
        #[doc = " ASYMMETRIC_SIGN, producing a signature that can be verified with the public"]
        #[doc = " key retrieved from [GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey]."]
        async fn asymmetric_sign(
            &self,
            request: tonic::Request<super::AsymmetricSignRequest>,
        ) -> Result<tonic::Response<super::AsymmetricSignResponse>, tonic::Status>;
        #[doc = " Decrypts data that was encrypted with a public key retrieved from"]
        #[doc = " [GetPublicKey][google.cloud.kms.v1.KeyManagementService.GetPublicKey] corresponding to a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] with"]
        #[doc = " [CryptoKey.purpose][google.cloud.kms.v1.CryptoKey.purpose] ASYMMETRIC_DECRYPT."]
        async fn asymmetric_decrypt(
            &self,
            request: tonic::Request<super::AsymmetricDecryptRequest>,
        ) -> Result<tonic::Response<super::AsymmetricDecryptResponse>, tonic::Status>;
        #[doc = " Update the version of a [CryptoKey][google.cloud.kms.v1.CryptoKey] that will be used in [Encrypt][google.cloud.kms.v1.KeyManagementService.Encrypt]."]
        #[doc = ""]
        #[doc = " Returns an error if called on an asymmetric key."]
        async fn update_crypto_key_primary_version(
            &self,
            request: tonic::Request<super::UpdateCryptoKeyPrimaryVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKey>, tonic::Status>;
        #[doc = " Schedule a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] for destruction."]
        #[doc = ""]
        #[doc = " Upon calling this method, [CryptoKeyVersion.state][google.cloud.kms.v1.CryptoKeyVersion.state] will be set to"]
        #[doc = " [DESTROY_SCHEDULED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROY_SCHEDULED]"]
        #[doc = " and [destroy_time][google.cloud.kms.v1.CryptoKeyVersion.destroy_time] will be set to a time 24"]
        #[doc = " hours in the future, at which point the [state][google.cloud.kms.v1.CryptoKeyVersion.state]"]
        #[doc = " will be changed to"]
        #[doc = " [DESTROYED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROYED], and the key"]
        #[doc = " material will be irrevocably destroyed."]
        #[doc = ""]
        #[doc = " Before the [destroy_time][google.cloud.kms.v1.CryptoKeyVersion.destroy_time] is reached,"]
        #[doc = " [RestoreCryptoKeyVersion][google.cloud.kms.v1.KeyManagementService.RestoreCryptoKeyVersion] may be called to reverse the process."]
        async fn destroy_crypto_key_version(
            &self,
            request: tonic::Request<super::DestroyCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status>;
        #[doc = " Restore a [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion] in the"]
        #[doc = " [DESTROY_SCHEDULED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DESTROY_SCHEDULED]"]
        #[doc = " state."]
        #[doc = ""]
        #[doc = " Upon restoration of the CryptoKeyVersion, [state][google.cloud.kms.v1.CryptoKeyVersion.state]"]
        #[doc = " will be set to [DISABLED][google.cloud.kms.v1.CryptoKeyVersion.CryptoKeyVersionState.DISABLED],"]
        #[doc = " and [destroy_time][google.cloud.kms.v1.CryptoKeyVersion.destroy_time] will be cleared."]
        async fn restore_crypto_key_version(
            &self,
            request: tonic::Request<super::RestoreCryptoKeyVersionRequest>,
        ) -> Result<tonic::Response<super::CryptoKeyVersion>, tonic::Status>;
    }
    #[doc = " Google Cloud Key Management Service"]
    #[doc = ""]
    #[doc = " Manages cryptographic keys and operations using those keys. Implements a REST"]
    #[doc = " model with the following objects:"]
    #[doc = ""]
    #[doc = " * [KeyRing][google.cloud.kms.v1.KeyRing]"]
    #[doc = " * [CryptoKey][google.cloud.kms.v1.CryptoKey]"]
    #[doc = " * [CryptoKeyVersion][google.cloud.kms.v1.CryptoKeyVersion]"]
    #[doc = " * [ImportJob][google.cloud.kms.v1.ImportJob]"]
    #[doc = ""]
    #[doc = " If you are using manual gRPC libraries, see"]
    #[doc = " [Using gRPC with Cloud KMS](https://cloud.google.com/kms/docs/grpc)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct KeyManagementServiceServer<T: KeyManagementService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: KeyManagementService> KeyManagementServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for KeyManagementServiceServer<T>
    where
        T: KeyManagementService,
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
                "/google.cloud.kms.v1.KeyManagementService/ListKeyRings" => {
                    #[allow(non_camel_case_types)]
                    struct ListKeyRingsSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::ListKeyRingsRequest>
                        for ListKeyRingsSvc<T>
                    {
                        type Response = super::ListKeyRingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListKeyRingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_key_rings(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListKeyRingsSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/ListCryptoKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ListCryptoKeysSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::ListCryptoKeysRequest>
                        for ListCryptoKeysSvc<T>
                    {
                        type Response = super::ListCryptoKeysResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCryptoKeysRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_crypto_keys(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListCryptoKeysSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/ListCryptoKeyVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListCryptoKeyVersionsSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::ListCryptoKeyVersionsRequest>
                        for ListCryptoKeyVersionsSvc<T>
                    {
                        type Response = super::ListCryptoKeyVersionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCryptoKeyVersionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_crypto_key_versions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListCryptoKeyVersionsSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/ListImportJobs" => {
                    #[allow(non_camel_case_types)]
                    struct ListImportJobsSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::ListImportJobsRequest>
                        for ListImportJobsSvc<T>
                    {
                        type Response = super::ListImportJobsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListImportJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_import_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListImportJobsSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/GetKeyRing" => {
                    #[allow(non_camel_case_types)]
                    struct GetKeyRingSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::GetKeyRingRequest> for GetKeyRingSvc<T>
                    {
                        type Response = super::KeyRing;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetKeyRingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_key_ring(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetKeyRingSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/GetCryptoKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetCryptoKeySvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::GetCryptoKeyRequest>
                        for GetCryptoKeySvc<T>
                    {
                        type Response = super::CryptoKey;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCryptoKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_crypto_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetCryptoKeySvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/GetCryptoKeyVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetCryptoKeyVersionSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::GetCryptoKeyVersionRequest>
                        for GetCryptoKeyVersionSvc<T>
                    {
                        type Response = super::CryptoKeyVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCryptoKeyVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_crypto_key_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetCryptoKeyVersionSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/GetPublicKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetPublicKeySvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::GetPublicKeyRequest>
                        for GetPublicKeySvc<T>
                    {
                        type Response = super::PublicKey;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPublicKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_public_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetPublicKeySvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/GetImportJob" => {
                    #[allow(non_camel_case_types)]
                    struct GetImportJobSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::GetImportJobRequest>
                        for GetImportJobSvc<T>
                    {
                        type Response = super::ImportJob;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetImportJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_import_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetImportJobSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/CreateKeyRing" => {
                    #[allow(non_camel_case_types)]
                    struct CreateKeyRingSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::CreateKeyRingRequest>
                        for CreateKeyRingSvc<T>
                    {
                        type Response = super::KeyRing;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateKeyRingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_key_ring(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateKeyRingSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/CreateCryptoKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCryptoKeySvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::CreateCryptoKeyRequest>
                        for CreateCryptoKeySvc<T>
                    {
                        type Response = super::CryptoKey;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCryptoKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_crypto_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateCryptoKeySvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/CreateCryptoKeyVersion" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCryptoKeyVersionSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::CreateCryptoKeyVersionRequest>
                        for CreateCryptoKeyVersionSvc<T>
                    {
                        type Response = super::CryptoKeyVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCryptoKeyVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_crypto_key_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateCryptoKeyVersionSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/ImportCryptoKeyVersion" => {
                    #[allow(non_camel_case_types)]
                    struct ImportCryptoKeyVersionSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::ImportCryptoKeyVersionRequest>
                        for ImportCryptoKeyVersionSvc<T>
                    {
                        type Response = super::CryptoKeyVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportCryptoKeyVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.import_crypto_key_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ImportCryptoKeyVersionSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/CreateImportJob" => {
                    #[allow(non_camel_case_types)]
                    struct CreateImportJobSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::CreateImportJobRequest>
                        for CreateImportJobSvc<T>
                    {
                        type Response = super::ImportJob;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateImportJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_import_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateImportJobSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/UpdateCryptoKey" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCryptoKeySvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::UpdateCryptoKeyRequest>
                        for UpdateCryptoKeySvc<T>
                    {
                        type Response = super::CryptoKey;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCryptoKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_crypto_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateCryptoKeySvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/UpdateCryptoKeyVersion" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCryptoKeyVersionSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::UpdateCryptoKeyVersionRequest>
                        for UpdateCryptoKeyVersionSvc<T>
                    {
                        type Response = super::CryptoKeyVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCryptoKeyVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_crypto_key_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateCryptoKeyVersionSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/Encrypt" => {
                    #[allow(non_camel_case_types)]
                    struct EncryptSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService> tonic::server::UnaryService<super::EncryptRequest> for EncryptSvc<T> {
                        type Response = super::EncryptResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EncryptRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.encrypt(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = EncryptSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/Decrypt" => {
                    #[allow(non_camel_case_types)]
                    struct DecryptSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService> tonic::server::UnaryService<super::DecryptRequest> for DecryptSvc<T> {
                        type Response = super::DecryptResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DecryptRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.decrypt(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DecryptSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/AsymmetricSign" => {
                    #[allow(non_camel_case_types)]
                    struct AsymmetricSignSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::AsymmetricSignRequest>
                        for AsymmetricSignSvc<T>
                    {
                        type Response = super::AsymmetricSignResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AsymmetricSignRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.asymmetric_sign(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AsymmetricSignSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/AsymmetricDecrypt" => {
                    #[allow(non_camel_case_types)]
                    struct AsymmetricDecryptSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::AsymmetricDecryptRequest>
                        for AsymmetricDecryptSvc<T>
                    {
                        type Response = super::AsymmetricDecryptResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AsymmetricDecryptRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.asymmetric_decrypt(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AsymmetricDecryptSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/UpdateCryptoKeyPrimaryVersion" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCryptoKeyPrimaryVersionSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::UpdateCryptoKeyPrimaryVersionRequest>
                        for UpdateCryptoKeyPrimaryVersionSvc<T>
                    {
                        type Response = super::CryptoKey;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCryptoKeyPrimaryVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                inner.update_crypto_key_primary_version(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateCryptoKeyPrimaryVersionSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/DestroyCryptoKeyVersion" => {
                    #[allow(non_camel_case_types)]
                    struct DestroyCryptoKeyVersionSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::DestroyCryptoKeyVersionRequest>
                        for DestroyCryptoKeyVersionSvc<T>
                    {
                        type Response = super::CryptoKeyVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DestroyCryptoKeyVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.destroy_crypto_key_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DestroyCryptoKeyVersionSvc(inner);
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
                "/google.cloud.kms.v1.KeyManagementService/RestoreCryptoKeyVersion" => {
                    #[allow(non_camel_case_types)]
                    struct RestoreCryptoKeyVersionSvc<T: KeyManagementService>(pub Arc<T>);
                    impl<T: KeyManagementService>
                        tonic::server::UnaryService<super::RestoreCryptoKeyVersionRequest>
                        for RestoreCryptoKeyVersionSvc<T>
                    {
                        type Response = super::CryptoKeyVersion;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestoreCryptoKeyVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.restore_crypto_key_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RestoreCryptoKeyVersionSvc(inner);
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
    impl<T: KeyManagementService> Clone for KeyManagementServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: KeyManagementService> Clone for _Inner<T> {
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
