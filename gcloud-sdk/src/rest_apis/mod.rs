#![allow(
    unused_imports,
    clippy::redundant_clone,
    clippy::large_enum_variant,
    clippy::too_many_arguments,
    clippy::derive_partial_eq_without_eq,
    clippy::manual_non_exhaustive
)] // for generated files

mod rest_api_client;
pub use rest_api_client::*;

pub mod google_rest_apis;

#[cfg(all(test, feature = "google-rest-fcm-v1"))]
mod tests {
    use super::google_rest_apis::fcm_v1::{urlencode, urlencode_path};

    #[test]
    fn urlencode_path_preserves_resource_name_slashes() {
        assert_eq!(
            urlencode_path("projects/my-project-id"),
            "projects/my-project-id"
        );
        assert_eq!(urlencode_path("projects/a b"), "projects/a+b");
        // plain urlencode still encodes slashes (required for e.g. GCS object names)
        assert_eq!(
            urlencode("projects/my-project-id"),
            "projects%2Fmy-project-id"
        );
    }
}
