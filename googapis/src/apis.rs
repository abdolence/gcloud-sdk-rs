//! This library generated from [Google API][`googleapis`] using [tonic-build][`tonic-build`].
//!
//! ## Example
//! The complete code can be found [here][`spanner-admin-example`].
//! ```
//!
//! [`googleapis`]: https://github.com/googleapis/googleapis
//! [`tonic-build`]: https://github.com/hyperium/tonic/tree/master/tonic-build
//! [`spanner-admin-example`]: https://github.com/mechiru/googapis/tree/master/examples/spanner-admin

/// The minimal google root set downloaded from https://pki.goog/roots.pem.
/// ````
pub const CERTIFICATES: &[u8] = include_bytes!("../data/roots.pem");

#[allow(unused_macros)]
macro_rules! include_proto {
    ($package: tt) => {
        include!(concat!("../genproto", concat!("/", $package, ".rs")));
    };
}

include!("googapis.rs");
