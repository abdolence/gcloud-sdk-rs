/// A [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] represents an individual Certificate Authority.
/// A [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] can be used to create [Certificates][google.cloud.security.privateca.v1beta1.Certificate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateAuthority {
    /// Output only. The resource name for this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] in the
    /// format `projects/*/locations/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Immutable. The [Type][google.cloud.security.privateca.v1beta1.CertificateAuthority.Type] of this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority].
    #[prost(enumeration = "certificate_authority::Type", tag = "2")]
    pub r#type: i32,
    /// Required. Immutable. The [Tier][google.cloud.security.privateca.v1beta1.CertificateAuthority.Tier] of this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority].
    #[prost(enumeration = "certificate_authority::Tier", tag = "3")]
    pub tier: i32,
    /// Required. Immutable. The config used to create a self-signed X.509 certificate or CSR.
    #[prost(message, optional, tag = "4")]
    pub config: ::std::option::Option<CertificateConfig>,
    /// Required. The desired lifetime of the CA certificate. Used to create the
    /// "not_before_time" and "not_after_time" fields inside an X.509
    /// certificate.
    #[prost(message, optional, tag = "5")]
    pub lifetime: ::std::option::Option<::prost_types::Duration>,
    /// Required. Immutable. Used when issuing certificates for this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]. If this
    /// [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] is a self-signed CertificateAuthority, this key
    /// is also used to sign the self-signed CA certificate. Otherwise, it
    /// is used to sign a CSR.
    #[prost(message, optional, tag = "6")]
    pub key_spec: ::std::option::Option<certificate_authority::KeyVersionSpec>,
    /// Optional. The [CertificateAuthorityPolicy][google.cloud.security.privateca.v1beta1.CertificateAuthority.CertificateAuthorityPolicy] to enforce when issuing
    /// [Certificates][google.cloud.security.privateca.v1beta1.Certificate] from this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority].
    #[prost(message, optional, tag = "7")]
    pub certificate_policy:
        ::std::option::Option<certificate_authority::CertificateAuthorityPolicy>,
    /// Optional. The [IssuingOptions][google.cloud.security.privateca.v1beta1.CertificateAuthority.IssuingOptions] to follow when issuing [Certificates][google.cloud.security.privateca.v1beta1.Certificate]
    /// from this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority].
    #[prost(message, optional, tag = "8")]
    pub issuing_options: ::std::option::Option<certificate_authority::IssuingOptions>,
    /// Optional. This [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]'s issuer chain. If self-signed, will be the
    /// same as 'pem_cert'. This may be updated (e.g., if an issuer's cert was
    /// replaced). Expected to be in issuer-to-root order according to RFC 5246.
    #[prost(string, repeated, tag = "9")]
    pub pem_issuer_cert_chain: ::std::vec::Vec<std::string::String>,
    /// Output only. The [State][google.cloud.security.privateca.v1beta1.CertificateAuthority.State] for this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority].
    #[prost(enumeration = "certificate_authority::State", tag = "10")]
    pub state: i32,
    /// Output only. This [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]'s CA certificate.
    #[prost(string, tag = "11")]
    pub pem_certificate: std::string::String,
    /// Output only. A structured description of this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]'s CA cert.
    #[prost(message, optional, tag = "12")]
    pub ca_certificate_description: ::std::option::Option<CertificateDescription>,
    /// Immutable. The name of a Cloud Storage bucket where this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] will
    /// publish content, such as the CA certificate and CRLs. This must be a bucket
    /// name, without any prefixes (such as `gs://`) or suffixes (such as
    /// `.googleapis.com`). For example, to use a bucket named `my-bucket`, you
    /// would simply specify `my-bucket`. If not specified, a managed bucket will
    /// be created.
    #[prost(string, tag = "13")]
    pub gcs_bucket: std::string::String,
    /// Output only. URLs for accessing content published by this CA, such as the CA certificate
    /// and CRLs.
    #[prost(message, optional, tag = "14")]
    pub access_urls: ::std::option::Option<certificate_authority::AccessUrls>,
    /// Output only. The time at which this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] was created.
    #[prost(message, optional, tag = "15")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] was updated.
    #[prost(message, optional, tag = "16")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] will be deleted, if
    /// scheduled for deletion.
    #[prost(message, optional, tag = "17")]
    pub deletion_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels with user-defined metadata.
    #[prost(map = "string, string", tag = "18")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
pub mod certificate_authority {
    /// Options that affect all certificates issued by a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IssuingOptions {
        /// Required. When true, includes a URL to the issuing CA certificate in the
        /// "authority information access" X.509 extension.
        #[prost(bool, tag = "1")]
        pub include_ca_cert_url: bool,
        /// Required. When true, includes a URL to the CRL corresponding to certificates
        /// issued from a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority].
        /// CRLs will expire 7 days from their creation. However, we will rebuild
        /// daily. CRLs are also rebuilt shortly after a certificate is revoked.
        #[prost(bool, tag = "2")]
        pub include_crl_access_url: bool,
    }
    /// The issuing policy for a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority].
    /// [Certificates][google.cloud.security.privateca.v1beta1.Certificate] will not be successfully issued from this
    /// [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] if they violate the policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CertificateAuthorityPolicy {
        /// Optional. If any [Subject][google.cloud.security.privateca.v1beta1.Subject] is specified here, then all
        /// [Certificates][google.cloud.security.privateca.v1beta1.Certificate] issued by the [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] must
        /// match at least one listed [Subject][google.cloud.security.privateca.v1beta1.Subject]. If a [Subject][google.cloud.security.privateca.v1beta1.Subject] has an empty
        /// field, any value will be allowed for that field.
        #[prost(message, repeated, tag = "3")]
        pub allowed_locations_and_organizations: ::std::vec::Vec<super::Subject>,
        /// Optional. If any value is specified here, then all
        /// [Certificates][google.cloud.security.privateca.v1beta1.Certificate] issued by the [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] must
        /// match at least one listed value. If no value is specified, all values
        /// will be allowed for this fied. Glob patterns are also supported.
        #[prost(string, repeated, tag = "4")]
        pub allowed_common_names: ::std::vec::Vec<std::string::String>,
        /// Optional. If a [AllowedSubjectAltNames][google.cloud.security.privateca.v1beta1.CertificateAuthority.CertificateAuthorityPolicy.AllowedSubjectAltNames] is specified here, then all
        /// [Certificates][google.cloud.security.privateca.v1beta1.Certificate] issued by the [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] must
        /// match [AllowedSubjectAltNames][google.cloud.security.privateca.v1beta1.CertificateAuthority.CertificateAuthorityPolicy.AllowedSubjectAltNames]. If no value or an empty value
        /// is specified, any value will be allowed for the [SubjectAltNames][google.cloud.security.privateca.v1beta1.SubjectAltNames]
        /// field.
        #[prost(message, optional, tag = "5")]
        pub allowed_sans:
            ::std::option::Option<certificate_authority_policy::AllowedSubjectAltNames>,
        /// Optional. The maximum lifetime allowed by the [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]. Note that
        /// if the any part if the issuing chain expires before a [Certificate][google.cloud.security.privateca.v1beta1.Certificate]'s
        /// requested maximum_lifetime, the effective lifetime will be explicitly
        /// truncated.
        #[prost(message, optional, tag = "6")]
        pub maximum_lifetime: ::std::option::Option<::prost_types::Duration>,
        /// Optional. If specified, use a Cloud Function to implement custom certificate policy
        /// for certificate issuance.
        #[prost(message, optional, tag = "7")]
        pub cloud_function_policy:
            ::std::option::Option<certificate_authority_policy::CloudFunctionPolicy>,
        /// Allowed configurations or a single configuration for all issued
        /// certificates.
        #[prost(oneof = "certificate_authority_policy::ConfigPolicy", tags = "1, 2")]
        pub config_policy: ::std::option::Option<certificate_authority_policy::ConfigPolicy>,
    }
    pub mod certificate_authority_policy {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AllowedConfigList {
            /// Required. All [Certificates][google.cloud.security.privateca.v1beta1.Certificate] issued by the [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]
            /// must match at least one listed [ReusableConfigWrapper][google.cloud.security.privateca.v1beta1.ReusableConfigWrapper]. If a
            /// [ReusableConfigWrapper][google.cloud.security.privateca.v1beta1.ReusableConfigWrapper] has an empty field, any value will be
            /// allowed for that field.
            #[prost(message, repeated, tag = "1")]
            pub allowed_config_values: ::std::vec::Vec<super::super::ReusableConfigWrapper>,
        }
        /// [AllowedSubjectAltNames][google.cloud.security.privateca.v1beta1.CertificateAuthority.CertificateAuthorityPolicy.AllowedSubjectAltNames] specifies the allowed values for
        /// [SubjectAltNames][google.cloud.security.privateca.v1beta1.SubjectAltNames] by the [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] when issuing
        /// [Certificates][google.cloud.security.privateca.v1beta1.Certificate].
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AllowedSubjectAltNames {
            /// Optional. Contains valid, fully-qualified host names. Glob patterns are also
            /// supported. To allow an explicit wildcard certificate, escape with
            /// backlash (i.e. "\*").
            /// E.g. for globbed entries: '*bar.com' will allow foo.bar.com, but not
            /// *.bar.com, unless the [allow_globbing_dns_wildcards][google.cloud.security.privateca.v1beta1.CertificateAuthority.CertificateAuthorityPolicy.AllowedSubjectAltNames.allow_globbing_dns_wildcards] field is set.
            /// E.g. for wildcard entries: '\*.bar.com' will allow '*.bar.com', but not
            /// 'foo.bar.com'.
            #[prost(string, repeated, tag = "1")]
            pub allowed_dns_names: ::std::vec::Vec<std::string::String>,
            /// Optional. Contains valid RFC 3986 URIs. Glob patterns are also supported. To
            /// match across path seperators (i.e. '/') use the double star glob
            /// pattern (i.e. '**').
            #[prost(string, repeated, tag = "2")]
            pub allowed_uris: ::std::vec::Vec<std::string::String>,
            /// Optional. Contains valid RFC 2822 E-mail addresses. Glob patterns are also
            /// supported.
            #[prost(string, repeated, tag = "3")]
            pub allowed_email_addresses: ::std::vec::Vec<std::string::String>,
            /// Optional. Contains valid 32-bit IPv4 addresses and subnet ranges or RFC 4291 IPv6
            /// addresses and subnet ranges. Subnet ranges are specified using the
            /// '/' notation (e.g. 10.0.0.0/8, 2001:700:300:1800::/64). Glob patterns
            /// are supported only for ip address entries (i.e. not for subnet ranges).
            #[prost(string, repeated, tag = "4")]
            pub allowed_ips: ::std::vec::Vec<std::string::String>,
            /// Optional. Specifies if glob patterns used for [allowed_dns_names][google.cloud.security.privateca.v1beta1.CertificateAuthority.CertificateAuthorityPolicy.AllowedSubjectAltNames.allowed_dns_names] allows
            /// wildcard certificates.
            #[prost(bool, tag = "5")]
            pub allow_globbing_dns_wildcards: bool,
            /// Optional. Specifies if to allow custom X509Extension values.
            #[prost(bool, tag = "6")]
            pub allow_custom_sans: bool,
        }
        /// [CloudFunctionPolicy][google.cloud.security.privateca.v1beta1.CertificateAuthority.CertificateAuthorityPolicy.CloudFunctionPolicy] specifies the Cloud Function custom certificate
        /// policy for certificate issuance.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CloudFunctionPolicy {
            /// Required. The resource name of the Cloud Function to invoke, in the format
            /// `projects/*/locations/*/functions/*`.
            #[prost(string, tag = "1")]
            pub name: std::string::String,
        }
        /// Allowed configurations or a single configuration for all issued
        /// certificates.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConfigPolicy {
            /// Optional. All [Certificates][google.cloud.security.privateca.v1beta1.Certificate] issued by the [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]
            /// must match at least one listed [ReusableConfigWrapper][google.cloud.security.privateca.v1beta1.ReusableConfigWrapper] in the list.
            #[prost(message, tag = "1")]
            AllowedConfigList(AllowedConfigList),
            /// Optional. All [Certificates][google.cloud.security.privateca.v1beta1.Certificate] issued by the [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]
            /// will use the provided configuration values, overwriting any requested
            /// configuration values.
            #[prost(message, tag = "2")]
            OverwriteConfigValues(super::super::ReusableConfigWrapper),
        }
    }
    /// URLs where a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] will publish content.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessUrls {
        /// The URL where this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]'s CA certificate is
        /// published. This will only be set for CAs that have been activated.
        #[prost(string, tag = "1")]
        pub ca_certificate_access_url: std::string::String,
        /// The URL where this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]'s CRLs are published. This
        /// will only be set for CAs that have been activated.
        #[prost(string, tag = "2")]
        pub crl_access_url: std::string::String,
    }
    /// A Cloud KMS key configuration that a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] will use.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyVersionSpec {
        #[prost(oneof = "key_version_spec::KeyVersion", tags = "1, 2")]
        pub key_version: ::std::option::Option<key_version_spec::KeyVersion>,
    }
    pub mod key_version_spec {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum KeyVersion {
            /// Required. The resource name for an existing Cloud KMS CryptoKeyVersion in the
            /// format`projects/*/locations/*/keyRings/*/cryptoKeys/*/cryptoKeyVersions/*`.
            /// This option enables full flexibility in the key's capabilities and
            /// properties.
            #[prost(string, tag = "1")]
            CloudKmsKeyVersion(std::string::String),
            /// Required. The algorithm to use for creating a managed Cloud KMS key for a for a
            /// simplified experience. All managed keys will be have their
            /// [ProtectionLevel][google.cloud.kms.v1.ProtectionLevel] as `HSM`.
            #[prost(enumeration = "super::SignHashAlgorithm", tag = "2")]
            Algorithm(i32),
        }
    }
    /// The type of a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority], indicating its issuing chain.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Not specified.
        Unspecified = 0,
        /// Self-signed CA.
        SelfSigned = 1,
        /// Subordinate CA. Could be issued by a Private CA [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]
        /// or an unmanaged CA.
        Subordinate = 2,
    }
    /// The tier of a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority], indicating its supported
    /// functionality and/or billing SKU.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Tier {
        /// Not specified.
        Unspecified = 0,
        /// Enterprise tier.
        Enterprise = 1,
        /// DevOps tier.
        Devops = 2,
    }
    /// The state of a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority], indicating if it can be used.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified.
        Unspecified = 0,
        /// Certificates can be issued from this CA. CRLs will be generated for this
        /// CA.
        Enabled = 1,
        /// Certificates cannot be issued from this CA. CRLs will still be generated.
        Disabled = 2,
        /// Certificates cannot be issued from this CA. CRLs will not be generated.
        PendingActivation = 3,
        /// Certificates cannot be issued from this CA. CRLs will not be generated.
        PendingDeletion = 4,
    }
    /// The algorithm of a Cloud KMS CryptoKeyVersion of a
    /// [CryptoKey][google.cloud.kms.v1.CryptoKey] with the
    /// [CryptoKeyPurpose][google.cloud.kms.v1.CryptoKey.CryptoKeyPurpose] value
    /// `ASYMMETRIC_SIGN`. These values correspond to the
    /// [CryptoKeyVersionAlgorithm][google.cloud.kms.v1.CryptoKey.CryptoKeyVersion.CryptoKeyVersionAlgorithm].
    /// values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SignHashAlgorithm {
        /// Not specified.
        Unspecified = 0,
        /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PKCS1_2048_SHA256
        Rsa2048Sha256 = 1,
        /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PKCS1_3072_SHA256
        Rsa3072Sha256 = 2,
        /// maps to CryptoKeyVersionAlgorithm.RSA_SIGN_PKCS1_4096_SHA256
        Rsa4096Sha256 = 3,
        /// maps to CryptoKeyVersionAlgorithm.EC_SIGN_P256_SHA256
        EcP256Sha256 = 4,
        /// maps to CryptoKeyVersionAlgorithm.EC_SIGN_P384_SHA256
        EcP384Sha256 = 5,
        /// maps to CryptoKeyVersionAlgorithm.EC_SIGN_P521_SHA256
        EcP521Sha256 = 6,
    }
}
/// A [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] corresponds to a signed X.509 certificate
/// Revocation List (CRL). A CRL contains the serial numbers of certificates that
/// should no longer be trusted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateRevocationList {
    /// Output only. The resource path for this [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] in
    /// the format
    /// `projects/*/locations/*/certificateAuthorities/*/
    ///    certificateRevocationLists/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The CRL sequence number that appears in pem_crl.
    #[prost(int64, tag = "2")]
    pub sequence_number: i64,
    /// Output only. The revoked serial numbers that appear in pem_crl.
    #[prost(message, repeated, tag = "3")]
    pub revoked_certificates: ::std::vec::Vec<certificate_revocation_list::RevokedCertificate>,
    /// Output only. The PEM-encoded X.509 CRL.
    #[prost(string, tag = "4")]
    pub pem_crl: std::string::String,
    /// Output only. The location where 'pem_crl' can be accessed.
    #[prost(string, tag = "5")]
    pub access_url: std::string::String,
    /// Output only. The [State][google.cloud.security.privateca.v1beta1.CertificateRevocationList.State] for this [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList].
    #[prost(enumeration = "certificate_revocation_list::State", tag = "6")]
    pub state: i32,
    /// Output only. The time at which this [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] was updated.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels with user-defined metadata.
    #[prost(map = "string, string", tag = "9")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
pub mod certificate_revocation_list {
    /// Describes a revoked [Certificate][google.cloud.security.privateca.v1beta1.Certificate].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RevokedCertificate {
        /// The resource path for the [Certificate][google.cloud.security.privateca.v1beta1.Certificate] in the format
        /// `projects/*/locations/*/certificateAuthorities/*/certificates/*`.
        #[prost(string, tag = "1")]
        pub certificate: std::string::String,
        /// The serial number of the [Certificate][google.cloud.security.privateca.v1beta1.Certificate].
        #[prost(string, tag = "2")]
        pub hex_serial_number: std::string::String,
        /// The reason the [Certificate][google.cloud.security.privateca.v1beta1.Certificate] was revoked.
        #[prost(enumeration = "super::RevocationReason", tag = "3")]
        pub revocation_reason: i32,
    }
    /// The state of a [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList], indicating if it is current.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified.
        Unspecified = 0,
        /// The [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] is up to date.
        Active = 1,
        /// The [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] is no longer current.
        Superseded = 2,
    }
}
/// A [Certificate][google.cloud.security.privateca.v1beta1.Certificate] corresponds to a signed X.509 certificate issued by a
/// [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Certificate {
    /// Output only. The resource path for this [Certificate][google.cloud.security.privateca.v1beta1.Certificate] in the format
    /// `projects/*/locations/*/certificateAuthorities/*/certificates/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The desired lifetime of a certificate. Used to create the
    /// "not_before_time" and "not_after_time" fields inside an X.509
    /// certificate. Note that the lifetime may be truncated if it would extend
    /// past the life of any certificate authority in the issuing chain.
    #[prost(message, optional, tag = "4")]
    pub lifetime: ::std::option::Option<::prost_types::Duration>,
    /// Output only. Details regarding the revocation of this [Certificate][google.cloud.security.privateca.v1beta1.Certificate]. This
    /// [Certificate][google.cloud.security.privateca.v1beta1.Certificate] is considered revoked if and only if this field is present.
    #[prost(message, optional, tag = "5")]
    pub revocation_details: ::std::option::Option<certificate::RevocationDetails>,
    /// Output only. The pem-encoded, signed X.509 certificate.
    #[prost(string, tag = "6")]
    pub pem_certificate: std::string::String,
    /// Output only. A structured description of the issued X.509 certificate.
    #[prost(message, optional, tag = "7")]
    pub certificate_description: ::std::option::Option<CertificateDescription>,
    /// Output only. The chain that may be used to verify the X.509 certificate. Expected to be
    /// in issuer-to-root order according to RFC 5246.
    #[prost(string, repeated, tag = "8")]
    pub pem_certificate_chain: ::std::vec::Vec<std::string::String>,
    /// Output only. The time at which this [Certificate][google.cloud.security.privateca.v1beta1.Certificate] was created.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this [Certificate][google.cloud.security.privateca.v1beta1.Certificate] was updated.
    #[prost(message, optional, tag = "10")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels with user-defined metadata.
    #[prost(map = "string, string", tag = "11")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The config used to create a signed X.509 certificate.
    #[prost(oneof = "certificate::CertificateConfig", tags = "2, 3")]
    pub certificate_config: ::std::option::Option<certificate::CertificateConfig>,
}
pub mod certificate {
    /// Describes fields that are relavent to the revocation of a [Certificate][google.cloud.security.privateca.v1beta1.Certificate].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RevocationDetails {
        /// Indicates why a [Certificate][google.cloud.security.privateca.v1beta1.Certificate] was revoked.
        #[prost(enumeration = "super::RevocationReason", tag = "1")]
        pub revocation_state: i32,
        /// The time at which this [Certificate][google.cloud.security.privateca.v1beta1.Certificate] was revoked.
        #[prost(message, optional, tag = "2")]
        pub revocation_time: ::std::option::Option<::prost_types::Timestamp>,
    }
    /// The config used to create a signed X.509 certificate.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CertificateConfig {
        /// Immutable. A pem-encoded X.509 certificate signing request (CSR).
        #[prost(string, tag = "2")]
        PemCsr(std::string::String),
        /// Immutable. A description of the certificate and key that does not require X.509 or
        /// ASN.1.
        #[prost(message, tag = "3")]
        Config(super::CertificateConfig),
    }
}
/// A [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig] refers to a managed [ReusableConfigValues][google.cloud.security.privateca.v1beta1.ReusableConfigValues]. Those, in
/// turn, are used to describe certain fields of an X.509 certificate, such as
/// the key usage fields, fields specific to CA certificates, certificate policy
/// extensions and custom extensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReusableConfig {
    /// Output only. The resource path for this [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig] in the format
    /// `projects/*/locations/*/reusableConfigs/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The config values.
    #[prost(message, optional, tag = "2")]
    pub values: ::std::option::Option<ReusableConfigValues>,
    /// Optional. A human-readable description of scenarios these ReusableConfigValues may be
    /// compatible with.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// Output only. The time at which this [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig] was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig] was updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. Labels with user-defined metadata.
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// A [ReusableConfigValues][google.cloud.security.privateca.v1beta1.ReusableConfigValues] is used to describe certain fields of an
/// X.509 certificate, such as the key usage fields, fields specific to CA
/// certificates, certificate policy extensions and custom extensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReusableConfigValues {
    /// Optional. Indicates the intended use for keys that correspond to a certificate.
    #[prost(message, optional, tag = "1")]
    pub key_usage: ::std::option::Option<KeyUsage>,
    /// Optional. Describes options in this [ReusableConfigValues][google.cloud.security.privateca.v1beta1.ReusableConfigValues] that are
    /// relevant in a CA certificate.
    #[prost(message, optional, tag = "2")]
    pub ca_options: ::std::option::Option<reusable_config_values::CaOptions>,
    /// Optional. Describes the X.509 certificate policy object identifiers, per
    /// https://tools.ietf.org/html/rfc5280#section-4.2.1.4rfc5280
    #[prost(message, repeated, tag = "3")]
    pub policy_ids: ::std::vec::Vec<ObjectId>,
    /// Optional. Describes Online Certificate Status Protocol (OCSP) endpoint addresses
    /// that appear in the "Authority Information Access" extension in the
    /// certificate.
    #[prost(string, repeated, tag = "4")]
    pub aia_ocsp_servers: ::std::vec::Vec<std::string::String>,
    /// Optional. Describes custom X.509 extensions.
    #[prost(message, repeated, tag = "5")]
    pub additional_extensions: ::std::vec::Vec<X509Extension>,
}
pub mod reusable_config_values {
    /// Describes values that are relevant in a CA certificate.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CaOptions {
        /// Optional. Refers to the "CA" X.509 extension, which is a boolean value. When this
        /// value is missing, the extension will be omitted from the CA certificate.
        #[prost(message, optional, tag = "1")]
        pub is_ca: ::std::option::Option<bool>,
        /// Optional. Refers to the path length restriction X.509 extension. For a CA
        /// certificate, this value describes the depth of subordinate CA
        /// certificates that are allowed.
        /// If this value is less than 0, the request will fail.
        /// If this value is missing, the max path length will be omitted from the
        /// CA certificate.
        #[prost(message, optional, tag = "2")]
        pub max_issuer_path_length: ::std::option::Option<i32>,
    }
}
/// A [ReusableConfigWrapper][google.cloud.security.privateca.v1beta1.ReusableConfigWrapper] describes values that may assist in creating an
/// X.509 certificate, or a reference to a pre-defined set of values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReusableConfigWrapper {
    /// Reusable or inline config values.
    #[prost(oneof = "reusable_config_wrapper::ConfigValues", tags = "1, 2")]
    pub config_values: ::std::option::Option<reusable_config_wrapper::ConfigValues>,
}
pub mod reusable_config_wrapper {
    /// Reusable or inline config values.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigValues {
        /// Required. A resource path to a [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig] in the format
        /// `projects/*/locations/*/reusableConfigs/*`.
        #[prost(string, tag = "1")]
        ReusableConfig(std::string::String),
        /// Required. A user-specified inline [ReusableConfigValues][google.cloud.security.privateca.v1beta1.ReusableConfigValues].
        #[prost(message, tag = "2")]
        ReusableConfigValues(super::ReusableConfigValues),
    }
}
/// A [PublicKey][google.cloud.security.privateca.v1beta1.PublicKey] describes a public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublicKey {
    /// Required. The type of public key.
    #[prost(enumeration = "public_key::KeyType", tag = "1")]
    pub r#type: i32,
    /// Required. A public key. Padding and encoding varies by 'KeyType' and is described
    /// along with the KeyType values.
    #[prost(bytes, tag = "2")]
    pub key: std::vec::Vec<u8>,
}
pub mod public_key {
    /// Types of public keys that are supported.
    /// At a minimum, we support RSA and ECDSA, for the key sizes or curves listed:
    /// https://cloud.google.com/kms/docs/algorithms#asymmetric_signing_algorithms
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeyType {
        /// Default unspecified value.
        Unspecified = 0,
        /// A PEM-encoded PKCS#1/RFC 3447 RSAPrivateKey structure.
        PemRsaKey = 1,
        /// A PEM-encoded compressed NIST P-256/secp256r1/prime256v1 or P-384 key.
        PemEcKey = 2,
    }
}
/// A [CertificateConfig][google.cloud.security.privateca.v1beta1.CertificateConfig] describes an X.509 certificate or CSR that is to be
/// created, as an alternative to using ASN.1.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateConfig {
    /// Required. Specifies some of the values in a certificate that are related to the
    /// subject.
    #[prost(message, optional, tag = "1")]
    pub subject_config: ::std::option::Option<certificate_config::SubjectConfig>,
    /// Required. Describes how some of the technical fields in a certificate should be
    /// populated.
    #[prost(message, optional, tag = "2")]
    pub reusable_config: ::std::option::Option<ReusableConfigWrapper>,
    /// Optional. The public key that corresponds to this config. This is, for example, used
    /// when issuing [Certificates][google.cloud.security.privateca.v1beta1.Certificate], but not when creating a
    /// self-signed [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] or [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] CSR.
    #[prost(message, optional, tag = "3")]
    pub public_key: ::std::option::Option<PublicKey>,
}
pub mod certificate_config {
    /// These values are used to create the distinguished name and subject
    /// alternative name fields in an X.509 certificate.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubjectConfig {
        /// Required. Contains distinguished name fields such as the location and organization.
        #[prost(message, optional, tag = "1")]
        pub subject: ::std::option::Option<super::Subject>,
        /// Optional. The "common name" of the distinguished name.
        #[prost(string, tag = "2")]
        pub common_name: std::string::String,
        /// Optional. The subject alternative name fields.
        #[prost(message, optional, tag = "3")]
        pub subject_alt_name: ::std::option::Option<super::SubjectAltNames>,
    }
}
/// A [CertificateDescription][google.cloud.security.privateca.v1beta1.CertificateDescription] describes an X.509 certificate or CSR that has
/// been issued, as an alternative to using ASN.1 / X.509.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateDescription {
    /// Describes some of the values in a certificate that are related to the
    /// subject and lifetime.
    #[prost(message, optional, tag = "1")]
    pub subject_description: ::std::option::Option<certificate_description::SubjectDescription>,
    /// Describes some of the technical fields in a certificate.
    #[prost(message, optional, tag = "2")]
    pub config_values: ::std::option::Option<ReusableConfigValues>,
    /// The public key that corresponds to an issued certificate.
    #[prost(message, optional, tag = "3")]
    pub public_key: ::std::option::Option<PublicKey>,
    /// Provides a means of identifiying certificates that contain a particular
    /// public key, per https://tools.ietf.org/html/rfc5280#section-4.2.1.2.
    #[prost(message, optional, tag = "4")]
    pub subject_key_id: ::std::option::Option<certificate_description::KeyId>,
    /// Identifies the subject_key_id of the parent certificate, per
    /// https://tools.ietf.org/html/rfc5280#section-4.2.1.1
    #[prost(message, optional, tag = "5")]
    pub authority_key_id: ::std::option::Option<certificate_description::KeyId>,
    /// Describes a list of locations to obtain CRL information, i.e.
    /// the DistributionPoint.fullName described by
    /// https://tools.ietf.org/html/rfc5280#section-4.2.1.13
    #[prost(string, repeated, tag = "6")]
    pub crl_distribution_points: ::std::vec::Vec<std::string::String>,
    /// Describes lists of issuer CA certificate URLs that appear in the
    /// "Authority Information Access" extension in the certificate.
    #[prost(string, repeated, tag = "7")]
    pub aia_issuing_certificate_urls: ::std::vec::Vec<std::string::String>,
}
pub mod certificate_description {
    /// These values describe fields in an issued X.509 certificate such as the
    /// distinguished name, subject alternative names, serial number, and lifetime.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubjectDescription {
        /// Contains distinguished name fields such as the location and organization.
        #[prost(message, optional, tag = "1")]
        pub subject: ::std::option::Option<super::Subject>,
        /// The "common name" of the distinguished name.
        #[prost(string, tag = "2")]
        pub common_name: std::string::String,
        /// The subject alternative name fields.
        #[prost(message, optional, tag = "3")]
        pub subject_alt_name: ::std::option::Option<super::SubjectAltNames>,
        /// The serial number encoded in lowercase hexadecimal.
        #[prost(string, tag = "4")]
        pub hex_serial_number: std::string::String,
        /// For convenience, the actual lifetime of an issued certificate.
        /// Corresponds to 'not_after_time' - 'not_before_time'.
        #[prost(message, optional, tag = "5")]
        pub lifetime: ::std::option::Option<::prost_types::Duration>,
        /// The time at which the certificate becomes valid.
        #[prost(message, optional, tag = "6")]
        pub not_before_time: ::std::option::Option<::prost_types::Timestamp>,
        /// The time at which the certificate expires.
        #[prost(message, optional, tag = "7")]
        pub not_after_time: ::std::option::Option<::prost_types::Timestamp>,
    }
    /// A KeyId identifies a specific public key, usually by hashing the public
    /// key.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyId {
        /// Optional. The value of this KeyId encoded in lowercase hexadecimal. This is most
        /// likely the 160 bit SHA-1 hash of the public key.
        #[prost(string, tag = "1")]
        pub key_id: std::string::String,
    }
}
/// An [ObjectId][google.cloud.security.privateca.v1beta1.ObjectId] specifies an object identifier (OID). These provide context
/// and describe types in ASN.1 messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectId {
    /// Required. The parts of an OID path. The most significant parts of the path come
    /// first.
    #[prost(int32, repeated, packed = "false", tag = "1")]
    pub object_id_path: ::std::vec::Vec<i32>,
}
/// An [X509Extension][google.cloud.security.privateca.v1beta1.X509Extension] specifies an X.509 extension, which may be used in
/// different parts of X.509 objects like certificates, CSRs, and CRLs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct X509Extension {
    /// Required. The OID for this X.509 extension.
    #[prost(message, optional, tag = "1")]
    pub object_id: ::std::option::Option<ObjectId>,
    /// Required. Indicates whether or not this extension is critical (i.e., if the client
    /// does not know how to handle this extension, the client should consider this
    /// to be an error).
    #[prost(bool, tag = "2")]
    pub critical: bool,
    /// Required. The value of this X.509 extension.
    #[prost(bytes, tag = "3")]
    pub value: std::vec::Vec<u8>,
}
/// A [KeyUsage][google.cloud.security.privateca.v1beta1.KeyUsage] describes key usage values that may appear in an X.509
/// certificate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyUsage {
    /// Describes high-level ways in which a key may be used.
    #[prost(message, optional, tag = "1")]
    pub base_key_usage: ::std::option::Option<key_usage::KeyUsageOptions>,
    /// Detailed scenarios in which a key may be used.
    #[prost(message, optional, tag = "2")]
    pub extended_key_usage: ::std::option::Option<key_usage::ExtendedKeyUsageOptions>,
    /// Used to describe extended key usages that are not listed in the
    /// [KeyUsage.ExtendedKeyUsageOptions][google.cloud.security.privateca.v1beta1.KeyUsage.ExtendedKeyUsageOptions] message.
    #[prost(message, repeated, tag = "3")]
    pub unknown_extended_key_usages: ::std::vec::Vec<ObjectId>,
}
pub mod key_usage {
    /// [KeyUsage.KeyUsageOptions][google.cloud.security.privateca.v1beta1.KeyUsage.KeyUsageOptions] corresponds to the key usage values
    /// described in https://tools.ietf.org/html/rfc5280#section-4.2.1.3.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyUsageOptions {
        /// The key may be used for digital signatures.
        #[prost(bool, tag = "1")]
        pub digital_signature: bool,
        /// The key may be used for cryptographic commitments. Note that this may
        /// also be referred to as "non-repudiation".
        #[prost(bool, tag = "2")]
        pub content_commitment: bool,
        /// The key may be used to encipher other keys.
        #[prost(bool, tag = "3")]
        pub key_encipherment: bool,
        /// The key may be used to encipher data.
        #[prost(bool, tag = "4")]
        pub data_encipherment: bool,
        /// The key may be used in a key agreement protocol.
        #[prost(bool, tag = "5")]
        pub key_agreement: bool,
        /// The key may be used to sign certificates.
        #[prost(bool, tag = "6")]
        pub cert_sign: bool,
        /// The key may be used sign certificate revocation lists.
        #[prost(bool, tag = "7")]
        pub crl_sign: bool,
        /// The key may be used to encipher only.
        #[prost(bool, tag = "8")]
        pub encipher_only: bool,
        /// The key may be used to decipher only.
        #[prost(bool, tag = "9")]
        pub decipher_only: bool,
    }
    /// [KeyUsage.ExtendedKeyUsageOptions][google.cloud.security.privateca.v1beta1.KeyUsage.ExtendedKeyUsageOptions] has fields that correspond to
    /// certain common OIDs that could be specified as an extended key usage value.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExtendedKeyUsageOptions {
        /// Corresponds to OID 1.3.6.1.5.5.7.3.1. Officially described as "TLS WWW
        /// server authentication", though regularly used for non-WWW TLS.
        #[prost(bool, tag = "1")]
        pub server_auth: bool,
        /// Corresponds to OID 1.3.6.1.5.5.7.3.2. Officially described as "TLS WWW
        /// client authentication", though regularly used for non-WWW TLS.
        #[prost(bool, tag = "2")]
        pub client_auth: bool,
        /// Corresponds to OID 1.3.6.1.5.5.7.3.3. Officially described as "Signing of
        /// downloadable executable code client authentication".
        #[prost(bool, tag = "3")]
        pub code_signing: bool,
        /// Corresponds to OID 1.3.6.1.5.5.7.3.4. Officially described as "Email
        /// protection".
        #[prost(bool, tag = "4")]
        pub email_protection: bool,
        /// Corresponds to OID 1.3.6.1.5.5.7.3.8. Officially described as "Binding
        /// the hash of an object to a time".
        #[prost(bool, tag = "5")]
        pub time_stamping: bool,
        /// Corresponds to OID 1.3.6.1.5.5.7.3.9. Officially described as "Signing
        /// OCSP responses".
        #[prost(bool, tag = "6")]
        pub ocsp_signing: bool,
    }
}
/// [Subject][google.cloud.security.privateca.v1beta1.Subject] describes parts of a distinguished name that, in turn,
/// describes the subject of the certificate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subject {
    /// The country code of the subject.
    #[prost(string, tag = "1")]
    pub country_code: std::string::String,
    /// The organization of the subject.
    #[prost(string, tag = "2")]
    pub organization: std::string::String,
    /// The organizational_unit of the subject.
    #[prost(string, tag = "3")]
    pub organizational_unit: std::string::String,
    /// The locality or city of the subject.
    #[prost(string, tag = "4")]
    pub locality: std::string::String,
    /// The province, territory, or regional state of the subject.
    #[prost(string, tag = "5")]
    pub province: std::string::String,
    /// The street address of the subject.
    #[prost(string, tag = "6")]
    pub street_address: std::string::String,
    /// The postal code of the subject.
    #[prost(string, tag = "7")]
    pub postal_code: std::string::String,
}
/// [SubjectAltNames][google.cloud.security.privateca.v1beta1.SubjectAltNames] corresponds to a more modern way of listing what
/// the asserted identity is in a certificate (i.e., compared to the "common
/// name" in the distinguished name).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectAltNames {
    /// Contains only valid, fully-qualified host names.
    #[prost(string, repeated, tag = "1")]
    pub dns_names: ::std::vec::Vec<std::string::String>,
    /// Contains only valid RFC 3986 URIs.
    #[prost(string, repeated, tag = "2")]
    pub uris: ::std::vec::Vec<std::string::String>,
    /// Contains only valid RFC 2822 E-mail addresses.
    #[prost(string, repeated, tag = "3")]
    pub email_addresses: ::std::vec::Vec<std::string::String>,
    /// Contains only valid 32-bit IPv4 addresses or RFC 4291 IPv6 addresses.
    #[prost(string, repeated, tag = "4")]
    pub ip_addresses: ::std::vec::Vec<std::string::String>,
    /// Contains additional subject alternative name values.
    #[prost(message, repeated, tag = "5")]
    pub custom_sans: ::std::vec::Vec<X509Extension>,
}
/// A [RevocationReason][google.cloud.security.privateca.v1beta1.RevocationReason] indicates whether a [Certificate][google.cloud.security.privateca.v1beta1.Certificate] has been revoked,
/// and the reason for revocation. These are standard revocation reasons from RFC
/// 5280.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RevocationReason {
    /// Default unspecified value. This value does indicate that a [Certificate][google.cloud.security.privateca.v1beta1.Certificate]
    /// has been revoked, but that a reason has not been recorded.
    Unspecified = 0,
    /// Key material for this [Certificate][google.cloud.security.privateca.v1beta1.Certificate] may have leaked.
    KeyCompromise = 1,
    /// The key material for a certificate authority in the issuing path may have
    /// leaked.
    CertificateAuthorityCompromise = 2,
    /// The subject or other attributes in this [Certificate][google.cloud.security.privateca.v1beta1.Certificate] have changed.
    AffiliationChanged = 3,
    /// This [Certificate][google.cloud.security.privateca.v1beta1.Certificate] has been superseded.
    Superseded = 4,
    /// This [Certificate][google.cloud.security.privateca.v1beta1.Certificate] or entities in the issuing path have ceased to
    /// operate.
    CessationOfOperation = 5,
    /// This [Certificate][google.cloud.security.privateca.v1beta1.Certificate] should not be considered valid, it is expected that it
    /// may become valid in the future.
    CertificateHold = 6,
    /// This [Certificate][google.cloud.security.privateca.v1beta1.Certificate] no longer has permission to assert the listed
    /// attributes.
    PrivilegeWithdrawn = 7,
    /// The authority which determines appropriate attributes for a [Certificate][google.cloud.security.privateca.v1beta1.Certificate]
    /// may have been compromised.
    AttributeAuthorityCompromise = 8,
}
/// Request message for
/// [CertificateAuthorityService.ListCertificateAuthorities][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.ListCertificateAuthorities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateAuthoritiesRequest {
    /// Required. The resource name of the location associated with the
    /// [CertificateAuthorities][google.cloud.security.privateca.v1beta1.CertificateAuthority], in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Limit on the number of [CertificateAuthorities][google.cloud.security.privateca.v1beta1.CertificateAuthority] to
    /// include in the response.
    /// Further [CertificateAuthorities][google.cloud.security.privateca.v1beta1.CertificateAuthority] can subsequently be
    /// obtained by including the
    /// [ListCertificateAuthoritiesResponse.next_page_token][google.cloud.security.privateca.v1beta1.ListCertificateAuthoritiesResponse.next_page_token] in a subsequent
    /// request. If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListCertificateAuthoritiesResponse.next_page_token][google.cloud.security.privateca.v1beta1.ListCertificateAuthoritiesResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. Only include resources that match the filter in the response.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specify how the results should be sorted.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.ListCertificateRevocationLists][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.ListCertificateRevocationLists].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateRevocationListsRequest {
    /// Required. The resource name of the location associated with the
    /// [CertificateRevocationLists][google.cloud.security.privateca.v1beta1.CertificateRevocationList], in the format
    /// `projects/*/locations/*/certificateauthorities/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Limit on the number of
    /// [CertificateRevocationLists][google.cloud.security.privateca.v1beta1.CertificateRevocationList] to include in the
    /// response. Further [CertificateRevocationLists][google.cloud.security.privateca.v1beta1.CertificateRevocationList]
    /// can subsequently be obtained by including the
    /// [ListCertificateRevocationListsResponse.next_page_token][google.cloud.security.privateca.v1beta1.ListCertificateRevocationListsResponse.next_page_token] in a subsequent
    /// request. If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListCertificateRevocationListsResponse.next_page_token][google.cloud.security.privateca.v1beta1.ListCertificateRevocationListsResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. Only include resources that match the filter in the response.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specify how the results should be sorted.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Request message for [CertificateAuthorityService.ListCertificates][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.ListCertificates].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificatesRequest {
    /// Required. The resource name of the location associated with the
    /// [Certificates][google.cloud.security.privateca.v1beta1.Certificate], in the format
    /// `projects/*/locations/*/certificateauthorities/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Limit on the number of
    /// [Certificates][google.cloud.security.privateca.v1beta1.Certificate] to include in the
    /// response. Further [Certificates][google.cloud.security.privateca.v1beta1.Certificate] can subsequently be obtained
    /// by including the
    /// [ListCertificatesResponse.next_page_token][google.cloud.security.privateca.v1beta1.ListCertificatesResponse.next_page_token] in a subsequent
    /// request. If unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListCertificatesResponse.next_page_token][google.cloud.security.privateca.v1beta1.ListCertificatesResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. Only include resources that match the filter in the response.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specify how the results should be sorted.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.ListReusableConfigs][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.ListReusableConfigs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReusableConfigsRequest {
    /// Required. The resource name of the location associated with the
    /// [ReusableConfigs][google.cloud.security.privateca.v1beta1.ReusableConfig], in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Limit on the number of
    /// [ReusableConfigs][google.cloud.security.privateca.v1beta1.ReusableConfig] to include in the response.
    /// Further [ReusableConfigs][google.cloud.security.privateca.v1beta1.ReusableConfig] can subsequently be
    /// obtained by including the
    /// [ListReusableConfigsResponse.next_page_token][google.cloud.security.privateca.v1beta1.ListReusableConfigsResponse.next_page_token] in a subsequent request. If
    /// unspecified, the server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListReusableConfigsResponse.next_page_token][google.cloud.security.privateca.v1beta1.ListReusableConfigsResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. Only include resources that match the filter in the response.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specify how the results should be sorted.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Response message for
/// [CertificateAuthorityService.ListCertificateAuthorities][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.ListCertificateAuthorities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateAuthoritiesResponse {
    /// The list of [CertificateAuthorities][google.cloud.security.privateca.v1beta1.CertificateAuthority].
    #[prost(message, repeated, tag = "1")]
    pub certificate_authorities: ::std::vec::Vec<CertificateAuthority>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListCertificateAuthoritiesRequest.next_page_token][] to retrieve the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Response message for
/// [CertificateAuthorityService.ListCertificateRevocationLists][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.ListCertificateRevocationLists].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificateRevocationListsResponse {
    /// The list of [CertificateRevocationLists][google.cloud.security.privateca.v1beta1.CertificateRevocationList].
    #[prost(message, repeated, tag = "1")]
    pub certificate_revocation_lists: ::std::vec::Vec<CertificateRevocationList>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListCertificateRevocationListsRequest.next_page_token][] to retrieve the
    /// next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Response message for [CertificateAuthorityService.ListCertificates][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.ListCertificates].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCertificatesResponse {
    /// The list of [Certificates][google.cloud.security.privateca.v1beta1.Certificate].
    #[prost(message, repeated, tag = "1")]
    pub certificates: ::std::vec::Vec<Certificate>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListCertificatesRequest.next_page_token][] to retrieve the
    /// next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Response message for
/// [CertificateAuthorityService.ListReusableConfigs][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.ListReusableConfigs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReusableConfigsResponse {
    /// The list of [ReusableConfigs][google.cloud.security.privateca.v1beta1.ReusableConfig].
    #[prost(message, repeated, tag = "1")]
    pub reusable_configs: ::std::vec::Vec<ReusableConfig>,
    /// A token to retrieve next page of results. Pass this value in
    /// [ListReusableConfigsRequest.next_page_token][] to retrieve
    /// the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// A list of locations (e.g. "us-west1") that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request message for [CertificateAuthorityService.GetCertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.GetCertificateAuthority].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateAuthorityRequest {
    /// Required. The [name][google.cloud.security.privateca.v1beta1.CertificateAuthority.name] of the [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] to
    /// get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.GetCertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.GetCertificateRevocationList].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateRevocationListRequest {
    /// Required. The [name][google.cloud.security.privateca.v1beta1.CertificateRevocationList.name] of the
    /// [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] to get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.GetCertificate][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.GetCertificate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateRequest {
    /// Required. The [name][google.cloud.security.privateca.v1beta1.Certificate.name] of the [Certificate][google.cloud.security.privateca.v1beta1.Certificate] to get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.GetReusableConfig][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.GetReusableConfig].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReusableConfigRequest {
    /// Required. The [name][ReusableConfigs.name] of the [ReusableConfigs][] to get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.CreateCertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.CreateCertificateAuthority].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCertificateAuthorityRequest {
    /// Required. The resource name of the location associated with the
    /// [CertificateAuthorities][google.cloud.security.privateca.v1beta1.CertificateAuthority], in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. It must be unique within a location and match the regular
    /// expression `[a-zA-Z0-9-]{1,63}`
    #[prost(string, tag = "2")]
    pub certificate_authority_id: std::string::String,
    /// Required. A [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub certificate_authority: ::std::option::Option<CertificateAuthority>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.CreateCertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.CreateCertificateRevocationList].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCertificateRevocationListRequest {
    /// Required. The resource name of the location and [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]
    /// associated with the [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList], in the format
    /// `projects/*/locations/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. It must be unique within a location and match the regular expression
    /// `[a-zA-Z0-9-]{1,63}`
    #[prost(string, tag = "2")]
    pub certificate_revocation_list_id: std::string::String,
    /// Required. A [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub certificate_revocation_list: ::std::option::Option<CertificateRevocationList>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: std::string::String,
}
/// Request message for [CertificateAuthorityService.CreateCertificate][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.CreateCertificate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCertificateRequest {
    /// Required. The resource name of the location and [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]
    /// associated with the [Certificate][google.cloud.security.privateca.v1beta1.Certificate], in the format
    /// `projects/*/locations/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. It must be unique within a location and match the regular
    /// expression `[a-zA-Z0-9-]{1,63}`
    #[prost(string, tag = "2")]
    pub certificate_id: std::string::String,
    /// Required. A [Certificate][google.cloud.security.privateca.v1beta1.Certificate] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub certificate: ::std::option::Option<Certificate>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.CreateReusableConfig][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.CreateReusableConfig].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReusableConfigRequest {
    /// Required. The resource name of the location associated with the
    /// [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig], in the format
    /// `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. It must be unique within a location and match the regular
    /// expression `[a-zA-Z0-9-]{1,63}`
    #[prost(string, tag = "2")]
    pub reusable_config_id: std::string::String,
    /// Required. A [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub reusable_config: ::std::option::Option<ReusableConfig>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.UpdateCertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.UpdateCertificateAuthority].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCertificateAuthorityRequest {
    /// Required. [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] with updated values.
    #[prost(message, optional, tag = "1")]
    pub certificate_authority: ::std::option::Option<CertificateAuthority>,
    /// Required. A list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.UpdateCertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.UpdateCertificateRevocationList].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCertificateRevocationListRequest {
    /// Required. [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] with updated values.
    #[prost(message, optional, tag = "1")]
    pub certificate_revocation_list: ::std::option::Option<CertificateRevocationList>,
    /// Required. A list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: std::string::String,
}
/// Request message for [CertificateAuthorityService.UpdateCertificate][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.UpdateCertificate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCertificateRequest {
    /// Required. [Certificate][google.cloud.security.privateca.v1beta1.Certificate] with updated values.
    #[prost(message, optional, tag = "1")]
    pub certificate: ::std::option::Option<Certificate>,
    /// Required. A list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.UpdateReusableConfig][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.UpdateReusableConfig].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReusableConfigRequest {
    /// Required. [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig] with updated values.
    #[prost(message, optional, tag = "1")]
    pub reusable_config: ::std::option::Option<ReusableConfig>,
    /// Required. A list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.GetCertificateAuthorityCsr][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.GetCertificateAuthorityCsr].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateAuthorityCsrRequest {
    /// Required. The resource name for this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] in the
    /// format `projects/*/locations/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for
/// [CertificateAuthorityService.GetCertificateAuthorityCsr][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.GetCertificateAuthorityCsr].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCertificateAuthorityCsrResponse {
    /// Output only. The PEM-encoded signed certificate signing request (CSR).
    #[prost(string, tag = "1")]
    pub pem_csr: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.ActivateCertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.ActivateCertificateAuthority].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivateCertificateAuthorityRequest {
    /// Required. The resource name for this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] in the
    /// format `projects/*/locations/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The signed CA certificate issued from
    /// [GetCertificateAuthorityCsrResponse.pem_csr][google.cloud.security.privateca.v1beta1.GetCertificateAuthorityCsrResponse.pem_csr].
    #[prost(string, tag = "2")]
    pub pem_ca_certificate: std::string::String,
    /// Required. Must include the issuer of 'pem_ca_certificate', and any further issuers
    /// until the self-signed CA. Expected to be in issuer-to-root order according
    /// to RFC 5246.
    #[prost(string, repeated, tag = "3")]
    pub pem_ca_certificate_chain: ::std::vec::Vec<std::string::String>,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.DisableCertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.DisableCertificateAuthority].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableCertificateAuthorityRequest {
    /// Required. The resource name for this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] in the
    /// format `projects/*/locations/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.EnableCertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.EnableCertificateAuthority].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableCertificateAuthorityRequest {
    /// Required. The resource name for this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] in the
    /// format `projects/*/locations/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.ScheduleDeleteCertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.ScheduleDeleteCertificateAuthority].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleDeleteCertificateAuthorityRequest {
    /// Required. The resource name for this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] in the
    /// format `projects/*/locations/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.RestoreCertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.RestoreCertificateAuthority].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreCertificateAuthorityRequest {
    /// Required. The resource name for this [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] in the
    /// format `projects/*/locations/*/certificateAuthorities/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
}
/// Request message for
/// [CertificateAuthorityService.RevokeCertificate][google.cloud.security.privateca.v1beta1.CertificateAuthorityService.RevokeCertificate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeCertificateRequest {
    /// Required. The resource name for this [Certificate][google.cloud.security.privateca.v1beta1.Certificate] in the
    /// format `projects/*/locations/*/certificateAuthorities/*/certificates/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The [RevocationReason][google.cloud.security.privateca.v1beta1.RevocationReason] for revoking this certificate.
    #[prost(enumeration = "RevocationReason", tag = "2")]
    pub reason: i32,
    /// Optional. An ID to identify requests. Specify a unique request ID so that if you must
    /// retry your request, the server will know to ignore the request if it has
    /// already been completed. The server will guarantee that for at least 60
    /// minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: std::string::String,
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: std::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: std::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: std::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod certificate_authority_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " [Certificate Authority Service][google.cloud.security.privateca.v1beta1.CertificateAuthorityService] manages private"]
    #[doc = " certificate authorities and issued certificates."]
    pub struct CertificateAuthorityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CertificateAuthorityServiceClient<T>
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
        #[doc = " Lists [CertificateAuthorities][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        pub async fn list_certificate_authorities(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCertificateAuthoritiesRequest>,
        ) -> Result<tonic::Response<super::ListCertificateAuthoritiesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ListCertificateAuthorities" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists [CertificateRevocationLists][google.cloud.security.privateca.v1beta1.CertificateRevocationList]."]
        pub async fn list_certificate_revocation_lists(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCertificateRevocationListsRequest>,
        ) -> Result<tonic::Response<super::ListCertificateRevocationListsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ListCertificateRevocationLists" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists [Certificates][google.cloud.security.privateca.v1beta1.Certificate]."]
        pub async fn list_certificates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCertificatesRequest>,
        ) -> Result<tonic::Response<super::ListCertificatesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ListCertificates" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists [ReusableConfigs][google.cloud.security.privateca.v1beta1.ReusableConfig]."]
        pub async fn list_reusable_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReusableConfigsRequest>,
        ) -> Result<tonic::Response<super::ListReusableConfigsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ListReusableConfigs" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        pub async fn get_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateAuthorityRequest>,
        ) -> Result<tonic::Response<super::CertificateAuthority>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/GetCertificateAuthority" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList]."]
        pub async fn get_certificate_revocation_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateRevocationListRequest>,
        ) -> Result<tonic::Response<super::CertificateRevocationList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/GetCertificateRevocationList" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a [Certificate][google.cloud.security.privateca.v1beta1.Certificate]."]
        pub async fn get_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateRequest>,
        ) -> Result<tonic::Response<super::Certificate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/GetCertificate" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig]."]
        pub async fn get_reusable_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReusableConfigRequest>,
        ) -> Result<tonic::Response<super::ReusableConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/GetReusableConfig" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] in a given Project and Location."]
        pub async fn create_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/CreateCertificateAuthority" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] in a given Project, Location"]
        #[doc = " for a particular [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        pub async fn create_certificate_revocation_list(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCertificateRevocationListRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/CreateCertificateRevocationList" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new [Certificate][google.cloud.security.privateca.v1beta1.Certificate] in a given Project, Location from a particular"]
        #[doc = " [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        pub async fn create_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCertificateRequest>,
        ) -> Result<tonic::Response<super::Certificate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/CreateCertificate" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig] in a given Project and Location."]
        pub async fn create_reusable_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReusableConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/CreateReusableConfig" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        pub async fn update_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/UpdateCertificateAuthority" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList]."]
        pub async fn update_certificate_revocation_list(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCertificateRevocationListRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/UpdateCertificateRevocationList" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a [Certificate][google.cloud.security.privateca.v1beta1.Certificate]."]
        pub async fn update_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCertificateRequest>,
        ) -> Result<tonic::Response<super::Certificate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/UpdateCertificate" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig]."]
        pub async fn update_reusable_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateReusableConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/UpdateReusableConfig" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the CSR for a pending [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        pub async fn get_certificate_authority_csr(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCertificateAuthorityCsrRequest>,
        ) -> Result<tonic::Response<super::GetCertificateAuthorityCsrResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/GetCertificateAuthorityCsr" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Activate a pending [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        pub async fn activate_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::ActivateCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ActivateCertificateAuthority" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Disable a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        pub async fn disable_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/DisableCertificateAuthority" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Enable a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        pub async fn enable_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/EnableCertificateAuthority" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Schedule a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] for deletion."]
        pub async fn schedule_delete_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::ScheduleDeleteCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ScheduleDeleteCertificateAuthority" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restore a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] that is scheduled for deletion."]
        pub async fn restore_certificate_authority(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/RestoreCertificateAuthority" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Revoke a [Certificate][google.cloud.security.privateca.v1beta1.Certificate]."]
        pub async fn revoke_certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeCertificateRequest>,
        ) -> Result<tonic::Response<super::Certificate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/RevokeCertificate" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CertificateAuthorityServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CertificateAuthorityServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CertificateAuthorityServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod certificate_authority_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CertificateAuthorityServiceServer."]
    #[async_trait]
    pub trait CertificateAuthorityService: Send + Sync + 'static {
        #[doc = " Lists [CertificateAuthorities][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        async fn list_certificate_authorities(
            &self,
            request: tonic::Request<super::ListCertificateAuthoritiesRequest>,
        ) -> Result<tonic::Response<super::ListCertificateAuthoritiesResponse>, tonic::Status>;
        #[doc = " Lists [CertificateRevocationLists][google.cloud.security.privateca.v1beta1.CertificateRevocationList]."]
        async fn list_certificate_revocation_lists(
            &self,
            request: tonic::Request<super::ListCertificateRevocationListsRequest>,
        ) -> Result<tonic::Response<super::ListCertificateRevocationListsResponse>, tonic::Status>;
        #[doc = " Lists [Certificates][google.cloud.security.privateca.v1beta1.Certificate]."]
        async fn list_certificates(
            &self,
            request: tonic::Request<super::ListCertificatesRequest>,
        ) -> Result<tonic::Response<super::ListCertificatesResponse>, tonic::Status>;
        #[doc = " Lists [ReusableConfigs][google.cloud.security.privateca.v1beta1.ReusableConfig]."]
        async fn list_reusable_configs(
            &self,
            request: tonic::Request<super::ListReusableConfigsRequest>,
        ) -> Result<tonic::Response<super::ListReusableConfigsResponse>, tonic::Status>;
        #[doc = " Returns a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        async fn get_certificate_authority(
            &self,
            request: tonic::Request<super::GetCertificateAuthorityRequest>,
        ) -> Result<tonic::Response<super::CertificateAuthority>, tonic::Status>;
        #[doc = " Returns a [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList]."]
        async fn get_certificate_revocation_list(
            &self,
            request: tonic::Request<super::GetCertificateRevocationListRequest>,
        ) -> Result<tonic::Response<super::CertificateRevocationList>, tonic::Status>;
        #[doc = " Returns a [Certificate][google.cloud.security.privateca.v1beta1.Certificate]."]
        async fn get_certificate(
            &self,
            request: tonic::Request<super::GetCertificateRequest>,
        ) -> Result<tonic::Response<super::Certificate>, tonic::Status>;
        #[doc = " Returns a [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig]."]
        async fn get_reusable_config(
            &self,
            request: tonic::Request<super::GetReusableConfigRequest>,
        ) -> Result<tonic::Response<super::ReusableConfig>, tonic::Status>;
        #[doc = " Create a new [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] in a given Project and Location."]
        async fn create_certificate_authority(
            &self,
            request: tonic::Request<super::CreateCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Create a new [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList] in a given Project, Location"]
        #[doc = " for a particular [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        async fn create_certificate_revocation_list(
            &self,
            request: tonic::Request<super::CreateCertificateRevocationListRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Create a new [Certificate][google.cloud.security.privateca.v1beta1.Certificate] in a given Project, Location from a particular"]
        #[doc = " [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        async fn create_certificate(
            &self,
            request: tonic::Request<super::CreateCertificateRequest>,
        ) -> Result<tonic::Response<super::Certificate>, tonic::Status>;
        #[doc = " Create a new [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig] in a given Project and Location."]
        async fn create_reusable_config(
            &self,
            request: tonic::Request<super::CreateReusableConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Update a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        async fn update_certificate_authority(
            &self,
            request: tonic::Request<super::UpdateCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Update a [CertificateRevocationList][google.cloud.security.privateca.v1beta1.CertificateRevocationList]."]
        async fn update_certificate_revocation_list(
            &self,
            request: tonic::Request<super::UpdateCertificateRevocationListRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Update a [Certificate][google.cloud.security.privateca.v1beta1.Certificate]."]
        async fn update_certificate(
            &self,
            request: tonic::Request<super::UpdateCertificateRequest>,
        ) -> Result<tonic::Response<super::Certificate>, tonic::Status>;
        #[doc = " Update a [ReusableConfig][google.cloud.security.privateca.v1beta1.ReusableConfig]."]
        async fn update_reusable_config(
            &self,
            request: tonic::Request<super::UpdateReusableConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Get the CSR for a pending [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        async fn get_certificate_authority_csr(
            &self,
            request: tonic::Request<super::GetCertificateAuthorityCsrRequest>,
        ) -> Result<tonic::Response<super::GetCertificateAuthorityCsrResponse>, tonic::Status>;
        #[doc = " Activate a pending [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        async fn activate_certificate_authority(
            &self,
            request: tonic::Request<super::ActivateCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Disable a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        async fn disable_certificate_authority(
            &self,
            request: tonic::Request<super::DisableCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Enable a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority]."]
        async fn enable_certificate_authority(
            &self,
            request: tonic::Request<super::EnableCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Schedule a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] for deletion."]
        async fn schedule_delete_certificate_authority(
            &self,
            request: tonic::Request<super::ScheduleDeleteCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Restore a [CertificateAuthority][google.cloud.security.privateca.v1beta1.CertificateAuthority] that is scheduled for deletion."]
        async fn restore_certificate_authority(
            &self,
            request: tonic::Request<super::RestoreCertificateAuthorityRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Revoke a [Certificate][google.cloud.security.privateca.v1beta1.Certificate]."]
        async fn revoke_certificate(
            &self,
            request: tonic::Request<super::RevokeCertificateRequest>,
        ) -> Result<tonic::Response<super::Certificate>, tonic::Status>;
    }
    #[doc = " [Certificate Authority Service][google.cloud.security.privateca.v1beta1.CertificateAuthorityService] manages private"]
    #[doc = " certificate authorities and issued certificates."]
    #[derive(Debug)]
    pub struct CertificateAuthorityServiceServer<T: CertificateAuthorityService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CertificateAuthorityService> CertificateAuthorityServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for CertificateAuthorityServiceServer<T>
    where
        T: CertificateAuthorityService,
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
            match req . uri ( ) . path ( ) { "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ListCertificateAuthorities" => { # [ allow ( non_camel_case_types ) ] struct ListCertificateAuthoritiesSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: ListCertificateAuthoritiesRequest > for ListCertificateAuthoritiesSvc < T > { type Response = super :: ListCertificateAuthoritiesResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListCertificateAuthoritiesRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . list_certificate_authorities ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListCertificateAuthoritiesSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ListCertificateRevocationLists" => { # [ allow ( non_camel_case_types ) ] struct ListCertificateRevocationListsSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: ListCertificateRevocationListsRequest > for ListCertificateRevocationListsSvc < T > { type Response = super :: ListCertificateRevocationListsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListCertificateRevocationListsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . list_certificate_revocation_lists ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListCertificateRevocationListsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ListCertificates" => { # [ allow ( non_camel_case_types ) ] struct ListCertificatesSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: ListCertificatesRequest > for ListCertificatesSvc < T > { type Response = super :: ListCertificatesResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListCertificatesRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . list_certificates ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListCertificatesSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ListReusableConfigs" => { # [ allow ( non_camel_case_types ) ] struct ListReusableConfigsSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: ListReusableConfigsRequest > for ListReusableConfigsSvc < T > { type Response = super :: ListReusableConfigsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListReusableConfigsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . list_reusable_configs ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListReusableConfigsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/GetCertificateAuthority" => { # [ allow ( non_camel_case_types ) ] struct GetCertificateAuthoritySvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: GetCertificateAuthorityRequest > for GetCertificateAuthoritySvc < T > { type Response = super :: CertificateAuthority ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetCertificateAuthorityRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . get_certificate_authority ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetCertificateAuthoritySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/GetCertificateRevocationList" => { # [ allow ( non_camel_case_types ) ] struct GetCertificateRevocationListSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: GetCertificateRevocationListRequest > for GetCertificateRevocationListSvc < T > { type Response = super :: CertificateRevocationList ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetCertificateRevocationListRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . get_certificate_revocation_list ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetCertificateRevocationListSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/GetCertificate" => { # [ allow ( non_camel_case_types ) ] struct GetCertificateSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: GetCertificateRequest > for GetCertificateSvc < T > { type Response = super :: Certificate ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetCertificateRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . get_certificate ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetCertificateSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/GetReusableConfig" => { # [ allow ( non_camel_case_types ) ] struct GetReusableConfigSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: GetReusableConfigRequest > for GetReusableConfigSvc < T > { type Response = super :: ReusableConfig ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetReusableConfigRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . get_reusable_config ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetReusableConfigSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/CreateCertificateAuthority" => { # [ allow ( non_camel_case_types ) ] struct CreateCertificateAuthoritySvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: CreateCertificateAuthorityRequest > for CreateCertificateAuthoritySvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateCertificateAuthorityRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . create_certificate_authority ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateCertificateAuthoritySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/CreateCertificateRevocationList" => { # [ allow ( non_camel_case_types ) ] struct CreateCertificateRevocationListSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: CreateCertificateRevocationListRequest > for CreateCertificateRevocationListSvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateCertificateRevocationListRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . create_certificate_revocation_list ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateCertificateRevocationListSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/CreateCertificate" => { # [ allow ( non_camel_case_types ) ] struct CreateCertificateSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: CreateCertificateRequest > for CreateCertificateSvc < T > { type Response = super :: Certificate ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateCertificateRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . create_certificate ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateCertificateSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/CreateReusableConfig" => { # [ allow ( non_camel_case_types ) ] struct CreateReusableConfigSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: CreateReusableConfigRequest > for CreateReusableConfigSvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateReusableConfigRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . create_reusable_config ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateReusableConfigSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/UpdateCertificateAuthority" => { # [ allow ( non_camel_case_types ) ] struct UpdateCertificateAuthoritySvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: UpdateCertificateAuthorityRequest > for UpdateCertificateAuthoritySvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateCertificateAuthorityRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . update_certificate_authority ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateCertificateAuthoritySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/UpdateCertificateRevocationList" => { # [ allow ( non_camel_case_types ) ] struct UpdateCertificateRevocationListSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: UpdateCertificateRevocationListRequest > for UpdateCertificateRevocationListSvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateCertificateRevocationListRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . update_certificate_revocation_list ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateCertificateRevocationListSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/UpdateCertificate" => { # [ allow ( non_camel_case_types ) ] struct UpdateCertificateSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: UpdateCertificateRequest > for UpdateCertificateSvc < T > { type Response = super :: Certificate ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateCertificateRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . update_certificate ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateCertificateSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/UpdateReusableConfig" => { # [ allow ( non_camel_case_types ) ] struct UpdateReusableConfigSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: UpdateReusableConfigRequest > for UpdateReusableConfigSvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateReusableConfigRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . update_reusable_config ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateReusableConfigSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/GetCertificateAuthorityCsr" => { # [ allow ( non_camel_case_types ) ] struct GetCertificateAuthorityCsrSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: GetCertificateAuthorityCsrRequest > for GetCertificateAuthorityCsrSvc < T > { type Response = super :: GetCertificateAuthorityCsrResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetCertificateAuthorityCsrRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . get_certificate_authority_csr ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetCertificateAuthorityCsrSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ActivateCertificateAuthority" => { # [ allow ( non_camel_case_types ) ] struct ActivateCertificateAuthoritySvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: ActivateCertificateAuthorityRequest > for ActivateCertificateAuthoritySvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ActivateCertificateAuthorityRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . activate_certificate_authority ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ActivateCertificateAuthoritySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/DisableCertificateAuthority" => { # [ allow ( non_camel_case_types ) ] struct DisableCertificateAuthoritySvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: DisableCertificateAuthorityRequest > for DisableCertificateAuthoritySvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DisableCertificateAuthorityRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . disable_certificate_authority ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DisableCertificateAuthoritySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/EnableCertificateAuthority" => { # [ allow ( non_camel_case_types ) ] struct EnableCertificateAuthoritySvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: EnableCertificateAuthorityRequest > for EnableCertificateAuthoritySvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: EnableCertificateAuthorityRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . enable_certificate_authority ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = EnableCertificateAuthoritySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/ScheduleDeleteCertificateAuthority" => { # [ allow ( non_camel_case_types ) ] struct ScheduleDeleteCertificateAuthoritySvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: ScheduleDeleteCertificateAuthorityRequest > for ScheduleDeleteCertificateAuthoritySvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ScheduleDeleteCertificateAuthorityRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . schedule_delete_certificate_authority ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ScheduleDeleteCertificateAuthoritySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/RestoreCertificateAuthority" => { # [ allow ( non_camel_case_types ) ] struct RestoreCertificateAuthoritySvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: RestoreCertificateAuthorityRequest > for RestoreCertificateAuthoritySvc < T > { type Response = super :: super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: RestoreCertificateAuthorityRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . restore_certificate_authority ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = RestoreCertificateAuthoritySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.security.privateca.v1beta1.CertificateAuthorityService/RevokeCertificate" => { # [ allow ( non_camel_case_types ) ] struct RevokeCertificateSvc < T : CertificateAuthorityService > ( pub Arc < T > ) ; impl < T : CertificateAuthorityService > tonic :: server :: UnaryService < super :: RevokeCertificateRequest > for RevokeCertificateSvc < T > { type Response = super :: Certificate ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: RevokeCertificateRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { ( * inner ) . revoke_certificate ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = RevokeCertificateSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: CertificateAuthorityService> Clone for CertificateAuthorityServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CertificateAuthorityService> Clone for _Inner<T> {
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
