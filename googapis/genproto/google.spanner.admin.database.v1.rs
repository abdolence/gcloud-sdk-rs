/// Encapsulates progress related information for a Cloud Spanner long
/// running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationProgress {
    /// Percent completion of the operation.
    /// Values are between 0 and 100 inclusive.
    #[prost(int32, tag = "1")]
    pub progress_percent: i32,
    /// Time the request was received.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// If set, the time at which this operation failed or was completed
    /// successfully.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// A backup of a Cloud Spanner database.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    /// Required for the [CreateBackup][google.spanner.admin.database.v1.DatabaseAdmin.CreateBackup] operation.
    /// Name of the database from which this backup was
    /// created. This needs to be in the same instance as the backup.
    /// Values are of the form
    /// `projects/<project>/instances/<instance>/databases/<database>`.
    #[prost(string, tag = "2")]
    pub database: std::string::String,
    /// Required for the [CreateBackup][google.spanner.admin.database.v1.DatabaseAdmin.CreateBackup]
    /// operation. The expiration time of the backup, with microseconds
    /// granularity that must be at least 6 hours and at most 366 days
    /// from the time the CreateBackup request is processed. Once the `expire_time`
    /// has passed, the backup is eligible to be automatically deleted by Cloud
    /// Spanner to free the resources used by the backup.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only for the [CreateBackup][google.spanner.admin.database.v1.DatabaseAdmin.CreateBackup] operation.
    /// Required for the [UpdateBackup][google.spanner.admin.database.v1.DatabaseAdmin.UpdateBackup] operation.
    ///
    /// A globally unique identifier for the backup which cannot be
    /// changed. Values are of the form
    /// `projects/<project>/instances/<instance>/backups/[a-z][a-z0-9_\-]*[a-z0-9]`
    /// The final segment of the name must be between 2 and 60 characters
    /// in length.
    ///
    /// The backup is stored in the location(s) specified in the instance
    /// configuration of the instance containing the backup, identified
    /// by the prefix of the backup name of the form
    /// `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The backup will contain an externally consistent
    /// copy of the database at the timestamp specified by
    /// `create_time`. `create_time` is approximately the time the
    /// [CreateBackup][google.spanner.admin.database.v1.DatabaseAdmin.CreateBackup] request is received.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Size of the backup in bytes.
    #[prost(int64, tag = "5")]
    pub size_bytes: i64,
    /// Output only. The current state of the backup.
    #[prost(enumeration = "backup::State", tag = "6")]
    pub state: i32,
    /// Output only. The names of the restored databases that reference the backup.
    /// The database names are of
    /// the form `projects/<project>/instances/<instance>/databases/<database>`.
    /// Referencing databases may exist in different instances. The existence of
    /// any referencing database prevents the backup from being deleted. When a
    /// restored database from the backup enters the `READY` state, the reference
    /// to the backup is removed.
    #[prost(string, repeated, tag = "7")]
    pub referencing_databases: ::std::vec::Vec<std::string::String>,
}
pub mod backup {
    /// Indicates the current state of the backup.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified.
        Unspecified = 0,
        /// The pending backup is still being created. Operations on the
        /// backup may fail with `FAILED_PRECONDITION` in this state.
        Creating = 1,
        /// The backup is complete and ready for use.
        Ready = 2,
    }
}
/// The request for [CreateBackup][google.spanner.admin.database.v1.DatabaseAdmin.CreateBackup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupRequest {
    /// Required. The name of the instance in which the backup will be
    /// created. This must be the same instance that contains the database the
    /// backup will be created from. The backup will be stored in the
    /// location(s) specified in the instance configuration of this
    /// instance. Values are of the form
    /// `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The id of the backup to be created. The `backup_id` appended to
    /// `parent` forms the full backup name of the form
    /// `projects/<project>/instances/<instance>/backups/<backup_id>`.
    #[prost(string, tag = "2")]
    pub backup_id: std::string::String,
    /// Required. The backup to create.
    #[prost(message, optional, tag = "3")]
    pub backup: ::std::option::Option<Backup>,
}
/// Metadata type for the operation returned by
/// [CreateBackup][google.spanner.admin.database.v1.DatabaseAdmin.CreateBackup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupMetadata {
    /// The name of the backup being created.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The name of the database the backup is created from.
    #[prost(string, tag = "2")]
    pub database: std::string::String,
    /// The progress of the
    /// [CreateBackup][google.spanner.admin.database.v1.DatabaseAdmin.CreateBackup] operation.
    #[prost(message, optional, tag = "3")]
    pub progress: ::std::option::Option<OperationProgress>,
    /// The time at which cancellation of this operation was received.
    /// [Operations.CancelOperation][google.longrunning.Operations.CancelOperation]
    /// starts asynchronous cancellation on a long-running operation. The server
    /// makes a best effort to cancel the operation, but success is not guaranteed.
    /// Clients can use
    /// [Operations.GetOperation][google.longrunning.Operations.GetOperation] or
    /// other methods to check whether the cancellation succeeded or whether the
    /// operation completed despite cancellation. On successful cancellation,
    /// the operation is not deleted; instead, it becomes an operation with
    /// an [Operation.error][] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(message, optional, tag = "4")]
    pub cancel_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// The request for [UpdateBackup][google.spanner.admin.database.v1.DatabaseAdmin.UpdateBackup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupRequest {
    /// Required. The backup to update. `backup.name`, and the fields to be updated
    /// as specified by `update_mask` are required. Other fields are ignored.
    /// Update is only supported for the following fields:
    ///  * `backup.expire_time`.
    #[prost(message, optional, tag = "1")]
    pub backup: ::std::option::Option<Backup>,
    /// Required. A mask specifying which fields (e.g. `expire_time`) in the
    /// Backup resource should be updated. This mask is relative to the Backup
    /// resource, not to the request message. The field mask must always be
    /// specified; this prevents any future fields from being erased accidentally
    /// by clients that do not know about them.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request for [GetBackup][google.spanner.admin.database.v1.DatabaseAdmin.GetBackup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupRequest {
    /// Required. Name of the backup.
    /// Values are of the form
    /// `projects/<project>/instances/<instance>/backups/<backup>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for [DeleteBackup][google.spanner.admin.database.v1.DatabaseAdmin.DeleteBackup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupRequest {
    /// Required. Name of the backup to delete.
    /// Values are of the form
    /// `projects/<project>/instances/<instance>/backups/<backup>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for [ListBackups][google.spanner.admin.database.v1.DatabaseAdmin.ListBackups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsRequest {
    /// Required. The instance to list backups from.  Values are of the
    /// form `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// An expression that filters the list of returned backups.
    ///
    /// A filter expression consists of a field name, a comparison operator, and a
    /// value for filtering.
    /// The value must be a string, a number, or a boolean. The comparison operator
    /// must be one of: `<`, `>`, `<=`, `>=`, `!=`, `=`, or `:`.
    /// Colon `:` is the contains operator. Filter rules are not case sensitive.
    ///
    /// The following fields in the [Backup][google.spanner.admin.database.v1.Backup] are eligible for filtering:
    ///
    ///   * `name`
    ///   * `database`
    ///   * `state`
    ///   * `create_time` (and values are of the format YYYY-MM-DDTHH:MM:SSZ)
    ///   * `expire_time` (and values are of the format YYYY-MM-DDTHH:MM:SSZ)
    ///   * `size_bytes`
    ///
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. By default, expressions are combined with AND logic, but
    /// you can specify AND, OR, and NOT logic explicitly.
    ///
    /// Here are a few examples:
    ///
    ///   * `name:Howl` - The backup's name contains the string "howl".
    ///   * `database:prod`
    ///          - The database's name contains the string "prod".
    ///   * `state:CREATING` - The backup is pending creation.
    ///   * `state:READY` - The backup is fully created and ready for use.
    ///   * `(name:howl) AND (create_time < \"2018-03-28T14:50:00Z\")`
    ///          - The backup name contains the string "howl" and `create_time`
    ///              of the backup is before 2018-03-28T14:50:00Z.
    ///   * `expire_time < \"2018-03-28T14:50:00Z\"`
    ///          - The backup `expire_time` is before 2018-03-28T14:50:00Z.
    ///   * `size_bytes > 10000000000` - The backup's size is greater than 10GB
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Number of backups to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// [next_page_token][google.spanner.admin.database.v1.ListBackupsResponse.next_page_token] from a
    /// previous [ListBackupsResponse][google.spanner.admin.database.v1.ListBackupsResponse] to the same `parent` and with the same
    /// `filter`.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response for [ListBackups][google.spanner.admin.database.v1.DatabaseAdmin.ListBackups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsResponse {
    /// The list of matching backups. Backups returned are ordered by `create_time`
    /// in descending order, starting from the most recent `create_time`.
    #[prost(message, repeated, tag = "1")]
    pub backups: ::std::vec::Vec<Backup>,
    /// `next_page_token` can be sent in a subsequent
    /// [ListBackups][google.spanner.admin.database.v1.DatabaseAdmin.ListBackups] call to fetch more
    /// of the matching backups.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for
/// [ListBackupOperations][google.spanner.admin.database.v1.DatabaseAdmin.ListBackupOperations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupOperationsRequest {
    /// Required. The instance of the backup operations. Values are of
    /// the form `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// An expression that filters the list of returned backup operations.
    ///
    /// A filter expression consists of a field name, a
    /// comparison operator, and a value for filtering.
    /// The value must be a string, a number, or a boolean. The comparison operator
    /// must be one of: `<`, `>`, `<=`, `>=`, `!=`, `=`, or `:`.
    /// Colon `:` is the contains operator. Filter rules are not case sensitive.
    ///
    /// The following fields in the [operation][google.longrunning.Operation]
    /// are eligible for filtering:
    ///
    ///   * `name` - The name of the long-running operation
    ///   * `done` - False if the operation is in progress, else true.
    ///   * `metadata.@type` - the type of metadata. For example, the type string
    ///      for [CreateBackupMetadata][google.spanner.admin.database.v1.CreateBackupMetadata] is
    ///      `type.googleapis.com/google.spanner.admin.database.v1.CreateBackupMetadata`.
    ///   * `metadata.<field_name>` - any field in metadata.value.
    ///   * `error` - Error associated with the long-running operation.
    ///   * `response.@type` - the type of response.
    ///   * `response.<field_name>` - any field in response.value.
    ///
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. By default, expressions are combined with AND logic, but
    /// you can specify AND, OR, and NOT logic explicitly.
    ///
    /// Here are a few examples:
    ///
    ///   * `done:true` - The operation is complete.
    ///   * `metadata.database:prod` - The database the backup was taken from has
    ///      a name containing the string "prod".
    ///   * `(metadata.@type=type.googleapis.com/google.spanner.admin.database.v1.CreateBackupMetadata) AND` <br/>
    ///     `(metadata.name:howl) AND` <br/>
    ///     `(metadata.progress.start_time < \"2018-03-28T14:50:00Z\") AND` <br/>
    ///     `(error:*)` - Returns operations where:
    ///     * The operation's metadata type is [CreateBackupMetadata][google.spanner.admin.database.v1.CreateBackupMetadata].
    ///     * The backup name contains the string "howl".
    ///     * The operation started before 2018-03-28T14:50:00Z.
    ///     * The operation resulted in an error.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Number of operations to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// [next_page_token][google.spanner.admin.database.v1.ListBackupOperationsResponse.next_page_token]
    /// from a previous [ListBackupOperationsResponse][google.spanner.admin.database.v1.ListBackupOperationsResponse] to the
    /// same `parent` and with the same `filter`.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response for
/// [ListBackupOperations][google.spanner.admin.database.v1.DatabaseAdmin.ListBackupOperations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupOperationsResponse {
    /// The list of matching backup [long-running
    /// operations][google.longrunning.Operation]. Each operation's name will be
    /// prefixed by the backup's name and the operation's
    /// [metadata][google.longrunning.Operation.metadata] will be of type
    /// [CreateBackupMetadata][google.spanner.admin.database.v1.CreateBackupMetadata]. Operations returned include those that are
    /// pending or have completed/failed/canceled within the last 7 days.
    /// Operations returned are ordered by
    /// `operation.metadata.value.progress.start_time` in descending order starting
    /// from the most recently started operation.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::std::vec::Vec<super::super::super::super::longrunning::Operation>,
    /// `next_page_token` can be sent in a subsequent
    /// [ListBackupOperations][google.spanner.admin.database.v1.DatabaseAdmin.ListBackupOperations]
    /// call to fetch more of the matching metadata.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Information about a backup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupInfo {
    /// Name of the backup.
    #[prost(string, tag = "1")]
    pub backup: std::string::String,
    /// The backup contains an externally consistent copy of `source_database` at
    /// the timestamp specified by `create_time`.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Name of the database the backup was created from.
    #[prost(string, tag = "3")]
    pub source_database: std::string::String,
}
/// Information about the database restore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreInfo {
    /// The type of the restore source.
    #[prost(enumeration = "RestoreSourceType", tag = "1")]
    pub source_type: i32,
    /// Information about the source used to restore the database.
    #[prost(oneof = "restore_info::SourceInfo", tags = "2")]
    pub source_info: ::std::option::Option<restore_info::SourceInfo>,
}
pub mod restore_info {
    /// Information about the source used to restore the database.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceInfo {
        /// Information about the backup used to restore the database. The backup
        /// may no longer exist.
        #[prost(message, tag = "2")]
        BackupInfo(super::BackupInfo),
    }
}
/// A Cloud Spanner database.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    /// Required. The name of the database. Values are of the form
    /// `projects/<project>/instances/<instance>/databases/<database>`,
    /// where `<database>` is as specified in the `CREATE DATABASE`
    /// statement. This name can be passed to other API methods to
    /// identify the database.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The current database state.
    #[prost(enumeration = "database::State", tag = "2")]
    pub state: i32,
    /// Output only. If exists, the time at which the database creation started.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Applicable only for restored databases. Contains information
    /// about the restore source.
    #[prost(message, optional, tag = "4")]
    pub restore_info: ::std::option::Option<RestoreInfo>,
}
pub mod database {
    /// Indicates the current state of the database.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified.
        Unspecified = 0,
        /// The database is still being created. Operations on the database may fail
        /// with `FAILED_PRECONDITION` in this state.
        Creating = 1,
        /// The database is fully created and ready for use.
        Ready = 2,
        /// The database is fully created and ready for use, but is still
        /// being optimized for performance and cannot handle full load.
        ///
        /// In this state, the database still references the backup
        /// it was restore from, preventing the backup
        /// from being deleted. When optimizations are complete, the full performance
        /// of the database will be restored, and the database will transition to
        /// `READY` state.
        ReadyOptimizing = 3,
    }
}
/// The request for [ListDatabases][google.spanner.admin.database.v1.DatabaseAdmin.ListDatabases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabasesRequest {
    /// Required. The instance whose databases should be listed.
    /// Values are of the form `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Number of databases to be returned in the response. If 0 or less,
    /// defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// [next_page_token][google.spanner.admin.database.v1.ListDatabasesResponse.next_page_token] from a
    /// previous [ListDatabasesResponse][google.spanner.admin.database.v1.ListDatabasesResponse].
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response for [ListDatabases][google.spanner.admin.database.v1.DatabaseAdmin.ListDatabases].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabasesResponse {
    /// Databases that matched the request.
    #[prost(message, repeated, tag = "1")]
    pub databases: ::std::vec::Vec<Database>,
    /// `next_page_token` can be sent in a subsequent
    /// [ListDatabases][google.spanner.admin.database.v1.DatabaseAdmin.ListDatabases] call to fetch more
    /// of the matching databases.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for [CreateDatabase][google.spanner.admin.database.v1.DatabaseAdmin.CreateDatabase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseRequest {
    /// Required. The name of the instance that will serve the new database.
    /// Values are of the form `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. A `CREATE DATABASE` statement, which specifies the ID of the
    /// new database.  The database ID must conform to the regular expression
    /// `[a-z][a-z0-9_\-]*[a-z0-9]` and be between 2 and 30 characters in length.
    /// If the database ID is a reserved word or if it contains a hyphen, the
    /// database ID must be enclosed in backticks (`` ` ``).
    #[prost(string, tag = "2")]
    pub create_statement: std::string::String,
    /// Optional. A list of DDL statements to run inside the newly created
    /// database. Statements can create tables, indexes, etc. These
    /// statements execute atomically with the creation of the database:
    /// if there is an error in any statement, the database is not created.
    #[prost(string, repeated, tag = "3")]
    pub extra_statements: ::std::vec::Vec<std::string::String>,
}
/// Metadata type for the operation returned by
/// [CreateDatabase][google.spanner.admin.database.v1.DatabaseAdmin.CreateDatabase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatabaseMetadata {
    /// The database being created.
    #[prost(string, tag = "1")]
    pub database: std::string::String,
}
/// The request for [GetDatabase][google.spanner.admin.database.v1.DatabaseAdmin.GetDatabase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseRequest {
    /// Required. The name of the requested database. Values are of the form
    /// `projects/<project>/instances/<instance>/databases/<database>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Enqueues the given DDL statements to be applied, in order but not
/// necessarily all at once, to the database schema at some point (or
/// points) in the future. The server checks that the statements
/// are executable (syntactically valid, name tables that exist, etc.)
/// before enqueueing them, but they may still fail upon
/// later execution (e.g., if a statement from another batch of
/// statements is applied first and it conflicts in some way, or if
/// there is some data-related problem like a `NULL` value in a column to
/// which `NOT NULL` would be added). If a statement fails, all
/// subsequent statements in the batch are automatically cancelled.
///
/// Each batch of statements is assigned a name which can be used with
/// the [Operations][google.longrunning.Operations] API to monitor
/// progress. See the
/// [operation_id][google.spanner.admin.database.v1.UpdateDatabaseDdlRequest.operation_id] field for more
/// details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatabaseDdlRequest {
    /// Required. The database to update.
    #[prost(string, tag = "1")]
    pub database: std::string::String,
    /// Required. DDL statements to be applied to the database.
    #[prost(string, repeated, tag = "2")]
    pub statements: ::std::vec::Vec<std::string::String>,
    /// If empty, the new update request is assigned an
    /// automatically-generated operation ID. Otherwise, `operation_id`
    /// is used to construct the name of the resulting
    /// [Operation][google.longrunning.Operation].
    ///
    /// Specifying an explicit operation ID simplifies determining
    /// whether the statements were executed in the event that the
    /// [UpdateDatabaseDdl][google.spanner.admin.database.v1.DatabaseAdmin.UpdateDatabaseDdl] call is replayed,
    /// or the return value is otherwise lost: the [database][google.spanner.admin.database.v1.UpdateDatabaseDdlRequest.database] and
    /// `operation_id` fields can be combined to form the
    /// [name][google.longrunning.Operation.name] of the resulting
    /// [longrunning.Operation][google.longrunning.Operation]: `<database>/operations/<operation_id>`.
    ///
    /// `operation_id` should be unique within the database, and must be
    /// a valid identifier: `[a-z][a-z0-9_]*`. Note that
    /// automatically-generated operation IDs always begin with an
    /// underscore. If the named operation already exists,
    /// [UpdateDatabaseDdl][google.spanner.admin.database.v1.DatabaseAdmin.UpdateDatabaseDdl] returns
    /// `ALREADY_EXISTS`.
    #[prost(string, tag = "3")]
    pub operation_id: std::string::String,
}
/// Metadata type for the operation returned by
/// [UpdateDatabaseDdl][google.spanner.admin.database.v1.DatabaseAdmin.UpdateDatabaseDdl].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatabaseDdlMetadata {
    /// The database being modified.
    #[prost(string, tag = "1")]
    pub database: std::string::String,
    /// For an update this list contains all the statements. For an
    /// individual statement, this list contains only that statement.
    #[prost(string, repeated, tag = "2")]
    pub statements: ::std::vec::Vec<std::string::String>,
    /// Reports the commit timestamps of all statements that have
    /// succeeded so far, where `commit_timestamps[i]` is the commit
    /// timestamp for the statement `statements[i]`.
    #[prost(message, repeated, tag = "3")]
    pub commit_timestamps: ::std::vec::Vec<::prost_types::Timestamp>,
}
/// The request for [DropDatabase][google.spanner.admin.database.v1.DatabaseAdmin.DropDatabase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropDatabaseRequest {
    /// Required. The database to be dropped.
    #[prost(string, tag = "1")]
    pub database: std::string::String,
}
/// The request for [GetDatabaseDdl][google.spanner.admin.database.v1.DatabaseAdmin.GetDatabaseDdl].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseDdlRequest {
    /// Required. The database whose schema we wish to get.
    #[prost(string, tag = "1")]
    pub database: std::string::String,
}
/// The response for [GetDatabaseDdl][google.spanner.admin.database.v1.DatabaseAdmin.GetDatabaseDdl].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatabaseDdlResponse {
    /// A list of formatted DDL statements defining the schema of the database
    /// specified in the request.
    #[prost(string, repeated, tag = "1")]
    pub statements: ::std::vec::Vec<std::string::String>,
}
/// The request for
/// [ListDatabaseOperations][google.spanner.admin.database.v1.DatabaseAdmin.ListDatabaseOperations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabaseOperationsRequest {
    /// Required. The instance of the database operations.
    /// Values are of the form `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// An expression that filters the list of returned operations.
    ///
    /// A filter expression consists of a field name, a
    /// comparison operator, and a value for filtering.
    /// The value must be a string, a number, or a boolean. The comparison operator
    /// must be one of: `<`, `>`, `<=`, `>=`, `!=`, `=`, or `:`.
    /// Colon `:` is the contains operator. Filter rules are not case sensitive.
    ///
    /// The following fields in the [Operation][google.longrunning.Operation]
    /// are eligible for filtering:
    ///
    ///   * `name` - The name of the long-running operation
    ///   * `done` - False if the operation is in progress, else true.
    ///   * `metadata.@type` - the type of metadata. For example, the type string
    ///      for [RestoreDatabaseMetadata][google.spanner.admin.database.v1.RestoreDatabaseMetadata] is
    ///      `type.googleapis.com/google.spanner.admin.database.v1.RestoreDatabaseMetadata`.
    ///   * `metadata.<field_name>` - any field in metadata.value.
    ///   * `error` - Error associated with the long-running operation.
    ///   * `response.@type` - the type of response.
    ///   * `response.<field_name>` - any field in response.value.
    ///
    /// You can combine multiple expressions by enclosing each expression in
    /// parentheses. By default, expressions are combined with AND logic. However,
    /// you can specify AND, OR, and NOT logic explicitly.
    ///
    /// Here are a few examples:
    ///
    ///   * `done:true` - The operation is complete.
    ///   * `(metadata.@type=type.googleapis.com/google.spanner.admin.database.v1.RestoreDatabaseMetadata) AND` <br/>
    ///     `(metadata.source_type:BACKUP) AND` <br/>
    ///     `(metadata.backup_info.backup:backup_howl) AND` <br/>
    ///     `(metadata.name:restored_howl) AND` <br/>
    ///     `(metadata.progress.start_time < \"2018-03-28T14:50:00Z\") AND` <br/>
    ///     `(error:*)` - Return operations where:
    ///     * The operation's metadata type is [RestoreDatabaseMetadata][google.spanner.admin.database.v1.RestoreDatabaseMetadata].
    ///     * The database is restored from a backup.
    ///     * The backup name contains "backup_howl".
    ///     * The restored database's name contains "restored_howl".
    ///     * The operation started before 2018-03-28T14:50:00Z.
    ///     * The operation resulted in an error.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Number of operations to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// [next_page_token][google.spanner.admin.database.v1.ListDatabaseOperationsResponse.next_page_token]
    /// from a previous [ListDatabaseOperationsResponse][google.spanner.admin.database.v1.ListDatabaseOperationsResponse] to the
    /// same `parent` and with the same `filter`.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response for
/// [ListDatabaseOperations][google.spanner.admin.database.v1.DatabaseAdmin.ListDatabaseOperations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatabaseOperationsResponse {
    /// The list of matching database [long-running
    /// operations][google.longrunning.Operation]. Each operation's name will be
    /// prefixed by the database's name. The operation's
    /// [metadata][google.longrunning.Operation.metadata] field type
    /// `metadata.type_url` describes the type of the metadata.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::std::vec::Vec<super::super::super::super::longrunning::Operation>,
    /// `next_page_token` can be sent in a subsequent
    /// [ListDatabaseOperations][google.spanner.admin.database.v1.DatabaseAdmin.ListDatabaseOperations]
    /// call to fetch more of the matching metadata.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for
/// [RestoreDatabase][google.spanner.admin.database.v1.DatabaseAdmin.RestoreDatabase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreDatabaseRequest {
    /// Required. The name of the instance in which to create the
    /// restored database. This instance must be in the same project and
    /// have the same instance configuration as the instance containing
    /// the source backup. Values are of the form
    /// `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The id of the database to create and restore to. This
    /// database must not already exist. The `database_id` appended to
    /// `parent` forms the full database name of the form
    /// `projects/<project>/instances/<instance>/databases/<database_id>`.
    #[prost(string, tag = "2")]
    pub database_id: std::string::String,
    /// Required. The source from which to restore.
    #[prost(oneof = "restore_database_request::Source", tags = "3")]
    pub source: ::std::option::Option<restore_database_request::Source>,
}
pub mod restore_database_request {
    /// Required. The source from which to restore.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Name of the backup from which to restore.  Values are of the form
        /// `projects/<project>/instances/<instance>/backups/<backup>`.
        #[prost(string, tag = "3")]
        Backup(std::string::String),
    }
}
/// Metadata type for the long-running operation returned by
/// [RestoreDatabase][google.spanner.admin.database.v1.DatabaseAdmin.RestoreDatabase].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreDatabaseMetadata {
    /// Name of the database being created and restored to.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The type of the restore source.
    #[prost(enumeration = "RestoreSourceType", tag = "2")]
    pub source_type: i32,
    /// The progress of the
    /// [RestoreDatabase][google.spanner.admin.database.v1.DatabaseAdmin.RestoreDatabase]
    /// operation.
    #[prost(message, optional, tag = "4")]
    pub progress: ::std::option::Option<OperationProgress>,
    /// The time at which cancellation of this operation was received.
    /// [Operations.CancelOperation][google.longrunning.Operations.CancelOperation]
    /// starts asynchronous cancellation on a long-running operation. The server
    /// makes a best effort to cancel the operation, but success is not guaranteed.
    /// Clients can use
    /// [Operations.GetOperation][google.longrunning.Operations.GetOperation] or
    /// other methods to check whether the cancellation succeeded or whether the
    /// operation completed despite cancellation. On successful cancellation,
    /// the operation is not deleted; instead, it becomes an operation with
    /// an [Operation.error][google.longrunning.Operation.error] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to `Code.CANCELLED`.
    #[prost(message, optional, tag = "5")]
    pub cancel_time: ::std::option::Option<::prost_types::Timestamp>,
    /// If exists, the name of the long-running operation that will be used to
    /// track the post-restore optimization process to optimize the performance of
    /// the restored database, and remove the dependency on the restore source.
    /// The name is of the form
    /// `projects/<project>/instances/<instance>/databases/<database>/operations/<operation>`
    /// where the <database> is the name of database being created and restored to.
    /// The metadata type of the  long-running operation is
    /// [OptimizeRestoredDatabaseMetadata][google.spanner.admin.database.v1.OptimizeRestoredDatabaseMetadata]. This long-running operation will be
    /// automatically created by the system after the RestoreDatabase long-running
    /// operation completes successfully. This operation will not be created if the
    /// restore was not successful.
    #[prost(string, tag = "6")]
    pub optimize_database_operation_name: std::string::String,
    /// Information about the source used to restore the database, as specified by
    /// `source` in [RestoreDatabaseRequest][google.spanner.admin.database.v1.RestoreDatabaseRequest].
    #[prost(oneof = "restore_database_metadata::SourceInfo", tags = "3")]
    pub source_info: ::std::option::Option<restore_database_metadata::SourceInfo>,
}
pub mod restore_database_metadata {
    /// Information about the source used to restore the database, as specified by
    /// `source` in [RestoreDatabaseRequest][google.spanner.admin.database.v1.RestoreDatabaseRequest].
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceInfo {
        /// Information about the backup used to restore the database.
        #[prost(message, tag = "3")]
        BackupInfo(super::BackupInfo),
    }
}
/// Metadata type for the long-running operation used to track the progress
/// of optimizations performed on a newly restored database. This long-running
/// operation is automatically created by the system after the successful
/// completion of a database restore, and cannot be cancelled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizeRestoredDatabaseMetadata {
    /// Name of the restored database being optimized.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The progress of the post-restore optimizations.
    #[prost(message, optional, tag = "2")]
    pub progress: ::std::option::Option<OperationProgress>,
}
/// Indicates the type of the restore source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RestoreSourceType {
    /// No restore associated.
    TypeUnspecified = 0,
    /// A backup was used as the source of the restore.
    Backup = 1,
}
#[doc = r" Generated client implementations."]
pub mod database_admin_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Cloud Spanner Database Admin API"]
    #[doc = ""]
    #[doc = " The Cloud Spanner Database Admin API can be used to create, drop, and"]
    #[doc = " list databases. It also enables updating the schema of pre-existing"]
    #[doc = " databases. It can be also used to create, delete and list backups for a"]
    #[doc = " database and to restore from an existing backup."]
    pub struct DatabaseAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DatabaseAdminClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DatabaseAdminClient<T>
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
        #[doc = " Lists Cloud Spanner databases."]
        pub async fn list_databases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatabasesRequest>,
        ) -> Result<tonic::Response<super::ListDatabasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/ListDatabases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Cloud Spanner database and starts to prepare it for serving."]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format `<database_name>/operations/<operation_id>` and"]
        #[doc = " can be used to track preparation of the database. The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateDatabaseMetadata][google.spanner.admin.database.v1.CreateDatabaseMetadata]. The"]
        #[doc = " [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Database][google.spanner.admin.database.v1.Database], if successful."]
        pub async fn create_database(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatabaseRequest>,
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
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/CreateDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the state of a Cloud Spanner database."]
        pub async fn get_database(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatabaseRequest>,
        ) -> Result<tonic::Response<super::Database>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/GetDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the schema of a Cloud Spanner database by"]
        #[doc = " creating/altering/dropping tables, columns, indexes, etc. The returned"]
        #[doc = " [long-running operation][google.longrunning.Operation] will have a name of"]
        #[doc = " the format `<database_name>/operations/<operation_id>` and can be used to"]
        #[doc = " track execution of the schema change(s). The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [UpdateDatabaseDdlMetadata][google.spanner.admin.database.v1.UpdateDatabaseDdlMetadata].  The operation has no response."]
        pub async fn update_database_ddl(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatabaseDdlRequest>,
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
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/UpdateDatabaseDdl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Drops (aka deletes) a Cloud Spanner database."]
        #[doc = " Completed backups for the database will be retained according to their"]
        #[doc = " `expire_time`."]
        pub async fn drop_database(
            &mut self,
            request: impl tonic::IntoRequest<super::DropDatabaseRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/DropDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the schema of a Cloud Spanner database as a list of formatted"]
        #[doc = " DDL statements. This method does not show pending schema updates, those may"]
        #[doc = " be queried using the [Operations][google.longrunning.Operations] API."]
        pub async fn get_database_ddl(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatabaseDdlRequest>,
        ) -> Result<tonic::Response<super::GetDatabaseDdlResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/GetDatabaseDdl",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on a database or backup resource."]
        #[doc = " Replaces any existing policy."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.databases.setIamPolicy`"]
        #[doc = " permission on [resource][google.iam.v1.SetIamPolicyRequest.resource]."]
        #[doc = " For backups, authorization requires `spanner.backups.setIamPolicy`"]
        #[doc = " permission on [resource][google.iam.v1.SetIamPolicyRequest.resource]."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a database or backup resource."]
        #[doc = " Returns an empty policy if a database or backup exists but does not have a"]
        #[doc = " policy set."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.databases.getIamPolicy` permission on"]
        #[doc = " [resource][google.iam.v1.GetIamPolicyRequest.resource]."]
        #[doc = " For backups, authorization requires `spanner.backups.getIamPolicy`"]
        #[doc = " permission on [resource][google.iam.v1.GetIamPolicyRequest.resource]."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that the caller has on the specified database or backup"]
        #[doc = " resource."]
        #[doc = ""]
        #[doc = " Attempting this RPC on a non-existent Cloud Spanner database will"]
        #[doc = " result in a NOT_FOUND error if the user has"]
        #[doc = " `spanner.databases.list` permission on the containing Cloud"]
        #[doc = " Spanner instance. Otherwise returns an empty set of permissions."]
        #[doc = " Calling this method on a backup that does not exist will"]
        #[doc = " result in a NOT_FOUND error if the user has"]
        #[doc = " `spanner.backups.list` permission on the containing instance."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts creating a new Cloud Spanner Backup."]
        #[doc = " The returned backup [long-running operation][google.longrunning.Operation]"]
        #[doc = " will have a name of the format"]
        #[doc = " `projects/<project>/instances/<instance>/backups/<backup>/operations/<operation_id>`"]
        #[doc = " and can be used to track creation of the backup. The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateBackupMetadata][google.spanner.admin.database.v1.CreateBackupMetadata]. The"]
        #[doc = " [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Backup][google.spanner.admin.database.v1.Backup], if successful. Cancelling the returned operation will stop the"]
        #[doc = " creation and delete the backup."]
        #[doc = " There can be only one pending backup creation per database. Backup creation"]
        #[doc = " of different databases can run concurrently."]
        pub async fn create_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBackupRequest>,
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
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/CreateBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata on a pending or completed [Backup][google.spanner.admin.database.v1.Backup]."]
        pub async fn get_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBackupRequest>,
        ) -> Result<tonic::Response<super::Backup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/GetBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a pending or completed [Backup][google.spanner.admin.database.v1.Backup]."]
        pub async fn update_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBackupRequest>,
        ) -> Result<tonic::Response<super::Backup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/UpdateBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a pending or completed [Backup][google.spanner.admin.database.v1.Backup]."]
        pub async fn delete_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBackupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/DeleteBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists completed and pending backups."]
        #[doc = " Backups returned are ordered by `create_time` in descending order,"]
        #[doc = " starting from the most recent `create_time`."]
        pub async fn list_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBackupsRequest>,
        ) -> Result<tonic::Response<super::ListBackupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/ListBackups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new database by restoring from a completed backup. The new"]
        #[doc = " database must be in the same project and in an instance with the same"]
        #[doc = " instance configuration as the instance containing"]
        #[doc = " the backup. The returned database [long-running"]
        #[doc = " operation][google.longrunning.Operation] has a name of the format"]
        #[doc = " `projects/<project>/instances/<instance>/databases/<database>/operations/<operation_id>`,"]
        #[doc = " and can be used to track the progress of the operation, and to cancel it."]
        #[doc = " The [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [RestoreDatabaseMetadata][google.spanner.admin.database.v1.RestoreDatabaseMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] type"]
        #[doc = " is [Database][google.spanner.admin.database.v1.Database], if"]
        #[doc = " successful. Cancelling the returned operation will stop the restore and"]
        #[doc = " delete the database."]
        #[doc = " There can be only one database being restored into an instance at a time."]
        #[doc = " Once the restore operation completes, a new restore operation can be"]
        #[doc = " initiated, without waiting for the optimize operation associated with the"]
        #[doc = " first restore to complete."]
        pub async fn restore_database(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreDatabaseRequest>,
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
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/RestoreDatabase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists database [longrunning-operations][google.longrunning.Operation]."]
        #[doc = " A database operation has a name of the form"]
        #[doc = " `projects/<project>/instances/<instance>/databases/<database>/operations/<operation>`."]
        #[doc = " The long-running operation"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type"]
        #[doc = " `metadata.type_url` describes the type of the metadata. Operations returned"]
        #[doc = " include those that have completed/failed/canceled within the last 7 days,"]
        #[doc = " and pending operations."]
        pub async fn list_database_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatabaseOperationsRequest>,
        ) -> Result<tonic::Response<super::ListDatabaseOperationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/ListDatabaseOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the backup [long-running operations][google.longrunning.Operation] in"]
        #[doc = " the given instance. A backup operation has a name of the form"]
        #[doc = " `projects/<project>/instances/<instance>/backups/<backup>/operations/<operation>`."]
        #[doc = " The long-running operation"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type"]
        #[doc = " `metadata.type_url` describes the type of the metadata. Operations returned"]
        #[doc = " include those that have completed/failed/canceled within the last 7 days,"]
        #[doc = " and pending operations. Operations returned are ordered by"]
        #[doc = " `operation.metadata.value.progress.start_time` in descending order starting"]
        #[doc = " from the most recently started operation."]
        pub async fn list_backup_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBackupOperationsRequest>,
        ) -> Result<tonic::Response<super::ListBackupOperationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.database.v1.DatabaseAdmin/ListBackupOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DatabaseAdminClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DatabaseAdminClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DatabaseAdminClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod database_admin_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DatabaseAdminServer."]
    #[async_trait]
    pub trait DatabaseAdmin: Send + Sync + 'static {
        #[doc = " Lists Cloud Spanner databases."]
        async fn list_databases(
            &self,
            request: tonic::Request<super::ListDatabasesRequest>,
        ) -> Result<tonic::Response<super::ListDatabasesResponse>, tonic::Status>;
        #[doc = " Creates a new Cloud Spanner database and starts to prepare it for serving."]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format `<database_name>/operations/<operation_id>` and"]
        #[doc = " can be used to track preparation of the database. The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateDatabaseMetadata][google.spanner.admin.database.v1.CreateDatabaseMetadata]. The"]
        #[doc = " [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Database][google.spanner.admin.database.v1.Database], if successful."]
        async fn create_database(
            &self,
            request: tonic::Request<super::CreateDatabaseRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Gets the state of a Cloud Spanner database."]
        async fn get_database(
            &self,
            request: tonic::Request<super::GetDatabaseRequest>,
        ) -> Result<tonic::Response<super::Database>, tonic::Status>;
        #[doc = " Updates the schema of a Cloud Spanner database by"]
        #[doc = " creating/altering/dropping tables, columns, indexes, etc. The returned"]
        #[doc = " [long-running operation][google.longrunning.Operation] will have a name of"]
        #[doc = " the format `<database_name>/operations/<operation_id>` and can be used to"]
        #[doc = " track execution of the schema change(s). The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [UpdateDatabaseDdlMetadata][google.spanner.admin.database.v1.UpdateDatabaseDdlMetadata].  The operation has no response."]
        async fn update_database_ddl(
            &self,
            request: tonic::Request<super::UpdateDatabaseDdlRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Drops (aka deletes) a Cloud Spanner database."]
        #[doc = " Completed backups for the database will be retained according to their"]
        #[doc = " `expire_time`."]
        async fn drop_database(
            &self,
            request: tonic::Request<super::DropDatabaseRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Returns the schema of a Cloud Spanner database as a list of formatted"]
        #[doc = " DDL statements. This method does not show pending schema updates, those may"]
        #[doc = " be queried using the [Operations][google.longrunning.Operations] API."]
        async fn get_database_ddl(
            &self,
            request: tonic::Request<super::GetDatabaseDdlRequest>,
        ) -> Result<tonic::Response<super::GetDatabaseDdlResponse>, tonic::Status>;
        #[doc = " Sets the access control policy on a database or backup resource."]
        #[doc = " Replaces any existing policy."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.databases.setIamPolicy`"]
        #[doc = " permission on [resource][google.iam.v1.SetIamPolicyRequest.resource]."]
        #[doc = " For backups, authorization requires `spanner.backups.setIamPolicy`"]
        #[doc = " permission on [resource][google.iam.v1.SetIamPolicyRequest.resource]."]
        async fn set_iam_policy(
            &self,
            request: tonic::Request<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        >;
        #[doc = " Gets the access control policy for a database or backup resource."]
        #[doc = " Returns an empty policy if a database or backup exists but does not have a"]
        #[doc = " policy set."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.databases.getIamPolicy` permission on"]
        #[doc = " [resource][google.iam.v1.GetIamPolicyRequest.resource]."]
        #[doc = " For backups, authorization requires `spanner.backups.getIamPolicy`"]
        #[doc = " permission on [resource][google.iam.v1.GetIamPolicyRequest.resource]."]
        async fn get_iam_policy(
            &self,
            request: tonic::Request<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        >;
        #[doc = " Returns permissions that the caller has on the specified database or backup"]
        #[doc = " resource."]
        #[doc = ""]
        #[doc = " Attempting this RPC on a non-existent Cloud Spanner database will"]
        #[doc = " result in a NOT_FOUND error if the user has"]
        #[doc = " `spanner.databases.list` permission on the containing Cloud"]
        #[doc = " Spanner instance. Otherwise returns an empty set of permissions."]
        #[doc = " Calling this method on a backup that does not exist will"]
        #[doc = " result in a NOT_FOUND error if the user has"]
        #[doc = " `spanner.backups.list` permission on the containing instance."]
        async fn test_iam_permissions(
            &self,
            request: tonic::Request<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        >;
        #[doc = " Starts creating a new Cloud Spanner Backup."]
        #[doc = " The returned backup [long-running operation][google.longrunning.Operation]"]
        #[doc = " will have a name of the format"]
        #[doc = " `projects/<project>/instances/<instance>/backups/<backup>/operations/<operation_id>`"]
        #[doc = " and can be used to track creation of the backup. The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateBackupMetadata][google.spanner.admin.database.v1.CreateBackupMetadata]. The"]
        #[doc = " [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Backup][google.spanner.admin.database.v1.Backup], if successful. Cancelling the returned operation will stop the"]
        #[doc = " creation and delete the backup."]
        #[doc = " There can be only one pending backup creation per database. Backup creation"]
        #[doc = " of different databases can run concurrently."]
        async fn create_backup(
            &self,
            request: tonic::Request<super::CreateBackupRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Gets metadata on a pending or completed [Backup][google.spanner.admin.database.v1.Backup]."]
        async fn get_backup(
            &self,
            request: tonic::Request<super::GetBackupRequest>,
        ) -> Result<tonic::Response<super::Backup>, tonic::Status>;
        #[doc = " Updates a pending or completed [Backup][google.spanner.admin.database.v1.Backup]."]
        async fn update_backup(
            &self,
            request: tonic::Request<super::UpdateBackupRequest>,
        ) -> Result<tonic::Response<super::Backup>, tonic::Status>;
        #[doc = " Deletes a pending or completed [Backup][google.spanner.admin.database.v1.Backup]."]
        async fn delete_backup(
            &self,
            request: tonic::Request<super::DeleteBackupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists completed and pending backups."]
        #[doc = " Backups returned are ordered by `create_time` in descending order,"]
        #[doc = " starting from the most recent `create_time`."]
        async fn list_backups(
            &self,
            request: tonic::Request<super::ListBackupsRequest>,
        ) -> Result<tonic::Response<super::ListBackupsResponse>, tonic::Status>;
        #[doc = " Create a new database by restoring from a completed backup. The new"]
        #[doc = " database must be in the same project and in an instance with the same"]
        #[doc = " instance configuration as the instance containing"]
        #[doc = " the backup. The returned database [long-running"]
        #[doc = " operation][google.longrunning.Operation] has a name of the format"]
        #[doc = " `projects/<project>/instances/<instance>/databases/<database>/operations/<operation_id>`,"]
        #[doc = " and can be used to track the progress of the operation, and to cancel it."]
        #[doc = " The [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [RestoreDatabaseMetadata][google.spanner.admin.database.v1.RestoreDatabaseMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] type"]
        #[doc = " is [Database][google.spanner.admin.database.v1.Database], if"]
        #[doc = " successful. Cancelling the returned operation will stop the restore and"]
        #[doc = " delete the database."]
        #[doc = " There can be only one database being restored into an instance at a time."]
        #[doc = " Once the restore operation completes, a new restore operation can be"]
        #[doc = " initiated, without waiting for the optimize operation associated with the"]
        #[doc = " first restore to complete."]
        async fn restore_database(
            &self,
            request: tonic::Request<super::RestoreDatabaseRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Lists database [longrunning-operations][google.longrunning.Operation]."]
        #[doc = " A database operation has a name of the form"]
        #[doc = " `projects/<project>/instances/<instance>/databases/<database>/operations/<operation>`."]
        #[doc = " The long-running operation"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type"]
        #[doc = " `metadata.type_url` describes the type of the metadata. Operations returned"]
        #[doc = " include those that have completed/failed/canceled within the last 7 days,"]
        #[doc = " and pending operations."]
        async fn list_database_operations(
            &self,
            request: tonic::Request<super::ListDatabaseOperationsRequest>,
        ) -> Result<tonic::Response<super::ListDatabaseOperationsResponse>, tonic::Status>;
        #[doc = " Lists the backup [long-running operations][google.longrunning.Operation] in"]
        #[doc = " the given instance. A backup operation has a name of the form"]
        #[doc = " `projects/<project>/instances/<instance>/backups/<backup>/operations/<operation>`."]
        #[doc = " The long-running operation"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type"]
        #[doc = " `metadata.type_url` describes the type of the metadata. Operations returned"]
        #[doc = " include those that have completed/failed/canceled within the last 7 days,"]
        #[doc = " and pending operations. Operations returned are ordered by"]
        #[doc = " `operation.metadata.value.progress.start_time` in descending order starting"]
        #[doc = " from the most recently started operation."]
        async fn list_backup_operations(
            &self,
            request: tonic::Request<super::ListBackupOperationsRequest>,
        ) -> Result<tonic::Response<super::ListBackupOperationsResponse>, tonic::Status>;
    }
    #[doc = " Cloud Spanner Database Admin API"]
    #[doc = ""]
    #[doc = " The Cloud Spanner Database Admin API can be used to create, drop, and"]
    #[doc = " list databases. It also enables updating the schema of pre-existing"]
    #[doc = " databases. It can be also used to create, delete and list backups for a"]
    #[doc = " database and to restore from an existing backup."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct DatabaseAdminServer<T: DatabaseAdmin> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DatabaseAdmin> DatabaseAdminServer<T> {
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
    impl<T, B> Service<http::Request<B>> for DatabaseAdminServer<T>
    where
        T: DatabaseAdmin,
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/ListDatabases" => {
                    #[allow(non_camel_case_types)]
                    struct ListDatabasesSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin> tonic::server::UnaryService<super::ListDatabasesRequest>
                        for ListDatabasesSvc<T>
                    {
                        type Response = super::ListDatabasesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDatabasesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_databases(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListDatabasesSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/CreateDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDatabaseSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin> tonic::server::UnaryService<super::CreateDatabaseRequest>
                        for CreateDatabaseSvc<T>
                    {
                        type Response = super::super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_database(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateDatabaseSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/GetDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatabaseSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin> tonic::server::UnaryService<super::GetDatabaseRequest>
                        for GetDatabaseSvc<T>
                    {
                        type Response = super::Database;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_database(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetDatabaseSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/UpdateDatabaseDdl" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDatabaseDdlSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin>
                        tonic::server::UnaryService<super::UpdateDatabaseDdlRequest>
                        for UpdateDatabaseDdlSvc<T>
                    {
                        type Response = super::super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDatabaseDdlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_database_ddl(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateDatabaseDdlSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/DropDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct DropDatabaseSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin> tonic::server::UnaryService<super::DropDatabaseRequest>
                        for DropDatabaseSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DropDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.drop_database(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DropDatabaseSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/GetDatabaseDdl" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatabaseDdlSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin> tonic::server::UnaryService<super::GetDatabaseDdlRequest>
                        for GetDatabaseDdlSvc<T>
                    {
                        type Response = super::GetDatabaseDdlResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatabaseDdlRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_database_ddl(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetDatabaseDdlSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/SetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetIamPolicySvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin>
                        tonic::server::UnaryService<
                            super::super::super::super::super::iam::v1::SetIamPolicyRequest,
                        > for SetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_iam_policy(request).await };
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/GetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetIamPolicySvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin>
                        tonic::server::UnaryService<
                            super::super::super::super::super::iam::v1::GetIamPolicyRequest,
                        > for GetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_iam_policy(request).await };
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/TestIamPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct TestIamPermissionsSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin>
                        tonic::server::UnaryService<
                            super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
                        > for TestIamPermissionsSvc<T>
                    {
                        type Response =
                            super::super::super::super::super::iam::v1::TestIamPermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request : tonic :: Request < super :: super :: super :: super :: super :: iam :: v1 :: TestIamPermissionsRequest >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.test_iam_permissions(request).await };
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/CreateBackup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBackupSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin> tonic::server::UnaryService<super::CreateBackupRequest>
                        for CreateBackupSvc<T>
                    {
                        type Response = super::super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBackupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_backup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateBackupSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/GetBackup" => {
                    #[allow(non_camel_case_types)]
                    struct GetBackupSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin> tonic::server::UnaryService<super::GetBackupRequest> for GetBackupSvc<T> {
                        type Response = super::Backup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBackupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_backup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBackupSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/UpdateBackup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBackupSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin> tonic::server::UnaryService<super::UpdateBackupRequest>
                        for UpdateBackupSvc<T>
                    {
                        type Response = super::Backup;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateBackupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_backup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateBackupSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/DeleteBackup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBackupSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin> tonic::server::UnaryService<super::DeleteBackupRequest>
                        for DeleteBackupSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBackupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_backup(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteBackupSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/ListBackups" => {
                    #[allow(non_camel_case_types)]
                    struct ListBackupsSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin> tonic::server::UnaryService<super::ListBackupsRequest>
                        for ListBackupsSvc<T>
                    {
                        type Response = super::ListBackupsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBackupsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_backups(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListBackupsSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/RestoreDatabase" => {
                    #[allow(non_camel_case_types)]
                    struct RestoreDatabaseSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin>
                        tonic::server::UnaryService<super::RestoreDatabaseRequest>
                        for RestoreDatabaseSvc<T>
                    {
                        type Response = super::super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestoreDatabaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.restore_database(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RestoreDatabaseSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/ListDatabaseOperations" => {
                    #[allow(non_camel_case_types)]
                    struct ListDatabaseOperationsSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin>
                        tonic::server::UnaryService<super::ListDatabaseOperationsRequest>
                        for ListDatabaseOperationsSvc<T>
                    {
                        type Response = super::ListDatabaseOperationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDatabaseOperationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_database_operations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListDatabaseOperationsSvc(inner);
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
                "/google.spanner.admin.database.v1.DatabaseAdmin/ListBackupOperations" => {
                    #[allow(non_camel_case_types)]
                    struct ListBackupOperationsSvc<T: DatabaseAdmin>(pub Arc<T>);
                    impl<T: DatabaseAdmin>
                        tonic::server::UnaryService<super::ListBackupOperationsRequest>
                        for ListBackupOperationsSvc<T>
                    {
                        type Response = super::ListBackupOperationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBackupOperationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_backup_operations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListBackupOperationsSvc(inner);
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
    impl<T: DatabaseAdmin> Clone for DatabaseAdminServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DatabaseAdmin> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DatabaseAdmin> tonic::transport::NamedService for DatabaseAdminServer<T> {
        const NAME: &'static str = "google.spanner.admin.database.v1.DatabaseAdmin";
    }
}
