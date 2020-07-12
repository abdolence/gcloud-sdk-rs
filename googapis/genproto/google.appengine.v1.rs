/// [Google Cloud
/// Endpoints](https://cloud.google.com/appengine/docs/python/endpoints/)
/// configuration for API handlers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiConfigHandler {
    /// Action to take when users access resources that require
    /// authentication. Defaults to `redirect`.
    #[prost(enumeration = "AuthFailAction", tag = "1")]
    pub auth_fail_action: i32,
    /// Level of login required to access this resource. Defaults to
    /// `optional`.
    #[prost(enumeration = "LoginRequirement", tag = "2")]
    pub login: i32,
    /// Path to the script from the application root directory.
    #[prost(string, tag = "3")]
    pub script: std::string::String,
    /// Security (HTTPS) enforcement for this URL.
    #[prost(enumeration = "SecurityLevel", tag = "4")]
    pub security_level: i32,
    /// URL to serve the endpoint at.
    #[prost(string, tag = "5")]
    pub url: std::string::String,
}
/// Custom static error page to be served when an error occurs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorHandler {
    /// Error condition this handler applies to.
    #[prost(enumeration = "error_handler::ErrorCode", tag = "1")]
    pub error_code: i32,
    /// Static file content to be served for this error.
    #[prost(string, tag = "2")]
    pub static_file: std::string::String,
    /// MIME type of file. Defaults to `text/html`.
    #[prost(string, tag = "3")]
    pub mime_type: std::string::String,
}
pub mod error_handler {
    /// Error codes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ErrorCode {
        /// Not specified. ERROR_CODE_DEFAULT is assumed.
        Unspecified = 0,
        /// Application has exceeded a resource quota.
        OverQuota = 1,
        /// Client blocked by the application's Denial of Service protection
        /// configuration.
        DosApiDenial = 2,
        /// Deadline reached before the application responds.
        Timeout = 3,
    }
}
/// URL pattern and description of how the URL should be handled. App Engine can
/// handle URLs by executing application code or by serving static files
/// uploaded with the version, such as images, CSS, or JavaScript.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlMap {
    /// URL prefix. Uses regular expression syntax, which means regexp
    /// special characters must be escaped, but should not contain groupings.
    /// All URLs that begin with this prefix are handled by this handler, using the
    /// portion of the URL after the prefix as part of the file path.
    #[prost(string, tag = "1")]
    pub url_regex: std::string::String,
    /// Security (HTTPS) enforcement for this URL.
    #[prost(enumeration = "SecurityLevel", tag = "5")]
    pub security_level: i32,
    /// Level of login required to access this resource.
    #[prost(enumeration = "LoginRequirement", tag = "6")]
    pub login: i32,
    /// Action to take when users access resources that require
    /// authentication. Defaults to `redirect`.
    #[prost(enumeration = "AuthFailAction", tag = "7")]
    pub auth_fail_action: i32,
    /// `30x` code to use when performing redirects for the `secure` field.
    /// Defaults to `302`.
    #[prost(enumeration = "url_map::RedirectHttpResponseCode", tag = "8")]
    pub redirect_http_response_code: i32,
    /// Type of handler for this URL pattern.
    #[prost(oneof = "url_map::HandlerType", tags = "2, 3, 4")]
    pub handler_type: ::std::option::Option<url_map::HandlerType>,
}
pub mod url_map {
    /// Redirect codes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RedirectHttpResponseCode {
        /// Not specified. `302` is assumed.
        Unspecified = 0,
        /// `301 Moved Permanently` code.
        RedirectHttpResponseCode301 = 1,
        /// `302 Moved Temporarily` code.
        RedirectHttpResponseCode302 = 2,
        /// `303 See Other` code.
        RedirectHttpResponseCode303 = 3,
        /// `307 Temporary Redirect` code.
        RedirectHttpResponseCode307 = 4,
    }
    /// Type of handler for this URL pattern.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HandlerType {
        /// Returns the contents of a file, such as an image, as the response.
        #[prost(message, tag = "2")]
        StaticFiles(super::StaticFilesHandler),
        /// Executes a script to handle the request that matches this URL
        /// pattern.
        #[prost(message, tag = "3")]
        Script(super::ScriptHandler),
        /// Uses API Endpoints to handle requests.
        #[prost(message, tag = "4")]
        ApiEndpoint(super::ApiEndpointHandler),
    }
}
/// Files served directly to the user for a given URL, such as images, CSS
/// stylesheets, or JavaScript source files. Static file handlers describe which
/// files in the application directory are static files, and which URLs serve
/// them.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticFilesHandler {
    /// Path to the static files matched by the URL pattern, from the
    /// application root directory. The path can refer to text matched in groupings
    /// in the URL pattern.
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// Regular expression that matches the file paths for all files that should be
    /// referenced by this handler.
    #[prost(string, tag = "2")]
    pub upload_path_regex: std::string::String,
    /// HTTP headers to use for all responses from these URLs.
    #[prost(map = "string, string", tag = "3")]
    pub http_headers: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// MIME type used to serve all files served by this handler.
    ///
    /// Defaults to file-specific MIME types, which are derived from each file's
    /// filename extension.
    #[prost(string, tag = "4")]
    pub mime_type: std::string::String,
    /// Time a static file served by this handler should be cached
    /// by web proxies and browsers.
    #[prost(message, optional, tag = "5")]
    pub expiration: ::std::option::Option<::prost_types::Duration>,
    /// Whether this handler should match the request if the file
    /// referenced by the handler does not exist.
    #[prost(bool, tag = "6")]
    pub require_matching_file: bool,
    /// Whether files should also be uploaded as code data. By default, files
    /// declared in static file handlers are uploaded as static
    /// data and are only served to end users; they cannot be read by the
    /// application. If enabled, uploads are charged against both your code and
    /// static data storage resource quotas.
    #[prost(bool, tag = "7")]
    pub application_readable: bool,
}
/// Executes a script to handle the request that matches the URL pattern.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScriptHandler {
    /// Path to the script from the application root directory.
    #[prost(string, tag = "1")]
    pub script_path: std::string::String,
}
/// Uses Google Cloud Endpoints to handle requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiEndpointHandler {
    /// Path to the script from the application root directory.
    #[prost(string, tag = "1")]
    pub script_path: std::string::String,
}
/// Health checking configuration for VM instances. Unhealthy instances
/// are killed and replaced with new instances. Only applicable for
/// instances in App Engine flexible environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheck {
    /// Whether to explicitly disable health checks for this instance.
    #[prost(bool, tag = "1")]
    pub disable_health_check: bool,
    /// Host header to send when performing an HTTP health check.
    /// Example: "myapp.appspot.com"
    #[prost(string, tag = "2")]
    pub host: std::string::String,
    /// Number of consecutive successful health checks required before receiving
    /// traffic.
    #[prost(uint32, tag = "3")]
    pub healthy_threshold: u32,
    /// Number of consecutive failed health checks required before removing
    /// traffic.
    #[prost(uint32, tag = "4")]
    pub unhealthy_threshold: u32,
    /// Number of consecutive failed health checks required before an instance is
    /// restarted.
    #[prost(uint32, tag = "5")]
    pub restart_threshold: u32,
    /// Interval between health checks.
    #[prost(message, optional, tag = "6")]
    pub check_interval: ::std::option::Option<::prost_types::Duration>,
    /// Time before the health check is considered failed.
    #[prost(message, optional, tag = "7")]
    pub timeout: ::std::option::Option<::prost_types::Duration>,
}
/// Third-party Python runtime library that is required by the application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Library {
    /// Name of the library. Example: "django".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Version of the library to select, or "latest".
    #[prost(string, tag = "2")]
    pub version: std::string::String,
}
/// Actions to take when the user is not logged in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthFailAction {
    /// Not specified. `AUTH_FAIL_ACTION_REDIRECT` is assumed.
    Unspecified = 0,
    /// Redirects user to "accounts.google.com". The user is redirected back to the
    /// application URL after signing in or creating an account.
    Redirect = 1,
    /// Rejects request with a `401` HTTP status code and an error
    /// message.
    Unauthorized = 2,
}
/// Methods to restrict access to a URL based on login status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoginRequirement {
    /// Not specified. `LOGIN_OPTIONAL` is assumed.
    LoginUnspecified = 0,
    /// Does not require that the user is signed in.
    LoginOptional = 1,
    /// If the user is not signed in, the `auth_fail_action` is taken.
    /// In addition, if the user is not an administrator for the
    /// application, they are given an error message regardless of
    /// `auth_fail_action`. If the user is an administrator, the handler
    /// proceeds.
    LoginAdmin = 2,
    /// If the user has signed in, the handler proceeds normally. Otherwise, the
    /// auth_fail_action is taken.
    LoginRequired = 3,
}
/// Methods to enforce security (HTTPS) on a URL.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityLevel {
    /// Not specified.
    SecureUnspecified = 0,
    /// Requests for a URL that match this handler that use HTTPS are automatically
    /// redirected to the HTTP equivalent URL.
    SecureNever = 1,
    /// Both HTTP and HTTPS requests with URLs that match the handler succeed
    /// without redirects. The application can examine the request to determine
    /// which protocol was used and respond accordingly.
    SecureOptional = 2,
    /// Requests for a URL that match this handler that do not use HTTPS are
    /// automatically redirected to the HTTPS URL with the same path. Query
    /// parameters are reserved for the redirect.
    SecureAlways = 3,
}
/// An Application resource contains the top-level configuration of an App
/// Engine application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Application {
    /// Full path to the Application resource in the API.
    /// Example: `apps/myapp`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Identifier of the Application resource. This identifier is equivalent
    /// to the project ID of the Google Cloud Platform project where you want to
    /// deploy your application.
    /// Example: `myapp`.
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// HTTP path dispatch rules for requests to the application that do not
    /// explicitly target a service or version. Rules are order-dependent.
    ///
    /// @OutputOnly
    #[prost(message, repeated, tag = "3")]
    pub dispatch_rules: ::std::vec::Vec<UrlDispatchRule>,
    /// Google Apps authentication domain that controls which users can access
    /// this application.
    ///
    /// Defaults to open access for any Google Account.
    #[prost(string, tag = "6")]
    pub auth_domain: std::string::String,
    /// Location from which this application will be run. Application instances
    /// will run out of data centers in the chosen location, which is also where
    /// all of the application's end user content is stored.
    ///
    /// Defaults to `us-central`.
    ///
    /// Options are:
    ///
    /// `us-central` - Central US
    ///
    /// `europe-west` - Western Europe
    ///
    /// `us-east1` - Eastern US
    #[prost(string, tag = "7")]
    pub location_id: std::string::String,
    /// Google Cloud Storage bucket that can be used for storing files
    /// associated with this application. This bucket is associated with the
    /// application and can be used by the gcloud deployment commands.
    ///
    /// @OutputOnly
    #[prost(string, tag = "8")]
    pub code_bucket: std::string::String,
    /// Cookie expiration policy for this application.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "9")]
    pub default_cookie_expiration: ::std::option::Option<::prost_types::Duration>,
    /// Hostname used to reach this application, as resolved by App Engine.
    ///
    /// @OutputOnly
    #[prost(string, tag = "11")]
    pub default_hostname: std::string::String,
    /// Google Cloud Storage bucket that can be used by this application to store
    /// content.
    ///
    /// @OutputOnly
    #[prost(string, tag = "12")]
    pub default_bucket: std::string::String,
}
/// Rules to match an HTTP request and dispatch that request to a service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlDispatchRule {
    /// Domain name to match against. The wildcard "`*`" is supported if
    /// specified before a period: "`*.`".
    ///
    /// Defaults to matching all domains: "`*`".
    #[prost(string, tag = "1")]
    pub domain: std::string::String,
    /// Pathname within the host. Must start with a "`/`". A
    /// single "`*`" can be included at the end of the path. The sum
    /// of the lengths of the domain and path may not exceed 100
    /// characters.
    #[prost(string, tag = "2")]
    pub path: std::string::String,
    /// Resource ID of a service in this application that should
    /// serve the matched request. The service must already
    /// exist. Example: `default`.
    #[prost(string, tag = "3")]
    pub service: std::string::String,
}
/// An Instance resource is the computing unit that App Engine uses to
/// automatically scale an application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Full path to the Instance resource in the API.
    /// Example: `apps/myapp/services/default/versions/v1/instances/instance-1`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Relative name of the instance within the version.
    /// Example: `instance-1`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// App Engine release this instance is running on.
    ///
    /// @OutputOnly
    #[prost(string, tag = "3")]
    pub app_engine_release: std::string::String,
    /// Availability of the instance.
    ///
    /// @OutputOnly
    #[prost(enumeration = "instance::Availability", tag = "4")]
    pub availability: i32,
    /// Name of the virtual machine where this instance lives. Only applicable
    /// for instances in App Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(string, tag = "5")]
    pub vm_name: std::string::String,
    /// Zone where the virtual machine is located. Only applicable for instances
    /// in App Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(string, tag = "6")]
    pub vm_zone_name: std::string::String,
    /// Virtual machine ID of this instance. Only applicable for instances in
    /// App Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(string, tag = "7")]
    pub vm_id: std::string::String,
    /// Time that this instance was started.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "8")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Number of requests since this instance was started.
    ///
    /// @OutputOnly
    #[prost(int32, tag = "9")]
    pub requests: i32,
    /// Number of errors since this instance was started.
    ///
    /// @OutputOnly
    #[prost(int32, tag = "10")]
    pub errors: i32,
    /// Average queries per second (QPS) over the last minute.
    ///
    /// @OutputOnly
    #[prost(float, tag = "11")]
    pub qps: f32,
    /// Average latency (ms) over the last minute.
    ///
    /// @OutputOnly
    #[prost(int32, tag = "12")]
    pub average_latency: i32,
    /// Total memory in use (bytes).
    ///
    /// @OutputOnly
    #[prost(int64, tag = "13")]
    pub memory_usage: i64,
    /// Status of the virtual machine where this instance lives. Only applicable
    /// for instances in App Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(string, tag = "14")]
    pub vm_status: std::string::String,
    /// Whether this instance is in debug mode. Only applicable for instances in
    /// App Engine flexible environment.
    ///
    /// @OutputOnly
    #[prost(bool, tag = "15")]
    pub vm_debug_enabled: bool,
}
pub mod instance {
    /// Availability of the instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Availability {
        Unspecified = 0,
        Resident = 1,
        Dynamic = 2,
    }
}
/// A Service resource is a logical component of an application that can share
/// state and communicate in a secure fashion with other services.
/// For example, an application that handles customer requests might
/// include separate services to handle tasks such as backend data
/// analysis or API requests from mobile devices. Each service has a
/// collection of versions that define a specific set of code used to
/// implement the functionality of that service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Full path to the Service resource in the API.
    /// Example: `apps/myapp/services/default`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Relative name of the service within the application.
    /// Example: `default`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// Mapping that defines fractional HTTP traffic diversion to
    /// different versions within the service.
    #[prost(message, optional, tag = "3")]
    pub split: ::std::option::Option<TrafficSplit>,
}
/// Traffic routing configuration for versions within a single service. Traffic
/// splits define how traffic directed to the service is assigned to versions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficSplit {
    /// Mechanism used to determine which version a request is sent to.
    /// The traffic selection algorithm will
    /// be stable for either type until allocations are changed.
    #[prost(enumeration = "traffic_split::ShardBy", tag = "1")]
    pub shard_by: i32,
    /// Mapping from version IDs within the service to fractional
    /// (0.000, 1] allocations of traffic for that version. Each version can
    /// be specified only once, but some versions in the service may not
    /// have any traffic allocation. Services that have traffic allocated
    /// cannot be deleted until either the service is deleted or
    /// their traffic allocation is removed. Allocations must sum to 1.
    /// Up to two decimal place precision is supported for IP-based splits and
    /// up to three decimal places is supported for cookie-based splits.
    #[prost(map = "string, double", tag = "2")]
    pub allocations: ::std::collections::HashMap<std::string::String, f64>,
}
pub mod traffic_split {
    /// Available sharding mechanisms.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ShardBy {
        /// Diversion method unspecified.
        Unspecified = 0,
        /// Diversion based on a specially named cookie, "GOOGAPPUID." The cookie
        /// must be set by the application itself or no diversion will occur.
        Cookie = 1,
        /// Diversion based on applying the modulus operation to a fingerprint
        /// of the IP address.
        Ip = 2,
    }
}
/// Code and application artifacts used to deploy a version to App Engine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// Manifest of the files stored in Google Cloud Storage that are included
    /// as part of this version. All files must be readable using the
    /// credentials supplied with this call.
    #[prost(map = "string, message", tag = "1")]
    pub files: ::std::collections::HashMap<std::string::String, FileInfo>,
    /// A Docker image that App Engine uses to run the version.
    /// Only applicable for instances in App Engine flexible environment.
    #[prost(message, optional, tag = "2")]
    pub container: ::std::option::Option<ContainerInfo>,
    /// The zip file for this deployment, if this is a zip deployment.
    #[prost(message, optional, tag = "3")]
    pub zip: ::std::option::Option<ZipInfo>,
}
/// Single source file that is part of the version to be deployed. Each source
/// file that is deployed must be specified separately.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    /// URL source to use to fetch this file. Must be a URL to a resource in
    /// Google Cloud Storage in the form
    /// 'http(s)://storage.googleapis.com/\<bucket\>/\<object\>'.
    #[prost(string, tag = "1")]
    pub source_url: std::string::String,
    /// The SHA1 hash of the file, in hex.
    #[prost(string, tag = "2")]
    pub sha1_sum: std::string::String,
    /// The MIME type of the file.
    ///
    /// Defaults to the value from Google Cloud Storage.
    #[prost(string, tag = "3")]
    pub mime_type: std::string::String,
}
/// Docker image that is used to start a VM container for the version you
/// deploy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerInfo {
    /// URI to the hosted container image in a Docker repository. The URI must be
    /// fully qualified and include a tag or digest.
    /// Examples: "gcr.io/my-project/image:tag" or "gcr.io/my-project/image@digest"
    #[prost(string, tag = "1")]
    pub image: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZipInfo {
    /// URL of the zip file to deploy from. Must be a URL to a resource in
    /// Google Cloud Storage in the form
    /// 'http(s)://storage.googleapis.com/\<bucket\>/\<object\>'.
    #[prost(string, tag = "3")]
    pub source_url: std::string::String,
    /// An estimate of the number of files in a zip for a zip deployment.
    /// If set, must be greater than or equal to the actual number of files.
    /// Used for optimizing performance; if not provided, deployment may be slow.
    #[prost(int32, tag = "4")]
    pub files_count: i32,
}
/// A Version resource is a specific set of source code and configuration files
/// that are deployed into a service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Full path to the Version resource in the API.  Example:
    /// `apps/myapp/services/default/versions/v1`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Relative name of the version within the service.  Example: `v1`.
    /// Version names can contain only lowercase letters, numbers, or hyphens.
    /// Reserved names: "default", "latest", and any name with the prefix "ah-".
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// Before an application can receive email or XMPP messages, the application
    /// must be configured to enable the service.
    #[prost(enumeration = "InboundServiceType", repeated, tag = "6")]
    pub inbound_services: ::std::vec::Vec<i32>,
    /// Instance class that is used to run this version. Valid values are:
    /// * AutomaticScaling: `F1`, `F2`, `F4`, `F4_1G`
    /// * ManualScaling or BasicScaling: `B1`, `B2`, `B4`, `B8`, `B4_1G`
    ///
    /// Defaults to `F1` for AutomaticScaling and `B1` for ManualScaling or
    /// BasicScaling.
    #[prost(string, tag = "7")]
    pub instance_class: std::string::String,
    /// Extra network settings. Only applicable for VM runtimes.
    #[prost(message, optional, tag = "8")]
    pub network: ::std::option::Option<Network>,
    /// Machine resources for this version. Only applicable for VM runtimes.
    #[prost(message, optional, tag = "9")]
    pub resources: ::std::option::Option<Resources>,
    /// Desired runtime. Example: `python27`.
    #[prost(string, tag = "10")]
    pub runtime: std::string::String,
    /// Whether multiple requests can be dispatched to this version at once.
    #[prost(bool, tag = "11")]
    pub threadsafe: bool,
    /// Whether to deploy this version in a container on a virtual machine.
    #[prost(bool, tag = "12")]
    pub vm: bool,
    /// Metadata settings that are supplied to this version to enable
    /// beta runtime features.
    #[prost(map = "string, string", tag = "13")]
    pub beta_settings: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// App Engine execution environment for this version.
    ///
    /// Defaults to `standard`.
    #[prost(string, tag = "14")]
    pub env: std::string::String,
    /// Current serving status of this version. Only the versions with a
    /// `SERVING` status create instances and can be billed.
    ///
    /// `SERVING_STATUS_UNSPECIFIED` is an invalid value. Defaults to `SERVING`.
    #[prost(enumeration = "ServingStatus", tag = "15")]
    pub serving_status: i32,
    /// Email address of the user who created this version.
    ///
    /// @OutputOnly
    #[prost(string, tag = "16")]
    pub created_by: std::string::String,
    /// Time that this version was created.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "17")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Total size in bytes of all the files that are included in this version
    /// and curerntly hosted on the App Engine disk.
    ///
    /// @OutputOnly
    #[prost(int64, tag = "18")]
    pub disk_usage_bytes: i64,
    /// An ordered list of URL-matching patterns that should be applied to incoming
    /// requests. The first matching URL handles the request and other request
    /// handlers are not attempted.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, repeated, tag = "100")]
    pub handlers: ::std::vec::Vec<UrlMap>,
    /// Custom static error pages. Limited to 10KB per page.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, repeated, tag = "101")]
    pub error_handlers: ::std::vec::Vec<ErrorHandler>,
    /// Configuration for third-party Python runtime libraries that are required
    /// by the application.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, repeated, tag = "102")]
    pub libraries: ::std::vec::Vec<Library>,
    /// Serving configuration for
    /// [Google Cloud
    /// Endpoints](https://cloud.google.com/appengine/docs/python/endpoints/).
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "103")]
    pub api_config: ::std::option::Option<ApiConfigHandler>,
    /// Environment variables available to the application.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(map = "string, string", tag = "104")]
    pub env_variables: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Duration that static files should be cached by web proxies and browsers.
    /// Only applicable if the corresponding
    /// [StaticFilesHandler](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#staticfileshandler)
    /// does not specify its own expiration time.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "105")]
    pub default_expiration: ::std::option::Option<::prost_types::Duration>,
    /// Configures health checking for VM instances. Unhealthy instances are
    /// stopped and replaced with new instances. Only applicable for VM
    /// runtimes.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "106")]
    pub health_check: ::std::option::Option<HealthCheck>,
    /// Files that match this pattern will not be built into this version.
    /// Only applicable for Go runtimes.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(string, tag = "107")]
    pub nobuild_files_regex: std::string::String,
    /// Code and application artifacts that make up this version.
    ///
    /// Only returned in `GET` requests if `view=FULL` is set.
    #[prost(message, optional, tag = "108")]
    pub deployment: ::std::option::Option<Deployment>,
    /// Serving URL for this version. Example:
    /// "https://myversion-dot-myservice-dot-myapp.appspot.com"
    ///
    /// @OutputOnly
    #[prost(string, tag = "109")]
    pub version_url: std::string::String,
    /// Controls how instances are created.
    ///
    /// Defaults to `AutomaticScaling`.
    #[prost(oneof = "version::Scaling", tags = "3, 4, 5")]
    pub scaling: ::std::option::Option<version::Scaling>,
}
pub mod version {
    /// Controls how instances are created.
    ///
    /// Defaults to `AutomaticScaling`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scaling {
        /// Automatic scaling is based on request rate, response latencies, and other
        /// application metrics.
        #[prost(message, tag = "3")]
        AutomaticScaling(super::AutomaticScaling),
        /// A service with basic scaling will create an instance when the application
        /// receives a request. The instance will be turned down when the app becomes
        /// idle. Basic scaling is ideal for work that is intermittent or driven by
        /// user activity.
        #[prost(message, tag = "4")]
        BasicScaling(super::BasicScaling),
        /// A service with manual scaling runs continuously, allowing you to perform
        /// complex initialization and rely on the state of its memory over time.
        #[prost(message, tag = "5")]
        ManualScaling(super::ManualScaling),
    }
}
/// Automatic scaling is based on request rate, response latencies, and other
/// application metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomaticScaling {
    /// Amount of time that the
    /// [Autoscaler](https://cloud.google.com/compute/docs/autoscaler/)
    /// should wait between changes to the number of virtual machines.
    /// Only applicable for VM runtimes.
    #[prost(message, optional, tag = "1")]
    pub cool_down_period: ::std::option::Option<::prost_types::Duration>,
    /// Target scaling by CPU usage.
    #[prost(message, optional, tag = "2")]
    pub cpu_utilization: ::std::option::Option<CpuUtilization>,
    /// Number of concurrent requests an automatic scaling instance can accept
    /// before the scheduler spawns a new instance.
    ///
    /// Defaults to a runtime-specific value.
    #[prost(int32, tag = "3")]
    pub max_concurrent_requests: i32,
    /// Maximum number of idle instances that should be maintained for this
    /// version.
    #[prost(int32, tag = "4")]
    pub max_idle_instances: i32,
    /// Maximum number of instances that should be started to handle requests.
    #[prost(int32, tag = "5")]
    pub max_total_instances: i32,
    /// Maximum amount of time that a request should wait in the pending queue
    /// before starting a new instance to handle it.
    #[prost(message, optional, tag = "6")]
    pub max_pending_latency: ::std::option::Option<::prost_types::Duration>,
    /// Minimum number of idle instances that should be maintained for
    /// this version. Only applicable for the default version of a service.
    #[prost(int32, tag = "7")]
    pub min_idle_instances: i32,
    /// Minimum number of instances that should be maintained for this version.
    #[prost(int32, tag = "8")]
    pub min_total_instances: i32,
    /// Minimum amount of time a request should wait in the pending queue before
    /// starting a new instance to handle it.
    #[prost(message, optional, tag = "9")]
    pub min_pending_latency: ::std::option::Option<::prost_types::Duration>,
    /// Target scaling by request utilization.
    #[prost(message, optional, tag = "10")]
    pub request_utilization: ::std::option::Option<RequestUtilization>,
    /// Target scaling by disk usage.
    #[prost(message, optional, tag = "11")]
    pub disk_utilization: ::std::option::Option<DiskUtilization>,
    /// Target scaling by network usage.
    #[prost(message, optional, tag = "12")]
    pub network_utilization: ::std::option::Option<NetworkUtilization>,
}
/// A service with basic scaling will create an instance when the application
/// receives a request. The instance will be turned down when the app becomes
/// idle. Basic scaling is ideal for work that is intermittent or driven by
/// user activity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicScaling {
    /// Duration of time after the last request that an instance must wait before
    /// the instance is shut down.
    #[prost(message, optional, tag = "1")]
    pub idle_timeout: ::std::option::Option<::prost_types::Duration>,
    /// Maximum number of instances to create for this version.
    #[prost(int32, tag = "2")]
    pub max_instances: i32,
}
/// A service with manual scaling runs continuously, allowing you to perform
/// complex initialization and rely on the state of its memory over time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualScaling {
    /// Number of instances to assign to the service at the start. This number
    /// can later be altered by using the
    /// [Modules
    /// API](https://cloud.google.com/appengine/docs/python/modules/functions)
    /// `set_num_instances()` function.
    #[prost(int32, tag = "1")]
    pub instances: i32,
}
/// Target scaling by CPU usage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpuUtilization {
    /// Period of time over which CPU utilization is calculated.
    #[prost(message, optional, tag = "1")]
    pub aggregation_window_length: ::std::option::Option<::prost_types::Duration>,
    /// Target CPU utilization ratio to maintain when scaling. Must be between 0
    /// and 1.
    #[prost(double, tag = "2")]
    pub target_utilization: f64,
}
/// Target scaling by request utilization. Only applicable for VM runtimes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestUtilization {
    /// Target requests per second.
    #[prost(int32, tag = "1")]
    pub target_request_count_per_second: i32,
    /// Target number of concurrent requests.
    #[prost(int32, tag = "2")]
    pub target_concurrent_requests: i32,
}
/// Target scaling by disk usage. Only applicable for VM runtimes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskUtilization {
    /// Target bytes written per second.
    #[prost(int32, tag = "14")]
    pub target_write_bytes_per_second: i32,
    /// Target ops written per second.
    #[prost(int32, tag = "15")]
    pub target_write_ops_per_second: i32,
    /// Target bytes read per second.
    #[prost(int32, tag = "16")]
    pub target_read_bytes_per_second: i32,
    /// Target ops read per seconds.
    #[prost(int32, tag = "17")]
    pub target_read_ops_per_second: i32,
}
/// Target scaling by network usage. Only applicable for VM runtimes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkUtilization {
    /// Target bytes sent per second.
    #[prost(int32, tag = "1")]
    pub target_sent_bytes_per_second: i32,
    /// Target packets sent per second.
    #[prost(int32, tag = "11")]
    pub target_sent_packets_per_second: i32,
    /// Target bytes received per second.
    #[prost(int32, tag = "12")]
    pub target_received_bytes_per_second: i32,
    /// Target packets received per second.
    #[prost(int32, tag = "13")]
    pub target_received_packets_per_second: i32,
}
/// Extra network settings. Only applicable for VM runtimes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Network {
    /// List of ports, or port pairs, to forward from the virtual machine to the
    /// application container.
    #[prost(string, repeated, tag = "1")]
    pub forwarded_ports: ::std::vec::Vec<std::string::String>,
    /// Tag to apply to the VM instance during creation.
    #[prost(string, tag = "2")]
    pub instance_tag: std::string::String,
    /// Google Cloud Platform network where the virtual machines are created.
    /// Specify the short name, not the resource path.
    ///
    /// Defaults to `default`.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
/// Machine resources for a version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resources {
    /// Number of CPU cores needed.
    #[prost(double, tag = "1")]
    pub cpu: f64,
    /// Disk size (GB) needed.
    #[prost(double, tag = "2")]
    pub disk_gb: f64,
    /// Memory (GB) needed.
    #[prost(double, tag = "3")]
    pub memory_gb: f64,
}
/// Available inbound services.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InboundServiceType {
    /// Not specified.
    InboundServiceUnspecified = 0,
    /// Allows an application to receive mail.
    InboundServiceMail = 1,
    /// Allows an application to receive email-bound notifications.
    InboundServiceMailBounce = 2,
    /// Allows an application to receive error stanzas.
    InboundServiceXmppError = 3,
    /// Allows an application to receive instant messages.
    InboundServiceXmppMessage = 4,
    /// Allows an application to receive user subscription POSTs.
    InboundServiceXmppSubscribe = 5,
    /// Allows an application to receive a user's chat presence.
    InboundServiceXmppPresence = 6,
    /// Registers an application for notifications when a client connects or
    /// disconnects from a channel.
    InboundServiceChannelPresence = 7,
    /// Enables warmup requests.
    InboundServiceWarmup = 9,
}
/// Run states of a version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServingStatus {
    /// Not specified.
    Unspecified = 0,
    /// Currently serving. Instances are created according to the
    /// scaling settings of the version.
    Serving = 1,
    /// Disabled. No instances will be created and the scaling
    /// settings are ignored until the state of the version changes
    /// to `SERVING`.
    Stopped = 2,
}
/// Request message for `Applications.GetApplication`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApplicationRequest {
    /// Name of the Application resource to get. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for 'Applications.RepairApplication'.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepairApplicationRequest {
    /// Name of the application to repair. Example: `apps/myapp`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Services.ListServices`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Name of the parent Application resource. Example: `apps/myapp`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for `Services.ListServices`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The services belonging to the requested application.
    #[prost(message, repeated, tag = "1")]
    pub services: ::std::vec::Vec<Service>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `Services.GetService`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Name of the resource requested. Example: `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Services.UpdateService`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Name of the resource to update. Example: `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A Service resource containing the updated service. Only fields set in the
    /// field mask will be updated.
    #[prost(message, optional, tag = "2")]
    pub service: ::std::option::Option<Service>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Set to `true` to gradually shift traffic from one version to another
    /// single version. By default, traffic is shifted immediately.
    /// For gradual traffic migration, the target version
    /// must be located within instances that are configured for both
    /// [warmup
    /// requests](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#inboundservicetype)
    /// and
    /// [automatic
    /// scaling](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#automaticscaling).
    /// You must specify the
    /// [`shardBy`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services#shardby)
    /// field in the Service resource. Gradual traffic migration is not
    /// supported in the App Engine flexible environment. For examples, see
    /// [Migrating and Splitting
    /// Traffic](https://cloud.google.com/appengine/docs/admin-api/migrating-splitting-traffic).
    #[prost(bool, tag = "4")]
    pub migrate_traffic: bool,
}
/// Request message for `Services.DeleteService`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Name of the resource requested. Example: `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Versions.ListVersions`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsRequest {
    /// Name of the parent Service resource. Example:
    /// `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Controls the set of fields returned in the `List` response.
    #[prost(enumeration = "VersionView", tag = "2")]
    pub view: i32,
    /// Maximum results to return per page.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// Response message for `Versions.ListVersions`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsResponse {
    /// The versions belonging to the requested service.
    #[prost(message, repeated, tag = "1")]
    pub versions: ::std::vec::Vec<Version>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `Versions.GetVersion`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Controls the set of fields returned in the `Get` response.
    #[prost(enumeration = "VersionView", tag = "2")]
    pub view: i32,
}
/// Request message for `Versions.CreateVersion`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionRequest {
    /// Name of the parent resource to create this version under. Example:
    /// `apps/myapp/services/default`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Application deployment configuration.
    #[prost(message, optional, tag = "2")]
    pub version: ::std::option::Option<Version>,
}
/// Request message for `Versions.UpdateVersion`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVersionRequest {
    /// Name of the resource to update. Example:
    /// `apps/myapp/services/default/versions/1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A Version containing the updated resource. Only fields set in the field
    /// mask will be updated.
    #[prost(message, optional, tag = "2")]
    pub version: ::std::option::Option<Version>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for `Versions.DeleteVersion`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVersionRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Instances.ListInstances`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Name of the parent Version resource. Example:
    /// `apps/myapp/services/default/versions/v1`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Maximum results to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for `Instances.ListInstances`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The instances belonging to the requested version.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::std::vec::Vec<Instance>,
    /// Continuation token for fetching the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `Instances.GetInstance`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1/instances/instance-1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Instances.DeleteInstance`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1/instances/instance-1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `Instances.DebugInstance`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugInstanceRequest {
    /// Name of the resource requested. Example:
    /// `apps/myapp/services/default/versions/v1/instances/instance-1`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Fields that should be returned when [Version][google.appengine.v1.Version]
/// resources are retreived.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VersionView {
    /// Basic version information including scaling and inbound services,
    /// but not detailed deployment information.
    Basic = 0,
    /// The information from `BASIC`, plus detailed information about the
    /// deployment. This format is required when creating resources, but
    /// is not returned in `Get` or `List` by default.
    Full = 1,
}
#[doc = r" Generated client implementations."]
pub mod instances_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages instances of a version."]
    pub struct InstancesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> InstancesClient<T>
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
        #[doc = " Lists the instances of a version."]
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1.Instances/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets instance information."]
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.appengine.v1.Instances/GetInstance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops a running instance."]
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1.Instances/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Enables debugging on a VM instance. This allows you to use the SSH"]
        #[doc = " command to connect to the virtual machine where the instance lives."]
        #[doc = " While in \"debug mode\", the instance continues to serve live traffic."]
        #[doc = " You should delete the instance when you are done debugging and then"]
        #[doc = " allow the system to take over and determine if another instance"]
        #[doc = " should be started."]
        #[doc = ""]
        #[doc = " Only applicable for instances in App Engine flexible environment."]
        pub async fn debug_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DebugInstanceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1.Instances/DebugInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for InstancesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for InstancesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "InstancesClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod versions_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages versions of a service."]
    pub struct VersionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VersionsClient<T>
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
        #[doc = " Lists the versions of a service."]
        pub async fn list_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVersionsRequest>,
        ) -> Result<tonic::Response<super::ListVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.appengine.v1.Versions/ListVersions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the specified Version resource."]
        #[doc = " By default, only a `BASIC_VIEW` will be returned."]
        #[doc = " Specify the `FULL_VIEW` parameter to get the full resource."]
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.appengine.v1.Versions/GetVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deploys code and resource files to a new version."]
        pub async fn create_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.appengine.v1.Versions/CreateVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified Version resource."]
        #[doc = " You can specify the following fields depending on the App Engine"]
        #[doc = " environment and type of scaling that the version resource uses:"]
        #[doc = ""]
        #[doc = " * [`serving_status`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status):"]
        #[doc = "   For Version resources that use basic scaling, manual scaling, or run in"]
        #[doc = "   the App Engine flexible environment."]
        #[doc = " * [`instance_class`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.instance_class):"]
        #[doc = "   For Version resources that run in the App Engine standard environment."]
        #[doc = " * [`automatic_scaling.min_idle_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling):"]
        #[doc = "   For Version resources that use automatic scaling and run in the App"]
        #[doc = "   Engine standard environment."]
        #[doc = " * [`automatic_scaling.max_idle_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling):"]
        #[doc = "   For Version resources that use automatic scaling and run in the App"]
        #[doc = "   Engine standard environment."]
        pub async fn update_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.appengine.v1.Versions/UpdateVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing Version resource."]
        pub async fn delete_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.appengine.v1.Versions/DeleteVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VersionsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VersionsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VersionsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod services_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages services of an application."]
    pub struct ServicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ServicesClient<T>
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
        #[doc = " Lists all the services in the application."]
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.appengine.v1.Services/ListServices");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the current configuration of the specified service."]
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.appengine.v1.Services/GetService");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the configuration of the specified service."]
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.appengine.v1.Services/UpdateService");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified service and all enclosed versions."]
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.appengine.v1.Services/DeleteService");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ServicesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ServicesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ServicesClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod applications_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages App Engine applications."]
    pub struct ApplicationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ApplicationsClient<T>
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
        #[doc = " Gets information about an application."]
        pub async fn get_application(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1.Applications/GetApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Recreates the required App Engine features for the application in your"]
        #[doc = " project, for example a Cloud Storage bucket or App Engine service account."]
        #[doc = " Use this method if you receive an error message about a missing feature,"]
        #[doc = " for example \"*Error retrieving the App Engine service account*\"."]
        pub async fn repair_application(
            &mut self,
            request: impl tonic::IntoRequest<super::RepairApplicationRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.appengine.v1.Applications/RepairApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ApplicationsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ApplicationsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ApplicationsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod instances_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with InstancesServer."]
    #[async_trait]
    pub trait Instances: Send + Sync + 'static {
        #[doc = " Lists the instances of a version."]
        async fn list_instances(
            &self,
            request: tonic::Request<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status>;
        #[doc = " Gets instance information."]
        async fn get_instance(
            &self,
            request: tonic::Request<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status>;
        #[doc = " Stops a running instance."]
        async fn delete_instance(
            &self,
            request: tonic::Request<super::DeleteInstanceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Enables debugging on a VM instance. This allows you to use the SSH"]
        #[doc = " command to connect to the virtual machine where the instance lives."]
        #[doc = " While in \"debug mode\", the instance continues to serve live traffic."]
        #[doc = " You should delete the instance when you are done debugging and then"]
        #[doc = " allow the system to take over and determine if another instance"]
        #[doc = " should be started."]
        #[doc = ""]
        #[doc = " Only applicable for instances in App Engine flexible environment."]
        async fn debug_instance(
            &self,
            request: tonic::Request<super::DebugInstanceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
    }
    #[doc = " Manages instances of a version."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct InstancesServer<T: Instances> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Instances> InstancesServer<T> {
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
    impl<T, B> Service<http::Request<B>> for InstancesServer<T>
    where
        T: Instances,
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
                "/google.appengine.v1.Instances/ListInstances" => {
                    #[allow(non_camel_case_types)]
                    struct ListInstancesSvc<T: Instances>(pub Arc<T>);
                    impl<T: Instances> tonic::server::UnaryService<super::ListInstancesRequest>
                        for ListInstancesSvc<T>
                    {
                        type Response = super::ListInstancesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInstancesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_instances(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListInstancesSvc(inner);
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
                "/google.appengine.v1.Instances/GetInstance" => {
                    #[allow(non_camel_case_types)]
                    struct GetInstanceSvc<T: Instances>(pub Arc<T>);
                    impl<T: Instances> tonic::server::UnaryService<super::GetInstanceRequest> for GetInstanceSvc<T> {
                        type Response = super::Instance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetInstanceSvc(inner);
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
                "/google.appengine.v1.Instances/DeleteInstance" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteInstanceSvc<T: Instances>(pub Arc<T>);
                    impl<T: Instances> tonic::server::UnaryService<super::DeleteInstanceRequest>
                        for DeleteInstanceSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteInstanceSvc(inner);
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
                "/google.appengine.v1.Instances/DebugInstance" => {
                    #[allow(non_camel_case_types)]
                    struct DebugInstanceSvc<T: Instances>(pub Arc<T>);
                    impl<T: Instances> tonic::server::UnaryService<super::DebugInstanceRequest>
                        for DebugInstanceSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DebugInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.debug_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DebugInstanceSvc(inner);
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
    impl<T: Instances> Clone for InstancesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Instances> Clone for _Inner<T> {
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
#[doc = r" Generated server implementations."]
pub mod versions_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with VersionsServer."]
    #[async_trait]
    pub trait Versions: Send + Sync + 'static {
        #[doc = " Lists the versions of a service."]
        async fn list_versions(
            &self,
            request: tonic::Request<super::ListVersionsRequest>,
        ) -> Result<tonic::Response<super::ListVersionsResponse>, tonic::Status>;
        #[doc = " Gets the specified Version resource."]
        #[doc = " By default, only a `BASIC_VIEW` will be returned."]
        #[doc = " Specify the `FULL_VIEW` parameter to get the full resource."]
        async fn get_version(
            &self,
            request: tonic::Request<super::GetVersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status>;
        #[doc = " Deploys code and resource files to a new version."]
        async fn create_version(
            &self,
            request: tonic::Request<super::CreateVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Updates the specified Version resource."]
        #[doc = " You can specify the following fields depending on the App Engine"]
        #[doc = " environment and type of scaling that the version resource uses:"]
        #[doc = ""]
        #[doc = " * [`serving_status`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.serving_status):"]
        #[doc = "   For Version resources that use basic scaling, manual scaling, or run in"]
        #[doc = "   the App Engine flexible environment."]
        #[doc = " * [`instance_class`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.instance_class):"]
        #[doc = "   For Version resources that run in the App Engine standard environment."]
        #[doc = " * [`automatic_scaling.min_idle_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling):"]
        #[doc = "   For Version resources that use automatic scaling and run in the App"]
        #[doc = "   Engine standard environment."]
        #[doc = " * [`automatic_scaling.max_idle_instances`](https://cloud.google.com/appengine/docs/admin-api/reference/rest/v1/apps.services.versions#Version.FIELDS.automatic_scaling):"]
        #[doc = "   For Version resources that use automatic scaling and run in the App"]
        #[doc = "   Engine standard environment."]
        async fn update_version(
            &self,
            request: tonic::Request<super::UpdateVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Deletes an existing Version resource."]
        async fn delete_version(
            &self,
            request: tonic::Request<super::DeleteVersionRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
    }
    #[doc = " Manages versions of a service."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct VersionsServer<T: Versions> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Versions> VersionsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for VersionsServer<T>
    where
        T: Versions,
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
                "/google.appengine.v1.Versions/ListVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListVersionsSvc<T: Versions>(pub Arc<T>);
                    impl<T: Versions> tonic::server::UnaryService<super::ListVersionsRequest> for ListVersionsSvc<T> {
                        type Response = super::ListVersionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListVersionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_versions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListVersionsSvc(inner);
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
                "/google.appengine.v1.Versions/GetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetVersionSvc<T: Versions>(pub Arc<T>);
                    impl<T: Versions> tonic::server::UnaryService<super::GetVersionRequest> for GetVersionSvc<T> {
                        type Response = super::Version;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetVersionSvc(inner);
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
                "/google.appengine.v1.Versions/CreateVersion" => {
                    #[allow(non_camel_case_types)]
                    struct CreateVersionSvc<T: Versions>(pub Arc<T>);
                    impl<T: Versions> tonic::server::UnaryService<super::CreateVersionRequest> for CreateVersionSvc<T> {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateVersionSvc(inner);
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
                "/google.appengine.v1.Versions/UpdateVersion" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateVersionSvc<T: Versions>(pub Arc<T>);
                    impl<T: Versions> tonic::server::UnaryService<super::UpdateVersionRequest> for UpdateVersionSvc<T> {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateVersionSvc(inner);
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
                "/google.appengine.v1.Versions/DeleteVersion" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteVersionSvc<T: Versions>(pub Arc<T>);
                    impl<T: Versions> tonic::server::UnaryService<super::DeleteVersionRequest> for DeleteVersionSvc<T> {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteVersionSvc(inner);
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
    impl<T: Versions> Clone for VersionsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Versions> Clone for _Inner<T> {
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
#[doc = r" Generated server implementations."]
pub mod services_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ServicesServer."]
    #[async_trait]
    pub trait Services: Send + Sync + 'static {
        #[doc = " Lists all the services in the application."]
        async fn list_services(
            &self,
            request: tonic::Request<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status>;
        #[doc = " Gets the current configuration of the specified service."]
        async fn get_service(
            &self,
            request: tonic::Request<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status>;
        #[doc = " Updates the configuration of the specified service."]
        async fn update_service(
            &self,
            request: tonic::Request<super::UpdateServiceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
        #[doc = " Deletes the specified service and all enclosed versions."]
        async fn delete_service(
            &self,
            request: tonic::Request<super::DeleteServiceRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
    }
    #[doc = " Manages services of an application."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ServicesServer<T: Services> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Services> ServicesServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ServicesServer<T>
    where
        T: Services,
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
                "/google.appengine.v1.Services/ListServices" => {
                    #[allow(non_camel_case_types)]
                    struct ListServicesSvc<T: Services>(pub Arc<T>);
                    impl<T: Services> tonic::server::UnaryService<super::ListServicesRequest> for ListServicesSvc<T> {
                        type Response = super::ListServicesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListServicesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_services(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListServicesSvc(inner);
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
                "/google.appengine.v1.Services/GetService" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceSvc<T: Services>(pub Arc<T>);
                    impl<T: Services> tonic::server::UnaryService<super::GetServiceRequest> for GetServiceSvc<T> {
                        type Response = super::Service;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetServiceSvc(inner);
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
                "/google.appengine.v1.Services/UpdateService" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateServiceSvc<T: Services>(pub Arc<T>);
                    impl<T: Services> tonic::server::UnaryService<super::UpdateServiceRequest> for UpdateServiceSvc<T> {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateServiceSvc(inner);
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
                "/google.appengine.v1.Services/DeleteService" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteServiceSvc<T: Services>(pub Arc<T>);
                    impl<T: Services> tonic::server::UnaryService<super::DeleteServiceRequest> for DeleteServiceSvc<T> {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteServiceSvc(inner);
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
    impl<T: Services> Clone for ServicesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Services> Clone for _Inner<T> {
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
#[doc = r" Generated server implementations."]
pub mod applications_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ApplicationsServer."]
    #[async_trait]
    pub trait Applications: Send + Sync + 'static {
        #[doc = " Gets information about an application."]
        async fn get_application(
            &self,
            request: tonic::Request<super::GetApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status>;
        #[doc = " Recreates the required App Engine features for the application in your"]
        #[doc = " project, for example a Cloud Storage bucket or App Engine service account."]
        #[doc = " Use this method if you receive an error message about a missing feature,"]
        #[doc = " for example \"*Error retrieving the App Engine service account*\"."]
        async fn repair_application(
            &self,
            request: tonic::Request<super::RepairApplicationRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>;
    }
    #[doc = " Manages App Engine applications."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ApplicationsServer<T: Applications> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Applications> ApplicationsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ApplicationsServer<T>
    where
        T: Applications,
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
                "/google.appengine.v1.Applications/GetApplication" => {
                    #[allow(non_camel_case_types)]
                    struct GetApplicationSvc<T: Applications>(pub Arc<T>);
                    impl<T: Applications> tonic::server::UnaryService<super::GetApplicationRequest>
                        for GetApplicationSvc<T>
                    {
                        type Response = super::Application;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetApplicationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_application(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetApplicationSvc(inner);
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
                "/google.appengine.v1.Applications/RepairApplication" => {
                    #[allow(non_camel_case_types)]
                    struct RepairApplicationSvc<T: Applications>(pub Arc<T>);
                    impl<T: Applications>
                        tonic::server::UnaryService<super::RepairApplicationRequest>
                        for RepairApplicationSvc<T>
                    {
                        type Response = super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RepairApplicationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.repair_application(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RepairApplicationSvc(inner);
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
    impl<T: Applications> Clone for ApplicationsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Applications> Clone for _Inner<T> {
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
/// App Engine admin service audit log.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditData {
    /// Detailed information about methods that require it. Does not include
    /// simple Get, List or Delete methods because all significant information
    /// (resource name, number of returned elements for List operations) is already
    /// included in parent audit log message.
    #[prost(oneof = "audit_data::Method", tags = "1, 2")]
    pub method: ::std::option::Option<audit_data::Method>,
}
pub mod audit_data {
    /// Detailed information about methods that require it. Does not include
    /// simple Get, List or Delete methods because all significant information
    /// (resource name, number of returned elements for List operations) is already
    /// included in parent audit log message.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        /// Detailed information about UpdateService call.
        #[prost(message, tag = "1")]
        UpdateService(super::UpdateServiceMethod),
        /// Detailed information about CreateVersion call.
        #[prost(message, tag = "2")]
        CreateVersion(super::CreateVersionMethod),
    }
}
/// Detailed information about UpdateService call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceMethod {
    /// Update service request.
    #[prost(message, optional, tag = "1")]
    pub request: ::std::option::Option<UpdateServiceRequest>,
}
/// Detailed information about CreateVersion call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionMethod {
    /// Create version request.
    #[prost(message, optional, tag = "1")]
    pub request: ::std::option::Option<CreateVersionRequest>,
}
/// Metadata for the given
/// [google.cloud.location.Location][google.cloud.location.Location].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// App Engine Standard Environment is available in the given location.
    ///
    /// @OutputOnly
    #[prost(bool, tag = "2")]
    pub standard_environment_available: bool,
    /// App Engine Flexible Environment is available in the given location.
    ///
    /// @OutputOnly
    #[prost(bool, tag = "4")]
    pub flexible_environment_available: bool,
}
/// Metadata for the given
/// [google.longrunning.Operation][google.longrunning.Operation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadataV1 {
    /// API method that initiated this operation. Example:
    /// `google.appengine.v1.Versions.CreateVersion`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "1")]
    pub method: std::string::String,
    /// Time that this operation was created.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "2")]
    pub insert_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Time that this operation completed.
    ///
    /// @OutputOnly
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// User who requested this operation.
    ///
    /// @OutputOnly
    #[prost(string, tag = "4")]
    pub user: std::string::String,
    /// Name of the resource that this operation is acting on. Example:
    /// `apps/myapp/services/default`.
    ///
    /// @OutputOnly
    #[prost(string, tag = "5")]
    pub target: std::string::String,
}
