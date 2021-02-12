/// A collection of user data indexed by row, column, and timestamp.
/// Each table is served using the resources of its parent cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    /// A unique identifier of the form
    /// <cluster_name>/tables/[_a-zA-Z0-9][-_.a-zA-Z0-9]*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If this Table is in the process of being created, the Operation used to
    /// track its progress. As long as this operation is present, the Table will
    /// not accept any Table Admin or Read/Write requests.
    #[prost(message, optional, tag = "2")]
    pub current_operation:
        ::core::option::Option<super::super::super::super::longrunning::Operation>,
    /// The column families configured for this table, mapped by column family id.
    #[prost(map = "string, message", tag = "3")]
    pub column_families: ::std::collections::HashMap<::prost::alloc::string::String, ColumnFamily>,
    /// The granularity (e.g. MILLIS, MICROS) at which timestamps are stored in
    /// this table. Timestamps not matching the granularity will be rejected.
    /// Cannot be changed once the table is created.
    #[prost(enumeration = "table::TimestampGranularity", tag = "4")]
    pub granularity: i32,
}
/// Nested message and enum types in `Table`.
pub mod table {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimestampGranularity {
        Millis = 0,
    }
}
/// A set of columns within a table which share a common configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnFamily {
    /// A unique identifier of the form <table_name>/columnFamilies/[-_.a-zA-Z0-9]+
    /// The last segment is the same as the "name" field in
    /// google.bigtable.v1.Family.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Garbage collection expression specified by the following grammar:
    ///   GC = EXPR
    ///      | "" ;
    ///   EXPR = EXPR, "||", EXPR              (* lowest precedence *)
    ///        | EXPR, "&&", EXPR
    ///        | "(", EXPR, ")"                (* highest precedence *)
    ///        | PROP ;
    ///   PROP = "version() >", NUM32
    ///        | "age() >", NUM64, [ UNIT ] ;
    ///   NUM32 = non-zero-digit { digit } ;    (* # NUM32 <= 2^32 - 1 *)
    ///   NUM64 = non-zero-digit { digit } ;    (* # NUM64 <= 2^63 - 1 *)
    ///   UNIT =  "d" | "h" | "m"  (* d=days, h=hours, m=minutes, else micros *)
    /// GC expressions can be up to 500 characters in length
    ///
    /// The different types of PROP are defined as follows:
    ///   version() - cell index, counting from most recent and starting at 1
    ///   age() - age of the cell (current time minus cell timestamp)
    ///
    /// Example: "version() > 3 || (age() > 3d && version() > 1)"
    ///   drop cells beyond the most recent three, and drop cells older than three
    ///   days unless they're the most recent cell in the row/column
    ///
    /// Garbage collection executes opportunistically in the background, and so
    /// it's possible for reads to return a cell even if it matches the active GC
    /// expression for its family.
    #[prost(string, tag = "2")]
    pub gc_expression: ::prost::alloc::string::String,
    /// Garbage collection rule specified as a protobuf.
    /// Supersedes `gc_expression`.
    /// Must serialize to at most 500 bytes.
    ///
    /// NOTE: Garbage collection executes opportunistically in the background, and
    /// so it's possible for reads to return a cell even if it matches the active
    /// GC expression for its family.
    #[prost(message, optional, tag = "3")]
    pub gc_rule: ::core::option::Option<GcRule>,
}
/// Rule for determining which cells to delete during garbage collection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcRule {
    #[prost(oneof = "gc_rule::Rule", tags = "1, 2, 3, 4")]
    pub rule: ::core::option::Option<gc_rule::Rule>,
}
/// Nested message and enum types in `GcRule`.
pub mod gc_rule {
    /// A GcRule which deletes cells matching all of the given rules.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Intersection {
        /// Only delete cells which would be deleted by every element of `rules`.
        #[prost(message, repeated, tag = "1")]
        pub rules: ::prost::alloc::vec::Vec<super::GcRule>,
    }
    /// A GcRule which deletes cells matching any of the given rules.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Union {
        /// Delete cells which would be deleted by any element of `rules`.
        #[prost(message, repeated, tag = "1")]
        pub rules: ::prost::alloc::vec::Vec<super::GcRule>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rule {
        /// Delete all cells in a column except the most recent N.
        #[prost(int32, tag = "1")]
        MaxNumVersions(i32),
        /// Delete cells in a column older than the given age.
        /// Values must be at least one millisecond, and will be truncated to
        /// microsecond granularity.
        #[prost(message, tag = "2")]
        MaxAge(::prost_types::Duration),
        /// Delete cells that would be deleted by every nested rule.
        #[prost(message, tag = "3")]
        Intersection(Intersection),
        /// Delete cells that would be deleted by any nested rule.
        #[prost(message, tag = "4")]
        Union(Union),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableRequest {
    /// The unique name of the cluster in which to create the new table.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name by which the new table should be referred to within the cluster,
    /// e.g. "foobar" rather than "<cluster_name>/tables/foobar".
    #[prost(string, tag = "2")]
    pub table_id: ::prost::alloc::string::String,
    /// The Table to create. The `name` field of the Table and all of its
    /// ColumnFamilies must be left blank, and will be populated in the response.
    #[prost(message, optional, tag = "3")]
    pub table: ::core::option::Option<Table>,
    /// The optional list of row keys that will be used to initially split the
    /// table into several tablets (Tablets are similar to HBase regions).
    /// Given two split keys, "s1" and "s2", three tablets will be created,
    /// spanning the key ranges: [, s1), [s1, s2), [s2, ).
    ///
    /// Example:
    ///  * Row keys := ["a", "apple", "custom", "customer_1", "customer_2",
    ///                 "other", "zz"]
    ///  * initial_split_keys := ["apple", "customer_1", "customer_2", "other"]
    ///  * Key assignment:
    ///    - Tablet 1 [, apple)                => {"a"}.
    ///    - Tablet 2 [apple, customer_1)      => {"apple", "custom"}.
    ///    - Tablet 3 [customer_1, customer_2) => {"customer_1"}.
    ///    - Tablet 4 [customer_2, other)      => {"customer_2"}.
    ///    - Tablet 5 [other, )                => {"other", "zz"}.
    #[prost(string, repeated, tag = "4")]
    pub initial_split_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTablesRequest {
    /// The unique name of the cluster for which tables should be listed.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTablesResponse {
    /// The tables present in the requested cluster.
    /// At present, only the names of the tables are populated.
    #[prost(message, repeated, tag = "1")]
    pub tables: ::prost::alloc::vec::Vec<Table>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableRequest {
    /// The unique name of the requested table.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTableRequest {
    /// The unique name of the table to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameTableRequest {
    /// The current unique name of the table.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The new name by which the table should be referred to within its containing
    /// cluster, e.g. "foobar" rather than "<cluster_name>/tables/foobar".
    #[prost(string, tag = "2")]
    pub new_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateColumnFamilyRequest {
    /// The unique name of the table in which to create the new column family.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name by which the new column family should be referred to within the
    /// table, e.g. "foobar" rather than "<table_name>/columnFamilies/foobar".
    #[prost(string, tag = "2")]
    pub column_family_id: ::prost::alloc::string::String,
    /// The column family to create. The `name` field must be left blank.
    #[prost(message, optional, tag = "3")]
    pub column_family: ::core::option::Option<ColumnFamily>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteColumnFamilyRequest {
    /// The unique name of the column family to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BulkDeleteRowsRequest {
    /// The unique name of the table on which to perform the bulk delete
    #[prost(string, tag = "1")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(oneof = "bulk_delete_rows_request::Target", tags = "2, 3")]
    pub target: ::core::option::Option<bulk_delete_rows_request::Target>,
}
/// Nested message and enum types in `BulkDeleteRowsRequest`.
pub mod bulk_delete_rows_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Delete all rows that start with this row key prefix. Prefix cannot be
        /// zero length.
        #[prost(bytes, tag = "2")]
        RowKeyPrefix(::prost::alloc::vec::Vec<u8>),
        /// Delete all rows in the table. Setting this to false is a no-op.
        #[prost(bool, tag = "3")]
        DeleteAllDataFromTable(bool),
    }
}
#[doc = r" Generated client implementations."]
pub mod bigtable_table_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for creating, configuring, and deleting Cloud Bigtable tables."]
    #[doc = " Provides access to the table schemas only, not the data stored within the"]
    #[doc = " tables."]
    pub struct BigtableTableServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigtableTableServiceClient<T>
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
        #[doc = " Creates a new table, to be served from a specified cluster."]
        #[doc = " The table can be created with a full set of initial column families,"]
        #[doc = " specified in the request."]
        pub async fn create_table(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTableRequest>,
        ) -> Result<tonic::Response<super::Table>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.table.v1.BigtableTableService/CreateTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the names of all tables served from a specified cluster."]
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
                "/google.bigtable.admin.table.v1.BigtableTableService/ListTables",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the schema of the specified table, including its column families."]
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
                "/google.bigtable.admin.table.v1.BigtableTableService/GetTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently deletes a specified table and all of its data."]
        pub async fn delete_table(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTableRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.table.v1.BigtableTableService/DeleteTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Changes the name of a specified table."]
        #[doc = " Cannot be used to move tables between clusters, zones, or projects."]
        pub async fn rename_table(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameTableRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.table.v1.BigtableTableService/RenameTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new column family within a specified table."]
        pub async fn create_column_family(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateColumnFamilyRequest>,
        ) -> Result<tonic::Response<super::ColumnFamily>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.table.v1.BigtableTableService/CreateColumnFamily",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Changes the configuration of a specified column family."]
        pub async fn update_column_family(
            &mut self,
            request: impl tonic::IntoRequest<super::ColumnFamily>,
        ) -> Result<tonic::Response<super::ColumnFamily>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.table.v1.BigtableTableService/UpdateColumnFamily",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently deletes a specified column family and all of its data."]
        pub async fn delete_column_family(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteColumnFamilyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.table.v1.BigtableTableService/DeleteColumnFamily",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete all rows in a table corresponding to a particular prefix"]
        pub async fn bulk_delete_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::BulkDeleteRowsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.table.v1.BigtableTableService/BulkDeleteRows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BigtableTableServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BigtableTableServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BigtableTableServiceClient {{ ... }}")
        }
    }
}
