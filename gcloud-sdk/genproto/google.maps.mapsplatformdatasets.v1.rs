/// The details about the data source when it is a local file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalFileSource {
    /// The file name of the uploaded file.
    #[prost(string, tag = "1")]
    pub filename: ::prost::alloc::string::String,
    /// The format of the file that is being uploaded.
    #[prost(enumeration = "FileFormat", tag = "2")]
    pub file_format: i32,
}
/// The details about the data source when it is in Google Cloud Storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Source data URI. For example, `gs://my_bucket/my_object`.
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// The file format of the Google Cloud Storage object. This is used mainly for
    /// validation.
    #[prost(enumeration = "FileFormat", tag = "2")]
    pub file_format: i32,
}
/// The format of the file being uploaded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FileFormat {
    /// Unspecified file format.
    Unspecified = 0,
    /// GeoJson file.
    Geojson = 1,
    /// KML file.
    Kml = 2,
    /// CSV file.
    Csv = 3,
}
impl FileFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FileFormat::Unspecified => "FILE_FORMAT_UNSPECIFIED",
            FileFormat::Geojson => "FILE_FORMAT_GEOJSON",
            FileFormat::Kml => "FILE_FORMAT_KML",
            FileFormat::Csv => "FILE_FORMAT_CSV",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FILE_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "FILE_FORMAT_GEOJSON" => Some(Self::Geojson),
            "FILE_FORMAT_KML" => Some(Self::Kml),
            "FILE_FORMAT_CSV" => Some(Self::Csv),
            _ => None,
        }
    }
}
/// A representation of a Maps Dataset resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    /// Resource name,
    /// projects/{project}/datasets/{dataset_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human readable name, shown in the console UI .
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A description of this dataset .
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The version ID of the dataset.
    #[prost(string, tag = "4")]
    pub version_id: ::prost::alloc::string::String,
    /// Specified use case for this dataset.
    #[prost(enumeration = "Usage", repeated, tag = "5")]
    pub usage: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The status of this dataset version.
    #[prost(message, optional, tag = "12")]
    pub status: ::core::option::Option<Status>,
    /// Output only. Time when the dataset was first created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the dataset metadata was last updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the data was uploaded.
    #[prost(message, optional, tag = "10")]
    pub version_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The description for this version of dataset. It is provided
    /// when importing data to the dataset.
    #[prost(string, tag = "11")]
    pub version_description: ::prost::alloc::string::String,
    /// Details about the source of the data for the dataset.
    #[prost(oneof = "dataset::DataSource", tags = "6, 7")]
    pub data_source: ::core::option::Option<dataset::DataSource>,
}
/// Nested message and enum types in `Dataset`.
pub mod dataset {
    /// Details about the source of the data for the dataset.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataSource {
        /// A local file source for the dataset for a single upload.
        #[prost(message, tag = "6")]
        LocalFileSource(super::LocalFileSource),
        /// A Google Cloud Storage file source for the dataset for a single upload.
        #[prost(message, tag = "7")]
        GcsSource(super::GcsSource),
    }
}
/// Status of the dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// State enum for status.
    #[prost(enumeration = "status::State", tag = "1")]
    pub state: i32,
    /// Error message indicating reason of failure. It is empty if the datasets is
    /// not in a failed state.
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Status`.
pub mod status {
    /// A list of states for the dataset.
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
    pub enum State {
        /// The state of this dataset is not set.
        Unspecified = 0,
        /// Data is being imported to a dataset.
        Importing = 1,
        /// Importing data to a dataset succeeded.
        ImportSucceeded = 2,
        /// Importing data to a dataset failed.
        ImportFailed = 3,
        /// The dataset is in the process of getting deleted.
        Deleting = 4,
        /// The deletion failed state. This state represents that dataset deletion
        /// has failed. Deletion may be retried.
        DeletionFailed = 5,
        /// Data is being processed.
        Processing = 6,
        /// The processing failed state. This state represents that processing has
        /// failed and may report errors.
        ProcessingFailed = 7,
        /// This state is currently not used.
        NeedsReview = 8,
        /// The publishing state. This state represents the publishing is in
        /// progress.
        Publishing = 9,
        /// The publishing failed states. This state represents that the
        /// publishing failed. Publishing may be retried.
        PublishingFailed = 10,
        /// The completed state. This state represents the dataset being
        /// available for its specific usage.
        Completed = 11,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Importing => "STATE_IMPORTING",
                State::ImportSucceeded => "STATE_IMPORT_SUCCEEDED",
                State::ImportFailed => "STATE_IMPORT_FAILED",
                State::Deleting => "STATE_DELETING",
                State::DeletionFailed => "STATE_DELETION_FAILED",
                State::Processing => "STATE_PROCESSING",
                State::ProcessingFailed => "STATE_PROCESSING_FAILED",
                State::NeedsReview => "STATE_NEEDS_REVIEW",
                State::Publishing => "STATE_PUBLISHING",
                State::PublishingFailed => "STATE_PUBLISHING_FAILED",
                State::Completed => "STATE_COMPLETED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATE_IMPORTING" => Some(Self::Importing),
                "STATE_IMPORT_SUCCEEDED" => Some(Self::ImportSucceeded),
                "STATE_IMPORT_FAILED" => Some(Self::ImportFailed),
                "STATE_DELETING" => Some(Self::Deleting),
                "STATE_DELETION_FAILED" => Some(Self::DeletionFailed),
                "STATE_PROCESSING" => Some(Self::Processing),
                "STATE_PROCESSING_FAILED" => Some(Self::ProcessingFailed),
                "STATE_NEEDS_REVIEW" => Some(Self::NeedsReview),
                "STATE_PUBLISHING" => Some(Self::Publishing),
                "STATE_PUBLISHING_FAILED" => Some(Self::PublishingFailed),
                "STATE_COMPLETED" => Some(Self::Completed),
                _ => None,
            }
        }
    }
}
/// Usage specifies where the data is intended to be used to inform how to
/// process the data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Usage {
    /// The usage of this dataset is not set.
    Unspecified = 0,
    /// This dataset will be used for data driven styling.
    DataDrivenStyling = 1,
}
impl Usage {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Usage::Unspecified => "USAGE_UNSPECIFIED",
            Usage::DataDrivenStyling => "USAGE_DATA_DRIVEN_STYLING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USAGE_UNSPECIFIED" => Some(Self::Unspecified),
            "USAGE_DATA_DRIVEN_STYLING" => Some(Self::DataDrivenStyling),
            _ => None,
        }
    }
}
/// Request to create a maps dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// Required. Parent project that will own the dataset.
    /// Format: projects/{$project}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The dataset version to create.
    #[prost(message, optional, tag = "2")]
    pub dataset: ::core::option::Option<Dataset>,
}
/// Request to update the metadata fields of the dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetMetadataRequest {
    /// Required. The dataset to update. The dataset's name is used to identify the
    /// dataset to be updated. The name has the format:
    /// projects/{project}/datasets/{dataset_id}
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<Dataset>,
    /// The list of fields to be updated. Support the value "*" for full
    /// replacement.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to get the specified dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetRequest {
    /// Required. Resource name. projects/{project}/datasets/{dataset_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to list datasets for the project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsRequest {
    /// Required. The name of the project to list all the datasets for.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of versions to return per page.
    /// If unspecified (or zero), all datasets will be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token, received from a previous ListDatasets call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response to list datasets for the project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsResponse {
    /// All the datasets for the project.
    #[prost(message, repeated, tag = "1")]
    pub datasets: ::prost::alloc::vec::Vec<Dataset>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to delete a dataset.
///
/// The dataset to be deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetRequest {
    /// Required. Format: projects/${project}/datasets/{dataset_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod maps_platform_datasets_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service definition for the Maps Platform Datasets API.
    #[derive(Debug, Clone)]
    pub struct MapsPlatformDatasetsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MapsPlatformDatasetsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MapsPlatformDatasetsClient<T>
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
        ) -> MapsPlatformDatasetsClient<InterceptedService<T, F>>
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
            MapsPlatformDatasetsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Create a new dataset for the specified project.
        pub async fn create_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
        ) -> std::result::Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets/CreateDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets",
                        "CreateDataset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Update the metadata for the dataset.
        pub async fn update_dataset_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets/UpdateDatasetMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets",
                        "UpdateDatasetMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the dataset.
        pub async fn get_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetRequest>,
        ) -> std::result::Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets/GetDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets",
                        "GetDataset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all the datasets for the specified project.
        pub async fn list_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatasetsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListDatasetsResponse>,
            tonic::Status,
        > {
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
                "/google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets/ListDatasets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets",
                        "ListDatasets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Delete the specified dataset .
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets/DeleteDataset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.maps.mapsplatformdatasets.v1.MapsPlatformDatasets",
                        "DeleteDataset",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}