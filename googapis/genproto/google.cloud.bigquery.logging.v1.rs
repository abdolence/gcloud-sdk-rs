/// BigQuery request and response messages for audit log.
/// Note: `Table.schema` has been deprecated in favor of `Table.schemaJson`.
/// `Table.schema` may continue to be present in your logs during this
/// transition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditData {
    /// A job completion event.
    #[prost(message, optional, tag = "17")]
    pub job_completed_event: ::std::option::Option<JobCompletedEvent>,
    /// Request data for each BigQuery method.
    #[prost(oneof = "audit_data::Request", tags = "1, 16, 2, 3, 4, 5, 6, 7, 8")]
    pub request: ::std::option::Option<audit_data::Request>,
    /// Response data for each BigQuery method.
    #[prost(oneof = "audit_data::Response", tags = "9, 10, 11, 12, 18, 13, 14, 15")]
    pub response: ::std::option::Option<audit_data::Response>,
}
pub mod audit_data {
    /// Request data for each BigQuery method.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Table insert request.
        #[prost(message, tag = "1")]
        TableInsertRequest(super::TableInsertRequest),
        /// Table update request.
        #[prost(message, tag = "16")]
        TableUpdateRequest(super::TableUpdateRequest),
        /// Dataset list request.
        #[prost(message, tag = "2")]
        DatasetListRequest(super::DatasetListRequest),
        /// Dataset insert request.
        #[prost(message, tag = "3")]
        DatasetInsertRequest(super::DatasetInsertRequest),
        /// Dataset update request.
        #[prost(message, tag = "4")]
        DatasetUpdateRequest(super::DatasetUpdateRequest),
        /// Job insert request.
        #[prost(message, tag = "5")]
        JobInsertRequest(super::JobInsertRequest),
        /// Job query request.
        #[prost(message, tag = "6")]
        JobQueryRequest(super::JobQueryRequest),
        /// Job get query results request.
        #[prost(message, tag = "7")]
        JobGetQueryResultsRequest(super::JobGetQueryResultsRequest),
        /// Table data-list request.
        #[prost(message, tag = "8")]
        TableDataListRequest(super::TableDataListRequest),
    }
    /// Response data for each BigQuery method.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// Table insert response.
        #[prost(message, tag = "9")]
        TableInsertResponse(super::TableInsertResponse),
        /// Table update response.
        #[prost(message, tag = "10")]
        TableUpdateResponse(super::TableUpdateResponse),
        /// Dataset insert response.
        #[prost(message, tag = "11")]
        DatasetInsertResponse(super::DatasetInsertResponse),
        /// Dataset update response.
        #[prost(message, tag = "12")]
        DatasetUpdateResponse(super::DatasetUpdateResponse),
        /// Job insert response.
        #[prost(message, tag = "18")]
        JobInsertResponse(super::JobInsertResponse),
        /// Job query response.
        #[prost(message, tag = "13")]
        JobQueryResponse(super::JobQueryResponse),
        /// Job get query results response.
        #[prost(message, tag = "14")]
        JobGetQueryResultsResponse(super::JobGetQueryResultsResponse),
        /// Deprecated: Job query-done response. Use this information for usage
        /// analysis.
        #[prost(message, tag = "15")]
        JobQueryDoneResponse(super::JobQueryDoneResponse),
    }
}
/// Table insert request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableInsertRequest {
    /// The new table.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<Table>,
}
/// Table update request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableUpdateRequest {
    /// The table to be updated.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<Table>,
}
/// Table insert response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableInsertResponse {
    /// Final state of the inserted table.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<Table>,
}
/// Table update response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableUpdateResponse {
    /// Final state of the updated table.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<Table>,
}
/// Dataset list request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetListRequest {
    /// Whether to list all datasets, including hidden ones.
    #[prost(bool, tag = "1")]
    pub list_all: bool,
}
/// Dataset insert request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetInsertRequest {
    /// The dataset to be inserted.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<Dataset>,
}
/// Dataset insert response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetInsertResponse {
    /// Final state of the inserted dataset.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<Dataset>,
}
/// Dataset update request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetUpdateRequest {
    /// The dataset to be updated.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<Dataset>,
}
/// Dataset update response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetUpdateResponse {
    /// Final state of the updated dataset.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<Dataset>,
}
/// Job insert request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobInsertRequest {
    /// Job insert request.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<Job>,
}
/// Job insert response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobInsertResponse {
    /// Job insert response.
    #[prost(message, optional, tag = "1")]
    pub resource: ::std::option::Option<Job>,
}
/// Job query request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobQueryRequest {
    /// The query.
    #[prost(string, tag = "1")]
    pub query: std::string::String,
    /// The maximum number of results.
    #[prost(uint32, tag = "2")]
    pub max_results: u32,
    /// The default dataset for tables that do not have a dataset specified.
    #[prost(message, optional, tag = "3")]
    pub default_dataset: ::std::option::Option<DatasetName>,
    /// Project that the query should be charged to.
    #[prost(string, tag = "4")]
    pub project_id: std::string::String,
    /// If true, don't actually run the job. Just check that it would run.
    #[prost(bool, tag = "5")]
    pub dry_run: bool,
}
/// Job query response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobQueryResponse {
    /// The total number of rows in the full query result set.
    #[prost(uint64, tag = "1")]
    pub total_results: u64,
    /// Information about the queried job.
    #[prost(message, optional, tag = "2")]
    pub job: ::std::option::Option<Job>,
}
/// Job getQueryResults request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobGetQueryResultsRequest {
    /// Maximum number of results to return.
    #[prost(uint32, tag = "1")]
    pub max_results: u32,
    /// Zero-based row number at which to start.
    #[prost(uint64, tag = "2")]
    pub start_row: u64,
}
/// Job getQueryResults response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobGetQueryResultsResponse {
    /// Total number of results in query results.
    #[prost(uint64, tag = "1")]
    pub total_results: u64,
    /// The job that was created to run the query.
    /// It completed if `job.status.state` is `DONE`.
    /// It failed if `job.status.errorResult` is also present.
    #[prost(message, optional, tag = "2")]
    pub job: ::std::option::Option<Job>,
}
/// Job getQueryDone response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobQueryDoneResponse {
    /// The job and status information.
    /// The job completed if `job.status.state` is `DONE`.
    #[prost(message, optional, tag = "1")]
    pub job: ::std::option::Option<Job>,
}
/// Query job completed event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobCompletedEvent {
    /// Name of the event.
    #[prost(string, tag = "1")]
    pub event_name: std::string::String,
    /// Job information.
    #[prost(message, optional, tag = "2")]
    pub job: ::std::option::Option<Job>,
}
/// Table data-list request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableDataListRequest {
    /// Starting row offset.
    #[prost(uint64, tag = "1")]
    pub start_row: u64,
    /// Maximum number of results to return.
    #[prost(uint32, tag = "2")]
    pub max_results: u32,
}
/// Describes a BigQuery table.
/// See the [Table](/bigquery/docs/reference/v2/tables) API resource
/// for more details on individual fields.
/// Note: `Table.schema` has been deprecated in favor of `Table.schemaJson`.
/// `Table.schema` may continue to be present in your logs during this
/// transition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    /// The name of the table.
    #[prost(message, optional, tag = "1")]
    pub table_name: ::std::option::Option<TableName>,
    /// User-provided metadata for the table.
    #[prost(message, optional, tag = "2")]
    pub info: ::std::option::Option<TableInfo>,
    /// A JSON representation of the table's schema.
    #[prost(string, tag = "8")]
    pub schema_json: std::string::String,
    /// If present, this is a virtual table defined by a SQL query.
    #[prost(message, optional, tag = "4")]
    pub view: ::std::option::Option<TableViewDefinition>,
    /// The expiration date for the table, after which the table
    /// is deleted and the storage reclaimed.
    /// If not present, the table persists indefinitely.
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time the table was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time the table was last truncated
    /// by an operation with a `writeDisposition` of `WRITE_TRUNCATE`.
    #[prost(message, optional, tag = "7")]
    pub truncate_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// User-provided metadata for a table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableInfo {
    /// A short name for the table, such as`"Analytics Data - Jan 2011"`.
    #[prost(string, tag = "1")]
    pub friendly_name: std::string::String,
    /// A long description, perhaps several paragraphs,
    /// describing the table contents in detail.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
}
/// Describes a virtual table defined by a SQL query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableViewDefinition {
    /// SQL query defining the view.
    #[prost(string, tag = "1")]
    pub query: std::string::String,
}
/// BigQuery dataset information.
/// See the [Dataset](/bigquery/docs/reference/v2/datasets) API resource
/// for more details on individual fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    /// The name of the dataset.
    #[prost(message, optional, tag = "1")]
    pub dataset_name: ::std::option::Option<DatasetName>,
    /// User-provided metadata for the dataset.
    #[prost(message, optional, tag = "2")]
    pub info: ::std::option::Option<DatasetInfo>,
    /// The time the dataset was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time the dataset was last modified.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The access control list for the dataset.
    #[prost(message, optional, tag = "6")]
    pub acl: ::std::option::Option<BigQueryAcl>,
    /// If this field is present, each table that does not specify an
    /// expiration time is assigned an expiration time by adding this
    /// duration to the table's `createTime`.  If this field is empty,
    /// there is no default table expiration time.
    #[prost(message, optional, tag = "8")]
    pub default_table_expire_duration: ::std::option::Option<::prost_types::Duration>,
}
/// User-provided metadata for a dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetInfo {
    /// A short name for the dataset, such as`"Analytics Data 2011"`.
    #[prost(string, tag = "1")]
    pub friendly_name: std::string::String,
    /// A long description, perhaps several paragraphs,
    /// describing the dataset contents in detail.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
}
/// An access control list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryAcl {
    /// Access control entry list.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::std::vec::Vec<big_query_acl::Entry>,
}
pub mod big_query_acl {
    /// Access control entry.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        /// The granted role, which can be `READER`, `WRITER`, or `OWNER`.
        #[prost(string, tag = "1")]
        pub role: std::string::String,
        /// Grants access to a group identified by an email address.
        #[prost(string, tag = "2")]
        pub group_email: std::string::String,
        /// Grants access to a user identified by an email address.
        #[prost(string, tag = "3")]
        pub user_email: std::string::String,
        /// Grants access to all members of a domain.
        #[prost(string, tag = "4")]
        pub domain: std::string::String,
        /// Grants access to special groups. Valid groups are `PROJECT_OWNERS`,
        /// `PROJECT_READERS`, `PROJECT_WRITERS` and `ALL_AUTHENTICATED_USERS`.
        #[prost(string, tag = "5")]
        pub special_group: std::string::String,
        /// Grants access to a BigQuery View.
        #[prost(message, optional, tag = "6")]
        pub view_name: ::std::option::Option<super::TableName>,
    }
}
/// Describes a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Job name.
    #[prost(message, optional, tag = "1")]
    pub job_name: ::std::option::Option<JobName>,
    /// Job configuration.
    #[prost(message, optional, tag = "2")]
    pub job_configuration: ::std::option::Option<JobConfiguration>,
    /// Job status.
    #[prost(message, optional, tag = "3")]
    pub job_status: ::std::option::Option<JobStatus>,
    /// Job statistics.
    #[prost(message, optional, tag = "4")]
    pub job_statistics: ::std::option::Option<JobStatistics>,
}
/// Job configuration information.
/// See the [Jobs](/bigquery/docs/reference/v2/jobs) API resource
/// for more details on individual fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobConfiguration {
    /// If true, don't actually run the job. Just check that it would run.
    #[prost(bool, tag = "9")]
    pub dry_run: bool,
    /// Job configuration information.
    #[prost(oneof = "job_configuration::Configuration", tags = "5, 6, 7, 8")]
    pub configuration: ::std::option::Option<job_configuration::Configuration>,
}
pub mod job_configuration {
    /// Describes a query job, which executes a SQL-like query.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Query {
        /// The SQL query to run.
        #[prost(string, tag = "1")]
        pub query: std::string::String,
        /// The table where results are written.
        #[prost(message, optional, tag = "2")]
        pub destination_table: ::std::option::Option<super::TableName>,
        /// Describes when a job is allowed to create a table:
        /// `CREATE_IF_NEEDED`, `CREATE_NEVER`.
        #[prost(string, tag = "3")]
        pub create_disposition: std::string::String,
        /// Describes how writes affect existing tables:
        /// `WRITE_TRUNCATE`, `WRITE_APPEND`, `WRITE_EMPTY`.
        #[prost(string, tag = "4")]
        pub write_disposition: std::string::String,
        /// If a table name is specified without a dataset in a query,
        /// this dataset will be added to table name.
        #[prost(message, optional, tag = "5")]
        pub default_dataset: ::std::option::Option<super::DatasetName>,
        /// Describes data sources outside BigQuery, if needed.
        #[prost(message, repeated, tag = "6")]
        pub table_definitions: ::std::vec::Vec<super::TableDefinition>,
    }
    /// Describes a load job, which loads data from an external source via
    /// the  import pipeline.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Load {
        /// URIs for the data to be imported. Only Google Cloud Storage URIs are
        /// supported.
        #[prost(string, repeated, tag = "1")]
        pub source_uris: ::std::vec::Vec<std::string::String>,
        /// The table schema in JSON format representation of a TableSchema.
        #[prost(string, tag = "6")]
        pub schema_json: std::string::String,
        /// The table where the imported data is written.
        #[prost(message, optional, tag = "3")]
        pub destination_table: ::std::option::Option<super::TableName>,
        /// Describes when a job is allowed to create a table:
        /// `CREATE_IF_NEEDED`, `CREATE_NEVER`.
        #[prost(string, tag = "4")]
        pub create_disposition: std::string::String,
        /// Describes how writes affect existing tables:
        /// `WRITE_TRUNCATE`, `WRITE_APPEND`, `WRITE_EMPTY`.
        #[prost(string, tag = "5")]
        pub write_disposition: std::string::String,
    }
    /// Describes an extract job, which exports data to an external source
    /// via the  export pipeline.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Extract {
        /// Google Cloud Storage URIs where extracted data should be written.
        #[prost(string, repeated, tag = "1")]
        pub destination_uris: ::std::vec::Vec<std::string::String>,
        /// The source table.
        #[prost(message, optional, tag = "2")]
        pub source_table: ::std::option::Option<super::TableName>,
    }
    /// Describes a copy job, which copies an existing table to another table.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableCopy {
        /// Source tables.
        #[prost(message, repeated, tag = "1")]
        pub source_tables: ::std::vec::Vec<super::TableName>,
        /// Destination table.
        #[prost(message, optional, tag = "2")]
        pub destination_table: ::std::option::Option<super::TableName>,
        /// Describes when a job is allowed to create a table:
        /// `CREATE_IF_NEEDED`, `CREATE_NEVER`.
        #[prost(string, tag = "3")]
        pub create_disposition: std::string::String,
        /// Describes how writes affect existing tables:
        /// `WRITE_TRUNCATE`, `WRITE_APPEND`, `WRITE_EMPTY`.
        #[prost(string, tag = "4")]
        pub write_disposition: std::string::String,
    }
    /// Job configuration information.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Configuration {
        /// Query job information.
        #[prost(message, tag = "5")]
        Query(Query),
        /// Load job information.
        #[prost(message, tag = "6")]
        Load(Load),
        /// Extract job information.
        #[prost(message, tag = "7")]
        Extract(Extract),
        /// TableCopy job information.
        #[prost(message, tag = "8")]
        TableCopy(TableCopy),
    }
}
/// Describes an external data source used in a query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableDefinition {
    /// Name of the table, used in queries.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Google Cloud Storage URIs for the data to be imported.
    #[prost(string, repeated, tag = "2")]
    pub source_uris: ::std::vec::Vec<std::string::String>,
}
/// Running state of a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobStatus {
    /// State of a job: `PENDING`, `RUNNING`, or `DONE`.
    #[prost(string, tag = "1")]
    pub state: std::string::String,
    /// If the job did not complete successfully, this field describes why.
    #[prost(message, optional, tag = "2")]
    pub error: ::std::option::Option<super::super::super::super::rpc::Status>,
}
/// Job statistics that may change after a job starts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobStatistics {
    /// Time when the job was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Time when the job started.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Time when the job ended.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Total bytes processed for a job.
    #[prost(int64, tag = "4")]
    pub total_processed_bytes: i64,
    /// Processed bytes, adjusted by the job's CPU usage.
    #[prost(int64, tag = "5")]
    pub total_billed_bytes: i64,
    /// The tier assigned by CPU-based billing.
    #[prost(int32, tag = "7")]
    pub billing_tier: i32,
}
/// The fully-qualified name for a dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetName {
    /// The project ID.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// The dataset ID within the project.
    #[prost(string, tag = "2")]
    pub dataset_id: std::string::String,
}
/// The fully-qualified name for a table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableName {
    /// The project ID.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// The dataset ID within the project.
    #[prost(string, tag = "2")]
    pub dataset_id: std::string::String,
    /// The table ID of the table within the dataset.
    #[prost(string, tag = "3")]
    pub table_id: std::string::String,
}
/// The fully-qualified name for a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobName {
    /// The project ID.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// The job ID within the project.
    #[prost(string, tag = "2")]
    pub job_id: std::string::String,
}
