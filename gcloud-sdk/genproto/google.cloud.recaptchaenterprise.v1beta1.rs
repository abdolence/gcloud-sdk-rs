/// The create assessment request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAssessmentRequest {
    /// Required. The name of the project in which the assessment will be created,
    /// in the format "projects/{project_number}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The assessment details.
    #[prost(message, optional, tag = "2")]
    pub assessment: ::core::option::Option<Assessment>,
}
/// The request message to annotate an Assessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateAssessmentRequest {
    /// Required. The resource name of the Assessment, in the format
    /// "projects/{project_number}/assessments/{assessment_id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The annotation that will be assigned to the Event. This field can be left
    /// empty to provide reasons that apply to an event without concluding whether
    /// the event is legitimate or fraudulent.
    #[prost(enumeration = "annotate_assessment_request::Annotation", tag = "2")]
    pub annotation: i32,
    /// Optional. Optional reasons for the annotation that will be assigned to the Event.
    #[prost(
        enumeration = "annotate_assessment_request::Reason",
        repeated,
        packed = "false",
        tag = "3"
    )]
    pub reasons: ::prost::alloc::vec::Vec<i32>,
    /// Optional. Optional unique stable hashed user identifier to apply to the assessment.
    /// This is an alternative to setting the hashed_account_id in
    /// CreateAssessment, for example when the account identifier is not yet known
    /// in the initial request. It is recommended that the identifier is hashed
    /// using hmac-sha256 with stable secret.
    #[prost(bytes = "vec", tag = "4")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `AnnotateAssessmentRequest`.
pub mod annotate_assessment_request {
    /// Enum that represents the types of annotations.
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
    pub enum Annotation {
        /// Default unspecified type.
        Unspecified = 0,
        /// Provides information that the event turned out to be legitimate.
        Legitimate = 1,
        /// Provides information that the event turned out to be fraudulent.
        Fraudulent = 2,
        /// Provides information that the event was related to a login event in which
        /// the user typed the correct password. Deprecated, prefer indicating
        /// CORRECT_PASSWORD through the reasons field instead.
        PasswordCorrect = 3,
        /// Provides information that the event was related to a login event in which
        /// the user typed the incorrect password. Deprecated, prefer indicating
        /// INCORRECT_PASSWORD through the reasons field instead.
        PasswordIncorrect = 4,
    }
    impl Annotation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Annotation::Unspecified => "ANNOTATION_UNSPECIFIED",
                Annotation::Legitimate => "LEGITIMATE",
                Annotation::Fraudulent => "FRAUDULENT",
                Annotation::PasswordCorrect => "PASSWORD_CORRECT",
                Annotation::PasswordIncorrect => "PASSWORD_INCORRECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ANNOTATION_UNSPECIFIED" => Some(Self::Unspecified),
                "LEGITIMATE" => Some(Self::Legitimate),
                "FRAUDULENT" => Some(Self::Fraudulent),
                "PASSWORD_CORRECT" => Some(Self::PasswordCorrect),
                "PASSWORD_INCORRECT" => Some(Self::PasswordIncorrect),
                _ => None,
            }
        }
    }
    /// Enum that represents potential reasons for annotating an assessment.
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
    pub enum Reason {
        /// Default unspecified reason.
        Unspecified = 0,
        /// Indicates a chargeback issued for the transaction with no other details.
        /// When possible, specify the type by using CHARGEBACK_FRAUD or
        /// CHARGEBACK_DISPUTE instead.
        Chargeback = 1,
        /// Indicates a chargeback related to an alleged unauthorized transaction
        /// from the cardholder's perspective (for example, the card number was
        /// stolen).
        ChargebackFraud = 8,
        /// Indicates a chargeback related to the cardholder having provided their
        /// card details but allegedly not being satisfied with the purchase
        /// (for example, misrepresentation, attempted cancellation).
        ChargebackDispute = 9,
        /// Indicates the transaction associated with the assessment is suspected of
        /// being fraudulent based on the payment method, billing details, shipping
        /// address or other transaction information.
        PaymentHeuristics = 2,
        /// Indicates that the user was served a 2FA challenge. An old assessment
        /// with `ENUM_VALUES.INITIATED_TWO_FACTOR` reason that has not been
        /// overwritten with `PASSED_TWO_FACTOR` is treated as an abandoned 2FA flow.
        /// This is equivalent to `FAILED_TWO_FACTOR`.
        InitiatedTwoFactor = 7,
        /// Indicates that the user passed a 2FA challenge.
        PassedTwoFactor = 3,
        /// Indicates that the user failed a 2FA challenge.
        FailedTwoFactor = 4,
        /// Indicates the user provided the correct password.
        CorrectPassword = 5,
        /// Indicates the user provided an incorrect password.
        IncorrectPassword = 6,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::Unspecified => "REASON_UNSPECIFIED",
                Reason::Chargeback => "CHARGEBACK",
                Reason::ChargebackFraud => "CHARGEBACK_FRAUD",
                Reason::ChargebackDispute => "CHARGEBACK_DISPUTE",
                Reason::PaymentHeuristics => "PAYMENT_HEURISTICS",
                Reason::InitiatedTwoFactor => "INITIATED_TWO_FACTOR",
                Reason::PassedTwoFactor => "PASSED_TWO_FACTOR",
                Reason::FailedTwoFactor => "FAILED_TWO_FACTOR",
                Reason::CorrectPassword => "CORRECT_PASSWORD",
                Reason::IncorrectPassword => "INCORRECT_PASSWORD",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "CHARGEBACK" => Some(Self::Chargeback),
                "CHARGEBACK_FRAUD" => Some(Self::ChargebackFraud),
                "CHARGEBACK_DISPUTE" => Some(Self::ChargebackDispute),
                "PAYMENT_HEURISTICS" => Some(Self::PaymentHeuristics),
                "INITIATED_TWO_FACTOR" => Some(Self::InitiatedTwoFactor),
                "PASSED_TWO_FACTOR" => Some(Self::PassedTwoFactor),
                "FAILED_TWO_FACTOR" => Some(Self::FailedTwoFactor),
                "CORRECT_PASSWORD" => Some(Self::CorrectPassword),
                "INCORRECT_PASSWORD" => Some(Self::IncorrectPassword),
                _ => None,
            }
        }
    }
}
/// Empty response for AnnotateAssessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateAssessmentResponse {}
/// Password leak verification info.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordLeakVerification {
    /// Optional. Scrypt hash of the username+password that the customer wants to verify
    /// against a known password leak.
    #[prost(bytes = "vec", tag = "1")]
    pub hashed_user_credentials: ::prost::alloc::vec::Vec<u8>,
    /// Output only. Whether or not the user's credentials are present in a known leak.
    #[prost(bool, tag = "2")]
    pub credentials_leaked: bool,
    /// Optional. The username part of the user credentials for which we want to trigger a
    /// leak check in canonicalized form. This is the same data used to create the
    /// hashed_user_credentials on the customer side.
    #[prost(string, tag = "3")]
    pub canonicalized_username: ::prost::alloc::string::String,
}
/// A recaptcha assessment resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assessment {
    /// Output only. The resource name for the Assessment in the format
    /// "projects/{project_number}/assessments/{assessment_id}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The event being assessed.
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<Event>,
    /// Output only. Legitimate event score from 0.0 to 1.0.
    /// (1.0 means very likely legitimate traffic while 0.0 means very likely
    /// non-legitimate traffic).
    #[prost(float, tag = "3")]
    pub score: f32,
    /// Output only. Properties of the provided event token.
    #[prost(message, optional, tag = "4")]
    pub token_properties: ::core::option::Option<TokenProperties>,
    /// Output only. Reasons contributing to the risk analysis verdict.
    #[prost(
        enumeration = "assessment::ClassificationReason",
        repeated,
        packed = "false",
        tag = "5"
    )]
    pub reasons: ::prost::alloc::vec::Vec<i32>,
    /// Information about the user's credentials used to check for leaks.
    /// This feature is part of the Early Access Program (EAP). Exercise caution,
    /// and do not deploy integrations based on this feature in a production
    /// environment.
    #[prost(message, optional, tag = "7")]
    pub password_leak_verification: ::core::option::Option<PasswordLeakVerification>,
    /// Assessment returned by Account Defender when a hashed_account_id is
    /// provided.
    #[prost(message, optional, tag = "8")]
    pub account_defender_assessment: ::core::option::Option<AccountDefenderAssessment>,
}
/// Nested message and enum types in `Assessment`.
pub mod assessment {
    /// Reasons contributing to the risk analysis verdict.
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
    pub enum ClassificationReason {
        /// Default unspecified type.
        Unspecified = 0,
        /// Interactions matched the behavior of an automated agent.
        Automation = 1,
        /// The event originated from an illegitimate environment.
        UnexpectedEnvironment = 2,
        /// Traffic volume from the event source is higher than normal.
        TooMuchTraffic = 3,
        /// Interactions with the site were significantly different than expected
        /// patterns.
        UnexpectedUsagePatterns = 4,
        /// Too little traffic has been received from this site thus far to generate
        /// quality risk analysis.
        LowConfidenceScore = 5,
    }
    impl ClassificationReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClassificationReason::Unspecified => "CLASSIFICATION_REASON_UNSPECIFIED",
                ClassificationReason::Automation => "AUTOMATION",
                ClassificationReason::UnexpectedEnvironment => "UNEXPECTED_ENVIRONMENT",
                ClassificationReason::TooMuchTraffic => "TOO_MUCH_TRAFFIC",
                ClassificationReason::UnexpectedUsagePatterns => {
                    "UNEXPECTED_USAGE_PATTERNS"
                }
                ClassificationReason::LowConfidenceScore => "LOW_CONFIDENCE_SCORE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLASSIFICATION_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTOMATION" => Some(Self::Automation),
                "UNEXPECTED_ENVIRONMENT" => Some(Self::UnexpectedEnvironment),
                "TOO_MUCH_TRAFFIC" => Some(Self::TooMuchTraffic),
                "UNEXPECTED_USAGE_PATTERNS" => Some(Self::UnexpectedUsagePatterns),
                "LOW_CONFIDENCE_SCORE" => Some(Self::LowConfidenceScore),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Optional. The user response token provided by the reCAPTCHA client-side integration
    /// on your site.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Optional. The site key that was used to invoke reCAPTCHA on your site and generate
    /// the token.
    #[prost(string, tag = "2")]
    pub site_key: ::prost::alloc::string::String,
    /// Optional. The user agent present in the request from the user's device related to
    /// this event.
    #[prost(string, tag = "3")]
    pub user_agent: ::prost::alloc::string::String,
    /// Optional. The IP address in the request from the user's device related to this event.
    #[prost(string, tag = "4")]
    pub user_ip_address: ::prost::alloc::string::String,
    /// Optional. The expected action for this type of event. This should be the same action
    /// provided at token generation time on client-side platforms already
    /// integrated with recaptcha enterprise.
    #[prost(string, tag = "5")]
    pub expected_action: ::prost::alloc::string::String,
    /// Optional. Optional unique stable hashed user identifier for the request. The
    /// identifier should ideally be hashed using sha256 with stable secret.
    #[prost(bytes = "vec", tag = "6")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenProperties {
    /// Whether the provided user response token is valid. When valid = false, the
    /// reason could be specified in invalid_reason or it could also be due to
    /// a user failing to solve a challenge or a sitekey mismatch (i.e the sitekey
    /// used to generate the token was different than the one specified in the
    /// assessment).
    #[prost(bool, tag = "1")]
    pub valid: bool,
    /// Reason associated with the response when valid = false.
    #[prost(enumeration = "token_properties::InvalidReason", tag = "2")]
    pub invalid_reason: i32,
    /// The timestamp corresponding to the generation of the token.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The hostname of the page on which the token was generated.
    #[prost(string, tag = "4")]
    pub hostname: ::prost::alloc::string::String,
    /// Action name provided at token generation.
    #[prost(string, tag = "5")]
    pub action: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TokenProperties`.
pub mod token_properties {
    /// Enum that represents the types of invalid token reasons.
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
    pub enum InvalidReason {
        /// Default unspecified type.
        Unspecified = 0,
        /// If the failure reason was not accounted for.
        UnknownInvalidReason = 1,
        /// The provided user verification token was malformed.
        Malformed = 2,
        /// The user verification token had expired.
        Expired = 3,
        /// The user verification had already been seen.
        Dupe = 4,
        /// The user verification token did not match the provided site key.
        /// This may be a configuration error (e.g. development keys used in
        /// production) or end users trying to use verification tokens from other
        /// sites.
        SiteMismatch = 5,
        /// The user verification token was not present.  It is a required input.
        Missing = 6,
        /// A retriable error (such as network failure) occurred on the browser.
        /// Could easily be simulated by an attacker.
        BrowserError = 7,
    }
    impl InvalidReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InvalidReason::Unspecified => "INVALID_REASON_UNSPECIFIED",
                InvalidReason::UnknownInvalidReason => "UNKNOWN_INVALID_REASON",
                InvalidReason::Malformed => "MALFORMED",
                InvalidReason::Expired => "EXPIRED",
                InvalidReason::Dupe => "DUPE",
                InvalidReason::SiteMismatch => "SITE_MISMATCH",
                InvalidReason::Missing => "MISSING",
                InvalidReason::BrowserError => "BROWSER_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INVALID_REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN_INVALID_REASON" => Some(Self::UnknownInvalidReason),
                "MALFORMED" => Some(Self::Malformed),
                "EXPIRED" => Some(Self::Expired),
                "DUPE" => Some(Self::Dupe),
                "SITE_MISMATCH" => Some(Self::SiteMismatch),
                "MISSING" => Some(Self::Missing),
                "BROWSER_ERROR" => Some(Self::BrowserError),
                _ => None,
            }
        }
    }
}
/// Account Defender risk assessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountDefenderAssessment {
    /// Labels for this request.
    #[prost(
        enumeration = "account_defender_assessment::AccountDefenderLabel",
        repeated,
        tag = "1"
    )]
    pub labels: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `AccountDefenderAssessment`.
pub mod account_defender_assessment {
    /// Labels returned by Account Defender for this request.
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
    pub enum AccountDefenderLabel {
        /// Default unspecified type.
        Unspecified = 0,
        /// The request matches a known good profile for the user.
        ProfileMatch = 1,
        /// The request is potentially a suspicious login event and should be further
        /// verified either via multi-factor authentication or another system.
        SuspiciousLoginActivity = 2,
        /// The request matched a profile that previously had suspicious account
        /// creation behavior. This could mean this is a fake account.
        SuspiciousAccountCreation = 3,
        /// The account in the request has a high number of related accounts. It does
        /// not necessarily imply that the account is bad but could require
        /// investigating.
        RelatedAccountsNumberHigh = 4,
    }
    impl AccountDefenderLabel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccountDefenderLabel::Unspecified => "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED",
                AccountDefenderLabel::ProfileMatch => "PROFILE_MATCH",
                AccountDefenderLabel::SuspiciousLoginActivity => {
                    "SUSPICIOUS_LOGIN_ACTIVITY"
                }
                AccountDefenderLabel::SuspiciousAccountCreation => {
                    "SUSPICIOUS_ACCOUNT_CREATION"
                }
                AccountDefenderLabel::RelatedAccountsNumberHigh => {
                    "RELATED_ACCOUNTS_NUMBER_HIGH"
                }
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACCOUNT_DEFENDER_LABEL_UNSPECIFIED" => Some(Self::Unspecified),
                "PROFILE_MATCH" => Some(Self::ProfileMatch),
                "SUSPICIOUS_LOGIN_ACTIVITY" => Some(Self::SuspiciousLoginActivity),
                "SUSPICIOUS_ACCOUNT_CREATION" => Some(Self::SuspiciousAccountCreation),
                "RELATED_ACCOUNTS_NUMBER_HIGH" => Some(Self::RelatedAccountsNumberHigh),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod recaptcha_enterprise_service_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to determine the likelihood an event is legitimate.
    #[derive(Debug, Clone)]
    pub struct RecaptchaEnterpriseServiceV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RecaptchaEnterpriseServiceV1Beta1Client<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RecaptchaEnterpriseServiceV1Beta1Client<T>
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
        ) -> RecaptchaEnterpriseServiceV1Beta1Client<InterceptedService<T, F>>
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
            RecaptchaEnterpriseServiceV1Beta1Client::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Creates an Assessment of the likelihood an event is legitimate.
        pub async fn create_assessment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAssessmentRequest>,
        ) -> std::result::Result<tonic::Response<super::Assessment>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/CreateAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
                        "CreateAssessment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Annotates a previously created Assessment to provide additional information
        /// on whether the event turned out to be authentic or fradulent.
        pub async fn annotate_assessment(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnotateAssessmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AnnotateAssessmentResponse>,
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
                "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/AnnotateAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1",
                        "AnnotateAssessment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
