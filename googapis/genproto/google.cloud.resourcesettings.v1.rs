/// The instantiation of a setting. Every setting value is parented by its
/// corresponding setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingValue {
    /// The resource name of the setting value. Must be in one of the following
    /// forms:
    ///
    /// * `projects/{project_number}/settings/{setting_name}/value`
    /// * `folders/{folder_id}/settings/{setting_name}/value`
    /// * `organizations/{organization_id}/settings/{setting_name}/value`
    ///
    /// For example, "/projects/123/settings/gcp-enableMyFeature/value"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The value of the setting. The data type of [Value][google.cloud.resourcesettings.v1.Value] must always be
    /// consistent with the data type defined by the parent setting.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
    /// A fingerprint used for optimistic concurrency. See
    /// [UpdateSettingValue][google.cloud.resourcesettings.v1.ResourceSettingsService.UpdateSettingValue] for more
    /// details.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. A flag indicating that this setting value cannot be modified.
    /// This flag is inherited from its parent setting and is for
    /// convenience purposes. See [Setting.read_only][google.cloud.resourcesettings.v1.Setting.read_only] for more details.
    #[prost(bool, tag = "4")]
    pub read_only: bool,
    /// Output only. The timestamp indicating when the setting value was last
    /// updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The schema for setting values. At a given Cloud resource, a setting can
/// parent at most one setting value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Setting {
    /// The resource name of the setting. Must be in one of the following forms:
    ///
    /// * `projects/{project_number}/settings/{setting_name}`
    /// * `folders/{folder_id}/settings/{setting_name}`
    /// * `organizations/{organization_id}/settings/{setting_name}`
    ///
    /// For example, "/projects/123/settings/gcp-enableMyFeature"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The human readable name for this setting.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A detailed description of what this setting does.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// A flag indicating that values of this setting cannot be modified (see
    /// documentation of the specific setting for updates and reasons).
    #[prost(bool, tag = "4")]
    pub read_only: bool,
    /// The data type for this setting.
    #[prost(enumeration = "setting::DataType", tag = "5")]
    pub data_type: i32,
    /// The value received by
    /// [LookupEffectiveSettingValue][google.cloud.resourcesettings.v1.ResourceSettingsService.LookupEffectiveSettingValue]
    /// if no setting value is explicitly set.
    ///
    /// Note: not all settings have a default value.
    #[prost(message, optional, tag = "6")]
    pub default_value: ::core::option::Option<Value>,
}
/// Nested message and enum types in `Setting`.
pub mod setting {
    /// The data type for setting values of this setting. See [Value][google.cloud.resourcesettings.v1.Value] for more
    /// details on the available data types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataType {
        /// Unspecified data type.
        Unspecified = 0,
        /// A boolean setting.
        Boolean = 1,
        /// A string setting.
        String = 2,
        /// A string set setting.
        StringSet = 3,
        /// A Enum setting
        EnumValue = 4,
    }
}
/// The data in a setting value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// Selects the data type and associated value.
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// A string set value that can hold a set of strings. The maximum length of
    /// each string is 60 characters and there can be a maximum of 50 strings in
    /// the string set.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringSet {
        /// The strings in the set
        #[prost(string, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A enum value that can hold any enum type setting values.
    /// Each enum type is represented by a number, this representation
    /// is stored in the definitions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EnumValue {
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
    }
    /// Selects the data type and associated value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Defines this value as being a boolean value.
        #[prost(bool, tag = "1")]
        BooleanValue(bool),
        /// Defines this value as being a string value.
        #[prost(string, tag = "2")]
        StringValue(::prost::alloc::string::String),
        /// Defines this value as being a StringSet.
        #[prost(message, tag = "3")]
        StringSetValue(StringSet),
        /// Defines this value as being a Enum.
        #[prost(message, tag = "4")]
        EnumValue(EnumValue),
    }
}
/// The request for ListSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSettingsRequest {
    /// Required. The Cloud resource that parents the setting. Must be in one of the
    /// following forms:
    ///
    /// * `projects/{project_number}`
    /// * `projects/{project_id}`
    /// * `folders/{folder_id}`
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Unused. The size of the page to be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Unused. A page token used to retrieve the next page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from ListSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSettingsResponse {
    /// A list of settings that are available at the specified Cloud resource.
    #[prost(message, repeated, tag = "1")]
    pub settings: ::prost::alloc::vec::Vec<Setting>,
    /// Unused. A page token used to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for SearchSettingValues.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSettingValuesRequest {
    /// Required. The Cloud resource that parents the setting. Must be in one of the
    /// following forms:
    ///
    /// * `projects/{project_number}`
    /// * `projects/{project_id}`
    /// * `folders/{folder_id}`
    /// * `organizations/{organization_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Unused. The size of the page to be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Unused. A page token used to retrieve the next page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from SearchSettingValues.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSettingValuesResponse {
    /// All setting values that exist on the specified Cloud resource.
    #[prost(message, repeated, tag = "1")]
    pub setting_values: ::prost::alloc::vec::Vec<SettingValue>,
    /// Unused. A page token used to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for GetSettingValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingValueRequest {
    /// Required. The name of the setting value to get. See [SettingValue][google.cloud.resourcesettings.v1.SettingValue] for naming
    /// requirements.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for LookupEffectiveSettingValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEffectiveSettingValueRequest {
    /// Required. The setting value for which an effective value will be evaluated.
    /// See [SettingValue][google.cloud.resourcesettings.v1.SettingValue] for naming requirements.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for CreateSettingValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSettingValueRequest {
    /// Required. The name of the setting for which a value should be created.
    /// See [Setting][google.cloud.resourcesettings.v1.Setting] for naming requirements.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The setting value to create. See [SettingValue][google.cloud.resourcesettings.v1.SettingValue] for field requirements.
    #[prost(message, optional, tag = "2")]
    pub setting_value: ::core::option::Option<SettingValue>,
}
/// The request for UpdateSettingValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingValueRequest {
    /// Required. The setting value to update. See [SettingValue][google.cloud.resourcesettings.v1.SettingValue] for field requirements.
    #[prost(message, optional, tag = "1")]
    pub setting_value: ::core::option::Option<SettingValue>,
}
/// The request for DeleteSettingValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSettingValueRequest {
    /// Required. The name of the setting value to delete. See [SettingValue][google.cloud.resourcesettings.v1.SettingValue] for naming
    /// requirements.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod resource_settings_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " An interface to interact with resource settings and setting values throughout"]
    #[doc = " the resource hierarchy."]
    #[doc = ""]
    #[doc = " Services may surface a number of settings for users to control how their"]
    #[doc = " resources behave. Setting values applied on a given Cloud resource are"]
    #[doc = " evaluated hierarchically and inherited by all descendants of that resource."]
    #[doc = ""]
    #[doc = " For all requests, returns a `google.rpc.Status` with"]
    #[doc = " `google.rpc.Code.PERMISSION_DENIED` if the IAM check fails or the `parent`"]
    #[doc = " resource is not in a Cloud Organization."]
    #[doc = " For all requests, returns a `google.rpc.Status` with"]
    #[doc = " `google.rpc.Code.INVALID_ARGUMENT` if the request is malformed."]
    pub struct ResourceSettingsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ResourceSettingsServiceClient<T>
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
        #[doc = " Lists all the settings that are available on the Cloud resource `parent`."]
        pub async fn list_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSettingsRequest>,
        ) -> Result<tonic::Response<super::ListSettingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcesettings.v1.ResourceSettingsService/ListSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches for all setting values that exist on the resource `parent`. The"]
        #[doc = " setting values are not limited to those of a particular setting."]
        pub async fn search_setting_values(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchSettingValuesRequest>,
        ) -> Result<tonic::Response<super::SearchSettingValuesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcesettings.v1.ResourceSettingsService/SearchSettingValues",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a setting value."]
        #[doc = ""]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the"]
        #[doc = " setting value does not exist."]
        pub async fn get_setting_value(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSettingValueRequest>,
        ) -> Result<tonic::Response<super::SettingValue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcesettings.v1.ResourceSettingsService/GetSettingValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Computes the effective setting value of a setting at the Cloud resource"]
        #[doc = " `parent`. The effective setting value is the calculated setting value at a"]
        #[doc = " Cloud resource and evaluates to one of the following options in the given"]
        #[doc = " order (the next option is used if the previous one does not exist):"]
        #[doc = ""]
        #[doc = " 1. the setting value on the given resource"]
        #[doc = " 2. the setting value on the given resource's nearest ancestor"]
        #[doc = " 3. the setting's default value"]
        #[doc = " 4. an empty setting value, defined as a `SettingValue` with all fields"]
        #[doc = " unset"]
        #[doc = ""]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the"]
        #[doc = " setting does not exist."]
        pub async fn lookup_effective_setting_value(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupEffectiveSettingValueRequest>,
        ) -> Result<tonic::Response<super::SettingValue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.resourcesettings.v1.ResourceSettingsService/LookupEffectiveSettingValue") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a setting value."]
        #[doc = ""]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the"]
        #[doc = " setting does not exist."]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the"]
        #[doc = " setting value already exists on the given Cloud resource."]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.FAILED_PRECONDITION` if"]
        #[doc = " the setting is flagged as read only."]
        pub async fn create_setting_value(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSettingValueRequest>,
        ) -> Result<tonic::Response<super::SettingValue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcesettings.v1.ResourceSettingsService/CreateSettingValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a setting value."]
        #[doc = ""]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the"]
        #[doc = " setting or the setting value does not exist."]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.FAILED_PRECONDITION` if"]
        #[doc = " the setting is flagged as read only."]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag"]
        #[doc = " supplied in the request does not match the persisted etag of the setting"]
        #[doc = " value."]
        #[doc = ""]
        #[doc = " Note: the supplied setting value will perform a full overwrite of all"]
        #[doc = " fields."]
        pub async fn update_setting_value(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingValueRequest>,
        ) -> Result<tonic::Response<super::SettingValue>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcesettings.v1.ResourceSettingsService/UpdateSettingValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a setting value. If the setting value does not exist, the operation"]
        #[doc = " is a no-op."]
        #[doc = ""]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the"]
        #[doc = " setting or the setting value does not exist. The setting value will not"]
        #[doc = " exist if a prior call to `DeleteSettingValue` for the setting value already"]
        #[doc = " returned a success code."]
        #[doc = " Returns a `google.rpc.Status` with `google.rpc.Code.FAILED_PRECONDITION` if"]
        #[doc = " the setting is flagged as read only."]
        pub async fn delete_setting_value(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSettingValueRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.resourcesettings.v1.ResourceSettingsService/DeleteSettingValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ResourceSettingsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ResourceSettingsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ResourceSettingsServiceClient {{ ... }}")
        }
    }
}
