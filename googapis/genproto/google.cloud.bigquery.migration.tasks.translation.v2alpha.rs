#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetReference {
    /// Required. A unique ID for this dataset, without the project name. The ID
    /// must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
    /// The maximum length is 1,024 characters.
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Optional. The ID of the project containing this dataset.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Mapping between an input and output file to be translated in a subtask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationFileMapping {
    /// The Cloud Storage path for a file to translation in a subtask.
    #[prost(string, tag = "1")]
    pub input_path: ::prost::alloc::string::String,
    /// The Cloud Storage path to write back the corresponding input file to.
    #[prost(string, tag = "2")]
    pub output_path: ::prost::alloc::string::String,
}
/// The translation task details to capture necessary settings for a translation
/// task and subtask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslationTaskDetails {
    /// The Cloud Storage path for translation input files.
    #[prost(string, tag = "1")]
    pub input_path: ::prost::alloc::string::String,
    /// The Cloud Storage path for translation output files.
    #[prost(string, tag = "2")]
    pub output_path: ::prost::alloc::string::String,
    /// Cloud Storage files to be processed for translation.
    #[prost(message, repeated, tag = "12")]
    pub file_paths: ::prost::alloc::vec::Vec<TranslationFileMapping>,
    /// The Cloud Storage path to DDL files as table schema to assist semantic
    /// translation.
    #[prost(string, tag = "3")]
    pub schema_path: ::prost::alloc::string::String,
    /// The file encoding type.
    #[prost(enumeration = "translation_task_details::FileEncoding", tag = "4")]
    pub file_encoding: i32,
    /// The settings for SQL identifiers.
    #[prost(message, optional, tag = "5")]
    pub identifier_settings: ::core::option::Option<IdentifierSettings>,
    /// The map capturing special tokens to be replaced during translation. The key
    /// is special token in string. The value is the token data type. This is used
    /// to translate SQL query template which contains special token as place
    /// holder. The special token makes a query invalid to parse. This map will be
    /// applied to annotate those special token with types to let parser understand
    /// how to parse them into proper structure with type information.
    #[prost(map = "string, enumeration(translation_task_details::TokenType)", tag = "6")]
    pub special_token_map: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    /// The filter applied to translation details.
    #[prost(message, optional, tag = "7")]
    pub filter: ::core::option::Option<Filter>,
    /// Specifies the exact name of the bigquery table ("dataset.table") to be used
    /// for surfacing raw translation errors. If the table does not exist, we will
    /// create it. If it already exists and the schema is the same, we will re-use.
    /// If the table exists and the schema is different, we will throw an error.
    #[prost(string, tag = "13")]
    pub translation_exception_table: ::prost::alloc::string::String,
    /// The language specific settings for the translation task.
    #[prost(oneof = "translation_task_details::LanguageOptions", tags = "10, 11")]
    pub language_options: ::core::option::Option<translation_task_details::LanguageOptions>,
}
/// Nested message and enum types in `TranslationTaskDetails`.
pub mod translation_task_details {
    /// The file encoding types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FileEncoding {
        /// File encoding setting is not specified.
        Unspecified = 0,
        /// File encoding is UTF_8.
        Utf8 = 1,
        /// File encoding is ISO_8859_1.
        Iso88591 = 2,
        /// File encoding is US_ASCII.
        UsAscii = 3,
        /// File encoding is UTF_16.
        Utf16 = 4,
        /// File encoding is UTF_16LE.
        Utf16le = 5,
        /// File encoding is UTF_16BE.
        Utf16be = 6,
    }
    /// The special token data type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TokenType {
        /// Token type is not specified.
        Unspecified = 0,
        /// Token type as string.
        String = 1,
        /// Token type as integer.
        Int64 = 2,
        /// Token type as numeric.
        Numeric = 3,
        /// Token type as boolean.
        Bool = 4,
        /// Token type as float.
        Float64 = 5,
        /// Token type as date.
        Date = 6,
        /// Token type as timestamp.
        Timestamp = 7,
    }
    /// The language specific settings for the translation task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LanguageOptions {
        /// The Teradata SQL specific settings for the translation task.
        #[prost(message, tag = "10")]
        TeradataOptions(super::TeradataOptions),
        /// The BTEQ specific settings for the translation task.
        #[prost(message, tag = "11")]
        BteqOptions(super::BteqOptions),
    }
}
/// The filter applied to fields of translation details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// The list of prefixes used to exclude processing for input files.
    #[prost(string, repeated, tag = "1")]
    pub input_file_exclusion_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Settings related to SQL identifiers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifierSettings {
    /// The setting to control output queries' identifier case.
    #[prost(enumeration = "identifier_settings::IdentifierCase", tag = "1")]
    pub output_identifier_case: i32,
    /// Specifies the rewrite mode for SQL identifiers.
    #[prost(enumeration = "identifier_settings::IdentifierRewriteMode", tag = "2")]
    pub identifier_rewrite_mode: i32,
}
/// Nested message and enum types in `IdentifierSettings`.
pub mod identifier_settings {
    /// The identifier case type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IdentifierCase {
        /// The identifier case is not specified.
        Unspecified = 0,
        /// Identifiers' cases will be kept as the original cases.
        Original = 1,
        /// Identifiers will be in upper cases.
        Upper = 2,
        /// Identifiers will be in lower cases.
        Lower = 3,
    }
    /// The SQL identifier rewrite mode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IdentifierRewriteMode {
        /// SQL Identifier rewrite mode is unspecified.
        Unspecified = 0,
        /// SQL identifiers won't be rewrite.
        None = 1,
        /// All SQL identifiers will be rewrite.
        RewriteAll = 2,
    }
}
/// Teradata SQL specific translation task related settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeradataOptions {}
/// BTEQ translation task related settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BteqOptions {
    /// Specifies the project and dataset in BigQuery that will be used for
    /// external table creation during the translation.
    #[prost(message, optional, tag = "1")]
    pub project_dataset: ::core::option::Option<DatasetReference>,
    /// The Cloud Storage location to be used as the default path for files that
    /// are not otherwise specified in the file replacement map.
    #[prost(string, tag = "2")]
    pub default_path_uri: ::prost::alloc::string::String,
    /// Maps the local paths that are used in BTEQ scripts (the keys) to the paths
    /// in Cloud Storage that should be used in their stead in the translation (the
    /// value).
    #[prost(map = "string, string", tag = "3")]
    pub file_replacement_map:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
