use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JobConfigurationLoad {
    /// [Optional] Accept rows that are missing trailing optional columns. The missing values are treated as nulls. If false, records with missing trailing columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. Only applicable to CSV, ignored for other formats.
    #[serde(rename = "allowJaggedRows", skip_serializing_if = "Option::is_none")]
    pub allow_jagged_rows: Option<bool>,
    /// Indicates if BigQuery should allow quoted data sections that contain newline characters in a CSV file. The default value is false.
    #[serde(
        rename = "allowQuotedNewlines",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_quoted_newlines: Option<bool>,
    /// [Optional] Indicates if we should automatically infer the options and schema for CSV and JSON sources.
    #[serde(rename = "autodetect", skip_serializing_if = "Option::is_none")]
    pub autodetect: Option<bool>,
    #[serde(rename = "clustering", skip_serializing_if = "Option::is_none")]
    pub clustering: Option<Box<crate::google_rest_apis::bigquery_v2::models::Clustering>>,
    /// Connection properties.
    #[serde(
        rename = "connectionProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_properties:
        Option<Vec<crate::google_rest_apis::bigquery_v2::models::ConnectionProperty>>,
    /// [Optional] Specifies whether the job is allowed to create new tables. The following values are supported: CREATE_IF_NEEDED: If the table does not exist, BigQuery creates the table. CREATE_NEVER: The table must already exist. If it does not, a 'notFound' error is returned in the job result. The default value is CREATE_IF_NEEDED. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename = "createDisposition", skip_serializing_if = "Option::is_none")]
    pub create_disposition: Option<String>,
    /// If true, creates a new session, where session id will be a server generated random id. If false, runs query with an existing session_id passed in ConnectionProperty, otherwise runs the load job in non-session mode.
    #[serde(rename = "createSession", skip_serializing_if = "Option::is_none")]
    pub create_session: Option<bool>,
    /// [Optional] Defines the list of possible SQL data types to which the source decimal values are converted. This list and the precision and the scale parameters of the decimal field determine the target type. In the order of NUMERIC, BIGNUMERIC, and STRING, a type is picked if it is in the specified list and if it supports the precision and the scale. STRING supports all precision and scale values. If none of the listed types supports the precision and the scale, the type supporting the widest range in the specified list is picked, and if a value exceeds the supported range when reading the data, an error will be thrown. Example: Suppose the value of this field is [\"NUMERIC\", \"BIGNUMERIC\"]. If (precision,scale) is: (38,9) -> NUMERIC; (39,9) -> BIGNUMERIC (NUMERIC cannot hold 30 integer digits); (38,10) -> BIGNUMERIC (NUMERIC cannot hold 10 fractional digits); (76,38) -> BIGNUMERIC; (77,38) -> BIGNUMERIC (error if value exeeds supported range). This field cannot contain duplicate types. The order of the types in this field is ignored. For example, [\"BIGNUMERIC\", \"NUMERIC\"] is the same as [\"NUMERIC\", \"BIGNUMERIC\"] and NUMERIC always takes precedence over BIGNUMERIC. Defaults to [\"NUMERIC\", \"STRING\"] for ORC and [\"NUMERIC\"] for the other file formats.
    #[serde(rename = "decimalTargetTypes", skip_serializing_if = "Option::is_none")]
    pub decimal_target_types: Option<Vec<String>>,
    #[serde(
        rename = "destinationEncryptionConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_encryption_configuration:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::EncryptionConfiguration>>,
    #[serde(rename = "destinationTable", skip_serializing_if = "Option::is_none")]
    pub destination_table:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::TableReference>>,
    #[serde(
        rename = "destinationTableProperties",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_table_properties:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::DestinationTableProperties>>,
    /// [Optional] The character encoding of the data. The supported values are UTF-8 or ISO-8859-1. The default value is UTF-8. BigQuery decodes the data after the raw, binary data has been split using the values of the quote and fieldDelimiter properties.
    #[serde(rename = "encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// [Optional] The separator for fields in a CSV file. The separator can be any ISO-8859-1 single-byte character. To use a character in the range 128-255, you must encode the character as UTF8. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. BigQuery also supports the escape sequence \"\\t\" to specify a tab separator. The default value is a comma (',').
    #[serde(rename = "fieldDelimiter", skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(
        rename = "hivePartitioningOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub hive_partitioning_options:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::HivePartitioningOptions>>,
    /// [Optional] Indicates if BigQuery should allow extra values that are not represented in the table schema. If true, the extra values are ignored. If false, records with extra columns are treated as bad records, and if there are too many bad records, an invalid error is returned in the job result. The default value is false. The sourceFormat property determines what BigQuery treats as an extra value: CSV: Trailing columns JSON: Named values that don't match any column names
    #[serde(
        rename = "ignoreUnknownValues",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_unknown_values: Option<bool>,
    /// [Optional] If sourceFormat is set to newline-delimited JSON, indicates whether it should be processed as a JSON variant such as GeoJSON. For a sourceFormat other than JSON, omit this field. If the sourceFormat is newline-delimited JSON: - for newline-delimited GeoJSON: set to GEOJSON.
    #[serde(rename = "jsonExtension", skip_serializing_if = "Option::is_none")]
    pub json_extension: Option<String>,
    /// [Optional] The maximum number of bad records that BigQuery can ignore when running the job. If the number of bad records exceeds this value, an invalid error is returned in the job result. This is only valid for CSV and JSON. The default value is 0, which requires that all records are valid.
    #[serde(rename = "maxBadRecords", skip_serializing_if = "Option::is_none")]
    pub max_bad_records: Option<i32>,
    /// [Optional] Specifies a string that represents a null value in a CSV file. For example, if you specify \"\\N\", BigQuery interprets \"\\N\" as a null value when loading a CSV file. The default value is the empty string. If you set this property to a custom value, BigQuery throws an error if an empty string is present for all data types except for STRING and BYTE. For STRING and BYTE columns, BigQuery interprets the empty string as an empty value.
    #[serde(rename = "nullMarker", skip_serializing_if = "Option::is_none")]
    pub null_marker: Option<String>,
    #[serde(rename = "parquetOptions", skip_serializing_if = "Option::is_none")]
    pub parquet_options: Option<Box<crate::google_rest_apis::bigquery_v2::models::ParquetOptions>>,
    /// [Optional] Preserves the embedded ASCII control characters (the first 32 characters in the ASCII-table, from '\\x00' to '\\x1F') when loading from CSV. Only applicable to CSV, ignored for other formats.
    #[serde(
        rename = "preserveAsciiControlCharacters",
        skip_serializing_if = "Option::is_none"
    )]
    pub preserve_ascii_control_characters: Option<bool>,
    /// If sourceFormat is set to \"DATASTORE_BACKUP\", indicates which entity properties to load into BigQuery from a Cloud Datastore backup. Property names are case sensitive and must be top-level properties. If no properties are specified, BigQuery loads all properties. If any named property isn't found in the Cloud Datastore backup, an invalid error is returned in the job result.
    #[serde(rename = "projectionFields", skip_serializing_if = "Option::is_none")]
    pub projection_fields: Option<Vec<String>>,
    /// [Optional] The value that is used to quote data sections in a CSV file. BigQuery converts the string to ISO-8859-1 encoding, and then uses the first byte of the encoded string to split the data in its raw, binary state. The default value is a double-quote ('\"'). If your data does not contain quoted sections, set the property value to an empty string. If your data contains quoted newline characters, you must also set the allowQuotedNewlines property to true.
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    #[serde(rename = "rangePartitioning", skip_serializing_if = "Option::is_none")]
    pub range_partitioning:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::RangePartitioning>>,
    /// User provided referencing file with the expected reader schema, Available for the format: AVRO, PARQUET, ORC.
    #[serde(
        rename = "referenceFileSchemaUri",
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_file_schema_uri: Option<String>,
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<Box<crate::google_rest_apis::bigquery_v2::models::TableSchema>>,
    /// [Deprecated] The inline schema. For CSV schemas, specify as \"Field1:Type1[,Field2:Type2]*\". For example, \"foo:STRING, bar:INTEGER, baz:FLOAT\".
    #[serde(rename = "schemaInline", skip_serializing_if = "Option::is_none")]
    pub schema_inline: Option<String>,
    /// [Deprecated] The format of the schemaInline property.
    #[serde(rename = "schemaInlineFormat", skip_serializing_if = "Option::is_none")]
    pub schema_inline_format: Option<String>,
    /// Allows the schema of the destination table to be updated as a side effect of the load job if a schema is autodetected or supplied in the job configuration. Schema update options are supported in two cases: when writeDisposition is WRITE_APPEND; when writeDisposition is WRITE_TRUNCATE and the destination table is a partition of a table, specified by partition decorators. For normal tables, WRITE_TRUNCATE will always overwrite the schema. One or more of the following values are specified: ALLOW_FIELD_ADDITION: allow adding a nullable field to the schema. ALLOW_FIELD_RELAXATION: allow relaxing a required field in the original schema to nullable.
    #[serde(
        rename = "schemaUpdateOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub schema_update_options: Option<Vec<String>>,
    /// [Optional] The number of rows at the top of a CSV file that BigQuery will skip when loading the data. The default value is 0. This property is useful if you have header rows in the file that should be skipped.
    #[serde(rename = "skipLeadingRows", skip_serializing_if = "Option::is_none")]
    pub skip_leading_rows: Option<i32>,
    /// [Optional] The format of the data files. For CSV files, specify \"CSV\". For datastore backups, specify \"DATASTORE_BACKUP\". For newline-delimited JSON, specify \"NEWLINE_DELIMITED_JSON\". For Avro, specify \"AVRO\". For parquet, specify \"PARQUET\". For orc, specify \"ORC\". The default value is CSV.
    #[serde(rename = "sourceFormat", skip_serializing_if = "Option::is_none")]
    pub source_format: Option<String>,
    /// [Required] The fully-qualified URIs that point to your data in Google Cloud. For Google Cloud Storage URIs: Each URI can contain one '*' wildcard character and it must come after the 'bucket' name. Size limits related to load jobs apply to external data sources. For Google Cloud Bigtable URIs: Exactly one URI can be specified and it has be a fully specified and valid HTTPS URL for a Google Cloud Bigtable table. For Google Cloud Datastore backups: Exactly one URI can be specified. Also, the '*' wildcard character is not allowed.
    #[serde(rename = "sourceUris", skip_serializing_if = "Option::is_none")]
    pub source_uris: Option<Vec<String>>,
    #[serde(rename = "timePartitioning", skip_serializing_if = "Option::is_none")]
    pub time_partitioning:
        Option<Box<crate::google_rest_apis::bigquery_v2::models::TimePartitioning>>,
    /// [Optional] If sourceFormat is set to \"AVRO\", indicates whether to interpret logical types as the corresponding BigQuery data type (for example, TIMESTAMP), instead of using the raw type (for example, INTEGER).
    #[serde(
        rename = "useAvroLogicalTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_avro_logical_types: Option<bool>,
    /// [Optional] Specifies the action that occurs if the destination table already exists. The following values are supported: WRITE_TRUNCATE: If the table already exists, BigQuery overwrites the table data. WRITE_APPEND: If the table already exists, BigQuery appends the data to the table. WRITE_EMPTY: If the table already exists and contains data, a 'duplicate' error is returned in the job result. The default value is WRITE_APPEND. Each action is atomic and only occurs if BigQuery is able to complete the job successfully. Creation, truncation and append actions occur as one atomic update upon job completion.
    #[serde(rename = "writeDisposition", skip_serializing_if = "Option::is_none")]
    pub write_disposition: Option<String>,
}

impl JobConfigurationLoad {
    pub fn new() -> JobConfigurationLoad {
        JobConfigurationLoad {
            allow_jagged_rows: None,
            allow_quoted_newlines: None,
            autodetect: None,
            clustering: None,
            connection_properties: None,
            create_disposition: None,
            create_session: None,
            decimal_target_types: None,
            destination_encryption_configuration: None,
            destination_table: None,
            destination_table_properties: None,
            encoding: None,
            field_delimiter: None,
            hive_partitioning_options: None,
            ignore_unknown_values: None,
            json_extension: None,
            max_bad_records: None,
            null_marker: None,
            parquet_options: None,
            preserve_ascii_control_characters: None,
            projection_fields: None,
            quote: None,
            range_partitioning: None,
            reference_file_schema_uri: None,
            schema: None,
            schema_inline: None,
            schema_inline_format: None,
            schema_update_options: None,
            skip_leading_rows: None,
            source_format: None,
            source_uris: None,
            time_partitioning: None,
            use_avro_logical_types: None,
            write_disposition: None,
        }
    }
}