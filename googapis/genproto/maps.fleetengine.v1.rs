/// Identifies a terminal point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerminalPointId {
    /// Unique ID of the terminal point.
    #[prost(string, tag = "4")]
    pub value: ::prost::alloc::string::String,
    /// Deprecated.
    #[prost(oneof = "terminal_point_id::Id", tags = "2, 3")]
    pub id: ::core::option::Option<terminal_point_id::Id>,
}
/// Nested message and enum types in `TerminalPointId`.
pub mod terminal_point_id {
    /// Deprecated.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        /// Deprecated.
        #[prost(string, tag = "2")]
        PlaceId(::prost::alloc::string::String),
        /// Deprecated.
        #[prost(string, tag = "3")]
        GeneratedId(::prost::alloc::string::String),
    }
}
/// Describes the location of a pickup or dropoff.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TerminalLocation {
    /// Required. Denotes the actual location of a pickup or dropoff.
    #[prost(message, optional, tag = "1")]
    pub point: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// Required. ID of the terminal point.
    #[prost(message, optional, tag = "2")]
    pub terminal_point_id: ::core::option::Option<TerminalPointId>,
    /// Deprecated.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub access_point_id: ::prost::alloc::string::String,
    /// Deprecated. Use vehicle.waypoint instead.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub trip_id: ::prost::alloc::string::String,
    /// Deprecated. Vehicle.waypoint will have this data.
    #[deprecated]
    #[prost(enumeration = "WaypointType", tag = "5")]
    pub terminal_location_type: i32,
}
/// Describes a stopping point on a vehicle's route or an ending point on a
/// vehicle's trip.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TripWaypoint {
    /// The location where this waypoint is
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<TerminalLocation>,
    /// The trip this waypoint is part of
    #[prost(string, tag = "2")]
    pub trip_id: ::prost::alloc::string::String,
    /// The type described the role the waypoint plays for this trip such as a
    /// pickup or dropoff.
    #[prost(enumeration = "WaypointType", tag = "3")]
    pub waypoint_type: i32,
    /// The path calculated by Fleet Engine from the previous waypoint to the
    /// current waypoint.
    #[prost(message, repeated, tag = "4")]
    pub path_to_waypoint: ::prost::alloc::vec::Vec<super::super::super::google::r#type::LatLng>,
    /// The path distance calculated by Fleet Engine from the previous waypoint to
    /// the current waypoint.
    /// If the current waypoint is the first waypoint in the list (Vehicle.waypoint
    /// or Trip.remaining_waypoints), then the starting point is the vehicle's
    /// location recorded at the time this TripWaypoint was added to the list.
    #[prost(message, optional, tag = "6")]
    pub distance_meters: ::core::option::Option<i32>,
    /// The arrival time to this waypoint calculated by Fleet Engine.
    #[prost(message, optional, tag = "7")]
    pub eta: ::core::option::Option<::prost_types::Timestamp>,
    /// The travel time from previous waypoint to this point.
    /// If the current waypoint is the first waypoint in the list (Vehicle.waypoint
    /// or Trip.remaining_waypoints), then the starting point is the vehicle's
    /// location recorded at the time that this waypoint was added to the list.
    /// This field is filled only when returning Trip/Vehicle data.
    #[prost(message, optional, tag = "8")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
/// The 'Status' defines a FleetEngine custom logical error mode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// The error code. It is not possible to have a value as 0 if it is explicitly
    /// set by the server.
    #[prost(enumeration = "status::Code", tag = "1")]
    pub code: i32,
    /// Detailed error message.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// A list of messages that carry the error details.  There is a common set of
    /// message types for APIs to use.
    #[prost(message, repeated, tag = "3")]
    pub details: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// Nested message and enum types in `Status`.
pub mod status {
    /// The canonical error code.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// Unspecified status, not a valid value to set.
        Unspecified = 0,
        /// Internal server error. Usually expect the client to retry in this case.
        Failure = 1,
        /// There is no possible route. Client should not retry.
        RouteNotPossible = 2,
    }
}
/// A full, human-readable address for the entity containing this message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormattedAddress {
    /// The lines of text that describe the address.
    /// At least one line must be present.
    #[prost(string, repeated, tag = "1")]
    pub lines: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Address of a place.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    /// A full, human-readable address for this place.
    #[prost(message, optional, tag = "1")]
    pub formatted_address: ::core::option::Option<FormattedAddress>,
}
/// Describes a vehicle attribute as a key-value pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleAttribute {
    /// The attribute's key. Keys may not contain the colon character (:).
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The attribute's value.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// The location, speed, and heading of a vehicle at a point in time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleLocation {
    /// The location of the vehicle.
    /// When it is sent to FleetEngine, the vehicle's location is a GPS location.
    /// When you receive it in a response, the vehicle's location can be either a
    /// GPS location or a supplemental location. The source is specified in the
    /// field 'location_sensor'.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// Deprecated. Use latlng_accuracy instead.
    #[deprecated]
    #[prost(message, optional, tag = "8")]
    pub horizontal_accuracy: ::core::option::Option<f64>,
    /// Accuracy of horizontal measurements (lat/lng) in meters as a radius.
    #[prost(message, optional, tag = "22")]
    pub latlng_accuracy: ::core::option::Option<f64>,
    /// Direction the vehicle is moving in degrees.  0 represents North.
    /// The valid range is [0,360).
    #[prost(message, optional, tag = "2")]
    pub heading: ::core::option::Option<i32>,
    /// Deprecated. Use heading_accuracy instead.
    #[deprecated]
    #[prost(message, optional, tag = "10")]
    pub bearing_accuracy: ::core::option::Option<f64>,
    /// Accuracy of heading (bearing) in degrees.
    #[prost(message, optional, tag = "23")]
    pub heading_accuracy: ::core::option::Option<f64>,
    /// Altitude in meters above WGS84.
    #[prost(message, optional, tag = "5")]
    pub altitude: ::core::option::Option<f64>,
    /// Deprecated. Use altitude_accurarcy instead.
    #[deprecated]
    #[prost(message, optional, tag = "9")]
    pub vertical_accuracy: ::core::option::Option<f64>,
    /// Accuracy of altitude measurement in meters.
    #[prost(message, optional, tag = "24")]
    pub altitude_accuracy: ::core::option::Option<f64>,
    /// Speed of the vehicle in kilometers per hour.
    /// Deprecated. Use speed instead.
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub speed_kmph: ::core::option::Option<i32>,
    /// Speed of the vehicle in meters/second
    #[prost(message, optional, tag = "6")]
    pub speed: ::core::option::Option<f64>,
    /// Accuracy of speed in meters/second.
    #[prost(message, optional, tag = "7")]
    pub speed_accuracy: ::core::option::Option<f64>,
    /// The time when the location was recorded.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the server receives the location information, filled by
    /// FleetEngine.
    #[prost(message, optional, tag = "13")]
    pub server_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Provider of location data (for example, "gps").
    #[prost(enumeration = "LocationSensor", tag = "11")]
    pub location_sensor: i32,
    /// Whether the vehicle location given by "location" field is snapped to a road
    /// closest to the location given by "raw_location".
    /// Driver SDK 1.15.1/2.1.1 and up will always set this field.
    /// Unset value will be treated as true.
    #[prost(message, optional, tag = "27")]
    pub is_road_snapped: ::core::option::Option<bool>,
    /// Input only. Indicates whether the GPS sensor is enabled.
    #[prost(message, optional, tag = "12")]
    pub is_gps_sensor_enabled: ::core::option::Option<bool>,
    /// Input only. Time (in seconds) since this location sample was first sent to the server.
    /// This will be zero for the first update. If the time is unknown
    /// (for example, when the app restarts), this value resets to zero.
    #[prost(message, optional, tag = "14")]
    pub time_since_update: ::core::option::Option<i32>,
    /// Input only. Number of additional attempts to send the current location to the server.
    /// If this value is zero, then it is not stale.
    #[prost(message, optional, tag = "15")]
    pub num_stale_updates: ::core::option::Option<i32>,
    /// Raw vehicle location (unprocessed by road-snapper).
    #[prost(message, optional, tag = "16")]
    pub raw_location: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// Input only. Timestamp associated with the raw location.
    #[prost(message, optional, tag = "17")]
    pub raw_location_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. Source of the raw location.
    #[prost(enumeration = "LocationSensor", tag = "28")]
    pub raw_location_sensor: i32,
    /// Input only. Accuracy of the raw location (lat/lng) as a radius, measured in meters.
    #[prost(message, optional, tag = "25")]
    pub raw_location_accuracy: ::core::option::Option<f64>,
    /// Input only. Supplemental location provided by the integrating app, such as the location
    /// provided by Fused Location Provider.
    #[prost(message, optional, tag = "18")]
    pub supplemental_location: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// Input only. Timestamp associated with the supplemental location.
    #[prost(message, optional, tag = "19")]
    pub supplemental_location_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Input only. Source of the supplemental location.
    #[prost(enumeration = "LocationSensor", tag = "20")]
    pub supplemental_location_sensor: i32,
    /// Input only. Accuracy of supplemental location (lat/lng) as a radius, measured in
    /// meters.
    #[prost(message, optional, tag = "21")]
    pub supplemental_location_accuracy: ::core::option::Option<f64>,
    /// Deprecated, use is_road_snapped instead.
    #[deprecated]
    #[prost(bool, tag = "26")]
    pub road_snapped: bool,
}
/// The type of a trip.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TripType {
    /// Default, used for unspecified or unrecognized trip types.
    UnknownTripType = 0,
    /// The trip may share a vehicle with other trips.
    Shared = 1,
    /// The trip is exclusive to a vehicle.
    Exclusive = 2,
}
/// The type of waypoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WaypointType {
    /// Default, unknown waypoint type
    UnknownWaypointType = 0,
    /// Waypoints for picking up customers or merchandise.
    PickupWaypointType = 1,
    /// Waypoints for dropping off customers or merchandise.
    DropOffWaypointType = 2,
    /// Waypoints for intermediate destinations in a multi-destination trip.
    IntermediateDestinationWaypointType = 3,
}
/// The type of polyline format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PolylineFormatType {
    /// Unspecified format type.
    UnknownFormatType = 0,
    /// Repeated LatLng.
    LatLngListType = 1,
    /// A polyline encoded with a polyline compression algorithm. Decoding is not
    /// yet supported.
    EncodedPolylineType = 2,
}
/// A set of values that specify the vehicle's navigation status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NavigationStatus {
    /// Unspecified navigation status.
    UnknownNavigationStatus = 0,
    /// The Driver app's navigation is in FREE_NAV mode.
    NoGuidance = 1,
    /// Turn-by-turn navigation starts and the Driver app navigation enters
    /// GUIDED_NAV mode showing the green header, route, and destination marker.
    EnrouteToDestination = 2,
    /// Vehicle has gone off the suggested route.
    OffRoute = 3,
    /// The vehicle is within 50m of the destination and onArrival was
    /// automatically triggered.
    ArrivedAtDestination = 4,
}
/// Possible location providers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocationSensor {
    /// Undefined sensor.
    UnknownSensor = 0,
    /// Sensors: (GPS, AGPS).
    Gps = 1,
    /// Sensors: (AGPS, CellID, WiFi MACID).
    Network = 2,
    /// Sensors: (CellID, WiFi MACID).
    Passive = 3,
    /// GMM's road snapped (gmfc) location.
    RoadSnappedLocationProvider = 4,
    /// Unspecified, but generated by the Fused Location Provider.
    FusedLocationProvider = 100,
}
/// A RequestHeader contains fields common to all Fleet Engine RPC requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestHeader {
    /// The language requested. The external form of Google International
    /// Identifiers Initiative (III) LanguageCode objects. If none is specified,
    /// return a name in any language, with a preference for English if such a
    /// name exists.
    #[prost(string, tag = "1")]
    pub language_code: ::prost::alloc::string::String,
    /// Required. CLDR region code of the region where the request originates.
    #[prost(string, tag = "2")]
    pub region_code: ::prost::alloc::string::String,
    /// Version of the calling SDK, if applicable.
    #[prost(string, tag = "3")]
    pub sdk_version: ::prost::alloc::string::String,
    /// Version of the operating system on which the calling SDK is running.
    #[prost(string, tag = "4")]
    pub os_version: ::prost::alloc::string::String,
    /// Model of the device on which the calling SDK is running.
    #[prost(string, tag = "5")]
    pub device_model: ::prost::alloc::string::String,
}
/// Trip metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trip {
    /// In the format "providers/{provider}/trips/{trip}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// ID of the vehicle making this trip.
    #[prost(string, tag = "2")]
    pub vehicle_id: ::prost::alloc::string::String,
    /// Current status of the trip.
    #[prost(enumeration = "TripStatus", tag = "3")]
    pub trip_status: i32,
    /// The type of the trip.
    #[prost(enumeration = "TripType", tag = "4")]
    pub trip_type: i32,
    /// Location where customer indicates they will be picked up.
    #[prost(message, optional, tag = "5")]
    pub pickup_point: ::core::option::Option<TerminalLocation>,
    /// Input only. The actual location when and where customer was picked up.
    /// This field is for provider to provide feedback on actual pickup
    /// information.
    #[prost(message, optional, tag = "22")]
    pub actual_pickup_point: ::core::option::Option<StopLocation>,
    /// Input only. The actual time and location of the driver arrival at
    /// the pickup point.
    /// This field is for provider to provide feedback on actual arrival
    /// information at the pickup point.
    #[prost(message, optional, tag = "32")]
    pub actual_pickup_arrival_point: ::core::option::Option<StopLocation>,
    /// Either the estimated future time when the rider(s) will be picked up, or
    /// the actual time when they were picked up.
    #[prost(message, optional, tag = "6")]
    pub pickup_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Intermediate stops in order that the trip requests (in addition
    /// to pickup and dropoff). Initially this will not be supported for shared
    /// trips.
    #[prost(message, repeated, tag = "14")]
    pub intermediate_destinations: ::prost::alloc::vec::Vec<TerminalLocation>,
    /// Indicates the last time the Trip.intermediate_destinations was modified.
    /// Your server should cache this value and pass it in UpdateTripRequest
    /// when update Trip.intermediate_destination_index to ensure the
    /// Trip.intermediate_destinations is not changed.
    #[prost(message, optional, tag = "25")]
    pub intermediate_destinations_version: ::core::option::Option<::prost_types::Timestamp>,
    /// When TripStatus is ENROUTE_TO_INTERMEDIATE_DESTINATION, a number between
    /// \[0..N-1\] indicating which intermediate destination the vehicle will cross
    /// next.
    /// When TripStatus is ARRIVED_AT_INTERMEDIATE_DESTINATION, a number between
    /// \[0..N-1\] indicating which intermediate destination the vehicle is at.
    /// The provider sets this value. If there are no intermediate_destinations,
    /// this field is ignored.
    #[prost(int32, tag = "15")]
    pub intermediate_destination_index: i32,
    /// Input only. The actual time and location of the driver's arrival at
    /// an intermediate destination.
    /// This field is for provider to provide feedback on actual arriaval
    /// information at intermediate destinations.
    #[prost(message, repeated, tag = "33")]
    pub actual_intermediate_destination_arrival_points: ::prost::alloc::vec::Vec<StopLocation>,
    /// Input only. The actual time and location when and where the customer was picked up from
    /// an intermediate destination.
    /// This field is for provider to provide feedback on actual pickup
    /// information at intermediate destinations.
    #[prost(message, repeated, tag = "34")]
    pub actual_intermediate_destinations: ::prost::alloc::vec::Vec<StopLocation>,
    /// Location where customer indicates they will be dropped off.
    #[prost(message, optional, tag = "7")]
    pub dropoff_point: ::core::option::Option<TerminalLocation>,
    /// Input only. The actual time and location when and where customer was dropped off.
    /// This field is for provider to provide feedback on actual dropoff
    /// information.
    #[prost(message, optional, tag = "23")]
    pub actual_dropoff_point: ::core::option::Option<StopLocation>,
    /// Either the estimated future time when the rider(s) will be dropped off at
    /// the final destination, or the actual time when they were dropped off.
    #[prost(message, optional, tag = "8")]
    pub dropoff_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The full path from the current location to the dropoff point, inclusive.
    /// If this is a shared ride, this path could include waypoints from other
    /// trips.
    #[prost(message, repeated, tag = "16")]
    pub remaining_waypoints: ::prost::alloc::vec::Vec<TripWaypoint>,
    /// This field supports manual ordering of the waypoints for the trip.
    /// It contains all of the remaining waypoints of vehicle assigned, as well as
    /// the pickup and drop-off waypoints for this trip.
    /// If the trip hasn't been assigned to a vehicle, then this field is ignored.
    /// For privacy reasons, this field is only populated by the server on
    /// UpdateTrip and CreateTrip calls, and NOT on GetTrip calls.
    #[prost(message, repeated, tag = "20")]
    pub vehicle_waypoints: ::prost::alloc::vec::Vec<TripWaypoint>,
    /// Anticipated route for this trip to the first entry in remaining_waypoints.
    /// If back_to_back or shared trips are enabled, the waypoint may belong to a
    /// different trip.
    #[prost(message, repeated, tag = "9")]
    pub route: ::prost::alloc::vec::Vec<super::super::super::google::r#type::LatLng>,
    /// The waypoint where current_route_segment ends. This can be supplied by
    /// drivers on UpdateVehicle calls either as a full trip waypoint, a waypoint
    /// latlng, or as a the last latlng of the current_route_segment. FleetEngine
    /// will then do its best to interpolate to an actual waypoint if it is not
    /// fully specified. It will be returned in GetTrip calls. It is not respected
    /// in Create/Update Trip calls.
    #[prost(message, optional, tag = "24")]
    pub current_route_segment_end_point: ::core::option::Option<TripWaypoint>,
    /// The remaining driving distance in Trip.current_route_segment field.
    /// This field facilitates journey sharing between a driver and rider and
    /// Fleet Engine does not update it. Your driver app is responsible for setting
    /// field on all of its current trips by passing
    /// Vehicle.remaining_distance_meters to an Vehicle.update call.
    /// The value is unspecified if the trip is not assigned to a vehicle, or the
    /// trip is inactive (completed or cancelled), or driver hasn't updated this
    /// value.
    #[prost(message, optional, tag = "12")]
    pub remaining_distance_meters: ::core::option::Option<i32>,
    /// Output only. The ETA to the next waypoint (the first entry in the
    /// Trip.remaining_waypoints field). This field facilitates journey sharing
    /// between a driver and a consumer. Fleet Engine does not update this value.
    /// Your driver app is responsible for setting this field by passing
    /// Vehicle.remaining_time_seconds in a call to Vehicle.update. FleetEngine
    /// converts the Vehicle.remaining_time_seconds to Trip.eta_to_first_waypoint,
    /// and returns it to the rider. The value is unspecified if the trip is not
    /// assigned to a vehicle, or the trip is inactive (completed or cancelled), or
    /// driver hasn't updated this value.
    #[prost(message, optional, tag = "13")]
    pub eta_to_first_waypoint: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The duration from when the Trip data is returned to the time in
    /// Trip.eta_to_first_waypoint.
    #[prost(message, optional, tag = "27")]
    pub remaining_time_to_first_waypoint: ::core::option::Option<::prost_types::Duration>,
    /// Indicates the last time that `remaining_waypoints` was changed (a
    /// waypoint was added, removed, or changed).
    #[prost(message, optional, tag = "19")]
    pub remaining_waypoints_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates the last time the remaining_waypoints.path_to_waypoint and
    /// remaining_waypoints.traffic_to_waypoint were modified. Your client app
    /// should cache this value and pass it in GetTripRequest to ensure the
    /// paths and traffic for remaining_waypoints are only returned if updated.
    #[prost(message, optional, tag = "29")]
    pub remaining_waypoints_route_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates the number of passengers on this trip and does not include the
    /// driver. A vehicle must have available_capacity to be returned
    /// in SearchTrips.
    #[prost(int32, tag = "10")]
    pub number_of_passengers: i32,
    /// Indicates the last reported location of the vehicle along the route.
    #[prost(message, optional, tag = "11")]
    pub last_location: ::core::option::Option<VehicleLocation>,
    /// Indicates whether the vehicle's last_location can be snapped to
    /// the current_route_segment. False if last_location or current_route_segment
    /// doesn't exist.
    /// It is computed by Fleet Engine. Any update from clients will be ignored.
    #[prost(bool, tag = "26")]
    pub last_location_snappable: bool,
    /// The subset of Trip fields that are populated and how they should be
    /// interpreted.
    #[prost(enumeration = "TripView", tag = "31")]
    pub view: i32,
}
/// The actual location where a stop (pickup/dropoff) happened.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopLocation {
    /// Required. Denotes the actual location.
    #[prost(message, optional, tag = "1")]
    pub point: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// The timestamp when the location was measured.
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates when the stop actually happened.
    #[prost(message, optional, tag = "3")]
    pub stop_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The status of a trip indicating its progression.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TripStatus {
    /// Default, used for unspecified or unrecognized trip status.
    UnknownTripStatus = 0,
    /// Newly created trip.
    New = 1,
    /// The driver is on their way to the pickup point.
    EnrouteToPickup = 2,
    /// The driver has arrived at the pickup point.
    ArrivedAtPickup = 3,
    /// The driver has arrived at an intermediate destination and is waiting for
    /// the rider.
    ArrivedAtIntermediateDestination = 7,
    /// The driver is on their way to an intermediate destination
    /// (not the dropoff point).
    EnrouteToIntermediateDestination = 8,
    /// The driver has picked up the rider and is on their way to the
    /// next destination.
    EnrouteToDropoff = 4,
    /// The rider has been dropped off and the trip is complete.
    Complete = 5,
    /// The trip was canceled prior to pickup by the driver, rider, or
    /// rideshare provider.
    Canceled = 6,
}
/// A set of values that indicate upon which platform the request was issued.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BillingPlatformIdentifier {
    /// Default. Used for unspecified platforms.
    Unspecified = 0,
    /// The platform is a client server.
    Server = 1,
    /// The platform is a web browser.
    Web = 2,
    /// The platform is an Android mobile device.
    Android = 3,
    /// The platform is an IOS mobile device.
    Ios = 4,
    /// Other platforms that are not listed in this enumeration.
    Others = 5,
}
/// Selector for different sets of Trip fields in a `GetTrip` response.  See
/// \[AIP-157\](<https://google.aip.dev/157>) for context. Additional views are
/// likely to be added.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TripView {
    /// The default value. For backwards-compatibility, the API will default to an
    /// SDK view. To ensure stability and support, customers are
    /// advised to select a `TripView` other than `SDK`.
    Unspecified = 0,
    /// Includes fields that may not be interpretable or supportable using
    /// publicly available libraries.
    Sdk = 1,
    /// Trip fields are populated for the Journey Sharing use case. This view is
    /// intended for server-to-server communications.
    JourneySharingV1s = 2,
}
/// CreateTrip request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTripRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format "providers/{provider}".
    /// The provider must be the Project ID (for example, sample-cloud-project)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique Trip ID; must be unique per provider.  The actual
    /// format and value is opaque to the Fleet Engine and is determined
    /// by the provider.
    #[prost(string, tag = "5")]
    pub trip_id: ::prost::alloc::string::String,
    /// Required. Trip entity to create.
    ///
    /// When creating a Trip, the following fields are required:
    ///
    /// * trip_type
    /// * pickup_point
    ///
    /// The following fields are used if you provide them:
    ///
    /// * number_of_passengers
    /// * vehicle_id
    /// * dropoff_point
    /// * intermediate_destinations
    ///
    /// Only EXCLUSIVE trips support multiple destinations.
    ///
    /// When vehicle_id is set for a shared trip, you must supply
    /// the list of `Trip.vehicle_waypoints` to specify the order of the remaining
    /// waypoints for the vehicle, otherwise the waypoint order will be
    /// undetermined.
    ///
    /// When you specify `Trip.vehicle_waypoints`, the list must contain all
    /// the remaining waypoints of the vehicle's trips, with no extra waypoints.
    /// You must order these waypoints such that for a given trip, the pickup
    /// point is before intermediate destinations, and all intermediate
    /// destinations come before the drop-off point. An `EXCLUSIVE` trip's
    /// waypoints must not interleave with any other trips.
    ///
    /// The `trip_id`, `waypoint_type` and `location` fields are used, and all
    /// other TripWaypoint fields in vehicle_waypoints are ignored.
    ///
    /// All other Trip fields are ignored.
    #[prost(message, optional, tag = "4")]
    pub trip: ::core::option::Option<Trip>,
}
/// GetTrip request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTripRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format "providers/{provider}/trips/{trip}".
    /// The provider must be the Project ID (for example, sample-cloud-project)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// The subset of Trip fields that should be returned and their interpretation.
    #[prost(enumeration = "TripView", tag = "11")]
    pub view: i32,
    /// Indicates the minimum timestamp (exclusive) for which Trip.route or
    /// Trip.current_route_segment data is retrieved. If route data is unchanged
    /// since this timestamp, the route field is not set in the response. If a
    /// minimum is unspecified, the route data is always retrieved.
    #[prost(message, optional, tag = "6")]
    pub current_route_segment_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates the minimum timestamp (exclusive) for which
    /// Trip.remaining_waypoints are retrieved. If they are unchanged since this
    /// timestamp, the remaining_waypoints are not set in the response. If this
    /// field is unspecified, remaining_waypoints is always retrieved.
    #[prost(message, optional, tag = "7")]
    pub remaining_waypoints_version: ::core::option::Option<::prost_types::Timestamp>,
    /// The returned current route format, LAT_LNG_LIST_TYPE (in Trip.route), or
    /// ENCODED_POLYLINE_TYPE (in Trip.current_route_segment).
    /// The default is LAT_LNG_LIST_TYPE.
    #[prost(enumeration = "PolylineFormatType", tag = "8")]
    pub route_format_type: i32,
}
/// ReportBillableTrip request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportBillableTripRequest {
    /// Required. Must be in the format
    /// "providers/{provider}/billableTrips/{billable_trip}". The
    /// provider must be the Project ID (for example, sample-cloud-project) of the
    /// Google Cloud Project of which the service account making this call is a
    /// member.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Required. Two letter country code of the country where the trip takes place. Price is
    /// defined according to country code.
    #[prost(string, tag = "3")]
    pub country_code: ::prost::alloc::string::String,
    /// The platform upon which the request was issued.
    #[prost(enumeration = "BillingPlatformIdentifier", tag = "5")]
    pub platform: i32,
    /// The identifiers that are directly related to the trip being reported. These
    /// are usually IDs (for example, session IDs) of pre-booking operations done
    /// before the trip ID is available. The number of related_ids is
    /// limited to 50.
    #[prost(string, repeated, tag = "6")]
    pub related_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The type of GMP product solution (for example,
    /// ON_DEMAND_RIDESHARING_AND_DELIVERIES) used for the reported trip.
    #[prost(enumeration = "report_billable_trip_request::SolutionType", tag = "7")]
    pub solution_type: i32,
}
/// Nested message and enum types in `ReportBillableTripRequest`.
pub mod report_billable_trip_request {
    /// Selector for different solution types of a reported trip.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SolutionType {
        /// The default value. For backwards-compatibility, the API will use
        /// ON_DEMAND_RIDESHARING_AND_DELIVERIES by default which is the first
        /// supported solution type.
        Unspecified = 0,
        /// The solution is an on-demand ridesharing and deliveries trip.
        OnDemandRidesharingAndDeliveries = 1,
    }
}
/// UpdateTrip request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTripRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format
    /// `providers/{provider}/trips/{trip}`. The provider must
    /// be the Project ID (for example, sample-consumer-project) of the Google
    /// Cloud Project of which the service account making this call is a member.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The Trip associated with the update.
    ///
    /// The following fields are maintained by the Fleet Engine. Do not update
    /// them using Trip.update.
    ///
    /// * current_route_segment
    /// * current_route_segment_version
    /// * eta_to_next_waypoint
    /// * intermediate_destinations_version
    /// * last_location
    /// * name
    /// * number_of_passengers
    /// * remaining_distance_meters
    /// * remaining_time_to_first_waypoint
    /// * remaining_waypoints
    /// * remaining_waypoints_version
    ///
    /// When you update the `Trip.vehicle_id` for a shared trip, you must supply
    /// the list of `Trip.vehicle_waypoints` to specify the order of the remaining
    /// waypoints, otherwise the order will be undetermined.
    ///
    /// When you specify `Trip.vehicle_waypoints`, the list must contain all
    /// the remaining waypoints of the vehicle's trips, with no extra waypoints.
    /// You must order these waypoints such that for a given trip, the pickup
    /// point is before intermediate destinations, and all intermediate
    /// destinations come before the drop-off point. An `EXCLUSIVE` trip's
    /// waypoints must not interleave with any other trips.
    /// The `trip_id`, `waypoint_type` and `location` fields are used, and all
    /// other TripWaypoint fields in vehicle_waypoints are ignored.
    ///
    /// To avoid a race condition for trips with multiple destinations, you
    /// should provide `Trip.intermediate_destinations_version` when updating
    /// the trip status to `ENROUTE_TO_INTERMEDIATE_DESTINATION`. The
    /// `Trip.intermediate_destinations_version` passed must be consistent with
    /// Fleet Engine's version. If it isn't, the request fails.
    #[prost(message, optional, tag = "4")]
    pub trip: ::core::option::Option<Trip>,
    /// Required. The field mask indicating which fields in Trip to update.
    /// The update_mask must contain at least one field.
    #[prost(message, optional, tag = "5")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// SearchTrips request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTripsRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format "providers/*"
    /// The provider must be the Project ID (for example, sample-cloud-project)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// The vehicle associated with the trips in the request. If unspecified, the
    /// returned trips do not contain:
    ///
    /// * current_route_segment
    /// * remaining_waypoints
    /// * remaining_distance_meters
    /// * eta_to_first_waypoint
    #[prost(string, tag = "4")]
    pub vehicle_id: ::prost::alloc::string::String,
    /// If set to true, only Trips that influence the drivers route
    /// are included in the response.
    #[prost(bool, tag = "5")]
    pub active_trips_only: bool,
    /// If not set, the server will decide the number of
    /// results to return.
    #[prost(int32, tag = "6")]
    pub page_size: i32,
    /// Set this to a value previously returned in the
    /// SearchTripsResponse to continue from previous results.
    #[prost(string, tag = "7")]
    pub page_token: ::prost::alloc::string::String,
    /// If specified, returns the trips that have not been updated after
    /// the time (current - minimum_staleness).
    #[prost(message, optional, tag = "8")]
    pub minimum_staleness: ::core::option::Option<::prost_types::Duration>,
}
/// SearchTrips response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTripsResponse {
    /// The list of trips for the requested vehicle.
    #[prost(message, repeated, tag = "1")]
    pub trips: ::prost::alloc::vec::Vec<Trip>,
    /// Pass this token in the SearchTripsRequest to continue to
    /// list results. If all results have been returned, this field is an empty
    /// string or not present in the response.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod trip_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Trip management service."]
    #[derive(Debug, Clone)]
    pub struct TripServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TripServiceClient<T>
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
        ) -> TripServiceClient<InterceptedService<T, F>>
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
            TripServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a trip in the Fleet Engine and returns the new trip."]
        pub async fn create_trip(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTripRequest>,
        ) -> Result<tonic::Response<super::Trip>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/maps.fleetengine.v1.TripService/CreateTrip");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get information about a single trip."]
        pub async fn get_trip(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTripRequest>,
        ) -> Result<tonic::Response<super::Trip>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/maps.fleetengine.v1.TripService/GetTrip");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Report billable trip usage."]
        pub async fn report_billable_trip(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportBillableTripRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.TripService/ReportBillableTrip",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get all the trips for a specific vehicle."]
        pub async fn search_trips(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchTripsRequest>,
        ) -> Result<tonic::Response<super::SearchTripsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.TripService/SearchTrips",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates trip data."]
        pub async fn update_trip(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTripRequest>,
        ) -> Result<tonic::Response<super::Trip>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/maps.fleetengine.v1.TripService/UpdateTrip");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Vehicle metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vehicle {
    /// The unique name for this vehicle.
    /// The format is providers/{provider}/vehicles/{vehicle}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The vehicle state.
    #[prost(enumeration = "VehicleState", tag = "2")]
    pub vehicle_state: i32,
    /// Supported trip types.
    #[prost(enumeration = "TripType", repeated, tag = "3")]
    pub supported_trip_types: ::prost::alloc::vec::Vec<i32>,
    /// List of IDs for trips in progress.
    #[prost(string, repeated, tag = "4")]
    pub current_trips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Last reported location of the vehicle.
    #[prost(message, optional, tag = "5")]
    pub last_location: ::core::option::Option<VehicleLocation>,
    /// Maximum capacity of the vehicle.  This is the total numbers of riders
    /// on trips this vehicle can contain.  The driver is not considered in
    /// this value.  This value must be greater than or equal to one.
    #[prost(int32, tag = "6")]
    pub maximum_capacity: i32,
    /// The current available capacity of the vehicle.  This is the
    /// maximum_capacity minus the current number of riders.
    #[prost(int32, tag = "7")]
    pub available_capacity: i32,
    /// List of vehicle service attributes.
    #[prost(message, repeated, tag = "8")]
    pub attributes: ::prost::alloc::vec::Vec<VehicleAttribute>,
    /// The type of this Vehicle.  Can be filtered during SearchVehicles.  Also
    /// influences ETA and route calculations.
    #[prost(message, optional, tag = "9")]
    pub vehicle_type: ::core::option::Option<vehicle::VehicleType>,
    /// License plate information for the vehicle.
    #[prost(message, optional, tag = "10")]
    pub license_plate: ::core::option::Option<LicensePlate>,
    /// Deprecated. Use vehicle.waypoint instead.
    #[deprecated]
    #[prost(message, repeated, tag = "12")]
    pub route: ::prost::alloc::vec::Vec<TerminalLocation>,
    /// The polyline specifying the route the driver app intends to take to
    /// the next waypoint. Your driver app updates this every time a waypoint is
    /// passed or the driver reroutes. This list is also returned in
    /// Trip.current_route_segment for all active trips assigned to the vehicle.
    /// Note: This field is intended only for use by the Driver SDK.
    #[prost(string, tag = "20")]
    pub current_route_segment: ::prost::alloc::string::String,
    /// Time when current_route_segment was set. This field is ignored in
    /// UpdateVehicleRequests as it is calculated by the server. It should be
    /// stored by client and passed in to future requests to prevent returning
    /// routes to first way point that haven't changed.
    #[prost(message, optional, tag = "15")]
    pub current_route_segment_version: ::core::option::Option<::prost_types::Timestamp>,
    /// The waypoint where current_route_segment ends. This can be supplied by
    /// drivers on UpdateVehicle calls either as a full trip waypoint, a waypoint
    /// latlnt, or as a the last latlng of the current_route_segment. FleetEngine
    /// will then do its best to interpolate to an actual waypoint if it is not
    /// fully specified. This field is ignored in UpdateVehicle calls unless
    /// current_route_segment is also specified.
    #[prost(message, optional, tag = "24")]
    pub current_route_segment_end_point: ::core::option::Option<TripWaypoint>,
    /// The remaining driving distance for the 'current_route_segment'. This field
    /// facilitates journey sharing between the Driver app and the Consumer app.
    /// This value is updated by the Driver SDK. Fleet Engine does not update it.
    /// This field is also returned in Trip.remaining_distance_meters for all
    /// active trips assigned to the vehicle. The value is unspecified if the
    /// `Vehicle.current_route_segment` field is empty, or if the Driver app has
    /// not updated its value.
    #[prost(message, optional, tag = "18")]
    pub remaining_distance_meters: ::core::option::Option<i32>,
    /// The ETA to the next waypoint that is the first entry in Vehicle.waypoint
    /// field. This field facilitates journey sharing between a Driver app and a
    /// Consumer app and is updated by the Driver SDK, and Fleet Engine does not
    /// update it. This field is also returned in Trip.eta_to_first_waypoint for
    /// all active trips assigned to the vehicle. The value is unspecified if the
    /// Vehicle.waypoint field is empty, or the Driver app has not updated its
    /// value.
    #[prost(message, optional, tag = "19")]
    pub eta_to_first_waypoint: ::core::option::Option<::prost_types::Timestamp>,
    /// The remaining driving time for the 'current_route_segment'. This field
    /// facilitates journey sharing between the Driver app and the Consumer app.
    /// This value is updated by the Driver SDK. Fleet Engine does not update it.
    /// The value is unspecified if the `Vehicle.current_route_segment` field is
    /// empty, or if the Driver app has not updated its value. This value should
    /// match eta_to_first_waypoint - current_time if all parties are using the
    /// same clock. This field is currently write-only and will not yet be
    /// populated in Vehicle's get/update/search operations. When updating a
    /// vehicle, if you update both eta_to_first_waypoint and
    /// remaining_time_seconds in the same request, then only
    /// remaining_time_seconds is considered.
    #[prost(message, optional, tag = "25")]
    pub remaining_time_seconds: ::core::option::Option<i32>,
    /// The remaining set of waypoints assigned to this Vehicle.
    #[prost(message, repeated, tag = "22")]
    pub waypoints: ::prost::alloc::vec::Vec<TripWaypoint>,
    /// Last time the waypoints was updated. Client should cache
    /// this value and pass it in GetVehicleRequest to ensure the
    /// waypoints.path_to_waypoint is only returned if it is updated
    #[prost(message, optional, tag = "16")]
    pub waypoints_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates if the driver accepts back-to-back rides. If
    /// `true`, services include the vehicle for back-to-back matches.
    /// If `false`, services exclude the vehicle from back-to-back matches.
    /// Default value is `false`.
    #[prost(bool, tag = "23")]
    pub back_to_back_enabled: bool,
    /// Vehicle's navigation status.
    #[prost(enumeration = "NavigationStatus", tag = "26")]
    pub navigation_status: i32,
    /// Information about various device settings. This is internal debug only
    /// field, not included in the response.
    #[prost(message, optional, tag = "27")]
    pub device_settings: ::core::option::Option<DeviceSettings>,
}
/// Nested message and enum types in `Vehicle`.
pub mod vehicle {
    /// Types of vehicles that may be filtered for in SearchVehicles.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VehicleType {
        /// Vehicle type category
        #[prost(enumeration = "vehicle_type::Category", tag = "1")]
        pub category: i32,
    }
    /// Nested message and enum types in `VehicleType`.
    pub mod vehicle_type {
        /// Vehicle type categories
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Category {
            /// Default, used for unspecified or unrecognized vehicle types.
            Unknown = 0,
            /// An automobile.
            Auto = 1,
            /// Any vehicle that acts as a taxi.
            Taxi = 2,
            /// Generally, a vehicle with a large storage capacity.
            Truck = 3,
            /// A motorcycle, moped, or other two-wheeled vehicle
            TwoWheeler = 4,
        }
    }
}
/// Information about the device's battery.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatteryInfo {
    /// Status of the battery, whether full or charging etc.
    #[prost(enumeration = "BatteryStatus", tag = "1")]
    pub battery_status: i32,
    /// Status of battery power source.
    #[prost(enumeration = "PowerSource", tag = "2")]
    pub power_source: i32,
    /// Current battery percentage \[0-100\].
    #[prost(float, tag = "3")]
    pub battery_percentage: f32,
}
/// Information about various settings on the device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceSettings {
    /// How location features are set to behave on the device when battery saver is
    /// on.
    #[prost(enumeration = "LocationPowerSaveMode", tag = "1")]
    pub location_power_save_mode: i32,
    /// Whether the device is currently in power save mode.
    #[prost(bool, tag = "2")]
    pub is_power_save_mode: bool,
    /// Whether the device is in an interactive state.
    #[prost(bool, tag = "3")]
    pub is_interactive: bool,
    /// Information about the battery state.
    #[prost(message, optional, tag = "4")]
    pub battery_info: ::core::option::Option<BatteryInfo>,
}
/// The license plate information of the Vehicle.  This is used to support
/// congestion pricing restrictions in certain areas.  To avoid storing
/// personally-identifiable information, only the minimum information
/// about the license plate is stored as part of the entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LicensePlate {
    /// Required. CLDR Country/Region Code.  For example, "US" for United States,
    /// or "IN" for India.
    #[prost(string, tag = "1")]
    pub country_code: ::prost::alloc::string::String,
    /// The last digit of the license plate or "-1" to denote no numeric value
    /// present in the license plate.
    ///
    /// * "ABC 1234" -> "4"
    /// * "AB 123 CD" -> "3"
    /// * "ABCDEF" -> "-1"
    #[prost(string, tag = "2")]
    pub last_character: ::prost::alloc::string::String,
}
/// The state of a Vehicle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VehicleState {
    /// Default, used for unspecified or unrecognized vehicle states.
    UnknownVehicleState = 0,
    /// The vehicle is not accepting new trips.
    Offline = 1,
    /// The vehicle is accepting new trips.
    Online = 2,
}
/// How location features are set to behave on the device when battery saver is
/// on.
/// (<https://developer.android.com/reference/android/os/PowerManager#getLocationPowerSaveMode(>))
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LocationPowerSaveMode {
    /// Undefined LocationPowerSaveMode
    UnknownLocationPowerSaveMode = 0,
    /// Either the location providers shouldn't be affected by battery saver, or
    /// battery saver is off.
    LocationModeNoChange = 1,
    /// The GPS based location provider should be disabled when battery saver is on
    /// and the device is non-interactive.
    LocationModeGpsDisabledWhenScreenOff = 2,
    /// All location providers should be disabled when battery saver is on and the
    /// device is non-interactive.
    LocationModeAllDisabledWhenScreenOff = 3,
    /// All the location providers will be kept available, but location fixes
    /// should only be provided to foreground apps.
    LocationModeForegroundOnly = 4,
    /// Location will not be turned off, but LocationManager will throttle all
    /// requests to providers when the device is non-interactive.
    LocationModeThrottleRequestsWhenScreenOff = 5,
}
/// Status of the battery, whether full or charging etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BatteryStatus {
    /// Battery status unknown.
    UnknownBatteryStatus = 0,
    /// Battery is being charged.
    Charging = 1,
    /// Battery is discharging.
    Discharging = 2,
    /// Battery is full.
    Full = 3,
    /// Battery is not charging.
    NotCharging = 4,
    /// Battery is low on power.
    PowerLow = 5,
}
/// Type of the charger being used to charge the battery.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PowerSource {
    /// Power source unknown.
    UnknownPowerSource = 0,
    /// Power source is an AC charger.
    Ac = 1,
    /// Power source is a USB port.
    Usb = 2,
    /// Power source is wireless.
    Wireless = 3,
    /// Battery is unplugged.
    Unplugged = 4,
}
/// CreateVehicle request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVehicleRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format "providers/{provider}".
    /// The provider must be the Project ID (for example, sample-cloud-project)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Unique Vehicle ID; must be unique per provider.  The actual
    /// format and value is opaque to the Fleet Engine and is determined
    /// by the provider.
    #[prost(string, tag = "4")]
    pub vehicle_id: ::prost::alloc::string::String,
    /// Required. The Vehicle entity to create. When creating a Vehicle, the following
    /// fields are required:
    ///
    /// * vehicle_state
    /// * supported_trip_types
    /// * maximum_capacity
    /// * vehicle_type
    ///
    /// When creating a Vehicle, the following fields are ignored:
    ///
    /// * name
    /// * current_trips
    /// * available_capacity
    /// * current_route_segment
    /// * current_route_segment_version
    /// * waypoints
    /// * waypoints_version
    /// * remaining_distance_meters
    /// * eta_to_next_waypoint
    /// * navigation_status
    ///
    /// All other fields will be used if provided.
    #[prost(message, optional, tag = "5")]
    pub vehicle: ::core::option::Option<Vehicle>,
}
/// GetVehicle request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVehicleRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format
    /// "providers/{provider}/vehicles/{vehicle}".
    /// The provider must be the Project ID (for example, sample-cloud-project)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Indicates the minimum timestamp (exclusive) for which
    /// vehicle.current_route_segment is retrieved.
    /// If route is unchanged since this timestamp, the current_route_segment
    /// field is not set in the response. If a minimum is unspecified, the
    /// current_route_segment is always retrieved.
    #[prost(message, optional, tag = "4")]
    pub current_route_segment_version: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates the minimum timestamp (exclusive) for which vehicle.waypoints
    /// data is retrieved. If data is unchanged since this timestamp, the
    /// vehicle.waypoints data is not set in the response. If this field is
    /// unspecified, vehicle.waypoints is always retrieved.
    #[prost(message, optional, tag = "5")]
    pub waypoints_version: ::core::option::Option<::prost_types::Timestamp>,
}
/// UpdateVehicle request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVehicleRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format
    /// "providers/{provider}/vehicles/{vehicle}".
    /// The {provider} must be the Project ID (for example, sample-cloud-project)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    ///
    /// Note that if the name is also specified in the name field of the
    /// vehicle and name is set in the update_mask, both names must be the
    /// same.  Otherwise it is an Error.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The Vehicle entity update to apply.  When updating a Vehicle,
    /// the following fields may not be updated as they are managed by the
    /// Fleet Engine.
    ///   current_trips
    ///   available_capacity
    ///   current_route_segment_version
    ///   waypoints_version
    /// Furthermore, the name of the vehicle cannot be updated.
    #[prost(message, optional, tag = "4")]
    pub vehicle: ::core::option::Option<Vehicle>,
    /// Required. A field mask indicating which fields of the Vehicle to update.
    /// The update_mask must contain at least one field.
    #[prost(message, optional, tag = "5")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// UpdateVehicleLocation request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVehicleLocationRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format
    /// "providers/{provider}/vehicles/{vehicle}.
    /// The {provider} must be the Project ID (for example, sample-cloud-project)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The location to update to.  The last_location and update_time
    /// subfields are required.
    #[prost(message, optional, tag = "4")]
    pub current_location: ::core::option::Option<VehicleLocation>,
    /// Set current vehicle state to either ONLINE or OFFLINE;
    /// if set to UNKNOWN_VEHICLE_STATE, vehicle state will not be altered.
    #[prost(enumeration = "VehicleState", tag = "5")]
    pub current_state: i32,
}
/// UpdateVehicleAttributes request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVehicleAttributesRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format
    /// "providers/{provider}/vehicles/{vehicle}.
    /// The provider must be the Project ID (for example, sample-cloud-project)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Required. The attributes to update;
    /// unmentioned attributes will not be altered or removed.
    /// At most 20 attributes; the combined "key:value" string length cannot
    /// exceed 256.
    #[prost(message, repeated, tag = "4")]
    pub attributes: ::prost::alloc::vec::Vec<VehicleAttribute>,
}
/// UpdateVehicleAttributes response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVehicleAttributesResponse {
    /// Required. The updated full list of vehicle attributes, including new,
    /// altered and untouched attributes.
    #[prost(message, repeated, tag = "1")]
    pub attributes: ::prost::alloc::vec::Vec<VehicleAttribute>,
}
/// SearchVehicles request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVehiclesRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format "providers/{provider}".
    /// The provider must be the Project ID (for example, sample-cloud-project)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The pickup point to search near.
    #[prost(message, optional, tag = "4")]
    pub pickup_point: ::core::option::Option<TerminalLocation>,
    /// The customer's intended dropoff location. The field is required if
    /// trip_types contains TripType.SHARED.
    #[prost(message, optional, tag = "5")]
    pub dropoff_point: ::core::option::Option<TerminalLocation>,
    /// Required. Defines the vehicle search radius around the pickup point. Only
    /// vehicles within the search radius will be returned. Value must be between
    /// 400 and 10000 meters.
    #[prost(int32, tag = "6")]
    pub pickup_radius_meters: i32,
    /// Required. Specifies the maximum number of available vehicles to return. By
    /// default, the Fleet Engine limits the number to  50.
    #[prost(int32, tag = "7")]
    pub count: i32,
    /// Required. Specifies the minimum number of passengers allowed in the
    /// vehicle. Must number must be greater than or equal to one. The driver is
    /// not considered in the capacity search. This number indicates the number of
    /// passengers being considered for a trip.
    #[prost(int32, tag = "8")]
    pub minimum_capacity: i32,
    /// Required. Restricts the search to only those vehicles that support at least
    /// one of the specified trip types.
    #[prost(enumeration = "TripType", repeated, packed = "false", tag = "9")]
    pub trip_types: ::prost::alloc::vec::Vec<i32>,
    /// Restricts the search to only those vehicles that have updated their
    /// locations within the specified duration back from now. If this field is not
    /// set, the server uses five minutes as the default value.
    #[prost(message, optional, tag = "10")]
    pub maximum_staleness: ::core::option::Option<::prost_types::Duration>,
    /// Required. Restricts the search to those vehicles with the specified types.
    /// At least one vehicle type must be specified.
    #[prost(message, repeated, tag = "14")]
    pub vehicle_types: ::prost::alloc::vec::Vec<vehicle::VehicleType>,
    /// Callers can form complex logical operations using the
    /// requiredAttributes and requiredOneOfAttributes fields.
    ///
    /// requiredAttributes is a list; requiredOneOfAttributes uses a message which
    /// allows a list of lists. In combination, the two fields allow the
    /// composition of this expression:
    ///
    /// ```
    /// (required_attribute\[0\] AND required_attribute\[1\] AND ...)
    /// AND
    /// (required_one_of_attribute\[0][0\] OR required_one_of_attribute\[0][1\] OR ...)
    /// AND
    /// (required_one_of_attribute\[1][0\] OR required_one_of_attribute\[1][1\] OR ...)
    /// ```
    ///
    /// Restricts the search to only those vehicles with the specified attributes.
    /// This field is a conjunction/AND operation. Your app can specify up to 100
    /// attributes; however, the combined key:value string length cannot exceed
    /// 1024 characters.
    #[prost(message, repeated, tag = "12")]
    pub required_attributes: ::prost::alloc::vec::Vec<VehicleAttribute>,
    /// Restricts the search to only those vehicles with at least one of
    /// the specified attributes applied to each VehicleAttributeList. Within each
    /// list, a vehicle must match at least one of the attributes. This field is an
    /// inclusive disjunction/OR operation in each VehicleAttributeList and a
    /// conjunction/AND operation across the collection of VehicleAttributeList.
    #[prost(message, repeated, tag = "15")]
    pub required_one_of_attributes: ::prost::alloc::vec::Vec<VehicleAttributeList>,
    /// Restricts the search to only those vehicles with at least one set of the
    /// specified attributes in the VehicleAttributeList. Within each list, a
    /// vehicle must match all of the attributes. This field is a conjunction/AND
    /// operation in each VehicleAttributeList and inclusive disjunction/OR
    /// operation across the collection of VehicleAttributeList.
    #[prost(message, repeated, tag = "20")]
    pub required_one_of_attribute_sets: ::prost::alloc::vec::Vec<VehicleAttributeList>,
    /// Required. Specifies ordering criterion for results.
    #[prost(enumeration = "search_vehicles_request::VehicleMatchOrder", tag = "13")]
    pub order_by: i32,
    /// Indicates if a vehicle with an active trip is eligible for
    /// another match. If `false`, a vehicle is excluded from search results.
    /// If `true`, search results include vehicles with `TripStatus` of
    /// `ENROUTE_TO_DROPOFF`. The services only use this field if
    /// the `SearchVehicles` request has `TripType` set to EXCLUSIVE.
    /// Default value is `false`.
    #[prost(bool, tag = "18")]
    pub include_back_to_back: bool,
    /// Indicates the ID of the trip the searchVehicleRequest is
    /// associated with.
    #[prost(string, tag = "19")]
    pub trip_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SearchVehiclesRequest`.
pub mod search_vehicles_request {
    /// Specifies the sort order of the vehicle matches in the response.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VehicleMatchOrder {
        /// Default, used for unspecified or unrecognized vehicle matches order.
        UnknownVehicleMatchOrder = 0,
        /// Ascending order by vehicle driving time to the pickup point.
        PickupPointEta = 1,
        /// Ascending order by the vehicle driving distance to the pickup point.
        PickupPointDistance = 2,
        /// Ascending order by vehicle driving time to the dropoff point. This order
        /// can only be used if the dropoff_point is specified in the request.
        DropoffPointEta = 3,
        /// Ascending order by straightline distance from vehicle location to pickup
        /// location. This is used primarily as a backup if the maps backend is not
        /// reachable.
        PickupPointStraightDistance = 4,
        /// Ascending order by the match cost.
        Cost = 5,
    }
}
/// SearchVehicles response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchVehiclesResponse {
    /// List of vehicles that match the request options.
    ///
    /// Ordered by ascending vehicle_pickup_eta, with ties broken by ascending
    /// trip_type enum value, followed by matches that don't have
    /// vehicle_pickup_eta set.
    ///
    /// Example response: (Logically represented, not actual response fields):
    ///
    /// * (VehicleId: Vehicle1, ETA: 10 AM, TripType: SHARED),
    /// * (VehicleId: Vehicle2, ETA: 10 AM, TripType: EXCLUSIVE),
    /// * (VehicleId: Vehicle3, ETA: 11 AM, TripType: EXCLUSIVE),
    /// * (VehicleId: Vehicle4, ETA: Not set, TripType: SHARED),
    /// * (VehicleId: Vehicle5, ETA: Not set, TripType: EXCLUSIVE)
    #[prost(message, repeated, tag = "1")]
    pub matches: ::prost::alloc::vec::Vec<VehicleMatch>,
}
/// ListVehicles request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVehiclesRequest {
    /// The standard Fleet Engine request header.
    #[prost(message, optional, tag = "12")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Required. Must be in the format "providers/{provider}".
    /// The provider must be the Project ID (for example, sample-cloud-project)
    /// of the Google Cloud Project of which the service account making
    /// this call is a member.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of vehicles to return.
    /// Default value: 100.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous response, if any.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Specifies the required minimum capacity of the vehicle.
    /// The driver is not considered in the capacity search.
    /// This is just the number of passengers being considered for a trip.
    /// If set, must be greater or equal to 0.
    #[prost(message, optional, tag = "6")]
    pub minimum_capacity: ::core::option::Option<i32>,
    /// Restrict the search to only those vehicles that support at least
    /// one of the specified trip types.
    #[prost(enumeration = "TripType", repeated, tag = "7")]
    pub trip_types: ::prost::alloc::vec::Vec<i32>,
    /// Restrict the search to only those vehicles that have updated
    /// their locations within the specified duration back from now.
    /// If present, must be a valid positive duration.
    #[prost(message, optional, tag = "8")]
    pub maximum_staleness: ::core::option::Option<::prost_types::Duration>,
    /// Required. Restrict the search to those vehicles with the specified type categories.
    #[prost(
        enumeration = "vehicle::vehicle_type::Category",
        repeated,
        packed = "false",
        tag = "9"
    )]
    pub vehicle_type_categories: ::prost::alloc::vec::Vec<i32>,
    /// Callers can form complex logical operations using the
    /// requiredAttributes and requiredOneOfAttributes fields.
    ///
    /// requiredAttributes is a list; requiredOneOfAttributes uses a message which
    /// allows a list of lists. In combination, the two fields allow the
    /// composition of this expression:
    ///
    /// ```
    /// (required_attribute\[0\] AND required_attribute\[1\] AND ...)
    /// AND
    /// (required_one_of_attribute\[0][0\] OR required_one_of_attribute\[0][1\] OR ...)
    /// AND
    /// (required_one_of_attribute\[1][0\] OR required_one_of_attribute\[1][1\] OR ...)
    /// ```
    ///
    /// Restrict the search to only those vehicles
    /// with the specified attributes. This field is a conjunction/AND operation.
    /// Your app can specify up to 100 attributes; however, the combined
    /// key:value string length cannot exceed 1024 characters.
    #[prost(string, repeated, tag = "10")]
    pub required_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Restrict the search to only those vehicles with at least one
    /// of the specified attributes applied to each VehicleAttributeList.
    /// Within each list, a vehicle must match at least one of the attributes.
    /// This field is an inclusive disjunction/OR operation in each
    /// VehicleAttributeList and a conjunction/AND operation across the collection
    /// of VehicleAttributeList.
    /// Format: key1:value1|key2:value2|key3:value3...
    #[prost(string, repeated, tag = "13")]
    pub required_one_of_attributes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Restrict the search to only those vehicles with at least one set of the
    /// specified attributes in the VehicleAttributeList. Within each list, a
    /// vehicle must match all of the attributes. This field is a conjunction/AND
    /// operation in each VehicleAttributeList and inclusive disjunction/OR
    /// operation across the collection of VehicleAttributeList.
    /// Format: key1:value1|key2:value2|key3:value3...
    #[prost(string, repeated, tag = "15")]
    pub required_one_of_attribute_sets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Restrict the search to only those vehicles that have this vehicle state.
    #[prost(enumeration = "VehicleState", tag = "11")]
    pub vehicle_state: i32,
    /// Only return the vehicles with current trip(s).
    #[prost(bool, tag = "14")]
    pub on_trip_only: bool,
}
/// ListVehicles response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVehiclesResponse {
    /// Depends on vehicles matching request criteria.
    /// There will be a maximum number of vehicles returned based on the page_size
    /// field in the request.
    #[prost(message, repeated, tag = "1")]
    pub vehicles: ::prost::alloc::vec::Vec<Vehicle>,
    /// Token to retrieve the next page of vehicles, or empty if there are no
    /// more vehicles in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Required. Total number of vehicles matching request criteria across all pages.
    #[prost(int64, tag = "3")]
    pub total_size: i64,
}
/// Waypoint describes intermediate points along a route.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Waypoint {
    /// The location of this waypoint.
    #[prost(message, optional, tag = "1")]
    pub lat_lng: ::core::option::Option<super::super::super::google::r#type::LatLng>,
    /// The estimated time that the vehicle will arrive at this waypoint.
    #[prost(message, optional, tag = "2")]
    pub eta: ::core::option::Option<::prost_types::Timestamp>,
}
/// VehicleMatch contains the vehicle, ETA, and distance calculations for a
/// vehicle that matches the SearchVehiclesRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleMatch {
    /// Required. A vehicle that matches the request.
    #[prost(message, optional, tag = "1")]
    pub vehicle: ::core::option::Option<Vehicle>,
    /// The vehicle's driving ETA to the pickup point specified in the
    /// request. An empty value indicates a failure in calculating ETA for the
    /// vehicle.
    #[prost(message, optional, tag = "2")]
    pub vehicle_pickup_eta: ::core::option::Option<::prost_types::Timestamp>,
    /// The vehicle's driving distance to the pickup point specified in
    /// the request, including any intermediate pickup or dropoff points for
    /// an existing ride.  An empty value indicates a failure in calculating
    /// distance for the vehicle.
    #[prost(message, optional, tag = "3")]
    pub vehicle_pickup_distance_meters: ::core::option::Option<i32>,
    /// Required. The straight-line distance between the vehicle and the pickup
    /// point specified in the request, including intermediate waypoints for
    /// existing trips.
    #[prost(message, optional, tag = "11")]
    pub vehicle_pickup_straight_line_distance_meters: ::core::option::Option<i32>,
    /// The complete vehicle's driving ETA to the drop off point
    /// specified in the request. The ETA includes any required visits for active
    /// trips that must be completed before the vehicle visits the dropoff_point
    /// specified in the request. The value will only be populated when a
    /// dropoff_point is specified in the request. An empty value indicates
    /// a failure in calculating the ETA for the vehicle to reach
    /// the dropoff_point.
    #[prost(message, optional, tag = "4")]
    pub vehicle_dropoff_eta: ::core::option::Option<::prost_types::Timestamp>,
    /// The vehicle's driving distance (in meters) from the pickup point
    /// to the drop off point specified in the request. The distance is only
    /// between the two points and does not include the vehicle location or any
    /// other points that must be visited before the vehicle visits either the
    /// pickup point or dropoff point. The value will only be populated when a
    /// dropoff_point is specified in the request. An empty value indicates
    /// a failure in calculating the distance from the pickup to
    /// dropoff points specified in the request.
    #[prost(message, optional, tag = "5")]
    pub vehicle_pickup_to_dropoff_distance_meters: ::core::option::Option<i32>,
    /// Required. The trip type of the request that was used to calculate the ETA
    /// to the pickup point.
    #[prost(enumeration = "TripType", tag = "6")]
    pub trip_type: i32,
    /// The ordered list of waypoints used to calculate the ETA. The list
    /// will include the vehicle location, the pickup/drop off points of active
    /// trips for the vehicle and the pickup/dropoff points provided in the
    /// request. An empty list indicates a failure in calculating ETA for the
    /// vehicle.
    #[prost(message, repeated, tag = "7")]
    pub vehicle_trips_waypoints: ::prost::alloc::vec::Vec<Waypoint>,
    /// Type of the vehicle match.
    #[prost(enumeration = "vehicle_match::VehicleMatchType", tag = "8")]
    pub vehicle_match_type: i32,
    /// The method the caller requested for sorting vehicle matches.
    #[prost(enumeration = "search_vehicles_request::VehicleMatchOrder", tag = "9")]
    pub requested_ordered_by: i32,
    /// The actual method that is used to order this vehicle. In normal cases this
    /// will match the 'order_by' field from the request, however in certain
    /// circumstances such as a failure of google maps backends, a different method
    /// may be used (such as PICKUP_POINT_STRAIGHT_DISTANCE).
    #[prost(enumeration = "search_vehicles_request::VehicleMatchOrder", tag = "10")]
    pub ordered_by: i32,
}
/// Nested message and enum types in `VehicleMatch`.
pub mod vehicle_match {
    /// Type of vehicle match.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VehicleMatchType {
        /// Unknown vehicle match type
        Unknown = 0,
        /// Exclusive vehicle trip match
        Exclusive = 1,
        /// Back to back ride match.
        BackToBack = 2,
        /// Carpool ride match.
        Carpool = 3,
        /// Carpool ride match. The car has an active exclusive trip.
        CarpoolBackToBack = 4,
    }
}
/// This messages allows a list-of-list datatype for VehicleAttribute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VehicleAttributeList {
    /// A list of attributes in this collection.
    #[prost(message, repeated, tag = "1")]
    pub attributes: ::prost::alloc::vec::Vec<VehicleAttribute>,
}
#[doc = r" Generated client implementations."]
pub mod vehicle_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Vehicle management service."]
    #[derive(Debug, Clone)]
    pub struct VehicleServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VehicleServiceClient<T>
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
        ) -> VehicleServiceClient<InterceptedService<T, F>>
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
            VehicleServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " CreateVehicle instantiates a new vehicle associated with a rideshare"]
        #[doc = " provider in the Fleet Engine. Vehicles must have a unique vehicle ID."]
        #[doc = ""]
        #[doc = " The following Vehicle fields are required when creating a Vehicle:"]
        #[doc = ""]
        #[doc = " * vehicleState"]
        #[doc = " * supportedTripTypes"]
        #[doc = " * maximumCapacity"]
        #[doc = " * vehicleType"]
        #[doc = ""]
        #[doc = " The following Vehicle fields are ignored when creating a Vehicle:"]
        #[doc = ""]
        #[doc = " * name"]
        #[doc = " * currentTrips"]
        #[doc = " * availableCapacity"]
        #[doc = " * current_route_segment"]
        #[doc = " * current_route_segment_version"]
        #[doc = " * waypoint"]
        #[doc = " * waypoints_version"]
        #[doc = " * remaining_distance_meters"]
        #[doc = " * eta_to_next_waypoint"]
        #[doc = " * navigation_status"]
        #[doc = ""]
        #[doc = " All other fields are optional and used if provided."]
        pub async fn create_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVehicleRequest>,
        ) -> Result<tonic::Response<super::Vehicle>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/CreateVehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetVehicle returns a vehicle from the Fleet Engine."]
        pub async fn get_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVehicleRequest>,
        ) -> Result<tonic::Response<super::Vehicle>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/GetVehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UpdateVehicle writes updated vehicle data to the Fleet Engine."]
        #[doc = ""]
        #[doc = " When updating a Vehicle, the following fields cannot be updated since they"]
        #[doc = " are managed by the Fleet Engine:"]
        #[doc = ""]
        #[doc = " * currentTrips"]
        #[doc = " * availableCapacity"]
        #[doc = " * current_route_segment_version"]
        #[doc = " * waypoints_version"]
        #[doc = ""]
        #[doc = " The vehicle name also cannot be updated."]
        #[doc = ""]
        #[doc = " The waypoints field can be updated, but must contain all the waypoints"]
        #[doc = " currently on the vehicle, and no other waypoints."]
        pub async fn update_vehicle(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVehicleRequest>,
        ) -> Result<tonic::Response<super::Vehicle>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/UpdateVehicle",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UpdateVehicleLocation updates the location of the vehicle."]
        #[doc = " This method is deprecated. Use UpdateVehicle method instead."]
        pub async fn update_vehicle_location(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVehicleLocationRequest>,
        ) -> Result<tonic::Response<super::VehicleLocation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/UpdateVehicleLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UpdateVehicleAttributes partially updates a vehicle's attributes."]
        #[doc = " Only the attributes mentioned in the request will be updated, other"]
        #[doc = " attributes will NOT be altered. Note: this is different in UpdateVehicle,"]
        #[doc = " where the whole `attributes` field will be replaced by the one in"]
        #[doc = " UpdateVehicleRequest, attributes not in the request would be removed."]
        pub async fn update_vehicle_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateVehicleAttributesRequest>,
        ) -> Result<tonic::Response<super::UpdateVehicleAttributesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/UpdateVehicleAttributes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ListVehicles returns a paginated list of vehicles associated with"]
        #[doc = " a provider that match the request options."]
        pub async fn list_vehicles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVehiclesRequest>,
        ) -> Result<tonic::Response<super::ListVehiclesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/ListVehicles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " SearchVehicles returns a list of vehicles that match the request options."]
        pub async fn search_vehicles(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchVehiclesRequest>,
        ) -> Result<tonic::Response<super::SearchVehiclesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/SearchVehicles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " SearchFuzzedVehicles returns a list of vehicles that match the request"]
        #[doc = " options with their locations fuzzed."]
        #[doc = " Request does not support 'order_by' field."]
        #[doc = " Vehicle matches in response will be in order of distance from pickup point."]
        #[doc = " Vehicle matches in response will only have 'vehicle' and 'trip_type' field"]
        #[doc = " set."]
        pub async fn search_fuzzed_vehicles(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchVehiclesRequest>,
        ) -> Result<tonic::Response<super::SearchVehiclesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/maps.fleetengine.v1.VehicleService/SearchFuzzedVehicles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
