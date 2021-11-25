/// A single book in the library.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Book {
    /// The resource name of the book.
    /// Book names have the form `shelves/{shelf_id}/books/{book_id}`.
    /// The name is ignored when creating a book.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the book author.
    #[prost(string, tag = "2")]
    pub author: ::prost::alloc::string::String,
    /// The title of the book.
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
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
    pub name: ::prost::alloc::string::String,
    /// The theme of the shelf
    #[prost(string, tag = "2")]
    pub theme: ::prost::alloc::string::String,
}
/// Request message for LibraryService.CreateShelf.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShelfRequest {
    /// The shelf to create.
    #[prost(message, optional, tag = "1")]
    pub shelf: ::core::option::Option<Shelf>,
}
/// Request message for LibraryService.GetShelf.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShelfRequest {
    /// The name of the shelf to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
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
    /// \[ListShelvesResponse.next_page_token][google.example.library.v1.ListShelvesResponse.next_page_token\]
    /// returned from the previous call to `ListShelves` method.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for LibraryService.ListShelves.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShelvesResponse {
    /// The list of shelves.
    #[prost(message, repeated, tag = "1")]
    pub shelves: ::prost::alloc::vec::Vec<Shelf>,
    /// A token to retrieve next page of results.
    /// Pass this value in the
    /// \[ListShelvesRequest.page_token][google.example.library.v1.ListShelvesRequest.page_token\]
    /// field in the subsequent call to `ListShelves` method to retrieve the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for LibraryService.DeleteShelf.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShelfRequest {
    /// The name of the shelf to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Describes the shelf being removed (other_shelf_name) and updated
/// (name) in this merge.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeShelvesRequest {
    /// The name of the shelf we're adding books to.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the shelf we're removing books from and deleting.
    #[prost(string, tag = "2")]
    pub other_shelf: ::prost::alloc::string::String,
}
/// Request message for LibraryService.CreateBook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBookRequest {
    /// The name of the shelf in which the book is created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The book to create.
    #[prost(message, optional, tag = "2")]
    pub book: ::core::option::Option<Book>,
}
/// Request message for LibraryService.GetBook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBookRequest {
    /// The name of the book to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for LibraryService.ListBooks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBooksRequest {
    /// The name of the shelf whose books we'd like to list.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Requested page size. Server may return fewer books than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results the server should return.
    /// Typically, this is the value of
    /// \[ListBooksResponse.next_page_token][google.example.library.v1.ListBooksResponse.next_page_token\].
    /// returned from the previous call to `ListBooks` method.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for LibraryService.ListBooks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBooksResponse {
    /// The list of books.
    #[prost(message, repeated, tag = "1")]
    pub books: ::prost::alloc::vec::Vec<Book>,
    /// A token to retrieve next page of results.
    /// Pass this value in the
    /// \[ListBooksRequest.page_token][google.example.library.v1.ListBooksRequest.page_token\]
    /// field in the subsequent call to `ListBooks` method to retrieve the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for LibraryService.UpdateBook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBookRequest {
    /// The name of the book to update.
    #[prost(message, optional, tag = "1")]
    pub book: ::core::option::Option<Book>,
    /// Required. Mask of fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for LibraryService.DeleteBook.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBookRequest {
    /// The name of the book to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Describes what book to move (name) and what shelf we're moving it
/// to (other_shelf_name).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveBookRequest {
    /// The name of the book to move.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the destination shelf.
    #[prost(string, tag = "2")]
    pub other_shelf_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod library_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " This API represents a simple digital library. It lets you manage Shelf"]
    #[doc = " resources and Book resources in the library. It defines the following"]
    #[doc = " resource model:"]
    #[doc = ""]
    #[doc = " - The API has a collection of [Shelf][google.example.library.v1.Shelf]"]
    #[doc = "   resources, named `shelves/*`"]
    #[doc = ""]
    #[doc = " - Each Shelf has a collection of [Book][google.example.library.v1.Book]"]
    #[doc = "   resources, named `shelves/*/books/*`"]
    #[derive(Debug, Clone)]
    pub struct LibraryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LibraryServiceClient<T>
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
        ) -> LibraryServiceClient<InterceptedService<T, F>>
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
            LibraryServiceClient::new(InterceptedService::new(inner, interceptor))
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
}
