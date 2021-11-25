#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerLicense {
    /// The type of API resource. This is always appsmarket#customerLicense.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The customer's license status. One of:
    ///
    /// - `ACTIVE`: The customer has a valid license.
    /// - `UNLICENSED`: There is no license: either this customer has never
    /// installed your application, or else has deleted it.
    #[prost(string, tag = "2")]
    pub state: ::prost::alloc::string::String,
    /// The ID of the application corresponding to this license query.
    #[prost(string, tag = "3")]
    pub application_id: ::prost::alloc::string::String,
    /// (Deprecated)
    #[deprecated]
    #[prost(message, repeated, tag = "4")]
    pub editions: ::prost::alloc::vec::Vec<customer_license::Editions>,
    /// The ID of the customer license.
    #[prost(string, tag = "101")]
    pub id: ::prost::alloc::string::String,
    /// The domain name of the customer.
    #[prost(string, tag = "102")]
    pub customer_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CustomerLicense`.
pub mod customer_license {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Editions {
        /// (Deprecated)
        #[deprecated]
        #[prost(string, tag = "405")]
        pub edition_id: ::prost::alloc::string::String,
        /// (Deprecated)
        #[deprecated]
        #[prost(int32, tag = "406")]
        pub seat_count: i32,
        /// (Deprecated)
        #[deprecated]
        #[prost(int32, tag = "409")]
        pub assigned_seats: i32,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LicenseNotification {
    /// The ID of the license notification.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The ID of the application according to this notification.
    #[prost(string, tag = "2")]
    pub application_id: ::prost::alloc::string::String,
    /// The time the event occurred, measuring in milliseconds since the UNIX
    /// epoch.
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
    /// The domain name of the customer corresponding to this notification.
    #[prost(string, tag = "4")]
    pub customer_id: ::prost::alloc::string::String,
    /// The type of API resource. This is always appsmarket#licenseNotification.
    #[prost(string, tag = "5")]
    pub kind: ::prost::alloc::string::String,
    /// The list of provisioning notifications.
    #[prost(message, repeated, tag = "6")]
    pub provisions: ::prost::alloc::vec::Vec<license_notification::Provisions>,
    /// The list of expiry notifications.
    #[prost(message, repeated, tag = "7")]
    pub expiries: ::prost::alloc::vec::Vec<license_notification::Expiries>,
    /// The list of reassignment notifications.
    #[prost(message, repeated, tag = "8")]
    pub reassignments: ::prost::alloc::vec::Vec<license_notification::Reassignments>,
    /// The list of deletion notifications.
    #[prost(message, repeated, tag = "9")]
    pub deletes: ::prost::alloc::vec::Vec<license_notification::Deletes>,
}
/// Nested message and enum types in `LicenseNotification`.
pub mod license_notification {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Deletes {
        #[prost(string, tag = "1")]
        pub kind: ::prost::alloc::string::String,
        /// (Deprecated)
        #[deprecated]
        #[prost(string, tag = "901")]
        pub edition_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Expiries {
        #[prost(string, tag = "1")]
        pub kind: ::prost::alloc::string::String,
        /// (Deprecated)
        #[deprecated]
        #[prost(string, tag = "701")]
        pub edition_id: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Provisions {
        #[prost(string, tag = "1")]
        pub kind: ::prost::alloc::string::String,
        /// (Deprecated)
        #[deprecated]
        #[prost(string, tag = "601")]
        pub edition_id: ::prost::alloc::string::String,
        /// The number of seats that were provisioned.
        #[prost(int64, tag = "602")]
        pub seat_count: i64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reassignments {
        #[prost(string, tag = "1")]
        pub kind: ::prost::alloc::string::String,
        /// The email address of the reassigned user.
        #[prost(string, tag = "801")]
        pub user_id: ::prost::alloc::string::String,
        #[prost(string, tag = "802")]
        pub r#type: ::prost::alloc::string::String,
        /// (Deprecated)
        #[deprecated]
        #[prost(string, tag = "803")]
        pub edition_id: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LicenseNotificationList {
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The list of notifications. One or more of:
    ///
    /// - `provisions`: A new license of the application has been provisioned.
    /// - `expiries`: A license of the application has expired.
    /// - `deletions`: An application has been deleted from a domain.
    /// - `reassignments`: An administrator has assigned or revoked a seat license
    /// for the application on the provided domain.
    #[prost(message, repeated, tag = "1007")]
    pub notifications: ::prost::alloc::vec::Vec<LicenseNotification>,
    /// The token used to continue querying for notifications after the final
    /// notification in the current result set.
    #[prost(string, tag = "100602")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLicense {
    /// The type of API resource. This is always appsmarket#userLicense.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The domain administrator has activated the application for this domain.
    #[prost(bool, tag = "2")]
    pub enabled: bool,
    /// The user's licensing status. One of:
    ///
    /// - `ACTIVE`: The user has a valid license and should be permitted to use the
    /// application.
    /// - `UNLICENSED`: The administrator of this user's domain never assigned a
    /// seat for the application to this user.
    /// - `EXPIRED`: The administrator assigned a seat to this user, but the
    /// license is expired.
    #[prost(string, tag = "3")]
    pub state: ::prost::alloc::string::String,
    /// (Deprecated)
    #[deprecated]
    #[prost(string, tag = "4")]
    pub edition_id: ::prost::alloc::string::String,
    /// The domain name of the user.
    #[prost(string, tag = "5")]
    pub customer_id: ::prost::alloc::string::String,
    /// The ID of the application corresponding to the license query.
    #[prost(string, tag = "6")]
    pub application_id: ::prost::alloc::string::String,
    /// The ID of user license.
    #[prost(string, tag = "101")]
    pub id: ::prost::alloc::string::String,
    /// The email address of the user.
    #[prost(string, tag = "102")]
    pub user_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerLicenseGetRequest {
    /// Application Id
    #[prost(string, tag = "1")]
    pub application_id: ::prost::alloc::string::String,
    /// Customer Id
    #[prost(string, tag = "2")]
    pub customer_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LicenseNotificationListRequest {
    /// Application Id
    #[prost(string, tag = "1")]
    pub application_id: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub max_results: u32,
    #[prost(string, tag = "3")]
    pub start_token: ::prost::alloc::string::String,
    /// Timestamp in milliseconds since epoch
    #[prost(uint64, tag = "4")]
    pub timestamp: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLicenseGetRequest {
    /// Application Id
    #[prost(string, tag = "1")]
    pub application_id: ::prost::alloc::string::String,
    /// User Id
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_license_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CustomerLicenseServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerLicenseServiceClient<T>
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
        ) -> CustomerLicenseServiceClient<InterceptedService<T, F>>
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
            CustomerLicenseServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Get the status of a license for a customer to determine if they have access"]
        #[doc = " for a given app."]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::CustomerLicenseGetRequest>,
        ) -> Result<tonic::Response<super::CustomerLicense>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ccc.hosted.marketplace.v2.CustomerLicenseService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod license_notification_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct LicenseNotificationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LicenseNotificationServiceClient<T>
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
        ) -> LicenseNotificationServiceClient<InterceptedService<T, F>>
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
            LicenseNotificationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Get a list of licensing notifications with regards to a given app."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::LicenseNotificationListRequest>,
        ) -> Result<tonic::Response<super::LicenseNotificationList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ccc.hosted.marketplace.v2.LicenseNotificationService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod user_license_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct UserLicenseServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserLicenseServiceClient<T>
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
        ) -> UserLicenseServiceClient<InterceptedService<T, F>>
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
            UserLicenseServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Get the user's licensing status for their permission to use a given app."]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::UserLicenseGetRequest>,
        ) -> Result<tonic::Response<super::UserLicense>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/ccc.hosted.marketplace.v2.UserLicenseService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
