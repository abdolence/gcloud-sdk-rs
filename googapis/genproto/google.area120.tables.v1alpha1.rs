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
    /// Supported types are number, text, boolean, number_list, text_list,
    /// boolean_list.
    #[prost(string, tag = "2")]
    pub data_type: ::prost::alloc::string::String,
    /// Internal id for a column.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
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
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Tables Service provides an API for reading and updating tables."]
    #[doc = " It defines the following resource model:"]
    #[doc = ""]
    #[doc = " - The API has a collection of [Table][google.area120.tables.v1alpha1.Table]"]
    #[doc = "   resources, named `tables/*`"]
    #[doc = ""]
    #[doc = " - Each Table has a collection of [Row][google.area120.tables.v1alpha1.Row]"]
    #[doc = "   resources, named `tables/*/rows/*`"]
    pub struct TablesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TablesServiceClient<T>
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
    }
    impl<T: Clone> Clone for TablesServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for TablesServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TablesServiceClient {{ ... }}")
        }
    }
}
