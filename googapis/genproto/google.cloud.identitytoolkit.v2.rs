/// The information required to auto-retrieve an SMS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoRetrievalInfo {
    /// The Android app's signature hash for Google Play Service's
    /// SMS Retriever API.
    #[prost(string, tag = "1")]
    pub app_signature_hash: ::prost::alloc::string::String,
}
/// App Verification info for a StartMfa request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMfaPhoneRequestInfo {
    /// Required for enrollment. Phone number to be enrolled as MFA.
    #[prost(string, tag = "1")]
    pub phone_number: ::prost::alloc::string::String,
    /// iOS only. Receipt of successful app token validation with APNS.
    #[prost(string, tag = "2")]
    pub ios_receipt: ::prost::alloc::string::String,
    /// iOS only. Secret delivered to iOS app via APNS.
    #[prost(string, tag = "3")]
    pub ios_secret: ::prost::alloc::string::String,
    /// Web only. Recaptcha solution.
    #[prost(string, tag = "4")]
    pub recaptcha_token: ::prost::alloc::string::String,
    /// Android only. Used by Google Play Services to identify the app for
    /// auto-retrieval.
    #[prost(message, optional, tag = "5")]
    pub auto_retrieval_info: ::core::option::Option<AutoRetrievalInfo>,
    /// Android only. Used to assert application identity in place of a
    /// recaptcha token. A SafetyNet Token can be generated via the
    /// [SafetyNet Android Attestation
    /// API](<https://developer.android.com/training/safetynet/attestation.html>),
    /// with the Base64 encoding of the `phone_number` field as the nonce.
    #[prost(string, tag = "6")]
    pub safety_net_token: ::prost::alloc::string::String,
}
/// Phone Verification info for a StartMfa response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMfaPhoneResponseInfo {
    /// An opaque string that represents the enrollment session.
    #[prost(string, tag = "1")]
    pub session_info: ::prost::alloc::string::String,
}
/// Phone Verification info for a FinalizeMfa request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeMfaPhoneRequestInfo {
    /// An opaque string that represents the enrollment session.
    #[prost(string, tag = "1")]
    pub session_info: ::prost::alloc::string::String,
    /// User-entered verification code.
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// Android only. Uses for "instant" phone number verification though GmsCore.
    #[prost(string, tag = "3")]
    pub android_verification_proof: ::prost::alloc::string::String,
    /// Required if Android verification proof is presented.
    #[prost(string, tag = "4")]
    pub phone_number: ::prost::alloc::string::String,
}
/// Phone Verification info for a FinalizeMfa response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeMfaPhoneResponseInfo {
    /// Android only. Long-lived replacement for valid code tied to android device.
    #[prost(string, tag = "1")]
    pub android_verification_proof: ::prost::alloc::string::String,
    /// Android only. Expiration time of verification proof in seconds.
    #[prost(message, optional, tag = "2")]
    pub android_verification_proof_expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// For Android verification proof.
    #[prost(string, tag = "3")]
    pub phone_number: ::prost::alloc::string::String,
}
/// Finishes enrolling a second factor for the user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeMfaEnrollmentRequest {
    /// Required. ID token.
    #[prost(string, tag = "1")]
    pub id_token: ::prost::alloc::string::String,
    /// Display name which is entered  by users to distinguish between different
    /// second factors with same type or different type.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// The ID of the Identity Platform tenant that the user enrolling MFA belongs
    /// to. If not set, the user belongs to the default Identity Platform project.
    #[prost(string, tag = "5")]
    pub tenant_id: ::prost::alloc::string::String,
    /// MFA enrollment information to be verified.
    #[prost(
        oneof = "finalize_mfa_enrollment_request::VerificationInfo",
        tags = "4"
    )]
    pub verification_info:
        ::core::option::Option<finalize_mfa_enrollment_request::VerificationInfo>,
}
/// Nested message and enum types in `FinalizeMfaEnrollmentRequest`.
pub mod finalize_mfa_enrollment_request {
    /// MFA enrollment information to be verified.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VerificationInfo {
        /// Verification info to authorize sending an SMS for phone verification.
        #[prost(message, tag = "4")]
        PhoneVerificationInfo(super::FinalizeMfaPhoneRequestInfo),
    }
}
/// FinalizeMfaEnrollment response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeMfaEnrollmentResponse {
    /// ID token updated to reflect MFA enrollment.
    #[prost(string, tag = "1")]
    pub id_token: ::prost::alloc::string::String,
    /// Refresh token updated to reflect MFA enrollment.
    #[prost(string, tag = "2")]
    pub refresh_token: ::prost::alloc::string::String,
    /// MFA verified enrollment information.
    #[prost(
        oneof = "finalize_mfa_enrollment_response::AuxiliaryAuthInfo",
        tags = "3"
    )]
    pub auxiliary_auth_info:
        ::core::option::Option<finalize_mfa_enrollment_response::AuxiliaryAuthInfo>,
}
/// Nested message and enum types in `FinalizeMfaEnrollmentResponse`.
pub mod finalize_mfa_enrollment_response {
    /// MFA verified enrollment information.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuxiliaryAuthInfo {
        #[prost(message, tag = "3")]
        PhoneAuthInfo(super::FinalizeMfaPhoneResponseInfo),
    }
}
/// Sends MFA enrollment verification SMS for a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMfaEnrollmentRequest {
    /// Required. User's ID token.
    #[prost(string, tag = "1")]
    pub id_token: ::prost::alloc::string::String,
    /// The ID of the Identity Platform tenant that the user enrolling MFA belongs
    /// to. If not set, the user belongs to the default Identity Platform project.
    #[prost(string, tag = "4")]
    pub tenant_id: ::prost::alloc::string::String,
    /// MFA information by type of 2nd factor.
    #[prost(oneof = "start_mfa_enrollment_request::EnrollmentInfo", tags = "3")]
    pub enrollment_info: ::core::option::Option<start_mfa_enrollment_request::EnrollmentInfo>,
}
/// Nested message and enum types in `StartMfaEnrollmentRequest`.
pub mod start_mfa_enrollment_request {
    /// MFA information by type of 2nd factor.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EnrollmentInfo {
        /// Verification info to authorize sending an SMS for phone verification.
        #[prost(message, tag = "3")]
        PhoneEnrollmentInfo(super::StartMfaPhoneRequestInfo),
    }
}
/// StartMfaEnrollment response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMfaEnrollmentResponse {
    /// MFA start enrollment response by 2nd factor type.
    #[prost(
        oneof = "start_mfa_enrollment_response::EnrollmentResponse",
        tags = "1"
    )]
    pub enrollment_response:
        ::core::option::Option<start_mfa_enrollment_response::EnrollmentResponse>,
}
/// Nested message and enum types in `StartMfaEnrollmentResponse`.
pub mod start_mfa_enrollment_response {
    /// MFA start enrollment response by 2nd factor type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EnrollmentResponse {
        /// Verification info to authorize sending an SMS for phone verification.
        #[prost(message, tag = "1")]
        PhoneSessionInfo(super::StartMfaPhoneResponseInfo),
    }
}
/// Withdraws MFA.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawMfaRequest {
    /// Required. User's ID token.
    #[prost(string, tag = "1")]
    pub id_token: ::prost::alloc::string::String,
    /// Required. MFA enrollment id from a current MFA enrollment.
    #[prost(string, tag = "2")]
    pub mfa_enrollment_id: ::prost::alloc::string::String,
    /// The ID of the Identity Platform tenant that the user unenrolling MFA
    /// belongs to. If not set, the user belongs to the default Identity Platform
    /// project.
    #[prost(string, tag = "3")]
    pub tenant_id: ::prost::alloc::string::String,
}
/// Withdraws MultiFactorAuth response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawMfaResponse {
    /// ID token updated to reflect removal of the second factor.
    #[prost(string, tag = "1")]
    pub id_token: ::prost::alloc::string::String,
    /// Refresh token updated to reflect removal of the second factor.
    #[prost(string, tag = "2")]
    pub refresh_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod account_management_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Account management for Identity Toolkit"]
    #[derive(Debug, Clone)]
    pub struct AccountManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccountManagementServiceClient<T>
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
        ) -> AccountManagementServiceClient<InterceptedService<T, F>>
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
            AccountManagementServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Finishes enrolling a second factor for the user."]
        pub async fn finalize_mfa_enrollment(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeMfaEnrollmentRequest>,
        ) -> Result<tonic::Response<super::FinalizeMfaEnrollmentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.identitytoolkit.v2.AccountManagementService/FinalizeMfaEnrollment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Step one of the MFA enrollment process. In SMS case, this sends an"]
        #[doc = " SMS verification code to the user."]
        pub async fn start_mfa_enrollment(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMfaEnrollmentRequest>,
        ) -> Result<tonic::Response<super::StartMfaEnrollmentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.identitytoolkit.v2.AccountManagementService/StartMfaEnrollment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Revokes one second factor from the enrolled second factors for an account."]
        pub async fn withdraw_mfa(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawMfaRequest>,
        ) -> Result<tonic::Response<super::WithdrawMfaResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.identitytoolkit.v2.AccountManagementService/WithdrawMfa",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Finalizes sign-in by verifying MFA challenge.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeMfaSignInRequest {
    /// Required. Pending credential from first factor sign-in.
    #[prost(string, tag = "2")]
    pub mfa_pending_credential: ::prost::alloc::string::String,
    /// The ID of the Identity Platform tenant the user is signing in to. If not
    /// set, the user will sign in to the default Identity Platform project.
    #[prost(string, tag = "4")]
    pub tenant_id: ::prost::alloc::string::String,
    /// Proof of completion of the MFA challenge.
    #[prost(oneof = "finalize_mfa_sign_in_request::VerificationInfo", tags = "3")]
    pub verification_info: ::core::option::Option<finalize_mfa_sign_in_request::VerificationInfo>,
}
/// Nested message and enum types in `FinalizeMfaSignInRequest`.
pub mod finalize_mfa_sign_in_request {
    /// Proof of completion of the MFA challenge.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VerificationInfo {
        /// Proof of completion of the SMS based MFA challenge.
        #[prost(message, tag = "3")]
        PhoneVerificationInfo(super::FinalizeMfaPhoneRequestInfo),
    }
}
/// FinalizeMfaSignIn response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeMfaSignInResponse {
    /// ID token for the authenticated user.
    #[prost(string, tag = "1")]
    pub id_token: ::prost::alloc::string::String,
    /// Refresh token for the authenticated user.
    #[prost(string, tag = "2")]
    pub refresh_token: ::prost::alloc::string::String,
    /// MFA verified sign-in information.
    #[prost(oneof = "finalize_mfa_sign_in_response::AuxiliaryAuthInfo", tags = "3")]
    pub auxiliary_auth_info:
        ::core::option::Option<finalize_mfa_sign_in_response::AuxiliaryAuthInfo>,
}
/// Nested message and enum types in `FinalizeMfaSignInResponse`.
pub mod finalize_mfa_sign_in_response {
    /// MFA verified sign-in information.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuxiliaryAuthInfo {
        /// Extra phone auth info, including android verification proof.
        #[prost(message, tag = "3")]
        PhoneAuthInfo(super::FinalizeMfaPhoneResponseInfo),
    }
}
/// Starts multi-factor sign-in by sending the multi-factor auth challenge.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMfaSignInRequest {
    /// Required. Pending credential from first factor sign-in.
    #[prost(string, tag = "2")]
    pub mfa_pending_credential: ::prost::alloc::string::String,
    /// Required. MFA enrollment id from the user's list of current MFA enrollments.
    #[prost(string, tag = "3")]
    pub mfa_enrollment_id: ::prost::alloc::string::String,
    /// The ID of the Identity Platform tenant the user is signing in to. If not
    /// set, the user will sign in to the default Identity Platform project.
    #[prost(string, tag = "5")]
    pub tenant_id: ::prost::alloc::string::String,
    /// MFA information by type of 2nd factor.
    #[prost(oneof = "start_mfa_sign_in_request::SignInInfo", tags = "4")]
    pub sign_in_info: ::core::option::Option<start_mfa_sign_in_request::SignInInfo>,
}
/// Nested message and enum types in `StartMfaSignInRequest`.
pub mod start_mfa_sign_in_request {
    /// MFA information by type of 2nd factor.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SignInInfo {
        /// Verification info to authorize sending an SMS for phone verification.
        #[prost(message, tag = "4")]
        PhoneSignInInfo(super::StartMfaPhoneRequestInfo),
    }
}
/// StartMfaSignIn response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMfaSignInResponse {
    /// MultiFactor start sign-in response by 2nd factor type.
    #[prost(oneof = "start_mfa_sign_in_response::ResponseInfo", tags = "1")]
    pub response_info: ::core::option::Option<start_mfa_sign_in_response::ResponseInfo>,
}
/// Nested message and enum types in `StartMfaSignInResponse`.
pub mod start_mfa_sign_in_response {
    /// MultiFactor start sign-in response by 2nd factor type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseInfo {
        /// MultiFactor sign-in session information specific to SMS-type second
        /// factors. Along with the one-time code retrieved from the sent SMS, the
        /// contents of this session information should be passed to
        /// FinalizeMfaSignIn to complete the sign in.
        #[prost(message, tag = "1")]
        PhoneResponseInfo(super::StartMfaPhoneResponseInfo),
    }
}
#[doc = r" Generated client implementations."]
pub mod authentication_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Authentication for Identity Toolkit"]
    #[derive(Debug, Clone)]
    pub struct AuthenticationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AuthenticationServiceClient<T>
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
        ) -> AuthenticationServiceClient<InterceptedService<T, F>>
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
            AuthenticationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Verifies the MFA challenge and performs sign-in"]
        pub async fn finalize_mfa_sign_in(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeMfaSignInRequest>,
        ) -> Result<tonic::Response<super::FinalizeMfaSignInResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.identitytoolkit.v2.AuthenticationService/FinalizeMfaSignIn",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sends the MFA challenge"]
        pub async fn start_mfa_sign_in(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMfaSignInRequest>,
        ) -> Result<tonic::Response<super::StartMfaSignInResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.identitytoolkit.v2.AuthenticationService/StartMfaSignIn",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
