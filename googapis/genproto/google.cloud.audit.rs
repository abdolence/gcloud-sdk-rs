/// Common audit log format for Google Cloud Platform API operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditLog {
    /// The name of the API service performing the operation. For example,
    /// `"datastore.googleapis.com"`.
    #[prost(string, tag = "7")]
    pub service_name: std::string::String,
    /// The name of the service method or operation.
    /// For API calls, this should be the name of the API method.
    /// For example,
    ///
    ///     "google.datastore.v1.Datastore.RunQuery"
    ///     "google.logging.v1.LoggingService.DeleteLog"
    #[prost(string, tag = "8")]
    pub method_name: std::string::String,
    /// The resource or collection that is the target of the operation.
    /// The name is a scheme-less URI, not including the API service name.
    /// For example:
    ///
    ///     "shelves/SHELF_ID/books"
    ///     "shelves/SHELF_ID/books/BOOK_ID"
    #[prost(string, tag = "11")]
    pub resource_name: std::string::String,
    /// The number of items returned from a List or Query API method,
    /// if applicable.
    #[prost(int64, tag = "12")]
    pub num_response_items: i64,
    /// The status of the overall operation.
    #[prost(message, optional, tag = "2")]
    pub status: ::std::option::Option<super::super::rpc::Status>,
    /// Authentication information.
    #[prost(message, optional, tag = "3")]
    pub authentication_info: ::std::option::Option<AuthenticationInfo>,
    /// Authorization information. If there are multiple
    /// resources or permissions involved, then there is
    /// one AuthorizationInfo element for each {resource, permission} tuple.
    #[prost(message, repeated, tag = "9")]
    pub authorization_info: ::std::vec::Vec<AuthorizationInfo>,
    /// Metadata about the operation.
    #[prost(message, optional, tag = "4")]
    pub request_metadata: ::std::option::Option<RequestMetadata>,
    /// The operation request. This may not include all request parameters,
    /// such as those that are too large, privacy-sensitive, or duplicated
    /// elsewhere in the log record.
    /// It should never include user-generated data, such as file contents.
    /// When the JSON object represented here has a proto equivalent, the proto
    /// name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "16")]
    pub request: ::std::option::Option<::prost_types::Struct>,
    /// The operation response. This may not include all response elements,
    /// such as those that are too large, privacy-sensitive, or duplicated
    /// elsewhere in the log record.
    /// It should never include user-generated data, such as file contents.
    /// When the JSON object represented here has a proto equivalent, the proto
    /// name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "17")]
    pub response: ::std::option::Option<::prost_types::Struct>,
    /// Other service-specific data about the request, response, and other
    /// activities.
    #[prost(message, optional, tag = "15")]
    pub service_data: ::std::option::Option<::prost_types::Any>,
}
/// Authentication information for the operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationInfo {
    /// The email address of the authenticated user making the request.
    #[prost(string, tag = "1")]
    pub principal_email: std::string::String,
}
/// Authorization information for the operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationInfo {
    /// The resource being accessed, as a REST-style string. For example:
    ///
    ///     bigquery.googlapis.com/projects/PROJECTID/datasets/DATASETID
    #[prost(string, tag = "1")]
    pub resource: std::string::String,
    /// The required IAM permission.
    #[prost(string, tag = "2")]
    pub permission: std::string::String,
    /// Whether or not authorization for `resource` and `permission`
    /// was granted.
    #[prost(bool, tag = "3")]
    pub granted: bool,
}
/// Metadata about the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    /// The IP address of the caller.
    #[prost(string, tag = "1")]
    pub caller_ip: std::string::String,
    /// The user agent of the caller.
    /// This information is not authenticated and should be treated accordingly.
    /// For example:
    ///
    /// +   `google-api-python-client/1.4.0`:
    ///     The request was made by the Google API client for Python.
    /// +   `Cloud SDK Command Line Tool apitools-client/1.0 gcloud/0.9.62`:
    ///     The request was made by the Google Cloud SDK CLI (gcloud).
    /// +   `AppEngine-Google; (+http://code.google.com/appengine; appid:
    /// s~my-project`:
    ///     The request was made from the `my-project` App Engine app.
    #[prost(string, tag = "2")]
    pub caller_supplied_user_agent: std::string::String,
}
