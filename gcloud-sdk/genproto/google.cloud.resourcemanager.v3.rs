/// A folder in an organization's resource hierarchy, used to
/// organize that organization's resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Folder {
    /// Output only. The resource name of the folder.
    /// Its format is `folders/{folder_id}`, for example: "folders/1234".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The folder's parent's resource name.
    /// Updates to the folder's parent must be performed using
    /// \[MoveFolder][google.cloud.resourcemanager.v3.Folders.MoveFolder\].
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
    /// The folder's display name.
    /// A folder's display name must be unique amongst its siblings. For example,
    /// no two folders with the same parent can share the same display name.
    /// The display name must start and end with a letter or digit, may contain
    /// letters, digits, spaces, hyphens and underscores and can be no longer
    /// than 30 characters. This is captured by the regular expression:
    /// `\[\p{L}\p{N}\]([\p{L}\p{N}_- ]{0,28}\[\p{L}\p{N}\])?`.
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The lifecycle state of the folder.
    /// Updates to the state must be performed using
    /// \[DeleteFolder][google.cloud.resourcemanager.v3.Folders.DeleteFolder\] and
    /// \[UndeleteFolder][google.cloud.resourcemanager.v3.Folders.UndeleteFolder\].
    #[prost(enumeration="folder::State", tag="4")]
    pub state: i32,
    /// Output only. Timestamp when the folder was created.
    #[prost(message, optional, tag="5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the folder was last modified.
    #[prost(message, optional, tag="6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the folder was requested to be deleted.
    #[prost(message, optional, tag="7")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A checksum computed by the server based on the current value of the folder
    /// resource. This may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag="8")]
    pub etag: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Folder`.
pub mod folder {
    /// Folder lifecycle states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// The normal and active state.
        Active = 1,
        /// The folder has been marked for deletion by the user.
        DeleteRequested = 2,
    }
}
/// The GetFolder request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFolderRequest {
    /// Required. The resource name of the folder to retrieve.
    /// Must be of the form `folders/{folder_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The ListFolders request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFoldersRequest {
    /// Required. The resource name of the organization or folder whose folders are
    /// being listed.
    /// Must be of the form `folders/{folder_id}` or `organizations/{org_id}`.
    /// Access to this method is controlled by checking the
    /// `resourcemanager.folders.list` permission on the `parent`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of folders to return in the response.
    /// If unspecified, server picks an appropriate default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to `ListFolders`
    /// that indicates where this listing should continue from.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Controls whether folders in the
    /// \[DELETE_REQUESTED][google.cloud.resourcemanager.v3.Folder.State.DELETE_REQUESTED\]
    /// state should be returned. Defaults to false.
    #[prost(bool, tag="4")]
    pub show_deleted: bool,
}
/// The ListFolders response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFoldersResponse {
    /// A possibly paginated list of folders that are direct descendants of
    /// the specified parent resource.
    #[prost(message, repeated, tag="1")]
    pub folders: ::prost::alloc::vec::Vec<Folder>,
    /// A pagination token returned from a previous call to `ListFolders`
    /// that indicates from where listing should continue.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for searching folders.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchFoldersRequest {
    /// Optional. The maximum number of folders to return in the response.
    /// If unspecified, server picks an appropriate default.
    #[prost(int32, tag="1")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to `SearchFolders`
    /// that indicates from where search should continue.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Search criteria used to select the folders to return.
    /// If no search criteria is specified then all accessible folders will be
    /// returned.
    ///
    /// Query expressions can be used to restrict results based upon displayName,
    /// state and parent, where the operators `=` (`:`) `NOT`, `AND` and `OR`
    /// can be used along with the suffix wildcard symbol `*`.
    ///
    /// The `displayName` field in a query expression should use escaped quotes
    /// for values that include whitespace to prevent unexpected behavior.
    ///
    /// ```
    /// | Field                   | Description                            |
    /// |-------------------------|----------------------------------------|
    /// | displayName             | Filters by displayName.                |
    /// | parent                  | Filters by parent (for example: folders/123). |
    /// | state, lifecycleState   | Filters by state.                      |
    /// ```
    ///
    /// Some example queries are:
    ///
    /// * Query `displayName=Test*` returns Folder resources whose display name
    /// starts with "Test".
    /// * Query `state=ACTIVE` returns Folder resources with
    /// `state` set to `ACTIVE`.
    /// * Query `parent=folders/123` returns Folder resources that have
    /// `folders/123` as a parent resource.
    /// * Query `parent=folders/123 AND state=ACTIVE` returns active
    /// Folder resources that have `folders/123` as a parent resource.
    /// * Query `displayName=\\"Test String\\"` returns Folder resources with
    /// display names that include both "Test" and "String".
    #[prost(string, tag="3")]
    pub query: ::prost::alloc::string::String,
}
/// The response message for searching folders.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchFoldersResponse {
    /// A possibly paginated folder search results.
    /// the specified parent resource.
    #[prost(message, repeated, tag="1")]
    pub folders: ::prost::alloc::vec::Vec<Folder>,
    /// A pagination token returned from a previous call to `SearchFolders`
    /// that indicates from where searching should continue.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The CreateFolder request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFolderRequest {
    /// Required. The folder being created, only the display name and parent will be
    /// consulted. All other fields will be ignored.
    #[prost(message, optional, tag="2")]
    pub folder: ::core::option::Option<Folder>,
}
/// Metadata pertaining to the Folder creation process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFolderMetadata {
    /// The display name of the folder.
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    /// The resource name of the folder or organization we are creating the folder
    /// under.
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
}
/// The request sent to the
/// \[UpdateFolder][google.cloud.resourcemanager.v3.Folder.UpdateFolder\]
/// method.
///
/// Only the `display_name` field can be changed. All other fields will be
/// ignored. Use the
/// \[MoveFolder][google.cloud.resourcemanager.v3.Folders.MoveFolder\] method to
/// change the `parent` field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFolderRequest {
    /// Required. The new definition of the Folder. It must include the `name` field, which
    /// cannot be changed.
    #[prost(message, optional, tag="1")]
    pub folder: ::core::option::Option<Folder>,
    /// Required. Fields to be updated.
    /// Only the `display_name` can be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A status object which is used as the `metadata` field for the Operation
/// returned by UpdateFolder.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFolderMetadata {
}
/// The MoveFolder request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveFolderRequest {
    /// Required. The resource name of the Folder to move.
    /// Must be of the form folders/{folder_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The resource name of the folder or organization which should be the
    /// folder's new parent.
    /// Must be of the form `folders/{folder_id}` or `organizations/{org_id}`.
    #[prost(string, tag="2")]
    pub destination_parent: ::prost::alloc::string::String,
}
/// Metadata pertaining to the folder move process.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveFolderMetadata {
    /// The display name of the folder.
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    /// The resource name of the folder's parent.
    #[prost(string, tag="2")]
    pub source_parent: ::prost::alloc::string::String,
    /// The resource name of the folder or organization to move the folder to.
    #[prost(string, tag="3")]
    pub destination_parent: ::prost::alloc::string::String,
}
/// The DeleteFolder request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFolderRequest {
    /// Required. The resource name of the folder to be deleted.
    /// Must be of the form `folders/{folder_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A status object which is used as the `metadata` field for the `Operation`
/// returned by `DeleteFolder`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFolderMetadata {
}
/// The UndeleteFolder request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteFolderRequest {
    /// Required. The resource name of the folder to undelete.
    /// Must be of the form `folders/{folder_id}`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A status object which is used as the `metadata` field for the `Operation`
/// returned by `UndeleteFolder`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteFolderMetadata {
}
/// Generated client implementations.
pub mod folders_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Manages Cloud Platform folder resources.
    /// Folders can be used to organize the resources under an
    /// organization and to control the policies applied to groups of resources.
    #[derive(Debug, Clone)]
    pub struct FoldersClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FoldersClient<tonic::transport::Channel> {
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
    impl<T> FoldersClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> FoldersClient<InterceptedService<T, F>>
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
            FoldersClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Retrieves a folder identified by the supplied resource name.
        /// Valid folder resource names have the format `folders/{folder_id}`
        /// (for example, `folders/1234`).
        /// The caller must have `resourcemanager.folders.get` permission on the
        /// identified folder.
        pub async fn get_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::Folder>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.Folders/GetFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the folders that are direct descendants of supplied parent resource.
        /// `list()` provides a strongly consistent view of the folders underneath
        /// the specified parent resource.
        /// `list()` returns folders sorted based upon the (ascending) lexical ordering
        /// of their display_name.
        /// The caller must have `resourcemanager.folders.list` permission on the
        /// identified parent.
        pub async fn list_folders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFoldersRequest>,
        ) -> Result<tonic::Response<super::ListFoldersResponse>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.Folders/ListFolders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Search for folders that match specific filter criteria.
        /// `search()` provides an eventually consistent view of the folders a user has
        /// access to which meet the specified filter criteria.
        ///
        /// This will only return folders on which the caller has the
        /// permission `resourcemanager.folders.get`.
        pub async fn search_folders(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchFoldersRequest>,
        ) -> Result<tonic::Response<super::SearchFoldersResponse>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.Folders/SearchFolders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a folder in the resource hierarchy.
        /// Returns an `Operation` which can be used to track the progress of the
        /// folder creation workflow.
        /// Upon success, the `Operation.response` field will be populated with the
        /// created Folder.
        ///
        /// In order to succeed, the addition of this new folder must not violate
        /// the folder naming, height, or fanout constraints.
        ///
        /// + The folder's `display_name` must be distinct from all other folders that
        /// share its parent.
        /// + The addition of the folder must not cause the active folder hierarchy
        /// to exceed a height of 10. Note, the full active + deleted folder hierarchy
        /// is allowed to reach a height of 20; this provides additional headroom when
        /// moving folders that contain deleted folders.
        /// + The addition of the folder must not cause the total number of folders
        /// under its parent to exceed 300.
        ///
        /// If the operation fails due to a folder constraint violation, some errors
        /// may be returned by the `CreateFolder` request, with status code
        /// `FAILED_PRECONDITION` and an error description. Other folder constraint
        /// violations will be communicated in the `Operation`, with the specific
        /// `PreconditionFailure` returned in the details list in the `Operation.error`
        /// field.
        ///
        /// The caller must have `resourcemanager.folders.create` permission on the
        /// identified parent.
        pub async fn create_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFolderRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.Folders/CreateFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a folder, changing its `display_name`.
        /// Changes to the folder `display_name` will be rejected if they violate
        /// either the `display_name` formatting rules or the naming constraints
        /// described in the [CreateFolder][google.cloud.resourcemanager.v3.Folders.CreateFolder] documentation.
        ///
        /// The folder's `display_name` must start and end with a letter or digit,
        /// may contain letters, digits, spaces, hyphens and underscores and can be
        /// between 3 and 30 characters. This is captured by the regular expression:
        /// `[\p{L}\p{N}][\p{L}\p{N}_- ]{1,28}[\p{L}\p{N}]`.
        /// The caller must have `resourcemanager.folders.update` permission on the
        /// identified folder.
        ///
        /// If the update fails due to the unique name constraint then a
        /// `PreconditionFailure` explaining this violation will be returned
        /// in the Status.details field.
        pub async fn update_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFolderRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.Folders/UpdateFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Moves a folder under a new resource parent.
        /// Returns an `Operation` which can be used to track the progress of the
        /// folder move workflow.
        /// Upon success, the `Operation.response` field will be populated with the
        /// moved folder.
        /// Upon failure, a `FolderOperationError` categorizing the failure cause will
        /// be returned - if the failure occurs synchronously then the
        /// `FolderOperationError` will be returned in the `Status.details` field.
        /// If it occurs asynchronously, then the FolderOperation will be returned
        /// in the `Operation.error` field.
        /// In addition, the `Operation.metadata` field will be populated with a
        /// `FolderOperation` message as an aid to stateless clients.
        /// Folder moves will be rejected if they violate either the naming, height,
        /// or fanout constraints described in the
        /// [CreateFolder][google.cloud.resourcemanager.v3.Folders.CreateFolder] documentation.
        /// The caller must have `resourcemanager.folders.move` permission on the
        /// folder's current and proposed new parent.
        pub async fn move_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveFolderRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.Folders/MoveFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Requests deletion of a folder. The folder is moved into the
        /// [DELETE_REQUESTED][google.cloud.resourcemanager.v3.Folder.State.DELETE_REQUESTED] state
        /// immediately, and is deleted approximately 30 days later. This method may
        /// only be called on an empty folder, where a folder is empty if it doesn't
        /// contain any folders or projects in the [ACTIVE][google.cloud.resourcemanager.v3.Folder.State.ACTIVE] state.
        /// If called on a folder in [DELETE_REQUESTED][google.cloud.resourcemanager.v3.Folder.State.DELETE_REQUESTED]
        /// state the operation will result in a no-op success.
        /// The caller must have `resourcemanager.folders.delete` permission on the
        /// identified folder.
        pub async fn delete_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFolderRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.Folders/DeleteFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Cancels the deletion request for a folder. This method may be called on a
        /// folder in any state. If the folder is in the [ACTIVE][google.cloud.resourcemanager.v3.Folder.State.ACTIVE]
        /// state the result will be a no-op success. In order to succeed, the folder's
        /// parent must be in the [ACTIVE][google.cloud.resourcemanager.v3.Folder.State.ACTIVE] state. In addition,
        /// reintroducing the folder into the tree must not violate folder naming,
        /// height, and fanout constraints described in the
        /// [CreateFolder][google.cloud.resourcemanager.v3.Folders.CreateFolder] documentation.
        /// The caller must have `resourcemanager.folders.undelete` permission on the
        /// identified folder.
        pub async fn undelete_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteFolderRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.Folders/UndeleteFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy for a folder. The returned policy may be
        /// empty if no such policy or resource exists. The `resource` field should
        /// be the folder's resource name, for example: "folders/1234".
        /// The caller must have `resourcemanager.folders.getIamPolicy` permission
        /// on the identified folder.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v3.Folders/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the access control policy on a folder, replacing any existing policy.
        /// The `resource` field should be the folder's resource name, for example:
        /// "folders/1234".
        /// The caller must have `resourcemanager.folders.setIamPolicy` permission
        /// on the identified folder.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v3.Folders/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns permissions that a caller has on the specified folder.
        /// The `resource` field should be the folder's resource name,
        /// for example: "folders/1234".
        ///
        /// There are no permissions required for making this API call.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.resourcemanager.v3.Folders/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The root node in the resource hierarchy to which a particular entity's
/// (a company, for example) resources belong.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Organization {
    /// Output only. The resource name of the organization. This is the
    /// organization's relative path in the API. Its format is
    /// "organizations/\[organization_id\]". For example, "organizations/1234".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. A human-readable string that refers to the organization in the
    /// Google Cloud Console. This string is set by the server and cannot be
    /// changed. The string will be set to the primary domain (for example,
    /// "google.com") of the Google Workspace customer that owns the organization.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The organization's current lifecycle state.
    #[prost(enumeration="organization::State", tag="4")]
    pub state: i32,
    /// Output only. Timestamp when the Organization was created.
    #[prost(message, optional, tag="5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the Organization was last modified.
    #[prost(message, optional, tag="6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the Organization was requested for deletion.
    #[prost(message, optional, tag="7")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A checksum computed by the server based on the current value of the
    /// Organization resource. This may be sent on update and delete requests to
    /// ensure the client has an up-to-date value before proceeding.
    #[prost(string, tag="8")]
    pub etag: ::prost::alloc::string::String,
    /// The owner of this organization. The owner should be specified on
    /// creation. Once set, it cannot be changed.
    ///
    /// The lifetime of the organization and all of its descendants are bound to
    /// the owner. If the owner is deleted, the organization and all its
    /// descendants will be deleted.
    #[prost(oneof="organization::Owner", tags="3")]
    pub owner: ::core::option::Option<organization::Owner>,
}
/// Nested message and enum types in `Organization`.
pub mod organization {
    /// Organization lifecycle states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.  This is only useful for distinguishing unset values.
        Unspecified = 0,
        /// The normal and active state.
        Active = 1,
        /// The organization has been marked for deletion by the user.
        DeleteRequested = 2,
    }
    /// The owner of this organization. The owner should be specified on
    /// creation. Once set, it cannot be changed.
    ///
    /// The lifetime of the organization and all of its descendants are bound to
    /// the owner. If the owner is deleted, the organization and all its
    /// descendants will be deleted.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Owner {
        /// Immutable. The G Suite / Workspace customer id used in the Directory API.
        #[prost(string, tag="3")]
        DirectoryCustomerId(::prost::alloc::string::String),
    }
}
/// The request sent to the `GetOrganization` method. The `name` field is
/// required. `organization_id` is no longer accepted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationRequest {
    /// Required. The resource name of the Organization to fetch. This is the organization's
    /// relative path in the API, formatted as "organizations/\[organizationId\]".
    /// For example, "organizations/1234".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to the `SearchOrganizations` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOrganizationsRequest {
    /// Optional. The maximum number of organizations to return in the response.
    /// If unspecified, server picks an appropriate default.
    #[prost(int32, tag="1")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to `SearchOrganizations`
    /// that indicates from where listing should continue.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. An optional query string used to filter the Organizations to return in
    /// the response. Query rules are case-insensitive.
    ///
    /// ```
    /// | Field            | Description                                |
    /// |------------------|--------------------------------------------|
    /// | directoryCustomerId, owner.directoryCustomerId | Filters by directory
    /// customer id. |
    /// | domain           | Filters by domain.                         |
    /// ```
    ///
    /// Organizations may be queried by `directoryCustomerId` or by
    /// `domain`, where the domain is a G Suite domain, for example:
    ///
    /// * Query `directorycustomerid:123456789` returns Organization
    /// resources with `owner.directory_customer_id` equal to `123456789`.
    /// * Query `domain:google.com` returns Organization resources corresponding
    /// to the domain `google.com`.
    #[prost(string, tag="3")]
    pub query: ::prost::alloc::string::String,
}
/// The response returned from the `SearchOrganizations` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchOrganizationsResponse {
    /// The list of Organizations that matched the search query, possibly
    /// paginated.
    #[prost(message, repeated, tag="1")]
    pub organizations: ::prost::alloc::vec::Vec<Organization>,
    /// A pagination token to be used to retrieve the next page of results. If the
    /// result is too large to fit within the page size specified in the request,
    /// this field will be set with a token that can be used to fetch the next page
    /// of results. If this field is empty, it indicates that this response
    /// contains the last page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A status object which is used as the `metadata` field for the operation
/// returned by DeleteOrganization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationMetadata {
}
/// A status object which is used as the `metadata` field for the Operation
/// returned by UndeleteOrganization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteOrganizationMetadata {
}
/// Generated client implementations.
pub mod organizations_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Allows users to manage their organization resources.
    #[derive(Debug, Clone)]
    pub struct OrganizationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrganizationsClient<tonic::transport::Channel> {
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
    impl<T> OrganizationsClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrganizationsClient<InterceptedService<T, F>>
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
            OrganizationsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Fetches an organization resource identified by the specified resource name.
        pub async fn get_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrganizationRequest>,
        ) -> Result<tonic::Response<super::Organization>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.Organizations/GetOrganization",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Searches organization resources that are visible to the user and satisfy
        /// the specified filter. This method returns organizations in an unspecified
        /// order. New organizations do not necessarily appear at the end of the
        /// results, and may take a small amount of time to appear.
        ///
        /// Search will only return organizations on which the user has the permission
        /// `resourcemanager.organizations.get`
        pub async fn search_organizations(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchOrganizationsRequest>,
        ) -> Result<tonic::Response<super::SearchOrganizationsResponse>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.Organizations/SearchOrganizations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy for an organization resource. The policy may
        /// be empty if no such policy or resource exists. The `resource` field should
        /// be the organization's resource name, for example: "organizations/123".
        ///
        /// Authorization requires the IAM permission
        /// `resourcemanager.organizations.getIamPolicy` on the specified organization.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v3.Organizations/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the access control policy on an organization resource. Replaces any
        /// existing policy. The `resource` field should be the organization's resource
        /// name, for example: "organizations/123".
        ///
        /// Authorization requires the IAM permission
        /// `resourcemanager.organizations.setIamPolicy` on the specified organization.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v3.Organizations/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the permissions that a caller has on the specified organization.
        /// The `resource` field should be the organization's resource name,
        /// for example: "organizations/123".
        ///
        /// There are no permissions required for making this API call.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.resourcemanager.v3.Organizations/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A project is a high-level Google Cloud entity. It is a
/// container for ACLs, APIs, App Engine Apps, VMs, and other
/// Google Cloud Platform resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Project {
    /// Output only. The unique resource name of the project. It is an int64 generated number
    /// prefixed by "projects/".
    ///
    /// Example: `projects/415104041262`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A reference to a parent Resource. eg., `organizations/123` or
    /// `folders/876`.
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
    /// Immutable. The unique, user-assigned id of the project.
    /// It must be 6 to 30 lowercase ASCII letters, digits, or hyphens.
    /// It must start with a letter.
    /// Trailing hyphens are prohibited.
    ///
    /// Example: `tokyo-rain-123`
    #[prost(string, tag="3")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. The project lifecycle state.
    #[prost(enumeration="project::State", tag="4")]
    pub state: i32,
    /// Optional. A user-assigned display name of the project.
    /// When present it must be between 4 to 30 characters.
    /// Allowed characters are: lowercase and uppercase letters, numbers,
    /// hyphen, single-quote, double-quote, space, and exclamation point.
    ///
    /// Example: `My Project`
    #[prost(string, tag="5")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Creation time.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The most recent time this resource was modified.
    #[prost(message, optional, tag="7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this resource was requested for deletion.
    #[prost(message, optional, tag="8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A checksum computed by the server based on the current value of the Project
    /// resource. This may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag="9")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. The labels associated with this project.
    ///
    /// Label keys must be between 1 and 63 characters long and must conform
    /// to the following regular expression: \\[a-z\](\[-a-z0-9\]*\[a-z0-9\\])?.
    ///
    /// Label values must be between 0 and 63 characters long and must conform
    /// to the regular expression (\\[a-z\](\[-a-z0-9\]*\[a-z0-9\\])?)?.
    ///
    /// No more than 256 labels can be associated with a given resource.
    ///
    /// Clients should store labels in a representation such as JSON that does not
    /// depend on specific characters being disallowed.
    ///
    /// Example: `"myBusinessDimension" : "businessValue"`
    #[prost(map="string, string", tag="10")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `Project`.
pub mod project {
    /// Project lifecycle states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.  This is only used/useful for distinguishing
        /// unset values.
        Unspecified = 0,
        /// The normal and active state.
        Active = 1,
        /// The project has been marked for deletion by the user
        /// (by invoking
        /// \[DeleteProject][google.cloud.resourcemanager.v3.Projects.DeleteProject\])
        /// or by the system (Google Cloud Platform).
        /// This can generally be reversed by invoking \[UndeleteProject\]
        /// \[google.cloud.resourcemanager.v3.Projects.UndeleteProject\].
        DeleteRequested = 2,
    }
}
/// The request sent to the
/// \[GetProject][google.cloud.resourcemanager.v3.Projects.GetProject\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectRequest {
    /// Required. The name of the project (for example, `projects/415104041262`).
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to the
/// \[ListProjects][google.cloud.resourcemanager.v3.Projects.ListProjects\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectsRequest {
    /// Required. The name of the parent resource to list projects under.
    ///
    /// For example, setting this field to 'folders/1234' would list all projects
    /// directly under that folder.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. A pagination token returned from a previous call to \[ListProjects\]
    /// \[google.cloud.resourcemanager.v3.Projects.ListProjects\]
    /// that indicates from where listing should continue.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The maximum number of projects to return in the response.
    /// The server can return fewer projects than requested.
    /// If unspecified, server picks an appropriate default.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// Optional. Indicate that projects in the `DELETE_REQUESTED` state should also be
    /// returned. Normally only `ACTIVE` projects are returned.
    #[prost(bool, tag="4")]
    pub show_deleted: bool,
}
/// A page of the response received from the
/// \[ListProjects][google.cloud.resourcemanager.v3.Projects.ListProjects\]
/// method.
///
/// A paginated response where more pages are available has
/// `next_page_token` set. This token can be used in a subsequent request to
/// retrieve the next request page.
///
/// NOTE: A response may contain fewer elements than the request `page_size` and
/// still have a `next_page_token`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectsResponse {
    /// The list of Projects under the parent. This list can be paginated.
    #[prost(message, repeated, tag="1")]
    pub projects: ::prost::alloc::vec::Vec<Project>,
    /// Pagination token.
    ///
    /// If the result set is too large to fit in a single response, this token
    /// is returned. It encodes the position of the current result cursor.
    /// Feeding this value into a new list request with the `page_token` parameter
    /// gives the next page of the results.
    ///
    /// When `next_page_token` is not filled in, there is no next page and
    /// the list returned is the last page in the result set.
    ///
    /// Pagination tokens have a limited lifetime.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request sent to the
/// \[SearchProjects][google.cloud.resourcemanager.v3.Projects.SearchProjects\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchProjectsRequest {
    /// Optional. A query string for searching for projects that the caller has
    /// `resourcemanager.projects.get` permission to. If multiple fields are
    /// included in the query, the it will return results that match any of the
    /// fields. Some eligible fields are:
    ///
    /// ```
    /// | Field                   | Description                                  |
    /// |-------------------------|----------------------------------------------|
    /// | displayName, name       | Filters by displayName.                      |
    /// | parent                  | Project's parent. (for example: folders/123,
    /// organizations/*) Prefer parent field over parent.type and parent.id. |
    /// | parent.type             | Parent's type: `folder` or `organization`.   |
    /// | parent.id               | Parent's id number (for example: 123)        |
    /// | id, projectId           | Filters by projectId.                        |
    /// | state, lifecycleState   | Filters by state.                            |
    /// | labels                  | Filters by label name or value.              |
    /// | labels.<key> (where *key* is the name of a label) | Filters by label
    /// name. |
    /// ```
    ///
    /// Search expressions are case insensitive.
    ///
    /// Some examples queries:
    ///
    /// ```
    /// | Query            | Description                                         |
    /// |------------------|-----------------------------------------------------|
    /// | name:how*        | The project's name starts with "how".               |
    /// | name:Howl        | The project's name is `Howl` or `howl`.             |
    /// | name:HOWL        | Equivalent to above.                                |
    /// | NAME:howl        | Equivalent to above.                                |
    /// | labels.color:*   | The project has the label `color`.                  |
    /// | labels.color:red | The project's label `color` has the value `red`.    |
    /// | labels.color:red&nbsp;labels.size:big | The project's label `color` has
    /// the value `red` and its label `size` has the value `big`.                |
    /// ```
    ///
    /// If no query is specified, the call will return projects for which the user
    /// has the `resourcemanager.projects.get` permission.
    #[prost(string, tag="1")]
    pub query: ::prost::alloc::string::String,
    /// Optional. A pagination token returned from a previous call to \[ListProjects\]
    /// \[google.cloud.resourcemanager.v3.Projects.ListProjects\]
    /// that indicates from where listing should continue.
    #[prost(string, tag="2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The maximum number of projects to return in the response.
    /// The server can return fewer projects than requested.
    /// If unspecified, server picks an appropriate default.
    #[prost(int32, tag="3")]
    pub page_size: i32,
}
/// A page of the response received from the
/// \[SearchProjects][google.cloud.resourcemanager.v3.Projects.SearchProjects\]
/// method.
///
/// A paginated response where more pages are available has
/// `next_page_token` set. This token can be used in a subsequent request to
/// retrieve the next request page.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchProjectsResponse {
    /// The list of Projects that matched the list filter query. This list can
    /// be paginated.
    #[prost(message, repeated, tag="1")]
    pub projects: ::prost::alloc::vec::Vec<Project>,
    /// Pagination token.
    ///
    /// If the result set is too large to fit in a single response, this token
    /// is returned. It encodes the position of the current result cursor.
    /// Feeding this value into a new list request with the `page_token` parameter
    /// gives the next page of the results.
    ///
    /// When `next_page_token` is not filled in, there is no next page and
    /// the list returned is the last page in the result set.
    ///
    /// Pagination tokens have a limited lifetime.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request sent to the
/// \[CreateProject][google.cloud.resourcemanager.v3.Projects.CreateProject\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectRequest {
    /// Required. The Project to create.
    ///
    /// Project ID is required. If the requested ID is unavailable, the request
    /// fails.
    ///
    /// If the `parent` field is set, the `resourcemanager.projects.create`
    /// permission is checked on the parent resource. If no parent is set and
    /// the authorization credentials belong to an Organziation, the parent
    /// will be set to that Organization.
    #[prost(message, optional, tag="1")]
    pub project: ::core::option::Option<Project>,
}
/// A status object which is used as the `metadata` field for the Operation
/// returned by CreateProject. It provides insight for when significant phases of
/// Project creation have completed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProjectMetadata {
    /// Creation time of the project creation workflow.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// True if the project can be retrieved using `GetProject`. No other
    /// operations on the project are guaranteed to work until the project creation
    /// is complete.
    #[prost(bool, tag="2")]
    pub gettable: bool,
    /// True if the project creation process is complete.
    #[prost(bool, tag="3")]
    pub ready: bool,
}
/// The request sent to the
/// \[UpdateProject][google.cloud.resourcemanager.v3.Projects.UpdateProject\]
/// method.
///
/// Only the `display_name` and `labels` fields can be change. Use the
/// \[MoveProject][google.cloud.resourcemanager.v3.Projects.MoveProject\] method to
/// change the `parent` field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectRequest {
    /// Required. The new definition of the project.
    #[prost(message, optional, tag="1")]
    pub project: ::core::option::Option<Project>,
    /// Optional. An update mask to selectively update fields.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// A status object which is used as the `metadata` field for the Operation
/// returned by UpdateProject.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectMetadata {
}
/// The request sent to
/// \[MoveProject][google.cloud.resourcemanager.v3.Projects.MoveProject\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveProjectRequest {
    /// Required. The name of the project to move.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The new parent to move the Project under.
    #[prost(string, tag="2")]
    pub destination_parent: ::prost::alloc::string::String,
}
/// A status object which is used as the `metadata` field for the Operation
/// returned by MoveProject.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveProjectMetadata {
}
/// \[DeleteProject][google.cloud.resourcemanager.v3.Projects.DeleteProject\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProjectRequest {
    /// Required. The name of the Project (for example, `projects/415104041262`).
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A status object which is used as the `metadata` field for the Operation
/// returned by `DeleteProject`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProjectMetadata {
}
/// The request sent to the \[UndeleteProject\]
/// \[google.cloud.resourcemanager.v3.Projects.UndeleteProject\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteProjectRequest {
    /// Required. The name of the project (for example, `projects/415104041262`).
    ///
    /// Required.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A status object which is used as the `metadata` field for the Operation
/// returned by `UndeleteProject`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteProjectMetadata {
}
/// Generated client implementations.
pub mod projects_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Manages Google Cloud Projects.
    #[derive(Debug, Clone)]
    pub struct ProjectsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProjectsClient<tonic::transport::Channel> {
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
    impl<T> ProjectsClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ProjectsClient<InterceptedService<T, F>>
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
            ProjectsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Retrieves the project identified by the specified `name` (for example,
        /// `projects/415104041262`).
        ///
        /// The caller must have `resourcemanager.projects.get` permission
        /// for this project.
        pub async fn get_project(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectRequest>,
        ) -> Result<tonic::Response<super::Project>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.Projects/GetProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists projects that are direct children of the specified folder or
        /// organization resource. `list()` provides a strongly consistent view of the
        /// projects underneath the specified parent resource. `list()` returns
        /// projects sorted based upon the (ascending) lexical ordering of their
        /// `display_name`. The caller must have `resourcemanager.projects.list`
        /// permission on the identified parent.
        pub async fn list_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectsRequest>,
        ) -> Result<tonic::Response<super::ListProjectsResponse>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.Projects/ListProjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Search for projects that the caller has both `resourcemanager.projects.get`
        /// permission on, and also satisfy the specified query.
        ///
        /// This method returns projects in an unspecified order.
        ///
        /// This method is eventually consistent with project mutations; this means
        /// that a newly created project may not appear in the results or recent
        /// updates to an existing project may not be reflected in the results. To
        /// retrieve the latest state of a project, use the
        /// [GetProject][google.cloud.resourcemanager.v3.Projects.GetProject] method.
        pub async fn search_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchProjectsRequest>,
        ) -> Result<tonic::Response<super::SearchProjectsResponse>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.Projects/SearchProjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Request that a new project be created. The result is an `Operation` which
        /// can be used to track the creation process. This process usually takes a few
        /// seconds, but can sometimes take much longer. The tracking `Operation` is
        /// automatically deleted after a few hours, so there is no need to call
        /// `DeleteOperation`.
        pub async fn create_project(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.Projects/CreateProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the `display_name` and labels of the project identified by the
        /// specified `name` (for example, `projects/415104041262`). Deleting all
        /// labels requires an update mask for labels field.
        ///
        /// The caller must have `resourcemanager.projects.update` permission for this
        /// project.
        pub async fn update_project(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.Projects/UpdateProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Move a project to another place in your resource hierarchy, under a new
        /// resource parent.
        ///
        /// Returns an operation which can be used to track the process of the project
        /// move workflow.
        /// Upon success, the `Operation.response` field will be populated with the
        /// moved project.
        ///
        /// The caller must have `resourcemanager.projects.update` permission on the
        /// project and have `resourcemanager.projects.move` permission on the
        /// project's current and proposed new parent.
        ///
        ///
        pub async fn move_project(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveProjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.Projects/MoveProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Marks the project identified by the specified
        /// `name` (for example, `projects/415104041262`) for deletion.
        ///
        /// This method will only affect the project if it has a lifecycle state of
        /// [ACTIVE][google.cloud.resourcemanager.v3.Project.State.ACTIVE].
        ///
        /// This method changes the Project's lifecycle state from
        /// [ACTIVE][google.cloud.resourcemanager.v3.Project.State.ACTIVE]
        /// to [DELETE_REQUESTED][google.cloud.resourcemanager.v3.Project.State.DELETE_REQUESTED].
        /// The deletion starts at an unspecified time,
        /// at which point the Project is no longer accessible.
        ///
        /// Until the deletion completes, you can check the lifecycle state
        /// checked by retrieving the project with [GetProject]
        /// [google.cloud.resourcemanager.v3.Projects.GetProject],
        /// and the project remains visible to [ListProjects]
        /// [google.cloud.resourcemanager.v3.Projects.ListProjects].
        /// However, you cannot update the project.
        ///
        /// After the deletion completes, the project is not retrievable by
        /// the  [GetProject]
        /// [google.cloud.resourcemanager.v3.Projects.GetProject],
        /// [ListProjects]
        /// [google.cloud.resourcemanager.v3.Projects.ListProjects], and
        /// [SearchProjects][google.cloud.resourcemanager.v3.Projects.SearchProjects]
        /// methods.
        ///
        /// This method behaves idempotently, such that deleting a `DELETE_REQUESTED`
        /// project will not cause an error, but also won't do anything.
        ///
        /// The caller must have `resourcemanager.projects.delete` permissions for this
        /// project.
        pub async fn delete_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.Projects/DeleteProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Restores the project identified by the specified
        /// `name` (for example, `projects/415104041262`).
        /// You can only use this method for a project that has a lifecycle state of
        /// [DELETE_REQUESTED]
        /// [Projects.State.DELETE_REQUESTED].
        /// After deletion starts, the project cannot be restored.
        ///
        /// The caller must have `resourcemanager.projects.undelete` permission for
        /// this project.
        pub async fn undelete_project(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteProjectRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.Projects/UndeleteProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the IAM access control policy for the specified project.
        /// Permission is denied if the policy or the resource do not exist.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v3.Projects/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the IAM access control policy for the specified project.
        ///
        /// CAUTION: This method will replace the existing policy, and cannot be used
        /// to append additional IAM settings.
        ///
        /// Note: Removing service accounts from policies or changing their roles can
        /// render services completely inoperable. It is important to understand how
        /// the service account is being used before removing or updating its roles.
        ///
        /// The following constraints apply when using `setIamPolicy()`:
        ///
        /// + Project does not support `allUsers` and `allAuthenticatedUsers` as
        /// `members` in a `Binding` of a `Policy`.
        ///
        /// + The owner role can be granted to a `user`, `serviceAccount`, or a group
        /// that is part of an organization. For example,
        /// group@myownpersonaldomain.com could be added as an owner to a project in
        /// the myownpersonaldomain.com organization, but not the examplepetstore.com
        /// organization.
        ///
        /// + Service accounts can be made owners of a project directly
        /// without any restrictions. However, to be added as an owner, a user must be
        /// invited using the Cloud Platform console and must accept the invitation.
        ///
        /// + A user cannot be granted the owner role using `setIamPolicy()`. The user
        /// must be granted the owner role using the Cloud Platform Console and must
        /// explicitly accept the invitation.
        ///
        /// + Invitations to grant the owner role cannot be sent using
        /// `setIamPolicy()`;
        /// they must be sent only using the Cloud Platform Console.
        ///
        /// + Membership changes that leave the project without any owners that have
        /// accepted the Terms of Service (ToS) will be rejected.
        ///
        /// + If the project is not part of an organization, there must be at least
        /// one owner who has accepted the Terms of Service (ToS) agreement in the
        /// policy. Calling `setIamPolicy()` to remove the last ToS-accepted owner
        /// from the policy will fail. This restriction also applies to legacy
        /// projects that no longer have owners who have accepted the ToS. Edits to
        /// IAM policies will be rejected until the lack of a ToS-accepting owner is
        /// rectified.
        ///
        /// + Calling this method requires enabling the App Engine Admin API.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v3.Projects/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns permissions that a caller has on the specified project.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.resourcemanager.v3.Projects/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A TagBinding represents a connection between a TagValue and a cloud
/// resource (currently project, folder, or organization). Once a TagBinding is
/// created, the TagValue is applied to all the descendants of the cloud
/// resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagBinding {
    /// Output only. The name of the TagBinding. This is a String of the form:
    /// `tagBindings/{full-resource-name}/{tag-value-name}` (e.g.
    /// `tagBindings/%2F%2Fcloudresourcemanager.googleapis.com%2Fprojects%2F123/tagValues/456`).
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The full resource name of the resource the TagValue is bound to.
    /// E.g. `//cloudresourcemanager.googleapis.com/projects/123`
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
    /// The TagValue of the TagBinding.
    /// Must be of the form `tagValues/456`.
    #[prost(string, tag="3")]
    pub tag_value: ::prost::alloc::string::String,
}
/// Runtime operation information for creating a TagValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagBindingMetadata {
}
/// The request message to create a TagBinding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagBindingRequest {
    /// Required. The TagBinding to be created.
    #[prost(message, optional, tag="1")]
    pub tag_binding: ::core::option::Option<TagBinding>,
    /// Optional. Set to true to perform the validations necessary for creating the resource,
    /// but not actually perform the action.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
}
/// Runtime operation information for deleting a TagBinding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagBindingMetadata {
}
/// The request message to delete a TagBinding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagBindingRequest {
    /// Required. The name of the TagBinding. This is a String of the form:
    /// `tagBindings/{id}` (e.g.
    /// `tagBindings/%2F%2Fcloudresourcemanager.googleapis.com%2Fprojects%2F123/tagValues/456`).
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message to list all TagBindings for a parent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagBindingsRequest {
    /// Required. The full resource name of a resource for which you want to list existing
    /// TagBindings.
    /// E.g. "//cloudresourcemanager.googleapis.com/projects/123"
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of TagBindings to return in the response. The server
    /// allows a maximum of 300 TagBindings to return. If unspecified, the server
    /// will use 100 as the default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to `ListTagBindings`
    /// that indicates where this listing should continue from.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The ListTagBindings response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagBindingsResponse {
    /// A possibly paginated list of TagBindings for the specified TagValue or
    /// resource.
    #[prost(message, repeated, tag="1")]
    pub tag_bindings: ::prost::alloc::vec::Vec<TagBinding>,
    /// Pagination token.
    ///
    /// If the result set is too large to fit in a single response, this token
    /// is returned. It encodes the position of the current result cursor.
    /// Feeding this value into a new list request with the `page_token` parameter
    /// gives the next page of the results.
    ///
    /// When `next_page_token` is not filled in, there is no next page and
    /// the list returned is the last page in the result set.
    ///
    /// Pagination tokens have a limited lifetime.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod tag_bindings_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Allow users to create and manage TagBindings between TagValues and
    /// different cloud resources throughout the GCP resource hierarchy.
    #[derive(Debug, Clone)]
    pub struct TagBindingsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TagBindingsClient<tonic::transport::Channel> {
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
    impl<T> TagBindingsClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TagBindingsClient<InterceptedService<T, F>>
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
            TagBindingsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Lists the TagBindings for the given cloud resource, as specified with
        /// `parent`.
        ///
        /// NOTE: The `parent` field is expected to be a full resource name:
        /// https://cloud.google.com/apis/design/resource_names#full_resource_name
        pub async fn list_tag_bindings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTagBindingsRequest>,
        ) -> Result<tonic::Response<super::ListTagBindingsResponse>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.TagBindings/ListTagBindings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a TagBinding between a TagValue and a cloud resource
        /// (currently project, folder, or organization).
        pub async fn create_tag_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagBindingRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.TagBindings/CreateTagBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a TagBinding.
        pub async fn delete_tag_binding(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagBindingRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.TagBindings/DeleteTagBinding",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A TagKey, used to group a set of TagValues.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagKey {
    /// Immutable. The resource name for a TagKey. Must be in the format
    /// `tagKeys/{tag_key_id}`, where `tag_key_id` is the generated numeric id for
    /// the TagKey.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The resource name of the new TagKey's parent.
    /// Must be of the form `organizations/{org_id}`.
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Immutable. The user friendly name for a TagKey. The short name should be
    /// unique for TagKeys within the same tag namespace.
    ///
    /// The short name must be 1-63 characters, beginning and ending with
    /// an alphanumeric character (\[a-z0-9A-Z\]) with dashes (-), underscores (_),
    /// dots (.), and alphanumerics between.
    #[prost(string, tag="3")]
    pub short_name: ::prost::alloc::string::String,
    /// Output only. Immutable. Namespaced name of the TagKey.
    #[prost(string, tag="4")]
    pub namespaced_name: ::prost::alloc::string::String,
    /// Optional. User-assigned description of the TagKey. Must not exceed 256 characters.
    ///
    /// Read-write.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Creation time.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time.
    #[prost(message, optional, tag="7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Entity tag which users can pass to prevent race conditions. This field is
    /// always set in server responses. See UpdateTagKeyRequest for details.
    #[prost(string, tag="8")]
    pub etag: ::prost::alloc::string::String,
}
/// The request message for listing all TagKeys under a parent resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagKeysRequest {
    /// Required. The resource name of the new TagKey's parent.
    /// Must be of the form `folders/{folder_id}` or `organizations/{org_id}`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of TagKeys to return in the response. The server allows
    /// a maximum of 300 TagKeys to return. If unspecified, the server will use 100
    /// as the default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to `ListTagKey`
    /// that indicates where this listing should continue from.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The ListTagKeys response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagKeysResponse {
    /// List of TagKeys that live under the specified parent in the request.
    #[prost(message, repeated, tag="1")]
    pub tag_keys: ::prost::alloc::vec::Vec<TagKey>,
    /// A pagination token returned from a previous call to `ListTagKeys`
    /// that indicates from where listing should continue.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for getting a TagKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagKeyRequest {
    /// Required. A resource name in the format `tagKeys/{id}`, such as
    /// `tagKeys/123`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for creating a TagKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagKeyRequest {
    /// Required. The TagKey to be created. Only fields `short_name`, `description`,
    /// and `parent` are considered during the creation request.
    #[prost(message, optional, tag="1")]
    pub tag_key: ::core::option::Option<TagKey>,
    /// Optional. Set to true to perform validations necessary for creating the resource, but
    /// not actually perform the action.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
}
/// Runtime operation information for creating a TagKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagKeyMetadata {
}
/// The request message for updating a TagKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagKeyRequest {
    /// Required. The new definition of the TagKey. Only the `description` and `etag` fields
    /// can be updated by this request. If the `etag` field is not empty, it
    /// must match the `etag` field of the existing tag key. Otherwise,
    /// `FAILED_PRECONDITION` will be returned.
    #[prost(message, optional, tag="1")]
    pub tag_key: ::core::option::Option<TagKey>,
    /// Fields to be updated. The mask may only contain `description` or
    /// `etag`. If omitted entirely, both `description` and `etag` are assumed to
    /// be significant.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Set as true to perform validations necessary for updating the resource, but
    /// not actually perform the action.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Runtime operation information for updating a TagKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagKeyMetadata {
}
/// The request message for deleting a TagKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagKeyRequest {
    /// Required. The resource name of a TagKey to be deleted in the format `tagKeys/123`.
    /// The TagKey cannot be a parent of any existing TagValues or it will not be
    /// deleted successfully.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Set as true to perform validations necessary for deletion, but not actually
    /// perform the action.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
    /// Optional. The etag known to the client for the expected state of the TagKey. This is
    /// to be used for optimistic concurrency.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
/// Runtime operation information for deleting a TagKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagKeyMetadata {
}
/// Generated client implementations.
pub mod tag_keys_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Allow users to create and manage tag keys.
    #[derive(Debug, Clone)]
    pub struct TagKeysClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TagKeysClient<tonic::transport::Channel> {
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
    impl<T> TagKeysClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TagKeysClient<InterceptedService<T, F>>
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
            TagKeysClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Lists all TagKeys for a parent resource.
        pub async fn list_tag_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTagKeysRequest>,
        ) -> Result<tonic::Response<super::ListTagKeysResponse>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.TagKeys/ListTagKeys",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves a TagKey. This method will return `PERMISSION_DENIED` if the
        /// key does not exist or the user does not have permission to view it.
        pub async fn get_tag_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTagKeyRequest>,
        ) -> Result<tonic::Response<super::TagKey>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.TagKeys/GetTagKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new TagKey. If another request with the same parameters is
        /// sent while the original request is in process, the second request
        /// will receive an error. A maximum of 300 TagKeys can exist under a parent at
        /// any given time.
        pub async fn create_tag_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagKeyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.TagKeys/CreateTagKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the attributes of the TagKey resource.
        pub async fn update_tag_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagKeyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.TagKeys/UpdateTagKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a TagKey. The TagKey cannot be deleted if it has any child
        /// TagValues.
        pub async fn delete_tag_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagKeyRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.TagKeys/DeleteTagKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy for a TagKey. The returned policy may be
        /// empty if no such policy or resource exists. The `resource` field should
        /// be the TagKey's resource name. For example, "tagKeys/1234".
        /// The caller must have
        /// `cloudresourcemanager.googleapis.com/tagKeys.getIamPolicy` permission on
        /// the specified TagKey.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v3.TagKeys/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the access control policy on a TagKey, replacing any existing
        /// policy. The `resource` field should be the TagKey's resource name.
        /// For example, "tagKeys/1234".
        /// The caller must have `resourcemanager.tagKeys.setIamPolicy` permission
        /// on the identified tagValue.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v3.TagKeys/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns permissions that a caller has on the specified TagKey.
        /// The `resource` field should be the TagKey's resource name.
        /// For example, "tagKeys/1234".
        ///
        /// There are no permissions required for making this API call.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.resourcemanager.v3.TagKeys/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A TagValue is a child of a particular TagKey. This is used to group
/// cloud resources for the purpose of controlling them using policies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagValue {
    /// Immutable. Resource name for TagValue in the format `tagValues/456`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. The resource name of the new TagValue's parent TagKey.
    /// Must be of the form `tagKeys/{tag_key_id}`.
    #[prost(string, tag="2")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Immutable. User-assigned short name for TagValue. The short name should be
    /// unique for TagValues within the same parent TagKey.
    ///
    /// The short name must be 63 characters or less, beginning and ending with
    /// an alphanumeric character (\[a-z0-9A-Z\]) with dashes (-), underscores (_),
    /// dots (.), and alphanumerics between.
    #[prost(string, tag="3")]
    pub short_name: ::prost::alloc::string::String,
    /// Output only. Namespaced name of the TagValue. Must be in the format
    /// `{organization_id}/{tag_key_short_name}/{short_name}`.
    #[prost(string, tag="4")]
    pub namespaced_name: ::prost::alloc::string::String,
    /// Optional. User-assigned description of the TagValue.
    /// Must not exceed 256 characters.
    ///
    /// Read-write.
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Creation time.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Update time.
    #[prost(message, optional, tag="7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Entity tag which users can pass to prevent race conditions. This field is
    /// always set in server responses. See UpdateTagValueRequest for details.
    #[prost(string, tag="8")]
    pub etag: ::prost::alloc::string::String,
}
/// The request message for listing TagValues for the specified TagKey.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagValuesRequest {
    /// Required. Resource name for TagKey, parent of the TagValues to be listed,
    /// in the format `tagKeys/123`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of TagValues to return in the response. The server
    /// allows a maximum of 300 TagValues to return. If unspecified, the server
    /// will use 100 as the default.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to `ListTagValues`
    /// that indicates where this listing should continue from.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The ListTagValues response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTagValuesResponse {
    /// A possibly paginated list of TagValues that are direct descendants of
    /// the specified parent TagKey.
    #[prost(message, repeated, tag="1")]
    pub tag_values: ::prost::alloc::vec::Vec<TagValue>,
    /// A pagination token returned from a previous call to `ListTagValues`
    /// that indicates from where listing should continue. This is currently not
    /// used, but the server may at any point start supplying a valid token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for getting a TagValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagValueRequest {
    /// Required. Resource name for TagValue to be fetched in the format `tagValues/456`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for creating a TagValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagValueRequest {
    /// Required. The TagValue to be created. Only fields `short_name`, `description`,
    /// and `parent` are considered during the creation request.
    #[prost(message, optional, tag="1")]
    pub tag_value: ::core::option::Option<TagValue>,
    /// Optional. Set as true to perform the validations necessary for creating the resource,
    /// but not actually perform the action.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
}
/// Runtime operation information for creating a TagValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagValueMetadata {
}
/// The request message for updating a TagValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagValueRequest {
    /// Required. The new definition of the TagValue. Only fields `description` and `etag`
    /// fields can be updated by this request. If the `etag` field is nonempty, it
    /// must match the `etag` field of the existing ControlGroup. Otherwise,
    /// `FAILED_PRECONDITION` will be returned.
    #[prost(message, optional, tag="1")]
    pub tag_value: ::core::option::Option<TagValue>,
    /// Optional. Fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. True to perform validations necessary for updating the resource, but not
    /// actually perform the action.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
}
/// Runtime operation information for updating a TagValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTagValueMetadata {
}
/// The request message for deleting a TagValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagValueRequest {
    /// Required. Resource name for TagValue to be deleted in the format tagValues/456.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Set as true to perform the validations necessary for deletion, but not
    /// actually perform the action.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
    /// Optional. The etag known to the client for the expected state of the TagValue. This
    /// is to be used for optimistic concurrency.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
/// Runtime operation information for deleting a TagValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTagValueMetadata {
}
/// Generated client implementations.
pub mod tag_values_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Allow users to create and manage tag values.
    #[derive(Debug, Clone)]
    pub struct TagValuesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TagValuesClient<tonic::transport::Channel> {
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
    impl<T> TagValuesClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TagValuesClient<InterceptedService<T, F>>
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
            TagValuesClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        /// Lists all TagValues for a specific TagKey.
        pub async fn list_tag_values(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTagValuesRequest>,
        ) -> Result<tonic::Response<super::ListTagValuesResponse>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.TagValues/ListTagValues",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves TagValue. If the TagValue or namespaced name does not exist, or
        /// if the user does not have permission to view it, this method will return
        /// `PERMISSION_DENIED`.
        pub async fn get_tag_value(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTagValueRequest>,
        ) -> Result<tonic::Response<super::TagValue>, tonic::Status> {
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
                "/google.cloud.resourcemanager.v3.TagValues/GetTagValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a TagValue as a child of the specified TagKey. If a another
        /// request with the same parameters is sent while the original request is in
        /// process the second request will receive an error. A maximum of 300
        /// TagValues can exist under a TagKey at any given time.
        pub async fn create_tag_value(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTagValueRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.TagValues/CreateTagValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the attributes of the TagValue resource.
        pub async fn update_tag_value(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTagValueRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.TagValues/UpdateTagValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a TagValue. The TagValue cannot have any bindings when it is
        /// deleted.
        pub async fn delete_tag_value(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTagValueRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.resourcemanager.v3.TagValues/DeleteTagValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the access control policy for a TagValue. The returned policy may be
        /// empty if no such policy or resource exists. The `resource` field should
        /// be the TagValue's resource name. For example: `tagValues/1234`.
        /// The caller must have the
        /// `cloudresourcemanager.googleapis.com/tagValues.getIamPolicy` permission on
        /// the identified TagValue to get the access control policy.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v3.TagValues/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the access control policy on a TagValue, replacing any existing
        /// policy. The `resource` field should be the TagValue's resource name.
        /// For example: `tagValues/1234`.
        /// The caller must have `resourcemanager.tagValues.setIamPolicy` permission
        /// on the identified tagValue.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.resourcemanager.v3.TagValues/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns permissions that a caller has on the specified TagValue.
        /// The `resource` field should be the TagValue's resource name. For example:
        /// `tagValues/1234`.
        ///
        /// There are no permissions required for making this API call.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.resourcemanager.v3.TagValues/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
