use serde::{Deserialize, Serialize}; /*
                                      * Cloud SQL Admin API
                                      *
                                      * API for Cloud SQL database instance management
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::sqladmin_v1::models;

/// ExportContext : Database instance export context.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportContext {
    #[serde(rename = "bakExportOptions", skip_serializing_if = "Option::is_none")]
    pub bak_export_options: Option<Box<models::ExportContextBakExportOptions>>,
    #[serde(rename = "csvExportOptions", skip_serializing_if = "Option::is_none")]
    pub csv_export_options: Option<Box<models::ExportContextCsvExportOptions>>,
    /// Databases to be exported. `MySQL instances:` If `fileType` is `SQL` and no database is specified, all databases are exported, except for the `mysql` system database. If `fileType` is `CSV`, you can specify one database, either by using this property or by using the `csvExportOptions.selectQuery` property, which takes precedence over this property. `PostgreSQL instances:` You must specify one database to be exported. If `fileType` is `CSV`, this database must match the one specified in the `csvExportOptions.selectQuery` property. `SQL Server instances:` You must specify one database to be exported, and the `fileType` must be `BAK`.
    #[serde(rename = "databases", skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<String>>,
    /// The file type for the specified uri.
    #[serde(rename = "fileType", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<FileType>,
    /// This is always `sql#exportContext`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Option for export offload.
    #[serde(rename = "offload", skip_serializing_if = "Option::is_none")]
    pub offload: Option<bool>,
    #[serde(rename = "sqlExportOptions", skip_serializing_if = "Option::is_none")]
    pub sql_export_options: Option<Box<models::ExportContextSqlExportOptions>>,
    /// The path to the file in Google Cloud Storage where the export will be stored. The URI is in the form `gs://bucketName/fileName`. If the file already exists, the request succeeds, but the operation fails. If `fileType` is `SQL` and the filename ends with .gz, the contents are compressed.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ExportContext {
    /// Database instance export context.
    pub fn new() -> ExportContext {
        ExportContext {
            bak_export_options: None,
            csv_export_options: None,
            databases: None,
            file_type: None,
            kind: None,
            offload: None,
            sql_export_options: None,
            uri: None,
        }
    }
}
/// The file type for the specified uri.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FileType {
    #[serde(rename = "SQL_FILE_TYPE_UNSPECIFIED")]
    SqlFileTypeUnspecified,
    #[serde(rename = "SQL")]
    Sql,
    #[serde(rename = "CSV")]
    Csv,
    #[serde(rename = "BAK")]
    Bak,
}

impl Default for FileType {
    fn default() -> FileType {
        Self::SqlFileTypeUnspecified
    }
}
