/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Specification of a port-based selector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficPortSelector {
    /// Optional. A list of ports. Can be port numbers or port range
    /// (example, \[80-90\] specifies all ports from 80 to 90, including
    /// 80 and 90) or named ports or * to specify all ports. If the
    /// list is empty, all ports are selected.
    #[prost(string, repeated, tag = "1")]
    pub ports: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A definition of a matcher that selects endpoints to which the policies
/// should be applied.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointMatcher {
    /// Specifies type of the matcher used for this endpoint matcher.
    #[prost(oneof = "endpoint_matcher::MatcherType", tags = "1")]
    pub matcher_type: ::core::option::Option<endpoint_matcher::MatcherType>,
}
/// Nested message and enum types in `EndpointMatcher`.
pub mod endpoint_matcher {
    /// The matcher that is based on node metadata presented by xDS clients.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetadataLabelMatcher {
        /// Specifies how matching should be done.
        ///
        /// Supported values are:
        /// MATCH_ANY: At least one of the Labels specified in the
        ///   matcher should match the metadata presented by xDS client.
        /// MATCH_ALL: The metadata presented by the xDS client should
        ///   contain all of the labels specified here.
        ///
        /// The selection is determined based on the best match. For
        /// example, suppose there are three EndpointPolicy
        /// resources P1, P2 and P3 and if P1 has a the matcher as
        /// MATCH_ANY <A:1, B:1>, P2 has MATCH_ALL <A:1,B:1>, and P3 has
        /// MATCH_ALL <A:1,B:1,C:1>.
        ///
        /// If a client with label <A:1> connects, the config from P1
        /// will be selected.
        ///
        /// If a client with label <A:1,B:1> connects, the config from P2
        /// will be selected.
        ///
        /// If a client with label <A:1,B:1,C:1> connects, the config
        /// from P3 will be selected.
        ///
        /// If there is more than one best match, (for example, if a
        /// config P4 with selector <A:1,D:1> exists and if a client with
        /// label <A:1,B:1,D:1> connects), an error will be thrown.
        #[prost(
            enumeration = "metadata_label_matcher::MetadataLabelMatchCriteria",
            tag = "1"
        )]
        pub metadata_label_match_criteria: i32,
        /// The list of label value pairs that must match labels in the
        /// provided metadata based on filterMatchCriteria This list can
        /// have at most 64 entries. The list can be empty if the match
        /// criteria is MATCH_ANY, to specify a wildcard match (i.e this
        /// matches any client).
        #[prost(message, repeated, tag = "2")]
        pub metadata_labels: ::prost::alloc::vec::Vec<metadata_label_matcher::MetadataLabels>,
    }
    /// Nested message and enum types in `MetadataLabelMatcher`.
    pub mod metadata_label_matcher {
        /// Defines a name-pair value for a single label.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetadataLabels {
            /// Required. Label name presented as key in xDS Node Metadata.
            #[prost(string, tag = "1")]
            pub label_name: ::prost::alloc::string::String,
            /// Required. Label value presented as value corresponding to the above
            /// key, in xDS Node Metadata.
            #[prost(string, tag = "2")]
            pub label_value: ::prost::alloc::string::String,
        }
        /// Possible criteria values that define logic of how matching is made.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum MetadataLabelMatchCriteria {
            /// Default value. Should not be used.
            Unspecified = 0,
            /// At least one of the Labels specified in the matcher should match the
            /// metadata presented by xDS client.
            MatchAny = 1,
            /// The metadata presented by the xDS client should contain all of the
            /// labels specified here.
            MatchAll = 2,
        }
    }
    /// Specifies type of the matcher used for this endpoint matcher.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatcherType {
        /// The matcher is based on node metadata presented by xDS clients.
        #[prost(message, tag = "1")]
        MetadataLabelMatcher(MetadataLabelMatcher),
    }
}
/// EndpointPolicy is a resource that helps apply desired configuration
/// on the endpoints that match specific criteria.
/// For example, this resource can be used to apply "authentication config"
/// an all endpoints that serve on port 8080.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndpointPolicy {
    /// Required. Name of the EndpointPolicy resource. It matches pattern
    /// `projects/{project}/locations/global/endpointPolicies/{endpoint_policy}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Set of label tags associated with the EndpointPolicy resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. The type of endpoint policy. This is primarily used to validate
    /// the configuration.
    #[prost(enumeration = "endpoint_policy::EndpointPolicyType", tag = "5")]
    pub r#type: i32,
    /// Optional. This field specifies the URL of AuthorizationPolicy resource that
    /// applies authorization policies to the inbound traffic at the
    /// matched endpoints. Refer to Authorization. If this field is not
    /// specified, authorization is disabled(no authz checks) for this
    /// endpoint. Applicable only when EndpointPolicyType is
    /// SIDECAR_PROXY.
    #[prost(string, tag = "7")]
    pub authorization_policy: ::prost::alloc::string::String,
    /// Required. A matcher that selects endpoints to which the policies should be applied.
    #[prost(message, optional, tag = "9")]
    pub endpoint_matcher: ::core::option::Option<EndpointMatcher>,
    /// Optional. Port selector for the (matched) endpoints. If no port selector is
    /// provided, the matched config is applied to all ports.
    #[prost(message, optional, tag = "10")]
    pub traffic_port_selector: ::core::option::Option<TrafficPortSelector>,
    /// Optional. A free-text description of the resource. Max length 1024 characters.
    #[prost(string, tag = "11")]
    pub description: ::prost::alloc::string::String,
    /// Optional. A URL referring to ServerTlsPolicy resource. ServerTlsPolicy is used to
    /// determine the authentication policy to be applied to terminate the inbound
    /// traffic at the identified backends. If this field is not set,
    /// authentication is disabled(open) for this endpoint.
    #[prost(string, tag = "12")]
    pub server_tls_policy: ::prost::alloc::string::String,
    /// Optional. A URL referring to a ClientTlsPolicy resource. ClientTlsPolicy can be set
    /// to specify the authentication for traffic from the proxy to the actual
    /// endpoints. More specifically, it is applied to the outgoing traffic from
    /// the proxy to the endpoint. This is typically used for sidecar model where
    /// the proxy identifies itself as endpoint to the control plane, with the
    /// connection between sidecar and endpoint requiring authentication. If this
    /// field is not set, authentication is disabled(open). Applicable only when
    /// EndpointPolicyType is SIDECAR_PROXY.
    #[prost(string, tag = "13")]
    pub client_tls_policy: ::prost::alloc::string::String,
}
/// Nested message and enum types in `EndpointPolicy`.
pub mod endpoint_policy {
    /// The type of endpoint policy.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EndpointPolicyType {
        /// Default value. Must not be used.
        Unspecified = 0,
        /// Represents a proxy deployed as a sidecar.
        SidecarProxy = 1,
        /// Represents a proxyless gRPC backend.
        GrpcServer = 2,
    }
}
/// Request used with the ListEndpointPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointPoliciesRequest {
    /// Required. The project and location from which the EndpointPolicies should be
    /// listed, specified in the format `projects/*/locations/global`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of EndpointPolicies to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListEndpointPoliciesResponse`
    /// Indicates that this is a continuation of a prior
    /// `ListEndpointPolicies` call, and that the system should return the
    /// next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response returned by the ListEndpointPolicies method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointPoliciesResponse {
    /// List of EndpointPolicy resources.
    #[prost(message, repeated, tag = "1")]
    pub endpoint_policies: ::prost::alloc::vec::Vec<EndpointPolicy>,
    /// If there might be more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request used with the GetEndpointPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointPolicyRequest {
    /// Required. A name of the EndpointPolicy to get. Must be in the format
    /// `projects/*/locations/global/endpointPolicies/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request used with the CreateEndpointPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEndpointPolicyRequest {
    /// Required. The parent resource of the EndpointPolicy. Must be in the
    /// format `projects/*/locations/global`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Short name of the EndpointPolicy resource to be created.
    /// E.g. "CustomECS".
    #[prost(string, tag = "2")]
    pub endpoint_policy_id: ::prost::alloc::string::String,
    /// Required. EndpointPolicy resource to be created.
    #[prost(message, optional, tag = "3")]
    pub endpoint_policy: ::core::option::Option<EndpointPolicy>,
}
/// Request used with the UpdateEndpointPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEndpointPolicyRequest {
    /// Optional. Field mask is used to specify the fields to be overwritten in the
    /// EndpointPolicy resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Updated EndpointPolicy resource.
    #[prost(message, optional, tag = "2")]
    pub endpoint_policy: ::core::option::Option<EndpointPolicy>,
}
/// Request used with the DeleteEndpointPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointPolicyRequest {
    /// Required. A name of the EndpointPolicy to delete. Must be in the format
    /// `projects/*/locations/global/endpointPolicies/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod network_services_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct NetworkServicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NetworkServicesClient<T>
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
        ) -> NetworkServicesClient<InterceptedService<T, F>>
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
            NetworkServicesClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists EndpointPolicies in a given project and location."]
        pub async fn list_endpoint_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEndpointPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListEndpointPoliciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkservices.v1beta1.NetworkServices/ListEndpointPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single EndpointPolicy."]
        pub async fn get_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointPolicyRequest>,
        ) -> Result<tonic::Response<super::EndpointPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.networkservices.v1beta1.NetworkServices/GetEndpointPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new EndpointPolicy in a given project and location."]
        pub async fn create_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEndpointPolicyRequest>,
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
                "/google.cloud.networkservices.v1beta1.NetworkServices/CreateEndpointPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single EndpointPolicy."]
        pub async fn update_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEndpointPolicyRequest>,
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
                "/google.cloud.networkservices.v1beta1.NetworkServices/UpdateEndpointPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single EndpointPolicy."]
        pub async fn delete_endpoint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEndpointPolicyRequest>,
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
                "/google.cloud.networkservices.v1beta1.NetworkServices/DeleteEndpointPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
