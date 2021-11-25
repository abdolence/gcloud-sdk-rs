/// A `constraint` describes a way to restrict resource's configuration. For
/// example, you could enforce a constraint that controls which cloud services
/// can be activated across an organization, or whether a Compute Engine instance
/// can have serial port connections established. `Constraints` can be configured
/// by the organization's policy adminstrator to fit the needs of the organzation
/// by setting a `policy` that includes `constraints` at different locations in
/// the organization's resource hierarchy. Policies are inherited down the
/// resource hierarchy from higher levels, but can also be overridden. For
/// details about the inheritance rules please read about
/// \[`policies`][google.cloud.OrgPolicy.v2.Policy\].
///
/// `Constraints` have a default behavior determined by the `constraint_default`
/// field, which is the enforcement behavior that is used in the absence of a
/// `policy` being defined or inherited for the resource in question.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraint {
    /// Immutable. The resource name of the Constraint. Must be in one of
    /// the following forms:
    /// * `projects/{project_number}/constraints/{constraint_name}`
    /// * `folders/{folder_id}/constraints/{constraint_name}`
    /// * `organizations/{organization_id}/constraints/{constraint_name}`
    ///
    /// For example, "/projects/123/constraints/compute.disableSerialPortAccess".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The human readable name.
    ///
    /// Mutable.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Detailed description of what this `Constraint` controls as well as how and
    /// where it is enforced.
    ///
    /// Mutable.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The evaluation behavior of this constraint in the absence of 'Policy'.
    #[prost(enumeration = "constraint::ConstraintDefault", tag = "4")]
    pub constraint_default: i32,
    /// The type of restrictions for this `Constraint`.
    ///
    /// Immutable after creation.
    #[prost(oneof = "constraint::ConstraintType", tags = "5, 6")]
    pub constraint_type: ::core::option::Option<constraint::ConstraintType>,
}
/// Nested message and enum types in `Constraint`.
pub mod constraint {
    /// A `Constraint` that allows or disallows a list of string values, which are
    /// configured by an Organization's policy administrator with a `Policy`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListConstraint {
        /// Indicates whether values grouped into categories can be used in
        /// `Policy.allowed_values` and `Policy.denied_values`. For example,
        /// `"in:Python"` would match any value in the 'Python' group.
        #[prost(bool, tag = "1")]
        pub supports_in: bool,
        /// Indicates whether subtrees of Cloud Resource Manager resource hierarchy
        /// can be used in `Policy.allowed_values` and `Policy.denied_values`. For
        /// example, `"under:folders/123"` would match any resource under the
        /// 'folders/123' folder.
        #[prost(bool, tag = "2")]
        pub supports_under: bool,
    }
    /// A `Constraint` that is either enforced or not.
    ///
    /// For example a constraint `constraints/compute.disableSerialPortAccess`.
    /// If it is enforced on a VM instance, serial port connections will not be
    /// opened to that instance.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BooleanConstraint {}
    /// Specifies the default behavior in the absence of any `Policy` for the
    /// `Constraint`. This must not be `CONSTRAINT_DEFAULT_UNSPECIFIED`.
    ///
    /// Immutable after creation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConstraintDefault {
        /// This is only used for distinguishing unset values and should never be
        /// used.
        Unspecified = 0,
        /// Indicate that all values are allowed for list constraints.
        /// Indicate that enforcement is off for boolean constraints.
        Allow = 1,
        /// Indicate that all values are denied for list constraints.
        /// Indicate that enforcement is on for boolean constraints.
        Deny = 2,
    }
    /// The type of restrictions for this `Constraint`.
    ///
    /// Immutable after creation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConstraintType {
        /// Defines this constraint as being a ListConstraint.
        #[prost(message, tag = "5")]
        ListConstraint(ListConstraint),
        /// Defines this constraint as being a BooleanConstraint.
        #[prost(message, tag = "6")]
        BooleanConstraint(BooleanConstraint),
    }
}
/// Defines a Cloud Organization `Policy` which is used to specify `Constraints`
/// for configurations of Cloud Platform resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Policy {
    /// Immutable. The resource name of the Policy. Must be one of the following
    /// forms, where constraint_name is the name of the constraint which this
    /// Policy configures:
    /// * `projects/{project_number}/policies/{constraint_name}`
    /// * `folders/{folder_id}/policies/{constraint_name}`
    /// * `organizations/{organization_id}/policies/{constraint_name}`
    ///
    /// For example, "projects/123/policies/compute.disableSerialPortAccess".
    ///
    /// Note: `projects/{project_id}/policies/{constraint_name}` is also an
    /// acceptable name for API requests, but responses will return the name using
    /// the equivalent project number.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Basic information about the Organization Policy.
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<PolicySpec>,
    /// An alternate policy configuration that will be used instead of the baseline
    /// policy configurations as determined by the launch.
    /// Currently the only way the launch can trigger the alternate configuration
    /// is via dry-run/darklaunch.
    #[prost(message, optional, tag = "3")]
    pub alternate: ::core::option::Option<AlternatePolicySpec>,
}
/// Similar to PolicySpec but with an extra 'launch' field for launch reference.
/// The PolicySpec here is specific for dry-run/darklaunch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlternatePolicySpec {
    /// Reference to the launch that will be used while audit logging and to
    /// control the launch.
    /// Should be set only in the alternate policy.
    #[prost(string, tag = "1")]
    pub launch: ::prost::alloc::string::String,
    /// Specify `Constraint` for configurations of Cloud Platform resources.
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<PolicySpec>,
}
/// Defines a Cloud Organization `PolicySpec` which is used to specify
/// `Constraints` for configurations of Cloud Platform resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicySpec {
    /// An opaque tag indicating the current version of the `Policy`, used for
    /// concurrency control.
    ///
    /// This field is ignored if used in a `CreatePolicy` request.
    ///
    /// When the `Policy` is returned from either a `GetPolicy` or a
    /// `ListPolicies` request, this `etag` indicates the version of the
    /// current `Policy` to use when executing a read-modify-write loop.
    ///
    /// When the `Policy` is returned from a `GetEffectivePolicy` request, the
    /// `etag` will be unset.
    #[prost(string, tag = "1")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. The time stamp this was previously updated. This
    /// represents the last time a call to `CreatePolicy` or `UpdatePolicy` was
    /// made for that `Policy`.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Up to 10 PolicyRules are allowed.
    ///
    /// In Policies for boolean constraints, the following requirements apply:
    ///   - There must be one and only one PolicyRule where condition is unset.
    ///   - BooleanPolicyRules with conditions must set `enforced` to the opposite
    ///     of the PolicyRule without a condition.
    ///   - During policy evaluation, PolicyRules with conditions that are
    ///     true for a target resource take precedence.
    #[prost(message, repeated, tag = "3")]
    pub rules: ::prost::alloc::vec::Vec<policy_spec::PolicyRule>,
    /// Determines the inheritance behavior for this `Policy`.
    ///
    /// If `inherit_from_parent` is true, PolicyRules set higher up in the
    /// hierarchy (up to the closest root) are inherited and present in the
    /// effective policy. If it is false, then no rules are inherited, and this
    /// Policy becomes the new root for evaluation.
    /// This field can be set only for Policies which configure list constraints.
    #[prost(bool, tag = "4")]
    pub inherit_from_parent: bool,
    /// Ignores policies set above this resource and restores the
    /// `constraint_default` enforcement behavior of the specific `Constraint` at
    /// this resource.
    /// This field can be set in policies for either list or boolean
    /// constraints. If set, `rules` must be empty and `inherit_from_parent`
    /// must be set to false.
    #[prost(bool, tag = "5")]
    pub reset: bool,
}
/// Nested message and enum types in `PolicySpec`.
pub mod policy_spec {
    /// A rule used to express this policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PolicyRule {
        /// A condition which determines whether this rule is used
        /// in the evaluation of the policy. When set, the `expression` field in
        /// the `Expr' must include from 1 to 10 subexpressions, joined by the "||"
        /// or "&&" operators. Each subexpression must be of the form
        /// "resource.matchLabels(key_name, value_name)",
        /// where key_name and value_name are the resource names for Label Keys
        /// and Values. These names are available from the Label Manager Service. An
        /// example expression is:
        /// "resource.matchLabels('labelKeys/123, 'labelValues/456')".
        #[prost(message, optional, tag = "5")]
        pub condition: ::core::option::Option<super::super::super::super::r#type::Expr>,
        #[prost(oneof = "policy_rule::Kind", tags = "1, 2, 3, 4")]
        pub kind: ::core::option::Option<policy_rule::Kind>,
    }
    /// Nested message and enum types in `PolicyRule`.
    pub mod policy_rule {
        /// A message that holds specific allowed and denied values.
        /// This message can define specific values and subtrees of Cloud Resource
        /// Manager resource hierarchy (`Organizations`, `Folders`, `Projects`) that
        /// are allowed or denied. This is achieved by using the `under:` and
        /// optional `is:` prefixes.
        /// The `under:` prefix is used to denote resource subtree values.
        /// The `is:` prefix is used to denote specific values, and is required only
        /// if the value contains a ":". Values prefixed with "is:" are treated the
        /// same as values with no prefix.
        /// Ancestry subtrees must be in one of the following formats:
        ///     - "projects/<project-id>", e.g. "projects/tokyo-rain-123"
        ///     - "folders/<folder-id>", e.g. "folders/1234"
        ///     - "organizations/<organization-id>", e.g. "organizations/1234"
        /// The `supports_under` field of the associated `Constraint`  defines
        /// whether ancestry prefixes can be used.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct StringValues {
            /// List of values allowed at this resource.
            #[prost(string, repeated, tag = "1")]
            pub allowed_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// List of values denied at this resource.
            #[prost(string, repeated, tag = "2")]
            pub denied_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Kind {
            /// List of values to be used for this PolicyRule. This field can be set
            /// only in Policies for list constraints.
            #[prost(message, tag = "1")]
            Values(StringValues),
            /// Setting this to true means that all values are allowed. This field can
            /// be set only in Policies for list constraints.
            #[prost(bool, tag = "2")]
            AllowAll(bool),
            /// Setting this to true means that all values are denied. This field can
            /// be set only in Policies for list constraints.
            #[prost(bool, tag = "3")]
            DenyAll(bool),
            /// If `true`, then the `Policy` is enforced. If `false`, then any
            /// configuration is acceptable.
            /// This field can be set only in Policies for boolean constraints.
            #[prost(bool, tag = "4")]
            Enforce(bool),
        }
    }
}
/// The request sent to the \[ListConstraints\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.ListConstraints\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConstraintsRequest {
    /// Required. The Cloud resource that parents the constraint. Must be in one of the
    /// following forms:
    /// * `projects/{project_number}`
    /// * `projects/{project_id}`
    /// * `folders/{folder_id}`
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Size of the pages to be returned. This is currently unsupported and will
    /// be ignored. The server may at any point start using this field to limit
    /// page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token used to retrieve the next page. This is currently unsupported
    /// and will be ignored. The server may at any point start using this field.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response returned from the \[ListConstraints\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.ListConstraints\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConstraintsResponse {
    /// The collection of constraints that are available on the targeted resource.
    #[prost(message, repeated, tag = "1")]
    pub constraints: ::prost::alloc::vec::Vec<Constraint>,
    /// Page token used to retrieve the next page. This is currently not used.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request sent to the \[ListPolicies\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.ListPolicies\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesRequest {
    /// Required. The target Cloud resource that parents the set of constraints and policies
    /// that will be returned from this call. Must be in one of the following
    /// forms:
    /// * `projects/{project_number}`
    /// * `projects/{project_id}`
    /// * `folders/{folder_id}`
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Size of the pages to be returned. This is currently unsupported and will
    /// be ignored. The server may at any point start using this field to limit
    /// page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token used to retrieve the next page. This is currently unsupported
    /// and will be ignored. The server may at any point start using this field.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response returned from the \[ListPolicies\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.ListPolicies\] method. It will be empty
/// if no `Policies` are set on the resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPoliciesResponse {
    /// All `Policies` that exist on the resource. It will be empty if no
    /// `Policies` are set.
    #[prost(message, repeated, tag = "1")]
    pub policies: ::prost::alloc::vec::Vec<Policy>,
    /// Page token used to retrieve the next page. This is currently not used, but
    /// the server may at any point start supplying a valid token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request sent to the \[GetPolicy\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.GetPolicy\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPolicyRequest {
    /// Required. Resource name of the policy. See `Policy` for naming requirements.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to the \[GetEffectivePolicy\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.GetEffectivePolicy\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEffectivePolicyRequest {
    /// Required. The effective policy to compute. See `Policy` for naming rules.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to the \[CreatePolicyRequest\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.CreatePolicy\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePolicyRequest {
    /// Required. The Cloud resource that will parent the new Policy. Must be in one of the
    /// following forms:
    /// * `projects/{project_number}`
    /// * `projects/{project_id}`
    /// * `folders/{folder_id}`
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. `Policy` to create.
    #[prost(message, optional, tag = "3")]
    pub policy: ::core::option::Option<Policy>,
}
/// The request sent to the \[UpdatePolicyRequest\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.UpdatePolicy\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePolicyRequest {
    /// Required. `Policy` to update.
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<Policy>,
}
/// The request sent to the \[DeletePolicy\]
/// \[google.cloud.orgpolicy.v2.OrgPolicy.DeletePolicy\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePolicyRequest {
    /// Required. Name of the policy to delete.
    /// See `Policy` for naming rules.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod org_policy_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " An interface for managing organization policies."]
    #[doc = ""]
    #[doc = " The Cloud Org Policy service provides a simple mechanism for organizations to"]
    #[doc = " restrict the allowed configurations across their entire Cloud Resource"]
    #[doc = " hierarchy."]
    #[doc = ""]
    #[doc = " You can use a `policy` to configure restrictions in Cloud resources. For"]
    #[doc = " example, you can enforce a `policy` that restricts which Google"]
    #[doc = " Cloud Platform APIs can be activated in a certain part of your resource"]
    #[doc = " hierarchy, or prevents serial port access to VM instances in a particular"]
    #[doc = " folder."]
    #[doc = ""]
    #[doc = " `Policies` are inherited down through the resource hierarchy. A `policy`"]
    #[doc = " applied to a parent resource automatically applies to all its child resources"]
    #[doc = " unless overridden with a `policy` lower in the hierarchy."]
    #[doc = ""]
    #[doc = " A `constraint` defines an aspect of a resource's configuration that can be"]
    #[doc = " controlled by an organization's policy administrator. `Policies` are a"]
    #[doc = " collection of `constraints` that defines their allowable configuration on a"]
    #[doc = " particular resource and its child resources."]
    #[derive(Debug, Clone)]
    pub struct OrgPolicyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OrgPolicyClient<T>
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
        ) -> OrgPolicyClient<InterceptedService<T, F>>
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
            OrgPolicyClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists `Constraints` that could be applied on the specified resource."]
        pub async fn list_constraints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConstraintsRequest>,
        ) -> Result<tonic::Response<super::ListConstraintsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.orgpolicy.v2.OrgPolicy/ListConstraints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves all of the `Policies` that exist on a particular resource."]
        pub async fn list_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListPoliciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.orgpolicy.v2.OrgPolicy/ListPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a `Policy` on a resource."]
        #[doc = ""]
        #[doc = " If no `Policy` is set on the resource, NOT_FOUND is returned. The"]
        #[doc = " `etag` value can be used with `UpdatePolicy()` to update a"]
        #[doc = " `Policy` during read-modify-write."]
        pub async fn get_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPolicyRequest>,
        ) -> Result<tonic::Response<super::Policy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.orgpolicy.v2.OrgPolicy/GetPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the effective `Policy` on a resource. This is the result of merging"]
        #[doc = " `Policies` in the resource hierarchy and evaluating conditions. The"]
        #[doc = " returned `Policy` will not have an `etag` or `condition` set because it is"]
        #[doc = " a computed `Policy` across multiple resources."]
        #[doc = " Subtrees of Resource Manager resource hierarchy with 'under:' prefix will"]
        #[doc = " not be expanded."]
        pub async fn get_effective_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEffectivePolicyRequest>,
        ) -> Result<tonic::Response<super::Policy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.orgpolicy.v2.OrgPolicy/GetEffectivePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a Policy."]
        #[doc = ""]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the"]
        #[doc = " constraint does not exist."]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the"]
        #[doc = " policy already exists on the given Cloud resource."]
        pub async fn create_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePolicyRequest>,
        ) -> Result<tonic::Response<super::Policy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.orgpolicy.v2.OrgPolicy/CreatePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a Policy."]
        #[doc = ""]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the"]
        #[doc = " constraint or the policy do not exist."]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag"]
        #[doc = " supplied in the request does not match the persisted etag of the policy"]
        #[doc = ""]
        #[doc = " Note: the supplied policy will perform a full overwrite of all"]
        #[doc = " fields."]
        pub async fn update_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePolicyRequest>,
        ) -> Result<tonic::Response<super::Policy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.orgpolicy.v2.OrgPolicy/UpdatePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a Policy."]
        #[doc = ""]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the"]
        #[doc = " constraint or Org Policy does not exist."]
        pub async fn delete_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePolicyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.orgpolicy.v2.OrgPolicy/DeletePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
