/// Device resource represents an instance of enterprise managed device in the
/// property.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// Required. The resource name of the device. For example:
    /// "enterprises/XYZ/devices/123".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. Type of the device for general display purposes.
    /// For example: "THERMOSTAT". The device type should not be used to deduce or
    /// infer functionality of the actual device it is assigned to. Instead, use
    /// the returned traits for the device.
    #[prost(string, tag = "2")]
    pub r#type: std::string::String,
    /// Output only. Device traits.
    #[prost(message, optional, tag = "4")]
    pub traits: ::std::option::Option<::prost_types::Struct>,
    /// Assignee details of the device.
    #[prost(message, repeated, tag = "5")]
    pub parent_relations: ::std::vec::Vec<ParentRelation>,
}
/// Represents device relationships, for instance, structure/room to which the
/// device is assigned to. For now this is only filled in the enterprise flow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentRelation {
    /// Output only. The name of the relation -- e.g., structure/room where the
    /// device is assigned to. For example: "enterprises/XYZ/structures/ABC" or
    /// "enterprises/XYZ/structures/ABC/rooms/123"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Output only. The custom name of the relation -- e.g., structure/room where
    /// the device is assigned to.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
}
/// Structure resource represents an instance of enterprise managed home or hotel
/// room.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Structure {
    /// Output only. The resource name of the structure. For example:
    /// "enterprises/XYZ/structures/ABC".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Structure traits.
    #[prost(message, optional, tag = "2")]
    pub traits: ::std::option::Option<::prost_types::Struct>,
    /// Assignee details of the structure.
    #[prost(message, repeated, tag = "3")]
    pub parent_relations: ::std::vec::Vec<StructureParentRelation>,
}
/// Room resource represents an instance of sub-space within a structure such as
/// rooms in a hotel suite or rental apartment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Room {
    /// Output only. The resource name of the room. For example:
    /// "enterprises/XYZ/structures/ABC/rooms/123".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Room traits.
    #[prost(message, optional, tag = "2")]
    pub traits: ::std::option::Option<::prost_types::Struct>,
}
/// Represents structure assignee relationships, for instance, group to which the
/// structure is assigned to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructureParentRelation {
    /// Output only. The name of the relation -- e.g., group to which the structure
    /// is assigned to. For example: "enterprises/XYZ/groups/ABC"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Output only. The custom name of the relation -- e.g., group, to which the
    /// structure is assigned to.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
}
/// Request message for SmartDeviceManagementService.GetDevice
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeviceRequest {
    /// The name of the device requested. For example:
    /// "enterprises/XYZ/devices/123"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for SmartDeviceManagementService.ListDevices
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDevicesRequest {
    /// The parent enterprise to list devices under. E.g. "enterprises/XYZ".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional requested page size. Server may return fewer devices than
    /// requested. If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional token of the page to retrieve.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional filter to list devices.
    ///
    /// Filters can match the exact assignee (could be a structure or a room).
    /// E.g. 'assignee=enterprises/XYZ/structures/abc'
    /// Also could filter by parent (group):
    /// 'parent=enterprises/XYZ/groups/jkl'
    /// or filter by device custom name (substring match):
    /// 'customName=wing'
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// Response message for SmartDeviceManagementService.ListDevices
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDevicesResponse {
    /// The list of devices.
    #[prost(message, repeated, tag = "1")]
    pub devices: ::std::vec::Vec<Device>,
    /// The pagination token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for SmartDeviceManagementService.ExecuteDeviceCommand
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteDeviceCommandRequest {
    /// The name of the device requested. For example:
    /// "enterprises/XYZ/devices/123"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The command name to execute, represented by the fully qualified protobuf
    /// message name.
    #[prost(string, tag = "2")]
    pub command: std::string::String,
    /// The command message to execute, represented as a Struct.
    #[prost(message, optional, tag = "3")]
    pub params: ::std::option::Option<::prost_types::Struct>,
}
/// Response message for SmartDeviceManagementService.ExecuteDeviceCommand
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteDeviceCommandResponse {
    /// The results of executing the command.
    #[prost(message, optional, tag = "1")]
    pub results: ::std::option::Option<::prost_types::Struct>,
}
/// Request message for SmartDeviceManagementService.GetStructure
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStructureRequest {
    /// The name of the structure requested. For example:
    /// "enterprises/XYZ/structures/ABC".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for SmartDeviceManagementService.ListStructures
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStructuresRequest {
    /// The parent enterprise to list structures under. E.g. "enterprises/XYZ".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Requested page size. Server may return fewer structures than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The token of the page to retrieve.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional filter to list structures.
    ///
    /// Filters can match the exact album assigned to the structure.
    /// E.g. 'album=enterprises/XYZ/albums/abc'
    /// It also support filtering by parent (only groups for now):
    /// E.g. 'parent=enterprises/XYZ/groups/124'
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// Response message for SmartDeviceManagementService.ListStructures
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStructuresResponse {
    /// The list of structures.
    #[prost(message, repeated, tag = "1")]
    pub structures: ::std::vec::Vec<Structure>,
    /// The pagination token to retrieve the next page of results.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for SmartDeviceManagementService.GetRoom
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoomRequest {
    /// The name of the room requested. For example:
    /// "enterprises/XYZ/structures/ABC/rooms/123".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for SmartDeviceManagementService.ListRooms
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoomsRequest {
    /// The parent resource name of the rooms requested. For example:
    /// "enterprises/XYZ/structures/ABC".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Requested page size. Server may return fewer rooms than requested.
    /// If unspecified, server will pick an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The token of the page to retrieve.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for SmartDeviceManagementService.ListRooms
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRoomsResponse {
    /// The list of rooms.
    #[prost(message, repeated, tag = "1")]
    pub rooms: ::std::vec::Vec<Room>,
    /// The pagination token to retrieve the next page of results.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod smart_device_management_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service that allows API consumers to provision and manage Google"]
    #[doc = " Home structures and devices for enterprise use cases."]
    pub struct SmartDeviceManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SmartDeviceManagementServiceClient<T>
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
        #[doc = " Gets a device managed by the enterprise."]
        pub async fn get_device(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeviceRequest>,
        ) -> Result<tonic::Response<super::Device>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/GetDevice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists devices managed by the enterprise."]
        pub async fn list_devices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDevicesRequest>,
        ) -> Result<tonic::Response<super::ListDevicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/ListDevices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Executes a command to device managed by the enterprise."]
        pub async fn execute_device_command(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteDeviceCommandRequest>,
        ) -> Result<tonic::Response<super::ExecuteDeviceCommandResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/ExecuteDeviceCommand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a structure managed by the enterprise."]
        pub async fn get_structure(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStructureRequest>,
        ) -> Result<tonic::Response<super::Structure>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/GetStructure",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists structures managed by the enterprise."]
        pub async fn list_structures(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStructuresRequest>,
        ) -> Result<tonic::Response<super::ListStructuresResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/ListStructures",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a room managed by the enterprise."]
        pub async fn get_room(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRoomRequest>,
        ) -> Result<tonic::Response<super::Room>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/GetRoom",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists rooms managed by the enterprise."]
        pub async fn list_rooms(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRoomsRequest>,
        ) -> Result<tonic::Response<super::ListRoomsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.enterprise.sdm.v1.SmartDeviceManagementService/ListRooms",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SmartDeviceManagementServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SmartDeviceManagementServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SmartDeviceManagementServiceClient {{ ... }}")
        }
    }
}
