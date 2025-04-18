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

/// Flag : A flag resource.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Flag {
    /// Use this field if only certain integers are accepted. Can be combined with min_value and max_value to add additional values.
    #[serde(rename = "allowedIntValues", skip_serializing_if = "Option::is_none")]
    pub allowed_int_values: Option<Vec<String>>,
    /// For `STRING` flags, a list of strings that the value can be set to.
    #[serde(
        rename = "allowedStringValues",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_string_values: Option<Vec<String>>,
    /// The database version this flag applies to. Can be MySQL instances: `MYSQL_8_0`, `MYSQL_8_0_18`, `MYSQL_8_0_26`, `MYSQL_5_7`, or `MYSQL_5_6`. PostgreSQL instances: `POSTGRES_9_6`, `POSTGRES_10`, `POSTGRES_11` or `POSTGRES_12`. SQL Server instances: `SQLSERVER_2017_STANDARD`, `SQLSERVER_2017_ENTERPRISE`, `SQLSERVER_2017_EXPRESS`, `SQLSERVER_2017_WEB`, `SQLSERVER_2019_STANDARD`, `SQLSERVER_2019_ENTERPRISE`, `SQLSERVER_2019_EXPRESS`, or `SQLSERVER_2019_WEB`. See [the complete list](/sql/docs/mysql/admin-api/rest/v1/SqlDatabaseVersion).
    #[serde(rename = "appliesTo", skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<Vec<AppliesTo>>,
    /// Whether or not the flag is considered in beta.
    #[serde(rename = "inBeta", skip_serializing_if = "Option::is_none")]
    pub in_beta: Option<bool>,
    /// This is always `sql#flag`.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// For `INTEGER` flags, the maximum allowed value.
    #[serde(rename = "maxValue", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
    /// For `INTEGER` flags, the minimum allowed value.
    #[serde(rename = "minValue", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
    /// This is the name of the flag. Flag names always use underscores, not hyphens, for example: `max_allowed_packet`
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Indicates whether changing this flag will trigger a database restart. Only applicable to Second Generation instances.
    #[serde(rename = "requiresRestart", skip_serializing_if = "Option::is_none")]
    pub requires_restart: Option<bool>,
    /// The type of the flag. Flags are typed to being `BOOLEAN`, `STRING`, `INTEGER` or `NONE`. `NONE` is used for flags that do not take a value, such as `skip_grant_tables`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl Flag {
    /// A flag resource.
    pub fn new() -> Flag {
        Flag {
            allowed_int_values: None,
            allowed_string_values: None,
            applies_to: None,
            in_beta: None,
            kind: None,
            max_value: None,
            min_value: None,
            name: None,
            requires_restart: None,
            r#type: None,
        }
    }
}
/// The database version this flag applies to. Can be MySQL instances: `MYSQL_8_0`, `MYSQL_8_0_18`, `MYSQL_8_0_26`, `MYSQL_5_7`, or `MYSQL_5_6`. PostgreSQL instances: `POSTGRES_9_6`, `POSTGRES_10`, `POSTGRES_11` or `POSTGRES_12`. SQL Server instances: `SQLSERVER_2017_STANDARD`, `SQLSERVER_2017_ENTERPRISE`, `SQLSERVER_2017_EXPRESS`, `SQLSERVER_2017_WEB`, `SQLSERVER_2019_STANDARD`, `SQLSERVER_2019_ENTERPRISE`, `SQLSERVER_2019_EXPRESS`, or `SQLSERVER_2019_WEB`. See [the complete list](/sql/docs/mysql/admin-api/rest/v1/SqlDatabaseVersion).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AppliesTo {
    #[serde(rename = "SQL_DATABASE_VERSION_UNSPECIFIED")]
    SqlDatabaseVersionUnspecified,
    #[serde(rename = "MYSQL_5_1")]
    Mysql51,
    #[serde(rename = "MYSQL_5_5")]
    Mysql55,
    #[serde(rename = "MYSQL_5_6")]
    Mysql56,
    #[serde(rename = "MYSQL_5_7")]
    Mysql57,
    #[serde(rename = "SQLSERVER_2017_STANDARD")]
    Sqlserver2017Standard,
    #[serde(rename = "SQLSERVER_2017_ENTERPRISE")]
    Sqlserver2017Enterprise,
    #[serde(rename = "SQLSERVER_2017_EXPRESS")]
    Sqlserver2017Express,
    #[serde(rename = "SQLSERVER_2017_WEB")]
    Sqlserver2017Web,
    #[serde(rename = "POSTGRES_9_6")]
    Postgres96,
    #[serde(rename = "POSTGRES_10")]
    Postgres10,
    #[serde(rename = "POSTGRES_11")]
    Postgres11,
    #[serde(rename = "POSTGRES_12")]
    Postgres12,
    #[serde(rename = "POSTGRES_13")]
    Postgres13,
    #[serde(rename = "POSTGRES_14")]
    Postgres14,
    #[serde(rename = "MYSQL_8_0")]
    Mysql80,
    #[serde(rename = "MYSQL_8_0_18")]
    Mysql8018,
    #[serde(rename = "MYSQL_8_0_26")]
    Mysql8026,
    #[serde(rename = "MYSQL_8_0_27")]
    Mysql8027,
    #[serde(rename = "MYSQL_8_0_28")]
    Mysql8028,
    #[serde(rename = "MYSQL_8_0_29")]
    Mysql8029,
    #[serde(rename = "MYSQL_8_0_30")]
    Mysql8030,
    #[serde(rename = "MYSQL_8_0_31")]
    Mysql8031,
    #[serde(rename = "SQLSERVER_2019_STANDARD")]
    Sqlserver2019Standard,
    #[serde(rename = "SQLSERVER_2019_ENTERPRISE")]
    Sqlserver2019Enterprise,
    #[serde(rename = "SQLSERVER_2019_EXPRESS")]
    Sqlserver2019Express,
    #[serde(rename = "SQLSERVER_2019_WEB")]
    Sqlserver2019Web,
}

impl Default for AppliesTo {
    fn default() -> AppliesTo {
        Self::SqlDatabaseVersionUnspecified
    }
}
/// The type of the flag. Flags are typed to being `BOOLEAN`, `STRING`, `INTEGER` or `NONE`. `NONE` is used for flags that do not take a value, such as `skip_grant_tables`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SQL_FLAG_TYPE_UNSPECIFIED")]
    SqlFlagTypeUnspecified,
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "MYSQL_TIMEZONE_OFFSET")]
    MysqlTimezoneOffset,
    #[serde(rename = "FLOAT")]
    Float,
    #[serde(rename = "REPEATED_STRING")]
    RepeatedString,
}

impl Default for Type {
    fn default() -> Type {
        Self::SqlFlagTypeUnspecified
    }
}
