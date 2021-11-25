/// Common audit log format for Google Cloud Platform API operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditLog {
    /// The name of the API service performing the operation. For example,
    /// `"compute.googleapis.com"`.
    #[prost(string, tag = "7")]
    pub service_name: ::prost::alloc::string::String,
    /// The name of the service method or operation.
    /// For API calls, this should be the name of the API method.
    /// For example,
    ///
    ///     "google.cloud.bigquery.v2.TableService.InsertTable"
    ///     "google.logging.v2.ConfigServiceV2.CreateSink"
    #[prost(string, tag = "8")]
    pub method_name: ::prost::alloc::string::String,
    /// The resource or collection that is the target of the operation.
    /// The name is a scheme-less URI, not including the API service name.
    /// For example:
    ///
    ///     "projects/PROJECT_ID/zones/us-central1-a/instances"
    ///     "projects/PROJECT_ID/datasets/DATASET_ID"
    #[prost(string, tag = "11")]
    pub resource_name: ::prost::alloc::string::String,
    /// The resource location information.
    #[prost(message, optional, tag = "20")]
    pub resource_location: ::core::option::Option<ResourceLocation>,
    /// The resource's original state before mutation. Present only for
    /// operations which have successfully modified the targeted resource(s).
    /// In general, this field should contain all changed fields, except those
    /// that are already been included in `request`, `response`, `metadata` or
    /// `service_data` fields.
    /// When the JSON object represented here has a proto equivalent,
    /// the proto name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "19")]
    pub resource_original_state: ::core::option::Option<::prost_types::Struct>,
    /// The number of items returned from a List or Query API method,
    /// if applicable.
    #[prost(int64, tag = "12")]
    pub num_response_items: i64,
    /// The status of the overall operation.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::rpc::Status>,
    /// Authentication information.
    #[prost(message, optional, tag = "3")]
    pub authentication_info: ::core::option::Option<AuthenticationInfo>,
    /// Authorization information. If there are multiple
    /// resources or permissions involved, then there is
    /// one AuthorizationInfo element for each {resource, permission} tuple.
    #[prost(message, repeated, tag = "9")]
    pub authorization_info: ::prost::alloc::vec::Vec<AuthorizationInfo>,
    /// Metadata about the operation.
    #[prost(message, optional, tag = "4")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// The operation request. This may not include all request parameters,
    /// such as those that are too large, privacy-sensitive, or duplicated
    /// elsewhere in the log record.
    /// It should never include user-generated data, such as file contents.
    /// When the JSON object represented here has a proto equivalent, the proto
    /// name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "16")]
    pub request: ::core::option::Option<::prost_types::Struct>,
    /// The operation response. This may not include all response elements,
    /// such as those that are too large, privacy-sensitive, or duplicated
    /// elsewhere in the log record.
    /// It should never include user-generated data, such as file contents.
    /// When the JSON object represented here has a proto equivalent, the proto
    /// name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "17")]
    pub response: ::core::option::Option<::prost_types::Struct>,
    /// Other service-specific data about the request, response, and other
    /// information associated with the current audited event.
    #[prost(message, optional, tag = "18")]
    pub metadata: ::core::option::Option<::prost_types::Struct>,
    /// Deprecated. Use the `metadata` field instead.
    /// Other service-specific data about the request, response, and other
    /// activities.
    #[deprecated]
    #[prost(message, optional, tag = "15")]
    pub service_data: ::core::option::Option<::prost_types::Any>,
}
/// Authentication information for the operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationInfo {
    /// The email address of the authenticated user (or service account on behalf
    /// of third party principal) making the request. For third party identity
    /// callers, the `principal_subject` field is populated instead of this field.
    /// For privacy reasons, the principal email address is sometimes redacted.
    /// For more information, see
    /// <https://cloud.google.com/logging/docs/audit#user-id.>
    #[prost(string, tag = "1")]
    pub principal_email: ::prost::alloc::string::String,
    /// The authority selector specified by the requestor, if any.
    /// It is not guaranteed that the principal was allowed to use this authority.
    #[prost(string, tag = "2")]
    pub authority_selector: ::prost::alloc::string::String,
    /// The third party identification (if any) of the authenticated user making
    /// the request.
    /// When the JSON object represented here has a proto equivalent, the proto
    /// name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "4")]
    pub third_party_principal: ::core::option::Option<::prost_types::Struct>,
    /// The name of the service account key used to create or exchange
    /// credentials for authenticating the service account making the request.
    /// This is a scheme-less URI full resource name. For example:
    ///
    /// "//iam.googleapis.com/projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}"
    #[prost(string, tag = "5")]
    pub service_account_key_name: ::prost::alloc::string::String,
    /// Identity delegation history of an authenticated service account that makes
    /// the request. It contains information on the real authorities that try to
    /// access GCP resources by delegating on a service account. When multiple
    /// authorities present, they are guaranteed to be sorted based on the original
    /// ordering of the identity delegation events.
    #[prost(message, repeated, tag = "6")]
    pub service_account_delegation_info: ::prost::alloc::vec::Vec<ServiceAccountDelegationInfo>,
    /// String representation of identity of requesting party.
    /// Populated for both first and third party identities.
    #[prost(string, tag = "8")]
    pub principal_subject: ::prost::alloc::string::String,
}
/// Authorization information for the operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationInfo {
    /// The resource being accessed, as a REST-style or cloud resource string.
    /// For example:
    ///
    ///     bigquery.googleapis.com/projects/PROJECTID/datasets/DATASETID
    /// or
    ///     projects/PROJECTID/datasets/DATASETID
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    /// The required IAM permission.
    #[prost(string, tag = "2")]
    pub permission: ::prost::alloc::string::String,
    /// Whether or not authorization for `resource` and `permission`
    /// was granted.
    #[prost(bool, tag = "3")]
    pub granted: bool,
    /// Resource attributes used in IAM condition evaluation. This field contains
    /// resource attributes like resource type and resource name.
    ///
    /// To get the whole view of the attributes used in IAM
    /// condition evaluation, the user must also look into
    /// `AuditLog.request_metadata.request_attributes`.
    #[prost(message, optional, tag = "5")]
    pub resource_attributes:
        ::core::option::Option<super::super::rpc::context::attribute_context::Resource>,
}
/// Metadata about the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    /// The IP address of the caller.
    /// For caller from internet, this will be public IPv4 or IPv6 address.
    /// For caller from a Compute Engine VM with external IP address, this
    /// will be the VM's external IP address. For caller from a Compute
    /// Engine VM without external IP address, if the VM is in the same
    /// organization (or project) as the accessed resource, `caller_ip` will
    /// be the VM's internal IPv4 address, otherwise the `caller_ip` will be
    /// redacted to "gce-internal-ip".
    /// See <https://cloud.google.com/compute/docs/vpc/> for more information.
    #[prost(string, tag = "1")]
    pub caller_ip: ::prost::alloc::string::String,
    /// The user agent of the caller.
    /// This information is not authenticated and should be treated accordingly.
    /// For example:
    ///
    /// +   `google-api-python-client/1.4.0`:
    ///     The request was made by the Google API client for Python.
    /// +   `Cloud SDK Command Line Tool apitools-client/1.0 gcloud/0.9.62`:
    ///     The request was made by the Google Cloud SDK CLI (gcloud).
    /// +   `AppEngine-Google; (+<http://code.google.com/appengine;> appid:
    /// s~my-project`:
    ///     The request was made from the `my-project` App Engine app.
    #[prost(string, tag = "2")]
    pub caller_supplied_user_agent: ::prost::alloc::string::String,
    /// The network of the caller.
    /// Set only if the network host project is part of the same GCP organization
    /// (or project) as the accessed resource.
    /// See <https://cloud.google.com/compute/docs/vpc/> for more information.
    /// This is a scheme-less URI full resource name. For example:
    ///
    ///     "//compute.googleapis.com/projects/PROJECT_ID/global/networks/NETWORK_ID"
    #[prost(string, tag = "3")]
    pub caller_network: ::prost::alloc::string::String,
    /// Request attributes used in IAM condition evaluation. This field contains
    /// request attributes like request time and access levels associated with
    /// the request.
    ///
    ///
    /// To get the whole view of the attributes used in IAM
    /// condition evaluation, the user must also look into
    /// `AuditLog.authentication_info.resource_attributes`.
    #[prost(message, optional, tag = "7")]
    pub request_attributes:
        ::core::option::Option<super::super::rpc::context::attribute_context::Request>,
    /// The destination of a network activity, such as accepting a TCP connection.
    /// In a multi hop network activity, the destination represents the receiver of
    /// the last hop. Only two fields are used in this message, Peer.port and
    /// Peer.ip. These fields are optionally populated by those services utilizing
    /// the IAM condition feature.
    #[prost(message, optional, tag = "8")]
    pub destination_attributes:
        ::core::option::Option<super::super::rpc::context::attribute_context::Peer>,
}
/// Location information about a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLocation {
    /// The locations of a resource after the execution of the operation.
    /// Requests to create or delete a location based resource must populate
    /// the 'current_locations' field and not the 'original_locations' field.
    /// For example:
    ///
    ///     "europe-west1-a"
    ///     "us-east1"
    ///     "nam3"
    #[prost(string, repeated, tag = "1")]
    pub current_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The locations of a resource prior to the execution of the operation.
    /// Requests that mutate the resource's location must populate both the
    /// 'original_locations' as well as the 'current_locations' fields.
    /// For example:
    ///
    ///     "europe-west1-a"
    ///     "us-east1"
    ///     "nam3"
    #[prost(string, repeated, tag = "2")]
    pub original_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Identity delegation history of an authenticated service account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccountDelegationInfo {
    /// A string representing the principal_subject associated with the identity.
    /// For most identities, the format will be
    /// `principal://iam.googleapis.com/{identity pool name}/subject/{subject)`
    /// except for some GKE identities (GKE_WORKLOAD, FREEFORM, GKE_HUB_WORKLOAD)
    /// that are still in the legacy format `serviceAccount:{identity pool
    /// name}\[{subject}\]`
    #[prost(string, tag = "3")]
    pub principal_subject: ::prost::alloc::string::String,
    /// Entity that creates credentials for service account and assumes its
    /// identity for authentication.
    #[prost(oneof = "service_account_delegation_info::Authority", tags = "1, 2")]
    pub authority: ::core::option::Option<service_account_delegation_info::Authority>,
}
/// Nested message and enum types in `ServiceAccountDelegationInfo`.
pub mod service_account_delegation_info {
    /// First party identity principal.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FirstPartyPrincipal {
        /// The email address of a Google account.
        #[prost(string, tag = "1")]
        pub principal_email: ::prost::alloc::string::String,
        /// Metadata about the service that uses the service account.
        #[prost(message, optional, tag = "2")]
        pub service_metadata: ::core::option::Option<::prost_types::Struct>,
    }
    /// Third party identity principal.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThirdPartyPrincipal {
        /// Metadata about third party identity.
        #[prost(message, optional, tag = "1")]
        pub third_party_claims: ::core::option::Option<::prost_types::Struct>,
    }
    /// Entity that creates credentials for service account and assumes its
    /// identity for authentication.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Authority {
        /// First party (Google) identity as the real authority.
        #[prost(message, tag = "1")]
        FirstPartyPrincipal(FirstPartyPrincipal),
        /// Third party identity as the real authority.
        #[prost(message, tag = "2")]
        ThirdPartyPrincipal(ThirdPartyPrincipal),
    }
}
/// Audit log format for BigQuery cloud audit logs metadata.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryAuditMetadata {
    /// BigQuery event information.
    #[prost(
        oneof = "big_query_audit_metadata::Event",
        tags = "1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 19, 16, 17, 18"
    )]
    pub event: ::core::option::Option<big_query_audit_metadata::Event>,
}
/// Nested message and enum types in `BigQueryAuditMetadata`.
pub mod big_query_audit_metadata {
    /// Job insertion event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct JobInsertion {
        /// Job metadata.
        #[prost(message, optional, tag = "1")]
        pub job: ::core::option::Option<Job>,
        /// Describes how the job was inserted.
        #[prost(enumeration = "job_insertion::Reason", tag = "2")]
        pub reason: i32,
    }
    /// Nested message and enum types in `JobInsertion`.
    pub mod job_insertion {
        /// Describes how the job was inserted.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Job was inserted using the jobs.insert API.
            JobInsertRequest = 1,
            /// Job was inserted using the jobs.query RPC.
            QueryRequest = 2,
        }
    }
    /// Job state change event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct JobChange {
        /// Job state before the job state change.
        #[prost(enumeration = "JobState", tag = "1")]
        pub before: i32,
        /// Job state after the job state change.
        #[prost(enumeration = "JobState", tag = "2")]
        pub after: i32,
        /// Job metadata.
        #[prost(message, optional, tag = "3")]
        pub job: ::core::option::Option<Job>,
    }
    /// Dataset creation event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetCreation {
        /// Dataset metadata.
        #[prost(message, optional, tag = "1")]
        pub dataset: ::core::option::Option<Dataset>,
        /// Describes how the dataset was created.
        #[prost(enumeration = "dataset_creation::Reason", tag = "2")]
        pub reason: i32,
        /// The URI of the job that created the dataset.
        /// Present if the reason is QUERY.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "3")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `DatasetCreation`.
    pub mod dataset_creation {
        /// Describes how the dataset was created.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Dataset was created using the datasets.create API.
            Create = 1,
            /// Dataset was created using a query job, e.g., CREATE SCHEMA statement.
            Query = 2,
        }
    }
    /// Dataset change event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetChange {
        /// Dataset metadata after the change.
        #[prost(message, optional, tag = "1")]
        pub dataset: ::core::option::Option<Dataset>,
        /// Describes how the dataset was changed.
        #[prost(enumeration = "dataset_change::Reason", tag = "2")]
        pub reason: i32,
        /// The URI of the job that updated the dataset.
        /// Present if the reason is QUERY.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "3")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `DatasetChange`.
    pub mod dataset_change {
        /// Describes how the dataset was changed.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Dataset was changed using the datasets.update or datasets.patch API.
            Update = 1,
            /// Dataset was changed using the SetIamPolicy API.
            SetIamPolicy = 2,
            /// Dataset was changed using a query job, e.g., ALTER SCHEMA statement.
            Query = 3,
        }
    }
    /// Dataset deletion event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DatasetDeletion {
        /// Describes how the dataset was deleted.
        #[prost(enumeration = "dataset_deletion::Reason", tag = "1")]
        pub reason: i32,
        /// The URI of the job that deleted the dataset.
        /// Present if the reason is QUERY.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "2")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `DatasetDeletion`.
    pub mod dataset_deletion {
        /// Describes how the dataset was deleted.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Dataset was deleted using the datasets.delete API.
            Delete = 1,
            /// Dataset was deleted using a query job, e.g., DROP SCHEMA statement.
            Query = 2,
        }
    }
    /// Table creation event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableCreation {
        /// Table metadata.
        #[prost(message, optional, tag = "1")]
        pub table: ::core::option::Option<Table>,
        /// Describes how the table was created.
        #[prost(enumeration = "table_creation::Reason", tag = "3")]
        pub reason: i32,
        /// The URI of the job that created a table.
        /// Present if the reason is JOB or QUERY.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "4")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `TableCreation`.
    pub mod table_creation {
        /// Describes how the table was created.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Table was created as a destination table during a query, load or copy
            /// job.
            Job = 1,
            /// Table was created using a DDL query.
            Query = 2,
            /// Table was created using the tables.create API.
            TableInsertRequest = 3,
        }
    }
    /// Model creation event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelCreation {
        /// Model metadata.
        #[prost(message, optional, tag = "1")]
        pub model: ::core::option::Option<Model>,
        /// Describes how the model was created.
        #[prost(enumeration = "model_creation::Reason", tag = "3")]
        pub reason: i32,
        /// The URI of the job that created the model.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "4")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `ModelCreation`.
    pub mod model_creation {
        /// Describes how the model was created.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Model was created using a DDL query.
            Query = 2,
        }
    }
    /// Routine creation event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoutineCreation {
        /// Created routine.
        #[prost(message, optional, tag = "1")]
        pub routine: ::core::option::Option<Routine>,
        /// Describes how the routine was created.
        #[prost(enumeration = "routine_creation::Reason", tag = "3")]
        pub reason: i32,
        /// The URI of the job that created the routine.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "4")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `RoutineCreation`.
    pub mod routine_creation {
        /// Describes how the routine was created.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Routine was created using a DDL query.
            Query = 1,
            /// Routine was created using the routines.create API.
            RoutineInsertRequest = 2,
        }
    }
    /// Table data read event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableDataRead {
        /// List of the accessed fields. Entire list is truncated if the record size
        /// exceeds 100K.
        #[prost(string, repeated, tag = "2")]
        pub fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// True if the fields list was truncated.
        #[prost(bool, tag = "8")]
        pub fields_truncated: bool,
        /// List of the referenced policy tags. That is, policy tags attached to the
        /// accessed fields or their ancestors.
        /// Policy tag resource name is a string of the format:
        /// `projects/<project_id>/locations/<location_id>/taxonomies/<taxonomy_id>/policyTags/<policy_tag_id>`
        #[prost(string, repeated, tag = "9")]
        pub policy_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// True if the policy tag list was truncated. At most 100 policy tags can be
        /// saved.
        #[prost(bool, tag = "10")]
        pub policy_tags_truncated: bool,
        /// Describes how the table data was read.
        #[prost(enumeration = "table_data_read::Reason", tag = "3")]
        pub reason: i32,
        /// The URI of the job that read a table.
        /// Present if the reason is JOB but can be redacted for privacy reasons.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "4")]
        pub job_name: ::prost::alloc::string::String,
        /// The URI of the read session that read a table.
        /// Present if the reason is CREATE_READ_SESSION.
        ///
        /// Format:
        /// `projects/<project_id>/locations/<location>/sessions/<session_id>`.
        #[prost(string, tag = "5")]
        pub session_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `TableDataRead`.
    pub mod table_data_read {
        /// Describes how the table data was read.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Table was used as a source table during a BigQuery job.
            Job = 1,
            /// Table data was accessed using the tabledata.list API.
            TabledataListRequest = 2,
            /// Table data was accessed using the jobs.getQueryResults API.
            GetQueryResultsRequest = 3,
            /// Table data was accessed using the jobs.query RPC.
            QueryRequest = 4,
            /// Table data was accessed using storage.CreateReadSession API.
            CreateReadSession = 5,
            /// Table data was accessed during a materialized view refresh.
            MaterializedViewRefresh = 6,
        }
    }
    /// Table metadata change event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableChange {
        /// Updated table metadata.
        #[prost(message, optional, tag = "1")]
        pub table: ::core::option::Option<Table>,
        /// True if the table was truncated.
        #[prost(bool, tag = "4")]
        pub truncated: bool,
        /// Describes how the table metadata was changed.
        #[prost(enumeration = "table_change::Reason", tag = "5")]
        pub reason: i32,
        /// The URI of the job that changed a table.
        /// Present if the reason is JOB or QUERY.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "6")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `TableChange`.
    pub mod table_change {
        /// Describes how the table metadata was changed.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Table metadata was updated using the tables.update or tables.patch API.
            TableUpdateRequest = 1,
            /// Table was used as a job destination table.
            Job = 2,
            /// Table metadata was updated using a DML or DDL query.
            Query = 3,
        }
    }
    /// Model metadata change event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelMetadataChange {
        /// Updated model.
        #[prost(message, optional, tag = "1")]
        pub model: ::core::option::Option<Model>,
        /// Describes how the model metadata was changed.
        #[prost(enumeration = "model_metadata_change::Reason", tag = "2")]
        pub reason: i32,
        /// The URI of the job that changed the model metadata.
        /// Present if and only if the reason is QUERY.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "3")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `ModelMetadataChange`.
    pub mod model_metadata_change {
        /// Describes how the model metadata was changed.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Model metadata was updated using the models.patch API.
            ModelPatchRequest = 1,
            /// Model metadata was updated using a DDL query.
            Query = 2,
        }
    }
    /// Routine change event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoutineChange {
        /// Updated routine.
        #[prost(message, optional, tag = "1")]
        pub routine: ::core::option::Option<Routine>,
        /// Describes how the routine was updated.
        #[prost(enumeration = "routine_change::Reason", tag = "3")]
        pub reason: i32,
        /// The URI of the job that updated the routine.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "4")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `RoutineChange`.
    pub mod routine_change {
        /// Describes how the routine was updated.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Routine was updated using a DDL query.
            Query = 1,
            /// Routine was updated using the routines.update or routines.patch API.
            RoutineUpdateRequest = 2,
        }
    }
    /// Table data change event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableDataChange {
        /// Number of deleted rows.
        #[prost(int64, tag = "1")]
        pub deleted_rows_count: i64,
        /// Number of inserted rows.
        #[prost(int64, tag = "2")]
        pub inserted_rows_count: i64,
        /// True if the table was truncated.
        #[prost(bool, tag = "3")]
        pub truncated: bool,
        /// Describes how the table data was changed.
        #[prost(enumeration = "table_data_change::Reason", tag = "4")]
        pub reason: i32,
        /// The URI of the job that changed a table.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "5")]
        pub job_name: ::prost::alloc::string::String,
        /// If written from WRITE_API, the name of the stream.
        ///
        /// Format:
        /// `projects/<project_id>/datasets/<dataset_id>/tables/<table_id>/streams/<stream_id>`
        #[prost(string, tag = "6")]
        pub stream_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `TableDataChange`.
    pub mod table_data_change {
        /// Describes how the table data was changed.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Table was used as a job destination table.
            Job = 1,
            /// Table data was updated using a DML or DDL query.
            Query = 2,
            /// Table data was updated during a materialized view refresh.
            MaterializedViewRefresh = 3,
            /// Table data was added using the Write API.
            WriteApi = 4,
        }
    }
    /// Model data change event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelDataChange {
        /// Describes how the model data was changed.
        #[prost(enumeration = "model_data_change::Reason", tag = "1")]
        pub reason: i32,
        /// The URI of the job that changed the model data.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "2")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `ModelDataChange`.
    pub mod model_data_change {
        /// Describes how the model data was changed.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Model data was changed using a DDL query.
            Query = 1,
        }
    }
    /// Model data read event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelDataRead {
        /// Describes how the model data was read.
        #[prost(enumeration = "model_data_read::Reason", tag = "1")]
        pub reason: i32,
        /// The URI of the job that read the model data.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "2")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `ModelDataRead`.
    pub mod model_data_read {
        /// Describes how the model data was read.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Model was used as a source model during a BigQuery job.
            Job = 1,
        }
    }
    /// BigQuery dataset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Dataset {
        /// Dataset URI.
        ///
        /// Format: `projects/<project_id>/datasets/<dataset_id>`.
        #[prost(string, tag = "1")]
        pub dataset_name: ::prost::alloc::string::String,
        /// Dataset creation time.
        #[prost(message, optional, tag = "3")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Dataset metadata last update time.
        #[prost(message, optional, tag = "4")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The access control list for the dataset.
        #[prost(message, optional, tag = "5")]
        pub acl: ::core::option::Option<BigQueryAcl>,
        /// Default expiration time for tables in the dataset.
        #[prost(message, optional, tag = "6")]
        pub default_table_expire_duration: ::core::option::Option<::prost_types::Duration>,
        /// User-provided metadata for the dataset.
        #[prost(message, optional, tag = "7")]
        pub dataset_info: ::core::option::Option<EntityInfo>,
        /// Default encryption for tables in the dataset.
        #[prost(message, optional, tag = "8")]
        pub default_encryption: ::core::option::Option<EncryptionInfo>,
    }
    /// Table deletion event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableDeletion {
        /// Describes how the table was deleted.
        #[prost(enumeration = "table_deletion::Reason", tag = "1")]
        pub reason: i32,
        /// The URI of the job that deleted a table.
        /// Present if the reason is QUERY.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "2")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `TableDeletion`.
    pub mod table_deletion {
        /// Describes how the table was deleted.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Table was deleted using the tables.delete API.
            TableDeleteRequest = 2,
            /// Table expired.
            Expired = 3,
            /// Table deleted using a DDL query.
            Query = 4,
        }
    }
    /// Model deletion event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelDeletion {
        /// Describes how the model was deleted.
        #[prost(enumeration = "model_deletion::Reason", tag = "1")]
        pub reason: i32,
        /// The URI of the job that deleted a model.
        /// Present if the reason is QUERY.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "2")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `ModelDeletion`.
    pub mod model_deletion {
        /// Describes how the model was deleted.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Model was deleted using the models.delete API.
            ModelDeleteRequest = 1,
            /// Model expired.
            Expired = 2,
            /// Model was deleted using DDL query.
            Query = 3,
        }
    }
    /// Trained BigQuery ML model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Model {
        /// Model URI.
        ///
        /// Format: `projects/<project_id>/datasets/<dataset_id>/models/<model_id>`.
        #[prost(string, tag = "1")]
        pub model_name: ::prost::alloc::string::String,
        /// User-provided metadata for the model.
        #[prost(message, optional, tag = "2")]
        pub model_info: ::core::option::Option<EntityInfo>,
        /// Model expiration time.
        #[prost(message, optional, tag = "5")]
        pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Model creation time.
        #[prost(message, optional, tag = "6")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Model last update time.
        #[prost(message, optional, tag = "7")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Model encryption information. Set when non-default encryption is used.
        #[prost(message, optional, tag = "8")]
        pub encryption: ::core::option::Option<EncryptionInfo>,
    }
    /// Routine deletion event.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoutineDeletion {
        /// Deleted routine.
        #[prost(message, optional, tag = "1")]
        pub routine: ::core::option::Option<Routine>,
        /// Describes how the routine was deleted.
        #[prost(enumeration = "routine_deletion::Reason", tag = "3")]
        pub reason: i32,
        /// The URI of the job that deleted the routine.
        /// Present if the reason is QUERY.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "4")]
        pub job_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `RoutineDeletion`.
    pub mod routine_deletion {
        /// Describes how the routine was deleted.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Reason {
            /// Unknown.
            Unspecified = 0,
            /// Routine was deleted using DDL query.
            Query = 1,
            /// Routine was deleted using the API.
            RoutineDeleteRequest = 2,
        }
    }
    /// BigQuery job.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Job {
        /// Job URI.
        ///
        /// Format: `projects/<project_id>/jobs/<job_id>`.
        #[prost(string, tag = "1")]
        pub job_name: ::prost::alloc::string::String,
        /// Job configuration.
        #[prost(message, optional, tag = "2")]
        pub job_config: ::core::option::Option<JobConfig>,
        /// Job status.
        #[prost(message, optional, tag = "3")]
        pub job_status: ::core::option::Option<JobStatus>,
        /// Job statistics.
        #[prost(message, optional, tag = "4")]
        pub job_stats: ::core::option::Option<JobStats>,
    }
    /// Job configuration.
    /// See the \[Jobs\](<https://cloud.google.com/bigquery/docs/reference/v2/jobs>)
    /// API resource for more details on individual fields.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct JobConfig {
        /// Job type.
        #[prost(enumeration = "job_config::Type", tag = "1")]
        pub r#type: i32,
        /// Labels provided for the job.
        #[prost(map = "string, string", tag = "6")]
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// Job configuration information.
        #[prost(oneof = "job_config::Config", tags = "2, 3, 4, 5")]
        pub config: ::core::option::Option<job_config::Config>,
    }
    /// Nested message and enum types in `JobConfig`.
    pub mod job_config {
        /// Query job configuration.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Query {
            /// The SQL query to run. Truncated if exceeds 50K.
            #[prost(string, tag = "1")]
            pub query: ::prost::alloc::string::String,
            /// True if the query field was truncated.
            #[prost(bool, tag = "10")]
            pub query_truncated: bool,
            /// The destination table for the query results.
            #[prost(string, tag = "2")]
            pub destination_table: ::prost::alloc::string::String,
            /// Destination table create disposition.
            #[prost(enumeration = "super::CreateDisposition", tag = "3")]
            pub create_disposition: i32,
            /// Destination table write disposition.
            #[prost(enumeration = "super::WriteDisposition", tag = "4")]
            pub write_disposition: i32,
            /// Default dataset for the query.
            #[prost(string, tag = "5")]
            pub default_dataset: ::prost::alloc::string::String,
            /// External data sources used in the query.
            #[prost(message, repeated, tag = "6")]
            pub table_definitions: ::prost::alloc::vec::Vec<super::TableDefinition>,
            /// Priority given to the query.
            #[prost(enumeration = "query::Priority", tag = "7")]
            pub priority: i32,
            /// Result table encryption information. Set when non-default encryption is
            /// used.
            #[prost(message, optional, tag = "8")]
            pub destination_table_encryption: ::core::option::Option<super::EncryptionInfo>,
            /// Type of the query.
            #[prost(enumeration = "super::QueryStatementType", tag = "9")]
            pub statement_type: i32,
        }
        /// Nested message and enum types in `Query`.
        pub mod query {
            /// Priority given to the query.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum Priority {
                /// Unknown.
                Unspecified = 0,
                /// Interactive query.
                QueryInteractive = 1,
                /// Batch query.
                QueryBatch = 2,
            }
        }
        /// Load job configuration.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Load {
            /// URIs for the data to be imported. Entire list is truncated if exceeds
            /// 40K.
            #[prost(string, repeated, tag = "1")]
            pub source_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// True if the source_URIs field was truncated.
            #[prost(bool, tag = "7")]
            pub source_uris_truncated: bool,
            /// The table schema in JSON format. Entire field is truncated if exceeds
            /// 40K.
            #[prost(string, tag = "2")]
            pub schema_json: ::prost::alloc::string::String,
            /// True if the schema_json field was truncated.
            #[prost(bool, tag = "8")]
            pub schema_json_truncated: bool,
            /// The destination table for the import.
            #[prost(string, tag = "3")]
            pub destination_table: ::prost::alloc::string::String,
            /// Destination table create disposition.
            #[prost(enumeration = "super::CreateDisposition", tag = "4")]
            pub create_disposition: i32,
            /// Destination table write disposition.
            #[prost(enumeration = "super::WriteDisposition", tag = "5")]
            pub write_disposition: i32,
            /// Result table encryption information. Set when non-default encryption is
            /// used.
            #[prost(message, optional, tag = "6")]
            pub destination_table_encryption: ::core::option::Option<super::EncryptionInfo>,
        }
        /// Extract job configuration.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Extract {
            /// URIs where extracted data should be written. Entire list is truncated
            /// if exceeds 50K.
            #[prost(string, repeated, tag = "1")]
            pub destination_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// True if the destination_URIs field was truncated.
            #[prost(bool, tag = "3")]
            pub destination_uris_truncated: bool,
            #[prost(oneof = "extract::Source", tags = "2, 4")]
            pub source: ::core::option::Option<extract::Source>,
        }
        /// Nested message and enum types in `Extract`.
        pub mod extract {
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Source {
                /// The source table.
                #[prost(string, tag = "2")]
                SourceTable(::prost::alloc::string::String),
                /// The source model.
                #[prost(string, tag = "4")]
                SourceModel(::prost::alloc::string::String),
            }
        }
        /// Table copy job configuration.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TableCopy {
            /// Source tables. Entire list is truncated if exceeds 50K.
            #[prost(string, repeated, tag = "1")]
            pub source_tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// True if the source_tables field was truncated.
            #[prost(bool, tag = "6")]
            pub source_tables_truncated: bool,
            /// Destination table.
            #[prost(string, tag = "2")]
            pub destination_table: ::prost::alloc::string::String,
            /// Destination table create disposition.
            #[prost(enumeration = "super::CreateDisposition", tag = "3")]
            pub create_disposition: i32,
            /// Destination table write disposition.
            #[prost(enumeration = "super::WriteDisposition", tag = "4")]
            pub write_disposition: i32,
            /// Result table encryption information. Set when non-default encryption is
            /// used.
            #[prost(message, optional, tag = "5")]
            pub destination_table_encryption: ::core::option::Option<super::EncryptionInfo>,
            /// Supported operation types in the table copy job.
            #[prost(enumeration = "super::OperationType", tag = "7")]
            pub operation_type: i32,
            /// Expiration time set on the destination table. Expired tables will be
            /// deleted and their storage reclaimed.
            #[prost(message, optional, tag = "8")]
            pub destination_expiration_time: ::core::option::Option<::prost_types::Timestamp>,
        }
        /// Job type.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Type {
            /// Unknown.
            Unspecified = 0,
            /// Query job.
            Query = 1,
            /// Table copy job.
            Copy = 2,
            /// Export (extract) job.
            Export = 3,
            /// Import (load) job.
            Import = 4,
        }
        /// Job configuration information.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Config {
            /// Query job information.
            #[prost(message, tag = "2")]
            QueryConfig(Query),
            /// Load job information.
            #[prost(message, tag = "3")]
            LoadConfig(Load),
            /// Extract job information.
            #[prost(message, tag = "4")]
            ExtractConfig(Extract),
            /// TableCopy job information.
            #[prost(message, tag = "5")]
            TableCopyConfig(TableCopy),
        }
    }
    /// Definition of an external data source used in a query.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableDefinition {
        /// Name of the table, used in queries.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// URIs for the data.
        #[prost(string, repeated, tag = "2")]
        pub source_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Status of a job.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct JobStatus {
        /// State of the job.
        #[prost(enumeration = "JobState", tag = "1")]
        pub job_state: i32,
        /// Job error, if the job failed.
        #[prost(message, optional, tag = "2")]
        pub error_result: ::core::option::Option<super::super::super::rpc::Status>,
        /// Errors encountered during the running of the job. Does not necessarily
        /// mean that the job has completed or was unsuccessful.
        #[prost(message, repeated, tag = "3")]
        pub errors: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    }
    /// Job statistics.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct JobStats {
        /// Job creation time.
        #[prost(message, optional, tag = "1")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Job execution start time.
        #[prost(message, optional, tag = "2")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Job completion time.
        #[prost(message, optional, tag = "3")]
        pub end_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The total number of slot-ms consumed by the query job.
        #[prost(int64, tag = "10")]
        pub total_slot_ms: i64,
        /// Reservation usage attributed from each tier of a reservation hierarchy.
        #[prost(message, repeated, tag = "11")]
        pub reservation_usage: ::prost::alloc::vec::Vec<job_stats::ReservationResourceUsage>,
        /// Parent job name. Only present for child jobs.
        #[prost(string, tag = "12")]
        pub parent_job_name: ::prost::alloc::string::String,
        /// Statistics specific to the job type.
        #[prost(oneof = "job_stats::Extended", tags = "8, 9, 13")]
        pub extended: ::core::option::Option<job_stats::Extended>,
    }
    /// Nested message and enum types in `JobStats`.
    pub mod job_stats {
        /// Query job statistics.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Query {
            /// Total bytes processed by the query job.
            #[prost(int64, tag = "1")]
            pub total_processed_bytes: i64,
            /// Total bytes billed by the query job.
            #[prost(int64, tag = "2")]
            pub total_billed_bytes: i64,
            /// The tier assigned by the CPU-based billing.
            #[prost(int32, tag = "3")]
            pub billing_tier: i32,
            /// Tables accessed by the query job.
            #[prost(string, repeated, tag = "6")]
            pub referenced_tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Views accessed by the query job.
            #[prost(string, repeated, tag = "7")]
            pub referenced_views: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Routines accessed by the query job.
            #[prost(string, repeated, tag = "10")]
            pub referenced_routines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Number of output rows produced by the query job.
            #[prost(int64, tag = "8")]
            pub output_row_count: i64,
            /// True if the query job results were read from the query cache.
            #[prost(bool, tag = "9")]
            pub cache_hit: bool,
        }
        /// Load job statistics.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Load {
            /// Total bytes loaded by the import job.
            #[prost(int64, tag = "1")]
            pub total_output_bytes: i64,
        }
        /// Extract job statistics.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Extract {
            /// Total bytes exported by the extract job.
            #[prost(int64, tag = "1")]
            pub total_input_bytes: i64,
        }
        /// Job resource usage breakdown by reservation.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ReservationResourceUsage {
            /// Reservation name or "unreserved" for on-demand resources usage.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// Total slot milliseconds used by the reservation for a particular job.
            #[prost(int64, tag = "2")]
            pub slot_ms: i64,
        }
        /// Statistics specific to the job type.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Extended {
            /// Query job statistics.
            #[prost(message, tag = "8")]
            QueryStats(Query),
            /// Load job statistics.
            #[prost(message, tag = "9")]
            LoadStats(Load),
            /// Extract job statistics.
            #[prost(message, tag = "13")]
            ExtractStats(Extract),
        }
    }
    /// BigQuery table.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Table {
        /// Table URI.
        ///
        /// Format: `projects/<project_id>/datasets/<dataset_id>/tables/<table_id>`.
        #[prost(string, tag = "1")]
        pub table_name: ::prost::alloc::string::String,
        /// A JSON representation of the table's schema. Entire field is truncated
        /// if exceeds 40K.
        #[prost(string, tag = "3")]
        pub schema_json: ::prost::alloc::string::String,
        /// True if the schema_json field was truncated.
        #[prost(bool, tag = "11")]
        pub schema_json_truncated: bool,
        /// View metadata. Only present for views.
        #[prost(message, optional, tag = "4")]
        pub view: ::core::option::Option<TableViewDefinition>,
        /// Table expiration time.
        #[prost(message, optional, tag = "5")]
        pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The table creation time.
        #[prost(message, optional, tag = "6")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The last time metadata update time.
        #[prost(message, optional, tag = "7")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The last table truncation time.
        #[prost(message, optional, tag = "8")]
        pub truncate_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Table encryption information. Set when non-default encryption is used.
        #[prost(message, optional, tag = "9")]
        pub encryption: ::core::option::Option<EncryptionInfo>,
        /// User-provided metadata for the table.
        #[prost(message, optional, tag = "10")]
        pub table_info: ::core::option::Option<EntityInfo>,
    }
    /// User Defined Function (UDF) or Stored Procedure.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Routine {
        /// Routine URI.
        ///
        /// Format:
        /// `projects/<project_id>/datasets/<dataset_id>/routines/<routine_id>`.
        #[prost(string, tag = "1")]
        pub routine_name: ::prost::alloc::string::String,
        /// Routine creation time.
        #[prost(message, optional, tag = "5")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Routine last update time.
        #[prost(message, optional, tag = "6")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// User-provided metadata for an entity, for e.g. dataset, table or model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EntityInfo {
        /// A short name for the entity.
        #[prost(string, tag = "1")]
        pub friendly_name: ::prost::alloc::string::String,
        /// A long description for the entity.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// Labels provided for the entity.
        #[prost(map = "string, string", tag = "3")]
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
    /// View definition.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableViewDefinition {
        /// SQL query defining the view. Truncated if exceeds 40K.
        #[prost(string, tag = "1")]
        pub query: ::prost::alloc::string::String,
        /// True if the schema_json field was truncated.
        #[prost(bool, tag = "2")]
        pub query_truncated: bool,
    }
    /// An access control list.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryAcl {
        /// IAM policy for the resource.
        #[prost(message, optional, tag = "1")]
        pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
        /// List of authorized views for a dataset.
        ///
        /// Format: `projects/<project_id>/datasets/<dataset_id>/tables/<view_id>`.
        #[prost(string, repeated, tag = "2")]
        pub authorized_views: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Encryption properties for a table or a job
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EncryptionInfo {
        /// Cloud kms key identifier.
        ///
        /// Format:
        /// `projects/<project_id>/locations/<location>/keyRings/<key_ring_name>/cryptoKeys/<key_name>`
        #[prost(string, tag = "1")]
        pub kms_key_name: ::prost::alloc::string::String,
    }
    /// Describes whether a job should overwrite or append the existing destination
    /// table if it already exists.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WriteDisposition {
        /// Unknown.
        Unspecified = 0,
        /// This job should only be writing to empty tables.
        WriteEmpty = 1,
        /// This job will truncate the existing table data.
        WriteTruncate = 2,
        /// This job will append to the table.
        WriteAppend = 3,
    }
    /// Table copy job operation type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OperationType {
        /// Unspecified operation type.
        Unspecified = 0,
        /// The source and the destination table have the same table type.
        Copy = 1,
        /// The source table type is TABLE and
        /// the destination table type is SNAPSHOT.
        Snapshot = 2,
        /// The source table type is SNAPSHOT and
        /// the destination table type is TABLE.
        Restore = 3,
    }
    /// Describes whether a job should create a destination table if it doesn't
    /// exist.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CreateDisposition {
        /// Unknown.
        Unspecified = 0,
        /// This job should never create tables.
        CreateNever = 1,
        /// This job should create a table if it doesn't already exist.
        CreateIfNeeded = 2,
    }
    /// State of a job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JobState {
        /// State unknown.
        Unspecified = 0,
        /// Job is waiting for the resources.
        Pending = 1,
        /// Job is running.
        Running = 2,
        /// Job is done.
        Done = 3,
    }
    /// Type of the statement (e.g. SELECT, INSERT, CREATE_TABLE, CREATE_MODEL..)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QueryStatementType {
        /// Unknown.
        Unspecified = 0,
        /// SELECT ... FROM &lt;Table list&gt; ...
        Select = 1,
        /// ASSERT &lt;condition&gt; AS 'description'
        Assert = 23,
        /// INSERT INTO &lt;Table&gt; ....
        Insert = 2,
        /// UPDATE &lt;Table&gt; SET ...
        Update = 3,
        /// DELETE &lt;Table&gt; ...
        Delete = 4,
        /// MERGE INTO &lt;Table&gt; ....
        Merge = 5,
        /// CREATE TABLE &lt;Table&gt; &lt;column list&gt;
        CreateTable = 6,
        /// CREATE TABLE &lt;Table&gt; AS SELECT
        CreateTableAsSelect = 7,
        /// CREATE VIEW &lt;View&gt;
        CreateView = 8,
        /// CREATE MODEL &lt;Model&gt; AS &lt;Query&gt;
        CreateModel = 9,
        /// CREATE MATERIALIZED VIEW &lt;View&gt; AS ...
        CreateMaterializedView = 13,
        /// CREATE FUNCTION &lt;Function&gt;(&lt;Signature&gt;) AS ...
        CreateFunction = 14,
        /// CREATE PROCEDURE &lt;Procedure&gt;
        CreateProcedure = 20,
        /// CREATE SCHEMA &lt;Schema&gt;
        CreateSchema = 53,
        /// DROP TABLE &lt;Table&gt;
        DropTable = 10,
        /// DROP EXTERNAL TABLE &lt;Table&gt;
        DropExternalTable = 33,
        /// DROP VIEW &lt;View&gt;
        DropView = 11,
        /// DROP MODEL &lt;Model&gt;
        DropModel = 12,
        /// DROP MATERIALIZED VIEW &lt;View&gt;
        DropMaterializedView = 15,
        /// DROP FUNCTION &lt;Function&gt;
        DropFunction = 16,
        /// DROP PROCEDURE &lt;Procedure&gt;
        DropProcedure = 21,
        /// DROP SCHEMA &lt;Schema&gt;
        DropSchema = 54,
        /// ALTER TABLE &lt;Table&gt;
        AlterTable = 17,
        /// ALTER VIEW &lt;View&gt;
        AlterView = 18,
        /// ALTER MATERIALIZED_VIEW &lt;view&gt;
        AlterMaterializedView = 22,
        /// ALTER SCHEMA &lt;Schema&gt;
        AlterSchema = 55,
        /// Script
        Script = 19,
        /// TRUNCATE TABLE &lt;Table&gt
        TruncateTable = 26,
        /// CREATE EXTERNAL TABLE &lt;TABLE&gt;
        CreateExternalTable = 27,
        /// EXPORT DATA;
        ExportData = 28,
        /// CALL &lt;stored procedure&gt;
        Call = 29,
    }
    /// BigQuery event information.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// Job insertion event.
        #[prost(message, tag = "1")]
        JobInsertion(JobInsertion),
        /// Job state change event.
        #[prost(message, tag = "2")]
        JobChange(JobChange),
        /// Dataset creation event.
        #[prost(message, tag = "3")]
        DatasetCreation(DatasetCreation),
        /// Dataset change event.
        #[prost(message, tag = "4")]
        DatasetChange(DatasetChange),
        /// Dataset deletion event.
        #[prost(message, tag = "5")]
        DatasetDeletion(DatasetDeletion),
        /// Table creation event.
        #[prost(message, tag = "6")]
        TableCreation(TableCreation),
        /// Table metadata change event.
        #[prost(message, tag = "8")]
        TableChange(TableChange),
        /// Table deletion event.
        #[prost(message, tag = "9")]
        TableDeletion(TableDeletion),
        /// Table data read event.
        #[prost(message, tag = "10")]
        TableDataRead(TableDataRead),
        /// Table data change event.
        #[prost(message, tag = "11")]
        TableDataChange(TableDataChange),
        /// Model deletion event.
        #[prost(message, tag = "12")]
        ModelDeletion(ModelDeletion),
        /// Model creation event.
        #[prost(message, tag = "13")]
        ModelCreation(ModelCreation),
        /// Model metadata change event.
        #[prost(message, tag = "14")]
        ModelMetadataChange(ModelMetadataChange),
        /// Model data change event.
        #[prost(message, tag = "15")]
        ModelDataChange(ModelDataChange),
        /// Model data read event.
        #[prost(message, tag = "19")]
        ModelDataRead(ModelDataRead),
        /// Routine creation event.
        #[prost(message, tag = "16")]
        RoutineCreation(RoutineCreation),
        /// Routine change event.
        #[prost(message, tag = "17")]
        RoutineChange(RoutineChange),
        /// Routine deletion event.
        #[prost(message, tag = "18")]
        RoutineDeletion(RoutineDeletion),
    }
}
