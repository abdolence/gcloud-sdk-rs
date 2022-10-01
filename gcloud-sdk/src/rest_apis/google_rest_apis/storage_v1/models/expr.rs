use serde::{Deserialize, Serialize}; /*
                                      * Cloud Storage JSON API
                                      *
                                      * Stores and retrieves potentially large, immutable data objects.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// Expr : Represents an expression text. Example: title: \"User account presence\" description: \"Determines whether the request has a user account\" expression: \"size(request.user) > 0\"

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Expr {
    /// An optional description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Textual representation of an expression in Common Expression Language syntax. The application context of the containing message determines which well-known feature set of CEL is supported.
    #[serde(rename = "expression", skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// An optional string indicating the location of the expression for error reporting, e.g. a file name and a position in the file.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// An optional title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl Expr {
    /// Represents an expression text. Example: title: \"User account presence\" description: \"Determines whether the request has a user account\" expression: \"size(request.user) > 0\"
    pub fn new() -> Expr {
        Expr {
            description: None,
            expression: None,
            location: None,
            title: None,
        }
    }
}