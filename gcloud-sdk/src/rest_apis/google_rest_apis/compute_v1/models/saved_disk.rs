use serde::{Deserialize, Serialize}; /*
                                      * Compute Engine API
                                      *
                                      * Creates and runs virtual machines on Google Cloud Platform.
                                      *
                                      * The version of the OpenAPI document: v1
                                      *
                                      * Generated by: https://openapi-generator.tech
                                      */

/// SavedDisk : An instance-attached disk resource.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SavedDisk {
    /// [Output Only] The architecture of the attached disk.
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Architecture>,
    /// [Output Only] Type of the resource. Always compute#savedDisk for attached disks.
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Specifies a URL of the disk attached to the source instance.
    #[serde(rename = "sourceDisk", skip_serializing_if = "Option::is_none")]
    pub source_disk: Option<String>,
    /// [Output Only] Size of the individual disk snapshot used by this machine image.
    #[serde(rename = "storageBytes", skip_serializing_if = "Option::is_none")]
    pub storage_bytes: Option<String>,
    /// [Output Only] An indicator whether storageBytes is in a stable state or it is being adjusted as a result of shared storage reallocation. This status can either be UPDATING, meaning the size of the snapshot is being updated, or UP_TO_DATE, meaning the size of the snapshot is up-to-date.
    #[serde(rename = "storageBytesStatus", skip_serializing_if = "Option::is_none")]
    pub storage_bytes_status: Option<StorageBytesStatus>,
}

impl SavedDisk {
    /// An instance-attached disk resource.
    pub fn new() -> SavedDisk {
        SavedDisk {
            architecture: None,
            kind: None,
            source_disk: None,
            storage_bytes: None,
            storage_bytes_status: None,
        }
    }
}

/// [Output Only] The architecture of the attached disk.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Architecture {
    #[serde(rename = "ARCHITECTURE_UNSPECIFIED")]
    ArchitectureUnspecified,
    #[serde(rename = "ARM64")]
    Arm64,
    #[serde(rename = "X86_64")]
    X8664,
}

impl Default for Architecture {
    fn default() -> Architecture {
        Self::ArchitectureUnspecified
    }
}
/// [Output Only] An indicator whether storageBytes is in a stable state or it is being adjusted as a result of shared storage reallocation. This status can either be UPDATING, meaning the size of the snapshot is being updated, or UP_TO_DATE, meaning the size of the snapshot is up-to-date.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StorageBytesStatus {
    #[serde(rename = "UPDATING")]
    Updating,
    #[serde(rename = "UP_TO_DATE")]
    UpToDate,
}

impl Default for StorageBytesStatus {
    fn default() -> StorageBytesStatus {
        Self::Updating
    }
}