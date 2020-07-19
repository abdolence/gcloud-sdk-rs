/// Metadata common to all Datastore Admin operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonMetadata {
    /// The time that work began on the operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time the operation ended, either successfully or otherwise.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The type of the operation. Can be used as a filter in
    /// ListOperationsRequest.
    #[prost(enumeration = "OperationType", tag = "3")]
    pub operation_type: i32,
    /// The client-assigned labels which were provided when the operation was
    /// created. May also include additional labels.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The current state of the Operation.
    #[prost(enumeration = "common_metadata::State", tag = "5")]
    pub state: i32,
}
pub mod common_metadata {
    /// The various possible states for an ongoing Operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified.
        Unspecified = 0,
        /// Request is being prepared for processing.
        Initializing = 1,
        /// Request is actively being processed.
        Processing = 2,
        /// Request is in the process of being cancelled after user called
        /// google.longrunning.Operations.CancelOperation on the operation.
        Cancelling = 3,
        /// Request has been processed and is in its finalization stage.
        Finalizing = 4,
        /// Request has completed successfully.
        Successful = 5,
        /// Request has finished being processed, but encountered an error.
        Failed = 6,
        /// Request has finished being cancelled after user called
        /// google.longrunning.Operations.CancelOperation.
        Cancelled = 7,
    }
}
/// Measures the progress of a particular metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Progress {
    /// The amount of work that has been completed. Note that this may be greater
    /// than work_estimated.
    #[prost(int64, tag = "1")]
    pub work_completed: i64,
    /// An estimate of how much work needs to be performed. May be zero if the
    /// work estimate is unavailable.
    #[prost(int64, tag = "2")]
    pub work_estimated: i64,
}
/// The request for
/// [google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities][google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesRequest {
    /// Project ID against which to make the request.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Client-assigned labels.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Description of what data from the project is included in the export.
    #[prost(message, optional, tag = "3")]
    pub entity_filter: ::std::option::Option<EntityFilter>,
    /// Location for the export metadata and data files.
    ///
    /// The full resource URL of the external storage location. Currently, only
    /// Google Cloud Storage is supported. So output_url_prefix should be of the
    /// form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the
    /// name of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud
    /// Storage namespace path (this is not a Cloud Datastore namespace). For more
    /// information about Cloud Storage namespace paths, see
    /// [Object name
    /// considerations](https://cloud.google.com/storage/docs/naming#object-considerations).
    ///
    /// The resulting files will be nested deeper than the specified URL prefix.
    /// The final output URL will be provided in the
    /// [google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url]
    /// field. That value should be used for subsequent ImportEntities operations.
    ///
    /// By nesting the data files deeper, the same Cloud Storage bucket can be used
    /// in multiple ExportEntities operations without conflict.
    #[prost(string, tag = "4")]
    pub output_url_prefix: std::string::String,
}
/// The request for
/// [google.datastore.admin.v1beta1.DatastoreAdmin.ImportEntities][google.datastore.admin.v1beta1.DatastoreAdmin.ImportEntities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntitiesRequest {
    /// Project ID against which to make the request.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Client-assigned labels.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The full resource URL of the external storage location. Currently, only
    /// Google Cloud Storage is supported. So input_url should be of the form:
    /// `gs://BUCKET_NAME[/NAMESPACE_PATH]/OVERALL_EXPORT_METADATA_FILE`, where
    /// `BUCKET_NAME` is the name of the Cloud Storage bucket, `NAMESPACE_PATH` is
    /// an optional Cloud Storage namespace path (this is not a Cloud Datastore
    /// namespace), and `OVERALL_EXPORT_METADATA_FILE` is the metadata file written
    /// by the ExportEntities operation. For more information about Cloud Storage
    /// namespace paths, see
    /// [Object name
    /// considerations](https://cloud.google.com/storage/docs/naming#object-considerations).
    ///
    /// For more information, see
    /// [google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url].
    #[prost(string, tag = "3")]
    pub input_url: std::string::String,
    /// Optionally specify which kinds/namespaces are to be imported. If provided,
    /// the list must be a subset of the EntityFilter used in creating the export,
    /// otherwise a FAILED_PRECONDITION error will be returned. If no filter is
    /// specified then all entities from the export are imported.
    #[prost(message, optional, tag = "4")]
    pub entity_filter: ::std::option::Option<EntityFilter>,
}
/// The response for
/// [google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities][google.datastore.admin.v1beta1.DatastoreAdmin.ExportEntities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesResponse {
    /// Location of the output metadata file. This can be used to begin an import
    /// into Cloud Datastore (this project or another project). See
    /// [google.datastore.admin.v1beta1.ImportEntitiesRequest.input_url][google.datastore.admin.v1beta1.ImportEntitiesRequest.input_url].
    /// Only present if the operation completed successfully.
    #[prost(string, tag = "1")]
    pub output_url: std::string::String,
}
/// Metadata for ExportEntities operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesMetadata {
    /// Metadata common to all Datastore Admin operations.
    #[prost(message, optional, tag = "1")]
    pub common: ::std::option::Option<CommonMetadata>,
    /// An estimate of the number of entities processed.
    #[prost(message, optional, tag = "2")]
    pub progress_entities: ::std::option::Option<Progress>,
    /// An estimate of the number of bytes processed.
    #[prost(message, optional, tag = "3")]
    pub progress_bytes: ::std::option::Option<Progress>,
    /// Description of which entities are being exported.
    #[prost(message, optional, tag = "4")]
    pub entity_filter: ::std::option::Option<EntityFilter>,
    /// Location for the export metadata and data files. This will be the same
    /// value as the
    /// [google.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix][google.datastore.admin.v1beta1.ExportEntitiesRequest.output_url_prefix]
    /// field. The final output location is provided in
    /// [google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url].
    #[prost(string, tag = "5")]
    pub output_url_prefix: std::string::String,
}
/// Metadata for ImportEntities operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntitiesMetadata {
    /// Metadata common to all Datastore Admin operations.
    #[prost(message, optional, tag = "1")]
    pub common: ::std::option::Option<CommonMetadata>,
    /// An estimate of the number of entities processed.
    #[prost(message, optional, tag = "2")]
    pub progress_entities: ::std::option::Option<Progress>,
    /// An estimate of the number of bytes processed.
    #[prost(message, optional, tag = "3")]
    pub progress_bytes: ::std::option::Option<Progress>,
    /// Description of which entities are being imported.
    #[prost(message, optional, tag = "4")]
    pub entity_filter: ::std::option::Option<EntityFilter>,
    /// The location of the import metadata file. This will be the same value as
    /// the
    /// [google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url][google.datastore.admin.v1beta1.ExportEntitiesResponse.output_url]
    /// field.
    #[prost(string, tag = "5")]
    pub input_url: std::string::String,
}
/// Identifies a subset of entities in a project. This is specified as
/// combinations of kinds and namespaces (either or both of which may be all, as
/// described in the following examples).
/// Example usage:
///
/// Entire project:
///   kinds=[], namespace_ids=[]
///
/// Kinds Foo and Bar in all namespaces:
///   kinds=['Foo', 'Bar'], namespace_ids=[]
///
/// Kinds Foo and Bar only in the default namespace:
///   kinds=['Foo', 'Bar'], namespace_ids=['']
///
/// Kinds Foo and Bar in both the default and Baz namespaces:
///   kinds=['Foo', 'Bar'], namespace_ids=['', 'Baz']
///
/// The entire Baz namespace:
///   kinds=[], namespace_ids=['Baz']
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityFilter {
    /// If empty, then this represents all kinds.
    #[prost(string, repeated, tag = "1")]
    pub kinds: ::std::vec::Vec<std::string::String>,
    /// An empty list represents all namespaces. This is the preferred
    /// usage for projects that don't use namespaces.
    ///
    /// An empty string element represents the default namespace. This should be
    /// used if the project has data in non-default namespaces, but doesn't want to
    /// include them.
    /// Each namespace in this list must be unique.
    #[prost(string, repeated, tag = "2")]
    pub namespace_ids: ::std::vec::Vec<std::string::String>,
}
/// Operation types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    /// Unspecified.
    Unspecified = 0,
    /// ExportEntities.
    ExportEntities = 1,
    /// ImportEntities.
    ImportEntities = 2,
}
#[doc = r" Generated client implementations."]
pub mod datastore_admin_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Google Cloud Datastore Admin API"]
    #[doc = ""]
    #[doc = " The Datastore Admin API provides several admin services for Cloud Datastore."]
    #[doc = ""]
    #[doc = " -----------------------------------------------------------------------------"]
    #[doc = " ## Concepts"]
    #[doc = ""]
    #[doc = " Project, namespace, kind, and entity as defined in the Google Cloud Datastore"]
    #[doc = " API."]
    #[doc = ""]
    #[doc = " Operation: An Operation represents work being performed in the background."]
    #[doc = ""]
    #[doc = " EntityFilter: Allows specifying a subset of entities in a project. This is"]
    #[doc = " specified as a combination of kinds and namespaces (either or both of which"]
    #[doc = " may be all)."]
    #[doc = ""]
    #[doc = " -----------------------------------------------------------------------------"]
    #[doc = " ## Services"]
    #[doc = ""]
    #[doc = " # Export/Import"]
    #[doc = ""]
    #[doc = " The Export/Import service provides the ability to copy all or a subset of"]
    #[doc = " entities to/from Google Cloud Storage."]
    #[doc = ""]
    #[doc = " Exported data may be imported into Cloud Datastore for any Google Cloud"]
    #[doc = " Platform project. It is not restricted to the export source project. It is"]
    #[doc = " possible to export from one project and then import into another."]
    #[doc = ""]
    #[doc = " Exported data can also be loaded into Google BigQuery for analysis."]
    #[doc = ""]
    #[doc = " Exports and imports are performed asynchronously. An Operation resource is"]
    #[doc = " created for each export/import. The state (including any errors encountered)"]
    #[doc = " of the export/import may be queried via the Operation resource."]
    #[doc = ""]
    #[doc = " # Operation"]
    #[doc = ""]
    #[doc = " The Operations collection provides a record of actions performed for the"]
    #[doc = " specified project (including any operations in progress). Operations are not"]
    #[doc = " created directly but through calls on other collections or resources."]
    #[doc = ""]
    #[doc = " An operation that is not yet done may be cancelled. The request to cancel is"]
    #[doc = " asynchronous and the operation may continue to run for some time after the"]
    #[doc = " request to cancel is made."]
    #[doc = ""]
    #[doc = " An operation that is done may be deleted so that it is no longer listed as"]
    #[doc = " part of the Operation collection."]
    #[doc = ""]
    #[doc = " ListOperations returns all pending operations, but not completed operations."]
    #[doc = ""]
    #[doc = " Operations are created by service DatastoreAdmin,"]
    #[doc = " but are accessed via service google.longrunning.Operations."]
    pub struct DatastoreAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DatastoreAdminClient<T>
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
        #[doc = " Exports a copy of all or a subset of entities from Google Cloud Datastore"]
        #[doc = " to another storage system, such as Google Cloud Storage. Recent updates to"]
        #[doc = " entities may not be reflected in the export. The export occurs in the"]
        #[doc = " background and its progress can be monitored and managed via the"]
        #[doc = " Operation resource that is created. The output of an export may only be"]
        #[doc = " used once the associated operation is done. If an export operation is"]
        #[doc = " cancelled before completion it may leave partial data behind in Google"]
        #[doc = " Cloud Storage."]
        pub async fn export_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportEntitiesRequest>,
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
                "/google.datastore.admin.v1beta1.DatastoreAdmin/ExportEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Imports entities into Google Cloud Datastore. Existing entities with the"]
        #[doc = " same key are overwritten. The import occurs in the background and its"]
        #[doc = " progress can be monitored and managed via the Operation resource that is"]
        #[doc = " created. If an ImportEntities operation is cancelled, it is possible"]
        #[doc = " that a subset of the data has already been imported to Cloud Datastore."]
        pub async fn import_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportEntitiesRequest>,
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
                "/google.datastore.admin.v1beta1.DatastoreAdmin/ImportEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DatastoreAdminClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DatastoreAdminClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DatastoreAdminClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod datastore_admin_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DatastoreAdminServer."]
    #[async_trait]
    pub trait DatastoreAdmin: Send + Sync + 'static {
        #[doc = " Exports a copy of all or a subset of entities from Google Cloud Datastore"]
        #[doc = " to another storage system, such as Google Cloud Storage. Recent updates to"]
        #[doc = " entities may not be reflected in the export. The export occurs in the"]
        #[doc = " background and its progress can be monitored and managed via the"]
        #[doc = " Operation resource that is created. The output of an export may only be"]
        #[doc = " used once the associated operation is done. If an export operation is"]
        #[doc = " cancelled before completion it may leave partial data behind in Google"]
        #[doc = " Cloud Storage."]
        async fn export_entities(
            &self,
            request: tonic::Request<super::ExportEntitiesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Imports entities into Google Cloud Datastore. Existing entities with the"]
        #[doc = " same key are overwritten. The import occurs in the background and its"]
        #[doc = " progress can be monitored and managed via the Operation resource that is"]
        #[doc = " created. If an ImportEntities operation is cancelled, it is possible"]
        #[doc = " that a subset of the data has already been imported to Cloud Datastore."]
        async fn import_entities(
            &self,
            request: tonic::Request<super::ImportEntitiesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Google Cloud Datastore Admin API"]
    #[doc = ""]
    #[doc = " The Datastore Admin API provides several admin services for Cloud Datastore."]
    #[doc = ""]
    #[doc = " -----------------------------------------------------------------------------"]
    #[doc = " ## Concepts"]
    #[doc = ""]
    #[doc = " Project, namespace, kind, and entity as defined in the Google Cloud Datastore"]
    #[doc = " API."]
    #[doc = ""]
    #[doc = " Operation: An Operation represents work being performed in the background."]
    #[doc = ""]
    #[doc = " EntityFilter: Allows specifying a subset of entities in a project. This is"]
    #[doc = " specified as a combination of kinds and namespaces (either or both of which"]
    #[doc = " may be all)."]
    #[doc = ""]
    #[doc = " -----------------------------------------------------------------------------"]
    #[doc = " ## Services"]
    #[doc = ""]
    #[doc = " # Export/Import"]
    #[doc = ""]
    #[doc = " The Export/Import service provides the ability to copy all or a subset of"]
    #[doc = " entities to/from Google Cloud Storage."]
    #[doc = ""]
    #[doc = " Exported data may be imported into Cloud Datastore for any Google Cloud"]
    #[doc = " Platform project. It is not restricted to the export source project. It is"]
    #[doc = " possible to export from one project and then import into another."]
    #[doc = ""]
    #[doc = " Exported data can also be loaded into Google BigQuery for analysis."]
    #[doc = ""]
    #[doc = " Exports and imports are performed asynchronously. An Operation resource is"]
    #[doc = " created for each export/import. The state (including any errors encountered)"]
    #[doc = " of the export/import may be queried via the Operation resource."]
    #[doc = ""]
    #[doc = " # Operation"]
    #[doc = ""]
    #[doc = " The Operations collection provides a record of actions performed for the"]
    #[doc = " specified project (including any operations in progress). Operations are not"]
    #[doc = " created directly but through calls on other collections or resources."]
    #[doc = ""]
    #[doc = " An operation that is not yet done may be cancelled. The request to cancel is"]
    #[doc = " asynchronous and the operation may continue to run for some time after the"]
    #[doc = " request to cancel is made."]
    #[doc = ""]
    #[doc = " An operation that is done may be deleted so that it is no longer listed as"]
    #[doc = " part of the Operation collection."]
    #[doc = ""]
    #[doc = " ListOperations returns all pending operations, but not completed operations."]
    #[doc = ""]
    #[doc = " Operations are created by service DatastoreAdmin,"]
    #[doc = " but are accessed via service google.longrunning.Operations."]
    #[derive(Debug)]
    pub struct DatastoreAdminServer<T: DatastoreAdmin> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DatastoreAdmin> DatastoreAdminServer<T> {
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
    impl<T, B> Service<http::Request<B>> for DatastoreAdminServer<T>
    where
        T: DatastoreAdmin,
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
                "/google.datastore.admin.v1beta1.DatastoreAdmin/ExportEntities" => {
                    #[allow(non_camel_case_types)]
                    struct ExportEntitiesSvc<T: DatastoreAdmin>(pub Arc<T>);
                    impl<T: DatastoreAdmin>
                        tonic::server::UnaryService<super::ExportEntitiesRequest>
                        for ExportEntitiesSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportEntitiesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).export_entities(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ExportEntitiesSvc(inner);
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
                "/google.datastore.admin.v1beta1.DatastoreAdmin/ImportEntities" => {
                    #[allow(non_camel_case_types)]
                    struct ImportEntitiesSvc<T: DatastoreAdmin>(pub Arc<T>);
                    impl<T: DatastoreAdmin>
                        tonic::server::UnaryService<super::ImportEntitiesRequest>
                        for ImportEntitiesSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportEntitiesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).import_entities(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ImportEntitiesSvc(inner);
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
    impl<T: DatastoreAdmin> Clone for DatastoreAdminServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DatastoreAdmin> Clone for _Inner<T> {
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
