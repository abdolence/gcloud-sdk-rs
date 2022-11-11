/// The lifecycle state of an object, such as label, field, or choice. The
/// lifecycle enforces the following transitions:
///
/// * `UNPUBLISHED_DRAFT` (starting state)
/// * `UNPUBLISHED_DRAFT` -> `PUBLISHED`
/// * `UNPUBLISHED_DRAFT` -> (Deleted)
/// * `PUBLISHED` -> `DISABLED`
/// * `DISABLED` -> `PUBLISHED`
/// * `DISABLED` -> (Deleted)
///
/// The published and disabled states have some distinct characteristics:
///
/// * Published—Some kinds of changes might be made to an object in this state,
///    in which case `has_unpublished_changes` will be true. Also, some kinds of
///    changes are not permitted. Generally, any change that would invalidate or
///    cause new restrictions on existing metadata related to the label are
///    rejected.
/// * Disabled—When disabled, the configured `DisabledPolicy` takes effect.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lifecycle {
    /// Output only. The state of the object associated with this lifecycle.
    #[prost(enumeration = "lifecycle::State", tag = "1")]
    pub state: i32,
    /// Output only. Whether the object associated with this lifecycle has
    /// unpublished changes.
    #[prost(bool, tag = "2")]
    pub has_unpublished_changes: bool,
    /// The policy that governs how to show a disabled label, field, or selection
    /// choice.
    #[prost(message, optional, tag = "3")]
    pub disabled_policy: ::core::option::Option<lifecycle::DisabledPolicy>,
}
/// Nested message and enum types in `Lifecycle`.
pub mod lifecycle {
    /// The policy that governs how to treat a disabled label, field, or selection
    /// choice in different contexts.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisabledPolicy {
        /// Whether to hide this disabled object in the search menu for Drive items.
        ///
        /// * When `false`, the object is generally shown in the UI as disabled but
        /// it appears in the search results when searching for Drive items.
        /// * When `true`, the object is generally hidden in the UI when
        ///    searching for Drive items.
        #[prost(bool, tag = "1")]
        pub hide_in_search: bool,
        /// Whether to show this disabled object in the apply menu on Drive items.
        ///
        /// * When `true`, the object is generally shown in the UI as disabled
        ///    and is unselectable.
        /// * When `false`, the object is generally hidden in the UI.
        #[prost(bool, tag = "2")]
        pub show_in_apply: bool,
    }
    /// The state of the object associated with this lifecycle.
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
    pub enum State {
        /// Unknown State.
        Unspecified = 0,
        /// The initial state of an object. Once published, the object can never
        /// return to this state. Once an object is published, certain kinds of
        /// changes are no longer permitted.
        UnpublishedDraft = 1,
        /// The object has been published. The object might have unpublished draft
        /// changes as indicated by `has_unpublished_changes`.
        Published = 2,
        /// The object has been published and has since been disabled. The object
        /// might have unpublished draft changes as indicated by
        /// `has_unpublished_changes`.
        Disabled = 3,
        /// The object has been deleted.
        Deleted = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::UnpublishedDraft => "UNPUBLISHED_DRAFT",
                State::Published => "PUBLISHED",
                State::Disabled => "DISABLED",
                State::Deleted => "DELETED",
            }
        }
    }
}
/// Information about a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// The identifier for this user that can be used with the People API to get
    /// more information.
    /// For example, people/12345678.
    #[prost(string, tag = "1")]
    pub person: ::prost::alloc::string::String,
}
/// Badge status of the label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadgeConfig {
    /// The color of the badge. When not specified, no badge is rendered.
    /// The background, foreground, and solo (light and dark mode) colors set here
    /// are changed in the Drive UI into the closest recommended supported color.
    #[prost(message, optional, tag = "1")]
    pub color: ::core::option::Option<super::super::super::super::r#type::Color>,
    /// Override the default global priority of this badge.
    /// When set to 0, the default priority heuristic is used.
    #[prost(int64, tag = "2")]
    pub priority_override: i64,
}
/// The color derived from BadgeConfig and changed to the closest recommended
/// supported color.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BadgeColors {
    /// Output only. Badge background that pairs with the foreground.
    #[prost(message, optional, tag = "1")]
    pub background_color: ::core::option::Option<
        super::super::super::super::r#type::Color,
    >,
    /// Output only. Badge foreground that pairs with the background.
    #[prost(message, optional, tag = "2")]
    pub foreground_color: ::core::option::Option<
        super::super::super::super::r#type::Color,
    >,
    /// Output only. Color that can be used for text without a background.
    #[prost(message, optional, tag = "3")]
    pub solo_color: ::core::option::Option<super::super::super::super::r#type::Color>,
}
/// Contains information about whether a label component should be considered
/// locked.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockStatus {
    /// Output only. Indicates whether this label component is the (direct) target
    /// of a LabelLock.  A label component can be implicitly locked even if it's
    /// not the direct target of a LabelLock, in which case this field is set to
    /// false.
    #[prost(bool, tag = "1")]
    pub locked: bool,
}
/// Describes violations in a request to create or update a Label or its
/// Fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidArgument {
    /// Describes all violations in a client request.
    #[prost(message, repeated, tag = "1")]
    pub field_violations: ::prost::alloc::vec::Vec<invalid_argument::FieldViolation>,
}
/// Nested message and enum types in `InvalidArgument`.
pub mod invalid_argument {
    /// Describes the Field in which the violation occurred.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldViolation {
        /// The path to the field where this violation occurred. This path is
        /// specified using `FieldMask` format:
        /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
        #[prost(string, tag = "1")]
        pub field: ::prost::alloc::string::String,
        /// The detailed reason for this FieldViolation.
        #[prost(enumeration = "field_violation::Reason", tag = "2")]
        pub reason: i32,
        /// A message that describes the violation. This message is intended to
        /// be shown to end users, and is localized into the requesting user's
        /// preferred language.
        #[prost(string, tag = "3")]
        pub display_message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `FieldViolation`.
    pub mod field_violation {
        /// Possible reasons a field is invalid.
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
            /// Unknown reason.
            Unspecified = 0,
            /// The referenced field is required.
            FieldRequired = 1,
            /// The referenced value was invalid.
            InvalidValue = 2,
            /// The specified numeric value is out of the allowed range.
            ValueOutOfRange = 3,
            /// The specified string value was too long.
            StringValueTooLong = 4,
            /// The number of entries exceeded the maximum.
            MaxEntriesExceeded = 5,
            /// The specified field is not found in the Label.
            FieldNotFound = 6,
            /// The specified choice is not found in the Field.
            ChoiceNotFound = 7,
        }
        impl Reason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Reason::Unspecified => "REASON_UNSPECIFIED",
                    Reason::FieldRequired => "FIELD_REQUIRED",
                    Reason::InvalidValue => "INVALID_VALUE",
                    Reason::ValueOutOfRange => "VALUE_OUT_OF_RANGE",
                    Reason::StringValueTooLong => "STRING_VALUE_TOO_LONG",
                    Reason::MaxEntriesExceeded => "MAX_ENTRIES_EXCEEDED",
                    Reason::FieldNotFound => "FIELD_NOT_FOUND",
                    Reason::ChoiceNotFound => "CHOICE_NOT_FOUND",
                }
            }
        }
    }
}
/// Describes what preconditions have failed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreconditionFailure {
    /// Describes all violations in a client request.
    #[prost(message, repeated, tag = "1")]
    pub violation: ::prost::alloc::vec::Vec<precondition_failure::Violation>,
}
/// Nested message and enum types in `PreconditionFailure`.
pub mod precondition_failure {
    /// Specific failure reason.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Violation {
        /// The path to the field where this violation occurred. This path is
        /// specified using `FieldMask` format:
        /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
        #[prost(string, tag = "1")]
        pub field: ::prost::alloc::string::String,
        /// The type of this violation.
        #[prost(enumeration = "violation::Reason", tag = "2")]
        pub reason: i32,
        /// A message that describes the violation. This message is intended to
        /// be shown to end users, and is localized into the requesting user's
        /// preferred language.
        #[prost(string, tag = "3")]
        pub display_message: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Violation`.
    pub mod violation {
        /// The possible reasons a the violation occurred.
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
            /// Unknown violation type.
            Unspecified = 0,
            /// This Resource cannot be Disabled. Only Published resources can be
            /// Disabled.
            CannotDisable = 1,
            /// This Resource cannot be Enabled. Only Disabled resources can be
            /// Enabled.
            CannotEnable = 2,
            /// This Resource cannot be Published. Only Draft or Disabled resources
            /// can be Published.
            CannotPublish = 3,
            /// This Resource cannot be Unpublished. Once published, resources may
            /// not be set in "Draft" state.
            CannotUnpublish = 4,
            /// This Resource cannot be Deleted. Only Disabled resources
            /// can be Deleted.
            CannotDelete = 5,
            /// The request modified a range in a Field, but the new range does
            /// not include the previous range. When this error happens, `field` points
            /// at the Field where the violation occurred.
            CannotRestrictRange = 6,
            /// The specified change cannot be made to published Resources.
            CannotChangePublishedField = 7,
            /// The customer cannot create new labels because the maximum number
            /// of labels for the customer has been reached.
            CannotCreateMoreLabels = 8,
            /// The Field type cannot be changed because the Field has been published.
            CannotChangePublishedFieldType = 9,
            /// The Label component is locked and cannot be modified
            CannotModifyLockedComponent = 10,
        }
        impl Reason {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Reason::Unspecified => "REASON_UNSPECIFIED",
                    Reason::CannotDisable => "CANNOT_DISABLE",
                    Reason::CannotEnable => "CANNOT_ENABLE",
                    Reason::CannotPublish => "CANNOT_PUBLISH",
                    Reason::CannotUnpublish => "CANNOT_UNPUBLISH",
                    Reason::CannotDelete => "CANNOT_DELETE",
                    Reason::CannotRestrictRange => "CANNOT_RESTRICT_RANGE",
                    Reason::CannotChangePublishedField => "CANNOT_CHANGE_PUBLISHED_FIELD",
                    Reason::CannotCreateMoreLabels => "CANNOT_CREATE_MORE_LABELS",
                    Reason::CannotChangePublishedFieldType => {
                        "CANNOT_CHANGE_PUBLISHED_FIELD_TYPE"
                    }
                    Reason::CannotModifyLockedComponent => {
                        "CANNOT_MODIFY_LOCKED_COMPONENT"
                    }
                }
            }
        }
    }
}
/// Normalized internal-only message that identifies the exact exception that
/// caused the error on the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExceptionDetail {
    /// The type of exception that occurred.
    /// required
    #[prost(enumeration = "ExceptionType", tag = "1")]
    pub error_type: i32,
}
/// Every ExceptionType maps to one and only one Exception class. This allows
/// internal clients to identify the exact server exception that caused the
/// error for debugging and logging purposes.
/// Add new ExceptionTypes to EXCEPTION_TYPE_TO_ERROR_CODE_MAP in
/// j/c/g/apps/boq/metadata/model/service/exceptions/CategoryExceptionHelper
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExceptionType {
    /// Unknown ExceptionType.
    Unspecified = 0,
    /// The required field is missing.
    FieldRequired = 1,
    /// Unable to create a metamodel with the given ID because it already exists.
    MetamodelAlreadyExists = 2,
    /// Metamodel was not found
    MetamodelNotFound = 3,
    /// Metamodel state transition isn't allowed.
    IllegalMetamodelStateTransition = 4,
    /// Metamodel deprecation policy is invalid.
    InvalidMetamodelDeprecationPolicy = 5,
    /// Cannot delete a metamodel due to the pending deprecation policy.
    MetamodelDeletionDeniedUntil = 6,
    /// A Field value is invalid.
    InvalidField = 7,
    /// Precondition failed when updating a metamodel
    MetamodelPreconditionFailed = 8,
    /// Multiple fields had the same key.
    DuplicateFieldKey = 9,
    /// Removing a field from a Metamodel (e.g. a published Metamodel) is not
    /// permitted.
    IllegalFieldRemoval = 10,
    /// Cannot specify field options for a different field type.
    IllegalFieldOptionsForField = 11,
    /// Some changes are not supported
    UnsupportedChangeToPublishedMetamodel = 12,
    /// Cannot change the metamodel state in an update
    IllegalMetamodelStateTransitionInUpdate = 13,
    /// The page token is expired
    PageTokenExpired = 14,
    /// The user is not authorized to make the request.
    NotAuthorized = 15,
    /// Illegal field state transition
    IllegalFieldStateTransition = 16,
    /// Illegal choice set option state transition
    IllegalChoiceSetOptionStateTransition = 17,
    /// Invalid choice set options
    InvalidChoiceSetOptions = 18,
    /// Invalid field key
    InvalidFieldKey = 19,
    /// A specified property on a field is outside the allowed range.
    InvalidFieldPropertyRange = 20,
    /// A localized string wasn't valid. This may be because the locale is invalid,
    /// its missing a default value, or the translation is empty for a set locale.
    InvalidLocalizedString = 21,
    /// cannot change a property on a published field
    IllegalChangeToPublishedField = 22,
    /// A field update is not inclusive of the previous value
    InvalidFieldUpdateNotInclusive = 23,
    /// A field update is not inclusive of the previous value
    InvalidChoiceSetState = 24,
    /// An unknown error occurred
    InternalServerError = 500,
}
impl ExceptionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExceptionType::Unspecified => "EXCEPTION_TYPE_UNSPECIFIED",
            ExceptionType::FieldRequired => "FIELD_REQUIRED",
            ExceptionType::MetamodelAlreadyExists => "METAMODEL_ALREADY_EXISTS",
            ExceptionType::MetamodelNotFound => "METAMODEL_NOT_FOUND",
            ExceptionType::IllegalMetamodelStateTransition => {
                "ILLEGAL_METAMODEL_STATE_TRANSITION"
            }
            ExceptionType::InvalidMetamodelDeprecationPolicy => {
                "INVALID_METAMODEL_DEPRECATION_POLICY"
            }
            ExceptionType::MetamodelDeletionDeniedUntil => {
                "METAMODEL_DELETION_DENIED_UNTIL"
            }
            ExceptionType::InvalidField => "INVALID_FIELD",
            ExceptionType::MetamodelPreconditionFailed => "METAMODEL_PRECONDITION_FAILED",
            ExceptionType::DuplicateFieldKey => "DUPLICATE_FIELD_KEY",
            ExceptionType::IllegalFieldRemoval => "ILLEGAL_FIELD_REMOVAL",
            ExceptionType::IllegalFieldOptionsForField => {
                "ILLEGAL_FIELD_OPTIONS_FOR_FIELD"
            }
            ExceptionType::UnsupportedChangeToPublishedMetamodel => {
                "UNSUPPORTED_CHANGE_TO_PUBLISHED_METAMODEL"
            }
            ExceptionType::IllegalMetamodelStateTransitionInUpdate => {
                "ILLEGAL_METAMODEL_STATE_TRANSITION_IN_UPDATE"
            }
            ExceptionType::PageTokenExpired => "PAGE_TOKEN_EXPIRED",
            ExceptionType::NotAuthorized => "NOT_AUTHORIZED",
            ExceptionType::IllegalFieldStateTransition => {
                "ILLEGAL_FIELD_STATE_TRANSITION"
            }
            ExceptionType::IllegalChoiceSetOptionStateTransition => {
                "ILLEGAL_CHOICE_SET_OPTION_STATE_TRANSITION"
            }
            ExceptionType::InvalidChoiceSetOptions => "INVALID_CHOICE_SET_OPTIONS",
            ExceptionType::InvalidFieldKey => "INVALID_FIELD_KEY",
            ExceptionType::InvalidFieldPropertyRange => "INVALID_FIELD_PROPERTY_RANGE",
            ExceptionType::InvalidLocalizedString => "INVALID_LOCALIZED_STRING",
            ExceptionType::IllegalChangeToPublishedField => {
                "ILLEGAL_CHANGE_TO_PUBLISHED_FIELD"
            }
            ExceptionType::InvalidFieldUpdateNotInclusive => {
                "INVALID_FIELD_UPDATE_NOT_INCLUSIVE"
            }
            ExceptionType::InvalidChoiceSetState => "INVALID_CHOICE_SET_STATE",
            ExceptionType::InternalServerError => "INTERNAL_SERVER_ERROR",
        }
    }
}
/// Defines a field that has a display name, data type, and other
/// configuration options. This field defines the kind of metadata that may be
/// set on a Drive item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    /// Output only. The key of a field, unique within a label or library.
    ///
    /// This value is autogenerated. Matches the regex: `(\[a-zA-Z0-9\])+`
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The key to use when constructing Drive search queries to find
    /// files based on values defined for this field on files.
    /// For example, "`{query_key}` > 2001-01-01".
    #[prost(string, tag = "2")]
    pub query_key: ::prost::alloc::string::String,
    /// The basic properties of the field.
    #[prost(message, optional, tag = "3")]
    pub properties: ::core::option::Option<field::Properties>,
    /// Output only. The lifecycle of this field.
    #[prost(message, optional, tag = "4")]
    pub lifecycle: ::core::option::Option<Lifecycle>,
    /// Output only. UI display hints for rendering a field.
    #[prost(message, optional, tag = "5")]
    pub display_hints: ::core::option::Option<field::DisplayHints>,
    /// Output only. The capabilities this user has when editing this field.
    #[prost(message, optional, tag = "6")]
    pub schema_capabilities: ::core::option::Option<field::SchemaCapabilities>,
    /// Output only. The capabilities this user has on this field and its value
    /// when the label is applied on Drive items.
    #[prost(message, optional, tag = "7")]
    pub applied_capabilities: ::core::option::Option<field::AppliedCapabilities>,
    /// Output only. The user who created this field.
    #[prost(message, optional, tag = "8")]
    pub creator: ::core::option::Option<UserInfo>,
    /// Output only. The time this field was created.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who modified this field.
    #[prost(message, optional, tag = "10")]
    pub updater: ::core::option::Option<UserInfo>,
    /// Output only. The time this field was updated.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who published this field. This value has no meaning
    /// when the field is not published.
    #[prost(message, optional, tag = "12")]
    pub publisher: ::core::option::Option<UserInfo>,
    /// Output only. The user who disabled this field. This value has no meaning
    /// when the field is not disabled.
    #[prost(message, optional, tag = "13")]
    pub disabler: ::core::option::Option<UserInfo>,
    /// Output only. The time this field was disabled. This value has no meaning
    /// when the field is not disabled.
    #[prost(message, optional, tag = "14")]
    pub disable_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The LockStatus of this field.
    #[prost(message, optional, tag = "15")]
    pub lock_status: ::core::option::Option<LockStatus>,
    /// The data type and options of this field.
    /// Once published, the data type cannot be changed.
    #[prost(oneof = "field::Type", tags = "16, 18, 19, 20, 21")]
    pub r#type: ::core::option::Option<field::Type>,
}
/// Nested message and enum types in `Field`.
pub mod field {
    /// The basic properties of the field.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Properties {
        /// Required. The display text to show in the UI identifying this field.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// Whether the field should be marked as required.
        #[prost(bool, tag = "2")]
        pub required: bool,
        /// Input only. Insert or move this field before the indicated field. If
        /// empty, the field is placed at the end of the list.
        #[prost(string, tag = "3")]
        pub insert_before_field: ::prost::alloc::string::String,
    }
    /// UI display hints for rendering a field.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisplayHints {
        /// Whether the field should be shown as required in the UI.
        #[prost(bool, tag = "1")]
        pub required: bool,
        /// Whether the field should be shown in the UI as disabled.
        #[prost(bool, tag = "2")]
        pub disabled: bool,
        /// This field should be hidden in the search menu when searching for Drive
        /// items.
        #[prost(bool, tag = "3")]
        pub hidden_in_search: bool,
        /// This field should be shown in the apply menu when applying values to a
        /// Drive item.
        #[prost(bool, tag = "4")]
        pub shown_in_apply: bool,
    }
    /// The capabilities related to this field when editing the field.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SchemaCapabilities {
        /// Whether the user can change this field.
        #[prost(bool, tag = "1")]
        pub can_update: bool,
        /// Whether the user can delete this field.
        /// The user must have permission and the field must be deprecated.
        #[prost(bool, tag = "2")]
        pub can_delete: bool,
        /// Whether the user can disable this field.
        /// The user must have permission and this field must not already be
        /// disabled.
        #[prost(bool, tag = "3")]
        pub can_disable: bool,
        /// Whether the user can enable this field.
        /// The user must have permission and this field must be disabled.
        #[prost(bool, tag = "4")]
        pub can_enable: bool,
    }
    /// The capabilities related to this field on applied metadata.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppliedCapabilities {
        /// Whether the user can read related applied metadata on items.
        #[prost(bool, tag = "1")]
        pub can_read: bool,
        /// Whether the user can search for Drive items referencing this field.
        #[prost(bool, tag = "2")]
        pub can_search: bool,
        /// Whether the user can set this field on Drive items.
        #[prost(bool, tag = "3")]
        pub can_write: bool,
    }
    /// Options for a multi-valued variant of an associated field type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListOptions {
        /// Maximum number of entries permitted.
        #[prost(int32, tag = "1")]
        pub max_entries: i32,
    }
    /// Options for the Text field type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextOptions {
        /// Output only. The minimum valid length of values for the text field.
        #[prost(int32, tag = "1")]
        pub min_length: i32,
        /// Output only. The maximum valid length of values for the text field.
        #[prost(int32, tag = "2")]
        pub max_length: i32,
    }
    /// Options for the Integer field type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IntegerOptions {
        /// Output only. The minimum valid value for the integer field.
        #[prost(int64, tag = "1")]
        pub min_value: i64,
        /// Output only. The maximum valid value for the integer field.
        #[prost(int64, tag = "2")]
        pub max_value: i64,
    }
    /// Options for the date field type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DateOptions {
        /// Localized date formatting option. Field values are rendered in
        /// this format according to their locale.
        #[prost(enumeration = "date_options::DateFormat", tag = "1")]
        pub date_format_type: i32,
        /// Output only. ICU date format.
        #[prost(string, tag = "2")]
        pub date_format: ::prost::alloc::string::String,
        /// Output only. Minimum valid value (year, month, day).
        #[prost(message, optional, tag = "3")]
        pub min_value: ::core::option::Option<
            super::super::super::super::super::r#type::Date,
        >,
        /// Output only. Maximum valid value (year, month, day).
        #[prost(message, optional, tag = "4")]
        pub max_value: ::core::option::Option<
            super::super::super::super::super::r#type::Date,
        >,
    }
    /// Nested message and enum types in `DateOptions`.
    pub mod date_options {
        /// Localized date format options.
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
        pub enum DateFormat {
            /// Date format unspecified.
            Unspecified = 0,
            /// Includes full month name.
            /// For example, January 12, 1999
            /// (MMMM d, y)
            LongDate = 1,
            /// Short, numeric, representation.
            /// For example, 12/13/99
            /// (M/d/yy)
            ShortDate = 2,
        }
        impl DateFormat {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    DateFormat::Unspecified => "DATE_FORMAT_UNSPECIFIED",
                    DateFormat::LongDate => "LONG_DATE",
                    DateFormat::ShortDate => "SHORT_DATE",
                }
            }
        }
    }
    /// Options for the selection field type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SelectionOptions {
        /// When specified, indicates this field supports a list of values.
        /// Once the field is published, this cannot be changed.
        #[prost(message, optional, tag = "1")]
        pub list_options: ::core::option::Option<ListOptions>,
        /// The options available for this selection field.
        /// The list order is consistent, and modified with `insert_before_choice`.
        #[prost(message, repeated, tag = "2")]
        pub choices: ::prost::alloc::vec::Vec<selection_options::Choice>,
    }
    /// Nested message and enum types in `SelectionOptions`.
    pub mod selection_options {
        /// Selection field choice.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Choice {
            /// The unique value of the choice.
            /// This ID is autogenerated. Matches the regex: `(\[a-zA-Z0-9_\])+`.
            #[prost(string, tag = "1")]
            pub id: ::prost::alloc::string::String,
            /// Basic properties of the choice.
            #[prost(message, optional, tag = "2")]
            pub properties: ::core::option::Option<choice::Properties>,
            /// Output only. Lifecycle of the choice.
            #[prost(message, optional, tag = "3")]
            pub lifecycle: ::core::option::Option<super::super::Lifecycle>,
            /// Output only. UI display hints for rendering a choice.
            #[prost(message, optional, tag = "4")]
            pub display_hints: ::core::option::Option<choice::DisplayHints>,
            /// Output only. The capabilities related to this option when editing the
            /// option.
            #[prost(message, optional, tag = "5")]
            pub schema_capabilities: ::core::option::Option<choice::SchemaCapabilities>,
            /// Output only. The capabilities related to this choice on applied
            /// metadata.
            #[prost(message, optional, tag = "6")]
            pub applied_capabilities: ::core::option::Option<
                choice::AppliedCapabilities,
            >,
            /// Output only. The user who created this choice.
            #[prost(message, optional, tag = "7")]
            pub creator: ::core::option::Option<super::super::UserInfo>,
            /// Output only. The time this choice was created.
            #[prost(message, optional, tag = "8")]
            pub create_time: ::core::option::Option<::prost_types::Timestamp>,
            /// Output only. The user who updated this choice last.
            #[prost(message, optional, tag = "9")]
            pub updater: ::core::option::Option<super::super::UserInfo>,
            /// Output only. The time this choice was updated last.
            #[prost(message, optional, tag = "10")]
            pub update_time: ::core::option::Option<::prost_types::Timestamp>,
            /// Output only. The user who published this choice. This value has no
            /// meaning when the choice is not published.
            #[prost(message, optional, tag = "11")]
            pub publisher: ::core::option::Option<super::super::UserInfo>,
            /// Output only. The time this choice was published. This value has no
            /// meaning when the choice is not published.
            #[prost(message, optional, tag = "12")]
            pub publish_time: ::core::option::Option<::prost_types::Timestamp>,
            /// Output only. The user who disabled this choice. This value has no
            /// meaning when the option is not disabled.
            #[prost(message, optional, tag = "13")]
            pub disabler: ::core::option::Option<super::super::UserInfo>,
            /// Output only. The time this choice was disabled. This value has no
            /// meaning when the choice is not disabled.
            #[prost(message, optional, tag = "14")]
            pub disable_time: ::core::option::Option<::prost_types::Timestamp>,
            /// Output only. The LockStatus of this choice.
            #[prost(message, optional, tag = "15")]
            pub lock_status: ::core::option::Option<super::super::LockStatus>,
        }
        /// Nested message and enum types in `Choice`.
        pub mod choice {
            /// Basic properties of the choice.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Properties {
                /// Required. The display text to show in the UI identifying this field.
                #[prost(string, tag = "1")]
                pub display_name: ::prost::alloc::string::String,
                /// The description of this label.
                #[prost(string, tag = "2")]
                pub description: ::prost::alloc::string::String,
                /// The badge configuration for this choice. When set, the
                /// label that owns this choice is considered a "badged label".
                #[prost(message, optional, tag = "3")]
                pub badge_config: ::core::option::Option<
                    super::super::super::BadgeConfig,
                >,
                /// Input only. Insert or move this choice before the indicated choice.
                /// If empty, the choice is placed at the end of the list.
                #[prost(string, tag = "4")]
                pub insert_before_choice: ::prost::alloc::string::String,
            }
            /// UI display hints for rendering an option.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DisplayHints {
                /// Whether the option should be shown in the UI as disabled.
                #[prost(bool, tag = "1")]
                pub disabled: bool,
                /// This option should be hidden in the search menu when searching for
                /// Drive items.
                #[prost(bool, tag = "2")]
                pub hidden_in_search: bool,
                /// This option should be shown in the apply menu when applying values to
                /// a Drive item.
                #[prost(bool, tag = "3")]
                pub shown_in_apply: bool,
                /// The colors to use for the badge. Changed to Google Material colors
                /// based on the chosen `properties.badge_config.color`.
                #[prost(message, optional, tag = "4")]
                pub badge_colors: ::core::option::Option<
                    super::super::super::BadgeColors,
                >,
                /// The dark-mode color to use for the badge. Changed to Google Material
                /// colors based on the chosen `properties.badge_config.color`.
                #[prost(message, optional, tag = "5")]
                pub dark_badge_colors: ::core::option::Option<
                    super::super::super::BadgeColors,
                >,
                /// The priority of this badge. Used to compare and sort between multiple
                /// badges. A lower number means the badge should be shown first.
                /// When a badging configuration is not present, this will be 0.
                /// Otherwise, this will be set to `BadgeConfig.priority_override` or the
                /// default heuristic which prefers creation date of the label, and field
                /// and option priority.
                #[prost(int64, tag = "6")]
                pub badge_priority: i64,
            }
            /// The capabilities related to this choice when editing the choice.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct SchemaCapabilities {
                /// Whether the user can update this choice.
                #[prost(bool, tag = "1")]
                pub can_update: bool,
                /// Whether the user can delete this choice.
                #[prost(bool, tag = "2")]
                pub can_delete: bool,
                /// Whether the user can disable this choice.
                #[prost(bool, tag = "3")]
                pub can_disable: bool,
                /// Whether the user can enable this choice.
                #[prost(bool, tag = "4")]
                pub can_enable: bool,
            }
            /// The capabilities related to this choice on applied metadata.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct AppliedCapabilities {
                /// Whether the user can read related applied metadata on items.
                #[prost(bool, tag = "1")]
                pub can_read: bool,
                /// Whether the user can use this choice in search queries.
                #[prost(bool, tag = "2")]
                pub can_search: bool,
                /// Whether the user can select this choice on an item.
                #[prost(bool, tag = "3")]
                pub can_select: bool,
            }
        }
    }
    /// Options for the user field type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserOptions {
        /// When specified, indicates that this field supports a list of values.
        /// Once the field is published, this cannot be changed.
        #[prost(message, optional, tag = "1")]
        pub list_options: ::core::option::Option<ListOptions>,
    }
    /// The data type and options of this field.
    /// Once published, the data type cannot be changed.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Text field options.
        #[prost(message, tag = "16")]
        TextOptions(TextOptions),
        /// Integer field options.
        #[prost(message, tag = "18")]
        IntegerOptions(IntegerOptions),
        /// Date field options.
        #[prost(message, tag = "19")]
        DateOptions(DateOptions),
        /// Selection field options.
        #[prost(message, tag = "20")]
        SelectionOptions(SelectionOptions),
        /// User field options.
        #[prost(message, tag = "21")]
        UserOptions(UserOptions),
    }
}
/// A label defines a taxonomy that can be applied to Drive items in order to
/// organize and search across items. Labels can be simple strings, or can
/// contain fields that describe additional metadata that can be further used to
/// organize and search Drive items.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    /// Output only. Resource name of the label. Will be in the form of either:
    /// `labels/{id}` or `labels/{id}@{revision_id}` depending on the request.
    /// See `id` and `revision_id` below.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Globally unique identifier of this label. ID makes up part of
    /// the label `name`, but unlike `name`, ID is consistent between revisions.
    /// Matches the regex: `(\[a-zA-Z0-9\])+`
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Revision ID of the label. Revision ID might be part of the
    /// label `name` depending on the request issued. A new revision is created
    /// whenever revisioned properties of a label are changed. Matches the regex:
    /// `(\[a-zA-Z0-9\])+`
    #[prost(string, tag = "3")]
    pub revision_id: ::prost::alloc::string::String,
    /// Required. The type of label.
    #[prost(enumeration = "label::LabelType", tag = "4")]
    pub label_type: i32,
    /// Output only. The user who created this label.
    #[prost(message, optional, tag = "5")]
    pub creator: ::core::option::Option<UserInfo>,
    /// Output only. The time this label was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who created this label revision.
    #[prost(message, optional, tag = "7")]
    pub revision_creator: ::core::option::Option<UserInfo>,
    /// Output only. The time this label revision was created.
    #[prost(message, optional, tag = "8")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who published this label.  This value has no meaning
    /// when the label is not published.
    #[prost(message, optional, tag = "9")]
    pub publisher: ::core::option::Option<UserInfo>,
    /// Output only. The time this label was published. This value has no meaning
    /// when the label is not published.
    #[prost(message, optional, tag = "10")]
    pub publish_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The user who disabled this label. This value has no meaning
    /// when the label is not disabled.
    #[prost(message, optional, tag = "11")]
    pub disabler: ::core::option::Option<UserInfo>,
    /// Output only. The time this label was disabled. This value has no meaning
    /// when the label is not disabled.
    #[prost(message, optional, tag = "12")]
    pub disable_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The basic properties of the label.
    #[prost(message, optional, tag = "14")]
    pub properties: ::core::option::Option<label::Properties>,
    /// Output only. The lifecycle state of the label including whether it's
    /// published, deprecated, and has draft changes.
    #[prost(message, optional, tag = "15")]
    pub lifecycle: ::core::option::Option<Lifecycle>,
    /// Output only. UI display hints for rendering the label.
    #[prost(message, optional, tag = "16")]
    pub display_hints: ::core::option::Option<label::DisplayHints>,
    /// Output only. The capabilities related to this label on applied metadata.
    #[prost(message, optional, tag = "17")]
    pub applied_capabilities: ::core::option::Option<label::AppliedCapabilities>,
    /// Output only. The capabilities the user has on this label.
    #[prost(message, optional, tag = "18")]
    pub schema_capabilities: ::core::option::Option<label::SchemaCapabilities>,
    /// Output only. Behavior of this label when it's applied to Drive items.
    #[prost(message, optional, tag = "19")]
    pub applied_label_policy: ::core::option::Option<label::AppliedLabelPolicy>,
    /// List of fields in descending priority order.
    #[prost(message, repeated, tag = "20")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    /// Custom URL to present to users to allow them to learn more about this label
    /// and how it should be used.
    #[prost(string, tag = "21")]
    pub learn_more_uri: ::prost::alloc::string::String,
    /// Output only. The LockStatus of this label.
    #[prost(message, optional, tag = "22")]
    pub lock_status: ::core::option::Option<LockStatus>,
}
/// Nested message and enum types in `Label`.
pub mod label {
    /// Basic properties of the label.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Properties {
        /// Required. Title of the label.
        #[prost(string, tag = "1")]
        pub title: ::prost::alloc::string::String,
        /// The description of the label.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
    }
    /// UI display hints for rendering the label.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisplayHints {
        /// Whether the label should be shown in the UI as disabled.
        #[prost(bool, tag = "1")]
        pub disabled: bool,
        /// This label should be hidden in the search menu when searching for Drive
        /// items.
        #[prost(bool, tag = "2")]
        pub hidden_in_search: bool,
        /// This label should be shown in the apply menu when applying values to a
        /// Drive item.
        #[prost(bool, tag = "3")]
        pub shown_in_apply: bool,
        /// Order to display label in a list.
        #[prost(int64, tag = "4")]
        pub priority: i64,
    }
    /// The capabilities a user has on this label's applied metadata.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppliedCapabilities {
        /// Whether the user can read applied metadata related to this label.
        #[prost(bool, tag = "1")]
        pub can_read: bool,
        /// Whether the user can apply this label to items.
        #[prost(bool, tag = "2")]
        pub can_apply: bool,
        /// Whether the user can remove this label from items.
        #[prost(bool, tag = "3")]
        pub can_remove: bool,
    }
    /// The capabilities related to this label when editing the label.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SchemaCapabilities {
        /// Whether the user can change this label.
        #[prost(bool, tag = "1")]
        pub can_update: bool,
        /// Whether the user can delete this label.
        /// The user must have permission and the label must be disabled.
        #[prost(bool, tag = "2")]
        pub can_delete: bool,
        /// Whether the user can disable this label.
        /// The user must have permission and this label must not already be
        /// disabled.
        #[prost(bool, tag = "3")]
        pub can_disable: bool,
        /// Whether the user can enable this label.
        /// The user must have permission and this label must be disabled.
        #[prost(bool, tag = "4")]
        pub can_enable: bool,
    }
    /// Behavior of this label when it's applied to Drive items.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppliedLabelPolicy {
        /// Indicates how the applied label and field values should be copied when
        /// a Drive item is copied.
        #[prost(enumeration = "applied_label_policy::CopyMode", tag = "1")]
        pub copy_mode: i32,
    }
    /// Nested message and enum types in `AppliedLabelPolicy`.
    pub mod applied_label_policy {
        /// Indicates how the applied label and field values should be copied when
        /// a Drive item is copied.
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
        pub enum CopyMode {
            /// Copy mode unspecified.
            Unspecified = 0,
            /// The applied label and field values are not copied by default when
            /// the Drive item it's applied to is copied.
            DoNotCopy = 1,
            /// The applied label and field values are always copied when the
            /// Drive item it's applied to is copied.
            /// Only admins can use this mode.
            AlwaysCopy = 2,
            /// The applied label and field values are copied if the
            /// label is appliable by the user making the copy.
            CopyAppliable = 3,
        }
        impl CopyMode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    CopyMode::Unspecified => "COPY_MODE_UNSPECIFIED",
                    CopyMode::DoNotCopy => "DO_NOT_COPY",
                    CopyMode::AlwaysCopy => "ALWAYS_COPY",
                    CopyMode::CopyAppliable => "COPY_APPLIABLE",
                }
            }
        }
    }
    /// The type of this label.
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
    pub enum LabelType {
        /// Unknown label type.
        Unspecified = 0,
        /// Shared labels may be shared with users to apply to Drive items.
        Shared = 1,
        /// Admin-owned label. Only creatable and editable by admins. Supports some
        /// additional admin-only features.
        Admin = 2,
    }
    impl LabelType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LabelType::Unspecified => "LABEL_TYPE_UNSPECIFIED",
                LabelType::Shared => "SHARED",
                LabelType::Admin => "ADMIN",
            }
        }
    }
}
/// The permission that applies to a principal (user, group, audience) on a
/// label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelPermission {}
/// Nested message and enum types in `LabelPermission`.
pub mod label_permission {
    /// Roles are concentric with subsequent role.
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
    pub enum LabelRole {
        /// Unknown role.
        Unspecified = 0,
        /// A reader can read the label and associated metadata applied to Drive
        /// items.
        Reader = 1,
        /// An applier can write associated metadata on Drive items in which they
        /// also have write access to. Implies `READER`.
        Applier = 2,
        /// An organizer can pin this label in shared drives they manage
        /// and add new appliers to the label.
        Organizer = 3,
        /// Editors can make any update including deleting the label which
        /// also deletes the associated Drive item metadata. Implies `APPLIER`.
        Editor = 4,
    }
    impl LabelRole {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LabelRole::Unspecified => "LABEL_ROLE_UNSPECIFIED",
                LabelRole::Reader => "READER",
                LabelRole::Applier => "APPLIER",
                LabelRole::Organizer => "ORGANIZER",
                LabelRole::Editor => "EDITOR",
            }
        }
    }
}
/// Request to get a label by resource name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLabelRequest {
    /// Required. Label resource name.
    ///
    /// May be any of:
    ///
    /// * `labels/{id}` (equivalent to labels/{id}@latest)
    /// * `labels/{id}@latest`
    /// * `labels/{id}@published`
    /// * `labels/{id}@{revision_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Set to `true` in order to use the user's admin credentials. The server
    /// verifies that the user is an admin for the label before allowing access.
    #[prost(bool, tag = "2")]
    pub use_admin_access: bool,
    /// The BCP-47 language code to use for evaluating localized field labels.
    /// When not specified, values in the default configured language are used.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
    /// When specified, only certain fields belonging to the indicated view are
    /// returned.
    #[prost(enumeration = "LabelView", tag = "4")]
    pub view: i32,
}
/// Request to list labels available to the current user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLabelsRequest {
    /// Whether to include only published labels in the results.
    ///
    /// * When `true`, only the current published label revisions are returned.
    ///    Disabled labels are included. Returned label resource names
    ///    reference the published revision (`labels/{id}/{revision_id}`).
    /// * When `false`, the current label revisions are returned, which might not
    ///    be published. Returned label resource names don't reference a specific
    ///    revision (`labels/{id}`).
    #[prost(bool, tag = "1")]
    pub published_only: bool,
    /// The BCP-47 language code to use for evaluating localized field labels.
    /// When not specified, values in the default configured language are used.
    #[prost(string, tag = "5")]
    pub language_code: ::prost::alloc::string::String,
    /// Maximum number of labels to return per page. Default: 50. Max: 200.
    #[prost(int32, tag = "6")]
    pub page_size: i32,
    /// The token of the page to return.
    #[prost(string, tag = "7")]
    pub page_token: ::prost::alloc::string::String,
    /// When specified, only certain fields belonging to the indicated view are
    /// returned.
    #[prost(enumeration = "LabelView", tag = "8")]
    pub view: i32,
    #[prost(oneof = "list_labels_request::Access", tags = "3, 4")]
    pub access: ::core::option::Option<list_labels_request::Access>,
}
/// Nested message and enum types in `ListLabelsRequest`.
pub mod list_labels_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Access {
        /// Set to `true` in order to use the user's admin credentials. This will
        /// return all Labels within the customer.
        #[prost(bool, tag = "3")]
        UseAdminAccess(bool),
        /// Specifies the level of access the user must have on the returned Labels.
        /// The minimum role a user must have on a label.
        /// Defaults to `READER`.
        #[prost(enumeration = "super::label_permission::LabelRole", tag = "4")]
        MinimumRole(i32),
    }
}
/// Response for listing Labels.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLabelsResponse {
    /// Labels.
    #[prost(message, repeated, tag = "1")]
    pub labels: ::prost::alloc::vec::Vec<Label>,
    /// The token of the next page in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Resource view that can be applied to label responses. The default value
/// `LABEL_VIEW_BASIC` implies the field mask:
/// `name,id,revision_id,label_type,properties.*`\
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LabelView {
    /// Implies the field mask:
    /// `name,id,revision_id,label_type,properties.*`
    Basic = 0,
    /// All possible fields.
    Full = 1,
}
impl LabelView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LabelView::Basic => "LABEL_VIEW_BASIC",
            LabelView::Full => "LABEL_VIEW_FULL",
        }
    }
}
/// Generated client implementations.
pub mod label_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Manage metadata taxonomies based on Labels and Fields that may be used within
    /// Google Drive to organize and find files using custom metadata.
    #[derive(Debug, Clone)]
    pub struct LabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LabelServiceClient<tonic::transport::Channel> {
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
    impl<T> LabelServiceClient<T>
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
        ) -> LabelServiceClient<InterceptedService<T, F>>
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
            LabelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// List labels.
        pub async fn list_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLabelsRequest>,
        ) -> Result<tonic::Response<super::ListLabelsResponse>, tonic::Status> {
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
                "/google.apps.drive.labels.v2.LabelService/ListLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get a label by its resource name.
        /// Resource name may be any of:
        ///
        /// * `labels/{id}` - See `labels/{id}@latest`
        /// * `labels/{id}@latest` - Gets the latest revision of the label.
        /// * `labels/{id}@published` - Gets the current published revision of the
        ///   label.
        /// * `labels/{id}@{revision_id}` - Gets the label at the specified revision
        ///   ID.
        pub async fn get_label(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLabelRequest>,
        ) -> Result<tonic::Response<super::Label>, tonic::Status> {
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
                "/google.apps.drive.labels.v2.LabelService/GetLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
