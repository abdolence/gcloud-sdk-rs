/// Request message for TablesService.GetTable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableRequest {
    /// Required. The name of the table to retrieve.
    /// Format: tables/{table}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for TablesService.ListTables.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTablesRequest {
    /// The maximum number of tables to return. The service may return fewer than
    /// this value.
    ///
    /// If unspecified, at most 20 tables are returned. The maximum value is 100;
    /// values above 100 are coerced to 100.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A page token, received from a previous `ListTables` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTables` must match
    /// the call that provided the page token.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for TablesService.ListTables.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTablesResponse {
    /// The list of tables.
    #[prost(message, repeated, tag = "1")]
    pub tables: ::prost::alloc::vec::Vec<Table>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is empty, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for TablesService.GetWorkspace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkspaceRequest {
    /// Required. The name of the workspace to retrieve.
    /// Format: workspaces/{workspace}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for TablesService.ListWorkspaces.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkspacesRequest {
    /// The maximum number of workspaces to return. The service may return fewer
    /// than this value.
    ///
    /// If unspecified, at most 10 workspaces are returned. The maximum value is
    /// 25; values above 25 are coerced to 25.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A page token, received from a previous `ListWorkspaces` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListWorkspaces` must
    /// match the call that provided the page token.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for TablesService.ListWorkspaces.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkspacesResponse {
    /// The list of workspaces.
    #[prost(message, repeated, tag = "1")]
    pub workspaces: ::prost::alloc::vec::Vec<Workspace>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is empty, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for TablesService.GetRow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRowRequest {
    /// Required. The name of the row to retrieve.
    /// Format: tables/{table}/rows/{row}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Column key to use for values in the row.
    /// Defaults to user entered name.
    #[prost(enumeration = "View", tag = "2")]
    pub view: i32,
}
/// Request message for TablesService.ListRows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRowsRequest {
    /// Required. The parent table.
    /// Format: tables/{table}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of rows to return. The service may return fewer than
    /// this value.
    ///
    /// If unspecified, at most 50 rows are returned. The maximum value is 1,000;
    /// values above 1,000 are coerced to 1,000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListRows` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListRows` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Column key to use for values in the row.
    /// Defaults to user entered name.
    #[prost(enumeration = "View", tag = "4")]
    pub view: i32,
    /// Optional. Raw text query to search for in rows of the table.
    /// Special characters must be escaped. Logical operators and field specific
    /// filtering not supported.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for TablesService.ListRows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRowsResponse {
    /// The rows from the specified table.
    #[prost(message, repeated, tag = "1")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is empty, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for TablesService.CreateRow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRowRequest {
    /// Required. The parent table where this row will be created.
    /// Format: tables/{table}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The row to create.
    #[prost(message, optional, tag = "2")]
    pub row: ::core::option::Option<Row>,
    /// Optional. Column key to use for values in the row.
    /// Defaults to user entered name.
    #[prost(enumeration = "View", tag = "3")]
    pub view: i32,
}
/// Request message for TablesService.BatchCreateRows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateRowsRequest {
    /// Required. The parent table where the rows will be created.
    /// Format: tables/{table}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request message specifying the rows to create.
    ///
    /// A maximum of 500 rows can be created in a single batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<CreateRowRequest>,
}
/// Response message for TablesService.BatchCreateRows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateRowsResponse {
    /// The created rows.
    #[prost(message, repeated, tag = "1")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
/// Request message for TablesService.UpdateRow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRowRequest {
    /// Required. The row to update.
    #[prost(message, optional, tag = "1")]
    pub row: ::core::option::Option<Row>,
    /// The list of fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. Column key to use for values in the row.
    /// Defaults to user entered name.
    #[prost(enumeration = "View", tag = "3")]
    pub view: i32,
}
/// Request message for TablesService.BatchUpdateRows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateRowsRequest {
    /// Required. The parent table shared by all rows being updated.
    /// Format: tables/{table}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request messages specifying the rows to update.
    ///
    /// A maximum of 500 rows can be modified in a single batch.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<UpdateRowRequest>,
}
/// Response message for TablesService.BatchUpdateRows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateRowsResponse {
    /// The updated rows.
    #[prost(message, repeated, tag = "1")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
}
/// Request message for TablesService.DeleteRow
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRowRequest {
    /// Required. The name of the row to delete.
    /// Format: tables/{table}/rows/{row}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for TablesService.BatchDeleteRows
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteRowsRequest {
    /// Required. The parent table shared by all rows being deleted.
    /// Format: tables/{table}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The names of the rows to delete. All rows must belong to the parent table
    /// or else the entire batch will fail. A maximum of 500 rows can be deleted
    /// in a batch.
    /// Format: tables/{table}/rows/{row}
    #[prost(string, repeated, tag = "2")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A single table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    /// The resource name of the table.
    /// Table names have the form `tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The human readable title of the table.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// List of columns in this table.
    /// Order of columns matches the display order.
    #[prost(message, repeated, tag = "3")]
    pub columns: ::prost::alloc::vec::Vec<ColumnDescription>,
}
/// Details on a column in the table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnDescription {
    /// column name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Data type of the column
    /// Supported types are auto_id, boolean, boolean_list, creator,
    /// create_timestamp, date, dropdown, location, integer,
    /// integer_list, number, number_list, person, person_list, tags, check_list,
    /// text, text_list, update_timestamp, updater, relationship,
    /// file_attachment_list.
    /// These types directly map to the column types supported on Tables website.
    #[prost(string, tag = "2")]
    pub data_type: ::prost::alloc::string::String,
    /// Internal id for a column.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    /// Optional. Range of labeled values for the column.
    /// Some columns like tags and drop-downs limit the values to a set of
    /// possible values. We return the range of values in such cases to help
    /// clients implement better user data validation.
    #[prost(message, repeated, tag = "4")]
    pub labels: ::prost::alloc::vec::Vec<LabeledItem>,
    /// Optional. Additional details about a relationship column. Specified when data_type
    /// is relationship.
    #[prost(message, optional, tag = "5")]
    pub relationship_details: ::core::option::Option<RelationshipDetails>,
    /// Optional. Indicates that this is a lookup column whose value is derived from the
    /// relationship column specified in the details. Lookup columns can not be
    /// updated directly. To change the value you must update the associated
    /// relationship column.
    #[prost(message, optional, tag = "6")]
    pub lookup_details: ::core::option::Option<LookupDetails>,
}
/// A single item in a labeled column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabeledItem {
    /// Display string as entered by user.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Internal id associated with the item.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// Details about a relationship column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationshipDetails {
    /// The name of the table this relationship is linked to.
    #[prost(string, tag = "1")]
    pub linked_table: ::prost::alloc::string::String,
}
/// Details about a lookup column whose value comes from the associated
/// relationship.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupDetails {
    /// The name of the relationship column associated with the lookup.
    #[prost(string, tag = "1")]
    pub relationship_column: ::prost::alloc::string::String,
    /// The id of the relationship column.
    #[prost(string, tag = "2")]
    pub relationship_column_id: ::prost::alloc::string::String,
}
/// A single row in a table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    /// The resource name of the row.
    /// Row names have the form `tables/{table}/rows/{row}`.
    /// The name is ignored when creating a row.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The values of the row. This is a map of column key to value.
    /// Key is user entered name(default) or the internal column id based on
    /// the view in the request.
    #[prost(map = "string, message", tag = "2")]
    pub values: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}
/// A single workspace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workspace {
    /// The resource name of the workspace.
    /// Workspace names have the form `workspaces/{workspace}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The human readable title of the workspace.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The list of tables in the workspace.
    #[prost(message, repeated, tag = "3")]
    pub tables: ::prost::alloc::vec::Vec<Table>,
}
/// Column identifier used for the values in the row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum View {
    /// Defaults to user entered text.
    Unspecified = 0,
    /// Uses internally generated column id to identify values.
    ColumnIdView = 1,
}
#[doc = r" Generated client implementations."]
pub mod tables_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The Tables Service provides an API for reading and updating tables."]
    #[doc = " It defines the following resource model:"]
    #[doc = ""]
    #[doc = " - The API has a collection of [Table][google.area120.tables.v1alpha1.Table]"]
    #[doc = "   resources, named `tables/*`"]
    #[doc = ""]
    #[doc = " - Each Table has a collection of [Row][google.area120.tables.v1alpha1.Row]"]
    #[doc = "   resources, named `tables/*/rows/*`"]
    #[doc = ""]
    #[doc = " - The API has a collection of"]
    #[doc = "   [Workspace][google.area120.tables.v1alpha1.Workspace]"]
    #[doc = "   resources, named `workspaces/*`."]
    #[derive(Debug, Clone)]
    pub struct TablesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TablesServiceClient<T>
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
        ) -> TablesServiceClient<InterceptedService<T, F>>
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
            TablesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Gets a table. Returns NOT_FOUND if the table does not exist."]
        pub async fn get_table(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTableRequest>,
        ) -> Result<tonic::Response<super::Table>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/GetTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists tables for the user."]
        pub async fn list_tables(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTablesRequest>,
        ) -> Result<tonic::Response<super::ListTablesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/ListTables",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a workspace. Returns NOT_FOUND if the workspace does not exist."]
        pub async fn get_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkspaceRequest>,
        ) -> Result<tonic::Response<super::Workspace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/GetWorkspace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists workspaces for the user."]
        pub async fn list_workspaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkspacesRequest>,
        ) -> Result<tonic::Response<super::ListWorkspacesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/ListWorkspaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a row. Returns NOT_FOUND if the row does not exist in the table."]
        pub async fn get_row(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRowRequest>,
        ) -> Result<tonic::Response<super::Row>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/GetRow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists rows in a table. Returns NOT_FOUND if the table does not exist."]
        pub async fn list_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRowsRequest>,
        ) -> Result<tonic::Response<super::ListRowsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/ListRows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a row."]
        pub async fn create_row(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRowRequest>,
        ) -> Result<tonic::Response<super::Row>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/CreateRow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates multiple rows."]
        pub async fn batch_create_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateRowsRequest>,
        ) -> Result<tonic::Response<super::BatchCreateRowsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/BatchCreateRows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a row."]
        pub async fn update_row(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRowRequest>,
        ) -> Result<tonic::Response<super::Row>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/UpdateRow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates multiple rows."]
        pub async fn batch_update_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateRowsRequest>,
        ) -> Result<tonic::Response<super::BatchUpdateRowsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/BatchUpdateRows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a row."]
        pub async fn delete_row(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRowRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/DeleteRow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes multiple rows."]
        pub async fn batch_delete_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteRowsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.area120.tables.v1alpha1.TablesService/BatchDeleteRows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
