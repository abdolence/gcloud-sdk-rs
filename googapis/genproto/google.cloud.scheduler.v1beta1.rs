/// Http target. The job will be pushed to the job handler by means of
/// an HTTP request via an [http_method][google.cloud.scheduler.v1beta1.HttpTarget.http_method] such as HTTP
/// POST, HTTP GET, etc. The job is acknowledged by means of an HTTP
/// response code in the range [200 - 299]. A failure to receive a response
/// constitutes a failed execution. For a redirected request, the response
/// returned by the redirected request is considered.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpTarget {
    /// Required. The full URI path that the request will be sent to. This string
    /// must begin with either "http://" or "https://". Some examples of
    /// valid values for [uri][google.cloud.scheduler.v1beta1.HttpTarget.uri] are:
    /// `http://acme.com` and `https://acme.com/sales:8080`. Cloud Scheduler will
    /// encode some characters for safety and compatibility. The maximum allowed
    /// URL length is 2083 characters after encoding.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
    /// Which HTTP method to use for the request.
    #[prost(enumeration = "HttpMethod", tag = "2")]
    pub http_method: i32,
    /// The user can specify HTTP request headers to send with the job's
    /// HTTP request. This map contains the header field names and
    /// values. Repeated headers are not supported, but a header value can
    /// contain commas. These headers represent a subset of the headers
    /// that will accompany the job's HTTP request. Some HTTP request
    /// headers will be ignored or replaced. A partial list of headers that
    /// will be ignored or replaced is below:
    /// - Host: This will be computed by Cloud Scheduler and derived from
    /// [uri][google.cloud.scheduler.v1beta1.HttpTarget.uri].
    /// * `Content-Length`: This will be computed by Cloud Scheduler.
    /// * `User-Agent`: This will be set to `"Google-Cloud-Scheduler"`.
    /// * `X-Google-*`: Google internal use only.
    /// * `X-AppEngine-*`: Google internal use only.
    ///
    /// The total size of headers must be less than 80KB.
    #[prost(map = "string, string", tag = "3")]
    pub headers: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// HTTP request body. A request body is allowed only if the HTTP
    /// method is POST, PUT, or PATCH. It is an error to set body on a job with an
    /// incompatible [HttpMethod][google.cloud.scheduler.v1beta1.HttpMethod].
    #[prost(bytes, tag = "4")]
    pub body: std::vec::Vec<u8>,
    /// The mode for generating an `Authorization` header for HTTP requests.
    ///
    /// If specified, all `Authorization` headers in the [HttpTarget.headers][google.cloud.scheduler.v1beta1.HttpTarget.headers]
    /// field will be overridden.
    #[prost(oneof = "http_target::AuthorizationHeader", tags = "5, 6")]
    pub authorization_header: ::std::option::Option<http_target::AuthorizationHeader>,
}
pub mod http_target {
    /// The mode for generating an `Authorization` header for HTTP requests.
    ///
    /// If specified, all `Authorization` headers in the [HttpTarget.headers][google.cloud.scheduler.v1beta1.HttpTarget.headers]
    /// field will be overridden.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuthorizationHeader {
        /// If specified, an
        /// [OAuth token](https://developers.google.com/identity/protocols/OAuth2)
        /// will be generated and attached as an `Authorization` header in the HTTP
        /// request.
        ///
        /// This type of authorization should generally only be used when calling
        /// Google APIs hosted on *.googleapis.com.
        #[prost(message, tag = "5")]
        OauthToken(super::OAuthToken),
        /// If specified, an
        /// [OIDC](https://developers.google.com/identity/protocols/OpenIDConnect)
        /// token will be generated and attached as an `Authorization` header in the
        /// HTTP request.
        ///
        /// This type of authorization can be used for many scenarios, including
        /// calling Cloud Run, or endpoints where you intend to validate the token
        /// yourself.
        #[prost(message, tag = "6")]
        OidcToken(super::OidcToken),
    }
}
/// App Engine target. The job will be pushed to a job handler by means
/// of an HTTP request via an [http_method][google.cloud.scheduler.v1beta1.AppEngineHttpTarget.http_method] such
/// as HTTP POST, HTTP GET, etc. The job is acknowledged by means of an
/// HTTP response code in the range [200 - 299]. Error 503 is
/// considered an App Engine system error instead of an application
/// error. Requests returning error 503 will be retried regardless of
/// retry configuration and not counted against retry counts. Any other
/// response code, or a failure to receive a response before the
/// deadline, constitutes a failed attempt.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppEngineHttpTarget {
    /// The HTTP method to use for the request. PATCH and OPTIONS are not
    /// permitted.
    #[prost(enumeration = "HttpMethod", tag = "1")]
    pub http_method: i32,
    /// App Engine Routing setting for the job.
    #[prost(message, optional, tag = "2")]
    pub app_engine_routing: ::std::option::Option<AppEngineRouting>,
    /// The relative URI.
    ///
    /// The relative URL must begin with "/" and must be a valid HTTP relative URL.
    /// It can contain a path, query string arguments, and `#` fragments.
    /// If the relative URL is empty, then the root path "/" will be used.
    /// No spaces are allowed, and the maximum length allowed is 2083 characters.
    #[prost(string, tag = "3")]
    pub relative_uri: std::string::String,
    /// HTTP request headers.
    ///
    /// This map contains the header field names and values. Headers can be set
    /// when the job is created.
    ///
    /// Cloud Scheduler sets some headers to default values:
    ///
    /// * `User-Agent`: By default, this header is
    ///   `"AppEngine-Google; (+http://code.google.com/appengine)"`.
    ///   This header can be modified, but Cloud Scheduler will append
    ///   `"AppEngine-Google; (+http://code.google.com/appengine)"` to the
    ///   modified `User-Agent`.
    /// * `X-CloudScheduler`: This header will be set to true.
    ///
    /// If the job has an [body][google.cloud.scheduler.v1beta1.AppEngineHttpTarget.body], Cloud Scheduler sets
    /// the following headers:
    ///
    /// * `Content-Type`: By default, the `Content-Type` header is set to
    ///   `"application/octet-stream"`. The default can be overridden by explictly
    ///   setting `Content-Type` to a particular media type when the job is
    ///   created.
    ///   For example, `Content-Type` can be set to `"application/json"`.
    /// * `Content-Length`: This is computed by Cloud Scheduler. This value is
    ///   output only. It cannot be changed.
    ///
    /// The headers below are output only. They cannot be set or overridden:
    ///
    /// * `X-Google-*`: For Google internal use only.
    /// * `X-AppEngine-*`: For Google internal use only.
    ///
    /// In addition, some App Engine headers, which contain
    /// job-specific information, are also be sent to the job handler.
    #[prost(map = "string, string", tag = "4")]
    pub headers: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Body.
    ///
    /// HTTP request body. A request body is allowed only if the HTTP method is
    /// POST or PUT. It will result in invalid argument error to set a body on a
    /// job with an incompatible [HttpMethod][google.cloud.scheduler.v1beta1.HttpMethod].
    #[prost(bytes, tag = "5")]
    pub body: std::vec::Vec<u8>,
}
/// Pub/Sub target. The job will be delivered by publishing a message to
/// the given Pub/Sub topic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubTarget {
    /// Required. The name of the Cloud Pub/Sub topic to which messages will
    /// be published when a job is delivered. The topic name must be in the
    /// same format as required by PubSub's
    /// [PublishRequest.name](https://cloud.google.com/pubsub/docs/reference/rpc/google.pubsub.v1#publishrequest),
    /// for example `projects/PROJECT_ID/topics/TOPIC_ID`.
    ///
    /// The topic must be in the same project as the Cloud Scheduler job.
    #[prost(string, tag = "1")]
    pub topic_name: std::string::String,
    /// The message payload for PubsubMessage.
    ///
    /// Pubsub message must contain either non-empty data, or at least one
    /// attribute.
    #[prost(bytes, tag = "3")]
    pub data: std::vec::Vec<u8>,
    /// Attributes for PubsubMessage.
    ///
    /// Pubsub message must contain either non-empty data, or at least one
    /// attribute.
    #[prost(map = "string, string", tag = "4")]
    pub attributes: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// App Engine Routing.
///
/// For more information about services, versions, and instances see
/// [An Overview of App
/// Engine](https://cloud.google.com/appengine/docs/python/an-overview-of-app-engine),
/// [Microservices Architecture on Google App
/// Engine](https://cloud.google.com/appengine/docs/python/microservices-on-app-engine),
/// [App Engine Standard request
/// routing](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed),
/// and [App Engine Flex request
/// routing](https://cloud.google.com/appengine/docs/flexible/python/how-requests-are-routed).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppEngineRouting {
    /// App service.
    ///
    /// By default, the job is sent to the service which is the default
    /// service when the job is attempted.
    #[prost(string, tag = "1")]
    pub service: std::string::String,
    /// App version.
    ///
    /// By default, the job is sent to the version which is the default
    /// version when the job is attempted.
    #[prost(string, tag = "2")]
    pub version: std::string::String,
    /// App instance.
    ///
    /// By default, the job is sent to an instance which is available when
    /// the job is attempted.
    ///
    /// Requests can only be sent to a specific instance if
    /// [manual scaling is used in App Engine
    /// Standard](https://cloud.google.com/appengine/docs/python/an-overview-of-app-engine?hl=en_US#scaling_types_and_instance_classes).
    /// App Engine Flex does not support instances. For more information, see
    /// [App Engine Standard request
    /// routing](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed)
    /// and [App Engine Flex request
    /// routing](https://cloud.google.com/appengine/docs/flexible/python/how-requests-are-routed).
    #[prost(string, tag = "3")]
    pub instance: std::string::String,
    /// Output only. The host that the job is sent to.
    ///
    /// For more information about how App Engine requests are routed, see
    /// [here](https://cloud.google.com/appengine/docs/standard/python/how-requests-are-routed).
    ///
    /// The host is constructed as:
    ///
    ///
    /// * `host = [application_domain_name]`</br>
    ///   `| [service] + '.' + [application_domain_name]`</br>
    ///   `| [version] + '.' + [application_domain_name]`</br>
    ///   `| [version_dot_service]+ '.' + [application_domain_name]`</br>
    ///   `| [instance] + '.' + [application_domain_name]`</br>
    ///   `| [instance_dot_service] + '.' + [application_domain_name]`</br>
    ///   `| [instance_dot_version] + '.' + [application_domain_name]`</br>
    ///   `| [instance_dot_version_dot_service] + '.' + [application_domain_name]`
    ///
    /// * `application_domain_name` = The domain name of the app, for
    ///   example <app-id>.appspot.com, which is associated with the
    ///   job's project ID.
    ///
    /// * `service =` [service][google.cloud.scheduler.v1beta1.AppEngineRouting.service]
    ///
    /// * `version =` [version][google.cloud.scheduler.v1beta1.AppEngineRouting.version]
    ///
    /// * `version_dot_service =`
    ///   [version][google.cloud.scheduler.v1beta1.AppEngineRouting.version] `+ '.' +`
    ///   [service][google.cloud.scheduler.v1beta1.AppEngineRouting.service]
    ///
    /// * `instance =` [instance][google.cloud.scheduler.v1beta1.AppEngineRouting.instance]
    ///
    /// * `instance_dot_service =`
    ///   [instance][google.cloud.scheduler.v1beta1.AppEngineRouting.instance] `+ '.' +`
    ///   [service][google.cloud.scheduler.v1beta1.AppEngineRouting.service]
    ///
    /// * `instance_dot_version =`
    ///   [instance][google.cloud.scheduler.v1beta1.AppEngineRouting.instance] `+ '.' +`
    ///   [version][google.cloud.scheduler.v1beta1.AppEngineRouting.version]
    ///
    /// * `instance_dot_version_dot_service =`
    ///   [instance][google.cloud.scheduler.v1beta1.AppEngineRouting.instance] `+ '.' +`
    ///   [version][google.cloud.scheduler.v1beta1.AppEngineRouting.version] `+ '.' +`
    ///   [service][google.cloud.scheduler.v1beta1.AppEngineRouting.service]
    ///
    ///
    /// If [service][google.cloud.scheduler.v1beta1.AppEngineRouting.service] is empty, then the job will be sent
    /// to the service which is the default service when the job is attempted.
    ///
    /// If [version][google.cloud.scheduler.v1beta1.AppEngineRouting.version] is empty, then the job will be sent
    /// to the version which is the default version when the job is attempted.
    ///
    /// If [instance][google.cloud.scheduler.v1beta1.AppEngineRouting.instance] is empty, then the job will be
    /// sent to an instance which is available when the job is attempted.
    ///
    /// If [service][google.cloud.scheduler.v1beta1.AppEngineRouting.service],
    /// [version][google.cloud.scheduler.v1beta1.AppEngineRouting.version], or
    /// [instance][google.cloud.scheduler.v1beta1.AppEngineRouting.instance] is invalid, then the job will be sent
    /// to the default version of the default service when the job is attempted.
    #[prost(string, tag = "4")]
    pub host: std::string::String,
}
/// Contains information needed for generating an
/// [OAuth token](https://developers.google.com/identity/protocols/OAuth2).
/// This type of authorization should generally only be used when calling Google
/// APIs hosted on *.googleapis.com.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuthToken {
    /// [Service account email](https://cloud.google.com/iam/docs/service-accounts)
    /// to be used for generating OAuth token.
    /// The service account must be within the same project as the job. The caller
    /// must have iam.serviceAccounts.actAs permission for the service account.
    #[prost(string, tag = "1")]
    pub service_account_email: std::string::String,
    /// OAuth scope to be used for generating OAuth access token.
    /// If not specified, "https://www.googleapis.com/auth/cloud-platform"
    /// will be used.
    #[prost(string, tag = "2")]
    pub scope: std::string::String,
}
/// Contains information needed for generating an
/// [OpenID Connect
/// token](https://developers.google.com/identity/protocols/OpenIDConnect).
/// This type of authorization can be used for many scenarios, including
/// calling Cloud Run, or endpoints where you intend to validate the token
/// yourself.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OidcToken {
    /// [Service account email](https://cloud.google.com/iam/docs/service-accounts)
    /// to be used for generating OIDC token.
    /// The service account must be within the same project as the job. The caller
    /// must have iam.serviceAccounts.actAs permission for the service account.
    #[prost(string, tag = "1")]
    pub service_account_email: std::string::String,
    /// Audience to be used when generating OIDC token. If not specified, the URI
    /// specified in target will be used.
    #[prost(string, tag = "2")]
    pub audience: std::string::String,
}
/// The HTTP method used to execute the job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HttpMethod {
    /// HTTP method unspecified. Defaults to POST.
    Unspecified = 0,
    /// HTTP POST
    Post = 1,
    /// HTTP GET
    Get = 2,
    /// HTTP HEAD
    Head = 3,
    /// HTTP PUT
    Put = 4,
    /// HTTP DELETE
    Delete = 5,
    /// HTTP PATCH
    Patch = 6,
    /// HTTP OPTIONS
    Options = 7,
}
/// Configuration for a job.
/// The maximum allowed size for a job is 100KB.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Optionally caller-specified in [CreateJob][google.cloud.scheduler.v1beta1.CloudScheduler.CreateJob], after
    /// which it becomes output only.
    ///
    /// The job name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`.
    ///
    /// * `PROJECT_ID` can contain letters ([A-Za-z]), numbers ([0-9]),
    ///    hyphens (-), colons (:), or periods (.).
    ///    For more information, see
    ///    [Identifying
    ///    projects](https://cloud.google.com/resource-manager/docs/creating-managing-projects#identifying_projects)
    /// * `LOCATION_ID` is the canonical ID for the job's location.
    ///    The list of available locations can be obtained by calling
    ///    [ListLocations][google.cloud.location.Locations.ListLocations].
    ///    For more information, see https://cloud.google.com/about/locations/.
    /// * `JOB_ID` can contain only letters ([A-Za-z]), numbers ([0-9]),
    ///    hyphens (-), or underscores (_). The maximum length is 500 characters.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optionally caller-specified in [CreateJob][google.cloud.scheduler.v1beta1.CloudScheduler.CreateJob] or
    /// [UpdateJob][google.cloud.scheduler.v1beta1.CloudScheduler.UpdateJob].
    ///
    /// A human-readable description for the job. This string must not contain
    /// more than 500 characters.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Required, except when used with [UpdateJob][google.cloud.scheduler.v1beta1.CloudScheduler.UpdateJob].
    ///
    /// Describes the schedule on which the job will be executed.
    ///
    /// The schedule can be either of the following types:
    ///
    /// * [Crontab](http://en.wikipedia.org/wiki/Cron#Overview)
    /// * English-like
    /// [schedule](https://cloud.google.com/scheduler/docs/configuring/cron-job-schedules)
    ///
    /// As a general rule, execution `n + 1` of a job will not begin
    /// until execution `n` has finished. Cloud Scheduler will never
    /// allow two simultaneously outstanding executions. For example,
    /// this implies that if the `n+1`th execution is scheduled to run at
    /// 16:00 but the `n`th execution takes until 16:15, the `n+1`th
    /// execution will not start until `16:15`.
    /// A scheduled start time will be delayed if the previous
    /// execution has not ended when its scheduled time occurs.
    ///
    /// If [retry_count][google.cloud.scheduler.v1beta1.RetryConfig.retry_count] > 0 and a job attempt fails,
    /// the job will be tried a total of [retry_count][google.cloud.scheduler.v1beta1.RetryConfig.retry_count]
    /// times, with exponential backoff, until the next scheduled start
    /// time.
    #[prost(string, tag = "20")]
    pub schedule: std::string::String,
    /// Specifies the time zone to be used in interpreting
    /// [schedule][google.cloud.scheduler.v1beta1.Job.schedule]. The value of this field must be a time
    /// zone name from the [tz database](http://en.wikipedia.org/wiki/Tz_database).
    ///
    /// Note that some time zones include a provision for
    /// daylight savings time. The rules for daylight saving time are
    /// determined by the chosen tz. For UTC use the string "utc". If a
    /// time zone is not specified, the default will be in UTC (also known
    /// as GMT).
    #[prost(string, tag = "21")]
    pub time_zone: std::string::String,
    /// Output only. The creation time of the job.
    #[prost(message, optional, tag = "9")]
    pub user_update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. State of the job.
    #[prost(enumeration = "job::State", tag = "10")]
    pub state: i32,
    /// Output only. The response from the target for the last attempted execution.
    #[prost(message, optional, tag = "11")]
    pub status: ::std::option::Option<super::super::super::rpc::Status>,
    /// Output only. The next time the job is scheduled. Note that this may be a
    /// retry of a previously failed attempt or the next execution time
    /// according to the schedule.
    #[prost(message, optional, tag = "17")]
    pub schedule_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the last job attempt started.
    #[prost(message, optional, tag = "18")]
    pub last_attempt_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Settings that determine the retry behavior.
    #[prost(message, optional, tag = "19")]
    pub retry_config: ::std::option::Option<RetryConfig>,
    /// The deadline for job attempts. If the request handler does not respond by
    /// this deadline then the request is cancelled and the attempt is marked as a
    /// `DEADLINE_EXCEEDED` failure. The failed attempt can be viewed in
    /// execution logs. Cloud Scheduler will retry the job according
    /// to the [RetryConfig][google.cloud.scheduler.v1beta1.RetryConfig].
    ///
    /// The allowed duration for this deadline is:
    ///
    /// * For [HTTP targets][google.cloud.scheduler.v1beta1.Job.http_target], between 15 seconds and 30 minutes.
    /// * For [App Engine HTTP targets][google.cloud.scheduler.v1beta1.Job.app_engine_http_target], between 15
    ///   seconds and 24 hours.
    /// * For [PubSub targets][google.cloud.scheduler.v1beta1.Job.pubsub_target], this field is ignored.
    #[prost(message, optional, tag = "22")]
    pub attempt_deadline: ::std::option::Option<::prost_types::Duration>,
    /// Required.
    ///
    /// Delivery settings containing destination and parameters.
    #[prost(oneof = "job::Target", tags = "4, 5, 6")]
    pub target: ::std::option::Option<job::Target>,
}
pub mod job {
    /// State of the job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// The job is executing normally.
        Enabled = 1,
        /// The job is paused by the user. It will not execute. A user can
        /// intentionally pause the job using
        /// [PauseJobRequest][google.cloud.scheduler.v1beta1.PauseJobRequest].
        Paused = 2,
        /// The job is disabled by the system due to error. The user
        /// cannot directly set a job to be disabled.
        Disabled = 3,
        /// The job state resulting from a failed [CloudScheduler.UpdateJob][google.cloud.scheduler.v1beta1.CloudScheduler.UpdateJob]
        /// operation. To recover a job from this state, retry
        /// [CloudScheduler.UpdateJob][google.cloud.scheduler.v1beta1.CloudScheduler.UpdateJob] until a successful response is received.
        UpdateFailed = 4,
    }
    /// Required.
    ///
    /// Delivery settings containing destination and parameters.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Pub/Sub target.
        #[prost(message, tag = "4")]
        PubsubTarget(super::PubsubTarget),
        /// App Engine HTTP target.
        #[prost(message, tag = "5")]
        AppEngineHttpTarget(super::AppEngineHttpTarget),
        /// HTTP target.
        #[prost(message, tag = "6")]
        HttpTarget(super::HttpTarget),
    }
}
/// Settings that determine the retry behavior.
///
/// By default, if a job does not complete successfully (meaning that
/// an acknowledgement is not received from the handler, then it will be retried
/// with exponential backoff according to the settings in [RetryConfig][google.cloud.scheduler.v1beta1.RetryConfig].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryConfig {
    /// The number of attempts that the system will make to run a job using the
    /// exponential backoff procedure described by
    /// [max_doublings][google.cloud.scheduler.v1beta1.RetryConfig.max_doublings].
    ///
    /// The default value of retry_count is zero.
    ///
    /// If retry_count is zero, a job attempt will *not* be retried if
    /// it fails. Instead the Cloud Scheduler system will wait for the
    /// next scheduled execution time.
    ///
    /// If retry_count is set to a non-zero number then Cloud Scheduler
    /// will retry failed attempts, using exponential backoff,
    /// retry_count times, or until the next scheduled execution time,
    /// whichever comes first.
    ///
    /// Values greater than 5 and negative values are not allowed.
    #[prost(int32, tag = "1")]
    pub retry_count: i32,
    /// The time limit for retrying a failed job, measured from time when an
    /// execution was first attempted. If specified with
    /// [retry_count][google.cloud.scheduler.v1beta1.RetryConfig.retry_count], the job will be retried until both
    /// limits are reached.
    ///
    /// The default value for max_retry_duration is zero, which means retry
    /// duration is unlimited.
    #[prost(message, optional, tag = "2")]
    pub max_retry_duration: ::std::option::Option<::prost_types::Duration>,
    /// The minimum amount of time to wait before retrying a job after
    /// it fails.
    ///
    /// The default value of this field is 5 seconds.
    #[prost(message, optional, tag = "3")]
    pub min_backoff_duration: ::std::option::Option<::prost_types::Duration>,
    /// The maximum amount of time to wait before retrying a job after
    /// it fails.
    ///
    /// The default value of this field is 1 hour.
    #[prost(message, optional, tag = "4")]
    pub max_backoff_duration: ::std::option::Option<::prost_types::Duration>,
    /// The time between retries will double `max_doublings` times.
    ///
    /// A job's retry interval starts at
    /// [min_backoff_duration][google.cloud.scheduler.v1beta1.RetryConfig.min_backoff_duration], then doubles
    /// `max_doublings` times, then increases linearly, and finally
    /// retries retries at intervals of
    /// [max_backoff_duration][google.cloud.scheduler.v1beta1.RetryConfig.max_backoff_duration] up to
    /// [retry_count][google.cloud.scheduler.v1beta1.RetryConfig.retry_count] times.
    ///
    /// For example, if [min_backoff_duration][google.cloud.scheduler.v1beta1.RetryConfig.min_backoff_duration] is
    /// 10s, [max_backoff_duration][google.cloud.scheduler.v1beta1.RetryConfig.max_backoff_duration] is 300s, and
    /// `max_doublings` is 3, then the a job will first be retried in 10s. The
    /// retry interval will double three times, and then increase linearly by
    /// 2^3 * 10s.  Finally, the job will retry at intervals of
    /// [max_backoff_duration][google.cloud.scheduler.v1beta1.RetryConfig.max_backoff_duration] until the job has
    /// been attempted [retry_count][google.cloud.scheduler.v1beta1.RetryConfig.retry_count] times. Thus, the
    /// requests will retry at 10s, 20s, 40s, 80s, 160s, 240s, 300s, 300s, ....
    ///
    /// The default value of this field is 5.
    #[prost(int32, tag = "5")]
    pub max_doublings: i32,
}
/// Request message for listing jobs using [ListJobs][google.cloud.scheduler.v1beta1.CloudScheduler.ListJobs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// Required. The location name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Requested page size.
    ///
    /// The maximum page size is 500. If unspecified, the page size will
    /// be the maximum. Fewer jobs than requested might be returned,
    /// even if more jobs exist; use next_page_token to determine if more
    /// jobs exist.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// A token identifying a page of results the server will return. To
    /// request the first page results, page_token must be empty. To
    /// request the next page of results, page_token must be the value of
    /// [next_page_token][google.cloud.scheduler.v1beta1.ListJobsResponse.next_page_token] returned from
    /// the previous call to [ListJobs][google.cloud.scheduler.v1beta1.CloudScheduler.ListJobs]. It is an error to
    /// switch the value of [filter][google.cloud.scheduler.v1beta1.ListJobsRequest.filter] or
    /// [order_by][google.cloud.scheduler.v1beta1.ListJobsRequest.order_by] while iterating through pages.
    #[prost(string, tag = "6")]
    pub page_token: std::string::String,
}
/// Response message for listing jobs using [ListJobs][google.cloud.scheduler.v1beta1.CloudScheduler.ListJobs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// The list of jobs.
    #[prost(message, repeated, tag = "1")]
    pub jobs: ::std::vec::Vec<Job>,
    /// A token to retrieve next page of results. Pass this value in the
    /// [page_token][google.cloud.scheduler.v1beta1.ListJobsRequest.page_token] field in the subsequent call to
    /// [ListJobs][google.cloud.scheduler.v1beta1.CloudScheduler.ListJobs] to retrieve the next page of results.
    /// If this is empty it indicates that there are no more results
    /// through which to paginate.
    ///
    /// The page token is valid for only 2 hours.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for [GetJob][google.cloud.scheduler.v1beta1.CloudScheduler.GetJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// Required. The job name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [CreateJob][google.cloud.scheduler.v1beta1.CloudScheduler.CreateJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobRequest {
    /// Required. The location name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The job to add. The user can optionally specify a name for the
    /// job in [name][google.cloud.scheduler.v1beta1.Job.name]. [name][google.cloud.scheduler.v1beta1.Job.name] cannot be the same as an
    /// existing job. If a name is not specified then the system will
    /// generate a random unique name that will be returned
    /// ([name][google.cloud.scheduler.v1beta1.Job.name]) in the response.
    #[prost(message, optional, tag = "2")]
    pub job: ::std::option::Option<Job>,
}
/// Request message for [UpdateJob][google.cloud.scheduler.v1beta1.CloudScheduler.UpdateJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJobRequest {
    /// Required. The new job properties. [name][google.cloud.scheduler.v1beta1.Job.name] must be specified.
    ///
    /// Output only fields cannot be modified using UpdateJob.
    /// Any value specified for an output only field will be ignored.
    #[prost(message, optional, tag = "1")]
    pub job: ::std::option::Option<Job>,
    /// A  mask used to specify which fields of the job are being updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for deleting a job using
/// [DeleteJob][google.cloud.scheduler.v1beta1.CloudScheduler.DeleteJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteJobRequest {
    /// Required. The job name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [PauseJob][google.cloud.scheduler.v1beta1.CloudScheduler.PauseJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseJobRequest {
    /// Required. The job name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [ResumeJob][google.cloud.scheduler.v1beta1.CloudScheduler.ResumeJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeJobRequest {
    /// Required. The job name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for forcing a job to run now using
/// [RunJob][google.cloud.scheduler.v1beta1.CloudScheduler.RunJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunJobRequest {
    /// Required. The job name. For example:
    /// `projects/PROJECT_ID/locations/LOCATION_ID/jobs/JOB_ID`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod cloud_scheduler_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Cloud Scheduler API allows external entities to reliably"]
    #[doc = " schedule asynchronous jobs."]
    pub struct CloudSchedulerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudSchedulerClient<T>
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
        #[doc = " Lists jobs."]
        pub async fn list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.scheduler.v1beta1.CloudScheduler/ListJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a job."]
        pub async fn get_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.scheduler.v1beta1.CloudScheduler/GetJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a job."]
        pub async fn create_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.scheduler.v1beta1.CloudScheduler/CreateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a job."]
        #[doc = ""]
        #[doc = " If successful, the updated [Job][google.cloud.scheduler.v1beta1.Job] is returned. If the job does"]
        #[doc = " not exist, `NOT_FOUND` is returned."]
        #[doc = ""]
        #[doc = " If UpdateJob does not successfully return, it is possible for the"]
        #[doc = " job to be in an [Job.State.UPDATE_FAILED][google.cloud.scheduler.v1beta1.Job.State.UPDATE_FAILED] state. A job in this state may"]
        #[doc = " not be executed. If this happens, retry the UpdateJob request"]
        #[doc = " until a successful response is received."]
        pub async fn update_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.scheduler.v1beta1.CloudScheduler/UpdateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a job."]
        pub async fn delete_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.scheduler.v1beta1.CloudScheduler/DeleteJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Pauses a job."]
        #[doc = ""]
        #[doc = " If a job is paused then the system will stop executing the job"]
        #[doc = " until it is re-enabled via [ResumeJob][google.cloud.scheduler.v1beta1.CloudScheduler.ResumeJob]. The"]
        #[doc = " state of the job is stored in [state][google.cloud.scheduler.v1beta1.Job.state]; if paused it"]
        #[doc = " will be set to [Job.State.PAUSED][google.cloud.scheduler.v1beta1.Job.State.PAUSED]. A job must be in [Job.State.ENABLED][google.cloud.scheduler.v1beta1.Job.State.ENABLED]"]
        #[doc = " to be paused."]
        pub async fn pause_job(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.scheduler.v1beta1.CloudScheduler/PauseJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Resume a job."]
        #[doc = ""]
        #[doc = " This method reenables a job after it has been [Job.State.PAUSED][google.cloud.scheduler.v1beta1.Job.State.PAUSED]. The"]
        #[doc = " state of a job is stored in [Job.state][google.cloud.scheduler.v1beta1.Job.state]; after calling this method it"]
        #[doc = " will be set to [Job.State.ENABLED][google.cloud.scheduler.v1beta1.Job.State.ENABLED]. A job must be in"]
        #[doc = " [Job.State.PAUSED][google.cloud.scheduler.v1beta1.Job.State.PAUSED] to be resumed."]
        pub async fn resume_job(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.scheduler.v1beta1.CloudScheduler/ResumeJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Forces a job to run now."]
        #[doc = ""]
        #[doc = " When this method is called, Cloud Scheduler will dispatch the job, even"]
        #[doc = " if the job is already running."]
        pub async fn run_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RunJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.scheduler.v1beta1.CloudScheduler/RunJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CloudSchedulerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CloudSchedulerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CloudSchedulerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cloud_scheduler_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CloudSchedulerServer."]
    #[async_trait]
    pub trait CloudScheduler: Send + Sync + 'static {
        #[doc = " Lists jobs."]
        async fn list_jobs(
            &self,
            request: tonic::Request<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status>;
        #[doc = " Gets a job."]
        async fn get_job(
            &self,
            request: tonic::Request<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
        #[doc = " Creates a job."]
        async fn create_job(
            &self,
            request: tonic::Request<super::CreateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
        #[doc = " Updates a job."]
        #[doc = ""]
        #[doc = " If successful, the updated [Job][google.cloud.scheduler.v1beta1.Job] is returned. If the job does"]
        #[doc = " not exist, `NOT_FOUND` is returned."]
        #[doc = ""]
        #[doc = " If UpdateJob does not successfully return, it is possible for the"]
        #[doc = " job to be in an [Job.State.UPDATE_FAILED][google.cloud.scheduler.v1beta1.Job.State.UPDATE_FAILED] state. A job in this state may"]
        #[doc = " not be executed. If this happens, retry the UpdateJob request"]
        #[doc = " until a successful response is received."]
        async fn update_job(
            &self,
            request: tonic::Request<super::UpdateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
        #[doc = " Deletes a job."]
        async fn delete_job(
            &self,
            request: tonic::Request<super::DeleteJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Pauses a job."]
        #[doc = ""]
        #[doc = " If a job is paused then the system will stop executing the job"]
        #[doc = " until it is re-enabled via [ResumeJob][google.cloud.scheduler.v1beta1.CloudScheduler.ResumeJob]. The"]
        #[doc = " state of the job is stored in [state][google.cloud.scheduler.v1beta1.Job.state]; if paused it"]
        #[doc = " will be set to [Job.State.PAUSED][google.cloud.scheduler.v1beta1.Job.State.PAUSED]. A job must be in [Job.State.ENABLED][google.cloud.scheduler.v1beta1.Job.State.ENABLED]"]
        #[doc = " to be paused."]
        async fn pause_job(
            &self,
            request: tonic::Request<super::PauseJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
        #[doc = " Resume a job."]
        #[doc = ""]
        #[doc = " This method reenables a job after it has been [Job.State.PAUSED][google.cloud.scheduler.v1beta1.Job.State.PAUSED]. The"]
        #[doc = " state of a job is stored in [Job.state][google.cloud.scheduler.v1beta1.Job.state]; after calling this method it"]
        #[doc = " will be set to [Job.State.ENABLED][google.cloud.scheduler.v1beta1.Job.State.ENABLED]. A job must be in"]
        #[doc = " [Job.State.PAUSED][google.cloud.scheduler.v1beta1.Job.State.PAUSED] to be resumed."]
        async fn resume_job(
            &self,
            request: tonic::Request<super::ResumeJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
        #[doc = " Forces a job to run now."]
        #[doc = ""]
        #[doc = " When this method is called, Cloud Scheduler will dispatch the job, even"]
        #[doc = " if the job is already running."]
        async fn run_job(
            &self,
            request: tonic::Request<super::RunJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
    }
    #[doc = " The Cloud Scheduler API allows external entities to reliably"]
    #[doc = " schedule asynchronous jobs."]
    #[derive(Debug)]
    pub struct CloudSchedulerServer<T: CloudScheduler> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CloudScheduler> CloudSchedulerServer<T> {
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
    impl<T, B> Service<http::Request<B>> for CloudSchedulerServer<T>
    where
        T: CloudScheduler,
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
                "/google.cloud.scheduler.v1beta1.CloudScheduler/ListJobs" => {
                    #[allow(non_camel_case_types)]
                    struct ListJobsSvc<T: CloudScheduler>(pub Arc<T>);
                    impl<T: CloudScheduler> tonic::server::UnaryService<super::ListJobsRequest> for ListJobsSvc<T> {
                        type Response = super::ListJobsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListJobsSvc(inner);
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
                "/google.cloud.scheduler.v1beta1.CloudScheduler/GetJob" => {
                    #[allow(non_camel_case_types)]
                    struct GetJobSvc<T: CloudScheduler>(pub Arc<T>);
                    impl<T: CloudScheduler> tonic::server::UnaryService<super::GetJobRequest> for GetJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetJobSvc(inner);
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
                "/google.cloud.scheduler.v1beta1.CloudScheduler/CreateJob" => {
                    #[allow(non_camel_case_types)]
                    struct CreateJobSvc<T: CloudScheduler>(pub Arc<T>);
                    impl<T: CloudScheduler> tonic::server::UnaryService<super::CreateJobRequest> for CreateJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateJobSvc(inner);
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
                "/google.cloud.scheduler.v1beta1.CloudScheduler/UpdateJob" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateJobSvc<T: CloudScheduler>(pub Arc<T>);
                    impl<T: CloudScheduler> tonic::server::UnaryService<super::UpdateJobRequest> for UpdateJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateJobSvc(inner);
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
                "/google.cloud.scheduler.v1beta1.CloudScheduler/DeleteJob" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteJobSvc<T: CloudScheduler>(pub Arc<T>);
                    impl<T: CloudScheduler> tonic::server::UnaryService<super::DeleteJobRequest> for DeleteJobSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteJobSvc(inner);
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
                "/google.cloud.scheduler.v1beta1.CloudScheduler/PauseJob" => {
                    #[allow(non_camel_case_types)]
                    struct PauseJobSvc<T: CloudScheduler>(pub Arc<T>);
                    impl<T: CloudScheduler> tonic::server::UnaryService<super::PauseJobRequest> for PauseJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PauseJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pause_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PauseJobSvc(inner);
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
                "/google.cloud.scheduler.v1beta1.CloudScheduler/ResumeJob" => {
                    #[allow(non_camel_case_types)]
                    struct ResumeJobSvc<T: CloudScheduler>(pub Arc<T>);
                    impl<T: CloudScheduler> tonic::server::UnaryService<super::ResumeJobRequest> for ResumeJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResumeJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).resume_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ResumeJobSvc(inner);
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
                "/google.cloud.scheduler.v1beta1.CloudScheduler/RunJob" => {
                    #[allow(non_camel_case_types)]
                    struct RunJobSvc<T: CloudScheduler>(pub Arc<T>);
                    impl<T: CloudScheduler> tonic::server::UnaryService<super::RunJobRequest> for RunJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RunJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).run_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RunJobSvc(inner);
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
    impl<T: CloudScheduler> Clone for CloudSchedulerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CloudScheduler> Clone for _Inner<T> {
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
