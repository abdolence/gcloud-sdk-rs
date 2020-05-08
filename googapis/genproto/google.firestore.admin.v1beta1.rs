/// A field of an index.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexField {
    /// The path of the field. Must match the field path specification described
    /// by [google.firestore.v1beta1.Document.fields][fields].
    /// Special field path `__name__` may be used by itself or at the end of a
    /// path. `__type__` may be used only at the end of path.
    #[prost(string, tag = "1")]
    pub field_path: std::string::String,
    /// The field's mode.
    #[prost(enumeration = "index_field::Mode", tag = "2")]
    pub mode: i32,
}
pub mod index_field {
    /// The mode determines how a field is indexed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// The mode is unspecified.
        Unspecified = 0,
        /// The field's values are indexed so as to support sequencing in
        /// ascending order and also query by <, >, <=, >=, and =.
        Ascending = 2,
        /// The field's values are indexed so as to support sequencing in
        /// descending order and also query by <, >, <=, >=, and =.
        Descending = 3,
        /// The field's array values are indexed so as to support membership using
        /// ARRAY_CONTAINS queries.
        ArrayContains = 4,
    }
}
/// An index definition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Index {
    /// The resource name of the index.
    /// Output only.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The collection ID to which this index applies. Required.
    #[prost(string, tag = "2")]
    pub collection_id: std::string::String,
    /// The fields to index.
    #[prost(message, repeated, tag = "3")]
    pub fields: ::std::vec::Vec<IndexField>,
    /// The state of the index.
    /// Output only.
    #[prost(enumeration = "index::State", tag = "6")]
    pub state: i32,
}
pub mod index {
    /// The state of an index. During index creation, an index will be in the
    /// `CREATING` state. If the index is created successfully, it will transition
    /// to the `READY` state. If the index is not able to be created, it will
    /// transition to the `ERROR` state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state is unspecified.
        Unspecified = 0,
        /// The index is being created.
        /// There is an active long-running operation for the index.
        /// The index is updated when writing a document.
        /// Some index data may exist.
        Creating = 3,
        /// The index is ready to be used.
        /// The index is updated when writing a document.
        /// The index is fully populated from all stored documents it applies to.
        Ready = 2,
        /// The index was being created, but something went wrong.
        /// There is no active long-running operation for the index,
        /// and the most recently finished long-running operation failed.
        /// The index is not updated when writing a document.
        /// Some index data may exist.
        Error = 5,
    }
}
/// Metadata for index operations. This metadata populates
/// the metadata field of [google.longrunning.Operation][google.longrunning.Operation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexOperationMetadata {
    /// The time that work began on the operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time the operation ended, either successfully or otherwise. Unset if
    /// the operation is still active.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The index resource that this operation is acting on. For example:
    /// `projects/{project_id}/databases/{database_id}/indexes/{index_id}`
    #[prost(string, tag = "3")]
    pub index: std::string::String,
    /// The type of index operation.
    #[prost(enumeration = "index_operation_metadata::OperationType", tag = "4")]
    pub operation_type: i32,
    /// True if the [google.longrunning.Operation] was cancelled. If the
    /// cancellation is in progress, cancelled will be true but
    /// [google.longrunning.Operation.done][google.longrunning.Operation.done] will be false.
    #[prost(bool, tag = "5")]
    pub cancelled: bool,
    /// Progress of the existing operation, measured in number of documents.
    #[prost(message, optional, tag = "6")]
    pub document_progress: ::std::option::Option<Progress>,
}
pub mod index_operation_metadata {
    /// The type of index operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OperationType {
        /// Unspecified. Never set by server.
        Unspecified = 0,
        /// The operation is creating the index. Initiated by a `CreateIndex` call.
        CreatingIndex = 1,
    }
}
/// Measures the progress of a particular metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Progress {
    /// An estimate of how much work has been completed. Note that this may be
    /// greater than `work_estimated`.
    #[prost(int64, tag = "1")]
    pub work_completed: i64,
    /// An estimate of how much work needs to be performed. Zero if the
    /// work estimate is unavailable. May change as work progresses.
    #[prost(int64, tag = "2")]
    pub work_estimated: i64,
}
/// The request for [FirestoreAdmin.CreateIndex][google.firestore.admin.v1beta1.FirestoreAdmin.CreateIndex].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIndexRequest {
    /// The name of the database this index will apply to. For example:
    /// `projects/{project_id}/databases/{database_id}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The index to create. The name and state fields are output only and will be
    /// ignored. Certain single field indexes cannot be created or deleted.
    #[prost(message, optional, tag = "2")]
    pub index: ::std::option::Option<Index>,
}
/// The request for [FirestoreAdmin.GetIndex][google.firestore.admin.v1beta1.FirestoreAdmin.GetIndex].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIndexRequest {
    /// The name of the index. For example:
    /// `projects/{project_id}/databases/{database_id}/indexes/{index_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for [FirestoreAdmin.ListIndexes][google.firestore.admin.v1beta1.FirestoreAdmin.ListIndexes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIndexesRequest {
    /// The database name. For example:
    /// `projects/{project_id}/databases/{database_id}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// The standard List page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard List page token.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The request for [FirestoreAdmin.DeleteIndex][google.firestore.admin.v1beta1.FirestoreAdmin.DeleteIndex].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIndexRequest {
    /// The index name. For example:
    /// `projects/{project_id}/databases/{database_id}/indexes/{index_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The response for [FirestoreAdmin.ListIndexes][google.firestore.admin.v1beta1.FirestoreAdmin.ListIndexes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIndexesResponse {
    /// The indexes.
    #[prost(message, repeated, tag = "1")]
    pub indexes: ::std::vec::Vec<Index>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for [FirestoreAdmin.ExportDocuments][google.firestore.admin.v1beta1.FirestoreAdmin.ExportDocuments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDocumentsRequest {
    /// Database to export. Should be of the form:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Which collection ids to export. Unspecified means all collections.
    #[prost(string, repeated, tag = "3")]
    pub collection_ids: ::std::vec::Vec<std::string::String>,
    /// The output URI. Currently only supports Google Cloud Storage URIs of the
    /// form: `gs://BUCKET_NAME[/NAMESPACE_PATH]`, where `BUCKET_NAME` is the name
    /// of the Google Cloud Storage bucket and `NAMESPACE_PATH` is an optional
    /// Google Cloud Storage namespace path. When
    /// choosing a name, be sure to consider Google Cloud Storage naming
    /// guidelines: https://cloud.google.com/storage/docs/naming.
    /// If the URI is a bucket (without a namespace path), a prefix will be
    /// generated based on the start time.
    #[prost(string, tag = "4")]
    pub output_uri_prefix: std::string::String,
}
/// The request for [FirestoreAdmin.ImportDocuments][google.firestore.admin.v1beta1.FirestoreAdmin.ImportDocuments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsRequest {
    /// Database to import into. Should be of the form:
    /// `projects/{project_id}/databases/{database_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Which collection ids to import. Unspecified means all collections included
    /// in the import.
    #[prost(string, repeated, tag = "3")]
    pub collection_ids: ::std::vec::Vec<std::string::String>,
    /// Location of the exported files.
    /// This must match the output_uri_prefix of an ExportDocumentsResponse from
    /// an export that has completed successfully.
    /// See:
    /// [google.firestore.admin.v1beta1.ExportDocumentsResponse.output_uri_prefix][google.firestore.admin.v1beta1.ExportDocumentsResponse.output_uri_prefix].
    #[prost(string, tag = "4")]
    pub input_uri_prefix: std::string::String,
}
/// Returned in the [google.longrunning.Operation][google.longrunning.Operation] response field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDocumentsResponse {
    /// Location of the output files. This can be used to begin an import
    /// into Cloud Firestore (this project or another project) after the operation
    /// completes successfully.
    #[prost(string, tag = "1")]
    pub output_uri_prefix: std::string::String,
}
/// Metadata for ExportDocuments operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDocumentsMetadata {
    /// The time that work began on the operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time the operation ended, either successfully or otherwise. Unset if
    /// the operation is still active.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The state of the export operation.
    #[prost(enumeration = "OperationState", tag = "3")]
    pub operation_state: i32,
    /// An estimate of the number of documents processed.
    #[prost(message, optional, tag = "4")]
    pub progress_documents: ::std::option::Option<Progress>,
    /// An estimate of the number of bytes processed.
    #[prost(message, optional, tag = "5")]
    pub progress_bytes: ::std::option::Option<Progress>,
    /// Which collection ids are being exported.
    #[prost(string, repeated, tag = "6")]
    pub collection_ids: ::std::vec::Vec<std::string::String>,
    /// Where the entities are being exported to.
    #[prost(string, tag = "7")]
    pub output_uri_prefix: std::string::String,
}
/// Metadata for ImportDocuments operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDocumentsMetadata {
    /// The time that work began on the operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time the operation ended, either successfully or otherwise. Unset if
    /// the operation is still active.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The state of the import operation.
    #[prost(enumeration = "OperationState", tag = "3")]
    pub operation_state: i32,
    /// An estimate of the number of documents processed.
    #[prost(message, optional, tag = "4")]
    pub progress_documents: ::std::option::Option<Progress>,
    /// An estimate of the number of bytes processed.
    #[prost(message, optional, tag = "5")]
    pub progress_bytes: ::std::option::Option<Progress>,
    /// Which collection ids are being imported.
    #[prost(string, repeated, tag = "6")]
    pub collection_ids: ::std::vec::Vec<std::string::String>,
    /// The location of the documents being imported.
    #[prost(string, tag = "7")]
    pub input_uri_prefix: std::string::String,
}
/// The various possible states for an ongoing Operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationState {
    /// Unspecified.
    StateUnspecified = 0,
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
#[doc = r" Generated client implementations."]
pub mod firestore_admin_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Cloud Firestore Admin API."]
    #[doc = ""]
    #[doc = " This API provides several administrative services for Cloud Firestore."]
    #[doc = ""]
    #[doc = " # Concepts"]
    #[doc = ""]
    #[doc = " Project, Database, Namespace, Collection, and Document are used as defined in"]
    #[doc = " the Google Cloud Firestore API."]
    #[doc = ""]
    #[doc = " Operation: An Operation represents work being performed in the background."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " # Services"]
    #[doc = ""]
    #[doc = " ## Index"]
    #[doc = ""]
    #[doc = " The index service manages Cloud Firestore indexes."]
    #[doc = ""]
    #[doc = " Index creation is performed asynchronously."]
    #[doc = " An Operation resource is created for each such asynchronous operation."]
    #[doc = " The state of the operation (including any errors encountered)"]
    #[doc = " may be queried via the Operation resource."]
    #[doc = ""]
    #[doc = " ## Metadata"]
    #[doc = ""]
    #[doc = " Provides metadata and statistical information about data in Cloud Firestore."]
    #[doc = " The data provided as part of this API may be stale."]
    #[doc = ""]
    #[doc = " ## Operation"]
    #[doc = ""]
    #[doc = " The Operations collection provides a record of actions performed for the"]
    #[doc = " specified Project (including any Operations in progress). Operations are not"]
    #[doc = " created directly but through calls on other collections or resources."]
    #[doc = ""]
    #[doc = " An Operation that is not yet done may be cancelled. The request to cancel is"]
    #[doc = " asynchronous and the Operation may continue to run for some time after the"]
    #[doc = " request to cancel is made."]
    #[doc = ""]
    #[doc = " An Operation that is done may be deleted so that it is no longer listed as"]
    #[doc = " part of the Operation collection."]
    #[doc = ""]
    #[doc = " Operations are created by service `FirestoreAdmin`, but are accessed via"]
    #[doc = " service `google.longrunning.Operations`."]
    pub struct FirestoreAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FirestoreAdminClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> FirestoreAdminClient<T>
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
        #[doc = " Creates the specified index."]
        #[doc = " A newly created index's initial state is `CREATING`. On completion of the"]
        #[doc = " returned [google.longrunning.Operation][google.longrunning.Operation], the state will be `READY`."]
        #[doc = " If the index already exists, the call will return an `ALREADY_EXISTS`"]
        #[doc = " status."]
        #[doc = ""]
        #[doc = " During creation, the process could result in an error, in which case the"]
        #[doc = " index will move to the `ERROR` state. The process can be recovered by"]
        #[doc = " fixing the data that caused the error, removing the index with"]
        #[doc = " [delete][google.firestore.admin.v1beta1.FirestoreAdmin.DeleteIndex], then re-creating the index with"]
        #[doc = " [create][google.firestore.admin.v1beta1.FirestoreAdmin.CreateIndex]."]
        #[doc = ""]
        #[doc = " Indexes with a single field cannot be created."]
        pub async fn create_index(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIndexRequest>,
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/CreateIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the indexes that match the specified filters."]
        pub async fn list_indexes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIndexesRequest>,
        ) -> Result<tonic::Response<super::ListIndexesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.admin.v1beta1.FirestoreAdmin/ListIndexes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets an index."]
        pub async fn get_index(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIndexRequest>,
        ) -> Result<tonic::Response<super::Index>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.admin.v1beta1.FirestoreAdmin/GetIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an index."]
        pub async fn delete_index(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIndexRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.firestore.admin.v1beta1.FirestoreAdmin/DeleteIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports a copy of all or a subset of documents from Google Cloud Firestore"]
        #[doc = " to another storage system, such as Google Cloud Storage. Recent updates to"]
        #[doc = " documents may not be reflected in the export. The export occurs in the"]
        #[doc = " background and its progress can be monitored and managed via the"]
        #[doc = " Operation resource that is created. The output of an export may only be"]
        #[doc = " used once the associated operation is done. If an export operation is"]
        #[doc = " cancelled before completion it may leave partial data behind in Google"]
        #[doc = " Cloud Storage."]
        pub async fn export_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportDocumentsRequest>,
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/ExportDocuments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Imports documents into Google Cloud Firestore. Existing documents with the"]
        #[doc = " same name are overwritten. The import occurs in the background and its"]
        #[doc = " progress can be monitored and managed via the Operation resource that is"]
        #[doc = " created. If an ImportDocuments operation is cancelled, it is possible"]
        #[doc = " that a subset of the data has already been imported to Cloud Firestore."]
        pub async fn import_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportDocumentsRequest>,
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/ImportDocuments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for FirestoreAdminClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for FirestoreAdminClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FirestoreAdminClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod firestore_admin_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with FirestoreAdminServer."]
    #[async_trait]
    pub trait FirestoreAdmin: Send + Sync + 'static {
        #[doc = " Creates the specified index."]
        #[doc = " A newly created index's initial state is `CREATING`. On completion of the"]
        #[doc = " returned [google.longrunning.Operation][google.longrunning.Operation], the state will be `READY`."]
        #[doc = " If the index already exists, the call will return an `ALREADY_EXISTS`"]
        #[doc = " status."]
        #[doc = ""]
        #[doc = " During creation, the process could result in an error, in which case the"]
        #[doc = " index will move to the `ERROR` state. The process can be recovered by"]
        #[doc = " fixing the data that caused the error, removing the index with"]
        #[doc = " [delete][google.firestore.admin.v1beta1.FirestoreAdmin.DeleteIndex], then re-creating the index with"]
        #[doc = " [create][google.firestore.admin.v1beta1.FirestoreAdmin.CreateIndex]."]
        #[doc = ""]
        #[doc = " Indexes with a single field cannot be created."]
        async fn create_index(
            &self,
            request: tonic::Request<super::CreateIndexRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Lists the indexes that match the specified filters."]
        async fn list_indexes(
            &self,
            request: tonic::Request<super::ListIndexesRequest>,
        ) -> Result<tonic::Response<super::ListIndexesResponse>, tonic::Status>;
        #[doc = " Gets an index."]
        async fn get_index(
            &self,
            request: tonic::Request<super::GetIndexRequest>,
        ) -> Result<tonic::Response<super::Index>, tonic::Status>;
        #[doc = " Deletes an index."]
        async fn delete_index(
            &self,
            request: tonic::Request<super::DeleteIndexRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Exports a copy of all or a subset of documents from Google Cloud Firestore"]
        #[doc = " to another storage system, such as Google Cloud Storage. Recent updates to"]
        #[doc = " documents may not be reflected in the export. The export occurs in the"]
        #[doc = " background and its progress can be monitored and managed via the"]
        #[doc = " Operation resource that is created. The output of an export may only be"]
        #[doc = " used once the associated operation is done. If an export operation is"]
        #[doc = " cancelled before completion it may leave partial data behind in Google"]
        #[doc = " Cloud Storage."]
        async fn export_documents(
            &self,
            request: tonic::Request<super::ExportDocumentsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Imports documents into Google Cloud Firestore. Existing documents with the"]
        #[doc = " same name are overwritten. The import occurs in the background and its"]
        #[doc = " progress can be monitored and managed via the Operation resource that is"]
        #[doc = " created. If an ImportDocuments operation is cancelled, it is possible"]
        #[doc = " that a subset of the data has already been imported to Cloud Firestore."]
        async fn import_documents(
            &self,
            request: tonic::Request<super::ImportDocumentsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " The Cloud Firestore Admin API."]
    #[doc = ""]
    #[doc = " This API provides several administrative services for Cloud Firestore."]
    #[doc = ""]
    #[doc = " # Concepts"]
    #[doc = ""]
    #[doc = " Project, Database, Namespace, Collection, and Document are used as defined in"]
    #[doc = " the Google Cloud Firestore API."]
    #[doc = ""]
    #[doc = " Operation: An Operation represents work being performed in the background."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " # Services"]
    #[doc = ""]
    #[doc = " ## Index"]
    #[doc = ""]
    #[doc = " The index service manages Cloud Firestore indexes."]
    #[doc = ""]
    #[doc = " Index creation is performed asynchronously."]
    #[doc = " An Operation resource is created for each such asynchronous operation."]
    #[doc = " The state of the operation (including any errors encountered)"]
    #[doc = " may be queried via the Operation resource."]
    #[doc = ""]
    #[doc = " ## Metadata"]
    #[doc = ""]
    #[doc = " Provides metadata and statistical information about data in Cloud Firestore."]
    #[doc = " The data provided as part of this API may be stale."]
    #[doc = ""]
    #[doc = " ## Operation"]
    #[doc = ""]
    #[doc = " The Operations collection provides a record of actions performed for the"]
    #[doc = " specified Project (including any Operations in progress). Operations are not"]
    #[doc = " created directly but through calls on other collections or resources."]
    #[doc = ""]
    #[doc = " An Operation that is not yet done may be cancelled. The request to cancel is"]
    #[doc = " asynchronous and the Operation may continue to run for some time after the"]
    #[doc = " request to cancel is made."]
    #[doc = ""]
    #[doc = " An Operation that is done may be deleted so that it is no longer listed as"]
    #[doc = " part of the Operation collection."]
    #[doc = ""]
    #[doc = " Operations are created by service `FirestoreAdmin`, but are accessed via"]
    #[doc = " service `google.longrunning.Operations`."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct FirestoreAdminServer<T: FirestoreAdmin> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: FirestoreAdmin> FirestoreAdminServer<T> {
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
    impl<T, B> Service<http::Request<B>> for FirestoreAdminServer<T>
    where
        T: FirestoreAdmin,
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/CreateIndex" => {
                    #[allow(non_camel_case_types)]
                    struct CreateIndexSvc<T: FirestoreAdmin>(pub Arc<T>);
                    impl<T: FirestoreAdmin> tonic::server::UnaryService<super::CreateIndexRequest>
                        for CreateIndexSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateIndexRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_index(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateIndexSvc(inner);
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/ListIndexes" => {
                    #[allow(non_camel_case_types)]
                    struct ListIndexesSvc<T: FirestoreAdmin>(pub Arc<T>);
                    impl<T: FirestoreAdmin> tonic::server::UnaryService<super::ListIndexesRequest>
                        for ListIndexesSvc<T>
                    {
                        type Response = super::ListIndexesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListIndexesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_indexes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListIndexesSvc(inner);
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/GetIndex" => {
                    #[allow(non_camel_case_types)]
                    struct GetIndexSvc<T: FirestoreAdmin>(pub Arc<T>);
                    impl<T: FirestoreAdmin> tonic::server::UnaryService<super::GetIndexRequest> for GetIndexSvc<T> {
                        type Response = super::Index;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetIndexRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_index(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetIndexSvc(inner);
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/DeleteIndex" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteIndexSvc<T: FirestoreAdmin>(pub Arc<T>);
                    impl<T: FirestoreAdmin> tonic::server::UnaryService<super::DeleteIndexRequest>
                        for DeleteIndexSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteIndexRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_index(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteIndexSvc(inner);
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/ExportDocuments" => {
                    #[allow(non_camel_case_types)]
                    struct ExportDocumentsSvc<T: FirestoreAdmin>(pub Arc<T>);
                    impl<T: FirestoreAdmin>
                        tonic::server::UnaryService<super::ExportDocumentsRequest>
                        for ExportDocumentsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportDocumentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.export_documents(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ExportDocumentsSvc(inner);
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
                "/google.firestore.admin.v1beta1.FirestoreAdmin/ImportDocuments" => {
                    #[allow(non_camel_case_types)]
                    struct ImportDocumentsSvc<T: FirestoreAdmin>(pub Arc<T>);
                    impl<T: FirestoreAdmin>
                        tonic::server::UnaryService<super::ImportDocumentsRequest>
                        for ImportDocumentsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportDocumentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.import_documents(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ImportDocumentsSvc(inner);
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
    impl<T: FirestoreAdmin> Clone for FirestoreAdminServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: FirestoreAdmin> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: FirestoreAdmin> tonic::transport::NamedService for FirestoreAdminServer<T> {
        const NAME: &'static str = "google.firestore.admin.v1beta1.FirestoreAdmin";
    }
}
/// The metadata message for [google.cloud.location.Location.metadata][google.cloud.location.Location.metadata].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {}
