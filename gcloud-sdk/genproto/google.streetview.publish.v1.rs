/// Upload reference for media files.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadRef {
    /// Required.
    #[prost(oneof="upload_ref::FileSource", tags="1")]
    pub file_source: ::core::option::Option<upload_ref::FileSource>,
}
/// Nested message and enum types in `UploadRef`.
pub mod upload_ref {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FileSource {
        /// An upload reference should be unique for each user. It follows
        /// the form:
        /// "<https://streetviewpublish.googleapis.com/media/user/{account_id}/photo/{upload_reference}">
        #[prost(string, tag="1")]
        UploadUrl(::prost::alloc::string::String),
    }
}
/// Identifier for a \[Photo][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoId {
    /// A unique identifier for a photo.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
/// Level information containing level number and its corresponding name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Level {
    /// Optional. Floor number, used for ordering. 0 indicates the ground level, 1 indicates
    /// the first level above ground level, -1 indicates the first level under
    /// ground level. Non-integer values are OK.
    #[prost(double, tag="1")]
    pub number: f64,
    /// Required. A name assigned to this Level, restricted to 3 characters.
    /// Consider how the elevator buttons would be labeled for this level if there
    /// was an elevator.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
/// Raw pose measurement for an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pose {
    /// Latitude and longitude pair of the pose, as explained here:
    /// <https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/LatLng>
    /// When creating a \[Photo][google.streetview.publish.v1.Photo\], if the
    /// latitude and longitude pair are not provided, the geolocation from the
    /// exif header is used. A latitude and longitude pair not provided in the
    /// photo or exif header causes the photo process to fail.
    #[prost(message, optional, tag="1")]
    pub lat_lng_pair: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Altitude of the pose in meters above WGS84 ellipsoid.
    /// NaN indicates an unmeasured quantity.
    #[prost(double, tag="2")]
    pub altitude: f64,
    /// The following pose parameters pertain to the center of the photo. They
    /// match <https://developers.google.com/streetview/spherical-metadata.>
    /// Compass heading, measured at the center of the photo in degrees clockwise
    /// from North. Value must be >=0 and <360. NaN indicates an unmeasured
    /// quantity.
    #[prost(double, tag="3")]
    pub heading: f64,
    /// Pitch, measured at the center of the photo in degrees. Value must be >=-90
    /// and <= 90. A value of -90 means looking directly down, and a value of 90
    /// means looking directly up.
    /// NaN indicates an unmeasured quantity.
    #[prost(double, tag="4")]
    pub pitch: f64,
    /// Roll, measured in degrees. Value must be >= 0 and <360. A value of 0
    /// means level with the horizon.
    /// NaN indicates an unmeasured quantity.
    #[prost(double, tag="5")]
    pub roll: f64,
    /// Level (the floor in a building) used to configure vertical navigation.
    #[prost(message, optional, tag="7")]
    pub level: ::core::option::Option<Level>,
    /// The estimated horizontal accuracy of this pose in meters with 68%
    /// confidence (one standard deviation). For example, on Android, this value is
    /// available from this method:
    /// <https://developer.android.com/reference/android/location/Location#getAccuracy(>).
    /// Other platforms have different methods of obtaining similar accuracy
    /// estimations.
    #[prost(float, tag="9")]
    pub accuracy_meters: f32,
}
/// Place metadata for an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Place {
    /// Place identifier, as described in
    /// <https://developers.google.com/places/place-id.>
    #[prost(string, tag="1")]
    pub place_id: ::prost::alloc::string::String,
    /// Output only. The name of the place, localized to the language_code.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The language_code that the name is localized with. This should
    /// be the language_code specified in the request, but may be a fallback.
    #[prost(string, tag="3")]
    pub language_code: ::prost::alloc::string::String,
}
/// A connection is the link from a source photo to a destination photo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// Required. The destination of the connection from the containing photo to
    /// another photo.
    #[prost(message, optional, tag="1")]
    pub target: ::core::option::Option<PhotoId>,
}
/// Photo is used to store 360 photos along with photo metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Photo {
    /// Required. Output only. Required when updating a photo. Output only when creating a photo.
    /// Identifier for the photo, which is unique among all photos in
    /// Google.
    #[prost(message, optional, tag="1")]
    pub photo_id: ::core::option::Option<PhotoId>,
    /// Input only. Required when creating a photo. Input only. The resource URL where the
    /// photo bytes are uploaded to.
    #[prost(message, optional, tag="2")]
    pub upload_reference: ::core::option::Option<UploadRef>,
    /// Output only. The download URL for the photo bytes. This field is set only
    /// when
    /// \[GetPhotoRequest.view][google.streetview.publish.v1.GetPhotoRequest.view\]
    /// is set to
    /// \[PhotoView.INCLUDE_DOWNLOAD_URL][google.streetview.publish.v1.PhotoView.INCLUDE_DOWNLOAD_URL\].
    #[prost(string, tag="3")]
    pub download_url: ::prost::alloc::string::String,
    /// Output only. The thumbnail URL for showing a preview of the given photo.
    #[prost(string, tag="9")]
    pub thumbnail_url: ::prost::alloc::string::String,
    /// Output only. The share link for the photo.
    #[prost(string, tag="11")]
    pub share_link: ::prost::alloc::string::String,
    /// Optional. Pose of the photo.
    #[prost(message, optional, tag="4")]
    pub pose: ::core::option::Option<Pose>,
    /// Optional. Connections to other photos. A connection represents the link from this
    /// photo to another photo.
    #[prost(message, repeated, tag="5")]
    pub connections: ::prost::alloc::vec::Vec<Connection>,
    /// Optional. Absolute time when the photo was captured.
    /// When the photo has no exif timestamp, this is used to set a timestamp in
    /// the photo metadata.
    #[prost(message, optional, tag="6")]
    pub capture_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the image was uploaded.
    #[prost(message, optional, tag="14")]
    pub upload_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Places where this photo belongs.
    #[prost(message, repeated, tag="7")]
    pub places: ::prost::alloc::vec::Vec<Place>,
    /// Output only. View count of the photo.
    #[prost(int64, tag="10")]
    pub view_count: i64,
    /// Output only. Status of rights transfer on this photo.
    #[prost(enumeration="photo::TransferStatus", tag="12")]
    pub transfer_status: i32,
    /// Output only. Status in Google Maps, whether this photo was published or rejected.
    #[prost(enumeration="photo::MapsPublishStatus", tag="13")]
    pub maps_publish_status: i32,
}
/// Nested message and enum types in `Photo`.
pub mod photo {
    /// Status of rights transfer.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TransferStatus {
        /// The status of this transfer is unspecified.
        Unknown = 0,
        /// This photo has never been in a transfer.
        NeverTransferred = 1,
        /// This photo transfer has been initiated, but the receiver has not yet
        /// responded.
        Pending = 2,
        /// The photo transfer has been completed, and this photo has been
        /// transferred to the recipient.
        Completed = 3,
        /// The recipient rejected this photo transfer.
        Rejected = 4,
        /// The photo transfer expired before the recipient took any action.
        Expired = 5,
        /// The sender cancelled this photo transfer.
        Cancelled = 6,
        /// The recipient owns this photo due to a rights transfer.
        ReceivedViaTransfer = 7,
    }
    impl TransferStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TransferStatus::Unknown => "TRANSFER_STATUS_UNKNOWN",
                TransferStatus::NeverTransferred => "NEVER_TRANSFERRED",
                TransferStatus::Pending => "PENDING",
                TransferStatus::Completed => "COMPLETED",
                TransferStatus::Rejected => "REJECTED",
                TransferStatus::Expired => "EXPIRED",
                TransferStatus::Cancelled => "CANCELLED",
                TransferStatus::ReceivedViaTransfer => "RECEIVED_VIA_TRANSFER",
            }
        }
    }
    /// Publication status of the photo in Google Maps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MapsPublishStatus {
        /// The status of the photo is unknown.
        UnspecifiedMapsPublishStatus = 0,
        /// The photo is published to the public through Google Maps.
        Published = 1,
        /// The photo has been rejected for an unknown reason.
        RejectedUnknown = 2,
    }
    impl MapsPublishStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MapsPublishStatus::UnspecifiedMapsPublishStatus => "UNSPECIFIED_MAPS_PUBLISH_STATUS",
                MapsPublishStatus::Published => "PUBLISHED",
                MapsPublishStatus::RejectedUnknown => "REJECTED_UNKNOWN",
            }
        }
    }
}
/// Request to create a \[Photo][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePhotoRequest {
    /// Required. Photo to create.
    #[prost(message, optional, tag="1")]
    pub photo: ::core::option::Option<Photo>,
}
/// Request to get a \[Photo][google.streetview.publish.v1.Photo\].
///
/// By default
///
/// * does not return the download URL for the photo bytes.
///
/// Parameters:
///
/// * `view` controls if the download URL for the photo bytes is returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhotoRequest {
    /// Required. ID of the \[Photo][google.streetview.publish.v1.Photo\].
    #[prost(string, tag="1")]
    pub photo_id: ::prost::alloc::string::String,
    /// Required. Specifies if a download URL for the photo bytes should be returned in the
    /// \[Photo][google.streetview.publish.v1.Photo\] response.
    #[prost(enumeration="PhotoView", tag="2")]
    pub view: i32,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    /// If language_code is unspecified, the user's language preference for Google
    /// services is used.
    #[prost(string, tag="3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Request to get one or more \[Photos][google.streetview.publish.v1.Photo\].
/// By default
///
/// * does not return the download URL for the photo bytes.
///
/// Parameters:
///
/// * `view` controls if the download URL for the photo bytes is returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetPhotosRequest {
    /// Required. IDs of the \[Photos][google.streetview.publish.v1.Photo\]. For HTTP
    /// GET requests, the URL query parameter should be
    /// `photoIds=<id1>&photoIds=<id2>&...`.
    #[prost(string, repeated, tag="1")]
    pub photo_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Specifies if a download URL for the photo bytes should be returned in the
    /// Photo response.
    #[prost(enumeration="PhotoView", tag="2")]
    pub view: i32,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    /// If language_code is unspecified, the user's language preference for Google
    /// services is used.
    #[prost(string, tag="3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response to batch get of \[Photos][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetPhotosResponse {
    /// List of results for each individual
    /// \[Photo][google.streetview.publish.v1.Photo\] requested, in the same order as
    /// the requests in
    /// \[BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos\].
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<PhotoResponse>,
}
/// Response payload for a single
/// \[Photo][google.streetview.publish.v1.Photo\]
/// in batch operations including
/// \[BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos\]
/// and
/// \[BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoResponse {
    /// The status for the operation to get or update a single photo in the batch
    /// request.
    #[prost(message, optional, tag="1")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
    /// The \[Photo][google.streetview.publish.v1.Photo\] resource, if the request
    /// was successful.
    #[prost(message, optional, tag="2")]
    pub photo: ::core::option::Option<Photo>,
}
/// Request to list all photos that belong to the user sending the request.
///
/// By default
///
/// * does not return the download URL for the photo bytes.
///
/// Parameters:
///
/// * `view` controls if the download URL for the photo bytes is returned.
/// * `pageSize` determines the maximum number of photos to return.
/// * `pageToken` is the next page token value returned from a previous
/// \[ListPhotos][google.streetview.publish.v1.StreetViewPublishService.ListPhotos\]
///      request, if any.
/// * `filter` allows filtering by a given parameter. 'placeId' is the only
/// parameter supported at the moment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhotosRequest {
    /// Required. Specifies if a download URL for the photos bytes should be returned in the
    /// Photos response.
    #[prost(enumeration="PhotoView", tag="1")]
    pub view: i32,
    /// Optional. The maximum number of photos to return.
    /// `pageSize` must be non-negative. If `pageSize` is zero or is not provided,
    /// the default page size of 100 is used.
    /// The number of photos returned in the response may be less than `pageSize`
    /// if the number of photos that belong to the user is less than `pageSize`.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// Optional. The
    /// \[nextPageToken][google.streetview.publish.v1.ListPhotosResponse.next_page_token\]
    /// value returned from a previous
    /// \[ListPhotos][google.streetview.publish.v1.StreetViewPublishService.ListPhotos\]
    /// request, if any.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter expression. For example: `placeId=ChIJj61dQgK6j4AR4GeTYWZsKWw`.
    ///
    /// The filters supported at the moment are: `placeId`.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
    /// If language_code is unspecified, the user's language preference for Google
    /// services is used.
    #[prost(string, tag="5")]
    pub language_code: ::prost::alloc::string::String,
}
/// Response to list all photos that belong to a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhotosResponse {
    /// List of photos. The
    /// \[pageSize][google.streetview.publish.v1.ListPhotosRequest.page_size\] field
    /// in the request determines the number of items returned.
    #[prost(message, repeated, tag="1")]
    pub photos: ::prost::alloc::vec::Vec<Photo>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request to update the metadata of a
/// \[Photo][google.streetview.publish.v1.Photo\]. Updating the pixels of a photo
/// is not supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePhotoRequest {
    /// Required. \[Photo][google.streetview.publish.v1.Photo\] object containing the
    /// new metadata.
    #[prost(message, optional, tag="1")]
    pub photo: ::core::option::Option<Photo>,
    /// Required. Mask that identifies fields on the photo metadata to update.
    /// If not present, the old \[Photo][google.streetview.publish.v1.Photo\]
    /// metadata is entirely replaced with the
    /// new \[Photo][google.streetview.publish.v1.Photo\] metadata in this request.
    /// The update fails if invalid fields are specified. Multiple fields can be
    /// specified in a comma-delimited list.
    ///
    /// The following fields are valid:
    ///
    /// * `pose.heading`
    /// * `pose.latLngPair`
    /// * `pose.pitch`
    /// * `pose.roll`
    /// * `pose.level`
    /// * `pose.altitude`
    /// * `connections`
    /// * `places`
    ///
    ///
    /// > Note: When
    /// \[updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask\]
    /// contains repeated fields, the entire set of repeated values get replaced
    /// with the new contents. For example, if
    /// \[updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask\]
    /// contains `connections` and `UpdatePhotoRequest.photo.connections` is empty,
    /// all connections are removed.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request to update the metadata of photos.
/// Updating the pixels of photos is not supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdatePhotosRequest {
    /// Required. List of
    /// \[UpdatePhotoRequests][google.streetview.publish.v1.UpdatePhotoRequest\].
    #[prost(message, repeated, tag="1")]
    pub update_photo_requests: ::prost::alloc::vec::Vec<UpdatePhotoRequest>,
}
/// Response to batch update of metadata of one or more
/// \[Photos][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdatePhotosResponse {
    /// List of results for each individual
    /// \[Photo][google.streetview.publish.v1.Photo\] updated, in the same order as
    /// the request.
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<PhotoResponse>,
}
/// Request to delete a \[Photo][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePhotoRequest {
    /// Required. ID of the \[Photo][google.streetview.publish.v1.Photo\].
    #[prost(string, tag="1")]
    pub photo_id: ::prost::alloc::string::String,
}
/// Request to delete multiple \[Photos][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeletePhotosRequest {
    /// Required. IDs of the \[Photos][google.streetview.publish.v1.Photo\]. HTTP
    /// GET requests require the following syntax for the URL query parameter:
    /// `photoIds=<id1>&photoIds=<id2>&...`.
    #[prost(string, repeated, tag="1")]
    pub photo_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response to batch delete of one or more
/// \[Photos][google.streetview.publish.v1.Photo\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeletePhotosResponse {
    /// The status for the operation to delete a single
    /// \[Photo][google.streetview.publish.v1.Photo\] in the batch request.
    #[prost(message, repeated, tag="1")]
    pub status: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Specifies which view of the \[Photo][google.streetview.publish.v1.Photo\]
/// to include in the response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PhotoView {
    /// Server responses do not include the download URL for the photo bytes.
    /// The default value.
    Basic = 0,
    /// Server responses include the download URL for the photo bytes.
    IncludeDownloadUrl = 1,
}
impl PhotoView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PhotoView::Basic => "BASIC",
            PhotoView::IncludeDownloadUrl => "INCLUDE_DOWNLOAD_URL",
        }
    }
}
/// Generated client implementations.
pub mod street_view_publish_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Publishes and connects user-contributed photos on Street View.
    #[derive(Debug, Clone)]
    pub struct StreetViewPublishServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StreetViewPublishServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StreetViewPublishServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StreetViewPublishServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            StreetViewPublishServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Creates an upload session to start uploading photo bytes.  The method uses
        /// the upload URL of the returned
        /// [UploadRef][google.streetview.publish.v1.UploadRef] to upload the bytes for
        /// the [Photo][google.streetview.publish.v1.Photo].
        ///
        /// In addition to the photo requirements shown in
        /// https://support.google.com/maps/answer/7012050?ref_topic=6275604,
        /// the photo must meet the following requirements:
        ///
        /// * Photo Sphere XMP metadata must be included in the photo metadata. See
        /// https://developers.google.com/streetview/spherical-metadata for the
        /// required fields.
        /// * The pixel size of the photo must meet the size requirements listed in
        /// https://support.google.com/maps/answer/7012050?ref_topic=6275604, and
        /// the photo must be a full 360 horizontally.
        ///
        /// After the upload completes, the method uses
        /// [UploadRef][google.streetview.publish.v1.UploadRef] with
        /// [CreatePhoto][google.streetview.publish.v1.StreetViewPublishService.CreatePhoto]
        /// to create the [Photo][google.streetview.publish.v1.Photo] object entry.
        pub async fn start_upload(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::UploadRef>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/StartUpload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// After the client finishes uploading the photo with the returned
        /// [UploadRef][google.streetview.publish.v1.UploadRef],
        /// [CreatePhoto][google.streetview.publish.v1.StreetViewPublishService.CreatePhoto]
        /// publishes the uploaded [Photo][google.streetview.publish.v1.Photo] to
        /// Street View on Google Maps.
        ///
        /// Currently, the only way to set heading, pitch, and roll in CreatePhoto is
        /// through the [Photo Sphere XMP
        /// metadata](https://developers.google.com/streetview/spherical-metadata) in
        /// the photo bytes. CreatePhoto ignores the  `pose.heading`, `pose.pitch`,
        /// `pose.roll`, `pose.altitude`, and `pose.level` fields in Pose.
        ///
        /// This method returns the following error codes:
        ///
        /// * [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT] if the request is malformed or if
        /// the uploaded photo is not a 360 photo.
        /// * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the upload reference does not exist.
        /// * [google.rpc.Code.RESOURCE_EXHAUSTED][google.rpc.Code.RESOURCE_EXHAUSTED] if the account has reached the
        /// storage limit.
        pub async fn create_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/CreatePhoto",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the metadata of the specified
        /// [Photo][google.streetview.publish.v1.Photo].
        ///
        /// This method returns the following error codes:
        ///
        /// * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if the requesting user did not
        /// create the requested [Photo][google.streetview.publish.v1.Photo].
        /// * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the requested
        /// [Photo][google.streetview.publish.v1.Photo] does not exist.
        /// * [google.rpc.Code.UNAVAILABLE][google.rpc.Code.UNAVAILABLE] if the requested
        /// [Photo][google.streetview.publish.v1.Photo] is still being indexed.
        pub async fn get_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/GetPhoto",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the metadata of the specified
        /// [Photo][google.streetview.publish.v1.Photo] batch.
        ///
        /// Note that if
        /// [BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos]
        /// fails, either critical fields are missing or there is an authentication
        /// error. Even if
        /// [BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos]
        /// succeeds, individual photos in the batch may have failures.
        /// These failures are specified in each
        /// [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]
        /// in
        /// [BatchGetPhotosResponse.results][google.streetview.publish.v1.BatchGetPhotosResponse.results].
        /// See
        /// [GetPhoto][google.streetview.publish.v1.StreetViewPublishService.GetPhoto]
        /// for specific failures that can occur per photo.
        pub async fn batch_get_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetPhotosRequest>,
        ) -> Result<tonic::Response<super::BatchGetPhotosResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/BatchGetPhotos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all the [Photos][google.streetview.publish.v1.Photo] that belong to
        /// the user.
        ///
        /// > Note: Recently created photos that are still
        /// being indexed are not returned in the response.
        pub async fn list_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPhotosRequest>,
        ) -> Result<tonic::Response<super::ListPhotosResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/ListPhotos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the metadata of a [Photo][google.streetview.publish.v1.Photo], such
        /// as pose, place association, connections, etc. Changing the pixels of a
        /// photo is not supported.
        ///
        /// Only the fields specified in the
        /// [updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask]
        /// field are used. If `updateMask` is not present, the update applies to all
        /// fields.
        ///
        /// This method returns the following error codes:
        ///
        /// * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if the requesting user did not
        /// create the requested photo.
        /// * [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT] if the request is malformed.
        /// * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the requested photo does not exist.
        /// * [google.rpc.Code.UNAVAILABLE][google.rpc.Code.UNAVAILABLE] if the requested
        /// [Photo][google.streetview.publish.v1.Photo] is still being indexed.
        pub async fn update_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/UpdatePhoto",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the metadata of [Photos][google.streetview.publish.v1.Photo], such
        /// as pose, place association, connections, etc. Changing the pixels of photos
        /// is not supported.
        ///
        /// Note that if
        /// [BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos]
        /// fails, either critical fields are missing or there is an authentication
        /// error. Even if
        /// [BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos]
        /// succeeds, individual photos in the batch may have failures.
        /// These failures are specified in each
        /// [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]
        /// in
        /// [BatchUpdatePhotosResponse.results][google.streetview.publish.v1.BatchUpdatePhotosResponse.results].
        /// See
        /// [UpdatePhoto][google.streetview.publish.v1.StreetViewPublishService.UpdatePhoto]
        /// for specific failures that can occur per photo.
        ///
        /// Only the fields specified in
        /// [updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask]
        /// field are used. If `updateMask` is not present, the update applies to all
        /// fields.
        ///
        /// The number of
        /// [UpdatePhotoRequest][google.streetview.publish.v1.UpdatePhotoRequest]
        /// messages in a
        /// [BatchUpdatePhotosRequest][google.streetview.publish.v1.BatchUpdatePhotosRequest]
        /// must not exceed 20.
        ///
        /// > Note: To update
        /// [Pose.altitude][google.streetview.publish.v1.Pose.altitude],
        /// [Pose.latLngPair][google.streetview.publish.v1.Pose.lat_lng_pair] has to be
        /// filled as well. Otherwise, the request will fail.
        pub async fn batch_update_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdatePhotosRequest>,
        ) -> Result<tonic::Response<super::BatchUpdatePhotosResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/BatchUpdatePhotos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a [Photo][google.streetview.publish.v1.Photo] and its metadata.
        ///
        /// This method returns the following error codes:
        ///
        /// * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if the requesting user did not
        /// create the requested photo.
        /// * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the photo ID does not exist.
        pub async fn delete_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePhotoRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/DeletePhoto",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a list of [Photos][google.streetview.publish.v1.Photo] and their
        /// metadata.
        ///
        /// Note that if
        /// [BatchDeletePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchDeletePhotos]
        /// fails, either critical fields are missing or there is an authentication
        /// error. Even if
        /// [BatchDeletePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchDeletePhotos]
        /// succeeds, individual photos in the batch may have failures.
        /// These failures are specified in each
        /// [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]
        /// in
        /// [BatchDeletePhotosResponse.results][google.streetview.publish.v1.BatchDeletePhotosResponse.status].
        /// See
        /// [DeletePhoto][google.streetview.publish.v1.StreetViewPublishService.DeletePhoto]
        /// for specific failures that can occur per photo.
        pub async fn batch_delete_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeletePhotosRequest>,
        ) -> Result<tonic::Response<super::BatchDeletePhotosResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.streetview.publish.v1.StreetViewPublishService/BatchDeletePhotos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
