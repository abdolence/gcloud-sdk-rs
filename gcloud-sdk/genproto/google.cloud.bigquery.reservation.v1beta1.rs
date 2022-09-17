/// A reservation is a mechanism used to guarantee slots to users.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reservation {
    /// The resource name of the reservation, e.g.,
    /// `projects/*/locations/*/reservations/team1-prod`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Minimum slots available to this reservation. A slot is a unit of
    /// computational power in BigQuery, and serves as the unit of parallelism.
    ///
    /// Queries using this reservation might use more slots during runtime if
    /// ignore_idle_slots is set to false.
    ///
    /// If the new reservation's slot capacity exceed the parent's slot capacity or
    /// if total slot capacity of the new reservation and its siblings exceeds the
    /// parent's slot capacity, the request will fail with
    /// `google.rpc.Code.RESOURCE_EXHAUSTED`.
    #[prost(int64, tag="2")]
    pub slot_capacity: i64,
    /// If false, any query using this reservation will use idle slots from other
    /// reservations within the same admin project. If true, a query using this
    /// reservation will execute with the slot capacity specified above at most.
    #[prost(bool, tag="4")]
    pub ignore_idle_slots: bool,
}
/// Capacity commitment is a way to purchase compute capacity for BigQuery jobs
/// (in the form of slots) with some committed period of usage. Annual
/// commitments renew by default. Commitments can be removed after their
/// commitment end time passes.
///
/// In order to remove annual commitment, its plan needs to be changed
/// to monthly or flex first.
///
/// A capacity commitment resource exists as a child resource of the admin
/// project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapacityCommitment {
    /// Output only. The resource name of the capacity commitment, e.g.,
    /// `projects/myproject/locations/US/capacityCommitments/123`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Number of slots in this commitment.
    #[prost(int64, tag="2")]
    pub slot_count: i64,
    /// Capacity commitment commitment plan.
    #[prost(enumeration="capacity_commitment::CommitmentPlan", tag="3")]
    pub plan: i32,
    /// Output only. State of the commitment.
    #[prost(enumeration="capacity_commitment::State", tag="4")]
    pub state: i32,
    /// Output only. The end of the current commitment period. It is applicable
    /// only for ACTIVE capacity commitments.
    #[prost(message, optional, tag="5")]
    pub commitment_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For FAILED commitment plan, provides the reason of failure.
    #[prost(message, optional, tag="7")]
    pub failure_status: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// The plan this capacity commitment is converted to after commitment_end_time
    /// passes. Once the plan is changed, committed period is extended according to
    /// commitment plan. Only applicable for ANNUAL commitments.
    #[prost(enumeration="capacity_commitment::CommitmentPlan", tag="8")]
    pub renewal_plan: i32,
}
/// Nested message and enum types in `CapacityCommitment`.
pub mod capacity_commitment {
    /// Commitment plan defines the current committed period. Capacity commitment
    /// cannot be deleted during it's committed period.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CommitmentPlan {
        /// Invalid plan value. Requests with this value will be rejected with
        /// error code `google.rpc.Code.INVALID_ARGUMENT`.
        Unspecified = 0,
        /// Flex commitments have committed period of 1 minute after becoming ACTIVE.
        /// After that, they are not in a committed period anymore and can be removed
        /// any time.
        Flex = 3,
        /// Trial commitments have a committed period of 182 days after becoming
        /// ACTIVE. After that, they are converted to a new commitment based on the
        /// `renewal_plan`. Default `renewal_plan` for Trial commitment is Flex so
        /// that it can be deleted right after committed period ends.
        Trial = 5,
        /// Monthly commitments have a committed period of 30 days after becoming
        /// ACTIVE. After that, they are not in a committed period anymore and can be
        /// removed any time.
        Monthly = 2,
        /// Annual commitments have a committed period of 365 days after becoming
        /// ACTIVE. After that they are converted to a new commitment based on the
        /// renewal_plan.
        Annual = 4,
    }
    impl CommitmentPlan {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CommitmentPlan::Unspecified => "COMMITMENT_PLAN_UNSPECIFIED",
                CommitmentPlan::Flex => "FLEX",
                CommitmentPlan::Trial => "TRIAL",
                CommitmentPlan::Monthly => "MONTHLY",
                CommitmentPlan::Annual => "ANNUAL",
            }
        }
    }
    /// Capacity commitment can either become ACTIVE right away or transition
    /// from PENDING to ACTIVE or FAILED.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid state value.
        Unspecified = 0,
        /// Capacity commitment is pending provisioning. Pending capacity commitment
        /// does not contribute to the parent's slot_capacity.
        Pending = 1,
        /// Once slots are provisioned, capacity commitment becomes active.
        /// slot_count is added to the parent's slot_capacity.
        Active = 2,
        /// Capacity commitment is failed to be activated by the backend.
        Failed = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Pending => "PENDING",
                State::Active => "ACTIVE",
                State::Failed => "FAILED",
            }
        }
    }
}
/// The request for
/// \[ReservationService.CreateReservation][google.cloud.bigquery.reservation.v1beta1.ReservationService.CreateReservation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReservationRequest {
    /// Required. Project, location. E.g.,
    /// `projects/myproject/locations/US`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The reservation ID. This field must only contain lower case alphanumeric
    /// characters or dash. Max length is 64 characters.
    #[prost(string, tag="2")]
    pub reservation_id: ::prost::alloc::string::String,
    /// Content of the new reservation to create.
    #[prost(message, optional, tag="3")]
    pub reservation: ::core::option::Option<Reservation>,
}
/// The request for
/// \[ReservationService.ListReservations][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListReservations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReservationsRequest {
    /// Required. The parent resource name containing project and location, e.g.:
    ///    `projects/myproject/locations/US`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Can be used to filter out reservations based on names, capacity, etc, e.g.:
    /// filter="reservation.slot_capacity > 200"
    /// filter="reservation.name = \"*dev/*\""
    /// Advanced filtering syntax can be
    /// \[here\](<https://cloud.google.com/logging/docs/view/advanced-filters>).
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
}
/// The response for
/// \[ReservationService.ListReservations][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListReservations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReservationsResponse {
    /// List of reservations visible to the user.
    #[prost(message, repeated, tag="1")]
    pub reservations: ::prost::alloc::vec::Vec<Reservation>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for
/// \[ReservationService.GetReservation][google.cloud.bigquery.reservation.v1beta1.ReservationService.GetReservation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReservationRequest {
    /// Required. Resource name of the reservation to retrieve. E.g.,
    ///     `projects/myproject/locations/US/reservations/team1-prod`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[ReservationService.DeleteReservation][google.cloud.bigquery.reservation.v1beta1.ReservationService.DeleteReservation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReservationRequest {
    /// Required. Resource name of the reservation to retrieve. E.g.,
    ///     `projects/myproject/locations/US/reservations/team1-prod`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[ReservationService.UpdateReservation][google.cloud.bigquery.reservation.v1beta1.ReservationService.UpdateReservation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReservationRequest {
    /// Content of the reservation to update.
    #[prost(message, optional, tag="1")]
    pub reservation: ::core::option::Option<Reservation>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request for
/// \[ReservationService.CreateCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.CreateCapacityCommitment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCapacityCommitmentRequest {
    /// Required. Resource name of the parent reservation. E.g.,
    ///     `projects/myproject/locations/US`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Content of the capacity commitment to create.
    #[prost(message, optional, tag="2")]
    pub capacity_commitment: ::core::option::Option<CapacityCommitment>,
    /// If true, fail the request if another project in the organization has a
    /// capacity commitment.
    #[prost(bool, tag="4")]
    pub enforce_single_admin_project_per_org: bool,
}
/// The request for
/// \[ReservationService.ListCapacityCommitments][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListCapacityCommitments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCapacityCommitmentsRequest {
    /// Required. Resource name of the parent reservation. E.g.,
    ///     `projects/myproject/locations/US`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for
/// \[ReservationService.ListCapacityCommitments][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListCapacityCommitments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCapacityCommitmentsResponse {
    /// List of capacity commitments visible to the user.
    #[prost(message, repeated, tag="1")]
    pub capacity_commitments: ::prost::alloc::vec::Vec<CapacityCommitment>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for
/// \[ReservationService.GetCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.GetCapacityCommitment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCapacityCommitmentRequest {
    /// Required. Resource name of the capacity commitment to retrieve. E.g.,
    ///     `projects/myproject/locations/US/capacityCommitments/123`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[ReservationService.DeleteCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.DeleteCapacityCommitment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCapacityCommitmentRequest {
    /// Required. Resource name of the capacity commitment to delete. E.g.,
    ///     `projects/myproject/locations/US/capacityCommitments/123`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[ReservationService.UpdateCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.UpdateCapacityCommitment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCapacityCommitmentRequest {
    /// Content of the capacity commitment to update.
    #[prost(message, optional, tag="1")]
    pub capacity_commitment: ::core::option::Option<CapacityCommitment>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request for
/// \[ReservationService.SplitCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.SplitCapacityCommitment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitCapacityCommitmentRequest {
    /// Required. The resource name e.g.,:
    ///   `projects/myproject/locations/US/capacityCommitments/123`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Number of slots in the capacity commitment after the split.
    #[prost(int64, tag="2")]
    pub slot_count: i64,
}
/// The response for
/// \[ReservationService.SplitCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.SplitCapacityCommitment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitCapacityCommitmentResponse {
    /// First capacity commitment, result of a split.
    #[prost(message, optional, tag="1")]
    pub first: ::core::option::Option<CapacityCommitment>,
    /// Second capacity commitment, result of a split.
    #[prost(message, optional, tag="2")]
    pub second: ::core::option::Option<CapacityCommitment>,
}
/// The request for
/// \[ReservationService.MergeCapacityCommitments][google.cloud.bigquery.reservation.v1beta1.ReservationService.MergeCapacityCommitments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeCapacityCommitmentsRequest {
    /// Parent resource that identifies admin project and location e.g.,
    ///   `projects/myproject/locations/us`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Ids of capacity commitments to merge.
    /// These capacity commitments must exist under admin project and location
    /// specified in the parent.
    #[prost(string, repeated, tag="2")]
    pub capacity_commitment_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Assignment allows a project to submit jobs
/// of a certain type using slots from the specified reservation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assignment {
    /// Output only. Name of the resource. E.g.:
    /// `projects/myproject/locations/US/reservations/team1-prod/assignments/123`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The resource which will use the reservation. E.g.
    /// `projects/myproject`, `folders/123`, or `organizations/456`.
    #[prost(string, tag="4")]
    pub assignee: ::prost::alloc::string::String,
    /// Which type of jobs will use the reservation.
    #[prost(enumeration="assignment::JobType", tag="3")]
    pub job_type: i32,
    /// Output only. State of the assignment.
    #[prost(enumeration="assignment::State", tag="6")]
    pub state: i32,
}
/// Nested message and enum types in `Assignment`.
pub mod assignment {
    /// Types of job, which could be specified when using the reservation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JobType {
        /// Invalid type. Requests with this value will be rejected with
        /// error code `google.rpc.Code.INVALID_ARGUMENT`.
        Unspecified = 0,
        /// Pipeline (load/export) jobs from the project will use the reservation.
        Pipeline = 1,
        /// Query jobs from the project will use the reservation.
        Query = 2,
    }
    impl JobType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JobType::Unspecified => "JOB_TYPE_UNSPECIFIED",
                JobType::Pipeline => "PIPELINE",
                JobType::Query => "QUERY",
            }
        }
    }
    /// Assignment will remain in PENDING state if no active capacity commitment is
    /// present. It will become ACTIVE when some capacity commitment becomes
    /// active.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid state value.
        Unspecified = 0,
        /// Queries from assignee will be executed as on-demand, if related
        /// assignment is pending.
        Pending = 1,
        /// Assignment is ready.
        Active = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Pending => "PENDING",
                State::Active => "ACTIVE",
            }
        }
    }
}
/// The request for
/// \[ReservationService.CreateAssignment][google.cloud.bigquery.reservation.v1beta1.ReservationService.CreateAssignment\].
/// Note: "bigquery.reservationAssignments.create" permission is required on the
/// related assignee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAssignmentRequest {
    /// Required. The parent resource name of the assignment
    /// E.g. `projects/myproject/locations/US/reservations/team1-prod`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Assignment resource to create.
    #[prost(message, optional, tag="2")]
    pub assignment: ::core::option::Option<Assignment>,
}
/// The request for
/// \[ReservationService.ListAssignments][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListAssignments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssignmentsRequest {
    /// Required. The parent resource name e.g.:
    ///
    /// `projects/myproject/locations/US/reservations/team1-prod`
    ///
    /// Or:
    ///
    /// `projects/myproject/locations/US/reservations/-`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for
/// \[ReservationService.ListAssignments][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListAssignments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssignmentsResponse {
    /// List of assignments visible to the user.
    #[prost(message, repeated, tag="1")]
    pub assignments: ::prost::alloc::vec::Vec<Assignment>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for
/// \[ReservationService.DeleteAssignment][google.cloud.bigquery.reservation.v1beta1.ReservationService.DeleteAssignment\].
/// Note: "bigquery.reservationAssignments.delete" permission is required on the
/// related assignee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAssignmentRequest {
    /// Required. Name of the resource, e.g.
    ///    `projects/myproject/locations/US/reservations/team1-prod/assignments/123`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[ReservationService.SearchAssignments][google.cloud.bigquery.reservation.v1beta1.ReservationService.SearchAssignments\].
/// Note: "bigquery.reservationAssignments.search" permission is required on the
/// related assignee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAssignmentsRequest {
    /// Required. The resource name of the admin project(containing project and
    /// location), e.g.:
    ///    `projects/myproject/locations/US`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Please specify resource name as assignee in the query.
    ///
    /// Examples:
    ///
    /// * `assignee=projects/myproject`
    /// * `assignee=folders/123`
    /// * `assignee=organizations/456`
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for
/// \[ReservationService.SearchAssignments][google.cloud.bigquery.reservation.v1beta1.ReservationService.SearchAssignments\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAssignmentsResponse {
    /// List of assignments visible to the user.
    #[prost(message, repeated, tag="1")]
    pub assignments: ::prost::alloc::vec::Vec<Assignment>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for
/// \[ReservationService.MoveAssignment][google.cloud.bigquery.reservation.v1beta1.ReservationService.MoveAssignment\].
///
/// **Note**: "bigquery.reservationAssignments.create" permission is required on
/// the destination_id.
///
/// **Note**: "bigquery.reservationAssignments.create" and
/// "bigquery.reservationAssignments.delete" permission are required on the
/// related assignee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveAssignmentRequest {
    /// Required. The resource name of the assignment,
    /// e.g.
    /// `projects/myproject/locations/US/reservations/team1-prod/assignments/123`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The new reservation ID, e.g.:
    ///    `projects/myotherproject/locations/US/reservations/team2-prod`
    #[prost(string, tag="3")]
    pub destination_id: ::prost::alloc::string::String,
}
/// Represents a BI Reservation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiReservation {
    /// The resource name of the singleton BI reservation.
    /// Reservation names have the form
    /// `projects/{project_id}/locations/{location_id}/bireservation`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The last update timestamp of a reservation.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Size of a reservation, in bytes.
    #[prost(int64, tag="4")]
    pub size: i64,
}
/// A request to get a singleton BI reservation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBiReservationRequest {
    /// Required. Name of the requested reservation, for example:
    /// `projects/{project_id}/locations/{location_id}/bireservation`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to update a BI reservation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBiReservationRequest {
    /// A reservation to update.
    #[prost(message, optional, tag="1")]
    pub reservation: ::core::option::Option<BiReservation>,
    /// A list of fields to be updated in this request.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Generated client implementations.
pub mod reservation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Deprecated: Please use the v1 api instead.
    /// This API allows users to manage their flat-rate BigQuery reservations.
    ///
    /// A reservation provides computational resource guarantees, in the form of
    /// [slots](https://cloud.google.com/bigquery/docs/slots), to users. A slot is a
    /// unit of computational power in BigQuery, and serves as the basic unit of
    /// parallelism. In a scan of a multi-partitioned table, a single slot operates
    /// on a single partition of the table. A reservation resource exists as a child
    /// resource of the admin project and location, e.g.:
    ///   `projects/myproject/locations/US/reservations/reservationName`.
    ///
    /// A capacity commitment is a way to purchase compute capacity for BigQuery jobs
    /// (in the form of slots) with some committed period of usage. A capacity
    /// commitment resource exists as a child resource of the admin project and
    /// location, e.g.:
    ///   `projects/myproject/locations/US/capacityCommitments/id`.
    #[derive(Debug, Clone)]
    pub struct ReservationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReservationServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ReservationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ReservationServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ReservationServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Creates a new reservation resource.
        pub async fn create_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReservationRequest>,
        ) -> Result<tonic::Response<super::Reservation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/CreateReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all the reservations for the project in the specified location.
        pub async fn list_reservations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReservationsRequest>,
        ) -> Result<tonic::Response<super::ListReservationsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/ListReservations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns information about the reservation.
        pub async fn get_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReservationRequest>,
        ) -> Result<tonic::Response<super::Reservation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/GetReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a reservation.
        /// Returns `google.rpc.Code.FAILED_PRECONDITION` when reservation has
        /// assignments.
        pub async fn delete_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReservationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/DeleteReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing reservation resource.
        pub async fn update_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateReservationRequest>,
        ) -> Result<tonic::Response<super::Reservation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/UpdateReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new capacity commitment resource.
        pub async fn create_capacity_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<super::CapacityCommitment>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/CreateCapacityCommitment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all the capacity commitments for the admin project.
        pub async fn list_capacity_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCapacityCommitmentsRequest>,
        ) -> Result<
            tonic::Response<super::ListCapacityCommitmentsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/ListCapacityCommitments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns information about the capacity commitment.
        pub async fn get_capacity_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<super::CapacityCommitment>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/GetCapacityCommitment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a capacity commitment. Attempting to delete capacity commitment
        /// before its commitment_end_time will fail with the error code
        /// `google.rpc.Code.FAILED_PRECONDITION`.
        pub async fn delete_capacity_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/DeleteCapacityCommitment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates an existing capacity commitment.
        ///
        /// Only `plan` and `renewal_plan` fields can be updated.
        ///
        /// Plan can only be changed to a plan of a longer commitment period.
        /// Attempting to change to a plan with shorter commitment period will fail
        /// with the error code `google.rpc.Code.FAILED_PRECONDITION`.
        pub async fn update_capacity_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<super::CapacityCommitment>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/UpdateCapacityCommitment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Splits capacity commitment to two commitments of the same plan and
        /// `commitment_end_time`.
        ///
        /// A common use case is to enable downgrading commitments.
        ///
        /// For example, in order to downgrade from 10000 slots to 8000, you might
        /// split a 10000 capacity commitment into commitments of 2000 and 8000. Then,
        /// you would change the plan of the first one to `FLEX` and then delete it.
        pub async fn split_capacity_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::SplitCapacityCommitmentRequest>,
        ) -> Result<
            tonic::Response<super::SplitCapacityCommitmentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/SplitCapacityCommitment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Merges capacity commitments of the same plan into a single commitment.
        ///
        /// The resulting capacity commitment has the greater commitment_end_time
        /// out of the to-be-merged capacity commitments.
        ///
        /// Attempting to merge capacity commitments of different plan will fail
        /// with the error code `google.rpc.Code.FAILED_PRECONDITION`.
        pub async fn merge_capacity_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeCapacityCommitmentsRequest>,
        ) -> Result<tonic::Response<super::CapacityCommitment>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/MergeCapacityCommitments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates an assignment object which allows the given project to submit jobs
        /// of a certain type using slots from the specified reservation.
        ///
        /// Currently a
        /// resource (project, folder, organization) can only have one assignment per
        /// each (job_type, location) combination, and that reservation will be used
        /// for all jobs of the matching type.
        ///
        /// Different assignments can be created on different levels of the
        /// projects, folders or organization hierarchy.  During query execution,
        /// the assignment is looked up at the project, folder and organization levels
        /// in that order. The first assignment found is applied to the query.
        ///
        /// When creating assignments, it does not matter if other assignments exist at
        /// higher levels.
        ///
        /// Example:
        ///
        /// * The organization `organizationA` contains two projects, `project1`
        ///   and `project2`.
        /// * Assignments for all three entities (`organizationA`, `project1`, and
        ///   `project2`) could all be created and mapped to the same or different
        ///   reservations.
        ///
        /// Returns `google.rpc.Code.PERMISSION_DENIED` if user does not have
        /// 'bigquery.admin' permissions on the project using the reservation
        /// and the project that owns this reservation.
        ///
        /// Returns `google.rpc.Code.INVALID_ARGUMENT` when location of the assignment
        /// does not match location of the reservation.
        pub async fn create_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAssignmentRequest>,
        ) -> Result<tonic::Response<super::Assignment>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/CreateAssignment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists assignments.
        ///
        /// Only explicitly created assignments will be returned.
        ///
        /// Example:
        ///
        /// * Organization `organizationA` contains two projects, `project1` and
        ///   `project2`.
        /// * Reservation `res1` exists and was created previously.
        /// * CreateAssignment was used previously to define the following
        ///   associations between entities and reservations: `<organizationA, res1>`
        ///   and `<project1, res1>`
        ///
        /// In this example, ListAssignments will just return the above two assignments
        /// for reservation `res1`, and no expansion/merge will happen.
        ///
        /// The wildcard "-" can be used for
        /// reservations in the request. In that case all assignments belongs to the
        /// specified project and location will be listed.
        ///
        /// **Note** "-" cannot be used for projects nor locations.
        pub async fn list_assignments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssignmentsRequest>,
        ) -> Result<tonic::Response<super::ListAssignmentsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/ListAssignments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a assignment. No expansion will happen.
        ///
        /// Example:
        ///
        /// * Organization `organizationA` contains two projects, `project1` and
        ///   `project2`.
        /// * Reservation `res1` exists and was created previously.
        /// * CreateAssignment was used previously to define the following
        ///   associations between entities and reservations: `<organizationA, res1>`
        ///   and `<project1, res1>`
        ///
        /// In this example, deletion of the `<organizationA, res1>` assignment won't
        /// affect the other assignment `<project1, res1>`. After said deletion,
        /// queries from `project1` will still use `res1` while queries from
        /// `project2` will switch to use on-demand mode.
        pub async fn delete_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAssignmentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/DeleteAssignment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Looks up assignments for a specified resource for a particular region.
        /// If the request is about a project:
        ///
        /// 1. Assignments created on the project will be returned if they exist.
        /// 2. Otherwise assignments created on the closest ancestor will be
        ///    returned.
        /// 3. Assignments for different JobTypes will all be returned.
        ///
        /// The same logic applies if the request is about a folder.
        ///
        /// If the request is about an organization, then assignments created on the
        /// organization will be returned (organization doesn't have ancestors).
        ///
        /// Comparing to ListAssignments, there are some behavior
        /// differences:
        ///
        /// 1. permission on the assignee will be verified in this API.
        /// 2. Hierarchy lookup (project->folder->organization) happens in this API.
        /// 3. Parent here is `projects/*/locations/*`, instead of
        ///    `projects/*/locations/*reservations/*`.
        ///
        /// **Note** "-" cannot be used for projects
        /// nor locations.
        pub async fn search_assignments(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAssignmentsRequest>,
        ) -> Result<tonic::Response<super::SearchAssignmentsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/SearchAssignments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Moves an assignment under a new reservation.
        ///
        /// This differs from removing an existing assignment and recreating a new one
        /// by providing a transactional change that ensures an assignee always has an
        /// associated reservation.
        pub async fn move_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveAssignmentRequest>,
        ) -> Result<tonic::Response<super::Assignment>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/MoveAssignment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves a BI reservation.
        pub async fn get_bi_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBiReservationRequest>,
        ) -> Result<tonic::Response<super::BiReservation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/GetBiReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a BI reservation.
        ///
        /// Only fields specified in the `field_mask` are updated.
        ///
        /// A singleton BI reservation always exists with default size 0.
        /// In order to reserve BI capacity it needs to be updated to an amount
        /// greater than 0. In order to release BI capacity reservation size
        /// must be set to 0.
        pub async fn update_bi_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBiReservationRequest>,
        ) -> Result<tonic::Response<super::BiReservation>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/UpdateBiReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
