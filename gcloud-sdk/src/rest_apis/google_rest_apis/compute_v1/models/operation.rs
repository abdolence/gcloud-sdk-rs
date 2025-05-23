use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

use crate::google_rest_apis::compute_v1::models;

/// Operation : Represents an Operation resource. Google Compute Engine has three Operation resources: * [Global](/compute/docs/reference/rest/v1/globalOperations) * [Regional](/compute/docs/reference/rest/v1/regionOperations) * [Zonal](/compute/docs/reference/rest/v1/zoneOperations) You can use an operation resource to manage asynchronous API requests. For more information, read Handling API responses. Operations can be global, regional or zonal. - For global operations, use the `globalOperations` resource. - For regional operations, use the `regionOperations` resource. - For zonal operations, use the `zoneOperations` resource. For more information, read Global, Regional, and Zonal Resources. Note that completed Operation resources have a limited retention period.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    /// [Output Only] The value of `requestId` if you provided it in the request. Not present otherwise.
    #[serde(rename = "clientOperationId", skip_serializing_if = "Option::is_none")]
    pub client_operation_id: Option<String>,
    /// [Deprecated] This field is deprecated.
    #[serde(rename = "creationTimestamp", skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    /// [Output Only] A textual description of the operation, which is set when the operation is created.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Output Only] The time that this operation was completed. This value is in RFC3339 text format.
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::OperationError>>,
    /// [Output Only] If the operation fails, this field contains the HTTP error message that was returned, such as `NOT FOUND`.
    #[serde(rename = "httpErrorMessage", skip_serializing_if = "Option::is_none")]
    pub http_error_message: Option<String>,
    /// [Output Only] If the operation fails, this field contains the HTTP error status code that was returned. For example, a `404` means the resource was not found.
    #[serde(
        rename = "httpErrorStatusCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_error_status_code: Option<i32>,
    /// [Output Only] The unique identifier for the operation. This identifier is defined by the server.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// [Output Only] The time that this operation was requested. This value is in RFC3339 text format.
    #[serde(rename = "insertTime", skip_serializing_if = "Option::is_none")]
    pub insert_time: Option<String>,
    #[serde(
        rename = "instancesBulkInsertOperationMetadata",
        skip_serializing_if = "Option::is_none"
    )]
    pub instances_bulk_insert_operation_metadata:
        Option<Box<models::InstancesBulkInsertOperationMetadata>>,
    /// [Output Only] Type of the resource. Always `compute#operation` for Operation resources.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// [Output Only] Name of the operation.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// [Output Only] An ID that represents a group of operations, such as when a group of operations results from a `bulkInsert` API request.
    #[serde(rename = "operationGroupId", skip_serializing_if = "Option::is_none")]
    pub operation_group_id: Option<String>,
    /// [Output Only] The type of operation, such as `insert`, `update`, or `delete`, and so on.
    #[serde(rename = "operationType", skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    /// [Output Only] An optional progress indicator that ranges from 0 to 100. There is no requirement that this be linear or support any granularity of operations. This should not be used to guess when the operation will be complete. This number should monotonically increase as the operation progresses.
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
    /// [Output Only] The URL of the region where the operation resides. Only applicable when performing regional operations.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// [Output Only] Server-defined URL for the resource.
    #[serde(rename = "selfLink", skip_serializing_if = "Option::is_none")]
    pub self_link: Option<String>,
    #[serde(
        rename = "setCommonInstanceMetadataOperationMetadata",
        skip_serializing_if = "Option::is_none"
    )]
    pub set_common_instance_metadata_operation_metadata:
        Option<Box<models::SetCommonInstanceMetadataOperationMetadata>>,
    /// [Output Only] The time that this operation was started by the server. This value is in RFC3339 text format.
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// [Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// [Output Only] An optional textual description of the current status of the operation.
    #[serde(rename = "statusMessage", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// [Output Only] The unique target ID, which identifies a specific incarnation of the target resource.
    #[serde(rename = "targetId", skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// [Output Only] The URL of the resource that the operation modifies. For operations related to creating a snapshot, this points to the persistent disk that the snapshot was created from.
    #[serde(rename = "targetLink", skip_serializing_if = "Option::is_none")]
    pub target_link: Option<String>,
    /// [Output Only] User who requested the operation, for example: `user@example.com` or `alice_smith_identifier (global/workforcePools/example-com-us-employees)`.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// [Output Only] If warning messages are generated during processing of the operation, this field will be populated.
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<models::OperationWarningsInner>>,
    /// [Output Only] The URL of the zone where the operation resides. Only applicable when performing per-zone operations.
    #[serde(rename = "zone", skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

impl Operation {
    /// Represents an Operation resource. Google Compute Engine has three Operation resources: * [Global](/compute/docs/reference/rest/v1/globalOperations) * [Regional](/compute/docs/reference/rest/v1/regionOperations) * [Zonal](/compute/docs/reference/rest/v1/zoneOperations) You can use an operation resource to manage asynchronous API requests. For more information, read Handling API responses. Operations can be global, regional or zonal. - For global operations, use the `globalOperations` resource. - For regional operations, use the `regionOperations` resource. - For zonal operations, use the `zoneOperations` resource. For more information, read Global, Regional, and Zonal Resources. Note that completed Operation resources have a limited retention period.
    pub fn new() -> Operation {
        Operation {
            client_operation_id: None,
            creation_timestamp: None,
            description: None,
            end_time: None,
            error: None,
            http_error_message: None,
            http_error_status_code: None,
            id: None,
            insert_time: None,
            instances_bulk_insert_operation_metadata: None,
            kind: None,
            name: None,
            operation_group_id: None,
            operation_type: None,
            progress: None,
            region: None,
            self_link: None,
            set_common_instance_metadata_operation_metadata: None,
            start_time: None,
            status: None,
            status_message: None,
            target_id: None,
            target_link: None,
            user: None,
            warnings: None,
            zone: None,
        }
    }
}
/// [Output Only] The status of the operation, which can be one of the following: `PENDING`, `RUNNING`, or `DONE`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "RUNNING")]
    Running,
}

impl Default for Status {
    fn default() -> Status {
        Self::Done
    }
}
