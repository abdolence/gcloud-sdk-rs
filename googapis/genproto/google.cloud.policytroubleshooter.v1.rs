/// Information about the member, resource, and permission to check.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTuple {
    /// Required. The member, or principal, whose access you want to check, in the form of
    /// the email address that represents that member. For example,
    /// `alice@example.com` or
    /// `my-service-account@my-project.iam.gserviceaccount.com`.
    ///
    /// The member must be a Google Account or a service account. Other types of
    /// members are not supported.
    #[prost(string, tag = "1")]
    pub principal: ::prost::alloc::string::String,
    /// Required. The full resource name that identifies the resource. For example,
    /// `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`.
    ///
    /// For examples of full resource names for Google Cloud services, see
    /// <https://cloud.google.com/iam/help/troubleshooter/full-resource-names.>
    #[prost(string, tag = "2")]
    pub full_resource_name: ::prost::alloc::string::String,
    /// Required. The IAM permission to check for the specified member and resource.
    ///
    /// For a complete list of IAM permissions, see
    /// <https://cloud.google.com/iam/help/permissions/reference.>
    ///
    /// For a complete list of predefined IAM roles and the permissions in each
    /// role, see <https://cloud.google.com/iam/help/roles/reference.>
    #[prost(string, tag = "3")]
    pub permission: ::prost::alloc::string::String,
}
/// Details about how a specific IAM \[Policy][google.iam.v1.Policy\] contributed
/// to the access check.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainedPolicy {
    /// Indicates whether _this policy_ provides the specified permission to the
    /// specified member for the specified resource.
    ///
    /// This field does _not_ indicate whether the member actually has the
    /// permission for the resource. There might be another policy that overrides
    /// this policy. To determine whether the member actually has the permission,
    /// use the `access` field in the
    /// \[TroubleshootIamPolicyResponse][IamChecker.TroubleshootIamPolicyResponse\].
    #[prost(enumeration = "AccessState", tag = "1")]
    pub access: i32,
    /// The full resource name that identifies the resource. For example,
    /// `//compute.googleapis.com/projects/my-project/zones/us-central1-a/instances/my-instance`.
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    ///
    /// For examples of full resource names for Google Cloud services, see
    /// <https://cloud.google.com/iam/help/troubleshooter/full-resource-names.>
    #[prost(string, tag = "2")]
    pub full_resource_name: ::prost::alloc::string::String,
    /// The IAM policy attached to the resource.
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is empty.
    #[prost(message, optional, tag = "3")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// Details about how each binding in the policy affects the member's ability,
    /// or inability, to use the permission for the resource.
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    #[prost(message, repeated, tag = "4")]
    pub binding_explanations: ::prost::alloc::vec::Vec<BindingExplanation>,
    /// The relevance of this policy to the overall determination in the
    /// \[TroubleshootIamPolicyResponse][IamChecker.TroubleshootIamPolicyResponse\].
    ///
    /// If the sender of the request does not have access to the policy, this field
    /// is omitted.
    #[prost(enumeration = "HeuristicRelevance", tag = "5")]
    pub relevance: i32,
}
/// Details about how a binding in a policy affects a member's ability to use a
/// permission.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindingExplanation {
    /// Required. Indicates whether _this binding_ provides the specified permission to the
    /// specified member for the specified resource.
    ///
    /// This field does _not_ indicate whether the member actually has the
    /// permission for the resource. There might be another binding that overrides
    /// this binding. To determine whether the member actually has the permission,
    /// use the `access` field in the
    /// \[TroubleshootIamPolicyResponse][IamChecker.TroubleshootIamPolicyResponse\].
    #[prost(enumeration = "AccessState", tag = "1")]
    pub access: i32,
    /// The role that this binding grants. For example,
    /// `roles/compute.serviceAgent`.
    ///
    /// For a complete list of predefined IAM roles, as well as the permissions in
    /// each role, see <https://cloud.google.com/iam/help/roles/reference.>
    #[prost(string, tag = "2")]
    pub role: ::prost::alloc::string::String,
    /// Indicates whether the role granted by this binding contains the specified
    /// permission.
    #[prost(enumeration = "binding_explanation::RolePermission", tag = "3")]
    pub role_permission: i32,
    /// The relevance of the permission's existence, or nonexistence, in the role
    /// to the overall determination for the entire policy.
    #[prost(enumeration = "HeuristicRelevance", tag = "4")]
    pub role_permission_relevance: i32,
    /// Indicates whether each member in the binding includes the member specified
    /// in the request, either directly or indirectly. Each key identifies a member
    /// in the binding, and each value indicates whether the member in the binding
    /// includes the member in the request.
    ///
    /// For example, suppose that a binding includes the following members:
    ///
    /// * `user:alice@example.com`
    /// * `group:product-eng@example.com`
    ///
    /// You want to troubleshoot access for `user:bob@example.com`. This user is a
    /// member of the group `group:product-eng@example.com`.
    ///
    /// For the first member in the binding, the key is `user:alice@example.com`,
    /// and the `membership` field in the value is set to
    /// `MEMBERSHIP_NOT_INCLUDED`.
    ///
    /// For the second member in the binding, the key is
    /// `group:product-eng@example.com`, and the `membership` field in the value is
    /// set to `MEMBERSHIP_INCLUDED`.
    #[prost(map = "string, message", tag = "5")]
    pub memberships: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        binding_explanation::AnnotatedMembership,
    >,
    /// The relevance of this binding to the overall determination for the entire
    /// policy.
    #[prost(enumeration = "HeuristicRelevance", tag = "6")]
    pub relevance: i32,
    /// A condition expression that prevents access unless the expression evaluates
    /// to `true`.
    ///
    /// To learn about IAM Conditions, see
    /// <http://cloud.google.com/iam/help/conditions/overview.>
    #[prost(message, optional, tag = "7")]
    pub condition: ::core::option::Option<super::super::super::r#type::Expr>,
}
/// Nested message and enum types in `BindingExplanation`.
pub mod binding_explanation {
    /// Details about whether the binding includes the member.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnnotatedMembership {
        /// Indicates whether the binding includes the member.
        #[prost(enumeration = "Membership", tag = "1")]
        pub membership: i32,
        /// The relevance of the member's status to the overall determination for the
        /// binding.
        #[prost(enumeration = "super::HeuristicRelevance", tag = "2")]
        pub relevance: i32,
    }
    /// Whether a role includes a specific permission.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RolePermission {
        /// Reserved for future use.
        Unspecified = 0,
        /// The permission is included in the role.
        Included = 1,
        /// The permission is not included in the role.
        NotIncluded = 2,
        /// The sender of the request is not allowed to access the binding.
        UnknownInfoDenied = 3,
    }
    /// Whether the binding includes the member.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Membership {
        /// Reserved for future use.
        Unspecified = 0,
        /// The binding includes the member. The member can be included directly
        /// or indirectly. For example:
        ///
        /// * A member is included directly if that member is listed in the binding.
        /// * A member is included indirectly if that member is in a Google group or
        ///   G Suite domain that is listed in the binding.
        Included = 1,
        /// The binding does not include the member.
        NotIncluded = 2,
        /// The sender of the request is not allowed to access the binding.
        UnknownInfoDenied = 3,
        /// The member is an unsupported type. Only Google Accounts and service
        /// accounts are supported.
        UnknownUnsupported = 4,
    }
}
/// Whether a member has a permission for a resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessState {
    /// Reserved for future use.
    Unspecified = 0,
    /// The member has the permission.
    Granted = 1,
    /// The member does not have the permission.
    NotGranted = 2,
    /// The member has the permission only if a condition expression evaluates to
    /// `true`.
    UnknownConditional = 3,
    /// The sender of the request does not have access to all of the policies that
    /// Policy Troubleshooter needs to evaluate.
    UnknownInfoDenied = 4,
}
/// The extent to which a single data point contributes to an overall
/// determination.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HeuristicRelevance {
    /// Reserved for future use.
    Unspecified = 0,
    /// The data point has a limited effect on the result. Changing the data point
    /// is unlikely to affect the overall determination.
    Normal = 1,
    /// The data point has a strong effect on the result. Changing the data point
    /// is likely to affect the overall determination.
    High = 2,
}
/// Request for \[TroubleshootIamPolicy][google.cloud.policytroubleshooter.v1.IamChecker.TroubleshootIamPolicy\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TroubleshootIamPolicyRequest {
    /// The information to use for checking whether a member has a permission for a
    /// resource.
    #[prost(message, optional, tag = "1")]
    pub access_tuple: ::core::option::Option<AccessTuple>,
}
/// Response for \[TroubleshootIamPolicy][google.cloud.policytroubleshooter.v1.IamChecker.TroubleshootIamPolicy\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TroubleshootIamPolicyResponse {
    /// Indicates whether the member has the specified permission for the specified
    /// resource, based on evaluating all of the applicable IAM policies.
    #[prost(enumeration = "AccessState", tag = "1")]
    pub access: i32,
    /// List of IAM policies that were evaluated to check the member's permissions,
    /// with annotations to indicate how each policy contributed to the final
    /// result.
    ///
    /// The list of policies can include the policy for the resource itself. It can
    /// also include policies that are inherited from higher levels of the resource
    /// hierarchy, including the organization, the folder, and the project.
    ///
    /// To learn more about the resource hierarchy, see
    /// <https://cloud.google.com/iam/help/resource-hierarchy.>
    #[prost(message, repeated, tag = "2")]
    pub explained_policies: ::prost::alloc::vec::Vec<ExplainedPolicy>,
}
#[doc = r" Generated client implementations."]
pub mod iam_checker_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " IAM Policy Troubleshooter service."]
    #[doc = ""]
    #[doc = " This service helps you troubleshoot access issues for Google Cloud resources."]
    #[derive(Debug, Clone)]
    pub struct IamCheckerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IamCheckerClient<T>
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
        ) -> IamCheckerClient<InterceptedService<T, F>>
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
            IamCheckerClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Checks whether a member has a specific permission for a specific resource,"]
        #[doc = " and explains why the member does or does not have that permission."]
        pub async fn troubleshoot_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::TroubleshootIamPolicyRequest>,
        ) -> Result<tonic::Response<super::TroubleshootIamPolicyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.policytroubleshooter.v1.IamChecker/TroubleshootIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
