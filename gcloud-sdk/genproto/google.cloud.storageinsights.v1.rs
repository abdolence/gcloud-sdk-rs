/// Message for requesting list of ReportConfigs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportConfigsRequest {
    /// Required. Parent value for ListReportConfigsRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing ReportConfigs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportConfigsResponse {
    /// The list of ReportConfig
    #[prost(message, repeated, tag = "1")]
    pub report_configs: ::prost::alloc::vec::Vec<ReportConfig>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a ReportConfig
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReportConfigRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message for creating a ReportConfig
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReportConfigRequest {
    /// Required. Value for parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The resource being created
    #[prost(message, optional, tag = "3")]
    pub report_config: ::core::option::Option<ReportConfig>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for updating a ReportConfig
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReportConfigRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// ReportConfig resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The resource being updated
    #[prost(message, optional, tag = "2")]
    pub report_config: ::core::option::Option<ReportConfig>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message for deleting a ReportConfig
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReportConfigRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If set, all ReportDetails for this ReportConfig will be deleted.
    #[prost(bool, tag = "2")]
    pub force: bool,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Message describing ReportDetail object. ReportDetail represents metadata of
/// generated reports for a ReportConfig.
/// Next ID: 8
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportDetail {
    /// Name of resource. It will be of form
    /// projects/<project>/locations/<location>/reportConfigs/<report-config-id>/reportDetails/<report-detail-id>.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The snapshot time.
    /// All the report data is referenced at this point of time.
    #[prost(message, optional, tag = "2")]
    pub snapshot_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Generated report's full path with name. It will be of the form
    /// destination_bucket/<destination_path>/<report>.
    #[prost(string, repeated, tag = "3")]
    pub report_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Status of the ReportDetail.
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// Labels as key value pairs
    #[prost(map = "string, string", tag = "5")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The date for which report is generated. The time part of target_datetime
    /// will be zero till we support multiple reports per day.
    #[prost(message, optional, tag = "6")]
    pub target_datetime: ::core::option::Option<super::super::super::r#type::DateTime>,
    /// Metrics of the report.
    #[prost(message, optional, tag = "7")]
    pub report_metrics: ::core::option::Option<report_detail::Metrics>,
}
/// Nested message and enum types in `ReportDetail`.
pub mod report_detail {
    /// Different metrics associated with the generated report.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metrics {
        /// Count of Cloud Storage objects which are part of the report.
        #[prost(int64, tag = "1")]
        pub processed_records_count: i64,
    }
}
/// Message for requesting list of ReportDetails
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportDetailsRequest {
    /// Required. Parent value for ListReportDetailsRequest
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer items than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filtering results
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Hint for how to order the results
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Message for response to listing ReportDetails
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReportDetailsResponse {
    /// The list of ReportDetail
    #[prost(message, repeated, tag = "1")]
    pub report_details: ::prost::alloc::vec::Vec<ReportDetail>,
    /// A token identifying a page of results the server should return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for getting a ReportDetail
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReportDetailRequest {
    /// Required. Name of the resource
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have been cancelled successfully
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// ReportConfig Resource:
///
/// Options to setup frequency of report generation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyOptions {
    /// Frequency of report generation.
    #[prost(enumeration = "frequency_options::Frequency", tag = "1")]
    pub frequency: i32,
    /// The date from which report generation should start.
    /// UTC time zone.
    #[prost(message, optional, tag = "2")]
    pub start_date: ::core::option::Option<super::super::super::r#type::Date>,
    /// The date on which report generation should stop (Inclusive).
    /// UTC time zone.
    #[prost(message, optional, tag = "3")]
    pub end_date: ::core::option::Option<super::super::super::r#type::Date>,
}
/// Nested message and enum types in `FrequencyOptions`.
pub mod frequency_options {
    /// This ENUM specifies possible frequencies of report generation.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Frequency {
        /// Unspecified.
        Unspecified = 0,
        /// Report will be generated daily.
        Daily = 1,
        /// Report will be generated weekly.
        Weekly = 2,
    }
    impl Frequency {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Frequency::Unspecified => "FREQUENCY_UNSPECIFIED",
                Frequency::Daily => "DAILY",
                Frequency::Weekly => "WEEKLY",
            }
        }
    }
}
/// Options to configure CSV formatted reports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsvOptions {
    /// Record separator characters in CSV.
    #[prost(string, tag = "1")]
    pub record_separator: ::prost::alloc::string::String,
    /// Delimiter characters in CSV.
    #[prost(string, tag = "2")]
    pub delimiter: ::prost::alloc::string::String,
    /// If set, will include a header row in the CSV report.
    #[prost(bool, tag = "3")]
    pub header_required: bool,
}
/// Options to filter data on storage systems.
/// Next ID: 2
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudStorageFilters {
    /// Bucket for which the report will be generated.
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
}
/// Options to store reports in storage systems.
/// Next ID: 3
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudStorageDestinationOptions {
    /// Destination bucket.
    #[prost(string, tag = "1")]
    pub bucket: ::prost::alloc::string::String,
    /// Destination path is the path in the bucket where the report should be
    /// generated.
    #[prost(string, tag = "2")]
    pub destination_path: ::prost::alloc::string::String,
}
/// Report specification for exporting object metadata.
/// Next ID: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectMetadataReportOptions {
    /// Metadata fields to be included in the report.
    #[prost(string, repeated, tag = "1")]
    pub metadata_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Filter options for storage systems.
    #[prost(oneof = "object_metadata_report_options::Filter", tags = "2")]
    pub filter: ::core::option::Option<object_metadata_report_options::Filter>,
    /// Options on destination for storage systems.
    #[prost(oneof = "object_metadata_report_options::DestinationOptions", tags = "3")]
    pub destination_options: ::core::option::Option<
        object_metadata_report_options::DestinationOptions,
    >,
}
/// Nested message and enum types in `ObjectMetadataReportOptions`.
pub mod object_metadata_report_options {
    /// Filter options for storage systems.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Cloud Storage as the storage system.
        #[prost(message, tag = "2")]
        StorageFilters(super::CloudStorageFilters),
    }
    /// Options on destination for storage systems.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DestinationOptions {
        /// Cloud Storage as the storage system.
        #[prost(message, tag = "3")]
        StorageDestinationOptions(super::CloudStorageDestinationOptions),
    }
}
/// Message describing ReportConfig object. ReportConfig is the configuration to
/// generate reports.
/// Next ID: 12
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportConfig {
    /// name of resource. It will be of form
    /// projects/<project>/locations/<location>/reportConfigs/<report-config-id>.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. [Output only] Create time stamp
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. [Output only] Update time stamp
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The frequency of report generation.
    #[prost(message, optional, tag = "5")]
    pub frequency_options: ::core::option::Option<FrequencyOptions>,
    /// Labels as key value pairs
    #[prost(map = "string, string", tag = "10")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// User provided display name which can be empty and limited to 256 characters
    /// that is editable.
    #[prost(string, tag = "11")]
    pub display_name: ::prost::alloc::string::String,
    /// Format in which report will be published.
    #[prost(oneof = "report_config::ReportFormat", tags = "6")]
    pub report_format: ::core::option::Option<report_config::ReportFormat>,
    /// Configuration options for report contents.
    #[prost(oneof = "report_config::ReportKind", tags = "8")]
    pub report_kind: ::core::option::Option<report_config::ReportKind>,
}
/// Nested message and enum types in `ReportConfig`.
pub mod report_config {
    /// Format in which report will be published.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ReportFormat {
        /// Options for CSV formatted reports.
        #[prost(message, tag = "6")]
        CsvOptions(super::CsvOptions),
    }
    /// Configuration options for report contents.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ReportKind {
        /// Report for exporting object metadata.
        #[prost(message, tag = "8")]
        ObjectMetadataReportOptions(super::ObjectMetadataReportOptions),
    }
}
/// Generated client implementations.
pub mod storage_insights_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service describing handlers for resources
    #[derive(Debug, Clone)]
    pub struct StorageInsightsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageInsightsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StorageInsightsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StorageInsightsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            StorageInsightsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Lists ReportConfigs in a given project and location.
        pub async fn list_report_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReportConfigsRequest>,
        ) -> Result<tonic::Response<super::ListReportConfigsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.storageinsights.v1.StorageInsights/ListReportConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single ReportConfig.
        pub async fn get_report_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReportConfigRequest>,
        ) -> Result<tonic::Response<super::ReportConfig>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.storageinsights.v1.StorageInsights/GetReportConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new ReportConfig in a given project and location.
        pub async fn create_report_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReportConfigRequest>,
        ) -> Result<tonic::Response<super::ReportConfig>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.storageinsights.v1.StorageInsights/CreateReportConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single ReportConfig.
        pub async fn update_report_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateReportConfigRequest>,
        ) -> Result<tonic::Response<super::ReportConfig>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.storageinsights.v1.StorageInsights/UpdateReportConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single ReportConfig.
        pub async fn delete_report_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReportConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.storageinsights.v1.StorageInsights/DeleteReportConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists ReportDetails in a given project and location.
        pub async fn list_report_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReportDetailsRequest>,
        ) -> Result<tonic::Response<super::ListReportDetailsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.storageinsights.v1.StorageInsights/ListReportDetails",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single ReportDetail.
        pub async fn get_report_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReportDetailRequest>,
        ) -> Result<tonic::Response<super::ReportDetail>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.storageinsights.v1.StorageInsights/GetReportDetail",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
