/// The create assessment request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAssessmentRequest {
    /// Required. The name of the project in which the assessment will be created,
    /// in the format "projects/{project}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The assessment details.
    #[prost(message, optional, tag = "2")]
    pub assessment: ::core::option::Option<Assessment>,
}
/// Describes an event in the lifecycle of a payment transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionEvent {
    /// Optional. The type of this transaction event.
    #[prost(enumeration = "transaction_event::TransactionEventType", tag = "1")]
    pub event_type: i32,
    /// Optional. The reason or standardized code that corresponds with this
    /// transaction event, if one exists. For example, a CHARGEBACK event with code
    /// 6005.
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    /// Optional. The value that corresponds with this transaction event, if one
    /// exists. For example, a refund event where $5.00 was refunded. Currency is
    /// obtained from the original transaction data.
    #[prost(double, tag = "3")]
    pub value: f64,
    /// Optional. Timestamp when this transaction event occurred; otherwise assumed
    /// to be the time of the API call.
    #[prost(message, optional, tag = "4")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `TransactionEvent`.
pub mod transaction_event {
    /// Enum that represents an event in the payment transaction lifecycle.
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
    pub enum TransactionEventType {
        /// Default, unspecified event type.
        Unspecified = 0,
        /// Indicates that the transaction is approved by the merchant. The
        /// accompanying reasons can include terms such as 'INHOUSE', 'ACCERTIFY',
        /// 'CYBERSOURCE', or 'MANUAL_REVIEW'.
        MerchantApprove = 1,
        /// Indicates that the transaction is denied and concluded due to risks
        /// detected by the merchant. The accompanying reasons can include terms such
        /// as 'INHOUSE',  'ACCERTIFY',  'CYBERSOURCE', or 'MANUAL_REVIEW'.
        MerchantDeny = 2,
        /// Indicates that the transaction is being evaluated by a human, due to
        /// suspicion or risk.
        ManualReview = 3,
        /// Indicates that the authorization attempt with the card issuer succeeded.
        Authorization = 4,
        /// Indicates that the authorization attempt with the card issuer failed.
        /// The accompanying reasons can include Visa's '54' indicating that the card
        /// is expired, or '82' indicating that the CVV is incorrect.
        AuthorizationDecline = 5,
        /// Indicates that the transaction is completed because the funds were
        /// settled.
        PaymentCapture = 6,
        /// Indicates that the transaction could not be completed because the funds
        /// were not settled.
        PaymentCaptureDecline = 7,
        /// Indicates that the transaction has been canceled. Specify the reason
        /// for the cancellation. For example, 'INSUFFICIENT_INVENTORY'.
        Cancel = 8,
        /// Indicates that the merchant has received a chargeback inquiry due to
        /// fraud for the transaction, requesting additional information before a
        /// fraud chargeback is officially issued and a formal chargeback
        /// notification is sent.
        ChargebackInquiry = 9,
        /// Indicates that the merchant has received a chargeback alert due to fraud
        /// for the transaction. The process of resolving the dispute without
        /// involving the payment network is started.
        ChargebackAlert = 10,
        /// Indicates that a fraud notification is issued for the transaction, sent
        /// by the payment instrument's issuing bank because the transaction appears
        /// to be fraudulent. We recommend including TC40 or SAFE data in the
        /// `reason` field for this event type. For partial chargebacks, we recommend
        /// that you include an amount in the `value` field.
        FraudNotification = 11,
        /// Indicates that the merchant is informed by the payment network that the
        /// transaction has entered the chargeback process due to fraud. Reason code
        /// examples include Discover's '6005' and '6041'. For partial chargebacks,
        /// we recommend that you include an amount in the `value` field.
        Chargeback = 12,
        /// Indicates that the transaction has entered the chargeback process due to
        /// fraud, and that the merchant has chosen to enter representment. Reason
        /// examples include Discover's '6005' and '6041'. For partial chargebacks,
        /// we recommend that you include an amount in the `value` field.
        ChargebackRepresentment = 13,
        /// Indicates that the transaction has had a fraud chargeback which was
        /// illegitimate and was reversed as a result. For partial chargebacks, we
        /// recommend that you include an amount in the `value` field.
        ChargebackReverse = 14,
        /// Indicates that the merchant has received a refund for a completed
        /// transaction. For partial refunds, we recommend that you include an amount
        /// in the `value` field. Reason example: 'TAX_EXEMPT' (partial refund of
        /// exempt tax)
        RefundRequest = 15,
        /// Indicates that the merchant has received a refund request for this
        /// transaction, but that they have declined it. For partial refunds, we
        /// recommend that you include an amount in the `value` field. Reason
        /// example: 'TAX_EXEMPT' (partial refund of exempt tax)
        RefundDecline = 16,
        /// Indicates that the completed transaction was refunded by the merchant.
        /// For partial refunds, we recommend that you include an amount in the
        /// `value` field. Reason example: 'TAX_EXEMPT' (partial refund of exempt
        /// tax)
        Refund = 17,
        /// Indicates that the completed transaction was refunded by the merchant,
        /// and that this refund was reversed. For partial refunds, we recommend that
        /// you include an amount in the `value` field.
        RefundReverse = 18,
    }
    impl TransactionEventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TransactionEventType::Unspecified => "TRANSACTION_EVENT_TYPE_UNSPECIFIED",
                TransactionEventType::MerchantApprove => "MERCHANT_APPROVE",
                TransactionEventType::MerchantDeny => "MERCHANT_DENY",
                TransactionEventType::ManualReview => "MANUAL_REVIEW",
                TransactionEventType::Authorization => "AUTHORIZATION",
                TransactionEventType::AuthorizationDecline => "AUTHORIZATION_DECLINE",
                TransactionEventType::PaymentCapture => "PAYMENT_CAPTURE",
                TransactionEventType::PaymentCaptureDecline => "PAYMENT_CAPTURE_DECLINE",
                TransactionEventType::Cancel => "CANCEL",
                TransactionEventType::ChargebackInquiry => "CHARGEBACK_INQUIRY",
                TransactionEventType::ChargebackAlert => "CHARGEBACK_ALERT",
                TransactionEventType::FraudNotification => "FRAUD_NOTIFICATION",
                TransactionEventType::Chargeback => "CHARGEBACK",
                TransactionEventType::ChargebackRepresentment => {
                    "CHARGEBACK_REPRESENTMENT"
                }
                TransactionEventType::ChargebackReverse => "CHARGEBACK_REVERSE",
                TransactionEventType::RefundRequest => "REFUND_REQUEST",
                TransactionEventType::RefundDecline => "REFUND_DECLINE",
                TransactionEventType::Refund => "REFUND",
                TransactionEventType::RefundReverse => "REFUND_REVERSE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TRANSACTION_EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "MERCHANT_APPROVE" => Some(Self::MerchantApprove),
                "MERCHANT_DENY" => Some(Self::MerchantDeny),
                "MANUAL_REVIEW" => Some(Self::ManualReview),
                "AUTHORIZATION" => Some(Self::Authorization),
                "AUTHORIZATION_DECLINE" => Some(Self::AuthorizationDecline),
                "PAYMENT_CAPTURE" => Some(Self::PaymentCapture),
                "PAYMENT_CAPTURE_DECLINE" => Some(Self::PaymentCaptureDecline),
                "CANCEL" => Some(Self::Cancel),
                "CHARGEBACK_INQUIRY" => Some(Self::ChargebackInquiry),
                "CHARGEBACK_ALERT" => Some(Self::ChargebackAlert),
                "FRAUD_NOTIFICATION" => Some(Self::FraudNotification),
                "CHARGEBACK" => Some(Self::Chargeback),
                "CHARGEBACK_REPRESENTMENT" => Some(Self::ChargebackRepresentment),
                "CHARGEBACK_REVERSE" => Some(Self::ChargebackReverse),
                "REFUND_REQUEST" => Some(Self::RefundRequest),
                "REFUND_DECLINE" => Some(Self::RefundDecline),
                "REFUND" => Some(Self::Refund),
                "REFUND_REVERSE" => Some(Self::RefundReverse),
                _ => None,
            }
        }
    }
}
/// The request message to annotate an Assessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateAssessmentRequest {
    /// Required. The resource name of the Assessment, in the format
    /// "projects/{project}/assessments/{assessment}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The annotation that will be assigned to the Event. This field can
    /// be left empty to provide reasons that apply to an event without concluding
    /// whether the event is legitimate or fraudulent.
    #[prost(enumeration = "annotate_assessment_request::Annotation", tag = "2")]
    pub annotation: i32,
    /// Optional. Optional reasons for the annotation that will be assigned to the
    /// Event.
    #[prost(
        enumeration = "annotate_assessment_request::Reason",
        repeated,
        packed = "false",
        tag = "3"
    )]
    pub reasons: ::prost::alloc::vec::Vec<i32>,
    /// Optional. Unique stable hashed user identifier to apply to the assessment.
    /// This is an alternative to setting the hashed_account_id in
    /// CreateAssessment, for example when the account identifier is not yet known
    /// in the initial request. It is recommended that the identifier is hashed
    /// using hmac-sha256 with stable secret.
    #[prost(bytes = "vec", tag = "4")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
    /// Optional. If the assessment is part of a payment transaction, provide
    /// details on payment lifecycle events that occur in the transaction.
    #[prost(message, optional, tag = "5")]
    pub transaction_event: ::core::option::Option<TransactionEvent>,
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
        /// Indicates that the transaction had a chargeback issued with no other
        /// details. When possible, specify the type by using CHARGEBACK_FRAUD or
        /// CHARGEBACK_DISPUTE instead.
        Chargeback = 1,
        /// Indicates that the transaction had a chargeback issued related to an
        /// alleged unauthorized transaction from the cardholder's perspective (for
        /// example, the card number was stolen).
        ChargebackFraud = 8,
        /// Indicates that the transaction had a chargeback issued related to the
        /// cardholder having provided their card details but allegedly not being
        /// satisfied with the purchase (for example, misrepresentation, attempted
        /// cancellation).
        ChargebackDispute = 9,
        /// Indicates that the completed payment transaction was refunded by the
        /// seller.
        Refund = 10,
        /// Indicates that the completed payment transaction was determined to be
        /// fraudulent by the seller, and was cancelled and refunded as a result.
        RefundFraud = 11,
        /// Indicates that the payment transaction was accepted, and the user was
        /// charged.
        TransactionAccepted = 12,
        /// Indicates that the payment transaction was declined, for example due to
        /// invalid card details.
        TransactionDeclined = 13,
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
        /// Indicates that the user sent unwanted and abusive messages to other users
        /// of the platform, such as spam, scams, phishing, or social engineering.
        SocialSpam = 14,
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
                Reason::Refund => "REFUND",
                Reason::RefundFraud => "REFUND_FRAUD",
                Reason::TransactionAccepted => "TRANSACTION_ACCEPTED",
                Reason::TransactionDeclined => "TRANSACTION_DECLINED",
                Reason::PaymentHeuristics => "PAYMENT_HEURISTICS",
                Reason::InitiatedTwoFactor => "INITIATED_TWO_FACTOR",
                Reason::PassedTwoFactor => "PASSED_TWO_FACTOR",
                Reason::FailedTwoFactor => "FAILED_TWO_FACTOR",
                Reason::CorrectPassword => "CORRECT_PASSWORD",
                Reason::IncorrectPassword => "INCORRECT_PASSWORD",
                Reason::SocialSpam => "SOCIAL_SPAM",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNSPECIFIED" => Some(Self::Unspecified),
                "CHARGEBACK" => Some(Self::Chargeback),
                "CHARGEBACK_FRAUD" => Some(Self::ChargebackFraud),
                "CHARGEBACK_DISPUTE" => Some(Self::ChargebackDispute),
                "REFUND" => Some(Self::Refund),
                "REFUND_FRAUD" => Some(Self::RefundFraud),
                "TRANSACTION_ACCEPTED" => Some(Self::TransactionAccepted),
                "TRANSACTION_DECLINED" => Some(Self::TransactionDeclined),
                "PAYMENT_HEURISTICS" => Some(Self::PaymentHeuristics),
                "INITIATED_TWO_FACTOR" => Some(Self::InitiatedTwoFactor),
                "PASSED_TWO_FACTOR" => Some(Self::PassedTwoFactor),
                "FAILED_TWO_FACTOR" => Some(Self::FailedTwoFactor),
                "CORRECT_PASSWORD" => Some(Self::CorrectPassword),
                "INCORRECT_PASSWORD" => Some(Self::IncorrectPassword),
                "SOCIAL_SPAM" => Some(Self::SocialSpam),
                _ => None,
            }
        }
    }
}
/// Empty response for AnnotateAssessment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateAssessmentResponse {}
/// Information about a verification endpoint that can be used for 2FA.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointVerificationInfo {
    /// Output only. Token to provide to the client to trigger endpoint
    /// verification. It must be used within 15 minutes.
    #[prost(string, tag = "3")]
    pub request_token: ::prost::alloc::string::String,
    /// Output only. Timestamp of the last successful verification for the
    /// endpoint, if any.
    #[prost(message, optional, tag = "4")]
    pub last_verification_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "endpoint_verification_info::Endpoint", tags = "1, 2")]
    pub endpoint: ::core::option::Option<endpoint_verification_info::Endpoint>,
}
/// Nested message and enum types in `EndpointVerificationInfo`.
pub mod endpoint_verification_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Endpoint {
        /// Email address for which to trigger a verification request.
        #[prost(string, tag = "1")]
        EmailAddress(::prost::alloc::string::String),
        /// Phone number for which to trigger a verification request. Should be given
        /// in E.164 format.
        #[prost(string, tag = "2")]
        PhoneNumber(::prost::alloc::string::String),
    }
}
/// Information about account verification, used for identity verification.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountVerificationInfo {
    /// Endpoints that can be used for identity verification.
    #[prost(message, repeated, tag = "1")]
    pub endpoints: ::prost::alloc::vec::Vec<EndpointVerificationInfo>,
    /// Language code preference for the verification message, set as a IETF BCP 47
    /// language code.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
    /// Output only. Result of the latest account verification challenge.
    #[prost(enumeration = "account_verification_info::Result", tag = "7")]
    pub latest_verification_result: i32,
    /// Username of the account that is being verified. Deprecated. Customers
    /// should now provide the hashed account ID field in Event.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AccountVerificationInfo`.
pub mod account_verification_info {
    /// Result of the account verification as contained in the verdict token issued
    /// at the end of the verification flow.
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
    pub enum Result {
        /// No information about the latest account verification.
        Unspecified = 0,
        /// The user was successfully verified. This means the account verification
        /// challenge was successfully completed.
        SuccessUserVerified = 1,
        /// The user failed the verification challenge.
        ErrorUserNotVerified = 2,
        /// The site is not properly onboarded to use the account verification
        /// feature.
        ErrorSiteOnboardingIncomplete = 3,
        /// The recipient is not allowed for account verification. This can occur
        /// during integration but should not occur in production.
        ErrorRecipientNotAllowed = 4,
        /// The recipient has already been sent too many verification codes in a
        /// short amount of time.
        ErrorRecipientAbuseLimitExhausted = 5,
        /// The verification flow could not be completed due to a critical internal
        /// error.
        ErrorCriticalInternal = 6,
        /// The client has exceeded their two factor request quota for this period of
        /// time.
        ErrorCustomerQuotaExhausted = 7,
        /// The request cannot be processed at the time because of an incident. This
        /// bypass can be restricted to a problematic destination email domain, a
        /// customer, or could affect the entire service.
        ErrorVerificationBypassed = 8,
        /// The request parameters do not match with the token provided and cannot be
        /// processed.
        ErrorVerdictMismatch = 9,
    }
    impl Result {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Result::Unspecified => "RESULT_UNSPECIFIED",
                Result::SuccessUserVerified => "SUCCESS_USER_VERIFIED",
                Result::ErrorUserNotVerified => "ERROR_USER_NOT_VERIFIED",
                Result::ErrorSiteOnboardingIncomplete => {
                    "ERROR_SITE_ONBOARDING_INCOMPLETE"
                }
                Result::ErrorRecipientNotAllowed => "ERROR_RECIPIENT_NOT_ALLOWED",
                Result::ErrorRecipientAbuseLimitExhausted => {
                    "ERROR_RECIPIENT_ABUSE_LIMIT_EXHAUSTED"
                }
                Result::ErrorCriticalInternal => "ERROR_CRITICAL_INTERNAL",
                Result::ErrorCustomerQuotaExhausted => "ERROR_CUSTOMER_QUOTA_EXHAUSTED",
                Result::ErrorVerificationBypassed => "ERROR_VERIFICATION_BYPASSED",
                Result::ErrorVerdictMismatch => "ERROR_VERDICT_MISMATCH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESULT_UNSPECIFIED" => Some(Self::Unspecified),
                "SUCCESS_USER_VERIFIED" => Some(Self::SuccessUserVerified),
                "ERROR_USER_NOT_VERIFIED" => Some(Self::ErrorUserNotVerified),
                "ERROR_SITE_ONBOARDING_INCOMPLETE" => {
                    Some(Self::ErrorSiteOnboardingIncomplete)
                }
                "ERROR_RECIPIENT_NOT_ALLOWED" => Some(Self::ErrorRecipientNotAllowed),
                "ERROR_RECIPIENT_ABUSE_LIMIT_EXHAUSTED" => {
                    Some(Self::ErrorRecipientAbuseLimitExhausted)
                }
                "ERROR_CRITICAL_INTERNAL" => Some(Self::ErrorCriticalInternal),
                "ERROR_CUSTOMER_QUOTA_EXHAUSTED" => {
                    Some(Self::ErrorCustomerQuotaExhausted)
                }
                "ERROR_VERIFICATION_BYPASSED" => Some(Self::ErrorVerificationBypassed),
                "ERROR_VERDICT_MISMATCH" => Some(Self::ErrorVerdictMismatch),
                _ => None,
            }
        }
    }
}
/// Private password leak verification info.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivatePasswordLeakVerification {
    /// Optional. Exactly 26-bit prefix of the SHA-256 hash of the canonicalized
    /// username. It is used to look up password leaks associated with that hash
    /// prefix.
    #[prost(bytes = "vec", tag = "1")]
    pub lookup_hash_prefix: ::prost::alloc::vec::Vec<u8>,
    /// Optional. Encrypted Scrypt hash of the canonicalized username+password. It
    /// is re-encrypted by the server and returned through
    /// `reencrypted_user_credentials_hash`.
    #[prost(bytes = "vec", tag = "2")]
    pub encrypted_user_credentials_hash: ::prost::alloc::vec::Vec<u8>,
    /// Output only. List of prefixes of the encrypted potential password leaks
    /// that matched the given parameters. They must be compared with the
    /// client-side decryption prefix of `reencrypted_user_credentials_hash`
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub encrypted_leak_match_prefixes: ::prost::alloc::vec::Vec<
        ::prost::alloc::vec::Vec<u8>,
    >,
    /// Output only. Corresponds to the re-encryption of the
    /// `encrypted_user_credentials_hash` field. It is used to match potential
    /// password leaks within `encrypted_leak_match_prefixes`.
    #[prost(bytes = "vec", tag = "4")]
    pub reencrypted_user_credentials_hash: ::prost::alloc::vec::Vec<u8>,
}
/// A reCAPTCHA Enterprise assessment resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assessment {
    /// Output only. The resource name for the Assessment in the format
    /// "projects/{project}/assessments/{assessment}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The event being assessed.
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<Event>,
    /// Output only. The risk analysis result for the event being assessed.
    #[prost(message, optional, tag = "3")]
    pub risk_analysis: ::core::option::Option<RiskAnalysis>,
    /// Output only. Properties of the provided event token.
    #[prost(message, optional, tag = "4")]
    pub token_properties: ::core::option::Option<TokenProperties>,
    /// Account verification information for identity verification. The assessment
    /// event must include a token and site key to use this feature.
    #[prost(message, optional, tag = "5")]
    pub account_verification: ::core::option::Option<AccountVerificationInfo>,
    /// Assessment returned by account defender when a hashed_account_id is
    /// provided.
    #[prost(message, optional, tag = "6")]
    pub account_defender_assessment: ::core::option::Option<AccountDefenderAssessment>,
    /// The private password leak verification field contains the parameters that
    /// are used to to check for leaks privately without sharing user credentials.
    #[prost(message, optional, tag = "8")]
    pub private_password_leak_verification: ::core::option::Option<
        PrivatePasswordLeakVerification,
    >,
    /// Assessment returned by Fraud Prevention when TransactionData is provided.
    #[prost(message, optional, tag = "11")]
    pub fraud_prevention_assessment: ::core::option::Option<FraudPreventionAssessment>,
}
/// The event being assessed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Optional. The user response token provided by the reCAPTCHA Enterprise
    /// client-side integration on your site.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// Optional. The site key that was used to invoke reCAPTCHA Enterprise on your
    /// site and generate the token.
    #[prost(string, tag = "2")]
    pub site_key: ::prost::alloc::string::String,
    /// Optional. The user agent present in the request from the user's device
    /// related to this event.
    #[prost(string, tag = "3")]
    pub user_agent: ::prost::alloc::string::String,
    /// Optional. The IP address in the request from the user's device related to
    /// this event.
    #[prost(string, tag = "4")]
    pub user_ip_address: ::prost::alloc::string::String,
    /// Optional. The expected action for this type of event. This should be the
    /// same action provided at token generation time on client-side platforms
    /// already integrated with recaptcha enterprise.
    #[prost(string, tag = "5")]
    pub expected_action: ::prost::alloc::string::String,
    /// Optional. Unique stable hashed user identifier for the request. The
    /// identifier must be hashed using hmac-sha256 with stable secret.
    #[prost(bytes = "vec", tag = "6")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
    /// Optional. Data describing a payment transaction to be assessed. Sending
    /// this data enables reCAPTCHA Enterprise Fraud Prevention and the
    /// FraudPreventionAssessment component in the response.
    #[prost(message, optional, tag = "13")]
    pub transaction_data: ::core::option::Option<TransactionData>,
}
/// Transaction data associated with a payment protected by reCAPTCHA Enterprise.
/// All fields are optional.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionData {
    /// Unique identifier for the transaction. This custom identifier can be used
    /// to reference this transaction in the future, for example, labeling a refund
    /// or chargeback event. Two attempts at the same transaction should use the
    /// same transaction id.
    #[prost(string, optional, tag = "11")]
    pub transaction_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The payment method for the transaction. The allowed values are:
    ///
    /// * credit-card
    /// * debit-card
    /// * gift-card
    /// * processor-{name} (If a third-party is used, for example,
    /// processor-paypal)
    /// * custom-{name} (If an alternative method is used, for example,
    /// custom-crypto)
    #[prost(string, tag = "1")]
    pub payment_method: ::prost::alloc::string::String,
    /// The Bank Identification Number - generally the first 6 or 8 digits of the
    /// card.
    #[prost(string, tag = "2")]
    pub card_bin: ::prost::alloc::string::String,
    /// The last four digits of the card.
    #[prost(string, tag = "3")]
    pub card_last_four: ::prost::alloc::string::String,
    /// The currency code in ISO-4217 format.
    #[prost(string, tag = "4")]
    pub currency_code: ::prost::alloc::string::String,
    /// The decimal value of the transaction in the specified currency.
    #[prost(double, tag = "5")]
    pub value: f64,
    /// The value of shipping in the specified currency. 0 for free or no shipping.
    #[prost(double, tag = "12")]
    pub shipping_value: f64,
    /// Destination address if this transaction involves shipping a physical item.
    #[prost(message, optional, tag = "6")]
    pub shipping_address: ::core::option::Option<transaction_data::Address>,
    /// Address associated with the payment method when applicable.
    #[prost(message, optional, tag = "7")]
    pub billing_address: ::core::option::Option<transaction_data::Address>,
    /// Information about the user paying/initiating the transaction.
    #[prost(message, optional, tag = "8")]
    pub user: ::core::option::Option<transaction_data::User>,
    /// Information about the user or users fulfilling the transaction.
    #[prost(message, repeated, tag = "13")]
    pub merchants: ::prost::alloc::vec::Vec<transaction_data::User>,
    /// Items purchased in this transaction.
    #[prost(message, repeated, tag = "14")]
    pub items: ::prost::alloc::vec::Vec<transaction_data::Item>,
    /// Information about the payment gateway's response to the transaction.
    #[prost(message, optional, tag = "10")]
    pub gateway_info: ::core::option::Option<transaction_data::GatewayInfo>,
}
/// Nested message and enum types in `TransactionData`.
pub mod transaction_data {
    /// Structured address format for billing and shipping addresses.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Address {
        /// The recipient name, potentially including information such as "care of".
        #[prost(string, tag = "1")]
        pub recipient: ::prost::alloc::string::String,
        /// The first lines of the address. The first line generally contains the
        /// street name and number, and further lines may include information such as
        /// an apartment number.
        #[prost(string, repeated, tag = "2")]
        pub address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The town/city of the address.
        #[prost(string, tag = "3")]
        pub locality: ::prost::alloc::string::String,
        /// The state, province, or otherwise administrative area of the address.
        #[prost(string, tag = "4")]
        pub administrative_area: ::prost::alloc::string::String,
        /// The CLDR country/region of the address.
        #[prost(string, tag = "5")]
        pub region_code: ::prost::alloc::string::String,
        /// The postal or ZIP code of the address.
        #[prost(string, tag = "6")]
        pub postal_code: ::prost::alloc::string::String,
    }
    /// Details about a user's account involved in the transaction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct User {
        /// Unique account identifier for this user. If using account defender,
        /// this should match the hashed_account_id field. Otherwise, a unique and
        /// persistent identifier for this account.
        #[prost(string, tag = "6")]
        pub account_id: ::prost::alloc::string::String,
        /// The epoch milliseconds of the user's account creation.
        #[prost(int64, tag = "1")]
        pub creation_ms: i64,
        /// The email address of the user.
        #[prost(string, tag = "2")]
        pub email: ::prost::alloc::string::String,
        /// Whether the email has been verified to be accessible by the user (OTP or
        /// similar).
        #[prost(bool, tag = "3")]
        pub email_verified: bool,
        /// The phone number of the user, with country code.
        #[prost(string, tag = "4")]
        pub phone_number: ::prost::alloc::string::String,
        /// Whether the phone number has been verified to be accessible by the user
        /// (OTP or similar).
        #[prost(bool, tag = "5")]
        pub phone_verified: bool,
    }
    /// Line items being purchased in this transaction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        /// The full name of the item.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The value per item that the user is paying, in the transaction currency,
        /// after discounts.
        #[prost(double, tag = "2")]
        pub value: f64,
        /// The quantity of this item that is being purchased.
        #[prost(int64, tag = "3")]
        pub quantity: i64,
        /// When a merchant is specified, its corresponding account_id. Necessary to
        /// populate marketplace-style transactions.
        #[prost(string, tag = "4")]
        pub merchant_account_id: ::prost::alloc::string::String,
    }
    /// Details about the transaction from the gateway.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GatewayInfo {
        /// Name of the gateway service (for example, stripe, square, paypal).
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Gateway response code describing the state of the transaction.
        #[prost(string, tag = "2")]
        pub gateway_response_code: ::prost::alloc::string::String,
        /// AVS response code from the gateway
        /// (available only when reCAPTCHA Enterprise is called after authorization).
        #[prost(string, tag = "3")]
        pub avs_response_code: ::prost::alloc::string::String,
        /// CVV response code from the gateway
        /// (available only when reCAPTCHA Enterprise is called after authorization).
        #[prost(string, tag = "4")]
        pub cvv_response_code: ::prost::alloc::string::String,
    }
}
/// Risk analysis result for an event.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RiskAnalysis {
    /// Legitimate event score from 0.0 to 1.0.
    /// (1.0 means very likely legitimate traffic while 0.0 means very likely
    /// non-legitimate traffic).
    #[prost(float, tag = "1")]
    pub score: f32,
    /// Reasons contributing to the risk analysis verdict.
    #[prost(enumeration = "risk_analysis::ClassificationReason", repeated, tag = "2")]
    pub reasons: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `RiskAnalysis`.
pub mod risk_analysis {
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
        /// The request matches behavioral characteristics of a carding attack.
        SuspectedCarding = 6,
        /// The request matches behavioral characteristics of chargebacks for fraud.
        SuspectedChargeback = 7,
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
                ClassificationReason::SuspectedCarding => "SUSPECTED_CARDING",
                ClassificationReason::SuspectedChargeback => "SUSPECTED_CHARGEBACK",
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
                "SUSPECTED_CARDING" => Some(Self::SuspectedCarding),
                "SUSPECTED_CHARGEBACK" => Some(Self::SuspectedChargeback),
                _ => None,
            }
        }
    }
}
/// Properties of the provided event token.
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
    /// The hostname of the page on which the token was generated (Web keys only).
    #[prost(string, tag = "4")]
    pub hostname: ::prost::alloc::string::String,
    /// The name of the Android package with which the token was generated (Android
    /// keys only).
    #[prost(string, tag = "8")]
    pub android_package_name: ::prost::alloc::string::String,
    /// The ID of the iOS bundle with which the token was generated (iOS keys
    /// only).
    #[prost(string, tag = "9")]
    pub ios_bundle_id: ::prost::alloc::string::String,
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
        /// The user verification token was not present.
        Missing = 5,
        /// A retriable error (such as network failure) occurred on the browser.
        /// Could easily be simulated by an attacker.
        BrowserError = 6,
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
                "MISSING" => Some(Self::Missing),
                "BROWSER_ERROR" => Some(Self::BrowserError),
                _ => None,
            }
        }
    }
}
/// Assessment for Fraud Prevention.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FraudPreventionAssessment {
    /// Probability (0-1) of this transaction being fraudulent. Summarizes the
    /// combined risk of attack vectors below.
    #[prost(float, tag = "1")]
    pub transaction_risk: f32,
    /// Assessment of this transaction for risk of a stolen instrument.
    #[prost(message, optional, tag = "2")]
    pub stolen_instrument_verdict: ::core::option::Option<
        fraud_prevention_assessment::StolenInstrumentVerdict,
    >,
    /// Assessment of this transaction for risk of being part of a card testing
    /// attack.
    #[prost(message, optional, tag = "3")]
    pub card_testing_verdict: ::core::option::Option<
        fraud_prevention_assessment::CardTestingVerdict,
    >,
}
/// Nested message and enum types in `FraudPreventionAssessment`.
pub mod fraud_prevention_assessment {
    /// Information about stolen instrument fraud, where the user is not the
    /// legitimate owner of the instrument being used for the purchase.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StolenInstrumentVerdict {
        /// Probability (0-1) of this transaction being executed with a stolen
        /// instrument.
        #[prost(float, tag = "1")]
        pub risk: f32,
    }
    /// Information about card testing fraud, where an adversary is testing
    /// fraudulently obtained cards or brute forcing their details.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CardTestingVerdict {
        /// Probability (0-1) of this transaction attempt being part of a card
        /// testing attack.
        #[prost(float, tag = "1")]
        pub risk: f32,
    }
}
/// Account defender risk assessment.
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
    /// Labels returned by account defender for this request.
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
        /// The request is potentially a suspicious login event and must be further
        /// verified either through multi-factor authentication or another system.
        SuspiciousLoginActivity = 2,
        /// The request matched a profile that previously had suspicious account
        /// creation behavior. This can mean that this is a fake account.
        SuspiciousAccountCreation = 3,
        /// The account in the request has a high number of related accounts. It does
        /// not necessarily imply that the account is bad but can require further
        /// investigation.
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
/// The create key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyRequest {
    /// Required. The name of the project in which the key will be created, in the
    /// format "projects/{project}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Information to create a reCAPTCHA Enterprise key.
    #[prost(message, optional, tag = "2")]
    pub key: ::core::option::Option<Key>,
}
/// The list keys request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeysRequest {
    /// Required. The name of the project that contains the keys that will be
    /// listed, in the format "projects/{project}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of keys to return. Default is 10. Max limit is
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous.
    /// ListKeysRequest, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response to request to list keys in a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeysResponse {
    /// Key details.
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<Key>,
    /// Token to retrieve the next page of results. It is set to empty if no keys
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The retrieve legacy secret key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveLegacySecretKeyRequest {
    /// Required. The public key name linked to the requested secret key in the
    /// format "projects/{project}/keys/{key}".
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// The get key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyRequest {
    /// Required. The name of the requested key, in the format
    /// "projects/{project}/keys/{key}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The update key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateKeyRequest {
    /// Required. The key to update.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<Key>,
    /// Optional. The mask to control which fields of the key get updated. If the
    /// mask is not present, all fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The delete key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeyRequest {
    /// Required. The name of the key to be deleted, in the format
    /// "projects/{project}/keys/{key}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The migrate key request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateKeyRequest {
    /// Required. The name of the key to be migrated, in the format
    /// "projects/{project}/keys/{key}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If true, skips the billing check.
    /// A reCAPTCHA Enterprise key or migrated key behaves differently than a
    /// reCAPTCHA (non-Enterprise version) key when you reach a quota limit (see
    /// <https://cloud.google.com/recaptcha-enterprise/quotas#quota_limit>). To avoid
    /// any disruption of your usage, we check that a billing account is present.
    /// If your usage of reCAPTCHA is under the free quota, you can safely skip the
    /// billing check and proceed with the migration. See
    /// <https://cloud.google.com/recaptcha-enterprise/docs/billing-information.>
    #[prost(bool, tag = "2")]
    pub skip_billing_check: bool,
}
/// The get metrics request message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricsRequest {
    /// Required. The name of the requested metrics, in the format
    /// "projects/{project}/keys/{key}/metrics".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metrics for a single Key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metrics {
    /// Output only. The name of the metrics, in the format
    /// "projects/{project}/keys/{key}/metrics".
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// Inclusive start time aligned to a day (UTC).
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Metrics will be continuous and in order by dates, and in the granularity
    /// of day. All Key types should have score-based data.
    #[prost(message, repeated, tag = "2")]
    pub score_metrics: ::prost::alloc::vec::Vec<ScoreMetrics>,
    /// Metrics will be continuous and in order by dates, and in the granularity
    /// of day. Only challenge-based keys (CHECKBOX, INVISIBLE), will have
    /// challenge-based data.
    #[prost(message, repeated, tag = "3")]
    pub challenge_metrics: ::prost::alloc::vec::Vec<ChallengeMetrics>,
}
/// Secret key is used only in legacy reCAPTCHA. It must be used in a 3rd party
/// integration with legacy reCAPTCHA.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrieveLegacySecretKeyResponse {
    /// The secret key (also known as shared secret) authorizes communication
    /// between your application backend and the reCAPTCHA Enterprise server to
    /// create an assessment.
    /// The secret key needs to be kept safe for security purposes.
    #[prost(string, tag = "1")]
    pub legacy_secret_key: ::prost::alloc::string::String,
}
/// A key used to identify and configure applications (web and/or mobile) that
/// use reCAPTCHA Enterprise.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    /// The resource name for the Key in the format
    /// "projects/{project}/keys/{key}".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human-readable display name of this key. Modifiable by user.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// See <a href="<https://cloud.google.com/recaptcha-enterprise/docs/labels">>
    /// Creating and managing labels</a>.
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The timestamp corresponding to the creation of this Key.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Options for user acceptance testing.
    #[prost(message, optional, tag = "9")]
    pub testing_options: ::core::option::Option<TestingOptions>,
    /// Settings for WAF
    #[prost(message, optional, tag = "10")]
    pub waf_settings: ::core::option::Option<WafSettings>,
    /// Platform specific settings for this key. The key can only be used on a
    /// platform for which the settings are enabled.
    #[prost(oneof = "key::PlatformSettings", tags = "3, 4, 5")]
    pub platform_settings: ::core::option::Option<key::PlatformSettings>,
}
/// Nested message and enum types in `Key`.
pub mod key {
    /// Platform specific settings for this key. The key can only be used on a
    /// platform for which the settings are enabled.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PlatformSettings {
        /// Settings for keys that can be used by websites.
        #[prost(message, tag = "3")]
        WebSettings(super::WebKeySettings),
        /// Settings for keys that can be used by Android apps.
        #[prost(message, tag = "4")]
        AndroidSettings(super::AndroidKeySettings),
        /// Settings for keys that can be used by iOS apps.
        #[prost(message, tag = "5")]
        IosSettings(super::IosKeySettings),
    }
}
/// Options for user acceptance testing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestingOptions {
    /// All assessments for this Key will return this score. Must be between 0
    /// (likely not legitimate) and 1 (likely legitimate) inclusive.
    #[prost(float, tag = "1")]
    pub testing_score: f32,
    /// For challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests
    /// for this site will return nocaptcha if NOCAPTCHA, or an unsolvable
    /// challenge if CHALLENGE.
    #[prost(enumeration = "testing_options::TestingChallenge", tag = "2")]
    pub testing_challenge: i32,
}
/// Nested message and enum types in `TestingOptions`.
pub mod testing_options {
    /// Enum that represents the challenge option for challenge-based (CHECKBOX,
    /// INVISIBLE) testing keys.
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
    pub enum TestingChallenge {
        /// Perform the normal risk analysis and return either nocaptcha or a
        /// challenge depending on risk and trust factors.
        Unspecified = 0,
        /// Challenge requests for this key always return a nocaptcha, which
        /// does not require a solution.
        Nocaptcha = 1,
        /// Challenge requests for this key always return an unsolvable
        /// challenge.
        UnsolvableChallenge = 2,
    }
    impl TestingChallenge {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TestingChallenge::Unspecified => "TESTING_CHALLENGE_UNSPECIFIED",
                TestingChallenge::Nocaptcha => "NOCAPTCHA",
                TestingChallenge::UnsolvableChallenge => "UNSOLVABLE_CHALLENGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TESTING_CHALLENGE_UNSPECIFIED" => Some(Self::Unspecified),
                "NOCAPTCHA" => Some(Self::Nocaptcha),
                "UNSOLVABLE_CHALLENGE" => Some(Self::UnsolvableChallenge),
                _ => None,
            }
        }
    }
}
/// Settings specific to keys that can be used by websites.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebKeySettings {
    /// If set to true, it means allowed_domains will not be enforced.
    #[prost(bool, tag = "3")]
    pub allow_all_domains: bool,
    /// Domains or subdomains of websites allowed to use the key. All subdomains
    /// of an allowed domain are automatically allowed. A valid domain requires a
    /// host and must not include any path, port, query or fragment.
    /// Examples: 'example.com' or 'subdomain.example.com'
    #[prost(string, repeated, tag = "1")]
    pub allowed_domains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If set to true, the key can be used on AMP (Accelerated Mobile Pages)
    /// websites. This is supported only for the SCORE integration type.
    #[prost(bool, tag = "2")]
    pub allow_amp_traffic: bool,
    /// Required. Describes how this key is integrated with the website.
    #[prost(enumeration = "web_key_settings::IntegrationType", tag = "4")]
    pub integration_type: i32,
    /// Settings for the frequency and difficulty at which this key triggers
    /// captcha challenges. This should only be specified for IntegrationTypes
    /// CHECKBOX and INVISIBLE.
    #[prost(enumeration = "web_key_settings::ChallengeSecurityPreference", tag = "5")]
    pub challenge_security_preference: i32,
}
/// Nested message and enum types in `WebKeySettings`.
pub mod web_key_settings {
    /// Enum that represents the integration types for web keys.
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
    pub enum IntegrationType {
        /// Default type that indicates this enum hasn't been specified. This is not
        /// a valid IntegrationType, one of the other types must be specified
        /// instead.
        Unspecified = 0,
        /// Only used to produce scores. It doesn't display the "I'm not a robot"
        /// checkbox and never shows captcha challenges.
        Score = 1,
        /// Displays the "I'm not a robot" checkbox and may show captcha challenges
        /// after it is checked.
        Checkbox = 2,
        /// Doesn't display the "I'm not a robot" checkbox, but may show captcha
        /// challenges after risk analysis.
        Invisible = 3,
    }
    impl IntegrationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IntegrationType::Unspecified => "INTEGRATION_TYPE_UNSPECIFIED",
                IntegrationType::Score => "SCORE",
                IntegrationType::Checkbox => "CHECKBOX",
                IntegrationType::Invisible => "INVISIBLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INTEGRATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "SCORE" => Some(Self::Score),
                "CHECKBOX" => Some(Self::Checkbox),
                "INVISIBLE" => Some(Self::Invisible),
                _ => None,
            }
        }
    }
    /// Enum that represents the possible challenge frequency and difficulty
    /// configurations for a web key.
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
    pub enum ChallengeSecurityPreference {
        /// Default type that indicates this enum hasn't been specified.
        Unspecified = 0,
        /// Key tends to show fewer and easier challenges.
        Usability = 1,
        /// Key tends to show balanced (in amount and difficulty) challenges.
        Balance = 2,
        /// Key tends to show more and harder challenges.
        Security = 3,
    }
    impl ChallengeSecurityPreference {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ChallengeSecurityPreference::Unspecified => {
                    "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED"
                }
                ChallengeSecurityPreference::Usability => "USABILITY",
                ChallengeSecurityPreference::Balance => "BALANCE",
                ChallengeSecurityPreference::Security => "SECURITY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED" => Some(Self::Unspecified),
                "USABILITY" => Some(Self::Usability),
                "BALANCE" => Some(Self::Balance),
                "SECURITY" => Some(Self::Security),
                _ => None,
            }
        }
    }
}
/// Settings specific to keys that can be used by Android apps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidKeySettings {
    /// If set to true, allowed_package_names are not enforced.
    #[prost(bool, tag = "2")]
    pub allow_all_package_names: bool,
    /// Android package names of apps allowed to use the key.
    /// Example: 'com.companyname.appname'
    #[prost(string, repeated, tag = "1")]
    pub allowed_package_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Settings specific to keys that can be used by iOS apps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosKeySettings {
    /// If set to true, allowed_bundle_ids are not enforced.
    #[prost(bool, tag = "2")]
    pub allow_all_bundle_ids: bool,
    /// iOS bundle ids of apps allowed to use the key.
    /// Example: 'com.companyname.productname.appname'
    #[prost(string, repeated, tag = "1")]
    pub allowed_bundle_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Score distribution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoreDistribution {
    /// Map key is score value multiplied by 100. The scores are discrete values
    /// between [0, 1]. The maximum number of buckets is on order of a few dozen,
    /// but typically much lower (ie. 10).
    #[prost(map = "int32, int64", tag = "1")]
    pub score_buckets: ::std::collections::HashMap<i32, i64>,
}
/// Metrics related to scoring.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoreMetrics {
    /// Aggregated score metrics for all traffic.
    #[prost(message, optional, tag = "1")]
    pub overall_metrics: ::core::option::Option<ScoreDistribution>,
    /// Action-based metrics. The map key is the action name which specified by the
    /// site owners at time of the "execute" client-side call.
    #[prost(map = "string, message", tag = "2")]
    pub action_metrics: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ScoreDistribution,
    >,
}
/// Metrics related to challenges.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChallengeMetrics {
    /// Count of reCAPTCHA checkboxes or badges rendered. This is mostly equivalent
    /// to a count of pageloads for pages that include reCAPTCHA.
    #[prost(int64, tag = "1")]
    pub pageload_count: i64,
    /// Count of nocaptchas (successful verification without a challenge) issued.
    #[prost(int64, tag = "2")]
    pub nocaptcha_count: i64,
    /// Count of submitted challenge solutions that were incorrect or otherwise
    /// deemed suspicious such that a subsequent challenge was triggered.
    #[prost(int64, tag = "3")]
    pub failed_count: i64,
    /// Count of nocaptchas (successful verification without a challenge) plus
    /// submitted challenge solutions that were correct and resulted in
    /// verification.
    #[prost(int64, tag = "4")]
    pub passed_count: i64,
}
/// The request message to list memberships in a related account group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRelatedAccountGroupMembershipsRequest {
    /// Required. The resource name for the related account group in the format
    /// `projects/{project}/relatedaccountgroups/{relatedaccountgroup}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of accounts to return. The service might
    /// return fewer than this value. If unspecified, at most 50 accounts are
    /// returned. The maximum value is 1000; values above 1000 are coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `ListRelatedAccountGroupMemberships` call.
    ///
    /// When paginating, all other parameters provided to
    /// `ListRelatedAccountGroupMemberships` must match the call that provided the
    /// page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response to a `ListRelatedAccountGroupMemberships` call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRelatedAccountGroupMembershipsResponse {
    /// The memberships listed by the query.
    #[prost(message, repeated, tag = "1")]
    pub related_account_group_memberships: ::prost::alloc::vec::Vec<
        RelatedAccountGroupMembership,
    >,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message to list related account groups.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRelatedAccountGroupsRequest {
    /// Required. The name of the project to list related account groups from, in
    /// the format "projects/{project}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of groups to return. The service might return
    /// fewer than this value. If unspecified, at most 50 groups are returned. The
    /// maximum value is 1000; values above 1000 are coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListRelatedAccountGroups`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListRelatedAccountGroups` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response to a `ListRelatedAccountGroups` call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRelatedAccountGroupsResponse {
    /// The groups of related accounts listed by the query.
    #[prost(message, repeated, tag = "1")]
    pub related_account_groups: ::prost::alloc::vec::Vec<RelatedAccountGroup>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message to search related account group memberships.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRelatedAccountGroupMembershipsRequest {
    /// Required. The name of the project to search related account group
    /// memberships from. Specify the project name in the following format:
    /// "projects/{project}".
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// Optional. The unique stable hashed user identifier we should search
    /// connections to. The identifier should correspond to a `hashed_account_id`
    /// provided in a previous `CreateAssessment` or `AnnotateAssessment` call.
    #[prost(bytes = "vec", tag = "2")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
    /// Optional. The maximum number of groups to return. The service might return
    /// fewer than this value. If unspecified, at most 50 groups are returned. The
    /// maximum value is 1000; values above 1000 are coerced to 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `SearchRelatedAccountGroupMemberships` call. Provide this to retrieve the
    /// subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `SearchRelatedAccountGroupMemberships` must match the call that provided
    /// the page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response to a `SearchRelatedAccountGroupMemberships` call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRelatedAccountGroupMembershipsResponse {
    /// The queried memberships.
    #[prost(message, repeated, tag = "1")]
    pub related_account_group_memberships: ::prost::alloc::vec::Vec<
        RelatedAccountGroupMembership,
    >,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A membership in a group of related accounts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedAccountGroupMembership {
    /// Required. The resource name for this membership in the format
    /// `projects/{project}/relatedaccountgroups/{relatedaccountgroup}/memberships/{membership}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The unique stable hashed user identifier of the member. The identifier
    /// corresponds to a `hashed_account_id` provided in a previous
    /// `CreateAssessment` or `AnnotateAssessment` call.
    #[prost(bytes = "vec", tag = "2")]
    pub hashed_account_id: ::prost::alloc::vec::Vec<u8>,
}
/// A group of related accounts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedAccountGroup {
    /// Required. The resource name for the related account group in the format
    /// `projects/{project}/relatedaccountgroups/{related_account_group}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Settings specific to keys that can be used for WAF (Web Application
/// Firewall).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WafSettings {
    /// Required. The WAF service that uses this key.
    #[prost(enumeration = "waf_settings::WafService", tag = "1")]
    pub waf_service: i32,
    /// Required. The WAF feature for which this key is enabled.
    #[prost(enumeration = "waf_settings::WafFeature", tag = "2")]
    pub waf_feature: i32,
}
/// Nested message and enum types in `WafSettings`.
pub mod waf_settings {
    /// Supported WAF features. For more information, see
    /// <https://cloud.google.com/recaptcha-enterprise/docs/usecase#comparison_of_features.>
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
    pub enum WafFeature {
        /// Undefined feature.
        Unspecified = 0,
        /// Redirects suspicious traffic to reCAPTCHA.
        ChallengePage = 1,
        /// Use reCAPTCHA session-tokens to protect the whole user session on the
        /// site's domain.
        SessionToken = 2,
        /// Use reCAPTCHA action-tokens to protect user actions.
        ActionToken = 3,
    }
    impl WafFeature {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WafFeature::Unspecified => "WAF_FEATURE_UNSPECIFIED",
                WafFeature::ChallengePage => "CHALLENGE_PAGE",
                WafFeature::SessionToken => "SESSION_TOKEN",
                WafFeature::ActionToken => "ACTION_TOKEN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WAF_FEATURE_UNSPECIFIED" => Some(Self::Unspecified),
                "CHALLENGE_PAGE" => Some(Self::ChallengePage),
                "SESSION_TOKEN" => Some(Self::SessionToken),
                "ACTION_TOKEN" => Some(Self::ActionToken),
                _ => None,
            }
        }
    }
    /// Web Application Firewalls supported by reCAPTCHA Enterprise.
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
    pub enum WafService {
        /// Undefined WAF
        Unspecified = 0,
        /// Cloud Armor
        Ca = 1,
    }
    impl WafService {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WafService::Unspecified => "WAF_SERVICE_UNSPECIFIED",
                WafService::Ca => "CA",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WAF_SERVICE_UNSPECIFIED" => Some(Self::Unspecified),
                "CA" => Some(Self::Ca),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod recaptcha_enterprise_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to determine the likelihood an event is legitimate.
    #[derive(Debug, Clone)]
    pub struct RecaptchaEnterpriseServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RecaptchaEnterpriseServiceClient<tonic::transport::Channel> {
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
    impl<T> RecaptchaEnterpriseServiceClient<T>
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
        ) -> RecaptchaEnterpriseServiceClient<InterceptedService<T, F>>
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
            RecaptchaEnterpriseServiceClient::new(
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/CreateAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "CreateAssessment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Annotates a previously created Assessment to provide additional information
        /// on whether the event turned out to be authentic or fraudulent.
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/AnnotateAssessment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "AnnotateAssessment",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new reCAPTCHA Enterprise key.
        pub async fn create_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::Key>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/CreateKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "CreateKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the list of all keys that belong to a project.
        pub async fn list_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListKeysResponse>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/ListKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "ListKeys",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the secret key related to the specified public key.
        /// You must use the legacy secret key only in a 3rd party integration with
        /// legacy reCAPTCHA.
        pub async fn retrieve_legacy_secret_key(
            &mut self,
            request: impl tonic::IntoRequest<super::RetrieveLegacySecretKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RetrieveLegacySecretKeyResponse>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/RetrieveLegacySecretKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "RetrieveLegacySecretKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the specified key.
        pub async fn get_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::Key>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/GetKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "GetKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates the specified key.
        pub async fn update_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::Key>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/UpdateKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "UpdateKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified key.
        pub async fn delete_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteKeyRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/DeleteKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "DeleteKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Migrates an existing key from reCAPTCHA to reCAPTCHA Enterprise.
        /// Once a key is migrated, it can be used from either product. SiteVerify
        /// requests are billed as CreateAssessment calls. You must be
        /// authenticated as one of the current owners of the reCAPTCHA Site Key, and
        /// your user must have the reCAPTCHA Enterprise Admin IAM role in the
        /// destination project.
        pub async fn migrate_key(
            &mut self,
            request: impl tonic::IntoRequest<super::MigrateKeyRequest>,
        ) -> std::result::Result<tonic::Response<super::Key>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/MigrateKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "MigrateKey",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get some aggregated metrics for a Key. This data can be used to build
        /// dashboards.
        pub async fn get_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetricsRequest>,
        ) -> std::result::Result<tonic::Response<super::Metrics>, tonic::Status> {
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/GetMetrics",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "GetMetrics",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List groups of related accounts.
        pub async fn list_related_account_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRelatedAccountGroupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRelatedAccountGroupsResponse>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/ListRelatedAccountGroups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "ListRelatedAccountGroups",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get memberships in a group of related accounts.
        pub async fn list_related_account_group_memberships(
            &mut self,
            request: impl tonic::IntoRequest<
                super::ListRelatedAccountGroupMembershipsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::ListRelatedAccountGroupMembershipsResponse>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/ListRelatedAccountGroupMemberships",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "ListRelatedAccountGroupMemberships",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Search group memberships related to a given account.
        pub async fn search_related_account_group_memberships(
            &mut self,
            request: impl tonic::IntoRequest<
                super::SearchRelatedAccountGroupMembershipsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::SearchRelatedAccountGroupMembershipsResponse>,
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
                "/google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService/SearchRelatedAccountGroupMemberships",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.recaptchaenterprise.v1.RecaptchaEnterpriseService",
                        "SearchRelatedAccountGroupMemberships",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
