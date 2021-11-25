/// IAM Policy analysis result, consisting of one IAM policy binding and derived
/// access control lists.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisResult {
    /// The full name of the resource to which the \[iam_binding][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.iam_binding\] policy attaches.
    #[prost(string, tag = "1")]
    pub attached_resource_full_name: ::prost::alloc::string::String,
    /// The Cloud IAM policy binding under analysis.
    #[prost(message, optional, tag = "2")]
    pub iam_binding: ::core::option::Option<super::super::super::iam::v1::Binding>,
    /// The access control lists derived from the \[iam_binding][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.iam_binding\] that match or
    /// potentially match resource and access selectors specified in the request.
    #[prost(message, repeated, tag = "3")]
    pub access_control_lists:
        ::prost::alloc::vec::Vec<iam_policy_analysis_result::AccessControlList>,
    /// The identity list derived from members of the \[iam_binding][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.iam_binding\] that match or
    /// potentially match identity selector specified in the request.
    #[prost(message, optional, tag = "4")]
    pub identity_list: ::core::option::Option<iam_policy_analysis_result::IdentityList>,
    /// Represents whether all nodes in the transitive closure of the
    /// \[iam_binding][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.iam_binding\] node have been explored.
    #[prost(bool, tag = "5")]
    pub fully_explored: bool,
}
/// Nested message and enum types in `IamPolicyAnalysisResult`.
pub mod iam_policy_analysis_result {
    /// Represents analysis state of each node in the result graph or non-critical
    /// errors in the response.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnalysisState {
        /// The Google standard error code that best describes the state.
        /// For example:
        /// - OK means the node has been successfully explored;
        /// - PERMISSION_DENIED means an access denied error is encountered;
        /// - DEADLINE_EXCEEDED means the node hasn't been explored in time;
        #[prost(enumeration = "super::super::super::super::rpc::Code", tag = "1")]
        pub code: i32,
        /// The human-readable description of the cause of failure.
        #[prost(string, tag = "2")]
        pub cause: ::prost::alloc::string::String,
    }
    /// A GCP resource that appears in an access control list.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        /// The [full resource name](<https://aip.dev/122#full-resource-names>).
        #[prost(string, tag = "1")]
        pub full_resource_name: ::prost::alloc::string::String,
        /// The analysis state of this resource node.
        #[prost(message, optional, tag = "2")]
        pub analysis_state: ::core::option::Option<AnalysisState>,
    }
    /// A role or permission that appears in an access control list.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Access {
        /// The analysis state of this access node.
        #[prost(message, optional, tag = "3")]
        pub analysis_state: ::core::option::Option<AnalysisState>,
        #[prost(oneof = "access::OneofAccess", tags = "1, 2")]
        pub oneof_access: ::core::option::Option<access::OneofAccess>,
    }
    /// Nested message and enum types in `Access`.
    pub mod access {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OneofAccess {
            /// The role.
            #[prost(string, tag = "1")]
            Role(::prost::alloc::string::String),
            /// The permission.
            #[prost(string, tag = "2")]
            Permission(::prost::alloc::string::String),
        }
    }
    /// A directional edge.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Edge {
        /// The source node of the edge.
        #[prost(string, tag = "1")]
        pub source_node: ::prost::alloc::string::String,
        /// The target node of the edge.
        #[prost(string, tag = "2")]
        pub target_node: ::prost::alloc::string::String,
    }
    /// An identity that appears in an access control list.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identity {
        /// The identity name in any form of members appear in
        /// [IAM policy
        /// binding](<https://cloud.google.com/iam/reference/rest/v1/Binding>), such
        /// as:
        /// - user:foo@google.com
        /// - group:group1@google.com
        /// - serviceAccount:s1@prj1.iam.gserviceaccount.com
        /// - projectOwner:some_project_id
        /// - domain:google.com
        /// - allUsers
        /// - etc.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The analysis state of this identity node.
        #[prost(message, optional, tag = "2")]
        pub analysis_state: ::core::option::Option<AnalysisState>,
    }
    /// An access control list, derived from the above IAM policy binding, which
    /// contains a set of resources and accesses. May include one
    /// item from each set to compose an access control entry.
    ///
    /// NOTICE that there could be multiple access control lists for one IAM policy
    /// binding. The access control lists are created based on resource and access
    /// combinations.
    ///
    /// For example, assume we have the following cases in one IAM policy binding:
    /// - Permission P1 and P2 apply to resource R1 and R2;
    /// - Permission P3 applies to resource R2 and R3;
    ///
    /// This will result in the following access control lists:
    /// - AccessControlList 1: [R1, R2], [P1, P2]
    /// - AccessControlList 2: [R2, R3], \[P3\]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessControlList {
        /// The resources that match one of the following conditions:
        /// - The resource_selector, if it is specified in request;
        /// - Otherwise, resources reachable from the policy attached resource.
        #[prost(message, repeated, tag = "1")]
        pub resources: ::prost::alloc::vec::Vec<Resource>,
        /// The accesses that match one of the following conditions:
        /// - The access_selector, if it is specified in request;
        /// - Otherwise, access specifiers reachable from the policy binding's role.
        #[prost(message, repeated, tag = "2")]
        pub accesses: ::prost::alloc::vec::Vec<Access>,
        /// Resource edges of the graph starting from the policy attached
        /// resource to any descendant resources. The \[Edge.source_node][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.Edge.source_node\] contains
        /// the full resource name of a parent resource and \[Edge.target_node][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.Edge.target_node\]
        /// contains the full resource name of a child resource. This field is
        /// present only if the output_resource_edges option is enabled in request.
        #[prost(message, repeated, tag = "3")]
        pub resource_edges: ::prost::alloc::vec::Vec<Edge>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentityList {
        /// Only the identities that match one of the following conditions will be
        /// presented:
        /// - The identity_selector, if it is specified in request;
        /// - Otherwise, identities reachable from the policy binding's members.
        #[prost(message, repeated, tag = "1")]
        pub identities: ::prost::alloc::vec::Vec<Identity>,
        /// Group identity edges of the graph starting from the binding's
        /// group members to any node of the \[identities][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.IdentityList.identities\]. The \[Edge.source_node][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.Edge.source_node\]
        /// contains a group, such as "group:parent@google.com". The
        /// \[Edge.target_node][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.Edge.target_node\] contains a member of the group,
        /// such as "group:child@google.com" or "user:foo@google.com".
        /// This field is present only if the output_group_edges option is enabled in
        /// request.
        #[prost(message, repeated, tag = "2")]
        pub group_edges: ::prost::alloc::vec::Vec<Edge>,
    }
}
/// IAM policy analysis query message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisQuery {
    /// Required. The relative name of the root asset. Only resources and IAM policies within
    /// the parent will be analyzed. This can only be an organization number (such
    /// as "organizations/123") or a folder number (such as "folders/123").
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. Specifies a resource for analysis. Leaving it empty means ANY.
    #[prost(message, optional, tag = "2")]
    pub resource_selector: ::core::option::Option<iam_policy_analysis_query::ResourceSelector>,
    /// Optional. Specifies an identity for analysis. Leaving it empty means ANY.
    #[prost(message, optional, tag = "3")]
    pub identity_selector: ::core::option::Option<iam_policy_analysis_query::IdentitySelector>,
    /// Optional. Specifies roles or permissions for analysis. Leaving it empty
    /// means ANY.
    #[prost(message, optional, tag = "4")]
    pub access_selector: ::core::option::Option<iam_policy_analysis_query::AccessSelector>,
}
/// Nested message and enum types in `IamPolicyAnalysisQuery`.
pub mod iam_policy_analysis_query {
    /// Specifies the resource to analyze for access policies, which may be set
    /// directly on the resource, or on ancestors such as organizations, folders or
    /// projects. At least one of \[ResourceSelector][google.cloud.asset.v1p4beta1.IamPolicyAnalysisQuery.ResourceSelector\], \[IdentitySelector][google.cloud.asset.v1p4beta1.IamPolicyAnalysisQuery.IdentitySelector\] or
    /// \[AccessSelector][google.cloud.asset.v1p4beta1.IamPolicyAnalysisQuery.AccessSelector\] must be specified in a request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceSelector {
        /// Required. The [full resource
        /// name](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
        /// .
        #[prost(string, tag = "1")]
        pub full_resource_name: ::prost::alloc::string::String,
    }
    /// Specifies an identity for which to determine resource access, based on
    /// roles assigned either directly to them or to the groups they belong to,
    /// directly or indirectly.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentitySelector {
        /// Required. The identity appear in the form of members in
        /// [IAM policy
        /// binding](<https://cloud.google.com/iam/reference/rest/v1/Binding>).
        #[prost(string, tag = "1")]
        pub identity: ::prost::alloc::string::String,
    }
    /// Specifies roles and/or permissions to analyze, to determine both the
    /// identities possessing them and the resources they control. If multiple
    /// values are specified, results will include identities and resources
    /// matching any of them.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessSelector {
        /// Optional. The roles to appear in result.
        #[prost(string, repeated, tag = "1")]
        pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. The permissions to appear in result.
        #[prost(string, repeated, tag = "2")]
        pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// A request message for \[AssetService.AnalyzeIamPolicy][google.cloud.asset.v1p4beta1.AssetService.AnalyzeIamPolicy\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyRequest {
    /// Required. The request query.
    #[prost(message, optional, tag = "1")]
    pub analysis_query: ::core::option::Option<IamPolicyAnalysisQuery>,
    /// Optional. The request options.
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<analyze_iam_policy_request::Options>,
}
/// Nested message and enum types in `AnalyzeIamPolicyRequest`.
pub mod analyze_iam_policy_request {
    /// Contains request options.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        /// Optional. If true, the identities section of the result will expand any
        /// Google groups appearing in an IAM policy binding.
        ///
        /// If \[identity_selector][\] is specified, the identity in the result will
        /// be determined by the selector, and this flag will have no effect.
        ///
        /// Default is false.
        #[prost(bool, tag = "1")]
        pub expand_groups: bool,
        /// Optional. If true, the access section of result will expand any roles
        /// appearing in IAM policy bindings to include their permissions.
        ///
        /// If \[access_selector][\] is specified, the access section of the result
        /// will be determined by the selector, and this flag will have no effect.
        ///
        /// Default is false.
        #[prost(bool, tag = "2")]
        pub expand_roles: bool,
        /// Optional. If true, the resource section of the result will expand any
        /// resource attached to an IAM policy to include resources lower in the
        /// resource hierarchy.
        ///
        /// For example, if the request analyzes for which resources user A has
        /// permission P, and the results include an IAM policy with P on a GCP
        /// folder, the results will also include resources in that folder with
        /// permission P.
        ///
        /// If \[resource_selector][\] is specified, the resource section of the result
        /// will be determined by the selector, and this flag will have no effect.
        /// Default is false.
        #[prost(bool, tag = "3")]
        pub expand_resources: bool,
        /// Optional. If true, the result will output resource edges, starting
        /// from the policy attached resource, to any expanded resources.
        /// Default is false.
        #[prost(bool, tag = "4")]
        pub output_resource_edges: bool,
        /// Optional. If true, the result will output group identity edges, starting
        /// from the binding's group members, to any expanded identities.
        /// Default is false.
        #[prost(bool, tag = "5")]
        pub output_group_edges: bool,
        /// Optional. If true, the response will include access analysis from identities to
        /// resources via service account impersonation. This is a very expensive
        /// operation, because many derived queries will be executed. We highly
        /// recommend you use ExportIamPolicyAnalysis rpc instead.
        ///
        /// For example, if the request analyzes for which resources user A has
        /// permission P, and there's an IAM policy states user A has
        /// iam.serviceAccounts.getAccessToken permission to a service account SA,
        /// and there's another IAM policy states service account SA has permission P
        /// to a GCP folder F, then user A potentially has access to the GCP folder
        /// F. And those advanced analysis results will be included in
        /// \[AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis\].
        ///
        /// Another example, if the request analyzes for who has
        /// permission P to a GCP folder F, and there's an IAM policy states user A
        /// has iam.serviceAccounts.actAs permission to a service account SA, and
        /// there's another IAM policy states service account SA has permission P to
        /// the GCP folder F, then user A potentially has access to the GCP folder
        /// F. And those advanced analysis results will be included in
        /// \[AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis\].
        ///
        /// Default is false.
        #[prost(bool, tag = "6")]
        pub analyze_service_account_impersonation: bool,
        /// Optional. Amount of time executable has to complete.  See JSON representation of
        /// \[Duration\](<https://developers.google.com/protocol-buffers/docs/proto3#json>).
        ///
        /// If this field is set with a value less than the RPC deadline, and the
        /// execution of your query hasn't finished in the specified
        /// execution timeout,  you will get a response with partial result.
        /// Otherwise, your query's execution will continue until the RPC deadline.
        /// If it's not finished until then, you will get a  DEADLINE_EXCEEDED error.
        ///
        /// Default is empty.
        #[prost(message, optional, tag = "7")]
        pub execution_timeout: ::core::option::Option<::prost_types::Duration>,
    }
}
/// A response message for \[AssetService.AnalyzeIamPolicy][google.cloud.asset.v1p4beta1.AssetService.AnalyzeIamPolicy\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyResponse {
    /// The main analysis that matches the original request.
    #[prost(message, optional, tag = "1")]
    pub main_analysis: ::core::option::Option<analyze_iam_policy_response::IamPolicyAnalysis>,
    /// The service account impersonation analysis if
    /// \[AnalyzeIamPolicyRequest.analyze_service_account_impersonation][\] is
    /// enabled.
    #[prost(message, repeated, tag = "2")]
    pub service_account_impersonation_analysis:
        ::prost::alloc::vec::Vec<analyze_iam_policy_response::IamPolicyAnalysis>,
    /// Represents whether all entries in the \[main_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.main_analysis\] and
    /// \[service_account_impersonation_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis\] have been fully explored to
    /// answer the query in the request.
    #[prost(bool, tag = "3")]
    pub fully_explored: bool,
    /// A list of non-critical errors happened during the request handling to
    /// explain why `fully_explored` is false, or empty if no error happened.
    #[prost(message, repeated, tag = "4")]
    pub non_critical_errors: ::prost::alloc::vec::Vec<iam_policy_analysis_result::AnalysisState>,
}
/// Nested message and enum types in `AnalyzeIamPolicyResponse`.
pub mod analyze_iam_policy_response {
    /// An analysis message to group the query and results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IamPolicyAnalysis {
        /// The analysis query.
        #[prost(message, optional, tag = "1")]
        pub analysis_query: ::core::option::Option<super::IamPolicyAnalysisQuery>,
        /// A list of \[IamPolicyAnalysisResult][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult\] that matches the analysis query, or
        /// empty if no result is found.
        #[prost(message, repeated, tag = "2")]
        pub analysis_results: ::prost::alloc::vec::Vec<super::IamPolicyAnalysisResult>,
        /// Represents whether all entries in the \[analysis_results][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.IamPolicyAnalysis.analysis_results\] have been
        /// fully explored to answer the query.
        #[prost(bool, tag = "3")]
        pub fully_explored: bool,
    }
}
/// Output configuration for export IAM policy analysis destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisOutputConfig {
    /// IAM policy analysis export destination.
    #[prost(oneof = "iam_policy_analysis_output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<iam_policy_analysis_output_config::Destination>,
}
/// Nested message and enum types in `IamPolicyAnalysisOutputConfig`.
pub mod iam_policy_analysis_output_config {
    /// A Cloud Storage location.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsDestination {
        /// Required. The uri of the Cloud Storage object. It's the same uri that is used by
        /// gsutil. For example: "gs://bucket_name/object_name". See [Viewing and
        /// Editing Object
        /// Metadata](<https://cloud.google.com/storage/docs/viewing-editing-metadata>)
        /// for more information.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
    /// IAM policy analysis export destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsDestination(GcsDestination),
    }
}
/// A request message for \[AssetService.ExportIamPolicyAnalysis][google.cloud.asset.v1p4beta1.AssetService.ExportIamPolicyAnalysis\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportIamPolicyAnalysisRequest {
    /// Required. The request query.
    #[prost(message, optional, tag = "1")]
    pub analysis_query: ::core::option::Option<IamPolicyAnalysisQuery>,
    /// Optional. The request options.
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<export_iam_policy_analysis_request::Options>,
    /// Required. Output configuration indicating where the results will be output to.
    #[prost(message, optional, tag = "3")]
    pub output_config: ::core::option::Option<IamPolicyAnalysisOutputConfig>,
}
/// Nested message and enum types in `ExportIamPolicyAnalysisRequest`.
pub mod export_iam_policy_analysis_request {
    /// Contains request options.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        /// Optional. If true, the identities section of the result will expand any
        /// Google groups appearing in an IAM policy binding.
        ///
        /// If \[identity_selector][\] is specified, the identity in the result will
        /// be determined by the selector, and this flag will have no effect.
        ///
        /// Default is false.
        #[prost(bool, tag = "1")]
        pub expand_groups: bool,
        /// Optional. If true, the access section of result will expand any roles
        /// appearing in IAM policy bindings to include their permissions.
        ///
        /// If \[access_selector][\] is specified, the access section of the result
        /// will be determined by the selector, and this flag will have no effect.
        ///
        /// Default is false.
        #[prost(bool, tag = "2")]
        pub expand_roles: bool,
        /// Optional. If true, the resource section of the result will expand any
        /// resource attached to an IAM policy to include resources lower in the
        /// resource hierarchy.
        ///
        /// For example, if the request analyzes for which resources user A has
        /// permission P, and the results include an IAM policy with P on a GCP
        /// folder, the results will also include resources in that folder with
        /// permission P.
        ///
        /// If \[resource_selector][\] is specified, the resource section of the result
        /// will be determined by the selector, and this flag will have no effect.
        /// Default is false.
        #[prost(bool, tag = "3")]
        pub expand_resources: bool,
        /// Optional. If true, the result will output resource edges, starting
        /// from the policy attached resource, to any expanded resources.
        /// Default is false.
        #[prost(bool, tag = "4")]
        pub output_resource_edges: bool,
        /// Optional. If true, the result will output group identity edges, starting
        /// from the binding's group members, to any expanded identities.
        /// Default is false.
        #[prost(bool, tag = "5")]
        pub output_group_edges: bool,
        /// Optional. If true, the response will include access analysis from identities to
        /// resources via service account impersonation. This is a very expensive
        /// operation, because many derived queries will be executed.
        ///
        /// For example, if the request analyzes for which resources user A has
        /// permission P, and there's an IAM policy states user A has
        /// iam.serviceAccounts.getAccessToken permission to a service account SA,
        /// and there's another IAM policy states service account SA has permission P
        /// to a GCP folder F, then user A potentially has access to the GCP folder
        /// F. And those advanced analysis results will be included in
        /// \[AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis\].
        ///
        /// Another example, if the request analyzes for who has
        /// permission P to a GCP folder F, and there's an IAM policy states user A
        /// has iam.serviceAccounts.actAs permission to a service account SA, and
        /// there's another IAM policy states service account SA has permission P to
        /// the GCP folder F, then user A potentially has access to the GCP folder
        /// F. And those advanced analysis results will be included in
        /// \[AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis\].
        ///
        /// Default is false.
        #[prost(bool, tag = "6")]
        pub analyze_service_account_impersonation: bool,
    }
}
/// The export IAM policy analysis response. This message is returned by the
/// \[google.longrunning.Operations.GetOperation][\] method in the returned
/// \[google.longrunning.Operation.response][\] field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportIamPolicyAnalysisResponse {
    /// Output configuration indicating where the results were output to.
    #[prost(message, optional, tag = "1")]
    pub output_config: ::core::option::Option<IamPolicyAnalysisOutputConfig>,
}
#[doc = r" Generated client implementations."]
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Asset service definition."]
    #[derive(Debug, Clone)]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetServiceClient<T>
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
        ) -> AssetServiceClient<InterceptedService<T, F>>
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
            AssetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Analyzes IAM policies based on the specified request. Returns"]
        #[doc = " a list of [IamPolicyAnalysisResult][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult] matching the request."]
        pub async fn analyze_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeIamPolicyRequest>,
        ) -> Result<tonic::Response<super::AnalyzeIamPolicyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1p4beta1.AssetService/AnalyzeIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports IAM policy analysis based on the specified request. This API"]
        #[doc = " implements the [google.longrunning.Operation][google.longrunning.Operation] API allowing you to keep"]
        #[doc = " track of the export. The metadata contains the request to help callers to"]
        #[doc = " map responses to requests."]
        pub async fn export_iam_policy_analysis(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportIamPolicyAnalysisRequest>,
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
                "/google.cloud.asset.v1p4beta1.AssetService/ExportIamPolicyAnalysis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
