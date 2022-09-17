/// A note that indicates a type of analysis a provider would perform. This note
/// exists in a provider's project. A `Discovery` occurrence is created in a
/// consumer's project at the start of analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Discovery {
    /// Required. Immutable. The kind of analysis that is handled by this
    /// discovery.
    #[prost(enumeration="super::NoteKind", tag="1")]
    pub analysis_kind: i32,
}
/// Details of a discovery occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Details {
    /// Required. Analysis status for the discovered resource.
    #[prost(message, optional, tag="1")]
    pub discovered: ::core::option::Option<Discovered>,
}
/// Provides information about the analysis status of a discovered resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Discovered {
    /// Whether the resource is continuously analyzed.
    #[prost(enumeration="discovered::ContinuousAnalysis", tag="1")]
    pub continuous_analysis: i32,
    /// The last time continuous analysis was done for this resource.
    #[prost(message, optional, tag="2")]
    pub last_analysis_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The status of discovery for the resource.
    #[prost(enumeration="discovered::AnalysisStatus", tag="3")]
    pub analysis_status: i32,
    /// When an error is encountered this will contain a LocalizedMessage under
    /// details to show to the user. The LocalizedMessage is output only and
    /// populated by the API.
    #[prost(message, optional, tag="4")]
    pub analysis_status_error: ::core::option::Option<super::super::super::google::rpc::Status>,
}
/// Nested message and enum types in `Discovered`.
pub mod discovered {
    /// Whether the resource is continuously analyzed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ContinuousAnalysis {
        /// Unknown.
        Unspecified = 0,
        /// The resource is continuously analyzed.
        Active = 1,
        /// The resource is ignored for continuous analysis.
        Inactive = 2,
    }
    impl ContinuousAnalysis {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ContinuousAnalysis::Unspecified => "CONTINUOUS_ANALYSIS_UNSPECIFIED",
                ContinuousAnalysis::Active => "ACTIVE",
                ContinuousAnalysis::Inactive => "INACTIVE",
            }
        }
    }
    /// Analysis status for a resource. Currently for initial analysis only (not
    /// updated in continuous analysis).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AnalysisStatus {
        /// Unknown.
        Unspecified = 0,
        /// Resource is known but no action has been taken yet.
        Pending = 1,
        /// Resource is being analyzed.
        Scanning = 2,
        /// Analysis has finished successfully.
        FinishedSuccess = 3,
        /// Analysis has finished unsuccessfully, the analysis itself is in a bad
        /// state.
        FinishedFailed = 4,
        /// The resource is known not to be supported
        FinishedUnsupported = 5,
    }
    impl AnalysisStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AnalysisStatus::Unspecified => "ANALYSIS_STATUS_UNSPECIFIED",
                AnalysisStatus::Pending => "PENDING",
                AnalysisStatus::Scanning => "SCANNING",
                AnalysisStatus::FinishedSuccess => "FINISHED_SUCCESS",
                AnalysisStatus::FinishedFailed => "FINISHED_FAILED",
                AnalysisStatus::FinishedUnsupported => "FINISHED_UNSUPPORTED",
            }
        }
    }
}
