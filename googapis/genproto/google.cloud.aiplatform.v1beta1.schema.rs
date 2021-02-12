/// An entry of mapping between color and AnnotationSpec. The mapping is used in
/// segmentation mask.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationSpecColor {
    /// The color of the AnnotationSpec in a segmentation mask.
    #[prost(message, optional, tag = "1")]
    pub color: ::core::option::Option<super::super::super::super::r#type::Color>,
    /// The display name of the AnnotationSpec represented by the color in the
    /// segmentation mask.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The ID of the AnnotationSpec represented by the color in the segmentation
    /// mask.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    /// X coordinate.
    #[prost(double, tag = "1")]
    pub x: f64,
    /// Y coordinate.
    #[prost(double, tag = "2")]
    pub y: f64,
}
/// Annotation details specific to image classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationAnnotation {
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "1")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
}
/// Annotation details specific to image object detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageBoundingBoxAnnotation {
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "1")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The leftmost coordinate of the bounding box.
    #[prost(double, tag = "3")]
    pub x_min: f64,
    /// The rightmost coordinate of the bounding box.
    #[prost(double, tag = "4")]
    pub x_max: f64,
    /// The topmost coordinate of the bounding box.
    #[prost(double, tag = "5")]
    pub y_min: f64,
    /// The bottommost coordinate of the bounding box.
    #[prost(double, tag = "6")]
    pub y_max: f64,
}
/// Annotation details specific to image segmentation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationAnnotation {
    #[prost(oneof = "image_segmentation_annotation::Annotation", tags = "3, 4, 5")]
    pub annotation: ::core::option::Option<image_segmentation_annotation::Annotation>,
}
/// Nested message and enum types in `ImageSegmentationAnnotation`.
pub mod image_segmentation_annotation {
    /// The mask based segmentation annotation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MaskAnnotation {
        /// Google Cloud Storage URI that points to the mask image. The image must be
        /// in PNG format. It must have the same size as the DataItem's image. Each
        /// pixel in the image mask represents the AnnotationSpec which the pixel in
        /// the image DataItem belong to. Each color is mapped to one AnnotationSpec
        /// based on annotation_spec_colors.
        #[prost(string, tag = "1")]
        pub mask_gcs_uri: ::prost::alloc::string::String,
        /// The mapping between color and AnnotationSpec for this Annotation.
        #[prost(message, repeated, tag = "2")]
        pub annotation_spec_colors: ::prost::alloc::vec::Vec<super::AnnotationSpecColor>,
    }
    /// Represents a polygon in image.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PolygonAnnotation {
        /// The vertexes are connected one by one and the last vertex is connected to
        /// the first one to represent a polygon.
        #[prost(message, repeated, tag = "1")]
        pub vertexes: ::prost::alloc::vec::Vec<super::Vertex>,
        /// The resource Id of the AnnotationSpec that this Annotation pertains to.
        #[prost(string, tag = "2")]
        pub annotation_spec_id: ::prost::alloc::string::String,
        /// The display name of the AnnotationSpec that this Annotation pertains to.
        #[prost(string, tag = "3")]
        pub display_name: ::prost::alloc::string::String,
    }
    /// Represents a polyline in image.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PolylineAnnotation {
        /// The vertexes are connected one by one and the last vertex in not
        /// connected to the first one.
        #[prost(message, repeated, tag = "1")]
        pub vertexes: ::prost::alloc::vec::Vec<super::Vertex>,
        /// The resource Id of the AnnotationSpec that this Annotation pertains to.
        #[prost(string, tag = "2")]
        pub annotation_spec_id: ::prost::alloc::string::String,
        /// The display name of the AnnotationSpec that this Annotation pertains to.
        #[prost(string, tag = "3")]
        pub display_name: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Annotation {
        /// Mask based segmentation annotation. Only one mask annotation can exist
        /// for one image.
        #[prost(message, tag = "3")]
        MaskAnnotation(MaskAnnotation),
        /// Polygon annotation.
        #[prost(message, tag = "4")]
        PolygonAnnotation(PolygonAnnotation),
        /// Polyline annotation.
        #[prost(message, tag = "5")]
        PolylineAnnotation(PolylineAnnotation),
    }
}
/// Annotation details specific to text classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextClassificationAnnotation {
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "1")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
}
/// Annotation details specific to text extraction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionAnnotation {
    /// The segment of the text content.
    #[prost(message, optional, tag = "1")]
    pub text_segment: ::core::option::Option<TextSegment>,
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "2")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
}
/// The text segment inside of DataItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSegment {
    /// Zero-based character index of the first character of the text
    /// segment (counting characters from the beginning of the text).
    #[prost(uint64, tag = "1")]
    pub start_offset: u64,
    /// Zero-based character index of the first character past the end of
    /// the text segment (counting character from the beginning of the text).
    /// The character at the end_offset is NOT included in the text segment.
    #[prost(uint64, tag = "2")]
    pub end_offset: u64,
    /// The text content in the segment for output only.
    #[prost(string, tag = "3")]
    pub content: ::prost::alloc::string::String,
}
/// Annotation details specific to text sentiment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentAnnotation {
    /// The sentiment score for text.
    #[prost(int32, tag = "1")]
    pub sentiment: i32,
    /// The sentiment max score for text.
    #[prost(int32, tag = "2")]
    pub sentiment_max: i32,
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "3")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
}
/// Annotation details specific to video classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationAnnotation {
    /// This Annotation applies to the time period represented by the TimeSegment.
    /// If it's not set, the Annotation applies to the whole video.
    #[prost(message, optional, tag = "1")]
    pub time_segment: ::core::option::Option<TimeSegment>,
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "2")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
}
/// A time period inside of a DataItem that has a time dimension (e.g. video).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSegment {
    /// Start of the time segment (inclusive), represented as the duration since
    /// the start of the DataItem.
    #[prost(message, optional, tag = "1")]
    pub start_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// End of the time segment (exclusive), represented as the duration since the
    /// start of the DataItem.
    #[prost(message, optional, tag = "2")]
    pub end_time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Annotation details specific to video object tracking.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingAnnotation {
    /// A time (frame) of a video to which this annotation pertains.
    /// Represented as the duration since the video's start.
    #[prost(message, optional, tag = "1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// The leftmost coordinate of the bounding box.
    #[prost(double, tag = "2")]
    pub x_min: f64,
    /// The rightmost coordinate of the bounding box.
    #[prost(double, tag = "3")]
    pub x_max: f64,
    /// The topmost coordinate of the bounding box.
    #[prost(double, tag = "4")]
    pub y_min: f64,
    /// The bottommost coordinate of the bounding box.
    #[prost(double, tag = "5")]
    pub y_max: f64,
    /// The instance of the object, expressed as a positive integer. Used to track
    /// the same object across different frames.
    #[prost(int64, tag = "6")]
    pub instance_id: i64,
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "7")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "8")]
    pub display_name: ::prost::alloc::string::String,
}
/// Annotation details specific to video action recognition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoActionRecognitionAnnotation {
    /// This Annotation applies to the time period represented by the TimeSegment.
    /// If it's not set, the Annotation applies to the whole video.
    #[prost(message, optional, tag = "1")]
    pub time_segment: ::core::option::Option<TimeSegment>,
    /// The resource Id of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "2")]
    pub annotation_spec_id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that this Annotation pertains to.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
}
/// Payload of Image DataItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageDataItem {
    /// Required. Google Cloud Storage URI points to the original image in user's bucket.
    /// The image is up to 30MB in size.
    #[prost(string, tag = "1")]
    pub gcs_uri: ::prost::alloc::string::String,
    /// Output only. The mime type of the content of the image. Only the images in below listed
    /// mime types are supported.
    /// - image/jpeg
    /// - image/gif
    /// - image/png
    /// - image/webp
    /// - image/bmp
    /// - image/tiff
    /// - image/vnd.microsoft.icon
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Payload of Video DataItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoDataItem {
    /// Required. Google Cloud Storage URI points to the original video in user's bucket.
    /// The video is up to 50 GB in size and up to 3 hour in duration.
    #[prost(string, tag = "1")]
    pub gcs_uri: ::prost::alloc::string::String,
    /// Output only. The mime type of the content of the video. Only the videos in below listed
    /// mime types are supported.
    /// Supported mime_type:
    /// - video/mp4
    /// - video/avi
    /// - video/quicktime
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Payload of Text DataItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextDataItem {
    /// Output only. Google Cloud Storage URI points to the original text in user's bucket.
    /// The text file is up to 10MB in size.
    #[prost(string, tag = "1")]
    pub gcs_uri: ::prost::alloc::string::String,
}
/// The metadata of Datasets that contain Image DataItems.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageDatasetMetadata {
    /// Points to a YAML file stored on Google Cloud Storage describing payload of
    /// the Image DataItems that belong to this Dataset.
    #[prost(string, tag = "1")]
    pub data_item_schema_uri: ::prost::alloc::string::String,
    /// Google Cloud Storage Bucket name that contains the blob data of this
    /// Dataset.
    #[prost(string, tag = "2")]
    pub gcs_bucket: ::prost::alloc::string::String,
}
/// The metadata of Datasets that contain Text DataItems.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextDatasetMetadata {
    /// Points to a YAML file stored on Google Cloud Storage describing payload of
    /// the Text DataItems that belong to this Dataset.
    #[prost(string, tag = "1")]
    pub data_item_schema_uri: ::prost::alloc::string::String,
    /// Google Cloud Storage Bucket name that contains the blob data of this
    /// Dataset.
    #[prost(string, tag = "2")]
    pub gcs_bucket: ::prost::alloc::string::String,
}
/// The metadata of Datasets that contain Video DataItems.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoDatasetMetadata {
    /// Points to a YAML file stored on Google Cloud Storage describing payload of
    /// the Video DataItems that belong to this Dataset.
    #[prost(string, tag = "1")]
    pub data_item_schema_uri: ::prost::alloc::string::String,
    /// Google Cloud Storage Bucket name that contains the blob data of this
    /// Dataset.
    #[prost(string, tag = "2")]
    pub gcs_bucket: ::prost::alloc::string::String,
}
/// The metadata of Datasets that contain tables data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TablesDatasetMetadata {
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<tables_dataset_metadata::InputConfig>,
}
/// Nested message and enum types in `TablesDatasetMetadata`.
pub mod tables_dataset_metadata {
    /// The tables Dataset's data source. The Dataset doesn't store the data
    /// directly, but only pointer(s) to its data.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputConfig {
        #[prost(oneof = "input_config::Source", tags = "1, 2")]
        pub source: ::core::option::Option<input_config::Source>,
    }
    /// Nested message and enum types in `InputConfig`.
    pub mod input_config {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            #[prost(message, tag = "1")]
            GcsSource(super::GcsSource),
            #[prost(message, tag = "2")]
            BigquerySource(super::BigQuerySource),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsSource {
        /// Google Cloud Storage URI to a input file, only .csv file is supported.
        #[prost(string, repeated, tag = "1")]
        pub uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQuerySource {
        /// The URI of a BigQuery table.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
}
/// The metadata of Datasets that contain time series data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesDatasetMetadata {
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<time_series_dataset_metadata::InputConfig>,
    /// The column name of the time series identifier column that identifies the
    /// time series.
    #[prost(string, tag = "2")]
    pub time_series_identifier_column: ::prost::alloc::string::String,
    /// The column name of the time column that identifies time order in the time
    /// series.
    #[prost(string, tag = "3")]
    pub time_column: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TimeSeriesDatasetMetadata`.
pub mod time_series_dataset_metadata {
    /// The time series Dataset's data source. The Dataset doesn't store the data
    /// directly, but only pointer(s) to its data.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputConfig {
        #[prost(oneof = "input_config::Source", tags = "1, 2")]
        pub source: ::core::option::Option<input_config::Source>,
    }
    /// Nested message and enum types in `InputConfig`.
    pub mod input_config {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            #[prost(message, tag = "1")]
            GcsSource(super::GcsSource),
            #[prost(message, tag = "2")]
            BigquerySource(super::BigQuerySource),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsSource {
        /// Google Cloud Storage URI to a input file, only .csv file is supported.
        #[prost(string, repeated, tag = "1")]
        pub uri: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQuerySource {
        /// The URI of a BigQuery table.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
}
/// Prediction input format for Image Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationPredictionInstance {
    /// The image bytes or GCS URI to make the prediction on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the image. Only the images in below listed
    /// MIME types are supported.
    /// - image/jpeg
    /// - image/gif
    /// - image/png
    /// - image/webp
    /// - image/bmp
    /// - image/tiff
    /// - image/vnd.microsoft.icon
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Prediction input format for Image Object Detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionPredictionInstance {
    /// The image bytes or GCS URI to make the prediction on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the image. Only the images in below listed
    /// MIME types are supported.
    /// - image/jpeg
    /// - image/gif
    /// - image/png
    /// - image/webp
    /// - image/bmp
    /// - image/tiff
    /// - image/vnd.microsoft.icon
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Prediction input format for Image Segmentation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationPredictionInstance {
    /// The image bytes to make the predictions on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the image. Only the images in below listed
    /// MIME types are supported.
    /// - image/jpeg
    /// - image/png
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Prediction input format for Video Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationPredictionInstance {
    /// The Google Cloud Storage location of the video on which to perform the
    /// prediction.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the video. Only the following are
    /// supported: video/mp4 video/avi video/quicktime
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision.
    #[prost(string, tag = "3")]
    pub time_segment_start: ::prost::alloc::string::String,
    /// The end, exclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision, and "Infinity" is allowed, which means the
    /// end of the video.
    #[prost(string, tag = "4")]
    pub time_segment_end: ::prost::alloc::string::String,
}
/// Prediction input format for Video Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingPredictionInstance {
    /// The Google Cloud Storage location of the video on which to perform the
    /// prediction.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the video. Only the following are
    /// supported: video/mp4 video/avi video/quicktime
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision.
    #[prost(string, tag = "3")]
    pub time_segment_start: ::prost::alloc::string::String,
    /// The end, exclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision, and "Infinity" is allowed, which means the
    /// end of the video.
    #[prost(string, tag = "4")]
    pub time_segment_end: ::prost::alloc::string::String,
}
/// Prediction input format for Video Action Recognition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoActionRecognitionPredictionInstance {
    /// The Google Cloud Storage location of the video on which to perform the
    /// prediction.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the content of the video. Only the following are
    /// supported: video/mp4 video/avi video/quicktime
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision.
    #[prost(string, tag = "3")]
    pub time_segment_start: ::prost::alloc::string::String,
    /// The end, exclusive, of the video's time segment on which to perform
    /// the prediction. Expressed as a number of seconds as measured from the
    /// start of the video, with "s" appended at the end. Fractions are allowed,
    /// up to a microsecond precision, and "Infinity" is allowed, which means the
    /// end of the video.
    #[prost(string, tag = "4")]
    pub time_segment_end: ::prost::alloc::string::String,
}
/// Prediction input format for Text Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextClassificationPredictionInstance {
    /// The text snippet to make the predictions on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the text snippet. The supported MIME types are listed
    /// below.
    /// - text/plain
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Prediction input format for Text Sentiment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentPredictionInstance {
    /// The text snippet to make the predictions on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the text snippet. The supported MIME types are listed
    /// below.
    /// - text/plain
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Prediction input format for Text Extraction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionPredictionInstance {
    /// The text snippet to make the predictions on.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The MIME type of the text snippet. The supported MIME types are listed
    /// below.
    /// - text/plain
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// This field is only used for batch prediction. If a key is provided, the
    /// batch prediction result will by mapped to this key. If omitted, then the
    /// batch prediction result will contain the entire input instance. AI Platform
    /// will not check if keys in the request are duplicates, so it is up to the
    /// caller to ensure the keys are unique.
    #[prost(string, tag = "3")]
    pub key: ::prost::alloc::string::String,
}
/// Prediction model parameters for Image Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageClassificationPredictionParams {
    /// The Model only returns predictions with at least this confidence score.
    /// Default value is 0.0
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// The Model only returns up to that many top, by confidence score,
    /// predictions per instance. If this number is very high, the Model may return
    /// fewer predictions. Default value is 10.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
}
/// Prediction model parameters for Image Object Detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionPredictionParams {
    /// The Model only returns predictions with at least this confidence score.
    /// Default value is 0.0
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// The Model only returns up to that many top, by confidence score,
    /// predictions per instance. Note that number of returned predictions is also
    /// limited by metadata's predictionsLimit. Default value is 10.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
}
/// Prediction model parameters for Image Segmentation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSegmentationPredictionParams {
    /// When the model predicts category of pixels of the image, it will only
    /// provide predictions for pixels that it is at least this much confident
    /// about. All other pixels will be classified as background. Default value is
    /// 0.5.
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
}
/// Prediction model parameters for Video Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationPredictionParams {
    /// The Model only returns predictions with at least this confidence score.
    /// Default value is 0.0
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// The Model only returns up to that many top, by confidence score,
    /// predictions per instance. If this number is very high, the Model may return
    /// fewer predictions. Default value is 10,000.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
    /// Set to true to request segment-level classification. AI Platform returns
    /// labels and their confidence scores for the entire time segment of the
    /// video that user specified in the input instance.
    /// Default value is true
    #[prost(bool, tag = "3")]
    pub segment_classification: bool,
    /// Set to true to request shot-level classification. AI Platform determines
    /// the boundaries for each camera shot in the entire time segment of the
    /// video that user specified in the input instance. AI Platform then
    /// returns labels and their confidence scores for each detected shot, along
    /// with the start and end time of the shot.
    /// WARNING: Model evaluation is not done for this classification type,
    /// the quality of it depends on the training data, but there are no metrics
    /// provided to describe that quality.
    /// Default value is false
    #[prost(bool, tag = "4")]
    pub shot_classification: bool,
    /// Set to true to request classification for a video at one-second intervals.
    /// AI Platform returns labels and their confidence scores for each second of
    /// the entire time segment of the video that user specified in the input
    /// WARNING: Model evaluation is not done for this classification type, the
    /// quality of it depends on the training data, but there are no metrics
    /// provided to describe that quality. Default value is false
    #[prost(bool, tag = "5")]
    pub one_sec_interval_classification: bool,
}
/// Prediction model parameters for Video Object Tracking.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingPredictionParams {
    /// The Model only returns predictions with at least this confidence score.
    /// Default value is 0.0
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// The model only returns up to that many top, by confidence score,
    /// predictions per frame of the video. If this number is very high, the
    /// Model may return fewer predictions per frame. Default value is 50.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
    /// Only bounding boxes with shortest edge at least that long as a relative
    /// value of video frame size are returned. Default value is 0.0.
    #[prost(float, tag = "3")]
    pub min_bounding_box_size: f32,
}
/// Prediction model parameters for Video Action Recognition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoActionRecognitionPredictionParams {
    /// The Model only returns predictions with at least this confidence score.
    /// Default value is 0.0
    #[prost(float, tag = "1")]
    pub confidence_threshold: f32,
    /// The model only returns up to that many top, by confidence score,
    /// predictions per frame of the video. If this number is very high, the
    /// Model may return fewer predictions per frame. Default value is 50.
    #[prost(int32, tag = "2")]
    pub max_predictions: i32,
}
/// Represents a line of JSONL in the batch prediction output file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictionResult {
    /// The prediction result.
    /// Value is used here instead of Any so that JsonFormat does not append an
    /// extra "@type" field when we convert the proto to JSON and so we can
    /// represent array of objects.
    #[prost(message, optional, tag = "3")]
    pub prediction: ::core::option::Option<::prost_types::Value>,
    /// Some identifier from the input so that the prediction can be mapped back to
    /// the input instance.
    #[prost(oneof = "prediction_result::Input", tags = "1, 2")]
    pub input: ::core::option::Option<prediction_result::Input>,
}
/// Nested message and enum types in `PredictionResult`.
pub mod prediction_result {
    /// Some identifier from the input so that the prediction can be mapped back to
    /// the input instance.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        /// User's input instance.
        /// Struct is used here instead of Any so that JsonFormat does not append an
        /// extra "@type" field when we convert the proto to JSON.
        #[prost(message, tag = "1")]
        Instance(::prost_types::Struct),
        /// Optional user-provided key from the input instance.
        #[prost(string, tag = "2")]
        Key(::prost::alloc::string::String),
    }
}
/// Represents a line of JSONL in the text sentiment batch prediction output
/// file. This is a hack to allow printing of integer values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentPredictionResult {
    /// User's input instance.
    #[prost(message, optional, tag = "1")]
    pub instance: ::core::option::Option<TextSentimentPredictionInstance>,
    /// The prediction result.
    #[prost(message, optional, tag = "2")]
    pub prediction: ::core::option::Option<text_sentiment_prediction_result::Prediction>,
}
/// Nested message and enum types in `TextSentimentPredictionResult`.
pub mod text_sentiment_prediction_result {
    /// Prediction output format for Text Sentiment.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Prediction {
        /// The integer sentiment labels between 0 (inclusive) and sentimentMax label
        /// (inclusive), while 0 maps to the least positive sentiment and
        /// sentimentMax maps to the most positive one. The higher the score is, the
        /// more positive the sentiment in the text snippet is. Note: sentimentMax is
        /// an integer value between 1 (inclusive) and 10 (inclusive).
        #[prost(int32, tag = "1")]
        pub sentiment: i32,
    }
}
/// Prediction output format for Image Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClassificationPredictionResult {
    /// The resource IDs of the AnnotationSpecs that had been identified, ordered
    /// by the confidence score descendingly.
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    /// The display names of the AnnotationSpecs that had been identified, order
    /// matches the IDs.
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Model's confidences in correctness of the predicted IDs, higher value
    /// means higher confidence. Order matches the Ids.
    #[prost(float, repeated, tag = "3")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
}
/// Prediction output format for Image Object Detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageObjectDetectionPredictionResult {
    /// The resource IDs of the AnnotationSpecs that had been identified, ordered
    /// by the confidence score descendingly.
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    /// The display names of the AnnotationSpecs that had been identified, order
    /// matches the IDs.
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Model's confidences in correctness of the predicted IDs, higher value
    /// means higher confidence. Order matches the Ids.
    #[prost(float, repeated, tag = "3")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
    /// Bounding boxes, i.e. the rectangles over the image, that pinpoint
    /// the found AnnotationSpecs. Given in order that matches the IDs. Each
    /// bounding box is an array of 4 numbers `xMin`, `xMax`, `yMin`, and
    /// `yMax`, which represent the extremal coordinates of the box. They are
    /// relative to the image size, and the point 0,0 is in the top left
    /// of the image.
    #[prost(message, repeated, tag = "4")]
    pub bboxes: ::prost::alloc::vec::Vec<::prost_types::ListValue>,
}
/// Prediction output format for Video Classification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoClassificationPredictionResult {
    /// The resource ID of the AnnotationSpec that had been identified.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that had been identified.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The type of the prediction. The requested types can be configured
    /// via parameters. This will be one of
    /// - segment-classification
    /// - shot-classification
    /// - one-sec-interval-classification
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment in which the
    /// AnnotationSpec has been identified. Expressed as a number of seconds as
    /// measured from the start of the video, with fractions up to a microsecond
    /// precision, and with "s" appended at the end. Note that for
    /// 'segment-classification' prediction type, this equals the original
    /// 'timeSegmentStart' from the input instance, for other types it is the
    /// start of a shot or a 1 second interval respectively.
    #[prost(message, optional, tag = "4")]
    pub time_segment_start: ::core::option::Option<::prost_types::Duration>,
    /// The end, exclusive, of the video's time segment in which the
    /// AnnotationSpec has been identified. Expressed as a number of seconds as
    /// measured from the start of the video, with fractions up to a microsecond
    /// precision, and with "s" appended at the end. Note that for
    /// 'segment-classification' prediction type, this equals the original
    /// 'timeSegmentEnd' from the input instance, for other types it is the end
    /// of a shot or a 1 second interval respectively.
    #[prost(message, optional, tag = "5")]
    pub time_segment_end: ::core::option::Option<::prost_types::Duration>,
    /// The Model's confidence in correction of this prediction, higher
    /// value means higher confidence.
    #[prost(message, optional, tag = "6")]
    pub confidence: ::core::option::Option<f32>,
}
/// Prediction output format for Video Object Tracking.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoObjectTrackingPredictionResult {
    /// The resource ID of the AnnotationSpec that had been identified.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The display name of the AnnotationSpec that had been identified.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The beginning, inclusive, of the video's time segment in which the
    /// object instance has been detected. Expressed as a number of seconds as
    /// measured from the start of the video, with fractions up to a microsecond
    /// precision, and with "s" appended at the end.
    #[prost(message, optional, tag = "3")]
    pub time_segment_start: ::core::option::Option<::prost_types::Duration>,
    /// The end, inclusive, of the video's time segment in which the
    /// object instance has been detected. Expressed as a number of seconds as
    /// measured from the start of the video, with fractions up to a microsecond
    /// precision, and with "s" appended at the end.
    #[prost(message, optional, tag = "4")]
    pub time_segment_end: ::core::option::Option<::prost_types::Duration>,
    /// The Model's confidence in correction of this prediction, higher
    /// value means higher confidence.
    #[prost(message, optional, tag = "5")]
    pub confidence: ::core::option::Option<f32>,
    /// All of the frames of the video in which a single object instance has been
    /// detected. The bounding boxes in the frames identify the same object.
    #[prost(message, repeated, tag = "6")]
    pub frames: ::prost::alloc::vec::Vec<video_object_tracking_prediction_result::Frame>,
}
/// Nested message and enum types in `VideoObjectTrackingPredictionResult`.
pub mod video_object_tracking_prediction_result {
    /// The fields `xMin`, `xMax`, `yMin`, and `yMax` refer to a bounding box,
    /// i.e. the rectangle over the video frame pinpointing the found
    /// AnnotationSpec. The coordinates are relative to the frame size, and the
    /// point 0,0 is in the top left of the frame.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Frame {
        /// A time (frame) of a video in which the object has been detected.
        /// Expressed as a number of seconds as measured from the
        /// start of the video, with fractions up to a microsecond precision, and
        /// with "s" appended at the end.
        #[prost(message, optional, tag = "1")]
        pub time_offset: ::core::option::Option<::prost_types::Duration>,
        /// The leftmost coordinate of the bounding box.
        #[prost(message, optional, tag = "2")]
        pub x_min: ::core::option::Option<f32>,
        /// The rightmost coordinate of the bounding box.
        #[prost(message, optional, tag = "3")]
        pub x_max: ::core::option::Option<f32>,
        /// The topmost coordinate of the bounding box.
        #[prost(message, optional, tag = "4")]
        pub y_min: ::core::option::Option<f32>,
        /// The bottommost coordinate of the bounding box.
        #[prost(message, optional, tag = "5")]
        pub y_max: ::core::option::Option<f32>,
    }
}
/// Prediction output format for Text Extraction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextExtractionPredictionResult {
    /// The resource IDs of the AnnotationSpecs that had been identified,
    /// ordered by the confidence score descendingly.
    #[prost(int64, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i64>,
    /// The display names of the AnnotationSpecs that had been identified,
    /// order matches the IDs.
    #[prost(string, repeated, tag = "2")]
    pub display_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The start offsets, inclusive, of the text segment in which the
    /// AnnotationSpec has been identified. Expressed as a zero-based number
    /// of characters as measured from the start of the text snippet.
    #[prost(int64, repeated, tag = "3")]
    pub text_segment_start_offsets: ::prost::alloc::vec::Vec<i64>,
    /// The end offsets, inclusive, of the text segment in which the
    /// AnnotationSpec has been identified. Expressed as a zero-based number
    /// of characters as measured from the start of the text snippet.
    #[prost(int64, repeated, tag = "4")]
    pub text_segment_end_offsets: ::prost::alloc::vec::Vec<i64>,
    /// The Model's confidences in correctness of the predicted IDs, higher
    /// value means higher confidence. Order matches the Ids.
    #[prost(float, repeated, tag = "5")]
    pub confidences: ::prost::alloc::vec::Vec<f32>,
}
/// The metadata of SavedQuery contains TextSentiment Annotations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextSentimentSavedQueryMetadata {
    /// The maximum sentiment of sentiment Anntoation in this SavedQuery.
    #[prost(int32, tag = "1")]
    pub sentiment_max: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisualInspectionClassificationLabelSavedQueryMetadata {
    /// Whether or not the classification label is multi_label.
    #[prost(bool, tag = "1")]
    pub multi_label: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisualInspectionMaskSavedQueryMetadata {
    /// The mapping between color and AnnotationSpec for this SavedQuery.
    #[prost(message, repeated, tag = "2")]
    pub color_map: ::prost::alloc::vec::Vec<AnnotationSpecColor>,
}
