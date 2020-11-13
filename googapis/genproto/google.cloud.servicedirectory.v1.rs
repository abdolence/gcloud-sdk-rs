/// An individual endpoint that provides a
/// [service][google.cloud.servicedirectory.v1.Service]. The service must
/// already exist to create an endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    /// Immutable. The resource name for the endpoint in the format
    /// 'projects/*/locations/*/namespaces/*/services/*/endpoints/*'.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. An IPv4 or IPv6 address. Service Directory will reject bad
    /// addresses like:
    ///   "8.8.8"
    ///   "8.8.8.8:53"
    ///   "test:bad:address"
    ///   "[::1]"
    ///   "[::1]:8080"
    /// Limited to 45 characters.
    #[prost(string, tag = "2")]
    pub address: std::string::String,
    /// Optional. Service Directory will reject values outside of [0, 65535].
    #[prost(int32, tag = "3")]
    pub port: i32,
    /// Optional. Annotations for the endpoint. This data can be consumed by
    /// service clients. Restrictions:
    ///  - The entire annotations dictionary may contain up to 512 characters,
    ///    spread accoss all key-value pairs. Annotations that goes beyond any
    ///    these limits will be rejected.
    ///  - Valid annotation keys have two segments: an optional prefix and name,
    ///    separated by a slash (/). The name segment is required and must be 63
    ///    characters or less, beginning and ending with an alphanumeric character
    ///    ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and
    ///    alphanumerics between. The prefix is optional. If specified, the prefix
    ///    must be a DNS subdomain: a series of DNS labels separated by dots (.),
    ///    not longer than 253 characters in total, followed by a slash (/).
    ///    Annotations that fails to meet these requirements will be rejected.
    ///  - The '(*.)google.com/' and '(*.)googleapis.com/' prefixes are reserved
    ///    for system annotations managed by Service Directory. If the user tries
    ///    to write to these keyspaces, those entries will be silently ignored by
    ///    the system.
    /// Note: This field is equivalent to the 'metadata' field in the v1beta1 API.
    /// They have the same syntax and read/write to the same location in Service
    /// Directory.
    #[prost(map = "string, string", tag = "5")]
    pub annotations: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// An individual service. A service contains a name and optional metadata.
/// A service must exist before
/// [endpoints][google.cloud.servicedirectory.v1.Endpoint] can be
/// added to it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Immutable. The resource name for the service in the format
    /// 'projects/*/locations/*/namespaces/*/services/*'.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. Annotations for the service. This data can be consumed by service
    /// clients.
    /// Restrictions:
    ///  - The entire annotations dictionary may contain up to 2000 characters,
    ///    spread accoss all key-value pairs. Annotations that goes beyond any
    ///    these limits will be rejected.
    ///  - Valid annotation keys have two segments: an optional prefix and name,
    ///    separated by a slash (/). The name segment is required and must be 63
    ///    characters or less, beginning and ending with an alphanumeric character
    ///    ([a-z0-9A-Z]) with dashes (-), underscores (_), dots (.), and
    ///    alphanumerics between. The prefix is optional. If specified, the prefix
    ///    must be a DNS subdomain: a series of DNS labels separated by dots (.),
    ///    not longer than 253 characters in total, followed by a slash (/).
    ///    Annotations that fails to meet these requirements will be rejected.
    ///  - The '(*.)google.com/' and '(*.)googleapis.com/' prefixes are reserved
    ///    for system annotations managed by Service Directory. If the user tries
    ///    to write to these keyspaces, those entries will be silently ignored by
    ///    the system.
    /// Note: This field is equivalent to the 'metadata' field in the v1beta1 API.
    /// They have the same syntax and read/write to the same location in Service
    /// Directory.
    #[prost(map = "string, string", tag = "4")]
    pub annotations: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Output only. Endpoints associated with this service. Returned on
    /// LookupService.Resolve. Control plane clients should use
    /// RegistrationService.ListEndpoints.
    #[prost(message, repeated, tag = "3")]
    pub endpoints: ::std::vec::Vec<Endpoint>,
}
/// The request message for
/// [LookupService.ResolveService][google.cloud.servicedirectory.v1.LookupService.ResolveService].
/// Looks up a service by its name, returns the service and its endpoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveServiceRequest {
    /// Required. The name of the service to resolve.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The maximum number of endpoints to return. Defaults to 25.
    /// Maximum is 100. If a value less than one is specified, the Default is used.
    /// If a value greater than the Maximum is specified, the Maximum is used.
    #[prost(int32, tag = "2")]
    pub max_endpoints: i32,
    /// Optional. The filter applied to the endpoints of the resolved service.
    ///
    /// General filter string syntax:
    /// <field> <operator> <value> (<logical connector>)
    /// <field> can be "name" or "metadata.<key>" for map field.
    /// <operator> can be "<, >, <=, >=, !=, =, :". Of which ":" means HAS and is
    /// roughly the same as "=".
    /// <value> must be the same data type as the field.
    /// <logical connector> can be "AND, OR, NOT".
    ///
    /// Examples of valid filters:
    /// * "metadata.owner" returns Endpoints that have a label with the
    ///   key "owner", this is the same as "metadata:owner"
    /// * "metadata.protocol=gRPC" returns Endpoints that have key/value
    ///   "protocol=gRPC"
    /// * "metadata.owner!=sd AND metadata.foo=bar" returns
    ///   Endpoints that have "owner" field in metadata with a value that is not
    ///   "sd" AND have the key/value foo=bar.
    #[prost(string, tag = "3")]
    pub endpoint_filter: std::string::String,
}
/// The response message for
/// [LookupService.ResolveService][google.cloud.servicedirectory.v1.LookupService.ResolveService].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveServiceResponse {
    #[prost(message, optional, tag = "1")]
    pub service: ::std::option::Option<Service>,
}
#[doc = r" Generated client implementations."]
pub mod lookup_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service Directory API for looking up service data at runtime."]
    pub struct LookupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LookupServiceClient<T>
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
        #[doc = " Returns a [service][google.cloud.servicedirectory.v1.Service] and its"]
        #[doc = " associated endpoints."]
        #[doc = " Resolving a service is not considered an active developer method."]
        pub async fn resolve_service(
            &mut self,
            request: impl tonic::IntoRequest<super::ResolveServiceRequest>,
        ) -> Result<tonic::Response<super::ResolveServiceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.LookupService/ResolveService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for LookupServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for LookupServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LookupServiceClient {{ ... }}")
        }
    }
}
/// A container for [services][google.cloud.servicedirectory.v1.Service].
/// Namespaces allow administrators to group services together and define
/// permissions for a collection of services.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Namespace {
    /// Immutable. The resource name for the namespace in the format
    /// 'projects/*/locations/*/namespaces/*'.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. Resource labels associated with this Namespace.
    /// No more than 64 user labels can be associated with a given resource.  Label
    /// keys and values can be no longer than 63 characters.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// The request message for
/// [RegistrationService.CreateNamespace][google.cloud.servicedirectory.v1.RegistrationService.CreateNamespace].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNamespaceRequest {
    /// Required. The resource name of the project and location the namespace
    /// will be created in.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The Resource ID must be 1-63 characters long, and comply with
    /// <a href="https://www.ietf.org/rfc/rfc1035.txt" target="_blank">RFC1035</a>.
    /// Specifically, the name must be 1-63 characters long and match the regular
    /// expression `[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?` which means the first
    /// character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which
    /// cannot be a dash.
    #[prost(string, tag = "2")]
    pub namespace_id: std::string::String,
    /// Required. A namespace with initial fields set.
    #[prost(message, optional, tag = "3")]
    pub namespace: ::std::option::Option<Namespace>,
}
/// The request message for
/// [RegistrationService.ListNamespaces][google.cloud.servicedirectory.v1.RegistrationService.ListNamespaces].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNamespacesRequest {
    /// Required. The resource name of the project and location whose namespaces
    /// we'd like to list.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. The filter to list result by.
    ///
    /// General filter string syntax:
    /// <field> <operator> <value> (<logical connector>)
    /// <field> can be "name", or "labels.<key>" for map field.
    /// <operator> can be "<, >, <=, >=, !=, =, :". Of which ":" means HAS, and
    /// is roughly the same as "=".
    /// <value> must be the same data type as field.
    /// <logical connector> can be "AND, OR, NOT".
    ///
    /// Examples of valid filters:
    /// * "labels.owner" returns Namespaces that have a label with the key "owner"
    ///   this is the same as "labels:owner".
    /// * "labels.protocol=gRPC" returns Namespaces that have key/value
    ///   "protocol=gRPC".
    /// * "name>projects/my-project/locations/us-east/namespaces/namespace-c"
    ///   returns Namespaces that have name that is alphabetically later than the
    ///   string, so "namespace-e" will be returned but "namespace-a" will not be.
    /// * "labels.owner!=sd AND labels.foo=bar" returns Namespaces that have
    ///   "owner" in label key but value is not "sd" AND have key/value foo=bar.
    /// * "doesnotexist.foo=bar" returns an empty list. Note that Namespace doesn't
    ///   have a field called "doesnotexist". Since the filter does not match any
    ///   Namespaces, it returns no results.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. The order to list result by.
    ///
    /// General order by string syntax:
    /// <field> (<asc|desc>) (,)
    /// <field> allows values {"name"}
    /// <asc/desc> ascending or descending order by <field>. If this is left
    /// blank, "asc" is used.
    /// Note that an empty order_by string result in default order, which is order
    /// by name in ascending order.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// The response message for
/// [RegistrationService.ListNamespaces][google.cloud.servicedirectory.v1.RegistrationService.ListNamespaces].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNamespacesResponse {
    /// The list of namespaces.
    #[prost(message, repeated, tag = "1")]
    pub namespaces: ::std::vec::Vec<Namespace>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for
/// [RegistrationService.GetNamespace][google.cloud.servicedirectory.v1.RegistrationService.GetNamespace].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNamespaceRequest {
    /// Required. The name of the namespace to retrieve.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for
/// [RegistrationService.UpdateNamespace][google.cloud.servicedirectory.v1.RegistrationService.UpdateNamespace].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNamespaceRequest {
    /// Required. The updated namespace.
    #[prost(message, optional, tag = "1")]
    pub namespace: ::std::option::Option<Namespace>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for
/// [RegistrationService.DeleteNamespace][google.cloud.servicedirectory.v1.RegistrationService.DeleteNamespace].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNamespaceRequest {
    /// Required. The name of the namespace to delete.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for
/// [RegistrationService.CreateService][google.cloud.servicedirectory.v1.RegistrationService.CreateService].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Required. The resource name of the namespace this service will belong to.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The Resource ID must be 1-63 characters long, and comply with
    /// <a href="https://www.ietf.org/rfc/rfc1035.txt" target="_blank">RFC1035</a>.
    /// Specifically, the name must be 1-63 characters long and match the regular
    /// expression `[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?` which means the first
    /// character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which
    /// cannot be a dash.
    #[prost(string, tag = "2")]
    pub service_id: std::string::String,
    /// Required. A service  with initial fields set.
    #[prost(message, optional, tag = "3")]
    pub service: ::std::option::Option<Service>,
}
/// The request message for
/// [RegistrationService.ListServices][google.cloud.servicedirectory.v1.RegistrationService.ListServices].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Required. The resource name of the namespace whose services we'd
    /// like to list.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. The filter to list result by.
    ///
    /// General filter string syntax:
    /// <field> <operator> <value> (<logical connector>)
    /// <field> can be "name", or "metadata.<key>" for map field.
    /// <operator> can be "<, >, <=, >=, !=, =, :". Of which ":" means HAS, and
    /// is roughly the same as "=".
    /// <value> must be the same data type as field.
    /// <logical connector> can be "AND, OR, NOT".
    ///
    /// Examples of valid filters:
    /// * "metadata.owner" returns Services that have a label with the key "owner"
    ///   this is the same as "metadata:owner".
    /// * "metadata.protocol=gRPC" returns Services that have key/value
    ///   "protocol=gRPC".
    /// * "name>projects/my-project/locations/us-east/namespaces/my-namespace/services/service-c"
    ///   returns Services that have name that is alphabetically later than the
    ///   string, so "service-e" will be returned but "service-a" will not be.
    /// * "metadata.owner!=sd AND metadata.foo=bar" returns Services that have
    ///   "owner" in label key but value is not "sd" AND have key/value foo=bar.
    /// * "doesnotexist.foo=bar" returns an empty list. Note that Service doesn't
    ///   have a field called "doesnotexist". Since the filter does not match any
    ///   Services, it returns no results.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. The order to list result by.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// The response message for
/// [RegistrationService.ListServices][google.cloud.servicedirectory.v1.RegistrationService.ListServices].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The list of services.
    #[prost(message, repeated, tag = "1")]
    pub services: ::std::vec::Vec<Service>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for
/// [RegistrationService.GetService][google.cloud.servicedirectory.v1.RegistrationService.GetService].
/// This should not be used for looking up a service. Insead, use the `resolve`
/// method as it will contain all endpoints and associated metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Required. The name of the service to get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for
/// [RegistrationService.UpdateService][google.cloud.servicedirectory.v1.RegistrationService.UpdateService].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Required. The updated service.
    #[prost(message, optional, tag = "1")]
    pub service: ::std::option::Option<Service>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for
/// [RegistrationService.DeleteService][google.cloud.servicedirectory.v1.RegistrationService.DeleteService].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Required. The name of the service to delete.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for
/// [RegistrationService.CreateEndpoint][google.cloud.servicedirectory.v1.RegistrationService.CreateEndpoint].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEndpointRequest {
    /// Required. The resource name of the service that this endpoint provides.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The Resource ID must be 1-63 characters long, and comply with
    /// <a href="https://www.ietf.org/rfc/rfc1035.txt" target="_blank">RFC1035</a>.
    /// Specifically, the name must be 1-63 characters long and match the regular
    /// expression `[a-z](?:[-a-z0-9]{0,61}[a-z0-9])?` which means the first
    /// character must be a lowercase letter, and all following characters must
    /// be a dash, lowercase letter, or digit, except the last character, which
    /// cannot be a dash.
    #[prost(string, tag = "2")]
    pub endpoint_id: std::string::String,
    /// Required. A endpoint with initial fields set.
    #[prost(message, optional, tag = "3")]
    pub endpoint: ::std::option::Option<Endpoint>,
}
/// The request message for
/// [RegistrationService.ListEndpoints][google.cloud.servicedirectory.v1.RegistrationService.ListEndpoints].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointsRequest {
    /// Required. The resource name of the service whose endpoints we'd like to
    /// list.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. The filter to list result by.
    ///
    /// General filter string syntax:
    /// <field> <operator> <value> (<logical connector>)
    /// <field> can be "name", "address", "port" or "metadata.<key>" for map field.
    /// <operator> can be "<, >, <=, >=, !=, =, :". Of which ":" means HAS, and
    /// is roughly the same as "=".
    /// <value> must be the same data type as field.
    /// <logical connector> can be "AND, OR, NOT".
    ///
    /// Examples of valid filters:
    /// * "metadata.owner" returns Endpoints that have a label with the key "owner"
    ///   this is the same as "metadata:owner".
    /// * "metadata.protocol=gRPC" returns Endpoints that have key/value
    ///   "protocol=gRPC".
    /// * "address=192.108.1.105" returns Endpoints that have this address.
    /// * "port>8080" returns Endpoints that have port number larger than 8080.
    /// * "name>projects/my-project/locations/us-east/namespaces/my-namespace/services/my-service/endpoints/endpoint-c"
    ///   returns Endpoints that have name that is alphabetically later than the
    ///   string, so "endpoint-e" will be returned but "endpoint-a" will not be.
    /// * "metadata.owner!=sd AND metadata.foo=bar" returns Endpoints that have
    ///   "owner" in label key but value is not "sd" AND have key/value foo=bar.
    /// * "doesnotexist.foo=bar" returns an empty list. Note that Endpoint doesn't
    ///   have a field called "doesnotexist". Since the filter does not match any
    ///   Endpoints, it returns no results.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. The order to list result by.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// The response message for
/// [RegistrationService.ListEndpoints][google.cloud.servicedirectory.v1.RegistrationService.ListEndpoints].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointsResponse {
    /// The list of endpoints.
    #[prost(message, repeated, tag = "1")]
    pub endpoints: ::std::vec::Vec<Endpoint>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for
/// [RegistrationService.GetEndpoint][google.cloud.servicedirectory.v1.RegistrationService.GetEndpoint].
/// This should not be used to lookup endpoints at runtime. Instead, use
/// the `resolve` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointRequest {
    /// Required. The name of the endpoint to get.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for
/// [RegistrationService.UpdateEndpoint][google.cloud.servicedirectory.v1.RegistrationService.UpdateEndpoint].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEndpointRequest {
    /// Required. The updated endpoint.
    #[prost(message, optional, tag = "1")]
    pub endpoint: ::std::option::Option<Endpoint>,
    /// Required. List of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for
/// [RegistrationService.DeleteEndpoint][google.cloud.servicedirectory.v1.RegistrationService.DeleteEndpoint].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointRequest {
    /// Required. The name of the endpoint to delete.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod registration_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service Directory API for registering services. It defines the following"]
    #[doc = " resource model:"]
    #[doc = ""]
    #[doc = " - The API has a collection of"]
    #[doc = " [Namespace][google.cloud.servicedirectory.v1.Namespace]"]
    #[doc = " resources, named `projects/*/locations/*/namespaces/*`."]
    #[doc = ""]
    #[doc = " - Each Namespace has a collection of"]
    #[doc = " [Service][google.cloud.servicedirectory.v1.Service] resources, named"]
    #[doc = " `projects/*/locations/*/namespaces/*/services/*`."]
    #[doc = ""]
    #[doc = " - Each Service has a collection of"]
    #[doc = " [Endpoint][google.cloud.servicedirectory.v1.Endpoint]"]
    #[doc = " resources, named"]
    #[doc = " `projects/*/locations/*/namespaces/*/services/*/endpoints/*`."]
    pub struct RegistrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RegistrationServiceClient<T>
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
        #[doc = " Creates a namespace, and returns the new Namespace."]
        pub async fn create_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNamespaceRequest>,
        ) -> Result<tonic::Response<super::Namespace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/CreateNamespace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all namespaces."]
        pub async fn list_namespaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNamespacesRequest>,
        ) -> Result<tonic::Response<super::ListNamespacesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/ListNamespaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a namespace."]
        pub async fn get_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNamespaceRequest>,
        ) -> Result<tonic::Response<super::Namespace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/GetNamespace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a namespace."]
        pub async fn update_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNamespaceRequest>,
        ) -> Result<tonic::Response<super::Namespace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/UpdateNamespace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a namespace. This also deletes all services and endpoints in"]
        #[doc = " the namespace."]
        pub async fn delete_namespace(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNamespaceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/DeleteNamespace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a service, and returns the new Service."]
        pub async fn create_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/CreateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all services belonging to a namespace."]
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/ListServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a service."]
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/GetService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a service."]
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/UpdateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a service. This also deletes all endpoints associated with"]
        #[doc = " the service."]
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/DeleteService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a endpoint, and returns the new Endpoint."]
        pub async fn create_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEndpointRequest>,
        ) -> Result<tonic::Response<super::Endpoint>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/CreateEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all endpoints."]
        pub async fn list_endpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEndpointsRequest>,
        ) -> Result<tonic::Response<super::ListEndpointsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/ListEndpoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a endpoint."]
        pub async fn get_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointRequest>,
        ) -> Result<tonic::Response<super::Endpoint>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/GetEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a endpoint."]
        pub async fn update_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEndpointRequest>,
        ) -> Result<tonic::Response<super::Endpoint>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/UpdateEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a endpoint."]
        pub async fn delete_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEndpointRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.servicedirectory.v1.RegistrationService/DeleteEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the IAM Policy for a resource (namespace or service only)."]
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
                "/google.cloud.servicedirectory.v1.RegistrationService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the IAM Policy for a resource (namespace or service only)."]
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
                "/google.cloud.servicedirectory.v1.RegistrationService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Tests IAM permissions for a resource (namespace or service only)."]
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
                "/google.cloud.servicedirectory.v1.RegistrationService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RegistrationServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RegistrationServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RegistrationServiceClient {{ ... }}")
        }
    }
}
