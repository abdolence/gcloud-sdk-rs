/// Datastore composite index definition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Index {
    /// Output only. Project ID.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. The resource ID of the index.
    #[prost(string, tag = "3")]
    pub index_id: ::prost::alloc::string::String,
    /// Required. The entity kind to which this index applies.
    #[prost(string, tag = "4")]
    pub kind: ::prost::alloc::string::String,
    /// Required. The index's ancestor mode.  Must not be ANCESTOR_MODE_UNSPECIFIED.
    #[prost(enumeration = "index::AncestorMode", tag = "5")]
    pub ancestor: i32,
    /// Required. An ordered sequence of property names and their index attributes.
    #[prost(message, repeated, tag = "6")]
    pub properties: ::prost::alloc::vec::Vec<index::IndexedProperty>,
    /// Output only. The state of the index.
    #[prost(enumeration = "index::State", tag = "7")]
    pub state: i32,
}
/// Nested message and enum types in `Index`.
pub mod index {
    /// A property of an index.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IndexedProperty {
        /// Required. The property name to index.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Required. The indexed property's direction.  Must not be DIRECTION_UNSPECIFIED.
        #[prost(enumeration = "Direction", tag = "2")]
        pub direction: i32,
    }
    /// For an ordered index, specifies whether each of the entity's ancestors
    /// will be included.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AncestorMode {
        /// The ancestor mode is unspecified.
        Unspecified = 0,
        /// Do not include the entity's ancestors in the index.
        None = 1,
        /// Include all the entity's ancestors in the index.
        AllAncestors = 2,
    }
    /// The direction determines how a property is indexed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        /// The direction is unspecified.
        Unspecified = 0,
        /// The property's values are indexed so as to support sequencing in
        /// ascending order and also query by <, >, <=, >=, and =.
        Ascending = 1,
        /// The property's values are indexed so as to support sequencing in
        /// descending order and also query by <, >, <=, >=, and =.
        Descending = 2,
    }
    /// The possible set of states of an index.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state is unspecified.
        Unspecified = 0,
        /// The index is being created, and cannot be used by queries.
        /// There is an active long-running operation for the index.
        /// The index is updated when writing an entity.
        /// Some index data may exist.
        Creating = 1,
        /// The index is ready to be used.
        /// The index is updated when writing an entity.
        /// The index is fully populated from all stored entities it applies to.
        Ready = 2,
        /// The index is being deleted, and cannot be used by queries.
        /// There is an active long-running operation for the index.
        /// The index is not updated when writing an entity.
        /// Some index data may exist.
        Deleting = 3,
        /// The index was being created or deleted, but something went wrong.
        /// The index cannot by used by queries.
        /// There is no active long-running operation for the index,
        /// and the most recently finished long-running operation failed.
        /// The index is not updated when writing an entity.
        /// Some index data may exist.
        Error = 4,
    }
}
/// Metadata common to all Datastore Admin operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonMetadata {
    /// The time that work began on the operation.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation ended, either successfully or otherwise.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The type of the operation. Can be used as a filter in
    /// ListOperationsRequest.
    #[prost(enumeration = "OperationType", tag = "3")]
    pub operation_type: i32,
    /// The client-assigned labels which were provided when the operation was
    /// created. May also include additional labels.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The current state of the Operation.
    #[prost(enumeration = "common_metadata::State", tag = "5")]
    pub state: i32,
}
/// Nested message and enum types in `CommonMetadata`.
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
/// \[google.datastore.admin.v1.DatastoreAdmin.ExportEntities][google.datastore.admin.v1.DatastoreAdmin.ExportEntities\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesRequest {
    /// Required. Project ID against which to make the request.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Client-assigned labels.
    #[prost(map = "string, string", tag = "2")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Description of what data from the project is included in the export.
    #[prost(message, optional, tag = "3")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
    /// Required. Location for the export metadata and data files.
    ///
    /// The full resource URL of the external storage location. Currently, only
    /// Google Cloud Storage is supported. So output_url_prefix should be of the
    /// form: `gs://BUCKET_NAME\[/NAMESPACE_PATH\]`, where `BUCKET_NAME` is the
    /// name of the Cloud Storage bucket and `NAMESPACE_PATH` is an optional Cloud
    /// Storage namespace path (this is not a Cloud Datastore namespace). For more
    /// information about Cloud Storage namespace paths, see
    /// [Object name
    /// considerations](<https://cloud.google.com/storage/docs/naming#object-considerations>).
    ///
    /// The resulting files will be nested deeper than the specified URL prefix.
    /// The final output URL will be provided in the
    /// \[google.datastore.admin.v1.ExportEntitiesResponse.output_url][google.datastore.admin.v1.ExportEntitiesResponse.output_url\] field. That
    /// value should be used for subsequent ImportEntities operations.
    ///
    /// By nesting the data files deeper, the same Cloud Storage bucket can be used
    /// in multiple ExportEntities operations without conflict.
    #[prost(string, tag = "4")]
    pub output_url_prefix: ::prost::alloc::string::String,
}
/// The request for
/// \[google.datastore.admin.v1.DatastoreAdmin.ImportEntities][google.datastore.admin.v1.DatastoreAdmin.ImportEntities\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntitiesRequest {
    /// Required. Project ID against which to make the request.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Client-assigned labels.
    #[prost(map = "string, string", tag = "2")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. The full resource URL of the external storage location. Currently, only
    /// Google Cloud Storage is supported. So input_url should be of the form:
    /// `gs://BUCKET_NAME\[/NAMESPACE_PATH\]/OVERALL_EXPORT_METADATA_FILE`, where
    /// `BUCKET_NAME` is the name of the Cloud Storage bucket, `NAMESPACE_PATH` is
    /// an optional Cloud Storage namespace path (this is not a Cloud Datastore
    /// namespace), and `OVERALL_EXPORT_METADATA_FILE` is the metadata file written
    /// by the ExportEntities operation. For more information about Cloud Storage
    /// namespace paths, see
    /// [Object name
    /// considerations](<https://cloud.google.com/storage/docs/naming#object-considerations>).
    ///
    /// For more information, see
    /// \[google.datastore.admin.v1.ExportEntitiesResponse.output_url][google.datastore.admin.v1.ExportEntitiesResponse.output_url\].
    #[prost(string, tag = "3")]
    pub input_url: ::prost::alloc::string::String,
    /// Optionally specify which kinds/namespaces are to be imported. If provided,
    /// the list must be a subset of the EntityFilter used in creating the export,
    /// otherwise a FAILED_PRECONDITION error will be returned. If no filter is
    /// specified then all entities from the export are imported.
    #[prost(message, optional, tag = "4")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
}
/// The response for
/// \[google.datastore.admin.v1.DatastoreAdmin.ExportEntities][google.datastore.admin.v1.DatastoreAdmin.ExportEntities\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesResponse {
    /// Location of the output metadata file. This can be used to begin an import
    /// into Cloud Datastore (this project or another project). See
    /// \[google.datastore.admin.v1.ImportEntitiesRequest.input_url][google.datastore.admin.v1.ImportEntitiesRequest.input_url\].
    /// Only present if the operation completed successfully.
    #[prost(string, tag = "1")]
    pub output_url: ::prost::alloc::string::String,
}
/// Metadata for ExportEntities operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportEntitiesMetadata {
    /// Metadata common to all Datastore Admin operations.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonMetadata>,
    /// An estimate of the number of entities processed.
    #[prost(message, optional, tag = "2")]
    pub progress_entities: ::core::option::Option<Progress>,
    /// An estimate of the number of bytes processed.
    #[prost(message, optional, tag = "3")]
    pub progress_bytes: ::core::option::Option<Progress>,
    /// Description of which entities are being exported.
    #[prost(message, optional, tag = "4")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
    /// Location for the export metadata and data files. This will be the same
    /// value as the
    /// \[google.datastore.admin.v1.ExportEntitiesRequest.output_url_prefix][google.datastore.admin.v1.ExportEntitiesRequest.output_url_prefix\]
    /// field. The final output location is provided in
    /// \[google.datastore.admin.v1.ExportEntitiesResponse.output_url][google.datastore.admin.v1.ExportEntitiesResponse.output_url\].
    #[prost(string, tag = "5")]
    pub output_url_prefix: ::prost::alloc::string::String,
}
/// Metadata for ImportEntities operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportEntitiesMetadata {
    /// Metadata common to all Datastore Admin operations.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonMetadata>,
    /// An estimate of the number of entities processed.
    #[prost(message, optional, tag = "2")]
    pub progress_entities: ::core::option::Option<Progress>,
    /// An estimate of the number of bytes processed.
    #[prost(message, optional, tag = "3")]
    pub progress_bytes: ::core::option::Option<Progress>,
    /// Description of which entities are being imported.
    #[prost(message, optional, tag = "4")]
    pub entity_filter: ::core::option::Option<EntityFilter>,
    /// The location of the import metadata file. This will be the same value as
    /// the \[google.datastore.admin.v1.ExportEntitiesResponse.output_url][google.datastore.admin.v1.ExportEntitiesResponse.output_url\] field.
    #[prost(string, tag = "5")]
    pub input_url: ::prost::alloc::string::String,
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
///   kinds=['Foo', 'Bar'], namespace_ids=\[''\]
///
/// Kinds Foo and Bar in both the default and Baz namespaces:
///   kinds=['Foo', 'Bar'], namespace_ids=['', 'Baz']
///
/// The entire Baz namespace:
///   kinds=[], namespace_ids=\['Baz'\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityFilter {
    /// If empty, then this represents all kinds.
    #[prost(string, repeated, tag = "1")]
    pub kinds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An empty list represents all namespaces. This is the preferred
    /// usage for projects that don't use namespaces.
    ///
    /// An empty string element represents the default namespace. This should be
    /// used if the project has data in non-default namespaces, but doesn't want to
    /// include them.
    /// Each namespace in this list must be unique.
    #[prost(string, repeated, tag = "2")]
    pub namespace_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request for
/// \[google.datastore.admin.v1.DatastoreAdmin.CreateIndex][google.datastore.admin.v1.DatastoreAdmin.CreateIndex\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIndexRequest {
    /// Project ID against which to make the request.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The index to create. The name and state fields are output only and will be
    /// ignored. Single property indexes cannot be created or deleted.
    #[prost(message, optional, tag = "3")]
    pub index: ::core::option::Option<Index>,
}
/// The request for
/// \[google.datastore.admin.v1.DatastoreAdmin.DeleteIndex][google.datastore.admin.v1.DatastoreAdmin.DeleteIndex\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIndexRequest {
    /// Project ID against which to make the request.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The resource ID of the index to delete.
    #[prost(string, tag = "3")]
    pub index_id: ::prost::alloc::string::String,
}
/// The request for \[google.datastore.admin.v1.DatastoreAdmin.GetIndex][google.datastore.admin.v1.DatastoreAdmin.GetIndex\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIndexRequest {
    /// Project ID against which to make the request.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The resource ID of the index to get.
    #[prost(string, tag = "3")]
    pub index_id: ::prost::alloc::string::String,
}
/// The request for
/// \[google.datastore.admin.v1.DatastoreAdmin.ListIndexes][google.datastore.admin.v1.DatastoreAdmin.ListIndexes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIndexesRequest {
    /// Project ID against which to make the request.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of items to return.  If zero, then all results will be
    /// returned.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for
/// \[google.datastore.admin.v1.DatastoreAdmin.ListIndexes][google.datastore.admin.v1.DatastoreAdmin.ListIndexes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIndexesResponse {
    /// The indexes.
    #[prost(message, repeated, tag = "1")]
    pub indexes: ::prost::alloc::vec::Vec<Index>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Metadata for Index operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexOperationMetadata {
    /// Metadata common to all Datastore Admin operations.
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<CommonMetadata>,
    /// An estimate of the number of entities processed.
    #[prost(message, optional, tag = "2")]
    pub progress_entities: ::core::option::Option<Progress>,
    /// The index resource ID that this operation is acting on.
    #[prost(string, tag = "3")]
    pub index_id: ::prost::alloc::string::String,
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
    /// CreateIndex.
    CreateIndex = 3,
    /// DeleteIndex.
    DeleteIndex = 4,
}
#[doc = r" Generated client implementations."]
pub mod datastore_admin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Google Cloud Datastore Admin API"]
    #[doc = ""]
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
    #[doc = " # Index"]
    #[doc = ""]
    #[doc = " The index service manages Cloud Datastore composite indexes."]
    #[doc = ""]
    #[doc = " Index creation and deletion are performed asynchronously."]
    #[doc = " An Operation resource is created for each such asynchronous operation."]
    #[doc = " The state of the operation (including any errors encountered)"]
    #[doc = " may be queried via the Operation resource."]
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
    #[derive(Debug, Clone)]
    pub struct DatastoreAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DatastoreAdminClient<T>
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
        ) -> DatastoreAdminClient<InterceptedService<T, F>>
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
            DatastoreAdminClient::new(InterceptedService::new(inner, interceptor))
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
                "/google.datastore.admin.v1.DatastoreAdmin/ExportEntities",
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
                "/google.datastore.admin.v1.DatastoreAdmin/ImportEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates the specified index."]
        #[doc = " A newly created index's initial state is `CREATING`. On completion of the"]
        #[doc = " returned [google.longrunning.Operation][google.longrunning.Operation], the state will be `READY`."]
        #[doc = " If the index already exists, the call will return an `ALREADY_EXISTS`"]
        #[doc = " status."]
        #[doc = ""]
        #[doc = " During index creation, the process could result in an error, in which"]
        #[doc = " case the index will move to the `ERROR` state. The process can be recovered"]
        #[doc = " by fixing the data that caused the error, removing the index with"]
        #[doc = " [delete][google.datastore.admin.v1.DatastoreAdmin.DeleteIndex], then"]
        #[doc = " re-creating the index with [create]"]
        #[doc = " [google.datastore.admin.v1.DatastoreAdmin.CreateIndex]."]
        #[doc = ""]
        #[doc = " Indexes with a single property cannot be created."]
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
                "/google.datastore.admin.v1.DatastoreAdmin/CreateIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing index."]
        #[doc = " An index can only be deleted if it is in a `READY` or `ERROR` state. On"]
        #[doc = " successful execution of the request, the index will be in a `DELETING`"]
        #[doc = " [state][google.datastore.admin.v1.Index.State]. And on completion of the"]
        #[doc = " returned [google.longrunning.Operation][google.longrunning.Operation], the index will be removed."]
        #[doc = ""]
        #[doc = " During index deletion, the process could result in an error, in which"]
        #[doc = " case the index will move to the `ERROR` state. The process can be recovered"]
        #[doc = " by fixing the data that caused the error, followed by calling"]
        #[doc = " [delete][google.datastore.admin.v1.DatastoreAdmin.DeleteIndex] again."]
        pub async fn delete_index(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIndexRequest>,
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
                "/google.datastore.admin.v1.DatastoreAdmin/DeleteIndex",
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
                "/google.datastore.admin.v1.DatastoreAdmin/GetIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the indexes that match the specified filters.  Datastore uses an"]
        #[doc = " eventually consistent query to fetch the list of indexes and may"]
        #[doc = " occasionally return stale results."]
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
                "/google.datastore.admin.v1.DatastoreAdmin/ListIndexes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// An event signifying a change in state of a [migration from Cloud Datastore to
/// Cloud Firestore in Datastore
/// mode](<https://cloud.google.com/datastore/docs/upgrade-to-firestore>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationStateEvent {
    /// The new state of the migration.
    #[prost(enumeration = "MigrationState", tag = "1")]
    pub state: i32,
}
/// An event signifying the start of a new step in a [migration from Cloud
/// Datastore to Cloud Firestore in Datastore
/// mode](<https://cloud.google.com/datastore/docs/upgrade-to-firestore>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationProgressEvent {
    /// The step that is starting.
    ///
    /// An event with step set to `START` indicates that the migration
    /// has been reverted back to the initial pre-migration state.
    #[prost(enumeration = "MigrationStep", tag = "1")]
    pub step: i32,
    /// Details about this step.
    #[prost(oneof = "migration_progress_event::StepDetails", tags = "2, 3")]
    pub step_details: ::core::option::Option<migration_progress_event::StepDetails>,
}
/// Nested message and enum types in `MigrationProgressEvent`.
pub mod migration_progress_event {
    /// Details for the `PREPARE` step.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrepareStepDetails {
        /// The concurrency mode this database will use when it reaches the
        /// `REDIRECT_WRITES` step.
        #[prost(enumeration = "ConcurrencyMode", tag = "1")]
        pub concurrency_mode: i32,
    }
    /// Details for the `REDIRECT_WRITES` step.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RedirectWritesStepDetails {
        /// Ths concurrency mode for this database.
        #[prost(enumeration = "ConcurrencyMode", tag = "1")]
        pub concurrency_mode: i32,
    }
    /// Concurrency modes for transactions in Cloud Firestore.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConcurrencyMode {
        /// Unspecified.
        Unspecified = 0,
        /// Pessimistic concurrency.
        Pessimistic = 1,
        /// Optimistic concurrency.
        Optimistic = 2,
    }
    /// Details about this step.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StepDetails {
        /// Details for the `PREPARE` step.
        #[prost(message, tag = "2")]
        PrepareStepDetails(PrepareStepDetails),
        /// Details for the `REDIRECT_WRITES` step.
        #[prost(message, tag = "3")]
        RedirectWritesStepDetails(RedirectWritesStepDetails),
    }
}
/// States for a migration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MigrationState {
    /// Unspecified.
    Unspecified = 0,
    /// The migration is running.
    Running = 1,
    /// The migration is paused.
    Paused = 2,
    /// The migration is complete.
    Complete = 3,
}
/// Steps in a migration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MigrationStep {
    /// Unspecified.
    Unspecified = 0,
    /// Pre-migration: the database is prepared for migration.
    Prepare = 6,
    /// Start of migration.
    Start = 1,
    /// Writes are applied synchronously to at least one replica.
    ApplyWritesSynchronously = 7,
    /// Data is copied to Cloud Firestore and then verified to match the data in
    /// Cloud Datastore.
    CopyAndVerify = 2,
    /// Eventually-consistent reads are redirected to Cloud Firestore.
    RedirectEventuallyConsistentReads = 3,
    /// Strongly-consistent reads are redirected to Cloud Firestore.
    RedirectStronglyConsistentReads = 4,
    /// Writes are redirected to Cloud Firestore.
    RedirectWrites = 5,
}
