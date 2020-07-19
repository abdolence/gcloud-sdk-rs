/// This enum describes all the possible systems that Data Catalog integrates
/// with.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IntegratedSystem {
    /// Default unknown system.
    Unspecified = 0,
    /// BigQuery.
    Bigquery = 1,
    /// Cloud Pub/Sub.
    CloudPubsub = 2,
}
/// Timestamps about this resource according to a particular system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemTimestamps {
    /// The creation time of the resource within the given system.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The last-modified time of the resource within the given system.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The expiration time of the resource within the given system.
    /// Currently only apllicable to BigQuery resources.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Describes a Cloud Storage fileset entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsFilesetSpec {
    /// Required. Patterns to identify a set of files in Google Cloud Storage.
    /// See [Cloud Storage
    /// documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames)
    /// for more information. Note that bucket wildcards are currently not
    /// supported.
    ///
    /// Examples of valid file_patterns:
    ///
    ///  * `gs://bucket_name/dir/*`: matches all files within `bucket_name/dir`
    ///                              directory.
    ///  * `gs://bucket_name/dir/**`: matches all files in `bucket_name/dir`
    ///                               spanning all subdirectories.
    ///  * `gs://bucket_name/file*`: matches files prefixed by `file` in
    ///                              `bucket_name`
    ///  * `gs://bucket_name/??.txt`: matches files with two characters followed by
    ///                               `.txt` in `bucket_name`
    ///  * `gs://bucket_name/[aeiou].txt`: matches files that contain a single
    ///                                    vowel character followed by `.txt` in
    ///                                    `bucket_name`
    ///  * `gs://bucket_name/[a-m].txt`: matches files that contain `a`, `b`, ...
    ///                                  or `m` followed by `.txt` in `bucket_name`
    ///  * `gs://bucket_name/a/*/b`: matches all files in `bucket_name` that match
    ///                              `a/*/b` pattern, such as `a/c/b`, `a/d/b`
    ///  * `gs://another_bucket/a.txt`: matches `gs://another_bucket/a.txt`
    ///
    /// You can combine wildcards to provide more powerful matches, for example:
    ///
    ///  * `gs://bucket_name/[a-m]??.j*g`
    #[prost(string, repeated, tag = "1")]
    pub file_patterns: ::std::vec::Vec<std::string::String>,
    /// Output only. Sample files contained in this fileset, not all files
    /// contained in this fileset are represented here.
    #[prost(message, repeated, tag = "2")]
    pub sample_gcs_file_specs: ::std::vec::Vec<GcsFileSpec>,
}
/// Specifications of a single file in Cloud Storage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsFileSpec {
    /// Required. The full file path. Example: `gs://bucket_name/a/b.txt`.
    #[prost(string, tag = "1")]
    pub file_path: std::string::String,
    /// Output only. Timestamps about the Cloud Storage file.
    #[prost(message, optional, tag = "2")]
    pub gcs_timestamps: ::std::option::Option<SystemTimestamps>,
    /// Output only. The size of the file, in bytes.
    #[prost(int64, tag = "4")]
    pub size_bytes: i64,
}
/// Represents a schema (e.g. BigQuery, GoogleSQL, Avro schema).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    /// Required. Schema of columns. A maximum of 10,000 columns and sub-columns can be
    /// specified.
    #[prost(message, repeated, tag = "2")]
    pub columns: ::std::vec::Vec<ColumnSchema>,
}
/// Representation of a column within a schema. Columns could be nested inside
/// other columns.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnSchema {
    /// Required. Name of the column.
    #[prost(string, tag = "6")]
    pub column: std::string::String,
    /// Required. Type of the column.
    #[prost(string, tag = "1")]
    pub r#type: std::string::String,
    /// Optional. Description of the column. Default value is an empty string.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Optional. A column's mode indicates whether the values in this column are required,
    /// nullable, etc. Only `NULLABLE`, `REQUIRED` and `REPEATED` are supported.
    /// Default mode is `NULLABLE`.
    #[prost(string, tag = "3")]
    pub mode: std::string::String,
    /// Optional. Schema of sub-columns. A column can have zero or more sub-columns.
    #[prost(message, repeated, tag = "7")]
    pub subcolumns: ::std::vec::Vec<ColumnSchema>,
}
/// A result that appears in the response of a search request. Each result
/// captures details of one entry that matches the search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogResult {
    /// Type of the search result. This field can be used to determine which Get
    /// method to call to fetch the full resource.
    #[prost(enumeration = "SearchResultType", tag = "1")]
    pub search_result_type: i32,
    /// Sub-type of the search result. This is a dot-delimited description of the
    /// resource's full type, and is the same as the value callers would provide in
    /// the "type" search facet.  Examples: `entry.table`, `entry.dataStream`,
    /// `tagTemplate`.
    #[prost(string, tag = "2")]
    pub search_result_subtype: std::string::String,
    /// The relative resource name of the resource in URL format.
    /// Examples:
    ///
    ///  * `projects/{project_id}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}`
    ///  * `projects/{project_id}/tagTemplates/{tag_template_id}`
    #[prost(string, tag = "3")]
    pub relative_resource_name: std::string::String,
    /// The full name of the cloud resource the entry belongs to. See:
    /// https://cloud.google.com/apis/design/resource_names#full_resource_name.
    /// Example:
    ///
    ///  * `//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId`
    #[prost(string, tag = "4")]
    pub linked_resource: std::string::String,
    /// The source system of the entry. Only applicable when `search_result_type`
    /// is ENTRY.
    #[prost(oneof = "search_catalog_result::System", tags = "8, 9")]
    pub system: ::std::option::Option<search_catalog_result::System>,
}
pub mod search_catalog_result {
    /// The source system of the entry. Only applicable when `search_result_type`
    /// is ENTRY.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum System {
        /// Output only. This field indicates the entry's source system that Data Catalog
        /// integrates with, such as BigQuery or Cloud Pub/Sub.
        #[prost(enumeration = "super::IntegratedSystem", tag = "8")]
        IntegratedSystem(i32),
        /// This field indicates the entry's source system that Data Catalog does not
        /// integrate with.
        #[prost(string, tag = "9")]
        UserSpecifiedSystem(std::string::String),
    }
}
/// The different types of resources that can be returned in search.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SearchResultType {
    /// Default unknown type.
    Unspecified = 0,
    /// An [Entry][google.cloud.datacatalog.v1.Entry].
    Entry = 1,
    /// A [TagTemplate][google.cloud.datacatalog.v1.TagTemplate].
    TagTemplate = 2,
    /// An [EntryGroup][google.cloud.datacatalog.v1.EntryGroup].
    EntryGroup = 3,
}
/// Describes a BigQuery table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryTableSpec {
    /// Output only. The table source type.
    #[prost(enumeration = "TableSourceType", tag = "1")]
    pub table_source_type: i32,
    /// Output only.
    #[prost(oneof = "big_query_table_spec::TypeSpec", tags = "2, 3")]
    pub type_spec: ::std::option::Option<big_query_table_spec::TypeSpec>,
}
pub mod big_query_table_spec {
    /// Output only.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeSpec {
        /// Table view specification. This field should only be populated if
        /// `table_source_type` is `BIGQUERY_VIEW`.
        #[prost(message, tag = "2")]
        ViewSpec(super::ViewSpec),
        /// Spec of a BigQuery table. This field should only be populated if
        /// `table_source_type` is `BIGQUERY_TABLE`.
        #[prost(message, tag = "3")]
        TableSpec(super::TableSpec),
    }
}
/// Table view specification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewSpec {
    /// Output only. The query that defines the table view.
    #[prost(string, tag = "1")]
    pub view_query: std::string::String,
}
/// Normal BigQuery table spec.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSpec {
    /// Output only. If the table is a dated shard, i.e., with name pattern `[prefix]YYYYMMDD`,
    /// `grouped_entry` is the Data Catalog resource name of the date sharded
    /// grouped entry, for example,
    /// `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`.
    /// Otherwise, `grouped_entry` is empty.
    #[prost(string, tag = "1")]
    pub grouped_entry: std::string::String,
}
/// Spec for a group of BigQuery tables with name pattern `[prefix]YYYYMMDD`.
/// Context:
/// https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDateShardedSpec {
    /// Output only. The Data Catalog resource name of the dataset entry the current table
    /// belongs to, for example,
    /// `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`.
    #[prost(string, tag = "1")]
    pub dataset: std::string::String,
    /// Output only. The table name prefix of the shards. The name of any given shard is
    /// `[table_prefix]YYYYMMDD`, for example, for shard `MyTable20180101`, the
    /// `table_prefix` is `MyTable`.
    #[prost(string, tag = "2")]
    pub table_prefix: std::string::String,
    /// Output only. Total number of shards.
    #[prost(int64, tag = "3")]
    pub shard_count: i64,
}
/// Table source type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TableSourceType {
    /// Default unknown type.
    Unspecified = 0,
    /// Table view.
    BigqueryView = 2,
    /// BigQuery native table.
    BigqueryTable = 5,
}
/// Tags are used to attach custom metadata to Data Catalog resources. Tags
/// conform to the specifications within their tag template.
///
/// See [Data Catalog
/// IAM](https://cloud.google.com/data-catalog/docs/concepts/iam) for information
/// on the permissions needed to create or view tags.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    /// The resource name of the tag in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}/tags/{tag_id}
    ///
    /// where `tag_id` is a system-generated identifier.
    /// Note that this Tag may not actually be stored in the location in this name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The resource name of the tag template that this tag uses. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}
    ///
    /// This field cannot be modified after creation.
    #[prost(string, tag = "2")]
    pub template: std::string::String,
    /// Output only. The display name of the tag template.
    #[prost(string, tag = "5")]
    pub template_display_name: std::string::String,
    /// Required. This maps the ID of a tag field to the value of and additional information
    /// about that field. Valid field IDs are defined by the tag's template. A tag
    /// must have at least 1 field and at most 500 fields.
    #[prost(map = "string, message", tag = "3")]
    pub fields: ::std::collections::HashMap<std::string::String, TagField>,
    /// The scope within the parent resource that this tag is attached to. If not
    /// provided, the tag is attached to the parent resource itself.
    /// Deleting the scope from the parent resource will delete all tags attached
    /// to that scope. These fields cannot be updated after creation.
    #[prost(oneof = "tag::Scope", tags = "4")]
    pub scope: ::std::option::Option<tag::Scope>,
}
pub mod tag {
    /// The scope within the parent resource that this tag is attached to. If not
    /// provided, the tag is attached to the parent resource itself.
    /// Deleting the scope from the parent resource will delete all tags attached
    /// to that scope. These fields cannot be updated after creation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scope {
        /// Resources like Entry can have schemas associated with them. This scope
        /// allows users to attach tags to an individual column based on that schema.
        ///
        /// For attaching a tag to a nested column, use `.` to separate the column
        /// names. Example:
        ///
        /// * `outer_column.inner_column`
        #[prost(string, tag = "4")]
        Column(std::string::String),
    }
}
/// Contains the value and supporting information for a field within
/// a [Tag][google.cloud.datacatalog.v1.Tag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagField {
    /// Output only. The display name of this field.
    #[prost(string, tag = "1")]
    pub display_name: std::string::String,
    /// Output only. The order of this field with respect to other fields in this tag. It can be
    /// set in [Tag][google.cloud.datacatalog.v1.TagTemplateField.order]. For
    /// example, a higher value can indicate a more important field. The value can
    /// be negative. Multiple fields can have the same order, and field orders
    /// within a tag do not have to be sequential.
    #[prost(int32, tag = "7")]
    pub order: i32,
    /// Required. The value of this field.
    #[prost(oneof = "tag_field::Kind", tags = "2, 3, 4, 5, 6")]
    pub kind: ::std::option::Option<tag_field::Kind>,
}
pub mod tag_field {
    /// Holds an enum value.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumValue {
        /// The display name of the enum value.
        #[prost(string, tag = "1")]
        pub display_name: std::string::String,
    }
    /// Required. The value of this field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Holds the value for a tag field with double type.
        #[prost(double, tag = "2")]
        DoubleValue(f64),
        /// Holds the value for a tag field with string type.
        #[prost(string, tag = "3")]
        StringValue(std::string::String),
        /// Holds the value for a tag field with boolean type.
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        /// Holds the value for a tag field with timestamp type.
        #[prost(message, tag = "5")]
        TimestampValue(::prost_types::Timestamp),
        /// Holds the value for a tag field with enum type. This value must be
        /// one of the allowed values in the definition of this enum.
        #[prost(message, tag = "6")]
        EnumValue(EnumValue),
    }
}
/// A tag template defines a tag, which can have one or more typed fields.
/// The template is used to create and attach the tag to GCP resources.
/// [Tag template
/// roles](https://cloud.google.com/iam/docs/understanding-roles#data-catalog-roles)
/// provide permissions to create, edit, and use the template. See, for example,
/// the [TagTemplate
/// User](https://cloud.google.com/data-catalog/docs/how-to/template-user) role,
/// which includes permission to use the tag template to tag resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTemplate {
    /// The resource name of the tag template in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}
    ///
    /// Note that this TagTemplate and its child resources may not actually be
    /// stored in the location in this name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The display name for this template. Defaults to an empty string.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Required. Map of tag template field IDs to the settings for the field.
    /// This map is an exhaustive list of the allowed fields. This map must contain
    /// at least one field and at most 500 fields.
    ///
    /// The keys to this map are tag template field IDs. Field IDs can contain
    /// letters (both uppercase and lowercase), numbers (0-9) and underscores (_).
    /// Field IDs must be at least 1 character long and at most
    /// 64 characters long. Field IDs must start with a letter or underscore.
    #[prost(map = "string, message", tag = "3")]
    pub fields: ::std::collections::HashMap<std::string::String, TagTemplateField>,
}
/// The template for an individual field within a tag template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagTemplateField {
    /// Output only. The resource name of the tag template field in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template}/fields/{field}
    ///
    /// Note that this TagTemplateField may not actually be stored in the location
    /// in this name.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
    /// The display name for this field. Defaults to an empty string.
    #[prost(string, tag = "1")]
    pub display_name: std::string::String,
    /// Required. The type of value this tag field can contain.
    #[prost(message, optional, tag = "2")]
    pub r#type: ::std::option::Option<FieldType>,
    /// Whether this is a required field. Defaults to false.
    #[prost(bool, tag = "3")]
    pub is_required: bool,
    /// The order of this field with respect to other fields in this tag
    /// template. For example, a higher value can indicate a more important field.
    /// The value can be negative. Multiple fields can have the same order, and
    /// field orders within a tag do not have to be sequential.
    #[prost(int32, tag = "5")]
    pub order: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldType {
    /// Required.
    #[prost(oneof = "field_type::TypeDecl", tags = "1, 2")]
    pub type_decl: ::std::option::Option<field_type::TypeDecl>,
}
pub mod field_type {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumType {
        /// Required on create; optional on update. The set of allowed values for
        /// this enum. This set must not be empty, the display names of the values in
        /// this set must not be empty and the display names of the values must be
        /// case-insensitively unique within this set. Currently, enum values can
        /// only be added to the list of allowed values. Deletion and renaming of
        /// enum values are not supported. Can have up to 500 allowed values.
        #[prost(message, repeated, tag = "1")]
        pub allowed_values: ::std::vec::Vec<enum_type::EnumValue>,
    }
    pub mod enum_type {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EnumValue {
            /// Required. The display name of the enum value. Must not be an empty string.
            #[prost(string, tag = "1")]
            pub display_name: std::string::String,
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PrimitiveType {
        /// This is the default invalid value for a type.
        Unspecified = 0,
        /// A double precision number.
        Double = 1,
        /// An UTF-8 string.
        String = 2,
        /// A boolean value.
        Bool = 3,
        /// A timestamp.
        Timestamp = 4,
    }
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeDecl {
        /// Represents primitive types - string, bool etc.
        #[prost(enumeration = "PrimitiveType", tag = "1")]
        PrimitiveType(i32),
        /// Represents an enum type.
        #[prost(message, tag = "2")]
        EnumType(EnumType),
    }
}
/// Request message for
/// [SearchCatalog][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogRequest {
    /// Required. The scope of this search request. A `scope` that has empty
    /// `include_org_ids`, `include_project_ids` AND false
    /// `include_gcp_public_datasets` is considered invalid. Data Catalog will
    /// return an error in such a case.
    #[prost(message, optional, tag = "6")]
    pub scope: ::std::option::Option<search_catalog_request::Scope>,
    /// Required. The query string in search query syntax. The query must be
    /// non-empty.
    ///
    /// Query strings can be simple as "x" or more qualified as:
    ///
    /// * name:x
    /// * column:x
    /// * description:y
    ///
    /// Note: Query tokens need to have a minimum of 3 characters for substring
    /// matching to work correctly. See [Data Catalog Search
    /// Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference)
    /// for more information.
    #[prost(string, tag = "1")]
    pub query: std::string::String,
    /// Number of results in the search page. If <=0 then defaults to 10. Max limit
    /// for page_size is 1000. Throws an invalid argument for page_size > 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token returned in an earlier
    /// [SearchCatalogResponse.next_page_token][google.cloud.datacatalog.v1.SearchCatalogResponse.next_page_token],
    /// which indicates that this is a continuation of a prior
    /// [SearchCatalogRequest][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog]
    /// call, and that the system should return the next page of data. If empty,
    /// the first page is returned.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Specifies the ordering of results, currently supported case-sensitive
    /// choices are:
    ///
    ///   * `relevance`, only supports descending
    ///   * `last_modified_timestamp [asc|desc]`, defaults to descending if not
    ///     specified
    ///
    /// If not specified, defaults to `relevance` descending.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
pub mod search_catalog_request {
    /// The criteria that select the subspace used for query matching.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Scope {
        /// The list of organization IDs to search within. To find your organization
        /// ID, follow instructions in
        /// https://cloud.google.com/resource-manager/docs/creating-managing-organization.
        #[prost(string, repeated, tag = "2")]
        pub include_org_ids: ::std::vec::Vec<std::string::String>,
        /// The list of project IDs to search within. To learn more about the
        /// distinction between project names/IDs/numbers, go to
        /// https://cloud.google.com/docs/overview/#projects.
        #[prost(string, repeated, tag = "3")]
        pub include_project_ids: ::std::vec::Vec<std::string::String>,
        /// If `true`, include Google Cloud Platform (GCP) public datasets in the
        /// search results. Info on GCP public datasets is available at
        /// https://cloud.google.com/public-datasets/. By default, GCP public
        /// datasets are excluded.
        #[prost(bool, tag = "7")]
        pub include_gcp_public_datasets: bool,
        /// Optional. The list of locations to search within.
        /// 1. If empty, search will be performed in all locations;
        /// 2. If any of the locations are NOT in the valid locations list, error
        /// will be returned;
        /// 3. Otherwise, search only the given locations for matching results.
        /// Typical usage is to leave this field empty. When a location is
        /// unreachable as returned in the `SearchCatalogResponse.unreachable` field,
        /// users can repeat the search request with this parameter set to get
        /// additional information on the error.
        ///
        /// Valid locations:
        ///  * asia-east1
        ///  * asia-east2
        ///  * asia-northeast1
        ///  * asia-northeast2
        ///  * asia-northeast3
        ///  * asia-south1
        ///  * asia-southeast1
        ///  * australia-southeast1
        ///  * eu
        ///  * europe-north1
        ///  * europe-west1
        ///  * europe-west2
        ///  * europe-west3
        ///  * europe-west4
        ///  * europe-west6
        ///  * global
        ///  * northamerica-northeast1
        ///  * southamerica-east1
        ///  * us
        ///  * us-central1
        ///  * us-east1
        ///  * us-east4
        ///  * us-west1
        ///  * us-west2
        #[prost(string, repeated, tag = "16")]
        pub restricted_locations: ::std::vec::Vec<std::string::String>,
    }
}
/// Response message for
/// [SearchCatalog][google.cloud.datacatalog.v1.DataCatalog.SearchCatalog].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchCatalogResponse {
    /// Search results.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<SearchCatalogResult>,
    /// The token that can be used to retrieve the next page of results.
    #[prost(string, tag = "3")]
    pub next_page_token: std::string::String,
    /// Unreachable locations. Search result does not include data from those
    /// locations. Users can get additional information on the error by repeating
    /// the search request with a more restrictive parameter -- setting the value
    /// for `SearchDataCatalogRequest.scope.include_locations`.
    #[prost(string, repeated, tag = "6")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request message for
/// [CreateEntryGroup][google.cloud.datacatalog.v1.DataCatalog.CreateEntryGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntryGroupRequest {
    /// Required. The name of the project this entry group is in. Example:
    ///
    /// * projects/{project_id}/locations/{location}
    ///
    /// Note that this EntryGroup and its child resources may not actually be
    /// stored in the location in this name.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The id of the entry group to create.
    /// The id must begin with a letter or underscore, contain only English
    /// letters, numbers and underscores, and be at most 64 characters.
    #[prost(string, tag = "3")]
    pub entry_group_id: std::string::String,
    /// The entry group to create. Defaults to an empty entry group.
    #[prost(message, optional, tag = "2")]
    pub entry_group: ::std::option::Option<EntryGroup>,
}
/// Request message for
/// [UpdateEntryGroup][google.cloud.datacatalog.v1.DataCatalog.UpdateEntryGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntryGroupRequest {
    /// Required. The updated entry group. "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub entry_group: ::std::option::Option<EntryGroup>,
    /// The fields to update on the entry group. If absent or empty, all modifiable
    /// fields are updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [GetEntryGroup][google.cloud.datacatalog.v1.DataCatalog.GetEntryGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntryGroupRequest {
    /// Required. The name of the entry group. For example,
    /// `projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The fields to return. If not set or empty, all fields are returned.
    #[prost(message, optional, tag = "2")]
    pub read_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteEntryGroup][google.cloud.datacatalog.v1.DataCatalog.DeleteEntryGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntryGroupRequest {
    /// Required. The name of the entry group. For example,
    /// `projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. If true, deletes all entries in the entry group.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// [ListEntryGroups][google.cloud.datacatalog.v1.DataCatalog.ListEntryGroups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntryGroupsRequest {
    /// Required. The name of the location that contains the entry groups, which
    /// can be provided in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return. Default is 10. Max limit
    /// is 1000. Throws an invalid argument for `page_size > 1000`.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Token that specifies which page is requested. If empty, the first
    /// page is returned.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for
/// [ListEntryGroups][google.cloud.datacatalog.v1.DataCatalog.ListEntryGroups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntryGroupsResponse {
    /// EntryGroup details.
    #[prost(message, repeated, tag = "1")]
    pub entry_groups: ::std::vec::Vec<EntryGroup>,
    /// Token to retrieve the next page of results. It is set to empty if no items
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for
/// [CreateEntry][google.cloud.datacatalog.v1.DataCatalog.CreateEntry].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntryRequest {
    /// Required. The name of the entry group this entry is in. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}
    ///
    /// Note that this Entry and its child resources may not actually be stored in
    /// the location in this name.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The id of the entry to create.
    #[prost(string, tag = "3")]
    pub entry_id: std::string::String,
    /// Required. The entry to create.
    #[prost(message, optional, tag = "2")]
    pub entry: ::std::option::Option<Entry>,
}
/// Request message for
/// [UpdateEntry][google.cloud.datacatalog.v1.DataCatalog.UpdateEntry].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntryRequest {
    /// Required. The updated entry. The "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub entry: ::std::option::Option<Entry>,
    /// The fields to update on the entry. If absent or empty, all modifiable
    /// fields are updated.
    ///
    /// The following fields are modifiable:
    /// * For entries with type `DATA_STREAM`:
    ///    * `schema`
    /// * For entries with type `FILESET`
    ///    * `schema`
    ///    * `display_name`
    ///    * `description`
    ///    * `gcs_fileset_spec`
    ///    * `gcs_fileset_spec.file_patterns`
    /// * For entries with `user_specified_type`
    ///    * `schema`
    ///    * `display_name`
    ///    * `description`
    ///    * user_specified_type
    ///    * user_specified_system
    ///    * linked_resource
    ///    * source_system_timestamps
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteEntry][google.cloud.datacatalog.v1.DataCatalog.DeleteEntry].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntryRequest {
    /// Required. The name of the entry. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [GetEntry][google.cloud.datacatalog.v1.DataCatalog.GetEntry].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntryRequest {
    /// Required. The name of the entry. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [LookupEntry][google.cloud.datacatalog.v1.DataCatalog.LookupEntry].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEntryRequest {
    /// Required. Represents either the Google Cloud Platform resource or SQL name
    /// for a Google Cloud Platform resource.
    #[prost(oneof = "lookup_entry_request::TargetName", tags = "1, 3")]
    pub target_name: ::std::option::Option<lookup_entry_request::TargetName>,
}
pub mod lookup_entry_request {
    /// Required. Represents either the Google Cloud Platform resource or SQL name
    /// for a Google Cloud Platform resource.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetName {
        /// The full name of the Google Cloud Platform resource the Data Catalog
        /// entry represents. See:
        /// https://cloud.google.com/apis/design/resource_names#full_resource_name.
        /// Full names are case-sensitive.
        ///
        /// Examples:
        ///
        ///  * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId
        ///  * //pubsub.googleapis.com/projects/projectId/topics/topicId
        #[prost(string, tag = "1")]
        LinkedResource(std::string::String),
        /// The SQL name of the entry. SQL names are case-sensitive.
        ///
        /// Examples:
        ///
        ///   * `pubsub.project_id.topic_id`
        ///   * ``pubsub.project_id.`topic.id.with.dots` ``
        ///   * `bigquery.table.project_id.dataset_id.table_id`
        ///   * `bigquery.dataset.project_id.dataset_id`
        ///   * `datacatalog.entry.project_id.location_id.entry_group_id.entry_id`
        ///
        /// `*_id`s shoud satisfy the standard SQL rules for identifiers.
        /// https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical.
        #[prost(string, tag = "3")]
        SqlResource(std::string::String),
    }
}
/// Entry Metadata.
/// A Data Catalog Entry resource represents another resource in Google
/// Cloud Platform (such as a BigQuery dataset or a Pub/Sub topic) or
/// outside of Google Cloud Platform. Clients can use the `linked_resource` field
/// in the Entry resource to refer to the original resource ID of the source
/// system.
///
/// An Entry resource contains resource details, such as its schema. An Entry can
/// also be used to attach flexible metadata, such as a
/// [Tag][google.cloud.datacatalog.v1.Tag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entry {
    /// The Data Catalog resource name of the entry in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    ///
    /// Note that this Entry and its child resources may not actually be stored in
    /// the location in this name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource this metadata entry refers to.
    ///
    /// For Google Cloud Platform resources, `linked_resource` is the [full name of
    /// the
    /// resource](https://cloud.google.com/apis/design/resource_names#full_resource_name).
    /// For example, the `linked_resource` for a table resource from BigQuery is:
    ///
    /// * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId
    ///
    /// Output only when Entry is of type in the EntryType enum. For entries with
    /// user_specified_type, this field is optional and defaults to an empty
    /// string.
    #[prost(string, tag = "9")]
    pub linked_resource: std::string::String,
    /// Display information such as title and description. A short name to identify
    /// the entry, for example, "Analytics Data - Jan 2011". Default value is an
    /// empty string.
    #[prost(string, tag = "3")]
    pub display_name: std::string::String,
    /// Entry description, which can consist of several sentences or paragraphs
    /// that describe entry contents. Default value is an empty string.
    #[prost(string, tag = "4")]
    pub description: std::string::String,
    /// Schema of the entry. An entry might not have any schema attached to it.
    #[prost(message, optional, tag = "5")]
    pub schema: ::std::option::Option<Schema>,
    /// Timestamps about the underlying resource, not about this Data Catalog
    /// entry. Output only when Entry is of type in the EntryType enum. For entries
    /// with user_specified_type, this field is optional and defaults to an empty
    /// timestamp.
    #[prost(message, optional, tag = "7")]
    pub source_system_timestamps: ::std::option::Option<SystemTimestamps>,
    /// Required. Entry type.
    #[prost(oneof = "entry::EntryType", tags = "2, 16")]
    pub entry_type: ::std::option::Option<entry::EntryType>,
    /// The source system of the entry.
    #[prost(oneof = "entry::System", tags = "17, 18")]
    pub system: ::std::option::Option<entry::System>,
    /// Type specification information.
    #[prost(oneof = "entry::TypeSpec", tags = "6, 12, 15")]
    pub type_spec: ::std::option::Option<entry::TypeSpec>,
}
pub mod entry {
    /// Required. Entry type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EntryType {
        /// The type of the entry.
        /// Only used for Entries with types in the EntryType enum.
        #[prost(enumeration = "super::EntryType", tag = "2")]
        Type(i32),
        /// Entry type if it does not fit any of the input-allowed values listed in
        /// `EntryType` enum above. When creating an entry, users should check the
        /// enum values first, if nothing matches the entry to be created, then
        /// provide a custom value, for example "my_special_type".
        /// `user_specified_type` strings must begin with a letter or underscore and
        /// can only contain letters, numbers, and underscores; are case insensitive;
        /// must be at least 1 character and at most 64 characters long.
        ///
        /// Currently, only FILESET enum value is allowed. All other entries created
        /// through Data Catalog must use `user_specified_type`.
        #[prost(string, tag = "16")]
        UserSpecifiedType(std::string::String),
    }
    /// The source system of the entry.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum System {
        /// Output only. This field indicates the entry's source system that Data
        /// Catalog integrates with, such as BigQuery or Pub/Sub.
        #[prost(enumeration = "super::IntegratedSystem", tag = "17")]
        IntegratedSystem(i32),
        /// This field indicates the entry's source system that Data Catalog does not
        /// integrate with. `user_specified_system` strings must begin with a letter
        /// or underscore and can only contain letters, numbers, and underscores; are
        /// case insensitive; must be at least 1 character and at most 64 characters
        /// long.
        #[prost(string, tag = "18")]
        UserSpecifiedSystem(std::string::String),
    }
    /// Type specification information.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TypeSpec {
        /// Specification that applies to a Cloud Storage fileset. This is only valid
        /// on entries of type FILESET.
        #[prost(message, tag = "6")]
        GcsFilesetSpec(super::GcsFilesetSpec),
        /// Specification that applies to a BigQuery table. This is only valid on
        /// entries of type `TABLE`.
        #[prost(message, tag = "12")]
        BigqueryTableSpec(super::BigQueryTableSpec),
        /// Specification for a group of BigQuery tables with name pattern
        /// `[prefix]YYYYMMDD`. Context:
        /// https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding.
        #[prost(message, tag = "15")]
        BigqueryDateShardedSpec(super::BigQueryDateShardedSpec),
    }
}
/// EntryGroup Metadata.
/// An EntryGroup resource represents a logical grouping of zero or more
/// Data Catalog [Entry][google.cloud.datacatalog.v1.Entry] resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryGroup {
    /// The resource name of the entry group in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}
    ///
    /// Note that this EntryGroup and its child resources may not actually be
    /// stored in the location in this name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A short name to identify the entry group, for example,
    /// "analytics data - jan 2011". Default value is an empty string.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Entry group description, which can consist of several sentences or
    /// paragraphs that describe entry group contents. Default value is an empty
    /// string.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// Output only. Timestamps about this EntryGroup. Default value is empty
    /// timestamps.
    #[prost(message, optional, tag = "4")]
    pub data_catalog_timestamps: ::std::option::Option<SystemTimestamps>,
}
/// Request message for
/// [CreateTagTemplate][google.cloud.datacatalog.v1.DataCatalog.CreateTagTemplate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagTemplateRequest {
    /// Required. The name of the project and the template location
    /// [region](https://cloud.google.com/data-catalog/docs/concepts/regions).
    ///
    /// Example:
    ///
    /// * projects/{project_id}/locations/us-central1
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The id of the tag template to create.
    #[prost(string, tag = "3")]
    pub tag_template_id: std::string::String,
    /// Required. The tag template to create.
    #[prost(message, optional, tag = "2")]
    pub tag_template: ::std::option::Option<TagTemplate>,
}
/// Request message for
/// [GetTagTemplate][google.cloud.datacatalog.v1.DataCatalog.GetTagTemplate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagTemplateRequest {
    /// Required. The name of the tag template. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [UpdateTagTemplate][google.cloud.datacatalog.v1.DataCatalog.UpdateTagTemplate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagTemplateRequest {
    /// Required. The template to update. The "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub tag_template: ::std::option::Option<TagTemplate>,
    /// The field mask specifies the parts of the template to overwrite.
    ///
    /// Allowed fields:
    ///
    ///   * `display_name`
    ///
    /// If absent or empty, all of the allowed fields above will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteTagTemplate][google.cloud.datacatalog.v1.DataCatalog.DeleteTagTemplate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagTemplateRequest {
    /// Required. The name of the tag template to delete. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Currently, this field must always be set to `true`.
    /// This confirms the deletion of any possible tags using this template.
    /// `force = false` will be supported in the future.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// [CreateTag][google.cloud.datacatalog.v1.DataCatalog.CreateTag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagRequest {
    /// Required. The name of the resource to attach this tag to. Tags can be
    /// attached to Entries. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    ///
    /// Note that this Tag and its child resources may not actually be stored in
    /// the location in this name.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The tag to create.
    #[prost(message, optional, tag = "2")]
    pub tag: ::std::option::Option<Tag>,
}
/// Request message for
/// [UpdateTag][google.cloud.datacatalog.v1.DataCatalog.UpdateTag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagRequest {
    /// Required. The updated tag. The "name" field must be set.
    #[prost(message, optional, tag = "1")]
    pub tag: ::std::option::Option<Tag>,
    /// The fields to update on the Tag. If absent or empty, all modifiable fields
    /// are updated. Currently the only modifiable field is the field `fields`.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [DeleteTag][google.cloud.datacatalog.v1.DataCatalog.DeleteTag].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagRequest {
    /// Required. The name of the tag to delete. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}/tags/{tag_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [CreateTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.CreateTagTemplateField].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagTemplateFieldRequest {
    /// Required. The name of the project and the template location
    /// [region](https://cloud.google.com/data-catalog/docs/concepts/regions).
    ///
    /// Example:
    ///
    /// * projects/{project_id}/locations/us-central1/tagTemplates/{tag_template_id}
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID of the tag template field to create.
    /// Field ids can contain letters (both uppercase and lowercase), numbers
    /// (0-9), underscores (_) and dashes (-). Field IDs must be at least 1
    /// character long and at most 128 characters long. Field IDs must also be
    /// unique within their template.
    #[prost(string, tag = "2")]
    pub tag_template_field_id: std::string::String,
    /// Required. The tag template field to create.
    #[prost(message, optional, tag = "3")]
    pub tag_template_field: ::std::option::Option<TagTemplateField>,
}
/// Request message for
/// [UpdateTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.UpdateTagTemplateField].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagTemplateFieldRequest {
    /// Required. The name of the tag template field. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The template to update.
    #[prost(message, optional, tag = "2")]
    pub tag_template_field: ::std::option::Option<TagTemplateField>,
    /// Optional. The field mask specifies the parts of the template to be updated.
    /// Allowed fields:
    ///
    ///   * `display_name`
    ///   * `type.enum_type`
    ///   * `is_required`
    ///
    /// If `update_mask` is not set or empty, all of the allowed fields above will
    /// be updated.
    ///
    /// When updating an enum type, the provided values will be merged with the
    /// existing values. Therefore, enum values can only be added, existing enum
    /// values cannot be deleted nor renamed. Updating a template field from
    /// optional to required is NOT allowed.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// [RenameTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.RenameTagTemplateField].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTagTemplateFieldRequest {
    /// Required. The name of the tag template. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The new ID of this tag template field. For example,
    /// `my_new_field`.
    #[prost(string, tag = "2")]
    pub new_tag_template_field_id: std::string::String,
}
/// Request message for
/// [DeleteTagTemplateField][google.cloud.datacatalog.v1.DataCatalog.DeleteTagTemplateField].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagTemplateFieldRequest {
    /// Required. The name of the tag template field to delete. Example:
    ///
    /// * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id}/fields/{tag_template_field_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Currently, this field must always be set to `true`.
    /// This confirms the deletion of this field from any tags using this field.
    /// `force = false` will be supported in the future.
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for
/// [ListTags][google.cloud.datacatalog.v1.DataCatalog.ListTags].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsRequest {
    /// Required. The name of the Data Catalog resource to list the tags of. The
    /// resource could be an [Entry][google.cloud.datacatalog.v1.Entry] or an
    /// [EntryGroup][google.cloud.datacatalog.v1.EntryGroup].
    ///
    /// Examples:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id}
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of tags to return. Default is 10. Max limit is 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token that specifies which page is requested. If empty, the first page is
    /// returned.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for
/// [ListTags][google.cloud.datacatalog.v1.DataCatalog.ListTags].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagsResponse {
    /// [Tag][google.cloud.datacatalog.v1.Tag] details.
    #[prost(message, repeated, tag = "1")]
    pub tags: ::std::vec::Vec<Tag>,
    /// Token to retrieve the next page of results. It is set to empty if no items
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for
/// [ListEntries][google.cloud.datacatalog.v1.DataCatalog.ListEntries].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntriesRequest {
    /// Required. The name of the entry group that contains the entries, which can
    /// be provided in URL format. Example:
    ///
    /// * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return. Default is 10. Max limit is 1000.
    /// Throws an invalid argument for `page_size > 1000`.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token that specifies which page is requested. If empty, the first page is
    /// returned.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// The fields to return for each Entry. If not set or empty, all
    /// fields are returned.
    /// For example, setting read_mask to contain only one path "name" will cause
    /// ListEntries to return a list of Entries with only "name" field.
    #[prost(message, optional, tag = "4")]
    pub read_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Response message for
/// [ListEntries][google.cloud.datacatalog.v1.DataCatalog.ListEntries].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntriesResponse {
    /// Entry details.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::std::vec::Vec<Entry>,
    /// Token to retrieve the next page of results. It is set to empty if no items
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Entry resources in Data Catalog can be of different types e.g. a BigQuery
/// Table entry is of type `TABLE`. This enum describes all the possible types
/// Data Catalog contains.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EntryType {
    /// Default unknown type.
    Unspecified = 0,
    /// Output only. The type of entry that has a GoogleSQL schema, including
    /// logical views.
    Table = 2,
    /// Output only. The type of models, examples include
    /// https://cloud.google.com/bigquery-ml/docs/bigqueryml-intro
    Model = 5,
    /// Output only. An entry type which is used for streaming entries. Example:
    /// Pub/Sub topic.
    DataStream = 3,
    /// An entry type which is a set of files or objects. Example:
    /// Cloud Storage fileset.
    Fileset = 4,
}
#[doc = r" Generated client implementations."]
pub mod data_catalog_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Data Catalog API service allows clients to discover, understand, and manage"]
    #[doc = " their data."]
    pub struct DataCatalogClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataCatalogClient<T>
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
        #[doc = " Searches Data Catalog for multiple resources like entries, tags that"]
        #[doc = " match a query."]
        #[doc = ""]
        #[doc = " This is a custom method"]
        #[doc = " (https://cloud.google.com/apis/design/custom_methods) and does not return"]
        #[doc = " the complete resource, only the resource identifier and high level"]
        #[doc = " fields. Clients can subsequentally call `Get` methods."]
        #[doc = ""]
        #[doc = " Note that Data Catalog search queries do not guarantee full recall. Query"]
        #[doc = " results that match your query may not be returned, even in subsequent"]
        #[doc = " result pages. Also note that results returned (and not returned) can vary"]
        #[doc = " across repeated search queries."]
        #[doc = ""]
        #[doc = " See [Data Catalog Search"]
        #[doc = " Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference)"]
        #[doc = " for more information."]
        pub async fn search_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchCatalogRequest>,
        ) -> Result<tonic::Response<super::SearchCatalogResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/SearchCatalog",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an EntryGroup."]
        #[doc = ""]
        #[doc = " An entry group contains logically related entries together with Cloud"]
        #[doc = " Identity and Access Management policies that specify the users who can"]
        #[doc = " create, edit, and view entries within the entry group."]
        #[doc = ""]
        #[doc = " Data Catalog automatically creates an entry group for BigQuery entries"]
        #[doc = " (\"@bigquery\") and Pub/Sub topics (\"@pubsub\"). Users create their own entry"]
        #[doc = " group to contain Cloud Storage fileset entries or custom type entries,"]
        #[doc = " and the IAM policies associated with those entries. Entry groups, like"]
        #[doc = " entries, can be searched."]
        #[doc = ""]
        #[doc = " A maximum of 10,000 entry groups may be created per organization across all"]
        #[doc = " locations."]
        #[doc = ""]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `parent` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn create_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntryGroupRequest>,
        ) -> Result<tonic::Response<super::EntryGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/CreateEntryGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets an EntryGroup."]
        pub async fn get_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntryGroupRequest>,
        ) -> Result<tonic::Response<super::EntryGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/GetEntryGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an EntryGroup. The user should enable the Data Catalog API in the"]
        #[doc = " project identified by the `entry_group.name` parameter (see [Data Catalog"]
        #[doc = " Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn update_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntryGroupRequest>,
        ) -> Result<tonic::Response<super::EntryGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateEntryGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an EntryGroup. Only entry groups that do not contain entries can be"]
        #[doc = " deleted. Users should enable the Data Catalog API in the project"]
        #[doc = " identified by the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn delete_entry_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntryGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteEntryGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists entry groups."]
        pub async fn list_entry_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntryGroupsRequest>,
        ) -> Result<tonic::Response<super::ListEntryGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/ListEntryGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an entry. Only entries of 'FILESET' type or user-specified type can"]
        #[doc = " be created."]
        #[doc = ""]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `parent` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        #[doc = ""]
        #[doc = " A maximum of 100,000 entries may be created per entry group."]
        pub async fn create_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/CreateEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing entry."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `entry.name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn update_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing entry. Only entries created through"]
        #[doc = " [CreateEntry][google.cloud.datacatalog.v1.DataCatalog.CreateEntry]"]
        #[doc = " method can be deleted."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn delete_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntryRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets an entry."]
        pub async fn get_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/GetEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get an entry by target resource name. This method allows clients to use"]
        #[doc = " the resource name from the source Google Cloud Platform service to get the"]
        #[doc = " Data Catalog Entry."]
        pub async fn lookup_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/LookupEntry",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists entries."]
        pub async fn list_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntriesRequest>,
        ) -> Result<tonic::Response<super::ListEntriesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/ListEntries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a tag template. The user should enable the Data Catalog API in"]
        #[doc = " the project identified by the `parent` parameter (see [Data Catalog"]
        #[doc = " Resource"]
        #[doc = " Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)"]
        #[doc = " for more information)."]
        pub async fn create_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagTemplateRequest>,
        ) -> Result<tonic::Response<super::TagTemplate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTagTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a tag template."]
        pub async fn get_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTagTemplateRequest>,
        ) -> Result<tonic::Response<super::TagTemplate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/GetTagTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a tag template. This method cannot be used to update the fields of"]
        #[doc = " a template. The tag template fields are represented as separate resources"]
        #[doc = " and should be updated using their own create/update/delete methods."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `tag_template.name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn update_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagTemplateRequest>,
        ) -> Result<tonic::Response<super::TagTemplate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTagTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a tag template and all tags using the template."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn delete_tag_template(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagTemplateRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTagTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a field in a tag template. The user should enable the Data Catalog"]
        #[doc = " API in the project identified by the `parent` parameter (see"]
        #[doc = " [Data Catalog Resource"]
        #[doc = " Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)"]
        #[doc = " for more information)."]
        pub async fn create_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<super::TagTemplateField>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTagTemplateField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a field in a tag template. This method cannot be used to update the"]
        #[doc = " field type. Users should enable the Data Catalog API in the project"]
        #[doc = " identified by the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn update_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<super::TagTemplateField>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTagTemplateField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Renames a field in a tag template. The user should enable the Data Catalog"]
        #[doc = " API in the project identified by the `name` parameter (see [Data Catalog"]
        #[doc = " Resource"]
        #[doc = " Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)"]
        #[doc = " for more information)."]
        pub async fn rename_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<super::TagTemplateField>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/RenameTagTemplateField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a field in a tag template and all uses of that field."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        pub async fn delete_tag_template_field(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTagTemplateField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a tag on an [Entry][google.cloud.datacatalog.v1.Entry]."]
        #[doc = " Note: The project identified by the `parent` parameter for the"]
        #[doc = " [tag](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries.tags/create#path-parameters)"]
        #[doc = " and the"]
        #[doc = " [tag"]
        #[doc = " template](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.tagTemplates/create#path-parameters)"]
        #[doc = " used to create the tag must be from the same organization."]
        pub async fn create_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagRequest>,
        ) -> Result<tonic::Response<super::Tag>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing tag."]
        pub async fn update_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagRequest>,
        ) -> Result<tonic::Response<super::Tag>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a tag."]
        pub async fn delete_tag(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTag",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the tags on an [Entry][google.cloud.datacatalog.v1.Entry]."]
        pub async fn list_tags(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTagsRequest>,
        ) -> Result<tonic::Response<super::ListTagsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/ListTags",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy for a resource. Replaces any existing"]
        #[doc = " policy."]
        #[doc = " Supported resources are:"]
        #[doc = "   - Tag templates."]
        #[doc = "   - Entries."]
        #[doc = "   - Entry groups."]
        #[doc = " Note, this method cannot be used to manage policies for BigQuery, Pub/Sub"]
        #[doc = " and any external Google Cloud Platform resources synced to Data Catalog."]
        #[doc = ""]
        #[doc = " Callers must have following Google IAM permission"]
        #[doc = "   - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag"]
        #[doc = "     templates."]
        #[doc = "   - `datacatalog.entries.setIamPolicy` to set policies on entries."]
        #[doc = "   - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a resource. A `NOT_FOUND` error"]
        #[doc = " is returned if the resource does not exist. An empty policy is returned"]
        #[doc = " if the resource exists but does not have a policy set on it."]
        #[doc = ""]
        #[doc = " Supported resources are:"]
        #[doc = "   - Tag templates."]
        #[doc = "   - Entries."]
        #[doc = "   - Entry groups."]
        #[doc = " Note, this method cannot be used to manage policies for BigQuery, Pub/Sub"]
        #[doc = " and any external Google Cloud Platform resources synced to Data Catalog."]
        #[doc = ""]
        #[doc = " Callers must have following Google IAM permission"]
        #[doc = "   - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag"]
        #[doc = "     templates."]
        #[doc = "   - `datacatalog.entries.getIamPolicy` to get policies on entries."]
        #[doc = "   - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the caller's permissions on a resource."]
        #[doc = " If the resource does not exist, an empty set of permissions is returned"]
        #[doc = " (We don't return a `NOT_FOUND` error)."]
        #[doc = ""]
        #[doc = " Supported resources are:"]
        #[doc = "   - Tag templates."]
        #[doc = "   - Entries."]
        #[doc = "   - Entry groups."]
        #[doc = " Note, this method cannot be used to manage policies for BigQuery, Pub/Sub"]
        #[doc = " and any external Google Cloud Platform resources synced to Data Catalog."]
        #[doc = ""]
        #[doc = " A caller is not required to have Google IAM permission to make this"]
        #[doc = " request."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datacatalog.v1.DataCatalog/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DataCatalogClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DataCatalogClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DataCatalogClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod data_catalog_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DataCatalogServer."]
    #[async_trait]
    pub trait DataCatalog: Send + Sync + 'static {
        #[doc = " Searches Data Catalog for multiple resources like entries, tags that"]
        #[doc = " match a query."]
        #[doc = ""]
        #[doc = " This is a custom method"]
        #[doc = " (https://cloud.google.com/apis/design/custom_methods) and does not return"]
        #[doc = " the complete resource, only the resource identifier and high level"]
        #[doc = " fields. Clients can subsequentally call `Get` methods."]
        #[doc = ""]
        #[doc = " Note that Data Catalog search queries do not guarantee full recall. Query"]
        #[doc = " results that match your query may not be returned, even in subsequent"]
        #[doc = " result pages. Also note that results returned (and not returned) can vary"]
        #[doc = " across repeated search queries."]
        #[doc = ""]
        #[doc = " See [Data Catalog Search"]
        #[doc = " Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference)"]
        #[doc = " for more information."]
        async fn search_catalog(
            &self,
            request: tonic::Request<super::SearchCatalogRequest>,
        ) -> Result<tonic::Response<super::SearchCatalogResponse>, tonic::Status>;
        #[doc = " Creates an EntryGroup."]
        #[doc = ""]
        #[doc = " An entry group contains logically related entries together with Cloud"]
        #[doc = " Identity and Access Management policies that specify the users who can"]
        #[doc = " create, edit, and view entries within the entry group."]
        #[doc = ""]
        #[doc = " Data Catalog automatically creates an entry group for BigQuery entries"]
        #[doc = " (\"@bigquery\") and Pub/Sub topics (\"@pubsub\"). Users create their own entry"]
        #[doc = " group to contain Cloud Storage fileset entries or custom type entries,"]
        #[doc = " and the IAM policies associated with those entries. Entry groups, like"]
        #[doc = " entries, can be searched."]
        #[doc = ""]
        #[doc = " A maximum of 10,000 entry groups may be created per organization across all"]
        #[doc = " locations."]
        #[doc = ""]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `parent` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        async fn create_entry_group(
            &self,
            request: tonic::Request<super::CreateEntryGroupRequest>,
        ) -> Result<tonic::Response<super::EntryGroup>, tonic::Status>;
        #[doc = " Gets an EntryGroup."]
        async fn get_entry_group(
            &self,
            request: tonic::Request<super::GetEntryGroupRequest>,
        ) -> Result<tonic::Response<super::EntryGroup>, tonic::Status>;
        #[doc = " Updates an EntryGroup. The user should enable the Data Catalog API in the"]
        #[doc = " project identified by the `entry_group.name` parameter (see [Data Catalog"]
        #[doc = " Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        async fn update_entry_group(
            &self,
            request: tonic::Request<super::UpdateEntryGroupRequest>,
        ) -> Result<tonic::Response<super::EntryGroup>, tonic::Status>;
        #[doc = " Deletes an EntryGroup. Only entry groups that do not contain entries can be"]
        #[doc = " deleted. Users should enable the Data Catalog API in the project"]
        #[doc = " identified by the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        async fn delete_entry_group(
            &self,
            request: tonic::Request<super::DeleteEntryGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists entry groups."]
        async fn list_entry_groups(
            &self,
            request: tonic::Request<super::ListEntryGroupsRequest>,
        ) -> Result<tonic::Response<super::ListEntryGroupsResponse>, tonic::Status>;
        #[doc = " Creates an entry. Only entries of 'FILESET' type or user-specified type can"]
        #[doc = " be created."]
        #[doc = ""]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `parent` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        #[doc = ""]
        #[doc = " A maximum of 100,000 entries may be created per entry group."]
        async fn create_entry(
            &self,
            request: tonic::Request<super::CreateEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status>;
        #[doc = " Updates an existing entry."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `entry.name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        async fn update_entry(
            &self,
            request: tonic::Request<super::UpdateEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status>;
        #[doc = " Deletes an existing entry. Only entries created through"]
        #[doc = " [CreateEntry][google.cloud.datacatalog.v1.DataCatalog.CreateEntry]"]
        #[doc = " method can be deleted."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        async fn delete_entry(
            &self,
            request: tonic::Request<super::DeleteEntryRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Gets an entry."]
        async fn get_entry(
            &self,
            request: tonic::Request<super::GetEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status>;
        #[doc = " Get an entry by target resource name. This method allows clients to use"]
        #[doc = " the resource name from the source Google Cloud Platform service to get the"]
        #[doc = " Data Catalog Entry."]
        async fn lookup_entry(
            &self,
            request: tonic::Request<super::LookupEntryRequest>,
        ) -> Result<tonic::Response<super::Entry>, tonic::Status>;
        #[doc = " Lists entries."]
        async fn list_entries(
            &self,
            request: tonic::Request<super::ListEntriesRequest>,
        ) -> Result<tonic::Response<super::ListEntriesResponse>, tonic::Status>;
        #[doc = " Creates a tag template. The user should enable the Data Catalog API in"]
        #[doc = " the project identified by the `parent` parameter (see [Data Catalog"]
        #[doc = " Resource"]
        #[doc = " Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)"]
        #[doc = " for more information)."]
        async fn create_tag_template(
            &self,
            request: tonic::Request<super::CreateTagTemplateRequest>,
        ) -> Result<tonic::Response<super::TagTemplate>, tonic::Status>;
        #[doc = " Gets a tag template."]
        async fn get_tag_template(
            &self,
            request: tonic::Request<super::GetTagTemplateRequest>,
        ) -> Result<tonic::Response<super::TagTemplate>, tonic::Status>;
        #[doc = " Updates a tag template. This method cannot be used to update the fields of"]
        #[doc = " a template. The tag template fields are represented as separate resources"]
        #[doc = " and should be updated using their own create/update/delete methods."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `tag_template.name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        async fn update_tag_template(
            &self,
            request: tonic::Request<super::UpdateTagTemplateRequest>,
        ) -> Result<tonic::Response<super::TagTemplate>, tonic::Status>;
        #[doc = " Deletes a tag template and all tags using the template."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        async fn delete_tag_template(
            &self,
            request: tonic::Request<super::DeleteTagTemplateRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Creates a field in a tag template. The user should enable the Data Catalog"]
        #[doc = " API in the project identified by the `parent` parameter (see"]
        #[doc = " [Data Catalog Resource"]
        #[doc = " Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)"]
        #[doc = " for more information)."]
        async fn create_tag_template_field(
            &self,
            request: tonic::Request<super::CreateTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<super::TagTemplateField>, tonic::Status>;
        #[doc = " Updates a field in a tag template. This method cannot be used to update the"]
        #[doc = " field type. Users should enable the Data Catalog API in the project"]
        #[doc = " identified by the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        async fn update_tag_template_field(
            &self,
            request: tonic::Request<super::UpdateTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<super::TagTemplateField>, tonic::Status>;
        #[doc = " Renames a field in a tag template. The user should enable the Data Catalog"]
        #[doc = " API in the project identified by the `name` parameter (see [Data Catalog"]
        #[doc = " Resource"]
        #[doc = " Project](https://cloud.google.com/data-catalog/docs/concepts/resource-project)"]
        #[doc = " for more information)."]
        async fn rename_tag_template_field(
            &self,
            request: tonic::Request<super::RenameTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<super::TagTemplateField>, tonic::Status>;
        #[doc = " Deletes a field in a tag template and all uses of that field."]
        #[doc = " Users should enable the Data Catalog API in the project identified by"]
        #[doc = " the `name` parameter (see [Data Catalog Resource Project]"]
        #[doc = " (https://cloud.google.com/data-catalog/docs/concepts/resource-project) for"]
        #[doc = " more information)."]
        async fn delete_tag_template_field(
            &self,
            request: tonic::Request<super::DeleteTagTemplateFieldRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Creates a tag on an [Entry][google.cloud.datacatalog.v1.Entry]."]
        #[doc = " Note: The project identified by the `parent` parameter for the"]
        #[doc = " [tag](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.entryGroups.entries.tags/create#path-parameters)"]
        #[doc = " and the"]
        #[doc = " [tag"]
        #[doc = " template](https://cloud.google.com/data-catalog/docs/reference/rest/v1/projects.locations.tagTemplates/create#path-parameters)"]
        #[doc = " used to create the tag must be from the same organization."]
        async fn create_tag(
            &self,
            request: tonic::Request<super::CreateTagRequest>,
        ) -> Result<tonic::Response<super::Tag>, tonic::Status>;
        #[doc = " Updates an existing tag."]
        async fn update_tag(
            &self,
            request: tonic::Request<super::UpdateTagRequest>,
        ) -> Result<tonic::Response<super::Tag>, tonic::Status>;
        #[doc = " Deletes a tag."]
        async fn delete_tag(
            &self,
            request: tonic::Request<super::DeleteTagRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists the tags on an [Entry][google.cloud.datacatalog.v1.Entry]."]
        async fn list_tags(
            &self,
            request: tonic::Request<super::ListTagsRequest>,
        ) -> Result<tonic::Response<super::ListTagsResponse>, tonic::Status>;
        #[doc = " Sets the access control policy for a resource. Replaces any existing"]
        #[doc = " policy."]
        #[doc = " Supported resources are:"]
        #[doc = "   - Tag templates."]
        #[doc = "   - Entries."]
        #[doc = "   - Entry groups."]
        #[doc = " Note, this method cannot be used to manage policies for BigQuery, Pub/Sub"]
        #[doc = " and any external Google Cloud Platform resources synced to Data Catalog."]
        #[doc = ""]
        #[doc = " Callers must have following Google IAM permission"]
        #[doc = "   - `datacatalog.tagTemplates.setIamPolicy` to set policies on tag"]
        #[doc = "     templates."]
        #[doc = "   - `datacatalog.entries.setIamPolicy` to set policies on entries."]
        #[doc = "   - `datacatalog.entryGroups.setIamPolicy` to set policies on entry groups."]
        async fn set_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Gets the access control policy for a resource. A `NOT_FOUND` error"]
        #[doc = " is returned if the resource does not exist. An empty policy is returned"]
        #[doc = " if the resource exists but does not have a policy set on it."]
        #[doc = ""]
        #[doc = " Supported resources are:"]
        #[doc = "   - Tag templates."]
        #[doc = "   - Entries."]
        #[doc = "   - Entry groups."]
        #[doc = " Note, this method cannot be used to manage policies for BigQuery, Pub/Sub"]
        #[doc = " and any external Google Cloud Platform resources synced to Data Catalog."]
        #[doc = ""]
        #[doc = " Callers must have following Google IAM permission"]
        #[doc = "   - `datacatalog.tagTemplates.getIamPolicy` to get policies on tag"]
        #[doc = "     templates."]
        #[doc = "   - `datacatalog.entries.getIamPolicy` to get policies on entries."]
        #[doc = "   - `datacatalog.entryGroups.getIamPolicy` to get policies on entry groups."]
        async fn get_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Returns the caller's permissions on a resource."]
        #[doc = " If the resource does not exist, an empty set of permissions is returned"]
        #[doc = " (We don't return a `NOT_FOUND` error)."]
        #[doc = ""]
        #[doc = " Supported resources are:"]
        #[doc = "   - Tag templates."]
        #[doc = "   - Entries."]
        #[doc = "   - Entry groups."]
        #[doc = " Note, this method cannot be used to manage policies for BigQuery, Pub/Sub"]
        #[doc = " and any external Google Cloud Platform resources synced to Data Catalog."]
        #[doc = ""]
        #[doc = " A caller is not required to have Google IAM permission to make this"]
        #[doc = " request."]
        async fn test_iam_permissions(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::TestIamPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        >;
    }
    #[doc = " Data Catalog API service allows clients to discover, understand, and manage"]
    #[doc = " their data."]
    #[derive(Debug)]
    pub struct DataCatalogServer<T: DataCatalog> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DataCatalog> DataCatalogServer<T> {
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
    impl<T, B> Service<http::Request<B>> for DataCatalogServer<T>
    where
        T: DataCatalog,
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
                "/google.cloud.datacatalog.v1.DataCatalog/SearchCatalog" => {
                    #[allow(non_camel_case_types)]
                    struct SearchCatalogSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::SearchCatalogRequest>
                        for SearchCatalogSvc<T>
                    {
                        type Response = super::SearchCatalogResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchCatalogRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search_catalog(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SearchCatalogSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/CreateEntryGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateEntryGroupSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::CreateEntryGroupRequest>
                        for CreateEntryGroupSvc<T>
                    {
                        type Response = super::EntryGroup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateEntryGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_entry_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateEntryGroupSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/GetEntryGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetEntryGroupSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::GetEntryGroupRequest>
                        for GetEntryGroupSvc<T>
                    {
                        type Response = super::EntryGroup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEntryGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_entry_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetEntryGroupSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateEntryGroup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateEntryGroupSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::UpdateEntryGroupRequest>
                        for UpdateEntryGroupSvc<T>
                    {
                        type Response = super::EntryGroup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateEntryGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_entry_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateEntryGroupSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteEntryGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteEntryGroupSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::DeleteEntryGroupRequest>
                        for DeleteEntryGroupSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteEntryGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_entry_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteEntryGroupSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/ListEntryGroups" => {
                    #[allow(non_camel_case_types)]
                    struct ListEntryGroupsSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::ListEntryGroupsRequest>
                        for ListEntryGroupsSvc<T>
                    {
                        type Response = super::ListEntryGroupsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListEntryGroupsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_entry_groups(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListEntryGroupsSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/CreateEntry" => {
                    #[allow(non_camel_case_types)]
                    struct CreateEntrySvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::CreateEntryRequest> for CreateEntrySvc<T> {
                        type Response = super::Entry;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateEntryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_entry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateEntrySvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateEntry" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateEntrySvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::UpdateEntryRequest> for UpdateEntrySvc<T> {
                        type Response = super::Entry;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateEntryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_entry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateEntrySvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteEntry" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteEntrySvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::DeleteEntryRequest> for DeleteEntrySvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteEntryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_entry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteEntrySvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/GetEntry" => {
                    #[allow(non_camel_case_types)]
                    struct GetEntrySvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::GetEntryRequest> for GetEntrySvc<T> {
                        type Response = super::Entry;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEntryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_entry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetEntrySvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/LookupEntry" => {
                    #[allow(non_camel_case_types)]
                    struct LookupEntrySvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::LookupEntryRequest> for LookupEntrySvc<T> {
                        type Response = super::Entry;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LookupEntryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).lookup_entry(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = LookupEntrySvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/ListEntries" => {
                    #[allow(non_camel_case_types)]
                    struct ListEntriesSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::ListEntriesRequest> for ListEntriesSvc<T> {
                        type Response = super::ListEntriesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListEntriesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_entries(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListEntriesSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTagTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTagTemplateSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog>
                        tonic::server::UnaryService<super::CreateTagTemplateRequest>
                        for CreateTagTemplateSvc<T>
                    {
                        type Response = super::TagTemplate;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTagTemplateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_tag_template(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateTagTemplateSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/GetTagTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct GetTagTemplateSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::GetTagTemplateRequest>
                        for GetTagTemplateSvc<T>
                    {
                        type Response = super::TagTemplate;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTagTemplateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_tag_template(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTagTemplateSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTagTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTagTemplateSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog>
                        tonic::server::UnaryService<super::UpdateTagTemplateRequest>
                        for UpdateTagTemplateSvc<T>
                    {
                        type Response = super::TagTemplate;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTagTemplateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_tag_template(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateTagTemplateSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTagTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTagTemplateSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog>
                        tonic::server::UnaryService<super::DeleteTagTemplateRequest>
                        for DeleteTagTemplateSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTagTemplateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_tag_template(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteTagTemplateSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTagTemplateField" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTagTemplateFieldSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog>
                        tonic::server::UnaryService<super::CreateTagTemplateFieldRequest>
                        for CreateTagTemplateFieldSvc<T>
                    {
                        type Response = super::TagTemplateField;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTagTemplateFieldRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).create_tag_template_field(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateTagTemplateFieldSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTagTemplateField" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTagTemplateFieldSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog>
                        tonic::server::UnaryService<super::UpdateTagTemplateFieldRequest>
                        for UpdateTagTemplateFieldSvc<T>
                    {
                        type Response = super::TagTemplateField;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTagTemplateFieldRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).update_tag_template_field(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateTagTemplateFieldSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/RenameTagTemplateField" => {
                    #[allow(non_camel_case_types)]
                    struct RenameTagTemplateFieldSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog>
                        tonic::server::UnaryService<super::RenameTagTemplateFieldRequest>
                        for RenameTagTemplateFieldSvc<T>
                    {
                        type Response = super::TagTemplateField;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RenameTagTemplateFieldRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).rename_tag_template_field(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RenameTagTemplateFieldSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTagTemplateField" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTagTemplateFieldSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog>
                        tonic::server::UnaryService<super::DeleteTagTemplateFieldRequest>
                        for DeleteTagTemplateFieldSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTagTemplateFieldRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).delete_tag_template_field(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteTagTemplateFieldSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/CreateTag" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTagSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::CreateTagRequest> for CreateTagSvc<T> {
                        type Response = super::Tag;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTagRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_tag(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateTagSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/UpdateTag" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTagSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::UpdateTagRequest> for UpdateTagSvc<T> {
                        type Response = super::Tag;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTagRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_tag(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateTagSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/DeleteTag" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTagSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::DeleteTagRequest> for DeleteTagSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTagRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_tag(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteTagSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/ListTags" => {
                    #[allow(non_camel_case_types)]
                    struct ListTagsSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog> tonic::server::UnaryService<super::ListTagsRequest> for ListTagsSvc<T> {
                        type Response = super::ListTagsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTagsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_tags(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListTagsSvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/SetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetIamPolicySvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::SetIamPolicyRequest,
                        > for SetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::SetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetIamPolicySvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/GetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetIamPolicySvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::GetIamPolicyRequest,
                        > for GetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::GetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetIamPolicySvc(inner);
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
                "/google.cloud.datacatalog.v1.DataCatalog/TestIamPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct TestIamPermissionsSvc<T: DataCatalog>(pub Arc<T>);
                    impl<T: DataCatalog>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::TestIamPermissionsRequest,
                        > for TestIamPermissionsSvc<T>
                    {
                        type Response =
                            super::super::super::super::iam::v1::TestIamPermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::TestIamPermissionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).test_iam_permissions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TestIamPermissionsSvc(inner);
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
    impl<T: DataCatalog> Clone for DataCatalogServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DataCatalog> Clone for _Inner<T> {
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
