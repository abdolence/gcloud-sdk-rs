//! The crypto provider `jsonwebtoken` signs and verifies with, chosen by the
//! `jwt-aws-lc-rs` and `jwt-rust-crypto` features.

/// Makes the provider of the enabled `jwt-*` feature the process default of
/// `jsonwebtoken`, unless the application installed one already; with both features,
/// aws-lc-rs. Call it before every `jsonwebtoken` signature or verification: without
/// either feature it fails with [`ErrorKind::JwtCryptoProviderMissing`], where
/// `jsonwebtoken` would panic.
///
/// [`ErrorKind::JwtCryptoProviderMissing`]: crate::error::ErrorKind::JwtCryptoProviderMissing
pub(crate) fn ensure_provider() -> crate::error::Result<()> {
    #[cfg(feature = "jwt-aws-lc-rs")]
    let provider = Some(&jsonwebtoken::crypto::aws_lc::DEFAULT_PROVIDER);
    #[cfg(all(feature = "jwt-rust-crypto", not(feature = "jwt-aws-lc-rs")))]
    let provider = Some(&jsonwebtoken::crypto::rust_crypto::DEFAULT_PROVIDER);
    #[cfg(not(any(feature = "jwt-aws-lc-rs", feature = "jwt-rust-crypto")))]
    let provider: Option<&'static jsonwebtoken::crypto::CryptoProvider> = None;

    match provider {
        Some(provider) => {
            // An `Err` means a provider is installed already, the application's or ours.
            let _ = provider.install_default();
            Ok(())
        }
        None => Err(crate::error::ErrorKind::JwtCryptoProviderMissing.into()),
    }
}
