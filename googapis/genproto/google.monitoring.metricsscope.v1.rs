/// Represents a [Metrics
/// Scope](<https://cloud.google.com/monitoring/settings#concept-scope>) in Cloud
/// Monitoring, which specifies one or more Google projects and zero or more AWS
/// accounts to monitor together.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricsScope {
    /// Immutable. The resource name of the Monitoring Metrics Scope.
    /// On input, the resource name can be specified with the
    /// scoping project ID or number. On output, the resource name is
    /// specified with the scoping project number.
    /// Example:
    /// `locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when this `Metrics Scope` was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when this `Metrics Scope` record was last updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The list of projects monitored by this `Metrics Scope`.
    #[prost(message, repeated, tag = "4")]
    pub monitored_projects: ::prost::alloc::vec::Vec<MonitoredProject>,
}
/// A [project being
/// monitored](<https://cloud.google.com/monitoring/settings/multiple-projects#create-multi>)
/// by a `Metrics Scope`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoredProject {
    /// Immutable. The resource name of the `MonitoredProject`. On input, the resource name
    /// includes the scoping project ID and monitored project ID. On output, it
    /// contains the equivalent project numbers.
    /// Example:
    /// `locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}/projects/{MONITORED_PROJECT_ID_OR_NUMBER}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The time when this `MonitoredProject` was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request for the `GetMetricsScope` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricsScopeRequest {
    /// Required. The resource name of the `Metrics Scope`.
    /// Example:
    /// `locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the `ListMetricsScopesByMonitoredProject` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetricsScopesByMonitoredProjectRequest {
    /// Required. The resource name of the `Monitored Project` being requested.
    /// Example:
    /// `projects/{MONITORED_PROJECT_ID_OR_NUMBER}`
    #[prost(string, tag = "1")]
    pub monitored_resource_container: ::prost::alloc::string::String,
}
/// Response for the `ListMetricsScopesByMonitoredProject` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetricsScopesByMonitoredProjectResponse {
    /// A set of all metrics scopes that the specified monitored project has been
    /// added to.
    #[prost(message, repeated, tag = "1")]
    pub metrics_scopes: ::prost::alloc::vec::Vec<MetricsScope>,
}
/// Request for the `CreateMonitoredProject` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMonitoredProjectRequest {
    /// Required. The resource name of the existing `Metrics Scope` that will monitor this
    /// project.
    /// Example:
    /// `locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The initial `MonitoredProject` configuration.
    /// Specify only the `monitored_project.name` field. All other fields are
    /// ignored. The `monitored_project.name` must be in the format:
    /// `locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}/projects/{MONITORED_PROJECT_ID_OR_NUMBER}`
    #[prost(message, optional, tag = "2")]
    pub monitored_project: ::core::option::Option<MonitoredProject>,
}
/// Request for the `DeleteMonitoredProject` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMonitoredProjectRequest {
    /// Required. The resource name of the `MonitoredProject`.
    /// Example:
    /// `locations/global/metricsScopes/{SCOPING_PROJECT_ID_OR_NUMBER}/projects/{MONITORED_PROJECT_ID_OR_NUMBER}`
    ///
    /// Authorization requires the following [Google
    /// IAM](<https://cloud.google.com/iam>) permissions on both the `Metrics Scope`
    /// and on the `MonitoredProject`: `monitoring.metricsScopes.link`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Contains metadata for longrunning operation for the edit Metrics Scope
/// endpoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Current state of the batch operation.
    #[prost(enumeration = "operation_metadata::State", tag = "1")]
    pub state: i32,
    /// The time when the batch request was received.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the operation result was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// Batch operation states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid.
        Unspecified = 0,
        /// Request has been received.
        Created = 1,
        /// Request is actively being processed.
        Running = 2,
        /// The batch processing is done.
        Done = 3,
        /// The batch processing was cancelled.
        Cancelled = 4,
    }
}
#[doc = r" Generated client implementations."]
pub mod metrics_scopes_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Manages Cloud Monitoring Metrics Scopes, and the monitoring of Google Cloud"]
    #[doc = " projects and AWS accounts."]
    #[derive(Debug, Clone)]
    pub struct MetricsScopesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MetricsScopesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MetricsScopesClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MetricsScopesClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Returns a specific `Metrics Scope`."]
        pub async fn get_metrics_scope(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetricsScopeRequest>,
        ) -> Result<tonic::Response<super::MetricsScope>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.metricsscope.v1.MetricsScopes/GetMetricsScope",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a list of every `Metrics Scope` that a specific `MonitoredProject`"]
        #[doc = " has been added to. The metrics scope representing the specified monitored"]
        #[doc = " project will always be the first entry in the response."]
        pub async fn list_metrics_scopes_by_monitored_project(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetricsScopesByMonitoredProjectRequest>,
        ) -> Result<
            tonic::Response<super::ListMetricsScopesByMonitoredProjectResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.monitoring.metricsscope.v1.MetricsScopes/ListMetricsScopesByMonitoredProject") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds a `MonitoredProject` with the given project ID"]
        #[doc = " to the specified `Metrics Scope`."]
        pub async fn create_monitored_project(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMonitoredProjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.monitoring.metricsscope.v1.MetricsScopes/CreateMonitoredProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a `MonitoredProject` from the specified `Metrics Scope`."]
        pub async fn delete_monitored_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMonitoredProjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.monitoring.metricsscope.v1.MetricsScopes/DeleteMonitoredProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
