/// The details about the data source when it is a local file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalFileSource {
    /// The file name and extension of the uploaded file.
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
    /// Protobuf file.
    Proto = 4,
    /// KMZ file.
    Kmz = 5,
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
            FileFormat::Proto => "FILE_FORMAT_PROTO",
            FileFormat::Kmz => "FILE_FORMAT_KMZ",
        }
    }
}
/// A representation of a maps platform dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    /// Resource name,
    /// projects/{project}/datasets/{dataset_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human readable name, shown in the console UI. Set by customer.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A description of this dataset; set by the customer.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The version of the dataset.
    #[prost(string, tag = "4")]
    pub version_id: ::prost::alloc::string::String,
    /// Specified use case(s) for this dataset.
    #[prost(enumeration = "Usage", repeated, tag = "5")]
    pub usage: ::prost::alloc::vec::Vec<i32>,
    /// The status of the import of the latest dataset version.
    #[prost(enumeration = "State", tag = "12")]
    pub status: i32,
    /// Output only. Time when the dataset was first created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the dataset metadata was last updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this version of dataset was created. (It happened when importing
    /// data to the dataset)
    #[prost(message, optional, tag = "10")]
    pub version_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The description for this version of dataset. It is provided when importing
    /// data to the dataset.
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
/// Usage specifies where the data is intended to be used to inform how to
/// process the data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Usage {
    /// The usage of this dataset is not set.
    Unspecified = 0,
    /// This dataset will be used for data driven styling.
    DataDrivenStyling = 1,
    /// This dataset will be used for area affordances in routing.
    AreaAffordances = 2,
    /// This dataset will be used for assisted driving in routing.
    AssistedDriving = 3,
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
            Usage::AreaAffordances => "USAGE_AREA_AFFORDANCES",
            Usage::AssistedDriving => "USAGE_ASSISTED_DRIVING",
        }
    }
}
/// State specifies the status of the import of the latest dataset version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// The state of this dataset is not set.
    Unspecified = 0,
    /// The dataset version is getting imported.
    Importing = 1,
    /// The dataset version succeeded in getting imported.
    ImportSucceeded = 2,
    /// The dataset version failed to get imported.
    ImportFailed = 3,
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
        }
    }
}
/// Request to create a maps dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// Required. Parent project that will own the dataset.
    /// Format: projects/{$project_number}
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
    /// Required. The dataset to update. The dataset's name is used to identify the dataset
    /// to be updated. The name has the format:
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
    /// Required. Resource name. Can also fetch a specified version
    /// projects/{project}/datasets/{dataset_id}
    /// projects/{project}/datasets/{dataset_id}@{version-id}
    ///
    /// In order to retrieve a previous version of the dataset, also provide
    /// the version ID.
    /// Example: projects/123/datasets/assisted-driving-preferences@c7cfa2a8
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If specified, will fetch the dataset details of the version published for
    /// the specified use case rather than the latest, if one exists. If a
    /// published version does not exist, will default to getting the dataset
    /// details of the latest version.
    #[prost(enumeration = "Usage", tag = "2")]
    pub published_usage: i32,
}
/// Request to list of all versions of the dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetVersionsRequest {
    /// Required. The name of the dataset to list all the versions for.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The maximum number of versions to return per page.
    /// If unspecified (or zero), at most 1000 versions will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token, received from a previous GetDatasetVersions call.
    /// Provide this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response with list of all versions of the dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetVersionsResponse {
    /// All the versions of the dataset.
    #[prost(message, repeated, tag = "1")]
    pub datasets: ::prost::alloc::vec::Vec<Dataset>,
    /// A token that can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to list datasets for the project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsRequest {
    /// Required. The name of the project to list all the datasets for.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of versions to return per page.
    /// If unspecified (or zero), at most 1000 datasets will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token, received from a previous GetDatasetVersions call.
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
    /// If set to true, any dataset version for this dataset will also be deleted.
    /// (Otherwise, the request will only work if the dataset has no versions.)
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request to delete a version of a dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetVersionRequest {
    /// Required. Format: projects/${project}/datasets/{dataset_id}@{version-id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod maps_platform_datasets_v1_alpha_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service definition for the Maps Platform Datasets API.
    #[derive(Debug, Clone)]
    pub struct MapsPlatformDatasetsV1AlphaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MapsPlatformDatasetsV1AlphaClient<tonic::transport::Channel> {
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
    impl<T> MapsPlatformDatasetsV1AlphaClient<T>
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
        ) -> MapsPlatformDatasetsV1AlphaClient<InterceptedService<T, F>>
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
            MapsPlatformDatasetsV1AlphaClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Create a new dataset for the specified project.
        pub async fn create_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.maps.mapsplatformdatasets.v1alpha.MapsPlatformDatasetsV1Alpha/CreateDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Update the metadata for the dataset. To update the data use: UploadDataset.
        pub async fn update_dataset_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetMetadataRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.maps.mapsplatformdatasets.v1alpha.MapsPlatformDatasetsV1Alpha/UpdateDatasetMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get the published or latest version of the dataset.
        pub async fn get_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
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
                "/google.maps.mapsplatformdatasets.v1alpha.MapsPlatformDatasetsV1Alpha/GetDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List all the versions of a dataset.
        pub async fn list_dataset_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatasetVersionsRequest>,
        ) -> Result<tonic::Response<super::ListDatasetVersionsResponse>, tonic::Status> {
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
                "/google.maps.mapsplatformdatasets.v1alpha.MapsPlatformDatasetsV1Alpha/ListDatasetVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List all the datasets for the specified project.
        pub async fn list_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatasetsRequest>,
        ) -> Result<tonic::Response<super::ListDatasetsResponse>, tonic::Status> {
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
                "/google.maps.mapsplatformdatasets.v1alpha.MapsPlatformDatasetsV1Alpha/ListDatasets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete the specified dataset and optionally all its corresponding
        /// versions.
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetRequest>,
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
                "/google.maps.mapsplatformdatasets.v1alpha.MapsPlatformDatasetsV1Alpha/DeleteDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a specific version of the dataset.
        pub async fn delete_dataset_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetVersionRequest>,
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
                "/google.maps.mapsplatformdatasets.v1alpha.MapsPlatformDatasetsV1Alpha/DeleteDatasetVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
