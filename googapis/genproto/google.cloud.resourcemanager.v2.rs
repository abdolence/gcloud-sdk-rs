/// A Folder in an Organization's resource hierarchy, used to
/// organize that Organization's resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Folder {
    /// Output only. The resource name of the Folder.
    /// Its format is `folders/{folder_id}`, for example: "folders/1234".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The Folder’s parent's resource name.
    /// Updates to the folder's parent must be performed via
    /// \[MoveFolder][google.cloud.resourcemanager.v2.Folders.MoveFolder\].
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    /// The folder’s display name.
    /// A folder’s display name must be unique amongst its siblings, e.g.
    /// no two folders with the same parent can share the same display name.
    /// The display name must start and end with a letter or digit, may contain
    /// letters, digits, spaces, hyphens and underscores and can be no longer
    /// than 30 characters. This is captured by the regular expression:
    /// \[\p{L}\p{N}\]([\p{L}\p{N}_- ]{0,28}\[\p{L}\p{N}\])?.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The lifecycle state of the folder.
    /// Updates to the lifecycle_state must be performed via
    /// \[DeleteFolder][google.cloud.resourcemanager.v2.Folders.DeleteFolder\] and
    /// \[UndeleteFolder][google.cloud.resourcemanager.v2.Folders.UndeleteFolder\].
    #[prost(enumeration = "folder::LifecycleState", tag = "4")]
    pub lifecycle_state: i32,
    /// Output only. Timestamp when the Folder was created. Assigned by the server.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the Folder was last modified.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `Folder`.
pub mod folder {
    /// Folder lifecycle states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LifecycleState {
        /// Unspecified state.
        Unspecified = 0,
        /// The normal and active state.
        Active = 1,
        /// The folder has been marked for deletion by the user.
        DeleteRequested = 2,
    }
}
/// The ListFolders request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFoldersRequest {
    /// Required. The resource name of the Organization or Folder whose Folders are
    /// being listed.
    /// Must be of the form `folders/{folder_id}` or `organizations/{org_id}`.
    /// Access to this method is controlled by checking the
    /// `resourcemanager.folders.list` permission on the `parent`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of Folders to return in the response.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to `ListFolders`
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Controls whether Folders in the
    /// \[DELETE_REQUESTED][google.cloud.resourcemanager.v2.Folder.LifecycleState.DELETE_REQUESTED\]
    /// state should be returned. Defaults to false.
    #[prost(bool, tag = "4")]
    pub show_deleted: bool,
}
/// The ListFolders response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFoldersResponse {
    /// A possibly paginated list of Folders that are direct descendants of
    /// the specified parent resource.
    #[prost(message, repeated, tag = "1")]
    pub folders: ::prost::alloc::vec::Vec<Folder>,
    /// A pagination token returned from a previous call to `ListFolders`
    /// that indicates from where listing should continue.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for searching folders.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchFoldersRequest {
    /// Optional. The maximum number of folders to return in the response.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to `SearchFolders`
    /// that indicates from where search should continue.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Search criteria used to select the Folders to return.
    /// If no search criteria is specified then all accessible folders will be
    /// returned.
    ///
    /// Query expressions can be used to restrict results based upon displayName,
    /// lifecycleState and parent, where the operators `=`, `NOT`, `AND` and `OR`
    /// can be used along with the suffix wildcard symbol `*`.
    ///
    /// The displayName field in a query expression should use escaped quotes
    /// for values that include whitespace to prevent unexpected behavior.
    ///
    /// Some example queries are:
    ///
    /// * Query `displayName=Test*` returns Folder resources whose display name
    /// starts with "Test".
    /// * Query `lifecycleState=ACTIVE` returns Folder resources with
    /// `lifecycleState` set to `ACTIVE`.
    /// * Query `parent=folders/123` returns Folder resources that have
    /// `folders/123` as a parent resource.
    /// * Query `parent=folders/123 AND lifecycleState=ACTIVE` returns active
    /// Folder resources that have `folders/123` as a parent resource.
    /// * Query `displayName=\\"Test String\\"` returns Folder resources with
    /// display names that include both "Test" and "String".
    #[prost(string, tag = "3")]
    pub query: ::prost::alloc::string::String,
}
/// The response message for searching folders.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchFoldersResponse {
    /// A possibly paginated folder search results.
    /// the specified parent resource.
    #[prost(message, repeated, tag = "1")]
    pub folders: ::prost::alloc::vec::Vec<Folder>,
    /// A pagination token returned from a previous call to `SearchFolders`
    /// that indicates from where searching should continue.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The GetFolder request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFolderRequest {
    /// Required. The resource name of the Folder to retrieve.
    /// Must be of the form `folders/{folder_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The CreateFolder request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFolderRequest {
    /// Required. The resource name of the new Folder's parent.
    /// Must be of the form `folders/{folder_id}` or `organizations/{org_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Folder being created, only the display name will be consulted.
    /// All other fields will be ignored.
    #[prost(message, optional, tag = "2")]
    pub folder: ::core::option::Option<Folder>,
}
/// The MoveFolder request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveFolderRequest {
    /// Required. The resource name of the Folder to move.
    /// Must be of the form folders/{folder_id}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The resource name of the Folder or Organization to reparent
    /// the folder under.
    /// Must be of the form `folders/{folder_id}` or `organizations/{org_id}`.
    #[prost(string, tag = "2")]
    pub destination_parent: ::prost::alloc::string::String,
}
/// The request message for updating a folder's display name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFolderRequest {
    /// Required. The new definition of the Folder. It must include a
    /// a `name` and `display_name` field. The other fields
    /// will be ignored.
    #[prost(message, optional, tag = "1")]
    pub folder: ::core::option::Option<Folder>,
    /// Required. Fields to be updated.
    /// Only the `display_name` can be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The DeleteFolder request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFolderRequest {
    /// Required. The resource name of the Folder to be deleted.
    /// Must be of the form `folders/{folder_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Instructs DeleteFolderAction to delete a folder even when the folder is not
    /// empty.
    #[prost(bool, tag = "2")]
    pub recursive_delete: bool,
}
/// The UndeleteFolder request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteFolderRequest {
    /// Required. The resource name of the Folder to undelete.
    /// Must be of the form `folders/{folder_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata describing a long running folder operation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FolderOperation {
    /// The display name of the folder.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// The type of this operation.
    #[prost(enumeration = "folder_operation::OperationType", tag = "2")]
    pub operation_type: i32,
    /// The resource name of the folder's parent.
    /// Only applicable when the operation_type is MOVE.
    #[prost(string, tag = "3")]
    pub source_parent: ::prost::alloc::string::String,
    /// The resource name of the folder or organization we are either creating
    /// the folder under or moving the folder to.
    #[prost(string, tag = "4")]
    pub destination_parent: ::prost::alloc::string::String,
}
/// Nested message and enum types in `FolderOperation`.
pub mod folder_operation {
    /// The type of operation that failed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OperationType {
        /// Operation type not specified.
        Unspecified = 0,
        /// A create folder operation.
        Create = 1,
        /// A move folder operation.
        Move = 2,
    }
}
#[doc = r" Generated client implementations."]
pub mod folders_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Manages Cloud Resource Folders."]
    #[doc = " Cloud Resource Folders can be used to organize the resources under an"]
    #[doc = " organization and to control the IAM policies applied to groups of resources."]
    #[derive(Debug, Clone)]
    pub struct FoldersClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FoldersClient<T>
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
        ) -> FoldersClient<InterceptedService<T, F>>
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
            FoldersClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists the Folders that are direct descendants of supplied parent resource."]
        #[doc = " List provides a strongly consistent view of the Folders underneath"]
        #[doc = " the specified parent resource."]
        #[doc = " List returns Folders sorted based upon the (ascending) lexical ordering"]
        #[doc = " of their display_name."]
        #[doc = " The caller must have `resourcemanager.folders.list` permission on the"]
        #[doc = " identified parent."]
        pub async fn list_folders(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFoldersRequest>,
        ) -> Result<tonic::Response<super::ListFoldersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcemanager.v2.Folders/ListFolders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Search for folders that match specific filter criteria."]
        #[doc = " Search provides an eventually consistent view of the folders a user has"]
        #[doc = " access to which meet the specified filter criteria."]
        #[doc = ""]
        #[doc = " This will only return folders on which the caller has the"]
        #[doc = " permission `resourcemanager.folders.get`."]
        pub async fn search_folders(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchFoldersRequest>,
        ) -> Result<tonic::Response<super::SearchFoldersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcemanager.v2.Folders/SearchFolders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves a Folder identified by the supplied resource name."]
        #[doc = " Valid Folder resource names have the format `folders/{folder_id}`"]
        #[doc = " (for example, `folders/1234`)."]
        #[doc = " The caller must have `resourcemanager.folders.get` permission on the"]
        #[doc = " identified folder."]
        pub async fn get_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFolderRequest>,
        ) -> Result<tonic::Response<super::Folder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcemanager.v2.Folders/GetFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a Folder in the resource hierarchy."]
        #[doc = " Returns an Operation which can be used to track the progress of the"]
        #[doc = " folder creation workflow."]
        #[doc = " Upon success the Operation.response field will be populated with the"]
        #[doc = " created Folder."]
        #[doc = ""]
        #[doc = " In order to succeed, the addition of this new Folder must not violate"]
        #[doc = " the Folder naming, height or fanout constraints."]
        #[doc = ""]
        #[doc = " + The Folder's display_name must be distinct from all other Folder's that"]
        #[doc = " share its parent."]
        #[doc = " + The addition of the Folder must not cause the active Folder hierarchy"]
        #[doc = " to exceed a height of 4. Note, the full active + deleted Folder hierarchy"]
        #[doc = " is allowed to reach a height of 8; this provides additional headroom when"]
        #[doc = " moving folders that contain deleted folders."]
        #[doc = " + The addition of the Folder must not cause the total number of Folders"]
        #[doc = " under its parent to exceed 100."]
        #[doc = ""]
        #[doc = " If the operation fails due to a folder constraint violation, some errors"]
        #[doc = " may be returned by the CreateFolder request, with status code"]
        #[doc = " FAILED_PRECONDITION and an error description. Other folder constraint"]
        #[doc = " violations will be communicated in the Operation, with the specific"]
        #[doc = " PreconditionFailure returned via the details list in the Operation.error"]
        #[doc = " field."]
        #[doc = ""]
        #[doc = " The caller must have `resourcemanager.folders.create` permission on the"]
        #[doc = " identified parent."]
        pub async fn create_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFolderRequest>,
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
                "/google.cloud.resourcemanager.v2.Folders/CreateFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a Folder, changing its display_name."]
        #[doc = " Changes to the folder display_name will be rejected if they violate either"]
        #[doc = " the display_name formatting rules or naming constraints described in"]
        #[doc = " the [CreateFolder][google.cloud.resourcemanager.v2.Folders.CreateFolder] documentation."]
        #[doc = ""]
        #[doc = " The Folder's display name must start and end with a letter or digit,"]
        #[doc = " may contain letters, digits, spaces, hyphens and underscores and can be"]
        #[doc = " no longer than 30 characters. This is captured by the regular expression:"]
        #[doc = " [\\p{L}\\p{N}]([\\p{L}\\p{N}_- ]{0,28}[\\p{L}\\p{N}])?."]
        #[doc = " The caller must have `resourcemanager.folders.update` permission on the"]
        #[doc = " identified folder."]
        #[doc = ""]
        #[doc = " If the update fails due to the unique name constraint then a"]
        #[doc = " PreconditionFailure explaining this violation will be returned"]
        #[doc = " in the Status.details field."]
        pub async fn update_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFolderRequest>,
        ) -> Result<tonic::Response<super::Folder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcemanager.v2.Folders/UpdateFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Moves a Folder under a new resource parent."]
        #[doc = " Returns an Operation which can be used to track the progress of the"]
        #[doc = " folder move workflow."]
        #[doc = " Upon success the Operation.response field will be populated with the"]
        #[doc = " moved Folder."]
        #[doc = " Upon failure, a FolderOperationError categorizing the failure cause will"]
        #[doc = " be returned - if the failure occurs synchronously then the"]
        #[doc = " FolderOperationError will be returned via the Status.details field"]
        #[doc = " and if it occurs asynchronously then the FolderOperation will be returned"]
        #[doc = " via the Operation.error field."]
        #[doc = " In addition, the Operation.metadata field will be populated with a"]
        #[doc = " FolderOperation message as an aid to stateless clients."]
        #[doc = " Folder moves will be rejected if they violate either the naming, height"]
        #[doc = " or fanout constraints described in the"]
        #[doc = " [CreateFolder][google.cloud.resourcemanager.v2.Folders.CreateFolder] documentation."]
        #[doc = " The caller must have `resourcemanager.folders.move` permission on the"]
        #[doc = " folder's current and proposed new parent."]
        pub async fn move_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveFolderRequest>,
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
                "/google.cloud.resourcemanager.v2.Folders/MoveFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Requests deletion of a Folder. The Folder is moved into the"]
        #[doc = " [DELETE_REQUESTED][google.cloud.resourcemanager.v2.Folder.LifecycleState.DELETE_REQUESTED] state"]
        #[doc = " immediately, and is deleted approximately 30 days later. This method may"]
        #[doc = " only be called on an empty Folder in the"]
        #[doc = " [ACTIVE][google.cloud.resourcemanager.v2.Folder.LifecycleState.ACTIVE] state, where a Folder is empty if"]
        #[doc = " it doesn't contain any Folders or Projects in the"]
        #[doc = " [ACTIVE][google.cloud.resourcemanager.v2.Folder.LifecycleState.ACTIVE] state."]
        #[doc = " The caller must have `resourcemanager.folders.delete` permission on the"]
        #[doc = " identified folder."]
        pub async fn delete_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFolderRequest>,
        ) -> Result<tonic::Response<super::Folder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcemanager.v2.Folders/DeleteFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels the deletion request for a Folder. This method may only be"]
        #[doc = " called on a Folder in the"]
        #[doc = " [DELETE_REQUESTED][google.cloud.resourcemanager.v2.Folder.LifecycleState.DELETE_REQUESTED] state."]
        #[doc = " In order to succeed, the Folder's parent must be in the"]
        #[doc = " [ACTIVE][google.cloud.resourcemanager.v2.Folder.LifecycleState.ACTIVE] state."]
        #[doc = " In addition, reintroducing the folder into the tree must not violate"]
        #[doc = " folder naming, height and fanout constraints described in the"]
        #[doc = " [CreateFolder][google.cloud.resourcemanager.v2.Folders.CreateFolder] documentation."]
        #[doc = " The caller must have `resourcemanager.folders.undelete` permission on the"]
        #[doc = " identified folder."]
        pub async fn undelete_folder(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteFolderRequest>,
        ) -> Result<tonic::Response<super::Folder>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcemanager.v2.Folders/UndeleteFolder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a Folder. The returned policy may be"]
        #[doc = " empty if no such policy or resource exists. The `resource` field should"]
        #[doc = " be the Folder's resource name, e.g. \"folders/1234\"."]
        #[doc = " The caller must have `resourcemanager.folders.getIamPolicy` permission"]
        #[doc = " on the identified folder."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcemanager.v2.Folders/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on a Folder, replacing any existing policy."]
        #[doc = " The `resource` field should be the Folder's resource name, e.g."]
        #[doc = " \"folders/1234\"."]
        #[doc = " The caller must have `resourcemanager.folders.setIamPolicy` permission"]
        #[doc = " on the identified folder."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcemanager.v2.Folders/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that a caller has on the specified Folder."]
        #[doc = " The `resource` field should be the Folder's resource name,"]
        #[doc = " e.g. \"folders/1234\"."]
        #[doc = ""]
        #[doc = " There are no permissions required for making this API call."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.cloud.resourcemanager.v2.Folders/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
