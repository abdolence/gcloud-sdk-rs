/// IAM Policy analysis result, consisting of one IAM policy binding and derived
/// access control lists.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisResult {
    /// The full name of the resource to which the [iam_binding][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.iam_binding] policy attaches.
    #[prost(string, tag = "1")]
    pub attached_resource_full_name: std::string::String,
    /// The Cloud IAM policy binding under analysis.
    #[prost(message, optional, tag = "2")]
    pub iam_binding: ::std::option::Option<super::super::super::iam::v1::Binding>,
    /// The access control lists derived from the [iam_binding][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.iam_binding] that match or
    /// potentially match resource and access selectors specified in the request.
    #[prost(message, repeated, tag = "3")]
    pub access_control_lists: ::std::vec::Vec<iam_policy_analysis_result::AccessControlList>,
    /// The identity list derived from members of the [iam_binding][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.iam_binding] that match or
    /// potentially match identity selector specified in the request.
    #[prost(message, optional, tag = "4")]
    pub identity_list: ::std::option::Option<iam_policy_analysis_result::IdentityList>,
    /// Represents whether all nodes in the transitive closure of the
    /// [iam_binding][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.iam_binding] node have been explored.
    #[prost(bool, tag = "5")]
    pub fully_explored: bool,
}
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
        pub cause: std::string::String,
    }
    /// A GCP resource that appears in an access control list.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        /// The [full resource name](https://aip.dev/122#full-resource-names).
        #[prost(string, tag = "1")]
        pub full_resource_name: std::string::String,
        /// The analysis state of this resource node.
        #[prost(message, optional, tag = "2")]
        pub analysis_state: ::std::option::Option<AnalysisState>,
    }
    /// A role or permission that appears in an access control list.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Access {
        /// The analysis state of this access node.
        #[prost(message, optional, tag = "3")]
        pub analysis_state: ::std::option::Option<AnalysisState>,
        #[prost(oneof = "access::OneofAccess", tags = "1, 2")]
        pub oneof_access: ::std::option::Option<access::OneofAccess>,
    }
    pub mod access {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OneofAccess {
            /// The role.
            #[prost(string, tag = "1")]
            Role(std::string::String),
            /// The permission.
            #[prost(string, tag = "2")]
            Permission(std::string::String),
        }
    }
    /// A directional edge.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Edge {
        /// The source node of the edge.
        #[prost(string, tag = "1")]
        pub source_node: std::string::String,
        /// The target node of the edge.
        #[prost(string, tag = "2")]
        pub target_node: std::string::String,
    }
    /// An identity that appears in an access control list.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identity {
        /// The identity name in any form of members appear in
        /// [IAM policy
        /// binding](https://cloud.google.com/iam/reference/rest/v1/Binding), such
        /// as:
        /// - user:foo@google.com
        /// - group:group1@google.com
        /// - serviceAccount:s1@prj1.iam.gserviceaccount.com
        /// - projectOwner:some_project_id
        /// - domain:google.com
        /// - allUsers
        /// - etc.
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        /// The analysis state of this identity node.
        #[prost(message, optional, tag = "2")]
        pub analysis_state: ::std::option::Option<AnalysisState>,
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
    /// - AccessControlList 2: [R2, R3], [P3]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessControlList {
        /// The resources that match one of the following conditions:
        /// - The resource_selector, if it is specified in request;
        /// - Otherwise, resources reachable from the policy attached resource.
        #[prost(message, repeated, tag = "1")]
        pub resources: ::std::vec::Vec<Resource>,
        /// The accesses that match one of the following conditions:
        /// - The access_selector, if it is specified in request;
        /// - Otherwise, access specifiers reachable from the policy binding's role.
        #[prost(message, repeated, tag = "2")]
        pub accesses: ::std::vec::Vec<Access>,
        /// Resource edges of the graph starting from the policy attached
        /// resource to any descendant resources. The [Edge.source_node][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.Edge.source_node] contains
        /// the full resource name of a parent resource and [Edge.target_node][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.Edge.target_node]
        /// contains the full resource name of a child resource. This field is
        /// present only if the output_resource_edges option is enabled in request.
        #[prost(message, repeated, tag = "3")]
        pub resource_edges: ::std::vec::Vec<Edge>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentityList {
        /// Only the identities that match one of the following conditions will be
        /// presented:
        /// - The identity_selector, if it is specified in request;
        /// - Otherwise, identities reachable from the policy binding's members.
        #[prost(message, repeated, tag = "1")]
        pub identities: ::std::vec::Vec<Identity>,
        /// Group identity edges of the graph starting from the binding's
        /// group members to any node of the [identities][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.IdentityList.identities]. The [Edge.source_node][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.Edge.source_node]
        /// contains a group, such as "group:parent@google.com". The
        /// [Edge.target_node][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult.Edge.target_node] contains a member of the group,
        /// such as "group:child@google.com" or "user:foo@google.com".
        /// This field is present only if the output_group_edges option is enabled in
        /// request.
        #[prost(message, repeated, tag = "2")]
        pub group_edges: ::std::vec::Vec<Edge>,
    }
}
/// IAM policy analysis query message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisQuery {
    /// Required. The relative name of the root asset. Only resources and IAM policies within
    /// the parent will be analyzed. This can only be an organization number (such
    /// as "organizations/123") or a folder number (such as "folders/123").
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Specifies a resource for analysis. Leaving it empty means ANY.
    #[prost(message, optional, tag = "2")]
    pub resource_selector: ::std::option::Option<iam_policy_analysis_query::ResourceSelector>,
    /// Optional. Specifies an identity for analysis. Leaving it empty means ANY.
    #[prost(message, optional, tag = "3")]
    pub identity_selector: ::std::option::Option<iam_policy_analysis_query::IdentitySelector>,
    /// Optional. Specifies roles or permissions for analysis. Leaving it empty
    /// means ANY.
    #[prost(message, optional, tag = "4")]
    pub access_selector: ::std::option::Option<iam_policy_analysis_query::AccessSelector>,
}
pub mod iam_policy_analysis_query {
    /// Specifies the resource to analyze for access policies, which may be set
    /// directly on the resource, or on ancestors such as organizations, folders or
    /// projects. At least one of [ResourceSelector][google.cloud.asset.v1p4beta1.IamPolicyAnalysisQuery.ResourceSelector], [IdentitySelector][google.cloud.asset.v1p4beta1.IamPolicyAnalysisQuery.IdentitySelector] or
    /// [AccessSelector][google.cloud.asset.v1p4beta1.IamPolicyAnalysisQuery.AccessSelector] must be specified in a request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceSelector {
        /// Required. The [full resource
        /// name](https://cloud.google.com/apis/design/resource_names#full_resource_name)
        /// .
        #[prost(string, tag = "1")]
        pub full_resource_name: std::string::String,
    }
    /// Specifies an identity for which to determine resource access, based on
    /// roles assigned either directly to them or to the groups they belong to,
    /// directly or indirectly.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentitySelector {
        /// Required. The identity appear in the form of members in
        /// [IAM policy
        /// binding](https://cloud.google.com/iam/reference/rest/v1/Binding).
        #[prost(string, tag = "1")]
        pub identity: std::string::String,
    }
    /// Specifies roles and/or permissions to analyze, to determine both the
    /// identities possessing them and the resources they control. If multiple
    /// values are specified, results will include identities and resources
    /// matching any of them.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessSelector {
        /// Optional. The roles to appear in result.
        #[prost(string, repeated, tag = "1")]
        pub roles: ::std::vec::Vec<std::string::String>,
        /// Optional. The permissions to appear in result.
        #[prost(string, repeated, tag = "2")]
        pub permissions: ::std::vec::Vec<std::string::String>,
    }
}
/// A request message for [AssetService.AnalyzeIamPolicy][google.cloud.asset.v1p4beta1.AssetService.AnalyzeIamPolicy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyRequest {
    /// Required. The request query.
    #[prost(message, optional, tag = "1")]
    pub analysis_query: ::std::option::Option<IamPolicyAnalysisQuery>,
    /// Optional. The request options.
    #[prost(message, optional, tag = "2")]
    pub options: ::std::option::Option<analyze_iam_policy_request::Options>,
}
pub mod analyze_iam_policy_request {
    /// Contains request options.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        /// Optional. If true, the identities section of the result will expand any
        /// Google groups appearing in an IAM policy binding.
        ///
        /// If [identity_selector][] is specified, the identity in the result will
        /// be determined by the selector, and this flag will have no effect.
        ///
        /// Default is false.
        #[prost(bool, tag = "1")]
        pub expand_groups: bool,
        /// Optional. If true, the access section of result will expand any roles
        /// appearing in IAM policy bindings to include their permissions.
        ///
        /// If [access_selector][] is specified, the access section of the result
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
        /// If [resource_selector][] is specified, the resource section of the result
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
        /// [AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis].
        ///
        /// Another example, if the request analyzes for who has
        /// permission P to a GCP folder F, and there's an IAM policy states user A
        /// has iam.serviceAccounts.actAs permission to a service account SA, and
        /// there's another IAM policy states service account SA has permission P to
        /// the GCP folder F, then user A potentially has access to the GCP folder
        /// F. And those advanced analysis results will be included in
        /// [AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis].
        ///
        /// Default is false.
        #[prost(bool, tag = "6")]
        pub analyze_service_account_impersonation: bool,
        /// Optional. Amount of time executable has to complete.  See JSON representation of
        /// [Duration](https://developers.google.com/protocol-buffers/docs/proto3#json).
        ///
        /// If this field is set with a value less than the RPC deadline, and the
        /// execution of your query hasn't finished in the specified
        /// execution timeout,  you will get a response with partial result.
        /// Otherwise, your query's execution will continue until the RPC deadline.
        /// If it's not finished until then, you will get a  DEADLINE_EXCEEDED error.
        ///
        /// Default is empty.
        #[prost(message, optional, tag = "7")]
        pub execution_timeout: ::std::option::Option<::prost_types::Duration>,
    }
}
/// A response message for [AssetService.AnalyzeIamPolicy][google.cloud.asset.v1p4beta1.AssetService.AnalyzeIamPolicy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyResponse {
    /// The main analysis that matches the original request.
    #[prost(message, optional, tag = "1")]
    pub main_analysis: ::std::option::Option<analyze_iam_policy_response::IamPolicyAnalysis>,
    /// The service account impersonation analysis if
    /// [AnalyzeIamPolicyRequest.analyze_service_account_impersonation][] is
    /// enabled.
    #[prost(message, repeated, tag = "2")]
    pub service_account_impersonation_analysis:
        ::std::vec::Vec<analyze_iam_policy_response::IamPolicyAnalysis>,
    /// Represents whether all entries in the [main_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.main_analysis] and
    /// [service_account_impersonation_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis] have been fully explored to
    /// answer the query in the request.
    #[prost(bool, tag = "3")]
    pub fully_explored: bool,
    /// A list of non-critical errors happened during the request handling to
    /// explain why `fully_explored` is false, or empty if no error happened.
    #[prost(message, repeated, tag = "4")]
    pub non_critical_errors: ::std::vec::Vec<iam_policy_analysis_result::AnalysisState>,
}
pub mod analyze_iam_policy_response {
    /// An analysis message to group the query and results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IamPolicyAnalysis {
        /// The analysis query.
        #[prost(message, optional, tag = "1")]
        pub analysis_query: ::std::option::Option<super::IamPolicyAnalysisQuery>,
        /// A list of [IamPolicyAnalysisResult][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult] that matches the analysis query, or
        /// empty if no result is found.
        #[prost(message, repeated, tag = "2")]
        pub analysis_results: ::std::vec::Vec<super::IamPolicyAnalysisResult>,
        /// Represents whether all entries in the [analysis_results][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.IamPolicyAnalysis.analysis_results] have been
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
    pub destination: ::std::option::Option<iam_policy_analysis_output_config::Destination>,
}
pub mod iam_policy_analysis_output_config {
    /// A Cloud Storage location.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsDestination {
        /// Required. The uri of the Cloud Storage object. It's the same uri that is used by
        /// gsutil. For example: "gs://bucket_name/object_name". See [Viewing and
        /// Editing Object
        /// Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata)
        /// for more information.
        #[prost(string, tag = "1")]
        pub uri: std::string::String,
    }
    /// IAM policy analysis export destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsDestination(GcsDestination),
    }
}
/// A request message for [AssetService.ExportIamPolicyAnalysis][google.cloud.asset.v1p4beta1.AssetService.ExportIamPolicyAnalysis].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportIamPolicyAnalysisRequest {
    /// Required. The request query.
    #[prost(message, optional, tag = "1")]
    pub analysis_query: ::std::option::Option<IamPolicyAnalysisQuery>,
    /// Optional. The request options.
    #[prost(message, optional, tag = "2")]
    pub options: ::std::option::Option<export_iam_policy_analysis_request::Options>,
    /// Required. Output configuration indicating where the results will be output to.
    #[prost(message, optional, tag = "3")]
    pub output_config: ::std::option::Option<IamPolicyAnalysisOutputConfig>,
}
pub mod export_iam_policy_analysis_request {
    /// Contains request options.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        /// Optional. If true, the identities section of the result will expand any
        /// Google groups appearing in an IAM policy binding.
        ///
        /// If [identity_selector][] is specified, the identity in the result will
        /// be determined by the selector, and this flag will have no effect.
        ///
        /// Default is false.
        #[prost(bool, tag = "1")]
        pub expand_groups: bool,
        /// Optional. If true, the access section of result will expand any roles
        /// appearing in IAM policy bindings to include their permissions.
        ///
        /// If [access_selector][] is specified, the access section of the result
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
        /// If [resource_selector][] is specified, the resource section of the result
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
        /// [AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis].
        ///
        /// Another example, if the request analyzes for who has
        /// permission P to a GCP folder F, and there's an IAM policy states user A
        /// has iam.serviceAccounts.actAs permission to a service account SA, and
        /// there's another IAM policy states service account SA has permission P to
        /// the GCP folder F, then user A potentially has access to the GCP folder
        /// F. And those advanced analysis results will be included in
        /// [AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1p4beta1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis].
        ///
        /// Default is false.
        #[prost(bool, tag = "6")]
        pub analyze_service_account_impersonation: bool,
    }
}
/// The export IAM policy analysis response. This message is returned by the
/// [google.longrunning.Operations.GetOperation][] method in the returned
/// [google.longrunning.Operation.response][] field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportIamPolicyAnalysisResponse {
    /// Output configuration indicating where the results were output to.
    #[prost(message, optional, tag = "1")]
    pub output_config: ::std::option::Option<IamPolicyAnalysisOutputConfig>,
}
#[doc = r" Generated client implementations."]
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Asset service definition."]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AssetServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AssetServiceClient<T>
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
    impl<T: Clone> Clone for AssetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AssetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AssetServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod asset_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AssetServiceServer."]
    #[async_trait]
    pub trait AssetService: Send + Sync + 'static {
        #[doc = " Analyzes IAM policies based on the specified request. Returns"]
        #[doc = " a list of [IamPolicyAnalysisResult][google.cloud.asset.v1p4beta1.IamPolicyAnalysisResult] matching the request."]
        async fn analyze_iam_policy(
            &self,
            request: tonic::Request<super::AnalyzeIamPolicyRequest>,
        ) -> Result<tonic::Response<super::AnalyzeIamPolicyResponse>, tonic::Status>;
        #[doc = " Exports IAM policy analysis based on the specified request. This API"]
        #[doc = " implements the [google.longrunning.Operation][google.longrunning.Operation] API allowing you to keep"]
        #[doc = " track of the export. The metadata contains the request to help callers to"]
        #[doc = " map responses to requests."]
        async fn export_iam_policy_analysis(
            &self,
            request: tonic::Request<super::ExportIamPolicyAnalysisRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Asset service definition."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct AssetServiceServer<T: AssetService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: AssetService> AssetServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for AssetServiceServer<T>
    where
        T: AssetService,
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
                "/google.cloud.asset.v1p4beta1.AssetService/AnalyzeIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct AnalyzeIamPolicySvc<T: AssetService>(pub Arc<T>);
                    impl<T: AssetService>
                        tonic::server::UnaryService<super::AnalyzeIamPolicyRequest>
                        for AnalyzeIamPolicySvc<T>
                    {
                        type Response = super::AnalyzeIamPolicyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AnalyzeIamPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.analyze_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AnalyzeIamPolicySvc(inner);
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
                "/google.cloud.asset.v1p4beta1.AssetService/ExportIamPolicyAnalysis" => {
                    #[allow(non_camel_case_types)]
                    struct ExportIamPolicyAnalysisSvc<T: AssetService>(pub Arc<T>);
                    impl<T: AssetService>
                        tonic::server::UnaryService<super::ExportIamPolicyAnalysisRequest>
                        for ExportIamPolicyAnalysisSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportIamPolicyAnalysisRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.export_iam_policy_analysis(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ExportIamPolicyAnalysisSvc(inner);
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
    impl<T: AssetService> Clone for AssetServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: AssetService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AssetService> tonic::transport::NamedService for AssetServiceServer<T> {
        const NAME: &'static str = "google.cloud.asset.v1p4beta1.AssetService";
    }
}
