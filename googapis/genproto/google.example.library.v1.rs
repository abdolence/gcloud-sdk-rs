/// A single book in the library.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Book {
    /// The resource name of the book.
    /// Book names have the form `shelves/{shelf_id}/books/{book_id}`.
    /// The name is ignored when creating a book.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The name of the book author.
    #[prost(string, tag = "2")]
    pub author: std::string::String,
    /// The title of the book.
    #[prost(string, tag = "3")]
    pub title: std::string::String,
    /// Value indicating whether the book has been read.
    #[prost(bool, tag = "4")]
    pub read: bool,
}
/// A Shelf contains a collection of books with a theme.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shelf {
    /// The resource name of the shelf.
    /// Shelf names have the form `shelves/{shelf_id}`.
    /// The name is ignored when creating a shelf.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The theme of the shelf
    #[prost(string, tag = "2")]
    pub theme: std::string::String,
}
/// Request message for LibraryService.CreateShelf.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShelfRequest {
    /// The shelf to create.
    #[prost(message, optional, tag = "1")]
    pub shelf: ::std::option::Option<Shelf>,
}
/// Request message for LibraryService.GetShelf.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShelfRequest {
    /// The name of the shelf to retrieve.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for LibraryService.ListShelves.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShelvesRequest {
    /// Requested page size. Server may return fewer shelves than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    /// Typically, this is the value of
    /// [ListShelvesResponse.next_page_token][google.example.library.v1.ListShelvesResponse.next_page_token]
    /// returned from the previous call to `ListShelves` method.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
}
/// Response message for LibraryService.ListShelves.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShelvesResponse {
    /// The list of shelves.
    #[prost(message, repeated, tag = "1")]
    pub shelves: ::std::vec::Vec<Shelf>,
    /// A token to retrieve next page of results.
    /// Pass this value in the
    /// [ListShelvesRequest.page_token][google.example.library.v1.ListShelvesRequest.page_token]
    /// field in the subsequent call to `ListShelves` method to retrieve the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for LibraryService.DeleteShelf.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShelfRequest {
    /// The name of the shelf to delete.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Describes the shelf being removed (other_shelf_name) and updated
/// (name) in this merge.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeShelvesRequest {
    /// The name of the shelf we're adding books to.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The name of the shelf we're removing books from and deleting.
    #[prost(string, tag = "2")]
    pub other_shelf_name: std::string::String,
}
/// Request message for LibraryService.CreateBook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBookRequest {
    /// The name of the shelf in which the book is created.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The book to create.
    #[prost(message, optional, tag = "2")]
    pub book: ::std::option::Option<Book>,
}
/// Request message for LibraryService.GetBook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBookRequest {
    /// The name of the book to retrieve.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for LibraryService.ListBooks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBooksRequest {
    /// The name of the shelf whose books we'd like to list.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Requested page size. Server may return fewer books than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    /// Typically, this is the value of
    /// [ListBooksResponse.next_page_token][google.example.library.v1.ListBooksResponse.next_page_token].
    /// returned from the previous call to `ListBooks` method.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for LibraryService.ListBooks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBooksResponse {
    /// The list of books.
    #[prost(message, repeated, tag = "1")]
    pub books: ::std::vec::Vec<Book>,
    /// A token to retrieve next page of results.
    /// Pass this value in the
    /// [ListBooksRequest.page_token][google.example.library.v1.ListBooksRequest.page_token]
    /// field in the subsequent call to `ListBooks` method to retrieve the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for LibraryService.UpdateBook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBookRequest {
    /// The name of the book to update.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The book to update with. The name must match or be empty.
    #[prost(message, optional, tag = "2")]
    pub book: ::std::option::Option<Book>,
}
/// Request message for LibraryService.DeleteBook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBookRequest {
    /// The name of the book to delete.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Describes what book to move (name) and what shelf we're moving it
/// to (other_shelf_name).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveBookRequest {
    /// The name of the book to move.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The name of the destination shelf.
    #[prost(string, tag = "2")]
    pub other_shelf_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod library_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " This API represents a simple digital library.  It lets you manage Shelf"]
    #[doc = " resources and Book resources in the library. It defines the following"]
    #[doc = " resource model:"]
    #[doc = ""]
    #[doc = " - The API has a collection of [Shelf][google.example.library.v1.Shelf]"]
    #[doc = "   resources, named `shelves/*`"]
    #[doc = ""]
    #[doc = " - Each Shelf has a collection of [Book][google.example.library.v1.Book]"]
    #[doc = "   resources, named `shelves/*/books/*`"]
    pub struct LibraryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LibraryServiceClient<T>
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
        #[doc = " Creates a shelf, and returns the new Shelf."]
        pub async fn create_shelf(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateShelfRequest>,
        ) -> Result<tonic::Response<super::Shelf>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/CreateShelf",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a shelf. Returns NOT_FOUND if the shelf does not exist."]
        pub async fn get_shelf(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShelfRequest>,
        ) -> Result<tonic::Response<super::Shelf>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/GetShelf",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists shelves. The order is unspecified but deterministic. Newly created"]
        #[doc = " shelves will not necessarily be added to the end of this list."]
        pub async fn list_shelves(
            &mut self,
            request: impl tonic::IntoRequest<super::ListShelvesRequest>,
        ) -> Result<tonic::Response<super::ListShelvesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/ListShelves",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a shelf. Returns NOT_FOUND if the shelf does not exist."]
        pub async fn delete_shelf(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShelfRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/DeleteShelf",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Merges two shelves by adding all books from the shelf named"]
        #[doc = " `other_shelf_name` to shelf `name`, and deletes"]
        #[doc = " `other_shelf_name`. Returns the updated shelf."]
        #[doc = " The book ids of the moved books may not be the same as the original books."]
        #[doc = ""]
        #[doc = " Returns NOT_FOUND if either shelf does not exist."]
        #[doc = " This call is a no-op if the specified shelves are the same."]
        pub async fn merge_shelves(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeShelvesRequest>,
        ) -> Result<tonic::Response<super::Shelf>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/MergeShelves",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a book, and returns the new Book."]
        pub async fn create_book(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBookRequest>,
        ) -> Result<tonic::Response<super::Book>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/CreateBook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a book. Returns NOT_FOUND if the book does not exist."]
        pub async fn get_book(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBookRequest>,
        ) -> Result<tonic::Response<super::Book>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/GetBook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists books in a shelf. The order is unspecified but deterministic. Newly"]
        #[doc = " created books will not necessarily be added to the end of this list."]
        #[doc = " Returns NOT_FOUND if the shelf does not exist."]
        pub async fn list_books(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBooksRequest>,
        ) -> Result<tonic::Response<super::ListBooksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/ListBooks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a book. Returns NOT_FOUND if the book does not exist."]
        pub async fn delete_book(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBookRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/DeleteBook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a book. Returns INVALID_ARGUMENT if the name of the book"]
        #[doc = " is non-empty and does not equal the existing name."]
        pub async fn update_book(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBookRequest>,
        ) -> Result<tonic::Response<super::Book>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/UpdateBook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Moves a book to another shelf, and returns the new book. The book"]
        #[doc = " id of the new book may not be the same as the original book."]
        pub async fn move_book(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveBookRequest>,
        ) -> Result<tonic::Response<super::Book>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.library.v1.LibraryService/MoveBook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for LibraryServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for LibraryServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LibraryServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod library_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with LibraryServiceServer."]
    #[async_trait]
    pub trait LibraryService: Send + Sync + 'static {
        #[doc = " Creates a shelf, and returns the new Shelf."]
        async fn create_shelf(
            &self,
            request: tonic::Request<super::CreateShelfRequest>,
        ) -> Result<tonic::Response<super::Shelf>, tonic::Status>;
        #[doc = " Gets a shelf. Returns NOT_FOUND if the shelf does not exist."]
        async fn get_shelf(
            &self,
            request: tonic::Request<super::GetShelfRequest>,
        ) -> Result<tonic::Response<super::Shelf>, tonic::Status>;
        #[doc = " Lists shelves. The order is unspecified but deterministic. Newly created"]
        #[doc = " shelves will not necessarily be added to the end of this list."]
        async fn list_shelves(
            &self,
            request: tonic::Request<super::ListShelvesRequest>,
        ) -> Result<tonic::Response<super::ListShelvesResponse>, tonic::Status>;
        #[doc = " Deletes a shelf. Returns NOT_FOUND if the shelf does not exist."]
        async fn delete_shelf(
            &self,
            request: tonic::Request<super::DeleteShelfRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Merges two shelves by adding all books from the shelf named"]
        #[doc = " `other_shelf_name` to shelf `name`, and deletes"]
        #[doc = " `other_shelf_name`. Returns the updated shelf."]
        #[doc = " The book ids of the moved books may not be the same as the original books."]
        #[doc = ""]
        #[doc = " Returns NOT_FOUND if either shelf does not exist."]
        #[doc = " This call is a no-op if the specified shelves are the same."]
        async fn merge_shelves(
            &self,
            request: tonic::Request<super::MergeShelvesRequest>,
        ) -> Result<tonic::Response<super::Shelf>, tonic::Status>;
        #[doc = " Creates a book, and returns the new Book."]
        async fn create_book(
            &self,
            request: tonic::Request<super::CreateBookRequest>,
        ) -> Result<tonic::Response<super::Book>, tonic::Status>;
        #[doc = " Gets a book. Returns NOT_FOUND if the book does not exist."]
        async fn get_book(
            &self,
            request: tonic::Request<super::GetBookRequest>,
        ) -> Result<tonic::Response<super::Book>, tonic::Status>;
        #[doc = " Lists books in a shelf. The order is unspecified but deterministic. Newly"]
        #[doc = " created books will not necessarily be added to the end of this list."]
        #[doc = " Returns NOT_FOUND if the shelf does not exist."]
        async fn list_books(
            &self,
            request: tonic::Request<super::ListBooksRequest>,
        ) -> Result<tonic::Response<super::ListBooksResponse>, tonic::Status>;
        #[doc = " Deletes a book. Returns NOT_FOUND if the book does not exist."]
        async fn delete_book(
            &self,
            request: tonic::Request<super::DeleteBookRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates a book. Returns INVALID_ARGUMENT if the name of the book"]
        #[doc = " is non-empty and does not equal the existing name."]
        async fn update_book(
            &self,
            request: tonic::Request<super::UpdateBookRequest>,
        ) -> Result<tonic::Response<super::Book>, tonic::Status>;
        #[doc = " Moves a book to another shelf, and returns the new book. The book"]
        #[doc = " id of the new book may not be the same as the original book."]
        async fn move_book(
            &self,
            request: tonic::Request<super::MoveBookRequest>,
        ) -> Result<tonic::Response<super::Book>, tonic::Status>;
    }
    #[doc = " This API represents a simple digital library.  It lets you manage Shelf"]
    #[doc = " resources and Book resources in the library. It defines the following"]
    #[doc = " resource model:"]
    #[doc = ""]
    #[doc = " - The API has a collection of [Shelf][google.example.library.v1.Shelf]"]
    #[doc = "   resources, named `shelves/*`"]
    #[doc = ""]
    #[doc = " - Each Shelf has a collection of [Book][google.example.library.v1.Book]"]
    #[doc = "   resources, named `shelves/*/books/*`"]
    #[derive(Debug)]
    pub struct LibraryServiceServer<T: LibraryService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: LibraryService> LibraryServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for LibraryServiceServer<T>
    where
        T: LibraryService,
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
                "/google.example.library.v1.LibraryService/CreateShelf" => {
                    #[allow(non_camel_case_types)]
                    struct CreateShelfSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::CreateShelfRequest>
                        for CreateShelfSvc<T>
                    {
                        type Response = super::Shelf;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateShelfRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_shelf(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateShelfSvc(inner);
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
                "/google.example.library.v1.LibraryService/GetShelf" => {
                    #[allow(non_camel_case_types)]
                    struct GetShelfSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::GetShelfRequest> for GetShelfSvc<T> {
                        type Response = super::Shelf;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetShelfRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_shelf(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetShelfSvc(inner);
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
                "/google.example.library.v1.LibraryService/ListShelves" => {
                    #[allow(non_camel_case_types)]
                    struct ListShelvesSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::ListShelvesRequest>
                        for ListShelvesSvc<T>
                    {
                        type Response = super::ListShelvesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListShelvesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_shelves(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListShelvesSvc(inner);
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
                "/google.example.library.v1.LibraryService/DeleteShelf" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteShelfSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::DeleteShelfRequest>
                        for DeleteShelfSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteShelfRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_shelf(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteShelfSvc(inner);
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
                "/google.example.library.v1.LibraryService/MergeShelves" => {
                    #[allow(non_camel_case_types)]
                    struct MergeShelvesSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::MergeShelvesRequest>
                        for MergeShelvesSvc<T>
                    {
                        type Response = super::Shelf;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MergeShelvesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).merge_shelves(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MergeShelvesSvc(inner);
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
                "/google.example.library.v1.LibraryService/CreateBook" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBookSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::CreateBookRequest> for CreateBookSvc<T> {
                        type Response = super::Book;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBookRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_book(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateBookSvc(inner);
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
                "/google.example.library.v1.LibraryService/GetBook" => {
                    #[allow(non_camel_case_types)]
                    struct GetBookSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::GetBookRequest> for GetBookSvc<T> {
                        type Response = super::Book;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBookRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_book(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBookSvc(inner);
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
                "/google.example.library.v1.LibraryService/ListBooks" => {
                    #[allow(non_camel_case_types)]
                    struct ListBooksSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::ListBooksRequest> for ListBooksSvc<T> {
                        type Response = super::ListBooksResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBooksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_books(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListBooksSvc(inner);
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
                "/google.example.library.v1.LibraryService/DeleteBook" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBookSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::DeleteBookRequest> for DeleteBookSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBookRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_book(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteBookSvc(inner);
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
                "/google.example.library.v1.LibraryService/UpdateBook" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBookSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::UpdateBookRequest> for UpdateBookSvc<T> {
                        type Response = super::Book;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateBookRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_book(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateBookSvc(inner);
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
                "/google.example.library.v1.LibraryService/MoveBook" => {
                    #[allow(non_camel_case_types)]
                    struct MoveBookSvc<T: LibraryService>(pub Arc<T>);
                    impl<T: LibraryService> tonic::server::UnaryService<super::MoveBookRequest> for MoveBookSvc<T> {
                        type Response = super::Book;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveBookRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).move_book(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MoveBookSvc(inner);
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
    impl<T: LibraryService> Clone for LibraryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: LibraryService> Clone for _Inner<T> {
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
