/// An `AccessLevel` is a label that can be applied to requests to Google Cloud
/// services, along with a list of requirements necessary for the label to be
/// applied.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessLevel {
    /// Required. Resource name for the Access Level. The `short_name` component
    /// must begin with a letter and only include alphanumeric and '_'. Format:
    /// `accessPolicies/{policy_id}/accessLevels/{short_name}`. The maximum length
    /// of the `short_name` component is 50 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human readable title. Must be unique within the Policy.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Description of the `AccessLevel` and its use. Does not affect behavior.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time the `AccessLevel` was created in UTC.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the `AccessLevel` was updated in UTC.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Describes the necessary conditions for the level to apply.
    #[prost(oneof = "access_level::Level", tags = "4, 5")]
    pub level: ::core::option::Option<access_level::Level>,
}
/// Nested message and enum types in `AccessLevel`.
pub mod access_level {
    /// Required. Describes the necessary conditions for the level to apply.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Level {
        /// A `BasicLevel` composed of `Conditions`.
        #[prost(message, tag = "4")]
        Basic(super::BasicLevel),
        /// A `CustomLevel` written in the Common Expression Language.
        #[prost(message, tag = "5")]
        Custom(super::CustomLevel),
    }
}
/// `BasicLevel` is an `AccessLevel` using a set of recommended features.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicLevel {
    /// Required. A list of requirements for the `AccessLevel` to be granted.
    #[prost(message, repeated, tag = "1")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// How the `conditions` list should be combined to determine if a request is
    /// granted this `AccessLevel`. If AND is used, each `Condition` in
    /// `conditions` must be satisfied for the `AccessLevel` to be applied. If OR
    /// is used, at least one `Condition` in `conditions` must be satisfied for the
    /// `AccessLevel` to be applied. Default behavior is AND.
    #[prost(enumeration = "basic_level::ConditionCombiningFunction", tag = "2")]
    pub combining_function: i32,
}
/// Nested message and enum types in `BasicLevel`.
pub mod basic_level {
    /// Options for how the `conditions` list should be combined to determine if
    /// this `AccessLevel` is applied. Default is AND.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConditionCombiningFunction {
        /// All `Conditions` must be true for the `BasicLevel` to be true.
        And = 0,
        /// If at least one `Condition` is true, then the `BasicLevel` is true.
        Or = 1,
    }
}
/// A condition necessary for an `AccessLevel` to be granted. The Condition is an
/// AND over its fields. So a Condition is true if: 1) the request IP is from one
/// of the listed subnetworks AND 2) the originating device complies with the
/// listed device policy AND 3) all listed access levels are granted AND 4) the
/// request was sent at a time allowed by the DateTimeRestriction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// CIDR block IP subnetwork specification. May be IPv4 or IPv6. Note that for
    /// a CIDR IP address block, the specified IP address portion must be properly
    /// truncated (i.e. all the host bits must be zero) or the input is considered
    /// malformed. For example, "192.0.2.0/24" is accepted but "192.0.2.1/24" is
    /// not. Similarly, for IPv6, "2001:db8::/32" is accepted whereas
    /// "2001:db8::1/32" is not. The originating IP of a request must be in one of
    /// the listed subnets in order for this Condition to be true. If empty, all IP
    /// addresses are allowed.
    #[prost(string, repeated, tag = "1")]
    pub ip_subnetworks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Device specific restrictions, all restrictions must hold for the
    /// Condition to be true. If not specified, all devices are allowed.
    #[prost(message, optional, tag = "2")]
    pub device_policy: ::core::option::Option<DevicePolicy>,
    /// A list of other access levels defined in the same `Policy`, referenced by
    /// resource name. Referencing an `AccessLevel` which does not exist is an
    /// error. All access levels listed must be granted for the Condition
    /// to be true. Example:
    /// "`accessPolicies/MY_POLICY/accessLevels/LEVEL_NAME"`
    #[prost(string, repeated, tag = "3")]
    pub required_access_levels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Whether to negate the Condition. If true, the Condition becomes a NAND over
    /// its non-empty fields, each field must be false for the Condition overall to
    /// be satisfied. Defaults to false.
    #[prost(bool, tag = "5")]
    pub negate: bool,
    /// The request must be made by one of the provided user or service
    /// accounts. Groups are not supported.
    /// Syntax:
    /// `user:{emailid}`
    /// `serviceAccount:{emailid}`
    /// If not specified, a request may come from any user.
    #[prost(string, repeated, tag = "6")]
    pub members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The request must originate from one of the provided countries/regions.
    /// Must be valid ISO 3166-1 alpha-2 codes.
    #[prost(string, repeated, tag = "7")]
    pub regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// `CustomLevel` is an `AccessLevel` using the Cloud Common Expression Language
/// to represent the necessary conditions for the level to apply to a request.
/// See CEL spec at: https://github.com/google/cel-spec
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomLevel {
    /// Required. A Cloud CEL expression evaluating to a boolean.
    #[prost(message, optional, tag = "1")]
    pub expr: ::core::option::Option<super::super::super::r#type::Expr>,
}
/// `DevicePolicy` specifies device specific restrictions necessary to acquire a
/// given access level. A `DevicePolicy` specifies requirements for requests from
/// devices to be granted access levels, it does not do any enforcement on the
/// device. `DevicePolicy` acts as an AND over all specified fields, and each
/// repeated field is an OR over its elements. Any unset fields are ignored. For
/// example, if the proto is { os_type : DESKTOP_WINDOWS, os_type :
/// DESKTOP_LINUX, encryption_status: ENCRYPTED}, then the DevicePolicy will be
/// true for requests originating from encrypted Linux desktops and encrypted
/// Windows desktops.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DevicePolicy {
    /// Whether or not screenlock is required for the DevicePolicy to be true.
    /// Defaults to `false`.
    #[prost(bool, tag = "1")]
    pub require_screenlock: bool,
    /// Allowed encryptions statuses, an empty list allows all statuses.
    #[prost(
        enumeration = "super::r#type::DeviceEncryptionStatus",
        repeated,
        tag = "2"
    )]
    pub allowed_encryption_statuses: ::prost::alloc::vec::Vec<i32>,
    /// Allowed OS versions, an empty list allows all types and all versions.
    #[prost(message, repeated, tag = "3")]
    pub os_constraints: ::prost::alloc::vec::Vec<OsConstraint>,
    /// Allowed device management levels, an empty list allows all management
    /// levels.
    #[prost(
        enumeration = "super::r#type::DeviceManagementLevel",
        repeated,
        tag = "6"
    )]
    pub allowed_device_management_levels: ::prost::alloc::vec::Vec<i32>,
    /// Whether the device needs to be approved by the customer admin.
    #[prost(bool, tag = "7")]
    pub require_admin_approval: bool,
    /// Whether the device needs to be corp owned.
    #[prost(bool, tag = "8")]
    pub require_corp_owned: bool,
}
/// A restriction on the OS type and version of devices making requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsConstraint {
    /// Required. The allowed OS type.
    #[prost(enumeration = "super::r#type::OsType", tag = "1")]
    pub os_type: i32,
    /// The minimum allowed OS version. If not set, any version of this OS
    /// satisfies the constraint. Format: `"major.minor.patch"`.
    /// Examples: `"10.5.301"`, `"9.2.1"`.
    #[prost(string, tag = "2")]
    pub minimum_version: ::prost::alloc::string::String,
    /// Only allows requests from devices with a verified Chrome OS.
    /// Verifications includes requirements that the device is enterprise-managed,
    /// conformant to domain policies, and the caller has permission to call
    /// the API targeted by the request.
    #[prost(bool, tag = "3")]
    pub require_verified_chrome_os: bool,
}
/// `AccessPolicy` is a container for `AccessLevels` (which define the necessary
/// attributes to use Google Cloud services) and `ServicePerimeters` (which
/// define regions of services able to freely pass data within a perimeter). An
/// access policy is globally visible within an organization, and the
/// restrictions it specifies apply to all projects within an organization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessPolicy {
    /// Output only. Resource name of the `AccessPolicy`. Format:
    /// `accessPolicies/{policy_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The parent of this `AccessPolicy` in the Cloud Resource
    /// Hierarchy. Currently immutable once created. Format:
    /// `organizations/{organization_id}`
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Human readable title. Does not affect behavior.
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
    /// Output only. Time the `AccessPolicy` was created in UTC.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the `AccessPolicy` was updated in UTC.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. An opaque identifier for the current version of the
    /// `AccessPolicy`. This will always be a strongly validated etag, meaning that
    /// two Access Polices will be identical if and only if their etags are
    /// identical. Clients should not expect this to be in any specific format.
    #[prost(string, tag = "6")]
    pub etag: ::prost::alloc::string::String,
}
/// `ServicePerimeter` describes a set of Google Cloud resources which can freely
/// import and export data amongst themselves, but not export outside of the
/// `ServicePerimeter`. If a request with a source within this `ServicePerimeter`
/// has a target outside of the `ServicePerimeter`, the request will be blocked.
/// Otherwise the request is allowed. There are two types of Service Perimeter -
/// Regular and Bridge. Regular Service Perimeters cannot overlap, a single
/// Google Cloud project can only belong to a single regular Service Perimeter.
/// Service Perimeter Bridges can contain only Google Cloud projects as members,
/// a single Google Cloud project may belong to multiple Service Perimeter
/// Bridges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServicePerimeter {
    /// Required. Resource name for the ServicePerimeter.  The `short_name`
    /// component must begin with a letter and only include alphanumeric and '_'.
    /// Format: `accessPolicies/{policy_id}/servicePerimeters/{short_name}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human readable title. Must be unique within the Policy.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Description of the `ServicePerimeter` and its use. Does not affect
    /// behavior.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Time the `ServicePerimeter` was created in UTC.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the `ServicePerimeter` was updated in UTC.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Perimeter type indicator. A single project is
    /// allowed to be a member of single regular perimeter, but multiple service
    /// perimeter bridges. A project cannot be a included in a perimeter bridge
    /// without being included in regular perimeter. For perimeter bridges,
    /// the restricted service list as well as access level lists must be
    /// empty.
    #[prost(enumeration = "service_perimeter::PerimeterType", tag = "6")]
    pub perimeter_type: i32,
    /// Current ServicePerimeter configuration. Specifies sets of resources,
    /// restricted services and access levels that determine perimeter
    /// content and boundaries.
    #[prost(message, optional, tag = "7")]
    pub status: ::core::option::Option<ServicePerimeterConfig>,
    /// Proposed (or dry run) ServicePerimeter configuration. This configuration
    /// allows to specify and test ServicePerimeter configuration without enforcing
    /// actual access restrictions. Only allowed to be set when the
    /// "use_explicit_dry_run_spec" flag is set.
    #[prost(message, optional, tag = "8")]
    pub spec: ::core::option::Option<ServicePerimeterConfig>,
    /// Use explicit dry run spec flag. Ordinarily, a dry-run spec implicitly
    /// exists  for all Service Perimeters, and that spec is identical to the
    /// status for those Service Perimeters. When this flag is set, it inhibits the
    /// generation of the implicit spec, thereby allowing the user to explicitly
    /// provide a configuration ("spec") to use in a dry-run version of the Service
    /// Perimeter. This allows the user to test changes to the enforced config
    /// ("status") without actually enforcing them. This testing is done through
    /// analyzing the differences between currently enforced and suggested
    /// restrictions. use_explicit_dry_run_spec must bet set to True if any of the
    /// fields in the spec are set to non-default values.
    #[prost(bool, tag = "9")]
    pub use_explicit_dry_run_spec: bool,
}
/// Nested message and enum types in `ServicePerimeter`.
pub mod service_perimeter {
    /// Specifies the type of the Perimeter. There are two types: regular and
    /// bridge. Regular Service Perimeter contains resources, access levels, and
    /// restricted services. Every resource can be in at most ONE
    /// regular Service Perimeter.
    ///
    /// In addition to being in a regular service perimeter, a resource can also
    /// be in zero or more perimeter bridges.  A perimeter bridge only contains
    /// resources.  Cross project operations are permitted if all effected
    /// resources share some perimeter (whether bridge or regular). Perimeter
    /// Bridge does not contain access levels or services: those are governed
    /// entirely by the regular perimeter that resource is in.
    ///
    /// Perimeter Bridges are typically useful when building more complex toplogies
    /// with many independent perimeters that need to share some data with a common
    /// perimeter, but should not be able to share data among themselves.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PerimeterType {
        /// Regular Perimeter.
        Regular = 0,
        /// Perimeter Bridge.
        Bridge = 1,
    }
}
/// `ServicePerimeterConfig` specifies a set of Google Cloud resources that
/// describe specific Service Perimeter configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServicePerimeterConfig {
    /// A list of Google Cloud resources that are inside of the service perimeter.
    /// Currently only projects are allowed. Format: `projects/{project_number}`
    #[prost(string, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of `AccessLevel` resource names that allow resources within the
    /// `ServicePerimeter` to be accessed from the internet. `AccessLevels` listed
    /// must be in the same policy as this `ServicePerimeter`. Referencing a
    /// nonexistent `AccessLevel` is a syntax error. If no `AccessLevel` names are
    /// listed, resources within the perimeter can only be accessed via Google
    /// Cloud calls with request origins within the perimeter. Example:
    /// `"accessPolicies/MY_POLICY/accessLevels/MY_LEVEL"`.
    /// For Service Perimeter Bridge, must be empty.
    #[prost(string, repeated, tag = "2")]
    pub access_levels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Google Cloud services that are subject to the Service Perimeter
    /// restrictions. For example, if `storage.googleapis.com` is specified, access
    /// to the storage buckets inside the perimeter must meet the perimeter's
    /// access restrictions.
    #[prost(string, repeated, tag = "4")]
    pub restricted_services: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Configuration for APIs allowed within Perimeter.
    #[prost(message, optional, tag = "10")]
    pub vpc_accessible_services:
        ::core::option::Option<service_perimeter_config::VpcAccessibleServices>,
}
/// Nested message and enum types in `ServicePerimeterConfig`.
pub mod service_perimeter_config {
    /// Specifies how APIs are allowed to communicate within the Service
    /// Perimeter.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VpcAccessibleServices {
        /// Whether to restrict API calls within the Service Perimeter to the list of
        /// APIs specified in 'allowed_services'.
        #[prost(bool, tag = "1")]
        pub enable_restriction: bool,
        /// The list of APIs usable within the Service Perimeter. Must be empty
        /// unless 'enable_restriction' is True.
        #[prost(string, repeated, tag = "2")]
        pub allowed_services: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
