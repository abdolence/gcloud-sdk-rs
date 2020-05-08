/// The create assessment request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAssessmentRequest {
    /// Required. The name of the project in which the assessment will be created,
    /// in the format "projects/{project_number}".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The assessment details.
    #[prost(message, optional, tag = "2")]
    pub assessment: ::std::option::Option<Assessment>,
}
/// The request message to annotate an Assessment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateAssessmentRequest {
    /// Required. The resource name of the Assessment, in the format
    /// "projects/{project_number}/assessments/{assessment_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The annotation that will be assigned to the Event.
    #[prost(enumeration = "annotate_assessment_request::Annotation", tag = "2")]
    pub annotation: i32,
}
pub mod annotate_assessment_request {
    /// Enum that reprensents the types of annotations.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Annotation {
        /// Default unspecified type.
        Unspecified = 0,
        /// Provides information that the event turned out to be legitimate.
        Legitimate = 1,
        /// Provides information that the event turned out to be fraudulent.
        Fraudulent = 2,
    }
}
/// Empty response for AnnotateAssessment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateAssessmentResponse {}
/// A recaptcha assessment resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assessment {
    /// Output only. The resource name for the Assessment in the format
    /// "projects/{project_number}/assessments/{assessment_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The event being assessed.
    #[prost(message, optional, tag = "2")]
    pub event: ::std::option::Option<Event>,
    /// Output only. Legitimate event score from 0.0 to 1.0.
    /// (1.0 means very likely legitimate traffic while 0.0 means very likely
    /// non-legitimate traffic).
    #[prost(float, tag = "3")]
    pub score: f32,
    /// Output only. Properties of the provided event token.
    #[prost(message, optional, tag = "4")]
    pub token_properties: ::std::option::Option<TokenProperties>,
    /// Output only. Reasons contributing to the risk analysis verdict.
    #[prost(
        enumeration = "assessment::ClassificationReason",
        repeated,
        packed = "false",
        tag = "5"
    )]
    pub reasons: ::std::vec::Vec<i32>,
}
pub mod assessment {
    /// LINT.IfChange(classification_reason)
    /// Reasons contributing to the risk analysis verdict.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClassificationReason {
        /// Default unspecified type.
        Unspecified = 0,
        /// Interactions matched the behavior of an automated agent.
        Automation = 1,
        /// The event originated from an illegitimate environment.
        UnexpectedEnvironment = 2,
        /// Traffic volume from the event source is higher than normal.
        TooMuchTraffic = 3,
        /// Interactions with the site were significantly different than expected
        /// patterns.
        UnexpectedUsagePatterns = 4,
        /// Too little traffic has been received from this site thus far to generate
        /// quality risk analysis.
        LowConfidenceScore = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Optional. The user response token provided by the reCAPTCHA client-side integration
    /// on your site.
    #[prost(string, tag = "1")]
    pub token: std::string::String,
    /// Optional. The site key that was used to invoke reCAPTCHA on your site and generate
    /// the token.
    #[prost(string, tag = "2")]
    pub site_key: std::string::String,
    /// Optional. The user agent present in the request from the user's device related to
    /// this event.
    #[prost(string, tag = "3")]
    pub user_agent: std::string::String,
    /// Optional. The IP address in the request from the user's device related to this event.
    #[prost(string, tag = "4")]
    pub user_ip_address: std::string::String,
    /// Optional. The expected action for this type of event. This should be the same action
    /// provided at token generation time on client-side platforms already
    /// integrated with recaptcha enterprise.
    #[prost(string, tag = "5")]
    pub expected_action: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenProperties {
    /// Whether the provided user response token is valid.
    #[prost(bool, tag = "1")]
    pub valid: bool,
    /// Reason associated with the response when valid = false.
    #[prost(enumeration = "token_properties::InvalidReason", tag = "2")]
    pub invalid_reason: i32,
    /// The timestamp corresponding to the generation of the token.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The hostname of the page on which the token was generated.
    #[prost(string, tag = "4")]
    pub hostname: std::string::String,
    /// Action name provided at token generation.
    #[prost(string, tag = "5")]
    pub action: std::string::String,
}
pub mod token_properties {
    /// LINT.IfChange
    /// Enum that represents the types of invalid token reasons.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InvalidReason {
        /// Default unspecified type.
        Unspecified = 0,
        /// If the failure reason was not accounted for.
        UnknownInvalidReason = 1,
        /// The provided user verification token was malformed.
        Malformed = 2,
        /// The user verification token had expired.
        Expired = 3,
        /// The user verification had already been seen.
        Dupe = 4,
        /// The user verification token did not match the provided site key.
        /// This may be a configuration error (e.g. development keys used in
        /// production) or end users trying to use verification tokens from other
        /// sites.
        SiteMismatch = 5,
        /// The user verification token was not present.  It is a required input.
        Missing = 6,
    }
}
/// The create key request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyRequest {
    /// Required. The name of the project in which the key will be created, in the
    /// format "projects/{project_number}".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Information to create a reCAPTCHA Enterprise key.
    #[prost(message, optional, tag = "2")]
    pub key: ::std::option::Option<Key>,
}
/// The list keys request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeysRequest {
    /// Required. The name of the project that contains the keys that will be
    /// listed, in the format "projects/{project_number}".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of keys to return. Default is 10. Max limit is
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous.
    /// ListKeysRequest, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response to request to list keys in a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListKeysResponse {
    /// Key details.
    #[prost(message, repeated, tag = "1")]
    pub keys: ::std::vec::Vec<Key>,
    /// Token to retrieve the next page of results. It is set to empty if no keys
    /// remain in results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The get key request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyRequest {
    /// Required. The name of the requested key, in the format
    /// "projects/{project_number}/keys/{key_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The update key request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateKeyRequest {
    /// Required. The key to update.
    #[prost(message, optional, tag = "1")]
    pub key: ::std::option::Option<Key>,
    /// Optional. The mask to control which field of the key get updated. If the mask is not
    /// present, all fields will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The delete key request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeyRequest {
    /// Required. The name of the key to be deleted, in the format
    /// "projects/{project_number}/keys/{key_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A key used to identify and configure applications (web and/or mobile) that
/// use reCAPTCHA Enterprise.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    /// The resource name for the Key in the format
    /// "projects/{project_number}/keys/{key_id}".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Human-readable display name of this key. Modifiable by user.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Platform specific settings for this key. The key can only be used on one
    /// platform, the one it has settings for.
    #[prost(oneof = "key::PlatformSettings", tags = "3, 4, 5")]
    pub platform_settings: ::std::option::Option<key::PlatformSettings>,
}
pub mod key {
    /// Platform specific settings for this key. The key can only be used on one
    /// platform, the one it has settings for.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PlatformSettings {
        /// Settings for keys that can be used by websites.
        #[prost(message, tag = "3")]
        WebSettings(super::WebKeySettings),
        /// Settings for keys that can be used by Android apps.
        #[prost(message, tag = "4")]
        AndroidSettings(super::AndroidKeySettings),
        /// Settings for keys that can be used by iOS apps.
        #[prost(message, tag = "5")]
        IosSettings(super::IosKeySettings),
    }
}
/// Settings specific to keys that can be used by websites.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebKeySettings {
    /// Whether allowed_domains is enforced or not.
    #[prost(bool, tag = "3")]
    pub enforce_allowed_domains: bool,
    /// Domains or subdomains of websites allowed to use the key. All subdomains
    /// of an allowed domain are automatically allowed. A valid domain requires a
    /// host and must not include any path, port, query or fragment.
    /// Examples: 'example.com' or 'subdomain.example.com'
    #[prost(string, repeated, tag = "1")]
    pub allowed_domains: ::std::vec::Vec<std::string::String>,
    /// Whether this key can be used on AMP (Accelerated Mobile Pages) websites.
    #[prost(bool, tag = "2")]
    pub allow_amp_traffic: bool,
    /// Required. Describes how this key is integrated with the website.
    #[prost(enumeration = "web_key_settings::IntegrationType", tag = "4")]
    pub integration_type: i32,
    /// Settings for the frequency and difficulty at which this key triggers
    /// captcha challenges. This should only be specified for IntegrationTypes
    /// CHECKBOX_CHALLENGE and INVISIBLE_CHALLENGE.
    #[prost(
        enumeration = "web_key_settings::ChallengeSecurityPreference",
        tag = "5"
    )]
    pub challenge_security_preference: i32,
}
pub mod web_key_settings {
    /// Enum that represents the integration types for web keys.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IntegrationType {
        /// Default type that indicates this enum hasn't been specified. This is not
        /// a valid IntegrationType, one of the other types must be specified
        /// instead.
        Unspecified = 0,
        /// Only used to produce scores. It doesn't display the "I'm not a robot"
        /// checkbox and never shows captcha challenges.
        ScoreOnly = 1,
        /// Displays the "I'm not a robot" checkbox and may show captcha challenges
        /// after it is checked.
        CheckboxChallenge = 2,
        /// Doesn't display the "I'm not a robot" checkbox, but may show captcha
        /// challenges after risk analysis.
        InvisibleChallenge = 3,
    }
    /// Enum that represents the possible challenge frequency and difficulty
    /// configurations for a web key.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChallengeSecurityPreference {
        /// Default type that indicates this enum hasn't been specified.
        Unspecified = 0,
        /// Key tends to show fewer and easier challenges.
        Usability = 1,
        /// Key tends to show balanced (in amount and difficulty) challenges.
        Balanced = 2,
        /// Key tends to show more and harder challenges.
        Security = 3,
    }
}
/// Settings specific to keys that can be used by Android apps.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidKeySettings {
    /// Android package names of apps allowed to use the key.
    /// Example: 'com.companyname.appname'
    #[prost(string, repeated, tag = "1")]
    pub allowed_package_names: ::std::vec::Vec<std::string::String>,
}
/// Settings specific to keys that can be used by iOS apps.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosKeySettings {
    /// iOS bundle ids of apps allowed to use the key.
    /// Example: 'com.companyname.productname.appname'
    #[prost(string, repeated, tag = "1")]
    pub allowed_bundle_ids: ::std::vec::Vec<std::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod recaptcha_enterprise_service_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to determine the likelihood an event is legitimate."]
    pub struct RecaptchaEnterpriseServiceV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RecaptchaEnterpriseServiceV1Beta1Client<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RecaptchaEnterpriseServiceV1Beta1Client<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates an Assessment of the likelihood an event is legitimate."]
        pub async fn create_assessment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAssessmentRequest>,
        ) -> Result<tonic::Response<super::Assessment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/CreateAssessment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Annotates a previously created Assessment to provide additional information"]
        #[doc = " on whether the event turned out to be authentic or fradulent."]
        pub async fn annotate_assessment(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnotateAssessmentRequest>,
        ) -> Result<tonic::Response<super::AnnotateAssessmentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/AnnotateAssessment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new reCAPTCHA Enterprise key."]
        pub async fn create_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKeyRequest>,
        ) -> Result<tonic::Response<super::Key>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/CreateKey" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of all keys that belong to a project."]
        pub async fn list_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKeysRequest>,
        ) -> Result<tonic::Response<super::ListKeysResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/ListKeys" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the specified key."]
        pub async fn get_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeyRequest>,
        ) -> Result<tonic::Response<super::Key>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/GetKey" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified key."]
        pub async fn update_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateKeyRequest>,
        ) -> Result<tonic::Response<super::Key>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/UpdateKey" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified key."]
        pub async fn delete_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteKeyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/DeleteKey" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RecaptchaEnterpriseServiceV1Beta1Client<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RecaptchaEnterpriseServiceV1Beta1Client<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RecaptchaEnterpriseServiceV1Beta1Client {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod recaptcha_enterprise_service_v1_beta1_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RecaptchaEnterpriseServiceV1Beta1Server."]
    #[async_trait]
    pub trait RecaptchaEnterpriseServiceV1Beta1: Send + Sync + 'static {
        #[doc = " Creates an Assessment of the likelihood an event is legitimate."]
        async fn create_assessment(
            &self,
            request: tonic::Request<super::CreateAssessmentRequest>,
        ) -> Result<tonic::Response<super::Assessment>, tonic::Status>;
        #[doc = " Annotates a previously created Assessment to provide additional information"]
        #[doc = " on whether the event turned out to be authentic or fradulent."]
        async fn annotate_assessment(
            &self,
            request: tonic::Request<super::AnnotateAssessmentRequest>,
        ) -> Result<tonic::Response<super::AnnotateAssessmentResponse>, tonic::Status>;
        #[doc = " Creates a new reCAPTCHA Enterprise key."]
        async fn create_key(
            &self,
            request: tonic::Request<super::CreateKeyRequest>,
        ) -> Result<tonic::Response<super::Key>, tonic::Status>;
        #[doc = " Returns the list of all keys that belong to a project."]
        async fn list_keys(
            &self,
            request: tonic::Request<super::ListKeysRequest>,
        ) -> Result<tonic::Response<super::ListKeysResponse>, tonic::Status>;
        #[doc = " Returns the specified key."]
        async fn get_key(
            &self,
            request: tonic::Request<super::GetKeyRequest>,
        ) -> Result<tonic::Response<super::Key>, tonic::Status>;
        #[doc = " Updates the specified key."]
        async fn update_key(
            &self,
            request: tonic::Request<super::UpdateKeyRequest>,
        ) -> Result<tonic::Response<super::Key>, tonic::Status>;
        #[doc = " Deletes the specified key."]
        async fn delete_key(
            &self,
            request: tonic::Request<super::DeleteKeyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " Service to determine the likelihood an event is legitimate."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct RecaptchaEnterpriseServiceV1Beta1Server<T: RecaptchaEnterpriseServiceV1Beta1> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: RecaptchaEnterpriseServiceV1Beta1> RecaptchaEnterpriseServiceV1Beta1Server<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for RecaptchaEnterpriseServiceV1Beta1Server<T>
    where
        T: RecaptchaEnterpriseServiceV1Beta1,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req . uri ( ) . path ( ) { "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/CreateAssessment" => { # [ allow ( non_camel_case_types ) ] struct CreateAssessmentSvc < T : RecaptchaEnterpriseServiceV1Beta1 > ( pub Arc < T > ) ; impl < T : RecaptchaEnterpriseServiceV1Beta1 > tonic :: server :: UnaryService < super :: CreateAssessmentRequest > for CreateAssessmentSvc < T > { type Response = super :: Assessment ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateAssessmentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_assessment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateAssessmentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/AnnotateAssessment" => { # [ allow ( non_camel_case_types ) ] struct AnnotateAssessmentSvc < T : RecaptchaEnterpriseServiceV1Beta1 > ( pub Arc < T > ) ; impl < T : RecaptchaEnterpriseServiceV1Beta1 > tonic :: server :: UnaryService < super :: AnnotateAssessmentRequest > for AnnotateAssessmentSvc < T > { type Response = super :: AnnotateAssessmentResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: AnnotateAssessmentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . annotate_assessment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = AnnotateAssessmentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/CreateKey" => { # [ allow ( non_camel_case_types ) ] struct CreateKeySvc < T : RecaptchaEnterpriseServiceV1Beta1 > ( pub Arc < T > ) ; impl < T : RecaptchaEnterpriseServiceV1Beta1 > tonic :: server :: UnaryService < super :: CreateKeyRequest > for CreateKeySvc < T > { type Response = super :: Key ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateKeyRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_key ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateKeySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/ListKeys" => { # [ allow ( non_camel_case_types ) ] struct ListKeysSvc < T : RecaptchaEnterpriseServiceV1Beta1 > ( pub Arc < T > ) ; impl < T : RecaptchaEnterpriseServiceV1Beta1 > tonic :: server :: UnaryService < super :: ListKeysRequest > for ListKeysSvc < T > { type Response = super :: ListKeysResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListKeysRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_keys ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListKeysSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/GetKey" => { # [ allow ( non_camel_case_types ) ] struct GetKeySvc < T : RecaptchaEnterpriseServiceV1Beta1 > ( pub Arc < T > ) ; impl < T : RecaptchaEnterpriseServiceV1Beta1 > tonic :: server :: UnaryService < super :: GetKeyRequest > for GetKeySvc < T > { type Response = super :: Key ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetKeyRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_key ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetKeySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/UpdateKey" => { # [ allow ( non_camel_case_types ) ] struct UpdateKeySvc < T : RecaptchaEnterpriseServiceV1Beta1 > ( pub Arc < T > ) ; impl < T : RecaptchaEnterpriseServiceV1Beta1 > tonic :: server :: UnaryService < super :: UpdateKeyRequest > for UpdateKeySvc < T > { type Response = super :: Key ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateKeyRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_key ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateKeySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1/DeleteKey" => { # [ allow ( non_camel_case_types ) ] struct DeleteKeySvc < T : RecaptchaEnterpriseServiceV1Beta1 > ( pub Arc < T > ) ; impl < T : RecaptchaEnterpriseServiceV1Beta1 > tonic :: server :: UnaryService < super :: DeleteKeyRequest > for DeleteKeySvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteKeyRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_key ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteKeySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: RecaptchaEnterpriseServiceV1Beta1> Clone for RecaptchaEnterpriseServiceV1Beta1Server<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: RecaptchaEnterpriseServiceV1Beta1> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RecaptchaEnterpriseServiceV1Beta1> tonic::transport::NamedService
        for RecaptchaEnterpriseServiceV1Beta1Server<T>
    {
        const NAME: &'static str =
            "google.cloud.recaptchaenterprise.v1beta1.RecaptchaEnterpriseServiceV1Beta1";
    }
}
