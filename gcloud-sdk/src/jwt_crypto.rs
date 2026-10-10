//! The crypto provider `jsonwebtoken` signs and verifies with, chosen by the
//! `jwt-aws-lc-rs` and `jwt-rust-crypto` features.

/// Makes the provider of the enabled `jwt-*` feature the process default of
/// `jsonwebtoken`, unless the application installed one already; with both features,
/// aws-lc-rs. Call it before every `jsonwebtoken` signature or verification.
///
/// Without either feature it does nothing, so the provider the application installed,
/// through its own `jsonwebtoken` features or `install_default`, is the one used.
pub(crate) fn ensure_provider() {
    // An `Err` from `install_default` means a provider is installed already, the
    // application's or ours.
    #[cfg(feature = "jwt-aws-lc-rs")]
    let _ = jsonwebtoken::crypto::aws_lc::DEFAULT_PROVIDER.install_default();
    #[cfg(all(feature = "jwt-rust-crypto", not(feature = "jwt-aws-lc-rs")))]
    let _ = jsonwebtoken::crypto::rust_crypto::DEFAULT_PROVIDER.install_default();
}
