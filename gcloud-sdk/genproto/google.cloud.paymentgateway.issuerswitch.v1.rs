/// A reference to uniquely identify an account according to India's UPI
/// standards.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountReference {
    /// IFSC code of a bank's branch.
    #[prost(string, tag="1")]
    pub ifsc_code: ::prost::alloc::string::String,
    /// Type of account. Examples include SAVINGS, CURRENT, etc.
    #[prost(string, tag="2")]
    pub account_type: ::prost::alloc::string::String,
    /// Unique number for an account in a bank and branch.
    #[prost(string, tag="3")]
    pub account_number: ::prost::alloc::string::String,
}
/// A participant in a payment settlement transaction processed by the issuer
/// switch. The participant could either be the payer or the payee in the
/// transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettlementParticipant {
    /// The participant information.
    #[prost(message, optional, tag="1")]
    pub participant: ::core::option::Option<Participant>,
    /// Unique identification of an account according to India's UPI standards.
    #[prost(message, optional, tag="2")]
    pub account: ::core::option::Option<AccountReference>,
    /// Information about a merchant who is a participant in the payment. This
    /// field will be specified only if the participant is a merchant.
    #[prost(message, optional, tag="3")]
    pub merchant_info: ::core::option::Option<MerchantInfo>,
    /// Output only. The mobile number of the participant.
    #[prost(string, tag="4")]
    pub mobile: ::prost::alloc::string::String,
    /// Output only. The device id of the participant.
    #[prost(string, tag="5")]
    pub device_id: ::prost::alloc::string::String,
}
/// A participant in a transaction processed by the issuer switch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Participant {
    /// The virtual payment address (VPA) of the participant.
    #[prost(string, tag="1")]
    pub virtual_payment_address: ::prost::alloc::string::String,
    /// The persona of the participant.
    #[prost(enumeration="participant::Persona", tag="2")]
    pub persona: i32,
    /// The name of the participant.
    #[prost(string, tag="3")]
    pub user: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Participant`.
pub mod participant {
    /// The type of the participant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Persona {
        /// Unspecified persona.
        Unspecified = 0,
        /// Participant is an entity.
        Entity = 1,
        /// Participant is a person.
        Person = 2,
    }
    impl Persona {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Persona::Unspecified => "PERSONA_UNSPECIFIED",
                Persona::Entity => "ENTITY",
                Persona::Person => "PERSON",
            }
        }
    }
}
/// A merchant entity participating in a payment settlement transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantInfo {
    /// A unique identifier for the merchant.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The name of the merchant who is a party in the payment. Includes multiple
    /// possible names for the merchant.
    #[prost(message, optional, tag="2")]
    pub merchant: ::core::option::Option<MerchantName>,
    /// Additional information about the merchant.
    #[prost(message, optional, tag="3")]
    pub additional_info: ::core::option::Option<MerchantAdditionalInfo>,
}
/// The name of a merchant who is a participant in a payment settlement
/// transaction. Includes multiple possible names for the merchant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantName {
    /// The brand name of the merchant.
    #[prost(string, tag="1")]
    pub brand: ::prost::alloc::string::String,
    /// The merchant's legal name.
    #[prost(string, tag="2")]
    pub legal: ::prost::alloc::string::String,
    /// The franchise name under which the merchant operates.
    #[prost(string, tag="3")]
    pub franchise: ::prost::alloc::string::String,
}
/// Additional merchant information specific to India's UPI requirements.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantAdditionalInfo {
    /// Merchant Category Code (MCC) as specified by UPI. This is a four-digit
    /// number listed in ISO 18245 for retail financial services.
    #[prost(string, tag="1")]
    pub category_code: ::prost::alloc::string::String,
    /// A unique identifier for the merchant store where the payment settlement
    /// transaction occurred.
    #[prost(string, tag="2")]
    pub store_id: ::prost::alloc::string::String,
    /// A unique identifier for the POS terminal in the store where the payment
    /// settlement transaction occurred.
    #[prost(string, tag="3")]
    pub terminal_id: ::prost::alloc::string::String,
    /// Indicates the type of merchant.
    #[prost(enumeration="merchant_additional_info::Type", tag="4")]
    pub r#type: i32,
    /// Indicates the genre of the merchant.
    #[prost(enumeration="merchant_additional_info::Genre", tag="5")]
    pub genre: i32,
    /// Indicates the merchant's onboarding type.
    #[prost(enumeration="merchant_additional_info::OnboardingType", tag="6")]
    pub onboarding_type: i32,
    /// Indicates the merchant's owner type.
    #[prost(enumeration="merchant_additional_info::OwnershipType", tag="7")]
    pub ownership_type: i32,
}
/// Nested message and enum types in `MerchantAdditionalInfo`.
pub mod merchant_additional_info {
    /// Indicates the merchant's type as a small or large merchant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified merchant type.
        Unspecified = 0,
        /// Large merchant.
        Large = 1,
        /// Small merchant.
        Small = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::Large => "LARGE",
                Type::Small => "SMALL",
            }
        }
    }
    /// Indicates whether the merchant is an online or offline merchant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Genre {
        /// Unspecified merchant genre.
        Unspecified = 0,
        /// Offline merchant
        Offline = 1,
        /// Online merchant.
        Online = 2,
    }
    impl Genre {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Genre::Unspecified => "GENRE_UNSPECIFIED",
                Genre::Offline => "OFFLINE",
                Genre::Online => "ONLINE",
            }
        }
    }
    /// Indicates whether the merchant has been onboarded by a bank or an
    /// aggregator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OnboardingType {
        /// Unspecified merchant onboarding type.
        Unspecified = 0,
        /// Onboarded by aggreagator.
        Aggregator = 1,
        /// Onboarded by bank.
        Bank = 2,
        /// Onboarded by the UPI network.
        Network = 3,
        /// Onboarded by the TPAP.
        Tpap = 4,
    }
    impl OnboardingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OnboardingType::Unspecified => "ONBOARDING_TYPE_UNSPECIFIED",
                OnboardingType::Aggregator => "AGGREGATOR",
                OnboardingType::Bank => "BANK",
                OnboardingType::Network => "NETWORK",
                OnboardingType::Tpap => "TPAP",
            }
        }
    }
    /// Indicates the ownership type of the merchant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OwnershipType {
        /// Unspecified merchant ownership type.
        Unspecified = 0,
        /// Properietary ownership.
        Proprietary = 1,
        /// Partnership ownership.
        Partnership = 2,
        /// Public ownership.
        Public = 3,
        /// Private ownership.
        Private = 4,
        /// Other ownership model.
        Others = 5,
    }
    impl OwnershipType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OwnershipType::Unspecified => "OWNERSHIP_TYPE_UNSPECIFIED",
                OwnershipType::Proprietary => "PROPRIETARY",
                OwnershipType::Partnership => "PARTNERSHIP",
                OwnershipType::Public => "PUBLIC",
                OwnershipType::Private => "PRIVATE",
                OwnershipType::Others => "OTHERS",
            }
        }
    }
}
/// The API type for a transaction. Every transaction processed by the issuer
/// switch will be one of these API types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApiType {
    /// Unspecified API type.
    Unspecified = 0,
    /// Balance API. Maps to UPI's `BalEnq` API. This is a metadata
    /// transaction API.
    Balance = 1,
    /// Check transaction status API. Maps to UPI's `ChkTxn` API. This is a
    /// metadata transaction API.
    CheckStatus = 2,
    /// Complain API. Maps to UPI's `Complaint` API. This is a metadata transaction
    /// API.
    Complaint = 3,
    /// Heart beat API. Maps to UPI's `Hbt` API. This is a metadata transaction
    /// API.
    HeartBeat = 4,
    /// Initiate registration API. Maps to UPI's `Otp` API. This is a metadata
    /// transaction API.
    InitiateRegistration = 5,
    /// List accounts API. Maps to UPI's `ListAccount` API. This is a metadata
    /// transaction API.
    ListAccounts = 6,
    /// Mandate API. Maps to UPI's `Mandate` API. This is a metadata transaction
    /// API.
    Mandate = 7,
    /// Payment settlement API. Maps to UPI's `Pay` API. This is a financial
    /// transaction API.
    SettlePayment = 8,
    /// Update credentials API. Maps to UPI's `SetCre` API. This is a metadata
    /// transaction API.
    UpdateCredentials = 9,
    /// Validate registration API. Maps to UPI's `RegMob` API. This is a metadata
    /// transaction API.
    ValidateRegistration = 10,
    /// Voucher confirmation API. Maps to UPI's `VoucherConfirmation` API.
    VoucherConfirmation = 11,
}
impl ApiType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ApiType::Unspecified => "API_TYPE_UNSPECIFIED",
            ApiType::Balance => "BALANCE",
            ApiType::CheckStatus => "CHECK_STATUS",
            ApiType::Complaint => "COMPLAINT",
            ApiType::HeartBeat => "HEART_BEAT",
            ApiType::InitiateRegistration => "INITIATE_REGISTRATION",
            ApiType::ListAccounts => "LIST_ACCOUNTS",
            ApiType::Mandate => "MANDATE",
            ApiType::SettlePayment => "SETTLE_PAYMENT",
            ApiType::UpdateCredentials => "UPDATE_CREDENTIALS",
            ApiType::ValidateRegistration => "VALIDATE_REGISTRATION",
            ApiType::VoucherConfirmation => "VOUCHER_CONFIRMATION",
        }
    }
}
/// The type of a transaction. Every transaction processed by the issuer switch
/// will be one of these transaction types. Transaction types are associated with
/// a particular API type. This associated is documented with each value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionType {
    /// Unspecified transaction type.
    Unspecified = 0,
    /// Autoupdate transaction type. This is associated with the `CHECK_STATUS`
    /// API type. Maps to UPI's `AUTOUPDATE` type.
    Autoupdate = 1,
    /// Balance check transaction type. This is associated with the
    /// `BALANCE_ENQUIRY` API type. Maps to UPI's `BalChk` type.
    BalanceCheck = 3,
    /// Balance enquiry transaction type. This is associated with the
    /// `BALANCE_ENQUIRY` API type. Maps to UPI's `BalEnq` type.
    BalanceEnquiry = 4,
    /// Check status transaction type. This is associated with the `COMPLAINT` API
    /// type. Maps to UPI's `CHECKSTATUS` type.
    CheckStatus = 5,
    /// Check transaction type. This is associated with the `CHECK_STATUS` API
    /// type. Maps to UPI's `ChkTxn` type.
    CheckTransaction = 6,
    /// Complaint transaction type. This is associated with the `COMPLAINT` API
    /// type. Maps to UPI's `COMPLAINT` type.
    Complaint = 7,
    /// Create transaction type. This is associated with the `MANDATE` API type.
    /// Maps to UPI's `CREATE` type.
    Create = 8,
    /// Credit transaction type. This is associated with the `SETTLE_PAYMENT` API
    /// type. Maps to UPI's `CREDIT` type.
    Credit = 9,
    /// Debit transaction type. This is associated with the `SETTLE_PAYMENT` API
    /// type. Maps to UPI's `DEBIT` type.
    Debit = 10,
    /// Dispute transaction type. This is associated with the `COMPLAINT` API
    /// type. Maps to UPI's `DISPUTE` type.
    Dispute = 11,
    /// Heart beat transaction type. This is associated with `HEART_BEAT` API type.
    /// Maps to UPI's `Hbt` type.
    HeartBeat = 12,
    /// List accounts transaction type. This is associated with `LIST_ACCOUNTS` API
    /// type. Maps to UPI's `ListAccount` type.
    ListAccounts = 13,
    /// OTP transaction type. This is associated with the `INITIATE_REGISTRATION`
    /// API type. Maps to UPI's `Otp` type.
    Otp = 14,
    /// Register mobile transaction type. This is associated with the
    /// `VALIDATE_REGISTRATION` API type. Maps to UPI's `RegMob` type.
    RegisterMobile = 15,
    /// Refund transaction type. This is associated with the `COMPLAINT` API
    /// type. Maps to UPI's `REFUND` type.
    Refund = 16,
    /// Reversal transaction type. This is associated with the `SETTLE_PAYMENT` and
    /// `COMPLAINT` API types. Maps to UPI's `REVERSAL` type.
    Reversal = 17,
    /// Revoke transaction type. This is associated with the `MANDATE` API type.
    /// Maps to UPI's `REVOKE` type.
    Revoke = 18,
    /// Status update transaction type. This is associated with the `COMPLAINT` API
    /// type. Maps to UPI's `STATUSUPDATE` type.
    StatusUpdate = 19,
    /// Update transaction type. This is associated with the `MANDATE` API type.
    /// Maps to UPI's `UPDATE` type.
    Update = 20,
    /// Update credentials transaction type. This is associated with
    /// `UPDATE_CREDENTIALS` API type. Maps to UPI's `SetCre` type.
    UpdateCredentials = 21,
    /// Redeem transaction type. This is associated with the `VOUCHER_CONFIRMATION`
    /// API type. Maps to UPI's `REDEEM` type.
    Redeem = 22,
}
impl TransactionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionType::Unspecified => "TRANSACTION_TYPE_UNSPECIFIED",
            TransactionType::Autoupdate => "TRANSACTION_TYPE_AUTOUPDATE",
            TransactionType::BalanceCheck => "TRANSACTION_TYPE_BALANCE_CHECK",
            TransactionType::BalanceEnquiry => "TRANSACTION_TYPE_BALANCE_ENQUIRY",
            TransactionType::CheckStatus => "TRANSACTION_TYPE_CHECK_STATUS",
            TransactionType::CheckTransaction => "TRANSACTION_TYPE_CHECK_TRANSACTION",
            TransactionType::Complaint => "TRANSACTION_TYPE_COMPLAINT",
            TransactionType::Create => "TRANSACTION_TYPE_CREATE",
            TransactionType::Credit => "TRANSACTION_TYPE_CREDIT",
            TransactionType::Debit => "TRANSACTION_TYPE_DEBIT",
            TransactionType::Dispute => "TRANSACTION_TYPE_DISPUTE",
            TransactionType::HeartBeat => "TRANSACTION_TYPE_HEART_BEAT",
            TransactionType::ListAccounts => "TRANSACTION_TYPE_LIST_ACCOUNTS",
            TransactionType::Otp => "TRANSACTION_TYPE_OTP",
            TransactionType::RegisterMobile => "TRANSACTION_TYPE_REGISTER_MOBILE",
            TransactionType::Refund => "TRANSACTION_TYPE_REFUND",
            TransactionType::Reversal => "TRANSACTION_TYPE_REVERSAL",
            TransactionType::Revoke => "TRANSACTION_TYPE_REVOKE",
            TransactionType::StatusUpdate => "TRANSACTION_TYPE_STATUS_UPDATE",
            TransactionType::Update => "TRANSACTION_TYPE_UPDATE",
            TransactionType::UpdateCredentials => "TRANSACTION_TYPE_UPDATE_CREDENTIALS",
            TransactionType::Redeem => "TRANSACTION_TYPE_REDEEM",
        }
    }
}
/// A complaint processed by the issuer switch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Complaint {
    /// The name of the complaint. This uniquely identifies the complaint.
    /// Format of name is
    /// projects/{project_id}/complaints/{complaint_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The reason for raising the complaint. This maps adjustment flag
    /// and reason code for the complaint to `reqAdjFlag` and `reqAdjCode` in
    /// complaint request respectively while raising a complaint.
    #[prost(message, optional, tag="2")]
    pub raise_complaint_adjustment: ::core::option::Option<RaiseComplaintAdjustment>,
    /// Required. Details required for raising / resolving a complaint.
    #[prost(message, optional, tag="4")]
    pub details: ::core::option::Option<CaseDetails>,
    /// Output only. Response to the raised / resolved complaint.
    #[prost(message, optional, tag="5")]
    pub response: ::core::option::Option<CaseResponse>,
    /// The reason for resolving the complaint. It provides adjustment values while
    /// resolving and for already resolved complaints. This maps adjustment flag
    /// and reason code for the complaint to `reqAdjFlag` and `reqAdjCode` in
    /// complaint request respectively when a complete resolution is done via
    /// Resolve Complaint API otherwise maps to `respAdjFlag` and `respAdjCode` in
    /// complaint response respectively when a complaint request from UPI is
    /// directly resolved by issuer switch.
    #[prost(message, optional, tag="6")]
    pub resolve_complaint_adjustment: ::core::option::Option<ResolveComplaintAdjustment>,
}
/// Request for the `CreateComplaint` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateComplaintRequest {
    /// Required. The parent resource for the complaint. The format is
    /// `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The complaint to be raised.
    #[prost(message, optional, tag="2")]
    pub complaint: ::core::option::Option<Complaint>,
}
/// Request for the `ResolveComplaint` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveComplaintRequest {
    /// Required. The complaint to be resolved.
    #[prost(message, optional, tag="1")]
    pub complaint: ::core::option::Option<Complaint>,
}
/// A dispute processed by the issuer switch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dispute {
    /// The name of the dispute. This uniquely identifies the dispute.
    /// Format of name is
    /// projects/{project_id}/disputes/{dispute_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The reason for raising the dispute. This maps adjustment flag
    /// and reason code for the dispute to `reqAdjFlag` and `reqAdjCode` in
    /// complaint request respectively while raising a dispute.
    #[prost(message, optional, tag="2")]
    pub raise_dispute_adjustment: ::core::option::Option<RaiseDisputeAdjustment>,
    /// Required. Details required for raising/resolving dispute.
    #[prost(message, optional, tag="4")]
    pub details: ::core::option::Option<CaseDetails>,
    /// Output only. Response to the raised/resolved dispute.
    #[prost(message, optional, tag="5")]
    pub response: ::core::option::Option<CaseResponse>,
    /// The reason for resolving the dispute. It provides adjustment values while
    /// resolving and for already resolved disputes. This maps adjustment flag
    /// and reason code for the dispute to `reqAdjFlag` and `reqAdjCode` in
    /// dispute request respectively while resolving a dispute.
    #[prost(message, optional, tag="6")]
    pub resolve_dispute_adjustment: ::core::option::Option<ResolveDisputeAdjustment>,
}
/// Request for the `CreateDispute` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDisputeRequest {
    /// Required. The parent resource for the dispute. The format is
    /// `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The dispute to be raised.
    #[prost(message, optional, tag="2")]
    pub dispute: ::core::option::Option<Dispute>,
}
/// Request for the `ResolveDispute` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveDisputeRequest {
    /// Required. The dispute to be resolved.
    #[prost(message, optional, tag="1")]
    pub dispute: ::core::option::Option<Dispute>,
}
/// Details of original transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OriginalTransaction {
    /// Required. Uniquely identifies the original transaction. This maps to the `Txn.Id`
    /// value of the original transaction in India's UPI system.
    #[prost(string, tag="1")]
    pub transaction_id: ::prost::alloc::string::String,
    /// Required. Retrieval Reference Number (RRN) of the original transaction.
    #[prost(string, tag="2")]
    pub retrieval_reference_number: ::prost::alloc::string::String,
    /// Timestamp of the original transaction request.
    #[prost(message, optional, tag="3")]
    pub request_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Details of the complaint or dispute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaseDetails {
    /// Required. Details of original transaction.
    #[prost(message, optional, tag="1")]
    pub original_transaction: ::core::option::Option<OriginalTransaction>,
    /// Required. Initiator of the complaint / dispute.
    #[prost(enumeration="TransactionSubType", tag="2")]
    pub transaction_sub_type: i32,
    /// Required. The adjustment amount in URCS for the complaint / dispute. This
    /// maps to `reqAdjAmount` in complaint request.
    #[prost(message, optional, tag="3")]
    pub amount: ::core::option::Option<super::super::super::super::r#type::Money>,
    /// The original response code which has been updated in the complaint
    /// Response. This should map to settlement response code currently available
    /// in URCS system.
    #[prost(string, tag="4")]
    pub original_settlement_response_code: ::prost::alloc::string::String,
    /// Required. Set to true if the complaint / dispute belongs to current settlement cycle,
    /// false otherwise.
    #[prost(bool, tag="5")]
    pub current_cycle: bool,
}
/// Response to the complaint or dispute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaseResponse {
    /// Complaint Reference Number(CRN) sent by UPI as a reference against the
    /// generated complaint / dispute.
    #[prost(string, tag="1")]
    pub complaint_reference_number: ::prost::alloc::string::String,
    /// The adjustment amount of the response. This maps to `adjAmt` in
    /// complaint response.
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::super::super::super::r#type::Money>,
    /// The adjustment flag in response to the complaint. This maps adjustment flag
    /// in URCS for the complaint transaction to `Resp.Ref.adjFlag` in complaint
    /// response.
    #[prost(string, tag="3")]
    pub adjustment_flag: ::prost::alloc::string::String,
    /// The adjustment code in response to the complaint. This maps reason code in
    /// URCS for the complaint transaction to `Resp.Ref.adjCode` in complaint
    /// response.
    #[prost(string, tag="4")]
    pub adjustment_code: ::prost::alloc::string::String,
    /// It defines the Adjustment Reference ID which has been updated in the
    /// complaint response. This maps to `adjRefID` in complaint response.
    #[prost(string, tag="5")]
    pub adjustment_reference_id: ::prost::alloc::string::String,
    /// Adjustment Remarks. This maps to `adjRemarks` in complaint response.
    #[prost(string, tag="6")]
    pub adjustment_remarks: ::prost::alloc::string::String,
    /// The Approval Reference Number. This maps to `approvalNum` in complaint
    /// response.
    #[prost(string, tag="7")]
    pub approval_number: ::prost::alloc::string::String,
    /// Process Status of the transaction. This maps to `procStatus` in complaint
    /// response.
    #[prost(string, tag="8")]
    pub process_status: ::prost::alloc::string::String,
    /// The adjustment timestamp when bank performs the adjustment for the received
    /// complaint request. This maps to `adjTs` in complaint response.
    #[prost(message, optional, tag="9")]
    pub adjustment_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The details of the participant of the original financial transaction.
    #[prost(oneof="case_response::Participant", tags="10, 11")]
    pub participant: ::core::option::Option<case_response::Participant>,
}
/// Nested message and enum types in `CaseResponse`.
pub mod case_response {
    /// The details of the participant of the original financial transaction.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Participant {
        /// The payer in the original financial transaction.
        #[prost(message, tag="10")]
        Payer(super::SettlementParticipant),
        /// The payee in the original financial transaction.
        #[prost(message, tag="11")]
        Payee(super::SettlementParticipant),
    }
}
/// The adjusment flag and reason code for raising complaint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaiseComplaintAdjustment {
    /// Required. The adjustment flag in URCS for the complaint transaction. This maps to
    /// `reqAdjFlag` in complaint request and `respAdjFlag` in complaint response.
    #[prost(enumeration="raise_complaint_adjustment::AdjustmentFlag", tag="1")]
    pub adjustment_flag: i32,
    /// Required. The adjustment code in URCS for the complaint transaction. This maps to
    /// `reqAdjCode` in complaint request.
    #[prost(enumeration="raise_complaint_adjustment::ReasonCode", tag="2")]
    pub adjustment_code: i32,
}
/// Nested message and enum types in `RaiseComplaintAdjustment`.
pub mod raise_complaint_adjustment {
    /// The adjusment flag for raising complaint.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdjustmentFlag {
        /// Unspecified adjustment flag.
        Unspecified = 0,
        /// Complaint Raise. This flag maps to the `PBRB` adjustment flag as defined
        /// in NPCI's `UDIR` specification.
        Raise = 1,
    }
    impl AdjustmentFlag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdjustmentFlag::Unspecified => "ADJUSTMENT_FLAG_UNSPECIFIED",
                AdjustmentFlag::Raise => "RAISE",
            }
        }
    }
    /// The reason for raising complaint.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReasonCode {
        /// Unspecified reason code.
        Unspecified = 0,
        /// Customer account has not yet reversed for a declined pay transaction.
        /// This reason code maps to the `U005` reason code as defined in NPCI's
        /// `UDIR` specification.
        CustomerAccountNotReversed = 1,
        /// Goods / services are not provided for approved transaction.
        /// This reason code maps to the `U008` reason code as defined in NPCI's
        /// `UDIR` specification.
        GoodsServicesNotProvided = 2,
        /// Customer account not credited back for declined transaction. This
        /// reason code maps to the `U009` reason code as defined in NPCI's `UDIR`
        /// specification.
        CustomerAccountNotCreditedBack = 3,
        /// Beneficiary account is not credited for successful pay transaction. This
        /// reason code maps to the `U010` reason code as defined in NPCI's `UDIR`
        /// specification.
        BeneficiaryAccountNotCredited = 4,
        /// Credit not processed for cancelled or returned goods and services.
        /// This reason code maps to the `U021` reason code as defined in NPCI's
        /// `UDIR` specification.
        GoodsServicesCreditNotProcessed = 5,
        /// Account debited but transaction confirmation not received at merchant
        /// location. This reason code maps to the `U022` reason code as defined in
        /// NPCI's `UDIR` specification.
        MerchantNotReceivedConfirmation = 6,
        /// Paid by alternate means / Duplicate payment. This reason code maps to the
        /// `U023` reason code as defined in NPCI's `UDIR` specification.
        PaidByAlternateMeans = 7,
    }
    impl ReasonCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReasonCode::Unspecified => "REASON_CODE_UNSPECIFIED",
                ReasonCode::CustomerAccountNotReversed => "CUSTOMER_ACCOUNT_NOT_REVERSED",
                ReasonCode::GoodsServicesNotProvided => "GOODS_SERVICES_NOT_PROVIDED",
                ReasonCode::CustomerAccountNotCreditedBack => "CUSTOMER_ACCOUNT_NOT_CREDITED_BACK",
                ReasonCode::BeneficiaryAccountNotCredited => "BENEFICIARY_ACCOUNT_NOT_CREDITED",
                ReasonCode::GoodsServicesCreditNotProcessed => "GOODS_SERVICES_CREDIT_NOT_PROCESSED",
                ReasonCode::MerchantNotReceivedConfirmation => "MERCHANT_NOT_RECEIVED_CONFIRMATION",
                ReasonCode::PaidByAlternateMeans => "PAID_BY_ALTERNATE_MEANS",
            }
        }
    }
}
/// The adjusment flag and reason code for resolving the complaint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveComplaintAdjustment {
    /// Required. The adjustment flag in URCS for the complaint transaction. This maps to
    /// `reqAdjFlag` in complaint request and `respAdjFlag` in complaint response.
    #[prost(enumeration="resolve_complaint_adjustment::AdjustmentFlag", tag="1")]
    pub adjustment_flag: i32,
    /// Required. The adjustment code in URCS for the complaint transaction. This maps to
    /// `reqAdjCode` in complaint request.
    #[prost(enumeration="resolve_complaint_adjustment::ReasonCode", tag="2")]
    pub adjustment_code: i32,
}
/// Nested message and enum types in `ResolveComplaintAdjustment`.
pub mod resolve_complaint_adjustment {
    /// The adjusment flag for resolving the complaint.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdjustmentFlag {
        /// Unspecified adjustment flag.
        Unspecified = 0,
        /// Debit Reversal Confirmation. This flag maps to the `DRC` adjustment flag
        /// as defined in NPCI's `UDIR` specification.
        DebitReversalConfirmation = 1,
        /// Return. This flag maps to the `RET` adjustment flag as defined in NPCI's
        /// `UDIR` specification.
        Return = 2,
        /// Refund Reversal Confirmation. This flag maps to the `RRC` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        RefundReversalConfirmation = 3,
        /// Transaction Credit Confirmation. This flag maps to the `TCC` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        TransactionCreditConfirmation = 4,
    }
    impl AdjustmentFlag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdjustmentFlag::Unspecified => "ADJUSTMENT_FLAG_UNSPECIFIED",
                AdjustmentFlag::DebitReversalConfirmation => "DEBIT_REVERSAL_CONFIRMATION",
                AdjustmentFlag::Return => "RETURN",
                AdjustmentFlag::RefundReversalConfirmation => "REFUND_REVERSAL_CONFIRMATION",
                AdjustmentFlag::TransactionCreditConfirmation => "TRANSACTION_CREDIT_CONFIRMATION",
            }
        }
    }
    /// The complaint resolution reason code.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReasonCode {
        /// Unspecified reason code.
        Unspecified = 0,
        /// Customer account has been reversed online for DRC dispute or beneficiary
        /// account has been credited online for TCC dispute. This reason code maps
        /// to the `102` reason code as defined in NPCI's `UDIR` specification.
        ComplaintResolvedOnline = 1,
        /// Customer account has been reversed now or manually post reconciliation
        /// for DRC dispute or beneficiary account has been credited now or manually
        /// post reconciliation for TCC dispute. This reason code maps to the `103`
        /// reason code as defined in NPCI's `UDIR` specification.
        ComplaintResolvedNowOrManually = 2,
        /// Online decline response failed. This reason code maps to the
        /// `104` reason code as defined in NPCI's `UDIR` specification.
        OriginalTransactionNotDone = 3,
        /// Account closed. This reason code maps to the `114` reason code for
        /// RET dispute as defined in NPCI's `UDIR` specification.
        RetAccountClosed = 4,
        /// Account does not exist. This reason code maps to the `115` reason code
        /// for RET dispute as defined in NPCI's `UDIR` specification.
        RetAccountDoesNotExist = 5,
        /// Party instructions. This reason code maps to the `116` reason code for
        /// RET dispute as defined in NPCI's `UDIR` specification.
        RetPartyInstructions = 6,
        /// NRI account. This reason code maps to the `117` reason code for RET
        /// dispute as defined in NPCI's `UDIR` specification.
        RetNriAccount = 7,
        /// Credit freezed. This reason code maps to the `118` reason code for RET
        /// dispute as defined in NPCI's `UDIR` specification.
        RetCreditFreezed = 8,
        /// Invalid beneficiary details. This reason code maps to the `119` reason
        /// code for RET dispute as defined in NPCI's `UDIR` specification.
        RetInvalidBeneficiaryDetails = 9,
        /// Any other reason. This reason code maps to the `120` reason code for RET
        /// dispute as defined in NPCI's `UDIR` specification.
        RetAnyOtherReason = 10,
        /// Beneficiary bank unable to credit their customer account.
        /// This reason code maps to the `1094` reason code for RET dispute as
        /// defined in NPCI's `UDIR` specification.
        RetBeneficiaryCannotCredit = 11,
        /// Account debited but transaction confirmation not received at merchant
        /// location. This reason code maps to the `1065` reason code for Credit
        /// adjustment and RET dispute as defined in NPCI's `UDIR` specification.
        RetMerchantNotReceivedConfirmation = 12,
        /// Customer account has been credited. This reason code maps to the `501`
        /// reason code for Refund reversal confirmation dispute as defined in NPCI's
        /// `UDIR` specification.
        RrcCustomerAccountCredited = 13,
    }
    impl ReasonCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReasonCode::Unspecified => "REASON_CODE_UNSPECIFIED",
                ReasonCode::ComplaintResolvedOnline => "COMPLAINT_RESOLVED_ONLINE",
                ReasonCode::ComplaintResolvedNowOrManually => "COMPLAINT_RESOLVED_NOW_OR_MANUALLY",
                ReasonCode::OriginalTransactionNotDone => "ORIGINAL_TRANSACTION_NOT_DONE",
                ReasonCode::RetAccountClosed => "RET_ACCOUNT_CLOSED",
                ReasonCode::RetAccountDoesNotExist => "RET_ACCOUNT_DOES_NOT_EXIST",
                ReasonCode::RetPartyInstructions => "RET_PARTY_INSTRUCTIONS",
                ReasonCode::RetNriAccount => "RET_NRI_ACCOUNT",
                ReasonCode::RetCreditFreezed => "RET_CREDIT_FREEZED",
                ReasonCode::RetInvalidBeneficiaryDetails => "RET_INVALID_BENEFICIARY_DETAILS",
                ReasonCode::RetAnyOtherReason => "RET_ANY_OTHER_REASON",
                ReasonCode::RetBeneficiaryCannotCredit => "RET_BENEFICIARY_CANNOT_CREDIT",
                ReasonCode::RetMerchantNotReceivedConfirmation => "RET_MERCHANT_NOT_RECEIVED_CONFIRMATION",
                ReasonCode::RrcCustomerAccountCredited => "RRC_CUSTOMER_ACCOUNT_CREDITED",
            }
        }
    }
}
/// The adjusment flag and reason code for raising dispute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaiseDisputeAdjustment {
    /// Required. The adjustment flag in URCS for the complaint transaction. This maps to
    /// `reqAdjFlag` in dispute request and `respAdjFlag` in dispute response.
    #[prost(enumeration="raise_dispute_adjustment::AdjustmentFlag", tag="1")]
    pub adjustment_flag: i32,
    /// Required. The adjustment code in URCS for the complaint transaction. This maps to
    /// `reqAdjCode` in dispute request.
    #[prost(enumeration="raise_dispute_adjustment::ReasonCode", tag="2")]
    pub adjustment_code: i32,
}
/// Nested message and enum types in `RaiseDisputeAdjustment`.
pub mod raise_dispute_adjustment {
    /// The adjusment flag for raising dispute.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdjustmentFlag {
        /// Unspecified adjustment flag.
        Unspecified = 0,
        /// Chargeback Raise. This flag maps to the `B` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ChargebackRaise = 1,
        /// Fraud Chargeback Raise. This flag maps to the `FC` adjustment flag
        /// as defined in NPCI's `UDIR` specification.
        FraudChargebackRaise = 2,
        /// Wrong Credit Chargeback Raise. This flag maps to the `WC` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        WrongCreditChargebackRaise = 3,
        /// Deferred Chargeback Raise. This flag maps to the `FB` adjustment flag
        /// as defined in NPCI's `UDIR` specification.
        DeferredChargebackRaise = 4,
        /// Pre-Arbitration Raise. This flag maps to the `P` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        PreArbitrationRaise = 5,
        /// Deferred Pre-Arbitration Raise. This flag maps to the `FP` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        DeferredPreArbitrationRaise = 6,
        /// Arbitration Raise. This flag maps to the `AR` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ArbitrationRaise = 7,
        /// Deferred Arbitration Raise. This flag maps to the `FAR` adjustment flag
        /// as defined in NPCI's `UDIR` specification.
        DeferredArbitrationRaise = 8,
    }
    impl AdjustmentFlag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdjustmentFlag::Unspecified => "ADJUSTMENT_FLAG_UNSPECIFIED",
                AdjustmentFlag::ChargebackRaise => "CHARGEBACK_RAISE",
                AdjustmentFlag::FraudChargebackRaise => "FRAUD_CHARGEBACK_RAISE",
                AdjustmentFlag::WrongCreditChargebackRaise => "WRONG_CREDIT_CHARGEBACK_RAISE",
                AdjustmentFlag::DeferredChargebackRaise => "DEFERRED_CHARGEBACK_RAISE",
                AdjustmentFlag::PreArbitrationRaise => "PRE_ARBITRATION_RAISE",
                AdjustmentFlag::DeferredPreArbitrationRaise => "DEFERRED_PRE_ARBITRATION_RAISE",
                AdjustmentFlag::ArbitrationRaise => "ARBITRATION_RAISE",
                AdjustmentFlag::DeferredArbitrationRaise => "DEFERRED_ARBITRATION_RAISE",
            }
        }
    }
    /// The reason for raising dispute.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReasonCode {
        /// Unspecified reason code.
        Unspecified = 0,
        /// Remitter account is debited but beneficiary account is not credited.
        /// This reason code maps to the `108` reason code as defined in
        /// NPCI's `UDIR` specification.
        ChargebackRaiseRemitterDebitedBeneficiaryNotCredited = 1,
        /// Remitter bank customer still disputes that beneficiary account is not
        /// credited. This reason code maps to the `109` reason code as defined in
        /// NPCI's `UDIR` specification.
        PreArbitrationRaiseBeneficiaryNotCredited = 2,
        /// TCC has been raised but customer still complaining that beneficiary
        /// account is not credited. This reason code maps to the `121` reason code
        /// as defined in NPCI's `UDIR` specification.
        DeferredChargebackRaiseBeneficiaryNotCredited = 3,
        /// Customer is still complaining for not crediting the beneficiary
        /// customer account. This reason code maps to the `124` reason code as
        /// defined in NPCI's `UDIR` specification.
        DeferredPreArbitrationRaiseBeneficiaryNotCredited = 4,
        /// Customer is complaining even after raising Deferred Chargeback and
        /// Pre-Arbitration on Deferred Chargeback where both have been rejected by
        /// beneficiary bank. This reason code maps to the `127` reason code as
        /// defined in NPCI's `UDIR` specification.
        DeferredArbitrationRaiseDeferredChargebackPreArbitrationRejected = 5,
        /// Chargeback on fraudulent transaction. This reason code maps to the `128`
        /// reason code as defined in NPCI's `UDIR` specification.
        ChargebackOnFraud = 6,
        /// Credit not processed for cancelled or returned goods and services. This
        /// reason code maps to the `1061` reason code as defined in NPCI's `UDIR`
        /// specification.
        GoodsServicesCreditNotProcessed = 7,
        /// Goods and services not as described / defective. This reason code maps to
        /// the `1062` reason code as defined in NPCI's `UDIR` specification.
        GoodsServicesDefective = 8,
        /// Paid by alternate means. This reason code maps to the `1063` reason code
        /// as defined in NPCI's `UDIR` specification.
        PaidByAlternateMeans = 9,
        /// Goods or services not provided / not received. This reason code maps to
        /// the `1064` reason code as defined in NPCI's `UDIR` specification.
        GoodsServicesNotReceived = 10,
        /// Account debited but transaction confirmation not received at merchant
        /// location. This reason code maps to the `1065` reason code for chargeback
        /// raise and deferred chargeback raise as defined in NPCI's `UDIR`
        /// specification.
        MerchantNotReceivedConfirmation = 11,
        /// Transaction not steeled within the specified timeframes. This reason code
        /// maps to the `1081` reason code as defined in NPCI's `UDIR` specification.
        TransactionNotSteeled = 12,
        /// Duplicate / Multiple transaction. This reason code maps to the `1084`
        /// reason code as defined in NPCI's `UDIR` specification.
        DuplicateTransaction = 13,
        /// Card holder was charged more than the transaction amount.
        /// This reason code maps to the `1085` reason code for Chargeback raise
        /// dispute as defined in NPCI's `UDIR` specification.
        ChargebackCardHolderChargedMore = 14,
        /// Customer is still claiming that services are not delivered. This reason
        /// code maps to the `1097` reason code as defined in NPCI's `UDIR`
        /// specification.
        CustomerClaimingGoodsServicesNotDelivered = 15,
        /// Both the parties denied to agree. This reason code maps to the `1100`
        /// reason code as defined in NPCI's `UDIR` specification.
        PartiesDenied = 16,
        /// Customer transferred funds to the unintended beneficiary account. This
        /// reason code maps to the `WC1` reason code as defined in NPCI's `UDIR`
        /// specification.
        FundsTransferredToUnintendedBeneficiary = 17,
    }
    impl ReasonCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReasonCode::Unspecified => "REASON_CODE_UNSPECIFIED",
                ReasonCode::ChargebackRaiseRemitterDebitedBeneficiaryNotCredited => "CHARGEBACK_RAISE_REMITTER_DEBITED_BENEFICIARY_NOT_CREDITED",
                ReasonCode::PreArbitrationRaiseBeneficiaryNotCredited => "PRE_ARBITRATION_RAISE_BENEFICIARY_NOT_CREDITED",
                ReasonCode::DeferredChargebackRaiseBeneficiaryNotCredited => "DEFERRED_CHARGEBACK_RAISE_BENEFICIARY_NOT_CREDITED",
                ReasonCode::DeferredPreArbitrationRaiseBeneficiaryNotCredited => "DEFERRED_PRE_ARBITRATION_RAISE_BENEFICIARY_NOT_CREDITED",
                ReasonCode::DeferredArbitrationRaiseDeferredChargebackPreArbitrationRejected => "DEFERRED_ARBITRATION_RAISE_DEFERRED_CHARGEBACK_PRE_ARBITRATION_REJECTED",
                ReasonCode::ChargebackOnFraud => "CHARGEBACK_ON_FRAUD",
                ReasonCode::GoodsServicesCreditNotProcessed => "GOODS_SERVICES_CREDIT_NOT_PROCESSED",
                ReasonCode::GoodsServicesDefective => "GOODS_SERVICES_DEFECTIVE",
                ReasonCode::PaidByAlternateMeans => "PAID_BY_ALTERNATE_MEANS",
                ReasonCode::GoodsServicesNotReceived => "GOODS_SERVICES_NOT_RECEIVED",
                ReasonCode::MerchantNotReceivedConfirmation => "MERCHANT_NOT_RECEIVED_CONFIRMATION",
                ReasonCode::TransactionNotSteeled => "TRANSACTION_NOT_STEELED",
                ReasonCode::DuplicateTransaction => "DUPLICATE_TRANSACTION",
                ReasonCode::ChargebackCardHolderChargedMore => "CHARGEBACK_CARD_HOLDER_CHARGED_MORE",
                ReasonCode::CustomerClaimingGoodsServicesNotDelivered => "CUSTOMER_CLAIMING_GOODS_SERVICES_NOT_DELIVERED",
                ReasonCode::PartiesDenied => "PARTIES_DENIED",
                ReasonCode::FundsTransferredToUnintendedBeneficiary => "FUNDS_TRANSFERRED_TO_UNINTENDED_BENEFICIARY",
            }
        }
    }
}
/// The adjusment flag and reason code for resolving the dispute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveDisputeAdjustment {
    /// Required. The adjustment flag in URCS for the complaint transaction. This maps to
    /// `reqAdjFlag` in dispute request and `respAdjFlag` in dispute response.
    #[prost(enumeration="resolve_dispute_adjustment::AdjustmentFlag", tag="1")]
    pub adjustment_flag: i32,
    /// Required. The adjustment code in URCS for the complaint transaction. This maps to
    /// `reqAdjCode` in dispute request.
    #[prost(enumeration="resolve_dispute_adjustment::ReasonCode", tag="2")]
    pub adjustment_code: i32,
}
/// Nested message and enum types in `ResolveDisputeAdjustment`.
pub mod resolve_dispute_adjustment {
    /// The adjusment flag for resolving the dispute.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdjustmentFlag {
        /// Unspecified adjustment flag.
        Unspecified = 0,
        /// Re-presentment Raise. This flag maps to the `R` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        RePresentmentRaise = 1,
        /// Deferred Re-presentment Raise. This flag maps to the `FR` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        DeferredRePresentmentRaise = 2,
        /// Chargeback Acceptance. This flag maps to the `A` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ChargebackAcceptance = 3,
        /// Deferred Chargeback Acceptance. This flag maps to the `FA` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        DeferredChargebackAcceptance = 4,
        /// Pre-Arbitration Acceptance. This flag maps to the `AP` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        PreArbitrationAcceptance = 5,
        /// Deferred Pre-Arbitration Acceptance. This flag maps to the `FAP`
        /// adjustment flag as defined in NPCI's `UDIR` specification.
        DeferredPreArbitrationAcceptance = 6,
        /// Pre-Arbitration Declined. This flag maps to the `PR` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        PreArbitrationDeclined = 7,
        /// Deferred Pre-Arbitration Declined. This flag maps to the `FPR` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        DeferredPreArbitrationDeclined = 8,
        /// Arbitration Acceptance. This flag maps to the `ACA` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ArbitrationAcceptance = 9,
        /// Arbitration Continuation. This flag maps to the `ACC` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ArbitrationContinuation = 10,
        /// Arbitration Withdrawn. This flag maps to the `ACW` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ArbitrationWithdrawn = 11,
        /// Arbitration Verdict. This flag maps to the `ACV` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        ArbitrationVerdict = 12,
        /// Credit Adjustment. This flag maps to the `C` adjustment flag as
        /// defined in NPCI's `UDIR` specification.
        CreditAdjustment = 13,
        /// Fraud Chargeback Representment. This flag maps to the `FCR` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        FraudChargebackRepresentment = 14,
        /// Fraud Chargeback Accept. This flag maps to the `FCA` adjustment flag
        /// as defined in NPCI's `UDIR` specification.
        FraudChargebackAccept = 15,
        /// Wrong Credit Representment. This flag maps to the `WR` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        WrongCreditRepresentment = 16,
        /// Wrong Credit Chargeback Acceptance. This flag maps to the `WA` adjustment
        /// flag as defined in NPCI's `UDIR` specification.
        WrongCreditChargebackAcceptance = 17,
        /// Manual Adjustment. This flag maps to the `MA` adjustment flag as defined
        /// in NPCI's `UDIR` specification.
        ManualAdjustment = 18,
    }
    impl AdjustmentFlag {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdjustmentFlag::Unspecified => "ADJUSTMENT_FLAG_UNSPECIFIED",
                AdjustmentFlag::RePresentmentRaise => "RE_PRESENTMENT_RAISE",
                AdjustmentFlag::DeferredRePresentmentRaise => "DEFERRED_RE_PRESENTMENT_RAISE",
                AdjustmentFlag::ChargebackAcceptance => "CHARGEBACK_ACCEPTANCE",
                AdjustmentFlag::DeferredChargebackAcceptance => "DEFERRED_CHARGEBACK_ACCEPTANCE",
                AdjustmentFlag::PreArbitrationAcceptance => "PRE_ARBITRATION_ACCEPTANCE",
                AdjustmentFlag::DeferredPreArbitrationAcceptance => "DEFERRED_PRE_ARBITRATION_ACCEPTANCE",
                AdjustmentFlag::PreArbitrationDeclined => "PRE_ARBITRATION_DECLINED",
                AdjustmentFlag::DeferredPreArbitrationDeclined => "DEFERRED_PRE_ARBITRATION_DECLINED",
                AdjustmentFlag::ArbitrationAcceptance => "ARBITRATION_ACCEPTANCE",
                AdjustmentFlag::ArbitrationContinuation => "ARBITRATION_CONTINUATION",
                AdjustmentFlag::ArbitrationWithdrawn => "ARBITRATION_WITHDRAWN",
                AdjustmentFlag::ArbitrationVerdict => "ARBITRATION_VERDICT",
                AdjustmentFlag::CreditAdjustment => "CREDIT_ADJUSTMENT",
                AdjustmentFlag::FraudChargebackRepresentment => "FRAUD_CHARGEBACK_REPRESENTMENT",
                AdjustmentFlag::FraudChargebackAccept => "FRAUD_CHARGEBACK_ACCEPT",
                AdjustmentFlag::WrongCreditRepresentment => "WRONG_CREDIT_REPRESENTMENT",
                AdjustmentFlag::WrongCreditChargebackAcceptance => "WRONG_CREDIT_CHARGEBACK_ACCEPTANCE",
                AdjustmentFlag::ManualAdjustment => "MANUAL_ADJUSTMENT",
            }
        }
    }
    /// The dispute resolution reason code.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReasonCode {
        /// Unspecified reason code.
        Unspecified = 0,
        /// Beneficiary bank unable to credit their customer account for Chargeback
        /// Acceptance dispute or duplicate processing for Pre Arbitration Acceptance
        /// dispute. This reason code maps to the `111` reason code as defined in
        /// NPCI's `UDIR` specification.
        ChargebackBeneficiaryCannotCreditOrPreArbitrationDuplicateProcess = 1,
        /// Beneficiary account has been credited online. This reason code maps to
        /// the `112` reason code for Pre-arbitration declined dispute as defined in
        /// NPCI's `UDIR` specification.
        PreArbitrationDeclinedBeneficiaryCreditedOnline = 3,
        /// Beneficiary account has been credited manually post reconciliation. This
        /// reason code maps to the `113` reason code for Pre-arbitration declined
        /// dispute as defined in NPCI's `UDIR` specification.
        PreArbitrationDeclinedBeneficiaryCreditedManually = 4,
        /// Customer account is not credited, TCC raised inadvertently. This reason
        /// code maps to the `122` reason code as defined in NPCI's `UDIR`
        /// specification.
        DeferredChargebackAcceptanceAccountNotCreditedTccRaised = 5,
        /// Customer account is credited successfully and TCC raised accordingly.
        /// This reason code maps to the `123` reason code as defined in NPCI's
        /// `UDIR` specification.
        DeferredRePresentmentRaiseAccountCreditedTccRaised = 6,
        /// Customer account is not credited, TCC and Re-Presentment raised
        /// inadvertently. This reason code maps to the `125` reason code as defined
        /// in NPCI's `UDIR` specification.
        DeferredPreArbitrationAcceptanceAccountNotCredited = 7,
        /// Customer account is credited successfully and TCC and Re-Presentment
        /// raised accordingly. This reason code maps to the `126` reason code as
        /// defined in NPCI's `UDIR` specification.
        DeferredPreArbitrationDeclinedAccountCredited = 8,
        /// Amount has been recovered successfully from the fraudulent customer
        /// account. This reason code maps to the `129` reason code as defined
        /// in NPCI's `UDIR` specification.
        FraudChargebackAcceptAmountRecoveredFromFraudulentAccount = 9,
        /// Lien marked however, customer account is not having sufficient balance to
        /// debit. This reason code maps to the `130` reason code for
        /// Fraud chargeback representment dispute as defined in NPCI's `UDIR`
        /// specification.
        FraudChargebackRepresentmentLienMarkedInsufficientBalance = 10,
        /// FIR Copy not provided for the disputed transaction. This reason code maps
        /// to the `131` reason code as defined in NPCI's `UDIR` specification.
        FraudChargebackRepresentmentFirNotProvided = 11,
        /// Other reason for Fraud chargeback representment dispute. This reason code
        /// maps to the `132` reason code as defined in NPCI's `UDIR` specification.
        FraudChargebackRepresentmentReasonOthers = 12,
        /// Beneficiary account credited online. This reason code maps to the `208`
        /// reason code for Re-presentment raise dispute as defined in NPCI's `UDIR`
        /// specification.
        RePresentmentRaiseBeneficiaryCreditedOnline = 13,
        /// Beneficiary account credited manually post reconciliation. This reason
        /// code maps to the `209` reason code for Re-presentment raise dispute as
        /// defined in NPCI's `UDIR` specification.
        RePresentmentRaiseBeneficiaryCreditedManually = 14,
        /// Credit not processed for cancelled or returned goods and services. This
        /// reason code maps to the `1061` reason code as defined in NPCI's `UDIR`
        /// specification.
        CreditAdjustmentGoodsServicesCreditNotProcessed = 15,
        /// Goods and Services not as described / defective. This reason code maps to
        /// the `1062` reason code as defined in NPCI's `UDIR` specification.
        CreditAdjustmentGoodsServicesDefective = 16,
        /// Paid by alternate means. This reason code maps to the `1063` reason code
        /// as defined in NPCI's `UDIR` specification.
        CreditAdjustmentPaidByAlternateMeans = 17,
        /// Goods or Services Not Provided / Not Received. This reason code maps to
        /// the `1064` reason code as defined in NPCI's `UDIR` specification.
        CreditAdjustmentGoodsServicesNotReceived = 18,
        /// Account debited but transaction confirmation not received at merchant
        /// location. This reason code maps to the `1065` reason code for Credit
        /// adjustment as defined in NPCI's `UDIR` specification.
        CreditAdjustmentMerchantNotReceivedConfirmation = 19,
        /// Duplicate /Multiple Transaction. This reason code maps to the `1084`
        /// reason code as defined in NPCI's `UDIR` specification.
        CreditAdjustmentDuplicateTransaction = 20,
        /// Other reason for Credit adjustment. This reason code maps to the `1090`
        /// reason code as defined in NPCI's `UDIR` specification.
        CreditAdjustmentReasonOthers = 21,
        /// Non Matching account number. This reason code maps to the `1091`
        /// reason code as defined in NPCI's `UDIR` specification.
        CreditAdjustmentNonMatchingAccountNumber = 22,
        /// Card holder was charged more than the transaction amount.
        /// This reason code maps to the `1092` reason code as defined in NPCI's
        /// `UDIR` specification.
        CreditAdjustmentCardHolderChargedMore = 23,
        /// Credit not Processed. This reason code maps to the `1093` reason code as
        /// defined in NPCI's `UDIR` specification.
        CreditAdjustmentCreditNotProcessed = 24,
        /// Beneficiary bank unable to credit their customer account. This reason
        /// code maps to the `1094` reason code for Credit Adjustment dispute as
        /// defined in NPCI's `UDIR` specification.
        CreditAdjustmentBeneficiaryCannotCredit = 25,
        /// Merchant was unable to provide the service. This reason code maps to the
        /// `1095` reason code as defined in NPCI's `UDIR` specification.
        ChargebackAcceptanceMerchantCannotProvideService = 26,
        /// Services/Goods provided see the supporting document. This reason code
        /// maps to the `1096` reason code as defined in NPCI's `UDIR` specification.
        RePresentmentRaiseGoodsServicesProvided = 27,
        /// Services provided later see supporting documents. This reason code maps
        /// to the `1098` reason code as defined in NPCI's `UDIR` specification.
        PreArbitrationDeclinedServicesProvidedLater = 28,
        /// Services not provided by the merchant. This reason code maps to the
        /// `1099` reason code as defined in NPCI's `UDIR` specification.
        PreArbitrationAcceptanceServicesNotProvidedByMerchant = 29,
        /// Illegible Fulfilment. This reason code maps to the `1101` reason code for
        /// arbitration acceptance dispute as defined in NPCI's `UDIR` specification.
        ArbitrationAcceptanceIllegibleFulfilment = 30,
        /// Customer has still not received the service. This reason code maps to the
        /// `1102` reason code as defined in NPCI's `UDIR` specification.
        ArbitrationContinuationCustomerStillNotReceivedService = 31,
        /// Customer has received the service later. This reason code maps to the
        /// `1103` reason code as defined in NPCI's `UDIR` specification.
        ArbitrationWithdrawnCustomerReceivedServiceLater = 32,
        /// Panel will give the verdict. This reason code maps to the `1104` reason
        /// code as defined in NPCI's `UDIR` specification.
        ArbitrationVerdictPanelVerdict = 33,
        /// Manual adjustment. This reason code maps to the `2001` reason code as
        /// defined in NPCI's `UDIR` specification.
        ManualAdjustmentReason = 34,
        /// Attributing to the Customer. This reason code maps to the `AC` reason
        /// code as defined in NPCI's `UDIR` specification.
        AttributingCustomer = 35,
        /// Attributing to the Technical issue at bank/aggregator/merchant. This
        /// reason code maps to the `AT` reason code as defined in NPCI's `UDIR`
        /// specification.
        AttributingTechnicalIssue = 36,
        /// Amount has been recovered successfully from the unintended customer
        /// account. This reason code maps to the `WC2` reason code as defined in
        /// NPCI's `UDIR` specification.
        WrongCreditChargebackAcceptanceAmountRecovered = 37,
        /// Lien marked however customer account is not having sufficient balance to
        /// debit the customer account. This reason code maps to the `WC3` reason
        /// code for Wrong credit representment dispute as defined in NPCI's `UDIR`
        /// specification.
        WrongCreditRepresentmentLienMarkedInsufficientBalance = 38,
        /// Customer is not accessible for obtaining debit confirmation. This reason
        /// code maps to the `WC4` reason code as defined in NPCI's `UDIR`
        /// specification.
        WrongCreditRepresentmentCustomerInaccessible = 39,
        /// Other reason for Wrong credit representment. This reason code maps to the
        /// `WC5` reason code as defined in NPCI's `UDIR` specification.
        WrongCreditRepresentmentReasonOthers = 40,
    }
    impl ReasonCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReasonCode::Unspecified => "REASON_CODE_UNSPECIFIED",
                ReasonCode::ChargebackBeneficiaryCannotCreditOrPreArbitrationDuplicateProcess => "CHARGEBACK_BENEFICIARY_CANNOT_CREDIT_OR_PRE_ARBITRATION_DUPLICATE_PROCESS",
                ReasonCode::PreArbitrationDeclinedBeneficiaryCreditedOnline => "PRE_ARBITRATION_DECLINED_BENEFICIARY_CREDITED_ONLINE",
                ReasonCode::PreArbitrationDeclinedBeneficiaryCreditedManually => "PRE_ARBITRATION_DECLINED_BENEFICIARY_CREDITED_MANUALLY",
                ReasonCode::DeferredChargebackAcceptanceAccountNotCreditedTccRaised => "DEFERRED_CHARGEBACK_ACCEPTANCE_ACCOUNT_NOT_CREDITED_TCC_RAISED",
                ReasonCode::DeferredRePresentmentRaiseAccountCreditedTccRaised => "DEFERRED_RE_PRESENTMENT_RAISE_ACCOUNT_CREDITED_TCC_RAISED",
                ReasonCode::DeferredPreArbitrationAcceptanceAccountNotCredited => "DEFERRED_PRE_ARBITRATION_ACCEPTANCE_ACCOUNT_NOT_CREDITED",
                ReasonCode::DeferredPreArbitrationDeclinedAccountCredited => "DEFERRED_PRE_ARBITRATION_DECLINED_ACCOUNT_CREDITED",
                ReasonCode::FraudChargebackAcceptAmountRecoveredFromFraudulentAccount => "FRAUD_CHARGEBACK_ACCEPT_AMOUNT_RECOVERED_FROM_FRAUDULENT_ACCOUNT",
                ReasonCode::FraudChargebackRepresentmentLienMarkedInsufficientBalance => "FRAUD_CHARGEBACK_REPRESENTMENT_LIEN_MARKED_INSUFFICIENT_BALANCE",
                ReasonCode::FraudChargebackRepresentmentFirNotProvided => "FRAUD_CHARGEBACK_REPRESENTMENT_FIR_NOT_PROVIDED",
                ReasonCode::FraudChargebackRepresentmentReasonOthers => "FRAUD_CHARGEBACK_REPRESENTMENT_REASON_OTHERS",
                ReasonCode::RePresentmentRaiseBeneficiaryCreditedOnline => "RE_PRESENTMENT_RAISE_BENEFICIARY_CREDITED_ONLINE",
                ReasonCode::RePresentmentRaiseBeneficiaryCreditedManually => "RE_PRESENTMENT_RAISE_BENEFICIARY_CREDITED_MANUALLY",
                ReasonCode::CreditAdjustmentGoodsServicesCreditNotProcessed => "CREDIT_ADJUSTMENT_GOODS_SERVICES_CREDIT_NOT_PROCESSED",
                ReasonCode::CreditAdjustmentGoodsServicesDefective => "CREDIT_ADJUSTMENT_GOODS_SERVICES_DEFECTIVE",
                ReasonCode::CreditAdjustmentPaidByAlternateMeans => "CREDIT_ADJUSTMENT_PAID_BY_ALTERNATE_MEANS",
                ReasonCode::CreditAdjustmentGoodsServicesNotReceived => "CREDIT_ADJUSTMENT_GOODS_SERVICES_NOT_RECEIVED",
                ReasonCode::CreditAdjustmentMerchantNotReceivedConfirmation => "CREDIT_ADJUSTMENT_MERCHANT_NOT_RECEIVED_CONFIRMATION",
                ReasonCode::CreditAdjustmentDuplicateTransaction => "CREDIT_ADJUSTMENT_DUPLICATE_TRANSACTION",
                ReasonCode::CreditAdjustmentReasonOthers => "CREDIT_ADJUSTMENT_REASON_OTHERS",
                ReasonCode::CreditAdjustmentNonMatchingAccountNumber => "CREDIT_ADJUSTMENT_NON_MATCHING_ACCOUNT_NUMBER",
                ReasonCode::CreditAdjustmentCardHolderChargedMore => "CREDIT_ADJUSTMENT_CARD_HOLDER_CHARGED_MORE",
                ReasonCode::CreditAdjustmentCreditNotProcessed => "CREDIT_ADJUSTMENT_CREDIT_NOT_PROCESSED",
                ReasonCode::CreditAdjustmentBeneficiaryCannotCredit => "CREDIT_ADJUSTMENT_BENEFICIARY_CANNOT_CREDIT",
                ReasonCode::ChargebackAcceptanceMerchantCannotProvideService => "CHARGEBACK_ACCEPTANCE_MERCHANT_CANNOT_PROVIDE_SERVICE",
                ReasonCode::RePresentmentRaiseGoodsServicesProvided => "RE_PRESENTMENT_RAISE_GOODS_SERVICES_PROVIDED",
                ReasonCode::PreArbitrationDeclinedServicesProvidedLater => "PRE_ARBITRATION_DECLINED_SERVICES_PROVIDED_LATER",
                ReasonCode::PreArbitrationAcceptanceServicesNotProvidedByMerchant => "PRE_ARBITRATION_ACCEPTANCE_SERVICES_NOT_PROVIDED_BY_MERCHANT",
                ReasonCode::ArbitrationAcceptanceIllegibleFulfilment => "ARBITRATION_ACCEPTANCE_ILLEGIBLE_FULFILMENT",
                ReasonCode::ArbitrationContinuationCustomerStillNotReceivedService => "ARBITRATION_CONTINUATION_CUSTOMER_STILL_NOT_RECEIVED_SERVICE",
                ReasonCode::ArbitrationWithdrawnCustomerReceivedServiceLater => "ARBITRATION_WITHDRAWN_CUSTOMER_RECEIVED_SERVICE_LATER",
                ReasonCode::ArbitrationVerdictPanelVerdict => "ARBITRATION_VERDICT_PANEL_VERDICT",
                ReasonCode::ManualAdjustmentReason => "MANUAL_ADJUSTMENT_REASON",
                ReasonCode::AttributingCustomer => "ATTRIBUTING_CUSTOMER",
                ReasonCode::AttributingTechnicalIssue => "ATTRIBUTING_TECHNICAL_ISSUE",
                ReasonCode::WrongCreditChargebackAcceptanceAmountRecovered => "WRONG_CREDIT_CHARGEBACK_ACCEPTANCE_AMOUNT_RECOVERED",
                ReasonCode::WrongCreditRepresentmentLienMarkedInsufficientBalance => "WRONG_CREDIT_REPRESENTMENT_LIEN_MARKED_INSUFFICIENT_BALANCE",
                ReasonCode::WrongCreditRepresentmentCustomerInaccessible => "WRONG_CREDIT_REPRESENTMENT_CUSTOMER_INACCESSIBLE",
                ReasonCode::WrongCreditRepresentmentReasonOthers => "WRONG_CREDIT_REPRESENTMENT_REASON_OTHERS",
            }
        }
    }
}
/// Metadata for CreateComplaint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateComplaintMetadata {
}
/// Metadata for ResolveComplaint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveComplaintMetadata {
}
/// Metadata for CreateDispute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDisputeMetadata {
}
/// Metadata for ResolveDispute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveDisputeMetadata {
}
/// The subtype of the complaint or dispute.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionSubType {
    /// Unspecified transaction subtype.
    Unspecified = 0,
    /// Beneficiary transaction subtype.
    Beneficiary = 1,
    /// Remitter transaction subtype.
    Remitter = 2,
}
impl TransactionSubType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionSubType::Unspecified => "TRANSACTION_SUB_TYPE_UNSPECIFIED",
            TransactionSubType::Beneficiary => "TRANSACTION_SUB_TYPE_BENEFICIARY",
            TransactionSubType::Remitter => "TRANSACTION_SUB_TYPE_REMITTER",
        }
    }
}
/// Generated client implementations.
pub mod issuer_switch_resolutions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Creates and resolves UPI complaints and disputes.
    #[derive(Debug, Clone)]
    pub struct IssuerSwitchResolutionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IssuerSwitchResolutionsClient<tonic::transport::Channel> {
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
    impl<T> IssuerSwitchResolutionsClient<T>
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
        ) -> IssuerSwitchResolutionsClient<InterceptedService<T, F>>
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
            IssuerSwitchResolutionsClient::new(
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
        /// Create a complaint. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`: [CreateComplaintMetadata][google.cloud.paymentgateway.issuerswitch.v1.CreateComplaintMetadata]
        /// - `response`: [Complaint][google.cloud.paymentgateway.issuerswitch.v1.Complaint]
        pub async fn create_complaint(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateComplaintRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions/CreateComplaint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Resolve a complaint. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`: [ResolveComplaintMetadata][google.cloud.paymentgateway.issuerswitch.v1.ResolveComplaintMetadata]
        /// - `response`: [Complaint][google.cloud.paymentgateway.issuerswitch.v1.Complaint]
        pub async fn resolve_complaint(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveComplaintRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions/ResolveComplaint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create a dispute. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`: [CreateDisputeMetadata][google.cloud.paymentgateway.issuerswitch.v1.CreateDisputeMetadata]
        /// - `response`: [Dispute][google.cloud.paymentgateway.issuerswitch.v1.Dispute]
        pub async fn create_dispute(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDisputeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions/CreateDispute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Resolve a dispute. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`: [ResolveDisputeMetadata][google.cloud.paymentgateway.issuerswitch.v1.ResolveDisputeMetadata]
        /// - `response`: [Dispute][google.cloud.paymentgateway.issuerswitch.v1.Dispute]
        pub async fn resolve_dispute(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveDisputeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchResolutions/ResolveDispute",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A rule that is executed by the issuer switch while processing an
/// API transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    /// The unique identifier for this resource.
    /// Format: projects/{project}/rules/{rule}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the rule.
    #[prost(string, tag="2")]
    pub rule_description: ::prost::alloc::string::String,
    /// The API Type for which this rule gets executed. A value of
    /// `API_TYPE_UNSPECIFIED` indicates that the rule is executed for all API
    /// transactions.
    #[prost(enumeration="ApiType", tag="3")]
    pub api_type: i32,
    /// The transaction type for which this rule gets executed. A value of
    /// `TRANSACTION_TYPE_UNSPECIFIED` indicates that the rule is executed for
    /// all transaction types.
    #[prost(enumeration="TransactionType", tag="4")]
    pub transaction_type: i32,
}
/// The metadata associated with a rule. This defines data that are used by the
/// rule during execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleMetadata {
    /// The unique identifier for this resource.
    /// Format: projects/{project}/rules/{rule}/metadata/{metadata}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the rule metadata.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Type of rule metadata.
    #[prost(enumeration="rule_metadata::Type", tag="3")]
    pub r#type: i32,
}
/// Nested message and enum types in `RuleMetadata`.
pub mod rule_metadata {
    /// The type of metadata.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified type.
        Unspecified = 0,
        /// List type. Indicates that the metadata contains a list of values which
        /// the rule requires for execution.
        List = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::List => "LIST",
            }
        }
    }
}
/// Represent a single value in a rule's metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleMetadataValue {
    /// Output only. The unique identifier for this resource.
    /// Format: projects/{project}/rules/{rule}/metadata/{metadata}/values/{value}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The value of the resource which could be of type string or
    /// AccountReference. The metadata values for rules
    /// BlockedPayeeAccountReqPayDebitRule, BlockedPayerAccountReqPayDebitRule,
    /// BlockedPayeeAccountReqPayCreditRule and BlockedPayerAccountReqPayCreditRule
    /// should be of type AccountReference. For all other rules, metadata values
    /// should be of type string.
    ///
    /// The length of the `value` field depends on the type of
    /// the value being used for the rule metadata. The following are the minimum
    /// and maximum lengths for the different types of values.
    ///
    /// Value Type | Minimum Length | Maximum Length |
    /// -------- | -------- | -------- |
    /// Bank account IFSC   | 11   | 11   |
    /// Bank account number   | 1   | 255  |
    /// Device identifier   | 1   | 255   |
    /// Mobile number   | 12   | 12  |
    /// Virtual private address (VPA)   | 3   | 255   |
    #[prost(oneof="rule_metadata_value::Value", tags="2, 3")]
    pub value: ::core::option::Option<rule_metadata_value::Value>,
}
/// Nested message and enum types in `RuleMetadataValue`.
pub mod rule_metadata_value {
    /// The value of the resource which could be of type string or
    /// AccountReference. The metadata values for rules
    /// BlockedPayeeAccountReqPayDebitRule, BlockedPayerAccountReqPayDebitRule,
    /// BlockedPayeeAccountReqPayCreditRule and BlockedPayerAccountReqPayCreditRule
    /// should be of type AccountReference. For all other rules, metadata values
    /// should be of type string.
    ///
    /// The length of the `value` field depends on the type of
    /// the value being used for the rule metadata. The following are the minimum
    /// and maximum lengths for the different types of values.
    ///
    /// Value Type | Minimum Length | Maximum Length |
    /// -------- | -------- | -------- |
    /// Bank account IFSC   | 11   | 11   |
    /// Bank account number   | 1   | 255  |
    /// Device identifier   | 1   | 255   |
    /// Mobile number   | 12   | 12  |
    /// Virtual private address (VPA)   | 3   | 255   |
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// The value for string metadata.
        #[prost(string, tag="2")]
        Id(::prost::alloc::string::String),
        /// The value for account reference metadata.
        #[prost(message, tag="3")]
        AccountReference(super::AccountReference),
    }
}
/// Request body for the `ListRules` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRulesRequest {
    /// Required. The parent resource must have the format of `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of rules to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 50,
    /// at most 50 rules will be returned. The maximum value is 1000; values above
    /// 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListRulesRequest` call.
    /// Specify this parameter to retrieve the next page of rules.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response body for the `ListRules` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRulesResponse {
    /// List of rules satisfying the specified filter criteria.
    #[prost(message, repeated, tag="1")]
    pub rules: ::prost::alloc::vec::Vec<Rule>,
    /// Pass this token in a subsequent `ListRulesRequest` call to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of rules matching request criteria across all pages.
    #[prost(int64, tag="3")]
    pub total_size: i64,
}
/// Request body for the `ListRuleMetadata` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleMetadataRequest {
    /// Required. The parent resource. The format is `projects/{project}/rules/{rule}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of rule metadata to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 50,
    /// at most 50 rule metadata will be returned. The maximum value is 1000;
    /// values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListRuleMetadataRequest` call.
    /// Specify this parameter to retrieve the next page of rule metadata.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response body for the `ListRuleMetadata` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleMetadataResponse {
    /// List of rule metadata associated with the rule.
    #[prost(message, repeated, tag="1")]
    pub rule_metadata: ::prost::alloc::vec::Vec<RuleMetadata>,
    /// Pass this token in a subsequent `ListRuleMetadataRequest` call to continue
    /// to list results. If all results have been returned, this field is an empty
    /// string or not present in the response.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of rule metadata matching request criteria across all pages.
    #[prost(int64, tag="3")]
    pub total_size: i64,
}
/// Request body for the `ListRuleMetadataValues` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleMetadataValuesRequest {
    /// Required. The parent resource. The format is
    /// `projects/{project}/rules/{rule}/metadata/{metadata}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of metadata values to return. The service may return
    /// fewer than this value. If unspecified or if the specified value is less
    /// than 1, at most 50 rule metadata values will be returned. The maximum
    /// value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token received from a previous `ListRuleMetadataValuesRequest`
    /// call. Specify this parameter to retrieve the next page of rule metadata
    /// values.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response body for ListRuleMetadataValues. Contains a List of values for a
/// given rule metadata resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRuleMetadataValuesResponse {
    /// List of values for a given rule metadata resource identifier.
    #[prost(message, repeated, tag="1")]
    pub rule_metadata_values: ::prost::alloc::vec::Vec<RuleMetadataValue>,
    /// Pass this token in a subsequent `ListRuleMetadataValuesRequest` call to
    /// continue to list results. If all results have been returned, this field is
    /// an empty string or not present in the response.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request body for the `BatchCreateRuleMetadataValues` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateRuleMetadataValuesRequest {
    /// The parent resource shared by all ruleMetadataValue being created. The
    /// format is `projects/{project}/rules/{rule}/metadata/{metadata}`. The
    /// \[CreateRuleMetadataValueRequest.parent][google.cloud.paymentgateway.issuerswitch.v1.CreateRuleMetadataValueRequest.parent\] field in the
    /// \[CreateRuleMetadataValueRequest][google.cloud.paymentgateway.issuerswitch.v1.CreateRuleMetadataValueRequest\] messages contained in this request must
    /// match this field.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request message specifying the resources to create.
    /// A maximum of 1000 RuleMetadataValues can be created in a batch.
    #[prost(message, repeated, tag="2")]
    pub requests: ::prost::alloc::vec::Vec<CreateRuleMetadataValueRequest>,
}
/// Response body for the `BatchCreateRuleMetadataValues` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateRuleMetadataValuesResponse {
    /// List of RuleMetadataValue created.
    #[prost(message, repeated, tag="1")]
    pub rule_metadata_value: ::prost::alloc::vec::Vec<RuleMetadataValue>,
}
/// Request for creating a single `RuleMetadataValue`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRuleMetadataValueRequest {
    /// Required. The parent resource where this RuleMetadataValue will be created. The
    /// format is `projects/{project}/rules/{rule}/metadata/{metadata}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The rule metadata value to create or add to a list.
    #[prost(message, optional, tag="2")]
    pub rule_metadata_value: ::core::option::Option<RuleMetadataValue>,
}
/// Request body for the `BatchDeleteRuleMetadataValues` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteRuleMetadataValuesRequest {
    /// The parent resource shared by all RuleMetadataValues being deleted. The
    /// format is `projects/{project}/rules/{rule}/metadata/{metadata}`. If this is
    /// set, the parent of all of the RuleMetadataValues specified in the
    /// list of names must match this field.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The names of the rule metadata values to delete.
    /// A maximum of 1000 RuleMetadataValue can be deleted in a batch.
    /// Format: projects/{project}/rules/{rule}/metadata/{metadata}/values/{value}
    #[prost(string, repeated, tag="2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
pub mod issuer_switch_rules_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manages rules used by the issuer switch's rules engine.
    #[derive(Debug, Clone)]
    pub struct IssuerSwitchRulesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IssuerSwitchRulesClient<tonic::transport::Channel> {
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
    impl<T> IssuerSwitchRulesClient<T>
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
        ) -> IssuerSwitchRulesClient<InterceptedService<T, F>>
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
            IssuerSwitchRulesClient::new(InterceptedService::new(inner, interceptor))
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
        /// List all rules that are applied on transactions by the issuer switch. Rules
        /// can be filtered on API type and transaction type.
        pub async fn list_rules(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRulesRequest>,
        ) -> Result<tonic::Response<super::ListRulesResponse>, tonic::Status> {
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules/ListRules",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List all rule metadata for a given rule identifier.
        pub async fn list_rule_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuleMetadataRequest>,
        ) -> Result<tonic::Response<super::ListRuleMetadataResponse>, tonic::Status> {
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules/ListRuleMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List all metadata values for a rule metadata identifier.
        pub async fn list_rule_metadata_values(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRuleMetadataValuesRequest>,
        ) -> Result<
            tonic::Response<super::ListRuleMetadataValuesResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules/ListRuleMetadataValues",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Create (add) multiple values to the list of values under the specified rule
        /// metadata resource.
        pub async fn batch_create_rule_metadata_values(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateRuleMetadataValuesRequest>,
        ) -> Result<
            tonic::Response<super::BatchCreateRuleMetadataValuesResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules/BatchCreateRuleMetadataValues",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete (remove) multiple values from the list of values under the specified
        /// rules metadata resource.
        pub async fn batch_delete_rule_metadata_values(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteRuleMetadataValuesRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchRules/BatchDeleteRuleMetadataValues",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Information about a transaction processed by the issuer switch.
/// The fields in this type are common across both financial and metadata
/// transactions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfo {
    /// Output only. An identifier that is mandatorily present in every transaction processed
    /// via UPI. This maps to UPI's transaction ID.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The API type of the transaction.
    #[prost(enumeration="ApiType", tag="2")]
    pub api_type: i32,
    /// Output only. The transaction type.
    #[prost(enumeration="TransactionType", tag="3")]
    pub transaction_type: i32,
    /// Output only. The transaction sub-type.
    #[prost(enumeration="transaction_info::TransactionSubType", tag="4")]
    pub transaction_sub_type: i32,
    /// Output only. The transaction's state.
    #[prost(enumeration="transaction_info::State", tag="5")]
    pub state: i32,
    /// Output only. Error code of the failed transaction.
    #[prost(string, tag="6")]
    pub error_code: ::prost::alloc::string::String,
    /// Output only. Error description for the failed transaction.
    #[prost(string, tag="7")]
    pub error_message: ::prost::alloc::string::String,
    /// Output only. The time at which the transaction resource was created by the
    /// issuer switch.
    #[prost(message, optional, tag="8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. List of Request IDs (colon separated) used when
    /// invoking the Bank Adapter APIs for fulfilling a transaction request.
    #[prost(string, tag="9")]
    pub bank_adapter_request_ids: ::prost::alloc::string::String,
    /// Output only. Error code as per the UPI specification. The issuer switch maps the
    /// ErrorCode to an appropriate error code that complies with the UPI
    /// specification.
    #[prost(string, tag="10")]
    pub upi_error_code: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TransactionInfo`.
pub mod transaction_info {
    /// Specifies the current state of the transaction.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecicifed state.
        Unspecified = 0,
        /// The transaction has successfully completed.
        Succeeded = 1,
        /// The transaction has failed.
        Failed = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
            }
        }
    }
    /// The sub-type of a transaction. This value is used only for certain API type
    /// and transaction type combinations.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TransactionSubType {
        /// Unspecified transaction sub-type.
        Unspecified = 0,
        /// Collect sub type. This is used in a `SETTLE_PAYMENT` API type
        /// transaction, with transaction type as either `CREDIT` or `DEBIT` when the
        /// payment was initiated by a collect request.
        Collect = 1,
        /// Debit sub type. This is used in a `SETTLE_PAYMENT` API type transaction,
        /// with transaction type as `REVERSAL` when the original payment was a
        /// debit request.
        Debit = 2,
        /// Pay sub type. This is used in a `SETTLE_PAYMENT` API type transaction,
        /// with transaction type as either `CREDIT` or `DEBIT` when the payment was
        /// initiated by a pay request.
        Pay = 3,
        /// Beneficiary subtype. This is used in a `COMPLAINT` API type transaction,
        /// when the complaint / dispute request is initiated / received by the
        /// beneficiary bank.
        Beneficiary = 4,
        /// Remitter subtype. This is used in a `COMPLAINT` API type transaction,
        /// when the complaint / dispute request is initiated / received by the
        /// remitter bank.
        Remitter = 5,
    }
    impl TransactionSubType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TransactionSubType::Unspecified => "TRANSACTION_SUB_TYPE_UNSPECIFIED",
                TransactionSubType::Collect => "COLLECT",
                TransactionSubType::Debit => "DEBIT",
                TransactionSubType::Pay => "PAY",
                TransactionSubType::Beneficiary => "BENEFICIARY",
                TransactionSubType::Remitter => "REMITTER",
            }
        }
    }
}
/// A metadata API transaction processed by the issuer switch. This
/// includes UPI APIs such as List Accounts, Balance Enquiry, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataTransaction {
    /// The name of the metadata transaction. This uniquely identifies the
    /// transaction. Format of name is
    /// projects/{project_id}/metadataTransaction/{metadata_transaction_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Information about the transaction.
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<TransactionInfo>,
    /// Output only. Virtual Payment Address (VPA) which originated the request.
    #[prost(string, tag="3")]
    pub origin_vpa: ::prost::alloc::string::String,
}
/// A financial API transaction processed by the issuer switch. In UPI, this maps
/// to the Pay API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinancialTransaction {
    /// The name of the financial transaction. This uniquely identifies the
    /// transaction. Format of name is
    /// projects/{project_id}/financialTransactions/{financial_transaction_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Information about the transaction.
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<TransactionInfo>,
    /// Output only. A 12 digit numeric code associated with the request. It could contain
    /// leading 0s. In UPI, this is also known as as the customer reference or the
    /// UPI transaction ID.
    #[prost(string, tag="3")]
    pub retrieval_reference_number: ::prost::alloc::string::String,
    /// Output only. The payer in the transaction.
    #[prost(message, optional, tag="4")]
    pub payer: ::core::option::Option<SettlementParticipant>,
    /// Output only. The payee in the transaction.
    #[prost(message, optional, tag="5")]
    pub payee: ::core::option::Option<SettlementParticipant>,
    /// Output only. The amount for payment settlement in the transaction.
    #[prost(message, optional, tag="6")]
    pub amount: ::core::option::Option<super::super::super::super::r#type::Money>,
}
/// A mandate processed by the issuer switch. In UPI, this maps to the Mandate
/// API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MandateTransaction {
    /// The name of the mandate transaction. This uniquely identifies the
    /// transaction. Format of name is
    /// projects/{project_id}/mandateTransactions/{mandate_transaction_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Information about the transaction.
    #[prost(message, optional, tag="2")]
    pub transaction_info: ::core::option::Option<TransactionInfo>,
    /// Output only. This maps to Unique Mandate Number (UMN) in UPI specification.
    #[prost(string, tag="3")]
    pub unique_mandate_number: ::prost::alloc::string::String,
    /// Output only. The virtual payment address (VPA) of the payer.
    #[prost(string, tag="4")]
    pub payer_vpa: ::prost::alloc::string::String,
    /// Output only. The virtual payment address (VPA) of the payee.
    #[prost(string, tag="5")]
    pub payee_vpa: ::prost::alloc::string::String,
    /// Output only. A unique identifier for merchant.
    #[prost(string, tag="6")]
    pub payee_merchant_id: ::prost::alloc::string::String,
    /// Output only. The mobile number of the payer consisting of total twelve digits where
    /// first two digits of country code (for eg. 91 for India) and then ten
    /// digits mobile number. For eg. 911234567890
    #[prost(string, tag="7")]
    pub payer_mobile_number: ::prost::alloc::string::String,
    /// Output only. The mobile number of the payer consisting of total twelve digits where
    /// first two digits of country code (for eg. 91 for India) and then ten
    /// digits mobile number. For eg. 911234567890
    #[prost(string, tag="8")]
    pub payee_mobile_number: ::prost::alloc::string::String,
    /// Output only. The type of recurrence pattern of the mandate.
    #[prost(enumeration="mandate_transaction::RecurrencePatternType", tag="9")]
    pub recurrence_pattern: i32,
    /// Output only. The type of recurrence rule of the mandate.
    #[prost(enumeration="mandate_transaction::RecurrenceRuleType", tag="10")]
    pub recurrence_rule_type: i32,
    /// Output only. The recurrence rule value of the mandate. This is a value from 1 to 31.
    #[prost(int32, tag="11")]
    pub recurrence_rule_value: i32,
    /// Output only. The start date of the mandate.
    #[prost(message, optional, tag="12")]
    pub start_date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Output only. The end date of the mandate.
    #[prost(message, optional, tag="13")]
    pub end_date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Output only. If true, this specifies mandate can be revoked.
    #[prost(bool, tag="14")]
    pub revokable: bool,
    /// Output only. The amount of the mandate.
    #[prost(double, tag="15")]
    pub amount: f64,
    /// Output only. The amount rule type of the mandate.
    #[prost(enumeration="mandate_transaction::AmountRuleType", tag="16")]
    pub amount_rule: i32,
    /// Output only. The Block funds reference generated by the bank, this will be available
    /// only when Recurrence is ONETIME.
    #[prost(string, tag="17")]
    pub approval_reference: ::prost::alloc::string::String,
    /// Output only. If true, this specifies the mandate transaction requested funds to be
    /// blocked.
    #[prost(bool, tag="18")]
    pub block_funds: bool,
    /// Output only. The last time at which the mandate resource was modified by the issuer
    /// switch.
    #[prost(message, optional, tag="19")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `MandateTransaction`.
pub mod mandate_transaction {
    /// RecurrencePatternType specifies the recurrence pattern type of the mandate.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RecurrencePatternType {
        /// Unspecified recurrence pattern.
        Unspecified = 0,
        /// As presented recurrence pattern.
        AsPresented = 1,
        /// Bi monthly recurrence pattern.
        Bimonthly = 2,
        /// Daily recurrence pattern.
        Daily = 3,
        /// Bi weekly recurrence pattern.
        Fortnightly = 4,
        /// Half yearly recurrence pattern.
        HalfYearly = 5,
        /// Monthly recurrence pattern.
        Monthly = 6,
        /// One time recurrence pattern.
        OneTime = 7,
        /// Quarterly recurrence pattern.
        Quarterly = 8,
        /// Weekly recurrence pattern.
        Weekly = 9,
        /// Yearly recurrence pattern.
        Yearly = 10,
    }
    impl RecurrencePatternType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RecurrencePatternType::Unspecified => "RECURRENCE_PATTERN_TYPE_UNSPECIFIED",
                RecurrencePatternType::AsPresented => "AS_PRESENTED",
                RecurrencePatternType::Bimonthly => "BIMONTHLY",
                RecurrencePatternType::Daily => "DAILY",
                RecurrencePatternType::Fortnightly => "FORTNIGHTLY",
                RecurrencePatternType::HalfYearly => "HALF_YEARLY",
                RecurrencePatternType::Monthly => "MONTHLY",
                RecurrencePatternType::OneTime => "ONE_TIME",
                RecurrencePatternType::Quarterly => "QUARTERLY",
                RecurrencePatternType::Weekly => "WEEKLY",
                RecurrencePatternType::Yearly => "YEARLY",
            }
        }
    }
    /// RecurrenceRuleType specifies the recurrence rule type of mandate.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RecurrenceRuleType {
        /// Unspecified recurrence rule type.
        Unspecified = 0,
        /// After recurrence rule type.
        After = 1,
        /// Before recurrence rule type.
        Before = 2,
        /// On recurrence rule type.
        On = 3,
    }
    impl RecurrenceRuleType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RecurrenceRuleType::Unspecified => "RECURRENCE_RULE_TYPE_UNSPECIFIED",
                RecurrenceRuleType::After => "AFTER",
                RecurrenceRuleType::Before => "BEFORE",
                RecurrenceRuleType::On => "ON",
            }
        }
    }
    /// AmountRuleType specifies the type of rule associated with the mandate
    /// amount.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AmountRuleType {
        /// Unspecified amount rule.
        Unspecified = 0,
        /// Exact amount rule. Amount specified is the exact amount for which
        /// mandate could be granted.
        Exact = 1,
        /// Max amount rule. Amount specified is the maximum amount for which
        /// mandate could be granted.
        Max = 2,
    }
    impl AmountRuleType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AmountRuleType::Unspecified => "AMOUNT_RULE_TYPE_UNSPECIFIED",
                AmountRuleType::Exact => "EXACT",
                AmountRuleType::Max => "MAX",
            }
        }
    }
}
/// A complaint API transaction processed by the issuer switch. In
/// UPI, this maps to the Complaint API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComplaintTransaction {
    /// The name of the complaint transaction. This uniquely identifies the
    /// transaction. Format of name is
    /// projects/{project_id}/complaintTransactions/{complaint_transaction_id}.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Information about the transaction.
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<TransactionInfo>,
    /// Information about the complaint transaction. It can be one of Complaint or
    /// Dispute.
    #[prost(oneof="complaint_transaction::Case", tags="3, 4")]
    pub case: ::core::option::Option<complaint_transaction::Case>,
}
/// Nested message and enum types in `ComplaintTransaction`.
pub mod complaint_transaction {
    /// Information about the complaint transaction. It can be one of Complaint or
    /// Dispute.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Case {
        /// Output only. Information about the complaint transaction when it is of type complaint.
        #[prost(message, tag="3")]
        Complaint(super::Complaint),
        /// Output only. Information about the complaint transaction when it is of type dispute.
        #[prost(message, tag="4")]
        Dispute(super::Dispute),
    }
}
/// Request for the `ListMetadataTransactions` method. Callers can request for
/// transactions to be filtered by the given filter criteria and specified
/// pagination parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetadataTransactionsRequest {
    /// Required. The parent resource. The format is `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of transactions to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 1,
    /// at most 50 transactions will be returned. The maximum value is 1000; values
    /// above 1000 will be coerced to 1000. While paginating, you can specify a new
    /// page size parameter for each page of transactions to be listed.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListMetadataTransactions` call.
    /// Specify this parameter to retrieve the next page of transactions.
    ///
    /// When paginating, you must specify only the `page_token` parameter. The
    /// filter that was specified in the initial call to the
    /// `ListMetadataTransactions` method that returned the page token will be
    /// reused for all further calls where the page token parameter is specified.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of metadata transactions.
    ///
    /// A filter expression consists of a field name, a comparison
    /// operator, and a value for filtering. The value must be a string, a
    /// number, or a boolean. The comparison operator must be one of: `<`, `>` or
    /// `=`. Filters are not case sensitive.
    ///
    /// The following fields in the `MetadataTransaction` are eligible for
    /// filtering:
    ///
    ///    * `apiType` - The API type of the metadata transaction. Must be one of
    ///    \[ApiType][google.cloud.paymentgateway.issuerswitch.v1.ApiType\] values. Allowed comparison operators: `=`.
    ///    * `transactionType` - The transaction type of the metadata transaction.
    ///    Must be one of \[TransactionType][google.cloud.paymentgateway.issuerswitch.v1.TransactionType\] values. Allowed comparison
    ///    operators: `=`.
    ///    * `transactionID` - The UPI transaction ID of the metadata transaction.
    ///    Allowed comparison operators: `=`.
    ///    * `originVPA` - The VPA of the orignitator of a metadata transaction.
    ///    Allowed comparison operators: `=`.
    ///    * `createTime` - The time at which the transaction was created
    ///    (received) by the issuer switch. The value should be in
    ///    the format `YYYY-MM-DDTHH:MM:SSZ`. Allowed comparison operators: `>`,
    ///    `<`.
    ///    * `state` - The state of the transaction. Must be one of
    ///    \[TransactionInfo.State][google.cloud.paymentgateway.issuerswitch.v1.TransactionInfo.State\] values. Allowed comparison operators: `=`.
    ///    * `errorCode` - Use this filter to list financial transactions which
    ///    have failed a particular error code. Allowed comparison operators:
    ///    `=`.
    ///    * `bankAdapterRequestID` - Request ID used when invoking the Bank
    ///    Adapter API for fulfilling a transaction request. Allowed comparison
    ///    operators: `=`.
    ///
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. Expressions are combined with AND logic. No other logical
    /// operators are supported.
    ///
    /// Here are a few examples:
    ///
    ///    * `apiType = LIST_ACCOUNTS` -  - The API type is _LIST_ACCOUNTS_.
    ///    * `state = SUCCEEDED` - The transaction's state is _SUCCEEDED_.
    ///    * `(apiType = LIST_ACCOUNTS) AND (create_time <
    ///    \"2021-08-15T14:50:00Z\")` - The API type is _LIST_ACCOUNTS_ and
    ///    the transaction was received before _2021-08-15 14:50:00 UTC_.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// Request for the `ListFinancialTransactions` method. Callers can request for
/// transactions to be filtered by the given filter criteria and specified
/// pagination parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFinancialTransactionsRequest {
    /// Required. The parent resource. The format is `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of transactions to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 1,
    /// at most 50 transactions will be returned. The maximum value is 1000; values
    /// above 1000 will be coerced to 1000. While paginating, you can specify a new
    /// page size parameter for each page of transactions to be listed.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListFinancialTransactions` call.
    /// Specify this parameter to retrieve the next page of transactions.
    ///
    /// When paginating, you must specify only the `page_token` parameter. The
    /// filter that was specified in the initial call to the
    /// `ListFinancialTransactions` method that returned the page token will be
    /// reused for all further calls where the page token parameter is specified.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of financial transactions.
    ///
    /// A filter expression consists of a field name, a comparison operator, and
    /// a value for filtering. The value must be a string, a number, or a
    /// boolean. The comparison operator must be one of: `<`, `>`, or `=`.
    /// Filters are not case sensitive.
    ///
    /// The following fields in the `FinancialTransaction` are eligible for
    /// filtering:
    ///
    ///    * `transactionType` - The transaction type of the financial
    ///    transaction. Must be one of \[TransactionType][google.cloud.paymentgateway.issuerswitch.v1.TransactionType\] values. For financial
    ///    transactions, only valid transaction types are `TRANSACTION_TYPE_CREDIT`,
    ///    `TRANSACTION_TYPE_DEBIT` and `TRANSACTION_TYPE_REVERSAL`. Allowed
    ///    comparison operators: `=`.
    ///    * `transactionID` - The UPI transaction ID of the financial
    ///    transaction. Allowed comparison operators: `=`.
    ///    * `RRN` - The retrieval reference number of the transaction. Allowed
    ///    comparison operators: `=`.
    ///    * `payerVPA` - The VPA of the payer in a financial transaction. Allowed
    ///    comparison operators: `=`.
    ///    * `payeeVPA` - The VPA of the payee in a financial transaction. Allowed
    ///    comparison operators: `=`.
    ///    * `payerMobileNumber` - The mobile number of the payer in a financial
    ///       transaction. Allowed comparison operators: `=`.
    ///    * `payeeMobileNumber` - The mobile number of the payee in a financial
    ///       transaction. Allowed comparison operators: `=`.
    ///    * `payeeMerchantId` - The merchant id of the payee in a financial
    ///       transaction. Allowed comparison operators: `=`.
    ///    * `createTime` - The time at which the transaction was created
    ///    (received) by the issuer switch. The value should be in
    ///    the format `YYYY-MM-DDTHH:MM:SSZ`. Allowed comparison operators: `>`,
    ///    `<`.
    ///    * `state` - The state of the transaction. Must be one of
    ///    \[TransactionInfo.State][google.cloud.paymentgateway.issuerswitch.v1.TransactionInfo.State\] values. Allowed comparison operators: `=`.
    ///    * `errorCode` - Use this filter to list financial transactions which
    ///    have failed a particular error code. Allowed comparison operators: `=`.
    ///    * `bankAdapterRequestID` - Request ID used when invoking the Bank
    ///    Adapter API for fulfilling a transaction request. Allowed comparison
    ///    operators: `=`.
    ///
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. Expressions are combined with AND logic. No other logical
    /// operators are supported.
    ///
    /// Here are a few examples:
    ///
    ///    * `transactionType = CREDIT` - The transaction type is _CREDIT_.
    ///    * `state = SUCCEEDED` - The transaction's state is _SUCCEEDED_.
    ///    * `payerVpa = example@okbank` - The VPA of the payer is the string
    ///    _example@okbank_.
    ///    * `(transactionType = DEBIT) AND (createTime < "2021-08-15T14:50:00Z")`
    ///    - The transaction type is _DEBIT_ and the transaction was received
    ///    before _2021-08-15 14:50:00 UTC_.
    ///    * `createTime > "2021-08-15T14:50:00Z" AND createTime <
    ///    "2021-08-16T14:50:00Z"` - The transaction was received between
    ///    _2021-08-15 14:50:00 UTC_ and _2021-08-16 14:50:00 UTC_.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// Request for the `ListMandateTransactions` method. Callers can request for
/// transactions to be filtered by the given filter criteria and specified
/// pagination parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMandateTransactionsRequest {
    /// Required. The parent resource. The format is `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of transactions to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 1,
    /// at most 50 transactions will be returned. The maximum value is 1000; values
    /// above 1000 will be coerced to 1000. While paginating, you can specify a new
    /// page size parameter for each page of transactions to be listed.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListMandateTransactions` call.
    /// Specify this parameter to retrieve the next page of transactions.
    ///
    /// When paginating, you must specify only the `page_token` parameter. The
    /// filter that was specified in the initial call to the
    /// `ListMandateTransactions` method that returned the page token will be
    /// reused for all further calls where the page token parameter is specified.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of mandate transactions.
    ///
    /// A filter expression consists of a field name, a comparison operator, and
    /// a value for filtering. The value must be a string, a number, or a
    /// boolean. The comparison operator must be one of: `<`, `>`, or `=`.
    /// Filters are not case sensitive.
    ///
    /// The following fields in the `Mandate` are eligible for
    /// filtering:
    ///
    ///    * `uniqueMandateNumber` - UPI Unique Mandate Number (UMN). Allowed
    ///    comparison operators: `=`.
    ///    * `transactionID` - The transaction ID of the mandate transaction.
    ///    Allowed comparison operators: `=`.
    ///    * `transactionType` - The transaction type of the mandate
    ///    transaction. Must be one of \[TransactionType][google.cloud.paymentgateway.issuerswitch.v1.TransactionType\] values. For mandate
    ///    transactions, only valid transaction types are
    ///    `TRANSACTION_TYPE_CREATE`, `TRANSACTION_TYPE_REVOKE` and
    ///    `TRANSACTION_TYPE_UPDATE`. Allowed comparison operators: `=`.
    ///    * `payerVPA` - The VPA of the payer in a mandate transaction. Allowed
    ///    comparison operators: `=`.
    ///    * `payeeVPA` - The VPA of the payee in a mandate transaction. Allowed
    ///    comparison operators: `=`.
    ///    * `payeeMerchantID` - The merchant ID of the payee in a mandate
    ///    transaction. Allowed comparison operators: `=`.
    ///    * `payerMobileNumber` - The mobile number of the payer in a mandate
    ///    transaction. Allowed comparison operators: `=`.
    ///    * `payeeMobileNumber` - The mobile number of the payee in a mandate
    ///    transaction. Allowed comparison operators: `=`.
    ///    * `createTime` - The time at which the transaction was created
    ///    (received) by the issuer switch. The value should be in
    ///    the format `YYYY-MM-DDTHH:MM:SSZ`. Allowed comparison
    ///    operators: `>`, `<`.
    ///    * `state` - The state of the transaction. Must be one of
    ///    \[TransactionInfo.State][google.cloud.paymentgateway.issuerswitch.v1.TransactionInfo.State\] values. Allowed comparison operators: `=`.
    ///    * `recurrencePattern` - The recurrence pattern of the mandate. Must be
    ///    one of \[MandateTransaction.RecurrencePatternType][google.cloud.paymentgateway.issuerswitch.v1.MandateTransaction.RecurrencePatternType\] values. Allowed
    ///    comparison operators: `=`.
    ///    * `startDate` - The start date of the mandate. The value should be in
    ///    the format `YYYY-MM-DD`. Allowed comparison operators: `<` and `>`.
    ///    * `endDate` - The end date of the mandate. The value should be in
    ///    the format `YYYY-MM-DD`. Allowed comparison operators: `<` and `>`.
    ///    * `errorCode` - Use this filter to list mandate transactions which
    ///    have failed a particular error code. Allowed comparison
    ///    operators: `=`.
    ///    * `bankAdapterRequestID` - Request ID used when invoking the Bank
    ///    Adapter API for fulfilling a transaction request. Allowed comparison
    ///    operators: `=`.
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. Expressions are combined with AND logic. No other logical
    /// operators are supported.
    ///
    /// Here are a few examples:
    ///    * `recurrencePattern = MONTHLY` - The recurrence pattern type is
    ///    monthly.
    ///    * `state = SUCCEEDED` - The transaction's state is _SUCCEEDED_.
    ///    * `payerVPA = example@okbank` - The VPA of the payer is the string
    ///    _example@okbank_.
    ///    * `(payerVPA = example@okbank) AND (createTime <
    ///    "2021-08-15T14:50:00Z")`
    ///    - The payer VPA example@okbank and the transaction was received
    ///    before _2021-08-15 14:50:00 UTC_.
    ///    * `createTime > "2021-08-15T14:50:00Z" AND createTime <
    ///    "2021-08-16T14:50:00Z"` - The transaction was received between
    ///    _2021-08-15 14:50:00 UTC_ and _2021-08-16 14:50:00 UTC_.
    ///    * `startDate > "2021-08-15" AND startDate < "2021-08-17"` - The start
    ///    date for mandate is between _2021-08-15_ and _2021-08-17_.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// Request for the `ListComplaintTransactions` method. Callers can request for
/// transactions to be filtered by the given filter criteria and specified
/// pagination parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListComplaintTransactionsRequest {
    /// Required. The parent resource. The format is `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of transactions to return. The service may return fewer
    /// than this value. If unspecified or if the specified value is less than 1,
    /// at most 50 transactions will be returned. The maximum value is 1000; values
    /// above 1000 will be coerced to 1000. While paginating, you can specify a new
    /// page size parameter for each page of transactions to be listed.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListComplaintTransactions` call.
    /// Specify this parameter to retrieve the next page of transactions.
    ///
    /// When paginating, you must specify only the `page_token` parameter. The
    /// filter that was specified in the initial call to the
    /// `ListComplaintTransactions` method that returned the page token will be
    /// reused for all further calls where the page token parameter is specified.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// An expression that filters the list of complaint transactions.
    ///
    /// A filter expression consists of a field name, a comparison operator, and
    /// a value for filtering. The value must be a string, a number, or a
    /// boolean. The comparison operator must be one of: `<`, `>`, or `=`.
    /// Filters are not case sensitive.
    ///
    /// The following fields in the `Complaint` are eligible for
    /// filtering:
    ///
    ///    * `transactionID` - The transaction ID of the complaint transaction.
    ///    Allowed comparison operators: `=`.
    ///    * `transactionType` - The transaction type of the complaint
    ///    transaction. Must be one of \[TransactionType][google.cloud.paymentgateway.issuerswitch.v1.TransactionType\] values. For complaint
    ///    transactions, only valid transaction types are
    ///   `TRANSACTION_TYPE_CHECK_STATUS`, `TRANSACTION_TYPE_COMPLAINT`,
    ///   `TRANSACTION_TYPE_REVERSAL`, `TRANSACTION_TYPE_DISPUTE`,
    ///   `TRANSACTION_TYPE_REFUND` or `TRANSACTION_TYPE_STATUS_UPDATE`. Allowed
    ///    comparison operators: `=`.
    ///    * `originalRRN` - The retrieval reference number of the original
    ///    transaction for which complaint / dispute was raised / resolved. Allowed
    ///    comparison operators: `=`.
    ///    * `createTime` - The time at which the transaction was created
    ///    (received) by the issuer switch. The value should be in
    ///    the format `YYYY-MM-DDTHH:MM:SSZ`. Allowed comparison
    ///    operators: `>`, `<`.
    ///    * `state` - The state of the transaction. Must be one of
    ///    \[TransactionInfo.State][google.cloud.paymentgateway.issuerswitch.v1.TransactionInfo.State\] values. Allowed comparison operators: `=`.
    ///    * `errorCode` - Use this filter to list complaint transactions which
    ///    have failed a particular error code. Allowed comparison
    ///    operators: `=`.
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. Expressions are combined with AND logic. No other logical
    /// operators are supported.
    ///
    /// Here are a few examples:
    ///
    ///    * `state = SUCCEEDED` - The transaction's state is _SUCCEEDED_.
    ///    * (createTime < "2021-08-15T14:50:00Z")`
    ///    - The transaction was received before _2021-08-15 14:50:00 UTC_.
    ///    * `createTime > "2021-08-15T14:50:00Z" AND createTime <
    ///    "2021-08-16T14:50:00Z"` - The transaction was received between
    ///    _2021-08-15 14:50:00 UTC_ and _2021-08-16 14:50:00 UTC_.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response for the `ListMetadataTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetadataTransactionsResponse {
    /// List of non financial metadata transactions satisfying the filtered
    /// request.
    #[prost(message, repeated, tag="1")]
    pub metadata_transactions: ::prost::alloc::vec::Vec<MetadataTransaction>,
    /// Pass this token in the ListMetadataTransactionsRequest to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Response for the `ListFinancialTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFinancialTransactionsResponse {
    /// List of financial transactions satisfying the filtered request.
    #[prost(message, repeated, tag="1")]
    pub financial_transactions: ::prost::alloc::vec::Vec<FinancialTransaction>,
    /// Pass this token in the ListFinancialTransactionsRequest to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Response for the `ListMandateTransactionsResponse` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMandateTransactionsResponse {
    /// List of mandate transactions satisfying the filtered request.
    #[prost(message, repeated, tag="1")]
    pub mandate_transactions: ::prost::alloc::vec::Vec<MandateTransaction>,
    /// Pass this token in the ListMandateTransactionsRequest to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Response for the `ListComplaintTransactionsResponse` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListComplaintTransactionsResponse {
    /// List of complaint transactions satisfying the filtered request.
    #[prost(message, repeated, tag="1")]
    pub complaint_transactions: ::prost::alloc::vec::Vec<ComplaintTransaction>,
    /// Pass this token in the ListComplaintTransactionsRequest to continue to list
    /// results. If all results have been returned, this field is an empty string
    /// or not present in the response.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the `ExportFinancialTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFinancialTransactionsRequest {
    /// Required. The parent resource for the transactions. The format is
    /// `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Transaction type for the financial transaction API. The possible values for
    /// transaction type are
    ///
    /// * TRANSACTION_TYPE_CREDIT
    /// * TRANSACTION_TYPE_DEBIT
    /// * TRANSACTION_TYPE_REVERSAL
    ///
    /// If no transaction type is specified, records of all the above transaction
    /// types will be exported.
    #[prost(enumeration="TransactionType", tag="2")]
    pub transaction_type: i32,
    /// The start time for the query.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time for the query.
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for the `ExportMetadataTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMetadataTransactionsRequest {
    /// Required. The parent resource for the transactions. The format is
    /// `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// API type of the metadata transaction API. The possible values for API type
    /// are
    ///
    /// * BALANCE
    /// * CHECK_STATUS
    /// * HEART_BEAT
    /// * INITIATE_REGISTRATION
    /// * LIST_ACCOUNTS
    /// * UPDATE_CREDENTIALS
    /// * VALIDATE_REGISTRATION
    ///
    /// If no API type is specified, records of all the above API types will be
    /// exported.
    #[prost(enumeration="ApiType", tag="2")]
    pub api_type: i32,
    /// The start time for the query.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time for the query.
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for the `ExportMandateTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMandateTransactionsRequest {
    /// Required. The parent resource for the transactions. The format is
    /// `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Transaction type for the mandate transaction API.  The possible values for
    /// transaction type are
    ///
    /// * TRANSACTION_TYPE_CREATE
    /// * TRANSACTION_TYPE_REVOKE
    /// * TRANSACTION_TYPE_UPDATE
    ///
    /// If no transaction type is specified, records of all the above transaction
    /// types will be exported.
    #[prost(enumeration="TransactionType", tag="2")]
    pub transaction_type: i32,
    /// The start time for the query.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time for the query.
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for the `ExportComplaintTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportComplaintTransactionsRequest {
    /// Required. The parent resource for the transactions. The format is
    /// `projects/{project}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Transaction type for the complaint transaction API. The possible values for
    /// transaction type are
    ///
    /// * TRANSACTION_TYPE_CHECK_STATUS
    /// * TRANSACTION_TYPE_COMPLAINT
    /// * TRANSACTION_TYPE_DISPUTE
    /// * TRANSACTION_TYPE_REFUND
    /// * TRANSACTION_TYPE_REVERSAL
    /// * TRANSACTION_TYPE_STATUS_UPDATE
    ///
    /// If no transaction type is specified, records of all the above transaction
    /// types will be exported.
    #[prost(enumeration="TransactionType", tag="2")]
    pub transaction_type: i32,
    /// The start time for the query.
    #[prost(message, optional, tag="3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time for the query.
    #[prost(message, optional, tag="4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response for the `ExportFinancialTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFinancialTransactionsResponse {
    /// URI of the exported file.
    #[prost(string, tag="1")]
    pub target_uri: ::prost::alloc::string::String,
}
/// Response for the `ExportMetadataTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMetadataTransactionsResponse {
    /// URI of the exported file.
    #[prost(string, tag="1")]
    pub target_uri: ::prost::alloc::string::String,
}
/// Response for the `ExportMandateTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMandateTransactionsResponse {
    /// URI of the exported file.
    #[prost(string, tag="1")]
    pub target_uri: ::prost::alloc::string::String,
}
/// Response for the `ExportComplaintTransactions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportComplaintTransactionsResponse {
    /// URI of the exported file.
    #[prost(string, tag="1")]
    pub target_uri: ::prost::alloc::string::String,
}
/// Metadata for ExportFinancialTransactions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportFinancialTransactionsMetadata {
}
/// Metadata for ExportMandateTransactions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMandateTransactionsMetadata {
}
/// Metadata for ExportMetadataTransactions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMetadataTransactionsMetadata {
}
/// Metadata for ExportComplaintTransactions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportComplaintTransactionsMetadata {
}
/// Generated client implementations.
pub mod issuer_switch_transactions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Lists and exports transactions processed by the issuer switch.
    #[derive(Debug, Clone)]
    pub struct IssuerSwitchTransactionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IssuerSwitchTransactionsClient<tonic::transport::Channel> {
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
    impl<T> IssuerSwitchTransactionsClient<T>
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
        ) -> IssuerSwitchTransactionsClient<InterceptedService<T, F>>
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
            IssuerSwitchTransactionsClient::new(
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
        /// List metadata transactions that satisfy the specified filter criteria.
        pub async fn list_metadata_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetadataTransactionsRequest>,
        ) -> Result<
            tonic::Response<super::ListMetadataTransactionsResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ListMetadataTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List financial transactions that satisfy specified filter criteria.
        pub async fn list_financial_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFinancialTransactionsRequest>,
        ) -> Result<
            tonic::Response<super::ListFinancialTransactionsResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ListFinancialTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List mandate transactions that satisfy specified filter criteria.
        pub async fn list_mandate_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMandateTransactionsRequest>,
        ) -> Result<
            tonic::Response<super::ListMandateTransactionsResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ListMandateTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List complaint transactions that satisfy specified filter criteria.
        pub async fn list_complaint_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListComplaintTransactionsRequest>,
        ) -> Result<
            tonic::Response<super::ListComplaintTransactionsResponse>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ListComplaintTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Export financial transactions received within the specified time range as a
        /// file into a configured target location. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`: [ExportFinancialTransactionsMetadata][google.cloud.paymentgateway.issuerswitch.v1.ExportFinancialTransactionsMetadata]
        /// - `response`: [ExportFinancialTransactionsResponse][google.cloud.paymentgateway.issuerswitch.v1.ExportFinancialTransactionsResponse]
        ///
        /// The exported file will be in the standard CSV format where each row in the
        /// file represents a transaction. The file has the following fields in order:
        ///
        /// - `TransactionID` - UPI transaction ID.
        /// - `TransactionType` - Type of the transaction. This will be one of
        /// `TRANSACTION_TYPE_CREDIT`, `TRANSACTION_TYPE_DEBIT` or
        /// `TRANSACTION_TYPE_REVERSAL`.
        /// - `TransactionSubType` - Subtype of the transaction. This will be one
        /// of `COLLECT`, `DEBIT` or `PAY`.
        /// - `RequestReceivedTimestamp` - Timestamp (in UTC) indicating when the
        /// transaction API request was received by the issuer switch.
        /// - `CreationTime` - Timestamp (in UTC) indicating when the
        /// issuer switch created the transaction resource for processing the
        /// transaction.
        /// - `State` - State of the transaction. This will be one of `FAILED` or
        /// `SUCCEEDED`.
        /// - `RRN` - Retrieval reference number associated with the transaction.
        /// - `PayerVPA` - Virtual Payment Address (VPA) of the payer.
        /// - `PayerMobileNumber` - Mobile number of the payer.
        /// - `PayerIFSCCode` - IFSC code of the payer's bank account.
        /// - `PayerAccountNumber` - Payer's bank account number.
        /// - `PayerAccountType` - Payer's bank account type.
        /// - `PayeeVPA` - Virtual Payment Address (VPA) of the payee.
        /// - `PayeeMobileNumber` - Payee's mobile number.
        /// - `PayeeIFSCCode` - IFSC code of the payee's bank account.
        /// - `PayeeAccountNumber` - Payee's bank account number.
        /// - `PayeeAccountType` - Payee's bank account type.
        /// - `PayeeMerchantID` - Payee's merchant ID, only if the payee is a merchant.
        /// - `PayeeMerchantName` - Payee's merchant name, only if the payee is a
        /// merchant.
        /// - `PayeeMCC` - Payee's Merchant Category Code (MCC), only if the payee is a
        /// merchant.
        /// - `Currency` - Currency of the amount involved in the transaction.
        /// - `Amount` - Amount involved in the transaction.
        /// - `BankAdapterRequestIDs` - List of Request IDs (colon separated) used when
        /// invoking the Bank Adapter APIs for fulfilling a transaction request.
        /// - `ErrorCode` - Error code of the failed transaction.
        /// - `ErrorMessage` - Error description for the failed transaction.
        /// - `UPIErrorCode` - Error code as per the UPI specification. The issuer
        /// switch maps the ErrorCode to an appropriate error code that complies with
        /// the UPI specification.
        pub async fn export_financial_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportFinancialTransactionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ExportFinancialTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Export metadata transactions received within the specified time range as a
        /// file into a configured target location. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`: [ExportMetadataTransactionsMetadata][google.cloud.paymentgateway.issuerswitch.v1.ExportMetadataTransactionsMetadata]
        /// - `response`: [ExportMetadataTransactionsResponse][google.cloud.paymentgateway.issuerswitch.v1.ExportMetadataTransactionsResponse]
        ///
        /// The exported file will be in the standard CSV format where each row in the
        /// file represents a transaction. The file has the following fields in order:
        ///
        /// - `TransactionID` - UPI transaction ID.
        /// - `APIType` - The transaction's API type. The value will be of the
        /// [ApiType][google.cloud.paymentgateway.issuerswitch.v1.ApiType] enum.
        /// - `TransactionType` - Type of the transaction. The value will be of the
        /// [TransactionType][google.cloud.paymentgateway.issuerswitch.v1.TransactionType] enum.
        /// - `RequestReceivedTimestamp` - Timestamp (in UTC) indicating when the
        /// transaction's API request was received by the issuer switch.
        /// - `CreationTime` - Timestamp (in UTC) indicating when the
        /// issuer switch created the transaction resource for processing the
        /// transaction.
        /// - `State` - State of transaction. This will be one of `FAILED` or
        /// `SUCCEEDED`.
        /// - `OriginVPA` - Virtual Payment Address (VPA) of the originator of the
        /// transaction.
        /// - `BankAdapterRequestIDs` - List of Request IDs (colon separated) used when
        /// invoking the Bank Adapter APIs for fulfilling a transaction request.
        /// - `ErrorCode` - Error code of the failed transaction.
        /// - `ErrorMessage` - Error description for the failed transaction.
        /// - `UPIErrorCode` - Error code as per the UPI specification. The issuer
        /// switch maps the ErrorCode to an appropriate error code that complies with
        /// the UPI specification.
        pub async fn export_metadata_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportMetadataTransactionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ExportMetadataTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Export mandate transactions received within the specified time range as a
        /// file into a configured target location. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`: [ExportMandateTransactionsMetadata][google.cloud.paymentgateway.issuerswitch.v1.ExportMandateTransactionsMetadata]
        /// - `response`: [ExportMandateTransactionsResponse][google.cloud.paymentgateway.issuerswitch.v1.ExportMandateTransactionsResponse]
        ///
        /// The exported file will be in the standard CSV format where each row in the
        /// file represents a transaction. The file has the following fields in order:
        ///
        /// - `TransactionID` - UPI transaction ID.
        /// - `UniqueMandateNumber` - UPI Unique Mandate Number.
        /// - `TransactionType` - Type of the transaction. This will be one of
        /// `TRANSACTION_TYPE_CREATE`, `TRANSACTION_TYPE_REVOKE` or
        /// `TRANSACTION_TYPE_UPDATE`.
        /// - `RequestReceivedTimestamp` - Timestamp (in UTC) indicating when the
        /// mandate API request was received by the issuer switch.
        /// - `CreationTime` - Timestamp (in UTC) indicating when the
        /// issuer switch created the transaction resource for processing the
        /// transaction.
        /// - `State` - State of the transaction. This will be one of
        /// `FAILED` or `SUCCEEDED`.
        /// - `PayerVPA` - Virtual Payment Address (VPA) of the payer.
        /// - `PayerMobileNumber` - Mobile number of the payer.
        /// - `PayeeVPA` - Virtual Payment Address (VPA) of the payee.
        /// - `PayeeMobileNumber` - Mobile number of the payee.
        /// - `PayeeMerchantID` - Payee's merchant ID.
        /// - `Amount` - Amount specified in the mandate.
        /// - `RecurrencePattern` - Reccurence pattern of the mandate. The value will
        /// be of the [MandateTransaction.RecurrencePattern][] enum.
        /// - `RecurrenceRuleType` - Reccurrence rule type of the mandate. The value
        /// will be of the [MandateTransaction.RecurrenceRuleType][google.cloud.paymentgateway.issuerswitch.v1.MandateTransaction.RecurrenceRuleType] enum.
        /// - `RecurrenceRuleValue` - Recurrence rule value of the mandate.
        /// - `Revokeable` - Boolean value specifying if the mandate is revokable.
        /// - `StartDate` - The start date of the mandate in `YYYY-MM-DD` format.
        /// - `EndDate` - The end date of the mandate in `YYYY-MM-DD` format.
        /// - `AmountRuleType` - The amount rule of the mandate. The value will
        /// be of the [MandateTransaction.AmountRuleType][google.cloud.paymentgateway.issuerswitch.v1.MandateTransaction.AmountRuleType] enum.
        /// - `ApprovalReference` - The block funds reference generated by the bank,
        /// if funds have been blocked for the mandate. This column have a value only
        /// when the RecurrencePattern is ONETIME.
        /// - `BlockFunds` - Boolean value specifying if the mandate transaction
        /// requested to block funds.
        /// - `LastUpdateTime` - Timestamp (in UTC) indicating when was the last
        /// modification made to the mandate.
        /// - `BankAdapterRequestIDs` - List of Request IDs (colon separated) used when
        /// invoking the Bank Adapter APIs for fulfilling a transaction request.
        /// - `ErrorCode` - Error code of the failed transaction.
        /// - `ErrorMessage` - Error description for the failed transaction.
        /// - `UPIErrorCode` - Error code as per the UPI specification. The issuer
        /// switch maps the ErrorCode to an appropriate error code that complies with
        /// the UPI specification.
        pub async fn export_mandate_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportMandateTransactionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ExportMandateTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Export complaint transactions received within the specified time range as a
        /// file into a configured target location. The returned `Operation` type has
        /// the following method-specific fields:
        ///
        /// - `metadata`: [ExportComplaintTransactionsMetadata][google.cloud.paymentgateway.issuerswitch.v1.ExportComplaintTransactionsMetadata]
        /// - `response`: [ExportComplaintTransactionsResponse][google.cloud.paymentgateway.issuerswitch.v1.ExportComplaintTransactionsResponse]
        ///
        /// The exported file will be in the standard CSV format where each row in the
        /// file represents a transaction. The file has the following fields in order:
        ///
        /// - `TransactionID` - UPI transaction ID.
        /// - `TransactionType` - Type of the transaction. This will be one of
        /// `TRANSACTION_TYPE_CHECK_STATUS`, `TRANSACTION_TYPE_COMPLAINT`,
        /// `TRANSACTION_TYPE_REVERSAL`, `TRANSACTION_TYPE_DISPUTE`,
        /// `TRANSACTION_TYPE_REFUND` or `TRANSACTION_TYPE_STATUS_UPDATE`.
        /// - `CreationTime` - Timestamp (in UTC) indicating when the issuer switch
        /// created the transaction resource for processing the transaction.
        /// - `State` - State of the transaction. This will be one of
        /// `FAILED` or `SUCCEEDED`.
        /// - `OriginalRRN` - RRN of the original payment transaction.
        /// - `BankType` - The subtype of the transaction based on the bank involved.
        /// - `OriginalTransactionID` - Transaction ID of the original unresolved
        /// transaction.
        /// - `RaiseComplaintAdjFlag` - Indicates the type of action to raise the
        /// complaint.
        /// - `RaiseComplaintAdjCode` - Indicates the reason of action to raise the
        /// complaint.
        /// - `ResolveComplaintAdjFlag` - Indicates the type of action to resolve the
        /// complaint.
        /// - `ResolveComplaintAdjCode` - Indicates the reason of action to resolve the
        /// complaint.
        /// - `RaiseDisputeAdjFlag` - Indicates the type of action to raise the
        /// dispute.
        /// - `RaiseDisputeAdjCode` - Indicates the reason of action to raise the
        /// dispute.
        /// - `ResolveDisputeAdjFlag` - Indicates the type of action to resolve the
        /// dispute.
        /// - `ResolveDisputeAdjCode` - Indicates the reason of action to resolve the
        /// dispute.
        /// - `Amount` - Amount to be resolved.
        /// - `CurrentCycle` - Boolean value specifying if the complaint / dispute
        /// belongs to current settlement cycle or not.
        /// - `CRN` - Defines the Complaint Reference number.
        /// - `AdjTime` - Indicates the time when the resolution was done.
        /// - `RespAdjFlag` - Indicates the response category type.
        /// - `RespAdjCode` - Indicates the response reason used.
        /// - `AdjRemarks` - Indicates the additional remarks for the complaint /
        /// dispute.
        /// - `BankAdapterRequestIDs` - List of Request IDs (colon separated) used when
        /// invoking the Bank Adapter APIs for fulfilling a transaction request.
        /// - `ErrorCode` - Error code of the failed transaction.
        /// - `ErrorMessage` - Error description for the failed transaction.
        /// - `UPIErrorCode` - Error code as per the UPI specification. The issuer
        /// switch service maps the ErrorCode to an appropriate error code that
        /// complies with the UPI specification.
        pub async fn export_complaint_transactions(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportComplaintTransactionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.paymentgateway.issuerswitch.v1.IssuerSwitchTransactions/ExportComplaintTransactions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
