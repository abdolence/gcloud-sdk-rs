/// Upload reference for media files.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadRef {
    /// Required. An upload reference should be unique for each user. It follows
    /// the form:
    /// "https://streetviewpublish.googleapis.com/media/user/{account_id}/photo/{upload_reference}"
    #[prost(string, tag = "1")]
    pub upload_url: std::string::String,
}
/// Identifier for a [Photo][google.streetview.publish.v1.Photo].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoId {
    /// Required. A unique identifier for a photo.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
}
/// Level information containing level number and its corresponding name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Level {
    /// Floor number, used for ordering. 0 indicates the ground level, 1 indicates
    /// the first level above ground level, -1 indicates the first level under
    /// ground level. Non-integer values are OK.
    #[prost(double, tag = "1")]
    pub number: f64,
    /// Required. A name assigned to this Level, restricted to 3 characters.
    /// Consider how the elevator buttons would be labeled for this level if there
    /// was an elevator.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
/// Raw pose measurement for an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pose {
    /// Latitude and longitude pair of the pose, as explained here:
    /// https://cloud.google.com/datastore/docs/reference/rest/Shared.Types/LatLng
    /// When creating a [Photo][google.streetview.publish.v1.Photo], if the
    /// latitude and longitude pair are not provided, the geolocation from the
    /// exif header is used. A latitude and longitude pair not provided in the
    /// photo or exif header causes the create photo process to fail.
    #[prost(message, optional, tag = "1")]
    pub lat_lng_pair: ::std::option::Option<super::super::super::r#type::LatLng>,
    /// Altitude of the pose in meters above WGS84 ellipsoid.
    /// NaN indicates an unmeasured quantity.
    #[prost(double, tag = "2")]
    pub altitude: f64,
    /// Compass heading, measured at the center of the photo in degrees clockwise
    /// from North. Value must be >=0 and <360.
    /// NaN indicates an unmeasured quantity.
    #[prost(double, tag = "3")]
    pub heading: f64,
    /// Pitch, measured at the center of the photo in degrees. Value must be >=-90
    /// and <= 90. A value of -90 means looking directly down, and a value of 90
    /// means looking directly up.
    /// NaN indicates an unmeasured quantity.
    #[prost(double, tag = "4")]
    pub pitch: f64,
    /// Roll, measured in degrees. Value must be >= 0 and <360. A value of 0
    /// means level with the horizon.
    /// NaN indicates an unmeasured quantity.
    #[prost(double, tag = "5")]
    pub roll: f64,
    /// Level (the floor in a building) used to configure vertical navigation.
    #[prost(message, optional, tag = "7")]
    pub level: ::std::option::Option<Level>,
    /// The estimated horizontal accuracy of this pose in meters with 68%
    /// confidence (one standard deviation). For example, on Android, this value is
    /// available from this method:
    /// https://developer.android.com/reference/android/location/Location#getAccuracy().
    /// Other platforms have different methods of obtaining similar accuracy
    /// estimations.
    #[prost(float, tag = "9")]
    pub accuracy_meters: f32,
}
/// Place metadata for an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Place {
    /// Place identifier, as described in
    /// https://developers.google.com/places/place-id.
    #[prost(string, tag = "1")]
    pub place_id: std::string::String,
    /// Output-only. The name of the place, localized to the language_code.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    /// Output-only. The language_code that the name is localized with. This should
    /// be the language_code specified in the request, but may be a fallback.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// A connection is the link from a source photo to a destination photo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connection {
    /// Required. The destination of the connection from the containing photo to
    /// another photo.
    #[prost(message, optional, tag = "1")]
    pub target: ::std::option::Option<PhotoId>,
}
/// Photo is used to store 360 photos along with photo metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Photo {
    /// Required when updating a photo. Output only when creating a photo.
    /// Identifier for the photo, which is unique among all photos in
    /// Google.
    #[prost(message, optional, tag = "1")]
    pub photo_id: ::std::option::Option<PhotoId>,
    /// Required when creating a photo. Input only. The resource URL where the
    /// photo bytes are uploaded to.
    #[prost(message, optional, tag = "2")]
    pub upload_reference: ::std::option::Option<UploadRef>,
    /// Output only. The download URL for the photo bytes. This field is set only
    /// when
    /// [GetPhotoRequest.view][google.streetview.publish.v1.GetPhotoRequest.view]
    /// is set to
    /// [PhotoView.INCLUDE_DOWNLOAD_URL][google.streetview.publish.v1.PhotoView.INCLUDE_DOWNLOAD_URL].
    #[prost(string, tag = "3")]
    pub download_url: std::string::String,
    /// Output only. The thumbnail URL for showing a preview of the given photo.
    #[prost(string, tag = "9")]
    pub thumbnail_url: std::string::String,
    /// Output only. The share link for the photo.
    #[prost(string, tag = "11")]
    pub share_link: std::string::String,
    /// Pose of the photo.
    #[prost(message, optional, tag = "4")]
    pub pose: ::std::option::Option<Pose>,
    /// Connections to other photos. A connection represents the link from this
    /// photo to another photo.
    #[prost(message, repeated, tag = "5")]
    pub connections: ::std::vec::Vec<Connection>,
    /// Absolute time when the photo was captured.
    /// When the photo has no exif timestamp, this is used to set a timestamp in
    /// the photo metadata.
    #[prost(message, optional, tag = "6")]
    pub capture_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Places where this photo belongs.
    #[prost(message, repeated, tag = "7")]
    pub places: ::std::vec::Vec<Place>,
    /// Output only. View count of the photo.
    #[prost(int64, tag = "10")]
    pub view_count: i64,
    /// Output only. Status of rights transfer on this photo.
    #[prost(enumeration = "photo::TransferStatus", tag = "12")]
    pub transfer_status: i32,
    /// Output only. Status in Google Maps, whether this photo was published or
    /// rejected.
    #[prost(enumeration = "photo::MapsPublishStatus", tag = "13")]
    pub maps_publish_status: i32,
}
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
}
/// Request to create a [Photo][google.streetview.publish.v1.Photo].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePhotoRequest {
    /// Required. Photo to create.
    #[prost(message, optional, tag = "1")]
    pub photo: ::std::option::Option<Photo>,
}
/// Request to get a [Photo][google.streetview.publish.v1.Photo].
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
    /// Required. ID of the [Photo][google.streetview.publish.v1.Photo].
    #[prost(string, tag = "1")]
    pub photo_id: std::string::String,
    /// Specifies if a download URL for the photo bytes should be returned in the
    /// [Photo][google.streetview.publish.v1.Photo] response.
    #[prost(enumeration = "PhotoView", tag = "2")]
    pub view: i32,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    /// If language_code is unspecified, the user's language preference for Google
    /// services is used.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// Request to get one or more [Photos][google.streetview.publish.v1.Photo].
/// By default
///
/// * does not return the download URL for the photo bytes.
///
/// Parameters:
///
/// * `view` controls if the download URL for the photo bytes is returned.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetPhotosRequest {
    /// Required. IDs of the [Photos][google.streetview.publish.v1.Photo]. HTTP GET
    /// requests require the following syntax for the URL query parameter:
    /// `photoIds=<id1>&photoIds=<id2>&...`.
    #[prost(string, repeated, tag = "1")]
    pub photo_ids: ::std::vec::Vec<std::string::String>,
    /// Specifies if a download URL for the photo bytes should be returned in the
    /// Photo response.
    #[prost(enumeration = "PhotoView", tag = "2")]
    pub view: i32,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    /// If language_code is unspecified, the user's language preference for Google
    /// services is used.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// Response to batch get of [Photos][google.streetview.publish.v1.Photo].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetPhotosResponse {
    /// List of results for each individual
    /// [Photo][google.streetview.publish.v1.Photo] requested, in the same order as
    /// the requests in
    /// [BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos].
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<PhotoResponse>,
}
/// Response payload for a single
/// [Photo][google.streetview.publish.v1.Photo]
/// in batch operations including
/// [BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos]
/// and
/// [BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhotoResponse {
    /// The status for the operation to get or update a single photo in the batch
    /// request.
    #[prost(message, optional, tag = "1")]
    pub status: ::std::option::Option<super::super::super::rpc::Status>,
    /// The [Photo][google.streetview.publish.v1.Photo] resource, if the request
    /// was successful.
    #[prost(message, optional, tag = "2")]
    pub photo: ::std::option::Option<Photo>,
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
/// [ListPhotos][google.streetview.publish.v1.StreetViewPublishService.ListPhotos]
///     request, if any.
/// * `filter` allows filtering by a given parameter. 'placeId' is the only
/// parameter supported at the moment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhotosRequest {
    /// Specifies if a download URL for the photos bytes should be returned in the
    /// Photos response.
    #[prost(enumeration = "PhotoView", tag = "1")]
    pub view: i32,
    /// The maximum number of photos to return.
    /// `pageSize` must be non-negative. If `pageSize` is zero or is not provided,
    /// the default page size of 100 is used.
    /// The number of photos returned in the response may be less than `pageSize`
    /// if the number of photos that belong to the user is less than `pageSize`.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The
    /// [nextPageToken][google.streetview.publish.v1.ListPhotosResponse.next_page_token]
    /// value returned from a previous
    /// [ListPhotos][google.streetview.publish.v1.StreetViewPublishService.ListPhotos]
    /// request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// The filter expression. For example: `placeId=ChIJj61dQgK6j4AR4GeTYWZsKWw`.
    ///
    /// The only filter supported at the moment is `placeId`.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
    /// information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    /// If language_code is unspecified, the user's language preference for Google
    /// services is used.
    #[prost(string, tag = "5")]
    pub language_code: std::string::String,
}
/// Response to list all photos that belong to a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPhotosResponse {
    /// List of photos. The
    /// [pageSize][google.streetview.publish.v1.ListPhotosRequest.page_size] field
    /// in the request determines the number of items returned.
    #[prost(message, repeated, tag = "1")]
    pub photos: ::std::vec::Vec<Photo>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request to update the metadata of a
/// [Photo][google.streetview.publish.v1.Photo]. Updating the pixels of a photo
/// is not supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePhotoRequest {
    /// Required. [Photo][google.streetview.publish.v1.Photo] object containing the
    /// new metadata.
    #[prost(message, optional, tag = "1")]
    pub photo: ::std::option::Option<Photo>,
    /// Mask that identifies fields on the photo metadata to update.
    /// If not present, the old [Photo][google.streetview.publish.v1.Photo]
    /// metadata is entirely replaced with the
    /// new [Photo][google.streetview.publish.v1.Photo] metadata in this request.
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
    /// <aside class="note"><b>Note:</b>  When
    /// [updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask]
    /// contains repeated fields, the entire set of repeated values get replaced
    /// with the new contents. For example, if
    /// [updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask]
    /// contains `connections` and `UpdatePhotoRequest.photo.connections` is empty,
    /// all connections are removed.</aside>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request to update the metadata of photos.
/// Updating the pixels of photos is not supported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdatePhotosRequest {
    /// Required. List of
    /// [UpdatePhotoRequests][google.streetview.publish.v1.UpdatePhotoRequest].
    #[prost(message, repeated, tag = "1")]
    pub update_photo_requests: ::std::vec::Vec<UpdatePhotoRequest>,
}
/// Response to batch update of metadata of one or more
/// [Photos][google.streetview.publish.v1.Photo].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdatePhotosResponse {
    /// List of results for each individual
    /// [Photo][google.streetview.publish.v1.Photo] updated, in the same order as
    /// the request.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<PhotoResponse>,
}
/// Request to delete a [Photo][google.streetview.publish.v1.Photo].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePhotoRequest {
    /// Required. ID of the [Photo][google.streetview.publish.v1.Photo].
    #[prost(string, tag = "1")]
    pub photo_id: std::string::String,
}
/// Request to delete multiple [Photos][google.streetview.publish.v1.Photo].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeletePhotosRequest {
    /// Required. IDs of the [Photos][google.streetview.publish.v1.Photo]. HTTP
    /// GET requests require the following syntax for the URL query parameter:
    /// `photoIds=<id1>&photoIds=<id2>&...`.
    #[prost(string, repeated, tag = "1")]
    pub photo_ids: ::std::vec::Vec<std::string::String>,
}
/// Response to batch delete of one or more
/// [Photos][google.streetview.publish.v1.Photo].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeletePhotosResponse {
    /// The status for the operation to delete a single
    /// [Photo][google.streetview.publish.v1.Photo] in the batch request.
    #[prost(message, repeated, tag = "1")]
    pub status: ::std::vec::Vec<super::super::super::rpc::Status>,
}
/// Specifies which view of the [Photo][google.streetview.publish.v1.Photo]
/// to include in the response.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PhotoView {
    /// Server reponses do not include the download URL for the photo bytes.
    /// The default value.
    Basic = 0,
    /// Server responses include the download URL for the photo bytes.
    IncludeDownloadUrl = 1,
}
#[doc = r" Generated client implementations."]
pub mod street_view_publish_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Publishes and connects user-contributed photos on Street View."]
    pub struct StreetViewPublishServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StreetViewPublishServiceClient<T>
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
        #[doc = " Creates an upload session to start uploading photo bytes. The method uses"]
        #[doc = " the upload URL of the returned"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef] to upload the bytes for"]
        #[doc = " the [Photo][google.streetview.publish.v1.Photo]."]
        #[doc = ""]
        #[doc = " In addition to the photo requirements shown in"]
        #[doc = " https://support.google.com/maps/answer/7012050?hl=en&ref_topic=6275604,"]
        #[doc = " the photo must meet the following requirements:"]
        #[doc = ""]
        #[doc = " * Photo Sphere XMP metadata must be included in the photo medadata. See"]
        #[doc = " https://developers.google.com/streetview/spherical-metadata for the"]
        #[doc = " required fields."]
        #[doc = " * The pixel size of the photo must meet the size requirements listed in"]
        #[doc = " https://support.google.com/maps/answer/7012050?hl=en&ref_topic=6275604, and"]
        #[doc = " the photo must be a full 360 horizontally."]
        #[doc = ""]
        #[doc = " After the upload completes, the method uses"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef] with"]
        #[doc = " [CreatePhoto][google.streetview.publish.v1.StreetViewPublishService.CreatePhoto]"]
        #[doc = " to create the [Photo][google.streetview.publish.v1.Photo] object entry."]
        pub async fn start_upload(
            &mut self,
            request: impl tonic::IntoRequest<()>,
        ) -> Result<tonic::Response<super::UploadRef>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " After the client finishes uploading the photo with the returned"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef],"]
        #[doc = " [CreatePhoto][google.streetview.publish.v1.StreetViewPublishService.CreatePhoto]"]
        #[doc = " publishes the uploaded [Photo][google.streetview.publish.v1.Photo] to"]
        #[doc = " Street View on Google Maps."]
        #[doc = ""]
        #[doc = " Currently, the only way to set heading, pitch, and roll in CreatePhoto is"]
        #[doc = " through the [Photo Sphere XMP"]
        #[doc = " metadata](https://developers.google.com/streetview/spherical-metadata) in"]
        #[doc = " the photo bytes. CreatePhoto ignores the `pose.heading`, `pose.pitch`,"]
        #[doc = " `pose.roll`, `pose.altitude`, and `pose.level` fields in Pose."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT] if"]
        #[doc = " the request is malformed or if the uploaded photo is not a 360 photo."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the upload"]
        #[doc = " reference does not exist."]
        #[doc = " * [google.rpc.Code.RESOURCE_EXHAUSTED][google.rpc.Code.RESOURCE_EXHAUSTED]"]
        #[doc = " if the account has reached the storage limit."]
        pub async fn create_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Gets the metadata of the specified"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo]."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if"]
        #[doc = " the requesting user did not create the requested"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo]."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the requested"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo] does not exist."]
        #[doc = " * [google.rpc.Code.UNAVAILABLE][google.rpc.Code.UNAVAILABLE] if the"]
        #[doc = " requested [Photo][google.streetview.publish.v1.Photo] is still being"]
        #[doc = " indexed."]
        pub async fn get_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Gets the metadata of the specified"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo] batch."]
        #[doc = ""]
        #[doc = " Note that if"]
        #[doc = " [BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos]"]
        #[doc = " fails, either critical fields are missing or there is an authentication"]
        #[doc = " error. Even if"]
        #[doc = " [BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos]"]
        #[doc = " succeeds, individual photos in the batch may have failures."]
        #[doc = " These failures are specified in each"]
        #[doc = " [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]"]
        #[doc = " in"]
        #[doc = " [BatchGetPhotosResponse.results][google.streetview.publish.v1.BatchGetPhotosResponse.results]."]
        #[doc = " See"]
        #[doc = " [GetPhoto][google.streetview.publish.v1.StreetViewPublishService.GetPhoto]"]
        #[doc = " for specific failures that can occur per photo."]
        pub async fn batch_get_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetPhotosRequest>,
        ) -> Result<tonic::Response<super::BatchGetPhotosResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Lists all the [Photos][google.streetview.publish.v1.Photo] that belong to"]
        #[doc = " the user."]
        #[doc = ""]
        #[doc = " <aside class=\"note\"><b>Note:</b> Recently created photos that are still"]
        #[doc = " being indexed are not returned in the response.</aside>"]
        pub async fn list_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPhotosRequest>,
        ) -> Result<tonic::Response<super::ListPhotosResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Updates the metadata of a [Photo][google.streetview.publish.v1.Photo], such"]
        #[doc = " as pose, place association, connections, etc. Changing the pixels of a"]
        #[doc = " photo is not supported."]
        #[doc = ""]
        #[doc = " Only the fields specified in the"]
        #[doc = " [updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask]"]
        #[doc = " field are used. If `updateMask` is not present, the update applies to all"]
        #[doc = " fields."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if"]
        #[doc = " the requesting user did not create the requested photo."]
        #[doc = " * [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT] if"]
        #[doc = " the request is malformed."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the requested"]
        #[doc = " photo does not exist."]
        #[doc = " * [google.rpc.Code.UNAVAILABLE][google.rpc.Code.UNAVAILABLE] if the"]
        #[doc = " requested [Photo][google.streetview.publish.v1.Photo] is still being"]
        #[doc = " indexed."]
        pub async fn update_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdatePhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Updates the metadata of [Photos][google.streetview.publish.v1.Photo], such"]
        #[doc = " as pose, place association, connections, etc. Changing the pixels of photos"]
        #[doc = " is not supported."]
        #[doc = ""]
        #[doc = " Note that if"]
        #[doc = " [BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos]"]
        #[doc = " fails, either critical fields are missing or there is an authentication"]
        #[doc = " error. Even if"]
        #[doc = " [BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos]"]
        #[doc = " succeeds, individual photos in the batch may have failures."]
        #[doc = " These failures are specified in each"]
        #[doc = " [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]"]
        #[doc = " in"]
        #[doc = " [BatchUpdatePhotosResponse.results][google.streetview.publish.v1.BatchUpdatePhotosResponse.results]."]
        #[doc = " See"]
        #[doc = " [UpdatePhoto][google.streetview.publish.v1.StreetViewPublishService.UpdatePhoto]"]
        #[doc = " for specific failures that can occur per photo."]
        #[doc = ""]
        #[doc = " Only the fields specified in"]
        #[doc = " [updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask]"]
        #[doc = " field are used. If `updateMask` is not present, the update applies to all"]
        #[doc = " fields."]
        #[doc = ""]
        #[doc = " The number of"]
        #[doc = " [UpdatePhotoRequest][google.streetview.publish.v1.UpdatePhotoRequest]"]
        #[doc = " messages in a"]
        #[doc = " [BatchUpdatePhotosRequest][google.streetview.publish.v1.BatchUpdatePhotosRequest]"]
        #[doc = " must not exceed 20."]
        #[doc = ""]
        #[doc = " <aside class=\"note\"><b>Note:</b> To update"]
        #[doc = " [Pose.altitude][google.streetview.publish.v1.Pose.altitude],"]
        #[doc = " [Pose.latLngPair][google.streetview.publish.v1.Pose.lat_lng_pair] has to be"]
        #[doc = " filled as well. Otherwise, the request will fail.</aside>"]
        pub async fn batch_update_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdatePhotosRequest>,
        ) -> Result<tonic::Response<super::BatchUpdatePhotosResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Deletes a [Photo][google.streetview.publish.v1.Photo] and its metadata."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if"]
        #[doc = " the requesting user did not create the requested photo."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the photo ID"]
        #[doc = " does not exist."]
        pub async fn delete_photo(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePhotoRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
        #[doc = " Deletes a list of [Photos][google.streetview.publish.v1.Photo] and their"]
        #[doc = " metadata."]
        #[doc = ""]
        #[doc = " Note that if"]
        #[doc = " [BatchDeletePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchDeletePhotos]"]
        #[doc = " fails, either critical fields are missing or there was an authentication"]
        #[doc = " error. Even if"]
        #[doc = " [BatchDeletePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchDeletePhotos]"]
        #[doc = " succeeds, individual photos in the batch may have failures."]
        #[doc = " These failures are specified in each"]
        #[doc = " [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]"]
        #[doc = " in"]
        #[doc = " [BatchDeletePhotosResponse.results][google.streetview.publish.v1.BatchDeletePhotosResponse.status]."]
        #[doc = " See"]
        #[doc = " [DeletePhoto][google.streetview.publish.v1.StreetViewPublishService.DeletePhoto]"]
        #[doc = " for specific failures that can occur per photo."]
        pub async fn batch_delete_photos(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeletePhotosRequest>,
        ) -> Result<tonic::Response<super::BatchDeletePhotosResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
    impl<T: Clone> Clone for StreetViewPublishServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for StreetViewPublishServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StreetViewPublishServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod street_view_publish_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with StreetViewPublishServiceServer."]
    #[async_trait]
    pub trait StreetViewPublishService: Send + Sync + 'static {
        #[doc = " Creates an upload session to start uploading photo bytes. The method uses"]
        #[doc = " the upload URL of the returned"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef] to upload the bytes for"]
        #[doc = " the [Photo][google.streetview.publish.v1.Photo]."]
        #[doc = ""]
        #[doc = " In addition to the photo requirements shown in"]
        #[doc = " https://support.google.com/maps/answer/7012050?hl=en&ref_topic=6275604,"]
        #[doc = " the photo must meet the following requirements:"]
        #[doc = ""]
        #[doc = " * Photo Sphere XMP metadata must be included in the photo medadata. See"]
        #[doc = " https://developers.google.com/streetview/spherical-metadata for the"]
        #[doc = " required fields."]
        #[doc = " * The pixel size of the photo must meet the size requirements listed in"]
        #[doc = " https://support.google.com/maps/answer/7012050?hl=en&ref_topic=6275604, and"]
        #[doc = " the photo must be a full 360 horizontally."]
        #[doc = ""]
        #[doc = " After the upload completes, the method uses"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef] with"]
        #[doc = " [CreatePhoto][google.streetview.publish.v1.StreetViewPublishService.CreatePhoto]"]
        #[doc = " to create the [Photo][google.streetview.publish.v1.Photo] object entry."]
        async fn start_upload(
            &self,
            request: tonic::Request<()>,
        ) -> Result<tonic::Response<super::UploadRef>, tonic::Status>;
        #[doc = " After the client finishes uploading the photo with the returned"]
        #[doc = " [UploadRef][google.streetview.publish.v1.UploadRef],"]
        #[doc = " [CreatePhoto][google.streetview.publish.v1.StreetViewPublishService.CreatePhoto]"]
        #[doc = " publishes the uploaded [Photo][google.streetview.publish.v1.Photo] to"]
        #[doc = " Street View on Google Maps."]
        #[doc = ""]
        #[doc = " Currently, the only way to set heading, pitch, and roll in CreatePhoto is"]
        #[doc = " through the [Photo Sphere XMP"]
        #[doc = " metadata](https://developers.google.com/streetview/spherical-metadata) in"]
        #[doc = " the photo bytes. CreatePhoto ignores the `pose.heading`, `pose.pitch`,"]
        #[doc = " `pose.roll`, `pose.altitude`, and `pose.level` fields in Pose."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT] if"]
        #[doc = " the request is malformed or if the uploaded photo is not a 360 photo."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the upload"]
        #[doc = " reference does not exist."]
        #[doc = " * [google.rpc.Code.RESOURCE_EXHAUSTED][google.rpc.Code.RESOURCE_EXHAUSTED]"]
        #[doc = " if the account has reached the storage limit."]
        async fn create_photo(
            &self,
            request: tonic::Request<super::CreatePhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status>;
        #[doc = " Gets the metadata of the specified"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo]."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if"]
        #[doc = " the requesting user did not create the requested"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo]."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the requested"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo] does not exist."]
        #[doc = " * [google.rpc.Code.UNAVAILABLE][google.rpc.Code.UNAVAILABLE] if the"]
        #[doc = " requested [Photo][google.streetview.publish.v1.Photo] is still being"]
        #[doc = " indexed."]
        async fn get_photo(
            &self,
            request: tonic::Request<super::GetPhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status>;
        #[doc = " Gets the metadata of the specified"]
        #[doc = " [Photo][google.streetview.publish.v1.Photo] batch."]
        #[doc = ""]
        #[doc = " Note that if"]
        #[doc = " [BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos]"]
        #[doc = " fails, either critical fields are missing or there is an authentication"]
        #[doc = " error. Even if"]
        #[doc = " [BatchGetPhotos][google.streetview.publish.v1.StreetViewPublishService.BatchGetPhotos]"]
        #[doc = " succeeds, individual photos in the batch may have failures."]
        #[doc = " These failures are specified in each"]
        #[doc = " [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]"]
        #[doc = " in"]
        #[doc = " [BatchGetPhotosResponse.results][google.streetview.publish.v1.BatchGetPhotosResponse.results]."]
        #[doc = " See"]
        #[doc = " [GetPhoto][google.streetview.publish.v1.StreetViewPublishService.GetPhoto]"]
        #[doc = " for specific failures that can occur per photo."]
        async fn batch_get_photos(
            &self,
            request: tonic::Request<super::BatchGetPhotosRequest>,
        ) -> Result<tonic::Response<super::BatchGetPhotosResponse>, tonic::Status>;
        #[doc = " Lists all the [Photos][google.streetview.publish.v1.Photo] that belong to"]
        #[doc = " the user."]
        #[doc = ""]
        #[doc = " <aside class=\"note\"><b>Note:</b> Recently created photos that are still"]
        #[doc = " being indexed are not returned in the response.</aside>"]
        async fn list_photos(
            &self,
            request: tonic::Request<super::ListPhotosRequest>,
        ) -> Result<tonic::Response<super::ListPhotosResponse>, tonic::Status>;
        #[doc = " Updates the metadata of a [Photo][google.streetview.publish.v1.Photo], such"]
        #[doc = " as pose, place association, connections, etc. Changing the pixels of a"]
        #[doc = " photo is not supported."]
        #[doc = ""]
        #[doc = " Only the fields specified in the"]
        #[doc = " [updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask]"]
        #[doc = " field are used. If `updateMask` is not present, the update applies to all"]
        #[doc = " fields."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if"]
        #[doc = " the requesting user did not create the requested photo."]
        #[doc = " * [google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT] if"]
        #[doc = " the request is malformed."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the requested"]
        #[doc = " photo does not exist."]
        #[doc = " * [google.rpc.Code.UNAVAILABLE][google.rpc.Code.UNAVAILABLE] if the"]
        #[doc = " requested [Photo][google.streetview.publish.v1.Photo] is still being"]
        #[doc = " indexed."]
        async fn update_photo(
            &self,
            request: tonic::Request<super::UpdatePhotoRequest>,
        ) -> Result<tonic::Response<super::Photo>, tonic::Status>;
        #[doc = " Updates the metadata of [Photos][google.streetview.publish.v1.Photo], such"]
        #[doc = " as pose, place association, connections, etc. Changing the pixels of photos"]
        #[doc = " is not supported."]
        #[doc = ""]
        #[doc = " Note that if"]
        #[doc = " [BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos]"]
        #[doc = " fails, either critical fields are missing or there is an authentication"]
        #[doc = " error. Even if"]
        #[doc = " [BatchUpdatePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchUpdatePhotos]"]
        #[doc = " succeeds, individual photos in the batch may have failures."]
        #[doc = " These failures are specified in each"]
        #[doc = " [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]"]
        #[doc = " in"]
        #[doc = " [BatchUpdatePhotosResponse.results][google.streetview.publish.v1.BatchUpdatePhotosResponse.results]."]
        #[doc = " See"]
        #[doc = " [UpdatePhoto][google.streetview.publish.v1.StreetViewPublishService.UpdatePhoto]"]
        #[doc = " for specific failures that can occur per photo."]
        #[doc = ""]
        #[doc = " Only the fields specified in"]
        #[doc = " [updateMask][google.streetview.publish.v1.UpdatePhotoRequest.update_mask]"]
        #[doc = " field are used. If `updateMask` is not present, the update applies to all"]
        #[doc = " fields."]
        #[doc = ""]
        #[doc = " The number of"]
        #[doc = " [UpdatePhotoRequest][google.streetview.publish.v1.UpdatePhotoRequest]"]
        #[doc = " messages in a"]
        #[doc = " [BatchUpdatePhotosRequest][google.streetview.publish.v1.BatchUpdatePhotosRequest]"]
        #[doc = " must not exceed 20."]
        #[doc = ""]
        #[doc = " <aside class=\"note\"><b>Note:</b> To update"]
        #[doc = " [Pose.altitude][google.streetview.publish.v1.Pose.altitude],"]
        #[doc = " [Pose.latLngPair][google.streetview.publish.v1.Pose.lat_lng_pair] has to be"]
        #[doc = " filled as well. Otherwise, the request will fail.</aside>"]
        async fn batch_update_photos(
            &self,
            request: tonic::Request<super::BatchUpdatePhotosRequest>,
        ) -> Result<tonic::Response<super::BatchUpdatePhotosResponse>, tonic::Status>;
        #[doc = " Deletes a [Photo][google.streetview.publish.v1.Photo] and its metadata."]
        #[doc = ""]
        #[doc = " This method returns the following error codes:"]
        #[doc = ""]
        #[doc = " * [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED] if"]
        #[doc = " the requesting user did not create the requested photo."]
        #[doc = " * [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND] if the photo ID"]
        #[doc = " does not exist."]
        async fn delete_photo(
            &self,
            request: tonic::Request<super::DeletePhotoRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Deletes a list of [Photos][google.streetview.publish.v1.Photo] and their"]
        #[doc = " metadata."]
        #[doc = ""]
        #[doc = " Note that if"]
        #[doc = " [BatchDeletePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchDeletePhotos]"]
        #[doc = " fails, either critical fields are missing or there was an authentication"]
        #[doc = " error. Even if"]
        #[doc = " [BatchDeletePhotos][google.streetview.publish.v1.StreetViewPublishService.BatchDeletePhotos]"]
        #[doc = " succeeds, individual photos in the batch may have failures."]
        #[doc = " These failures are specified in each"]
        #[doc = " [PhotoResponse.status][google.streetview.publish.v1.PhotoResponse.status]"]
        #[doc = " in"]
        #[doc = " [BatchDeletePhotosResponse.results][google.streetview.publish.v1.BatchDeletePhotosResponse.status]."]
        #[doc = " See"]
        #[doc = " [DeletePhoto][google.streetview.publish.v1.StreetViewPublishService.DeletePhoto]"]
        #[doc = " for specific failures that can occur per photo."]
        async fn batch_delete_photos(
            &self,
            request: tonic::Request<super::BatchDeletePhotosRequest>,
        ) -> Result<tonic::Response<super::BatchDeletePhotosResponse>, tonic::Status>;
    }
    #[doc = " Publishes and connects user-contributed photos on Street View."]
    #[derive(Debug)]
    pub struct StreetViewPublishServiceServer<T: StreetViewPublishService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: StreetViewPublishService> StreetViewPublishServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for StreetViewPublishServiceServer<T>
    where
        T: StreetViewPublishService,
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
            match req.uri().path() {
                "/google.streetview.publish.v1.StreetViewPublishService/StartUpload" => {
                    #[allow(non_camel_case_types)]
                    struct StartUploadSvc<T: StreetViewPublishService>(pub Arc<T>);
                    impl<T: StreetViewPublishService> tonic::server::UnaryService<()> for StartUploadSvc<T> {
                        type Response = super::UploadRef;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<()>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_upload(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StartUploadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.streetview.publish.v1.StreetViewPublishService/CreatePhoto" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePhotoSvc<T: StreetViewPublishService>(pub Arc<T>);
                    impl<T: StreetViewPublishService>
                        tonic::server::UnaryService<super::CreatePhotoRequest>
                        for CreatePhotoSvc<T>
                    {
                        type Response = super::Photo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePhotoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_photo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreatePhotoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.streetview.publish.v1.StreetViewPublishService/GetPhoto" => {
                    #[allow(non_camel_case_types)]
                    struct GetPhotoSvc<T: StreetViewPublishService>(pub Arc<T>);
                    impl<T: StreetViewPublishService>
                        tonic::server::UnaryService<super::GetPhotoRequest> for GetPhotoSvc<T>
                    {
                        type Response = super::Photo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPhotoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_photo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetPhotoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.streetview.publish.v1.StreetViewPublishService/BatchGetPhotos" => {
                    #[allow(non_camel_case_types)]
                    struct BatchGetPhotosSvc<T: StreetViewPublishService>(pub Arc<T>);
                    impl<T: StreetViewPublishService>
                        tonic::server::UnaryService<super::BatchGetPhotosRequest>
                        for BatchGetPhotosSvc<T>
                    {
                        type Response = super::BatchGetPhotosResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchGetPhotosRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).batch_get_photos(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchGetPhotosSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.streetview.publish.v1.StreetViewPublishService/ListPhotos" => {
                    #[allow(non_camel_case_types)]
                    struct ListPhotosSvc<T: StreetViewPublishService>(pub Arc<T>);
                    impl<T: StreetViewPublishService>
                        tonic::server::UnaryService<super::ListPhotosRequest> for ListPhotosSvc<T>
                    {
                        type Response = super::ListPhotosResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPhotosRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_photos(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListPhotosSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.streetview.publish.v1.StreetViewPublishService/UpdatePhoto" => {
                    #[allow(non_camel_case_types)]
                    struct UpdatePhotoSvc<T: StreetViewPublishService>(pub Arc<T>);
                    impl<T: StreetViewPublishService>
                        tonic::server::UnaryService<super::UpdatePhotoRequest>
                        for UpdatePhotoSvc<T>
                    {
                        type Response = super::Photo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdatePhotoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_photo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdatePhotoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.streetview.publish.v1.StreetViewPublishService/BatchUpdatePhotos" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUpdatePhotosSvc<T: StreetViewPublishService>(pub Arc<T>);
                    impl<T: StreetViewPublishService>
                        tonic::server::UnaryService<super::BatchUpdatePhotosRequest>
                        for BatchUpdatePhotosSvc<T>
                    {
                        type Response = super::BatchUpdatePhotosResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchUpdatePhotosRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).batch_update_photos(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchUpdatePhotosSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.streetview.publish.v1.StreetViewPublishService/DeletePhoto" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePhotoSvc<T: StreetViewPublishService>(pub Arc<T>);
                    impl<T: StreetViewPublishService>
                        tonic::server::UnaryService<super::DeletePhotoRequest>
                        for DeletePhotoSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePhotoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_photo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeletePhotoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.streetview.publish.v1.StreetViewPublishService/BatchDeletePhotos" => {
                    #[allow(non_camel_case_types)]
                    struct BatchDeletePhotosSvc<T: StreetViewPublishService>(pub Arc<T>);
                    impl<T: StreetViewPublishService>
                        tonic::server::UnaryService<super::BatchDeletePhotosRequest>
                        for BatchDeletePhotosSvc<T>
                    {
                        type Response = super::BatchDeletePhotosResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchDeletePhotosRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).batch_delete_photos(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchDeletePhotosSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: StreetViewPublishService> Clone for StreetViewPublishServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: StreetViewPublishService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
