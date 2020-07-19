#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaInfo {
    /// The location of the serving resources, e.g. "us-central1".
    #[prost(string, tag = "1")]
    pub location: std::string::String,
    /// The type of replica.
    #[prost(enumeration = "replica_info::ReplicaType", tag = "2")]
    pub r#type: i32,
    /// If true, this location is designated as the default leader location where
    /// leader replicas are placed. See the [region types
    /// documentation](https://cloud.google.com/spanner/docs/instances#region_types)
    /// for more details.
    #[prost(bool, tag = "3")]
    pub default_leader_location: bool,
}
pub mod replica_info {
    /// Indicates the type of replica.  See the [replica types
    /// documentation](https://cloud.google.com/spanner/docs/replication#replica_types)
    /// for more details.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReplicaType {
        /// Not specified.
        TypeUnspecified = 0,
        /// Read-write replicas support both reads and writes. These replicas:
        ///
        /// * Maintain a full copy of your data.
        /// * Serve reads.
        /// * Can vote whether to commit a write.
        /// * Participate in leadership election.
        /// * Are eligible to become a leader.
        ReadWrite = 1,
        /// Read-only replicas only support reads (not writes). Read-only replicas:
        ///
        /// * Maintain a full copy of your data.
        /// * Serve reads.
        /// * Do not participate in voting to commit writes.
        /// * Are not eligible to become a leader.
        ReadOnly = 2,
        /// Witness replicas don't support reads but do participate in voting to
        /// commit writes. Witness replicas:
        ///
        /// * Do not maintain a full copy of data.
        /// * Do not serve reads.
        /// * Vote whether to commit writes.
        /// * Participate in leader election but are not eligible to become leader.
        Witness = 3,
    }
}
/// A possible configuration for a Cloud Spanner instance. Configurations
/// define the geographic placement of nodes and their replication.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceConfig {
    /// A unique identifier for the instance configuration.  Values
    /// are of the form
    /// `projects/<project>/instanceConfigs/[a-z][-a-z0-9]*`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The name of this instance configuration as it appears in UIs.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// The geographic placement of nodes in this instance configuration and their
    /// replication properties.
    #[prost(message, repeated, tag = "3")]
    pub replicas: ::std::vec::Vec<ReplicaInfo>,
}
/// An isolated set of Cloud Spanner resources on which databases can be hosted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Required. A unique identifier for the instance, which cannot be changed
    /// after the instance is created. Values are of the form
    /// `projects/<project>/instances/[a-z][-a-z0-9]*[a-z0-9]`. The final
    /// segment of the name must be between 2 and 64 characters in length.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The name of the instance's configuration. Values are of the form
    /// `projects/<project>/instanceConfigs/<configuration>`. See
    /// also [InstanceConfig][google.spanner.admin.instance.v1.InstanceConfig] and
    /// [ListInstanceConfigs][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigs].
    #[prost(string, tag = "2")]
    pub config: std::string::String,
    /// Required. The descriptive name for this instance as it appears in UIs.
    /// Must be unique per project and between 4 and 30 characters in length.
    #[prost(string, tag = "3")]
    pub display_name: std::string::String,
    /// Required. The number of nodes allocated to this instance. This may be zero
    /// in API responses for instances that are not yet in state `READY`.
    ///
    /// See [the
    /// documentation](https://cloud.google.com/spanner/docs/instances#node_count)
    /// for more information about nodes.
    #[prost(int32, tag = "5")]
    pub node_count: i32,
    /// Output only. The current instance state. For
    /// [CreateInstance][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstance], the state must be
    /// either omitted or set to `CREATING`. For
    /// [UpdateInstance][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstance], the state must be
    /// either omitted or set to `READY`.
    #[prost(enumeration = "instance::State", tag = "6")]
    pub state: i32,
    /// Cloud Labels are a flexible and lightweight mechanism for organizing cloud
    /// resources into groups that reflect a customer's organizational needs and
    /// deployment strategies. Cloud Labels can be used to filter collections of
    /// resources. They can be used to control how resource metrics are aggregated.
    /// And they can be used as arguments to policy management rules (e.g. route,
    /// firewall, load balancing, etc.).
    ///
    ///  * Label keys must be between 1 and 63 characters long and must conform to
    ///    the following regular expression: `[a-z]([-a-z0-9]*[a-z0-9])?`.
    ///  * Label values must be between 0 and 63 characters long and must conform
    ///    to the regular expression `([a-z]([-a-z0-9]*[a-z0-9])?)?`.
    ///  * No more than 64 labels can be associated with a given resource.
    ///
    /// See https://goo.gl/xmQnxf for more information on and examples of labels.
    ///
    /// If you plan to use labels in your own code, please note that additional
    /// characters may be allowed in the future. And so you are advised to use an
    /// internal label representation, such as JSON, which doesn't rely upon
    /// specific characters being disallowed.  For example, representing labels
    /// as the string:  name + "_" + value  would prove problematic if we were to
    /// allow "_" in a future release.
    #[prost(map = "string, string", tag = "7")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Deprecated. This field is not populated.
    #[prost(string, repeated, tag = "8")]
    pub endpoint_uris: ::std::vec::Vec<std::string::String>,
}
pub mod instance {
    /// Indicates the current state of the instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified.
        Unspecified = 0,
        /// The instance is still being created. Resources may not be
        /// available yet, and operations such as database creation may not
        /// work.
        Creating = 1,
        /// The instance is fully created and ready to do work such as
        /// creating databases.
        Ready = 2,
    }
}
/// The request for [ListInstanceConfigs][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceConfigsRequest {
    /// Required. The name of the project for which a list of supported instance
    /// configurations is requested. Values are of the form
    /// `projects/<project>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Number of instance configurations to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// [next_page_token][google.spanner.admin.instance.v1.ListInstanceConfigsResponse.next_page_token]
    /// from a previous [ListInstanceConfigsResponse][google.spanner.admin.instance.v1.ListInstanceConfigsResponse].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response for [ListInstanceConfigs][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstanceConfigsResponse {
    /// The list of requested instance configurations.
    #[prost(message, repeated, tag = "1")]
    pub instance_configs: ::std::vec::Vec<InstanceConfig>,
    /// `next_page_token` can be sent in a subsequent
    /// [ListInstanceConfigs][google.spanner.admin.instance.v1.InstanceAdmin.ListInstanceConfigs] call to
    /// fetch more of the matching instance configurations.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for
/// [GetInstanceConfigRequest][google.spanner.admin.instance.v1.InstanceAdmin.GetInstanceConfig].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceConfigRequest {
    /// Required. The name of the requested instance configuration. Values are of
    /// the form `projects/<project>/instanceConfigs/<config>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for [GetInstance][google.spanner.admin.instance.v1.InstanceAdmin.GetInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. The name of the requested instance. Values are of the form
    /// `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// If field_mask is present, specifies the subset of [Instance][google.spanner.admin.instance.v1.Instance] fields that
    /// should be returned.
    /// If absent, all [Instance][google.spanner.admin.instance.v1.Instance] fields are returned.
    #[prost(message, optional, tag = "2")]
    pub field_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request for [CreateInstance][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The name of the project in which to create the instance. Values
    /// are of the form `projects/<project>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID of the instance to create.  Valid identifiers are of the
    /// form `[a-z][-a-z0-9]*[a-z0-9]` and must be between 2 and 64 characters in
    /// length.
    #[prost(string, tag = "2")]
    pub instance_id: std::string::String,
    /// Required. The instance to create.  The name may be omitted, but if
    /// specified must be `<parent>/instances/<instance_id>`.
    #[prost(message, optional, tag = "3")]
    pub instance: ::std::option::Option<Instance>,
}
/// The request for [ListInstances][google.spanner.admin.instance.v1.InstanceAdmin.ListInstances].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The name of the project for which a list of instances is
    /// requested. Values are of the form `projects/<project>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Number of instances to be returned in the response. If 0 or less, defaults
    /// to the server's maximum allowed page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// [next_page_token][google.spanner.admin.instance.v1.ListInstancesResponse.next_page_token] from a
    /// previous [ListInstancesResponse][google.spanner.admin.instance.v1.ListInstancesResponse].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// An expression for filtering the results of the request. Filter rules are
    /// case insensitive. The fields eligible for filtering are:
    ///
    ///   * `name`
    ///   * `display_name`
    ///   * `labels.key` where key is the name of a label
    ///
    /// Some examples of using filters are:
    ///
    ///   * `name:*` --> The instance has a name.
    ///   * `name:Howl` --> The instance's name contains the string "howl".
    ///   * `name:HOWL` --> Equivalent to above.
    ///   * `NAME:howl` --> Equivalent to above.
    ///   * `labels.env:*` --> The instance has the label "env".
    ///   * `labels.env:dev` --> The instance has the label "env" and the value of
    ///                        the label contains the string "dev".
    ///   * `name:howl labels.env:dev` --> The instance's name contains "howl" and
    ///                                  it has the label "env" with its value
    ///                                  containing "dev".
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// The response for [ListInstances][google.spanner.admin.instance.v1.InstanceAdmin.ListInstances].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The list of requested instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::std::vec::Vec<Instance>,
    /// `next_page_token` can be sent in a subsequent
    /// [ListInstances][google.spanner.admin.instance.v1.InstanceAdmin.ListInstances] call to fetch more
    /// of the matching instances.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for [UpdateInstance][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. The instance to update, which must always include the instance
    /// name.  Otherwise, only fields mentioned in [field_mask][google.spanner.admin.instance.v1.UpdateInstanceRequest.field_mask] need be included.
    #[prost(message, optional, tag = "1")]
    pub instance: ::std::option::Option<Instance>,
    /// Required. A mask specifying which fields in [Instance][google.spanner.admin.instance.v1.Instance] should be updated.
    /// The field mask must always be specified; this prevents any future fields in
    /// [Instance][google.spanner.admin.instance.v1.Instance] from being erased accidentally by clients that do not know
    /// about them.
    #[prost(message, optional, tag = "2")]
    pub field_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request for [DeleteInstance][google.spanner.admin.instance.v1.InstanceAdmin.DeleteInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. The name of the instance to be deleted. Values are of the form
    /// `projects/<project>/instances/<instance>`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Metadata type for the operation returned by
/// [CreateInstance][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceMetadata {
    /// The instance being created.
    #[prost(message, optional, tag = "1")]
    pub instance: ::std::option::Option<Instance>,
    /// The time at which the
    /// [CreateInstance][google.spanner.admin.instance.v1.InstanceAdmin.CreateInstance] request was
    /// received.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation was cancelled. If set, this operation is
    /// in the process of undoing itself (which is guaranteed to succeed) and
    /// cannot be cancelled again.
    #[prost(message, optional, tag = "3")]
    pub cancel_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation failed or was completed successfully.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Metadata type for the operation returned by
/// [UpdateInstance][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceMetadata {
    /// The desired end state of the update.
    #[prost(message, optional, tag = "1")]
    pub instance: ::std::option::Option<Instance>,
    /// The time at which [UpdateInstance][google.spanner.admin.instance.v1.InstanceAdmin.UpdateInstance]
    /// request was received.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation was cancelled. If set, this operation is
    /// in the process of undoing itself (which is guaranteed to succeed) and
    /// cannot be cancelled again.
    #[prost(message, optional, tag = "3")]
    pub cancel_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation failed or was completed successfully.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod instance_admin_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Cloud Spanner Instance Admin API"]
    #[doc = ""]
    #[doc = " The Cloud Spanner Instance Admin API can be used to create, delete,"]
    #[doc = " modify and list instances. Instances are dedicated Cloud Spanner serving"]
    #[doc = " and storage resources to be used by Cloud Spanner databases."]
    #[doc = ""]
    #[doc = " Each instance has a \"configuration\", which dictates where the"]
    #[doc = " serving resources for the Cloud Spanner instance are located (e.g.,"]
    #[doc = " US-central, Europe). Configurations are created by Google based on"]
    #[doc = " resource availability."]
    #[doc = ""]
    #[doc = " Cloud Spanner billing is based on the instances that exist and their"]
    #[doc = " sizes. After an instance exists, there are no additional"]
    #[doc = " per-database or per-operation charges for use of the instance"]
    #[doc = " (though there may be additional network bandwidth charges)."]
    #[doc = " Instances offer isolation: problems with databases in one instance"]
    #[doc = " will not affect other instances. However, within an instance"]
    #[doc = " databases can affect each other. For example, if one database in an"]
    #[doc = " instance receives a lot of requests and consumes most of the"]
    #[doc = " instance resources, fewer resources are available for other"]
    #[doc = " databases in that instance, and their performance may suffer."]
    pub struct InstanceAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> InstanceAdminClient<T>
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
        #[doc = " Lists the supported instance configurations for a given project."]
        pub async fn list_instance_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstanceConfigsRequest>,
        ) -> Result<tonic::Response<super::ListInstanceConfigsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/ListInstanceConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a particular instance configuration."]
        pub async fn get_instance_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceConfigRequest>,
        ) -> Result<tonic::Response<super::InstanceConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/GetInstanceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all instances in the given project."]
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a particular instance."]
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
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an instance and begins preparing it to begin serving. The"]
        #[doc = " returned [long-running operation][google.longrunning.Operation]"]
        #[doc = " can be used to track the progress of preparing the new"]
        #[doc = " instance. The instance name is assigned by the caller. If the"]
        #[doc = " named instance already exists, `CreateInstance` returns"]
        #[doc = " `ALREADY_EXISTS`."]
        #[doc = ""]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = ""]
        #[doc = "   * The instance is readable via the API, with all requested attributes"]
        #[doc = "     but no allocated resources. Its state is `CREATING`."]
        #[doc = ""]
        #[doc = " Until completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Cancelling the operation renders the instance immediately unreadable"]
        #[doc = "     via the API."]
        #[doc = "   * The instance can be deleted."]
        #[doc = "   * All other attempts to modify the instance are rejected."]
        #[doc = ""]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Billing for all successfully-allocated resources begins (some types"]
        #[doc = "     may have lower than the requested levels)."]
        #[doc = "   * Databases can be created in the instance."]
        #[doc = "   * The instance's allocated resource levels are readable via the API."]
        #[doc = "   * The instance's state becomes `READY`."]
        #[doc = ""]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format `<instance_name>/operations/<operation_id>` and"]
        #[doc = " can be used to track creation of the instance.  The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateInstanceMetadata][google.spanner.admin.instance.v1.CreateInstanceMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Instance][google.spanner.admin.instance.v1.Instance], if successful."]
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an instance, and begins allocating or releasing resources"]
        #[doc = " as requested. The returned [long-running"]
        #[doc = " operation][google.longrunning.Operation] can be used to track the"]
        #[doc = " progress of updating the instance. If the named instance does not"]
        #[doc = " exist, returns `NOT_FOUND`."]
        #[doc = ""]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = ""]
        #[doc = "   * For resource types for which a decrease in the instance's allocation"]
        #[doc = "     has been requested, billing is based on the newly-requested level."]
        #[doc = ""]
        #[doc = " Until completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Cancelling the operation sets its metadata's"]
        #[doc = "     [cancel_time][google.spanner.admin.instance.v1.UpdateInstanceMetadata.cancel_time], and begins"]
        #[doc = "     restoring resources to their pre-request values. The operation"]
        #[doc = "     is guaranteed to succeed at undoing all resource changes,"]
        #[doc = "     after which point it terminates with a `CANCELLED` status."]
        #[doc = "   * All other attempts to modify the instance are rejected."]
        #[doc = "   * Reading the instance via the API continues to give the pre-request"]
        #[doc = "     resource levels."]
        #[doc = ""]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Billing begins for all successfully-allocated resources (some types"]
        #[doc = "     may have lower than the requested levels)."]
        #[doc = "   * All newly-reserved resources are available for serving the instance's"]
        #[doc = "     tables."]
        #[doc = "   * The instance's new resource levels are readable via the API."]
        #[doc = ""]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format `<instance_name>/operations/<operation_id>` and"]
        #[doc = " can be used to track the instance modification.  The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [UpdateInstanceMetadata][google.spanner.admin.instance.v1.UpdateInstanceMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Instance][google.spanner.admin.instance.v1.Instance], if successful."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instances.update` permission on"]
        #[doc = " resource [name][google.spanner.admin.instance.v1.Instance.name]."]
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/UpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an instance."]
        #[doc = ""]
        #[doc = " Immediately upon completion of the request:"]
        #[doc = ""]
        #[doc = "   * Billing ceases for all of the instance's reserved resources."]
        #[doc = ""]
        #[doc = " Soon afterward:"]
        #[doc = ""]
        #[doc = "   * The instance and *all of its databases* immediately and"]
        #[doc = "     irrevocably disappear from the API. All data in the databases"]
        #[doc = "     is permanently deleted."]
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.spanner.admin.instance.v1.InstanceAdmin/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on an instance resource. Replaces any"]
        #[doc = " existing policy."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instances.setIamPolicy` on"]
        #[doc = " [resource][google.iam.v1.SetIamPolicyRequest.resource]."]
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for an instance resource. Returns an empty"]
        #[doc = " policy if an instance exists but does not have a policy set."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instances.getIamPolicy` on"]
        #[doc = " [resource][google.iam.v1.GetIamPolicyRequest.resource]."]
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that the caller has on the specified instance resource."]
        #[doc = ""]
        #[doc = " Attempting this RPC on a non-existent Cloud Spanner instance resource will"]
        #[doc = " result in a NOT_FOUND error if the user has `spanner.instances.list`"]
        #[doc = " permission on the containing Google Cloud Project. Otherwise returns an"]
        #[doc = " empty set of permissions."]
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for InstanceAdminClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for InstanceAdminClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "InstanceAdminClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod instance_admin_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with InstanceAdminServer."]
    #[async_trait]
    pub trait InstanceAdmin: Send + Sync + 'static {
        #[doc = " Lists the supported instance configurations for a given project."]
        async fn list_instance_configs(
            &self,
            request: tonic::Request<super::ListInstanceConfigsRequest>,
        ) -> Result<tonic::Response<super::ListInstanceConfigsResponse>, tonic::Status>;
        #[doc = " Gets information about a particular instance configuration."]
        async fn get_instance_config(
            &self,
            request: tonic::Request<super::GetInstanceConfigRequest>,
        ) -> Result<tonic::Response<super::InstanceConfig>, tonic::Status>;
        #[doc = " Lists all instances in the given project."]
        async fn list_instances(
            &self,
            request: tonic::Request<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status>;
        #[doc = " Gets information about a particular instance."]
        async fn get_instance(
            &self,
            request: tonic::Request<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status>;
        #[doc = " Creates an instance and begins preparing it to begin serving. The"]
        #[doc = " returned [long-running operation][google.longrunning.Operation]"]
        #[doc = " can be used to track the progress of preparing the new"]
        #[doc = " instance. The instance name is assigned by the caller. If the"]
        #[doc = " named instance already exists, `CreateInstance` returns"]
        #[doc = " `ALREADY_EXISTS`."]
        #[doc = ""]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = ""]
        #[doc = "   * The instance is readable via the API, with all requested attributes"]
        #[doc = "     but no allocated resources. Its state is `CREATING`."]
        #[doc = ""]
        #[doc = " Until completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Cancelling the operation renders the instance immediately unreadable"]
        #[doc = "     via the API."]
        #[doc = "   * The instance can be deleted."]
        #[doc = "   * All other attempts to modify the instance are rejected."]
        #[doc = ""]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Billing for all successfully-allocated resources begins (some types"]
        #[doc = "     may have lower than the requested levels)."]
        #[doc = "   * Databases can be created in the instance."]
        #[doc = "   * The instance's allocated resource levels are readable via the API."]
        #[doc = "   * The instance's state becomes `READY`."]
        #[doc = ""]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format `<instance_name>/operations/<operation_id>` and"]
        #[doc = " can be used to track creation of the instance.  The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateInstanceMetadata][google.spanner.admin.instance.v1.CreateInstanceMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Instance][google.spanner.admin.instance.v1.Instance], if successful."]
        async fn create_instance(
            &self,
            request: tonic::Request<super::CreateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Updates an instance, and begins allocating or releasing resources"]
        #[doc = " as requested. The returned [long-running"]
        #[doc = " operation][google.longrunning.Operation] can be used to track the"]
        #[doc = " progress of updating the instance. If the named instance does not"]
        #[doc = " exist, returns `NOT_FOUND`."]
        #[doc = ""]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = ""]
        #[doc = "   * For resource types for which a decrease in the instance's allocation"]
        #[doc = "     has been requested, billing is based on the newly-requested level."]
        #[doc = ""]
        #[doc = " Until completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Cancelling the operation sets its metadata's"]
        #[doc = "     [cancel_time][google.spanner.admin.instance.v1.UpdateInstanceMetadata.cancel_time], and begins"]
        #[doc = "     restoring resources to their pre-request values. The operation"]
        #[doc = "     is guaranteed to succeed at undoing all resource changes,"]
        #[doc = "     after which point it terminates with a `CANCELLED` status."]
        #[doc = "   * All other attempts to modify the instance are rejected."]
        #[doc = "   * Reading the instance via the API continues to give the pre-request"]
        #[doc = "     resource levels."]
        #[doc = ""]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = ""]
        #[doc = "   * Billing begins for all successfully-allocated resources (some types"]
        #[doc = "     may have lower than the requested levels)."]
        #[doc = "   * All newly-reserved resources are available for serving the instance's"]
        #[doc = "     tables."]
        #[doc = "   * The instance's new resource levels are readable via the API."]
        #[doc = ""]
        #[doc = " The returned [long-running operation][google.longrunning.Operation] will"]
        #[doc = " have a name of the format `<instance_name>/operations/<operation_id>` and"]
        #[doc = " can be used to track the instance modification.  The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [UpdateInstanceMetadata][google.spanner.admin.instance.v1.UpdateInstanceMetadata]."]
        #[doc = " The [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Instance][google.spanner.admin.instance.v1.Instance], if successful."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instances.update` permission on"]
        #[doc = " resource [name][google.spanner.admin.instance.v1.Instance.name]."]
        async fn update_instance(
            &self,
            request: tonic::Request<super::UpdateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes an instance."]
        #[doc = ""]
        #[doc = " Immediately upon completion of the request:"]
        #[doc = ""]
        #[doc = "   * Billing ceases for all of the instance's reserved resources."]
        #[doc = ""]
        #[doc = " Soon afterward:"]
        #[doc = ""]
        #[doc = "   * The instance and *all of its databases* immediately and"]
        #[doc = "     irrevocably disappear from the API. All data in the databases"]
        #[doc = "     is permanently deleted."]
        async fn delete_instance(
            &self,
            request: tonic::Request<super::DeleteInstanceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Sets the access control policy on an instance resource. Replaces any"]
        #[doc = " existing policy."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instances.setIamPolicy` on"]
        #[doc = " [resource][google.iam.v1.SetIamPolicyRequest.resource]."]
        async fn set_iam_policy(
            &self,
            request: tonic::Request<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        >;
        #[doc = " Gets the access control policy for an instance resource. Returns an empty"]
        #[doc = " policy if an instance exists but does not have a policy set."]
        #[doc = ""]
        #[doc = " Authorization requires `spanner.instances.getIamPolicy` on"]
        #[doc = " [resource][google.iam.v1.GetIamPolicyRequest.resource]."]
        async fn get_iam_policy(
            &self,
            request: tonic::Request<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
            tonic::Status,
        >;
        #[doc = " Returns permissions that the caller has on the specified instance resource."]
        #[doc = ""]
        #[doc = " Attempting this RPC on a non-existent Cloud Spanner instance resource will"]
        #[doc = " result in a NOT_FOUND error if the user has `spanner.instances.list`"]
        #[doc = " permission on the containing Google Cloud Project. Otherwise returns an"]
        #[doc = " empty set of permissions."]
        async fn test_iam_permissions(
            &self,
            request: tonic::Request<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        >;
    }
    #[doc = " Cloud Spanner Instance Admin API"]
    #[doc = ""]
    #[doc = " The Cloud Spanner Instance Admin API can be used to create, delete,"]
    #[doc = " modify and list instances. Instances are dedicated Cloud Spanner serving"]
    #[doc = " and storage resources to be used by Cloud Spanner databases."]
    #[doc = ""]
    #[doc = " Each instance has a \"configuration\", which dictates where the"]
    #[doc = " serving resources for the Cloud Spanner instance are located (e.g.,"]
    #[doc = " US-central, Europe). Configurations are created by Google based on"]
    #[doc = " resource availability."]
    #[doc = ""]
    #[doc = " Cloud Spanner billing is based on the instances that exist and their"]
    #[doc = " sizes. After an instance exists, there are no additional"]
    #[doc = " per-database or per-operation charges for use of the instance"]
    #[doc = " (though there may be additional network bandwidth charges)."]
    #[doc = " Instances offer isolation: problems with databases in one instance"]
    #[doc = " will not affect other instances. However, within an instance"]
    #[doc = " databases can affect each other. For example, if one database in an"]
    #[doc = " instance receives a lot of requests and consumes most of the"]
    #[doc = " instance resources, fewer resources are available for other"]
    #[doc = " databases in that instance, and their performance may suffer."]
    #[derive(Debug)]
    pub struct InstanceAdminServer<T: InstanceAdmin> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: InstanceAdmin> InstanceAdminServer<T> {
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
    impl<T, B> Service<http::Request<B>> for InstanceAdminServer<T>
    where
        T: InstanceAdmin,
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/ListInstanceConfigs" => {
                    #[allow(non_camel_case_types)]
                    struct ListInstanceConfigsSvc<T: InstanceAdmin>(pub Arc<T>);
                    impl<T: InstanceAdmin>
                        tonic::server::UnaryService<super::ListInstanceConfigsRequest>
                        for ListInstanceConfigsSvc<T>
                    {
                        type Response = super::ListInstanceConfigsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInstanceConfigsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_instance_configs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListInstanceConfigsSvc(inner);
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/GetInstanceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetInstanceConfigSvc<T: InstanceAdmin>(pub Arc<T>);
                    impl<T: InstanceAdmin>
                        tonic::server::UnaryService<super::GetInstanceConfigRequest>
                        for GetInstanceConfigSvc<T>
                    {
                        type Response = super::InstanceConfig;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInstanceConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_instance_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetInstanceConfigSvc(inner);
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/ListInstances" => {
                    #[allow(non_camel_case_types)]
                    struct ListInstancesSvc<T: InstanceAdmin>(pub Arc<T>);
                    impl<T: InstanceAdmin> tonic::server::UnaryService<super::ListInstancesRequest>
                        for ListInstancesSvc<T>
                    {
                        type Response = super::ListInstancesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInstancesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_instances(request).await };
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/GetInstance" => {
                    #[allow(non_camel_case_types)]
                    struct GetInstanceSvc<T: InstanceAdmin>(pub Arc<T>);
                    impl<T: InstanceAdmin> tonic::server::UnaryService<super::GetInstanceRequest>
                        for GetInstanceSvc<T>
                    {
                        type Response = super::Instance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_instance(request).await };
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/CreateInstance" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInstanceSvc<T: InstanceAdmin>(pub Arc<T>);
                    impl<T: InstanceAdmin> tonic::server::UnaryService<super::CreateInstanceRequest>
                        for CreateInstanceSvc<T>
                    {
                        type Response = super::super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateInstanceSvc(inner);
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/UpdateInstance" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateInstanceSvc<T: InstanceAdmin>(pub Arc<T>);
                    impl<T: InstanceAdmin> tonic::server::UnaryService<super::UpdateInstanceRequest>
                        for UpdateInstanceSvc<T>
                    {
                        type Response = super::super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateInstanceSvc(inner);
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/DeleteInstance" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteInstanceSvc<T: InstanceAdmin>(pub Arc<T>);
                    impl<T: InstanceAdmin> tonic::server::UnaryService<super::DeleteInstanceRequest>
                        for DeleteInstanceSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_instance(request).await };
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/SetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetIamPolicySvc<T: InstanceAdmin>(pub Arc<T>);
                    impl<T: InstanceAdmin>
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
                            let fut = async move { (*inner).set_iam_policy(request).await };
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/GetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetIamPolicySvc<T: InstanceAdmin>(pub Arc<T>);
                    impl<T: InstanceAdmin>
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
                            let fut = async move { (*inner).get_iam_policy(request).await };
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
                "/google.spanner.admin.instance.v1.InstanceAdmin/TestIamPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct TestIamPermissionsSvc<T: InstanceAdmin>(pub Arc<T>);
                    impl<T: InstanceAdmin>
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
                            let fut = async move { (*inner).test_iam_permissions(request).await };
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
    impl<T: InstanceAdmin> Clone for InstanceAdminServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: InstanceAdmin> Clone for _Inner<T> {
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
