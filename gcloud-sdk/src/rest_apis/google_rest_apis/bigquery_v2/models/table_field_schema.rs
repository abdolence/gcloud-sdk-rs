use serde::{Deserialize, Serialize}; /*
                                      * BigQuery API
                                      *
                                      * A data platform for customers to create, manage, share and query data.
                                      *
                                      * The version of the OpenAPI document: v2
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::bigquery_v2::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableFieldSchema {
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Box<models::TableFieldSchemaCategories>>,
    /// Optional. Collation specification of the field. It only can be set on string type field.
    #[serde(rename = "collation", skip_serializing_if = "Option::is_none")]
    pub collation: Option<String>,
    /// Optional. A SQL expression to specify the default value for this field. It can only be set for top level fields (columns). You can use struct or array expression to specify default value for the entire struct or array. The valid SQL expressions are: - Literals for all data types, including STRUCT and ARRAY. - Following functions: - CURRENT_TIMESTAMP - CURRENT_TIME - CURRENT_DATE - CURRENT_DATETIME - GENERATE_UUID - RAND - SESSION_USER - ST_GEOGPOINT - Struct or array composed with the above allowed functions, for example, [CURRENT_DATE(), DATE '2020-01-01']
    #[serde(
        rename = "defaultValueExpression",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_value_expression: Option<String>,
    /// [Optional] The field description. The maximum length is 1,024 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Optional] Describes the nested schema fields if the type property is set to RECORD.
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<models::TableFieldSchema>>,
    /// [Optional] Maximum length of values of this field for STRINGS or BYTES. If max_length is not specified, no maximum length constraint is imposed on this field. If type = \"STRING\", then max_length represents the maximum UTF-8 length of strings in this field. If type = \"BYTES\", then max_length represents the maximum number of bytes in this field. It is invalid to set this field if type ≠ \"STRING\" and ≠ \"BYTES\".
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,
    /// [Optional] The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The default value is NULLABLE.
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// [Required] The field name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_), and must start with a letter or underscore. The maximum length is 300 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyTags", skip_serializing_if = "Option::is_none")]
    pub policy_tags: Option<Box<models::TableFieldSchemaPolicyTags>>,
    /// [Optional] Precision (maximum number of total digits in base 10) and scale (maximum number of digits in the fractional part in base 10) constraints for values of this field for NUMERIC or BIGNUMERIC. It is invalid to set precision or scale if type ≠ \"NUMERIC\" and ≠ \"BIGNUMERIC\". If precision and scale are not specified, no value range constraint is imposed on this field insofar as values are permitted by the type. Values of this NUMERIC or BIGNUMERIC field must be in this range when: - Precision (P) and scale (S) are specified: [-10P-S + 10-S, 10P-S - 10-S] - Precision (P) is specified but not scale (and thus scale is interpreted to be equal to zero): [-10P + 1, 10P - 1]. Acceptable values for precision and scale if both are specified: - If type = \"NUMERIC\": 1 ≤ precision - scale ≤ 29 and 0 ≤ scale ≤ 9. - If type = \"BIGNUMERIC\": 1 ≤ precision - scale ≤ 38 and 0 ≤ scale ≤ 38. Acceptable values for precision if only precision is specified but not scale (and thus scale is interpreted to be equal to zero): - If type = \"NUMERIC\": 1 ≤ precision ≤ 29. - If type = \"BIGNUMERIC\": 1 ≤ precision ≤ 38. If scale is specified but not precision, then it is invalid.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    /// [Optional] See documentation for precision.
    #[serde(rename = "scale", skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,
    /// [Required] The field data type. Possible values include STRING, BYTES, INTEGER, INT64 (same as INTEGER), FLOAT, FLOAT64 (same as FLOAT), NUMERIC, BIGNUMERIC, BOOLEAN, BOOL (same as BOOLEAN), TIMESTAMP, DATE, TIME, DATETIME, INTERVAL, RECORD (where RECORD indicates that the field contains a nested schema) or STRUCT (same as RECORD).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl TableFieldSchema {
    pub fn new() -> TableFieldSchema {
        TableFieldSchema {
            categories: None,
            collation: None,
            default_value_expression: None,
            description: None,
            fields: None,
            max_length: None,
            mode: None,
            name: None,
            policy_tags: None,
            precision: None,
            scale: None,
            r#type: None,
        }
    }
}
