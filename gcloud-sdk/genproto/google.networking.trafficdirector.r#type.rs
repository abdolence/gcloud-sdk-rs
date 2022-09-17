/// A common proto for describing how the Traffic Director handles
/// xDS-connections/requests/responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficDirectorLogEntry {
    /// An ID of xDS-client connecting to the Traffic Director.
    #[prost(string, tag="1")]
    pub node_id: ::prost::alloc::string::String,
    /// The string representation of IPv4 or IPv6 address of xDS-client
    /// connecting to the Traffic Director.
    /// IPv4 address must be in the format defined in RFC791, four octets separated
    /// by a period. Size of a string is between 7-15 characters. Example: 1.2.3.4
    /// IPv6 address must be in one of the formats defined in RFC4291. Size of a
    /// string is between 7-39 characters. Example: 2001:DB8:0:0:8:800:200C:417A
    #[prost(string, tag="2")]
    pub node_ip: ::prost::alloc::string::String,
    /// A free text describing details of the event.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// Type of xDS-client connecting to Traffic Director
    #[prost(enumeration="traffic_director_log_entry::ClientType", tag="5")]
    pub client_type: i32,
    /// The version of xDS-client connecting to Traffic Director.
    #[prost(string, tag="6")]
    pub client_version: ::prost::alloc::string::String,
    /// The xDS API version used by xDS clients connecting to Traffic Director.
    #[prost(enumeration="traffic_director_log_entry::TransportApiVersion", tag="7")]
    pub transport_api_version: i32,
}
/// Nested message and enum types in `TrafficDirectorLogEntry`.
pub mod traffic_director_log_entry {
    /// Defines possible values of client type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClientType {
        /// Unspecified.
        Unspecified = 0,
        /// Envoy client.
        Envoy = 1,
        /// gRPC Java client.
        GrpcJava = 2,
        /// gRPC C++ client.
        GrpcCpp = 3,
        /// gRPC Python client.
        GrpcPython = 4,
        /// gRPC Go client.
        GrpcGo = 5,
        /// gRPC Ruby client.
        GrpcRuby = 6,
        /// gRPC Ruby client.
        GrpcPhp = 7,
        /// gRPC Node client.
        GrpcNode = 8,
        /// gRPC CSharp client.
        GrpcCsharp = 9,
        /// unknown client type.
        Unknown = 10,
    }
    impl ClientType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClientType::Unspecified => "CLIENT_TYPE_UNSPECIFIED",
                ClientType::Envoy => "ENVOY",
                ClientType::GrpcJava => "GRPC_JAVA",
                ClientType::GrpcCpp => "GRPC_CPP",
                ClientType::GrpcPython => "GRPC_PYTHON",
                ClientType::GrpcGo => "GRPC_GO",
                ClientType::GrpcRuby => "GRPC_RUBY",
                ClientType::GrpcPhp => "GRPC_PHP",
                ClientType::GrpcNode => "GRPC_NODE",
                ClientType::GrpcCsharp => "GRPC_CSHARP",
                ClientType::Unknown => "UNKNOWN",
            }
        }
    }
    /// Defines possible values of API version.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TransportApiVersion {
        /// Unspecified.
        Unspecified = 0,
        /// v2 xDS version.
        V2 = 1,
        /// v3 xDS version.
        V3 = 2,
    }
    impl TransportApiVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TransportApiVersion::Unspecified => "TRANSPORT_API_VERSION_UNSPECIFIED",
                TransportApiVersion::V2 => "V2",
                TransportApiVersion::V3 => "V3",
            }
        }
    }
}
