/// An artifact that can be deployed in some runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployable {
    /// Required. Resource URI for the artifact being deployed.
    #[prost(string, repeated, tag="1")]
    pub resource_uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Details of a deployment occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Details {
    /// Required. Deployment history for the resource.
    #[prost(message, optional, tag="1")]
    pub deployment: ::core::option::Option<Deployment>,
}
/// The period during which some deployable was active in a runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// Identity of the user that triggered this deployment.
    #[prost(string, tag="1")]
    pub user_email: ::prost::alloc::string::String,
    /// Required. Beginning of the lifetime of this deployment.
    #[prost(message, optional, tag="2")]
    pub deploy_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End of the lifetime of this deployment.
    #[prost(message, optional, tag="3")]
    pub undeploy_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Configuration used to create this deployment.
    #[prost(string, tag="4")]
    pub config: ::prost::alloc::string::String,
    /// Address of the runtime element hosting this deployment.
    #[prost(string, tag="5")]
    pub address: ::prost::alloc::string::String,
    /// Output only. Resource URI for the artifact being deployed taken from
    /// the deployable field with the same name.
    #[prost(string, repeated, tag="6")]
    pub resource_uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Platform hosting this deployment.
    #[prost(enumeration="deployment::Platform", tag="7")]
    pub platform: i32,
}
/// Nested message and enum types in `Deployment`.
pub mod deployment {
    /// Types of platforms.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Platform {
        /// Unknown.
        Unspecified = 0,
        /// Google Container Engine.
        Gke = 1,
        /// Google App Engine: Flexible Environment.
        Flex = 2,
        /// Custom user-defined platform.
        Custom = 3,
    }
    impl Platform {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Platform::Unspecified => "PLATFORM_UNSPECIFIED",
                Platform::Gke => "GKE",
                Platform::Flex => "FLEX",
                Platform::Custom => "CUSTOM",
            }
        }
    }
}
