/// Represents a hardware accelerator type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AcceleratorType {
    /// Unspecified accelerator type, which means no accelerator.
    Unspecified = 0,
    /// Nvidia Tesla K80 GPU.
    NvidiaTeslaK80 = 1,
    /// Nvidia Tesla P100 GPU.
    NvidiaTeslaP100 = 2,
    /// Nvidia Tesla V100 GPU.
    NvidiaTeslaV100 = 3,
    /// Nvidia Tesla P4 GPU.
    NvidiaTeslaP4 = 4,
    /// Nvidia Tesla T4 GPU.
    NvidiaTeslaT4 = 5,
}
/// References an API call. It contains more information about long running
/// operation and Jobs that are triggered by the API call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserActionReference {
    /// The method name of the API call. For example,
    /// "/google.cloud.aiplatform.v1alpha1.DatasetService.CreateDataset"
    #[prost(string, tag = "3")]
    pub method: ::prost::alloc::string::String,
    #[prost(oneof = "user_action_reference::Reference", tags = "1, 2")]
    pub reference: ::core::option::Option<user_action_reference::Reference>,
}
/// Nested message and enum types in `UserActionReference`.
pub mod user_action_reference {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Reference {
        /// For API calls that return a long running operation.
        /// Resource name of the long running operation.
        /// Format:
        /// 'projects/{project}/locations/{location}/operations/{operation}'
        #[prost(string, tag = "1")]
        Operation(::prost::alloc::string::String),
        /// For API calls that start a LabelingJob.
        /// Resource name of the LabelingJob.
        /// Format:
        /// 'projects/{project}/locations/{location}/dataLabelingJobs/{data_labeling_job}'
        #[prost(string, tag = "2")]
        DataLabelingJob(::prost::alloc::string::String),
    }
}
/// Used to assign specific AnnotationSpec to a particular area of a DataItem or
/// the whole part of the DataItem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotation {
    /// Output only. Resource name of the Annotation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Google Cloud Storage URI points to a YAML file describing [payload][google.cloud.aiplatform.v1beta1.Annotation.payload]. The
    /// schema is defined as an
    /// [OpenAPI 3.0.2 Schema Object](https://tinyurl.com/y538mdwt).
    /// The schema files that can be used here are found in
    /// gs://google-cloud-aiplatform/schema/dataset/annotation/, note that the
    /// chosen schema must be consistent with the parent Dataset's
    /// [metadata][google.cloud.aiplatform.v1beta1.Dataset.metadata_schema_uri].
    #[prost(string, tag = "2")]
    pub payload_schema_uri: ::prost::alloc::string::String,
    /// Required. The schema of the payload can be found in
    /// [payload_schema][google.cloud.aiplatform.v1beta1.Annotation.payload_schema_uri].
    #[prost(message, optional, tag = "3")]
    pub payload: ::core::option::Option<::prost_types::Value>,
    /// Output only. Timestamp when this Annotation was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when this Annotation was last updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Used to perform consistent read-modify-write updates. If not set, a blind
    /// "overwrite" update happens.
    #[prost(string, tag = "8")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. The source of the Annotation.
    #[prost(message, optional, tag = "5")]
    pub annotation_source: ::core::option::Option<UserActionReference>,
    /// Optional. The labels with user-defined metadata to organize your Annotations.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// No more than 64 user labels can be associated with one Annotation(System
    /// labels are excluded).
    ///
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    /// System reserved label keys are prefixed with "aiplatform.googleapis.com/"
    /// and are immutable. Following system labels exist for each Annotation:
    ///
    /// * "aiplatform.googleapis.com/annotation_set_name":
    ///   optional, name of the UI's annotation set this Annotation belongs to.
    ///   If not set, the Annotation is not visible in the UI.
    ///
    /// * "aiplatform.googleapis.com/payload_schema":
    ///   output only, its value is the [payload_schema's][google.cloud.aiplatform.v1beta1.Annotation.payload_schema_uri]
    ///   title.
    #[prost(map = "string, string", tag = "6")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Identifies a concept with which DataItems may be annotated with.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationSpec {
    /// Output only. Resource name of the AnnotationSpec.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The user-defined name of the AnnotationSpec.
    /// The name can be up to 128 characters long and can be consist of any UTF-8
    /// characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Timestamp when this AnnotationSpec was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when AnnotationSpec was last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Used to perform consistent read-modify-write updates. If not set, a blind
    /// "overwrite" update happens.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
}
/// Success and error statistics of processing multiple entities
/// (for example, DataItems or structured data rows) in batch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletionStats {
    /// Output only. The number of entities that had been processed successfully.
    #[prost(int64, tag = "1")]
    pub successful_count: i64,
    /// Output only. The number of entities for which any error was encountered.
    #[prost(int64, tag = "2")]
    pub failed_count: i64,
    /// Output only. In cases when enough errors are encountered a job, pipeline, or operation
    /// may be failed as a whole. Below is the number of entities for which the
    /// processing had not been finished (either in successful or failed state).
    /// Set to -1 if the number is unknown (for example, the operation failed
    /// before the total entity number could be collected).
    #[prost(int64, tag = "3")]
    pub incomplete_count: i64,
}
/// Represents a customer-managed encryption key spec that can be applied to
/// a top-level resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionSpec {
    /// Required. The Cloud KMS resource identifier of the customer managed encryption key
    /// used to protect a resource. Has the form:
    /// `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`.
    /// The key needs to be in the same region as where the compute resource is
    /// created.
    #[prost(string, tag = "1")]
    pub kms_key_name: ::prost::alloc::string::String,
}
/// Metadata describing the Model's input and output for explanation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplanationMetadata {
    /// Required. Map from feature names to feature input metadata. Keys are the name of the
    /// features. Values are the specification of the feature.
    ///
    /// An empty InputMetadata is valid. It describes a text feature which has the
    /// name specified as the key in [ExplanationMetadata.inputs][google.cloud.aiplatform.v1beta1.ExplanationMetadata.inputs]. The baseline
    /// of the empty feature is chosen by AI Platform.
    ///
    /// For AI Platform provided Tensorflow images, the key can be any friendly
    /// name of the feature. Once specified,
    /// [featureAttributions][google.cloud.aiplatform.v1beta1.Attribution.feature_attributions] are keyed by
    /// this key (if not grouped with another feature).
    ///
    /// For custom images, the key must match with the key in
    /// [instance][google.cloud.aiplatform.v1beta1.ExplainRequest.instances].
    #[prost(map = "string, message", tag = "1")]
    pub inputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        explanation_metadata::InputMetadata,
    >,
    /// Required. Map from output names to output metadata.
    ///
    /// For AI Platform provided Tensorflow images, keys can be any user defined
    /// string that consists of any UTF-8 characters.
    ///
    /// For custom images, keys are the name of the output field in the prediction
    /// to be explained.
    ///
    /// Currently only one key is allowed.
    #[prost(map = "string, message", tag = "2")]
    pub outputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        explanation_metadata::OutputMetadata,
    >,
    /// Points to a YAML file stored on Google Cloud Storage describing the format
    /// of the [feature attributions][google.cloud.aiplatform.v1beta1.Attribution.feature_attributions].
    /// The schema is defined as an OpenAPI 3.0.2
    /// [Schema Object](https://tinyurl.com/y538mdwt#schema-object).
    /// AutoML tabular Models always have this field populated by AI Platform.
    /// Note: The URI given on output may be different, including the URI scheme,
    /// than the one given on input. The output URI will point to a location where
    /// the user only has a read access.
    #[prost(string, tag = "3")]
    pub feature_attributions_schema_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ExplanationMetadata`.
pub mod explanation_metadata {
    /// Metadata of the input of a feature.
    ///
    /// Fields other than [InputMetadata.input_baselines][google.cloud.aiplatform.v1beta1.ExplanationMetadata.InputMetadata.input_baselines] are applicable only
    /// for Models that are using AI Platform-provided images for Tensorflow.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputMetadata {
        /// Baseline inputs for this feature.
        ///
        /// If no baseline is specified, AI Platform chooses the baseline for this
        /// feature. If multiple baselines are specified, AI Platform returns the
        /// average attributions across them in
        /// [Attributions.baseline_attribution][].
        ///
        /// For AI Platform provided Tensorflow images (both 1.x and 2.x), the shape
        /// of each baseline must match the shape of the input tensor. If a scalar is
        /// provided, we broadcast to the same shape as the input tensor.
        ///
        /// For custom images, the element of the baselines must be in the same
        /// format as the feature's input in the
        /// [instance][google.cloud.aiplatform.v1beta1.ExplainRequest.instances][]. The schema of any single instance
        /// may be specified via Endpoint's DeployedModels'
        /// [Model's][google.cloud.aiplatform.v1beta1.DeployedModel.model]
        /// [PredictSchemata's][google.cloud.aiplatform.v1beta1.Model.predict_schemata]
        /// [instance_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.instance_schema_uri].
        #[prost(message, repeated, tag = "1")]
        pub input_baselines: ::prost::alloc::vec::Vec<::prost_types::Value>,
        /// Name of the input tensor for this feature. Required and is only
        /// applicable to AI Platform provided images for Tensorflow.
        #[prost(string, tag = "2")]
        pub input_tensor_name: ::prost::alloc::string::String,
        /// Defines how the feature is encoded into the input tensor. Defaults to
        /// IDENTITY.
        #[prost(enumeration = "input_metadata::Encoding", tag = "3")]
        pub encoding: i32,
        /// Modality of the feature. Valid values are: numeric, image. Defaults to
        /// numeric.
        #[prost(string, tag = "4")]
        pub modality: ::prost::alloc::string::String,
        /// The domain details of the input feature value. Like min/max, original
        /// mean or standard deviation if normalized.
        #[prost(message, optional, tag = "5")]
        pub feature_value_domain: ::core::option::Option<input_metadata::FeatureValueDomain>,
        /// Specifies the index of the values of the input tensor.
        /// Required when the input tensor is a sparse representation. Refer to
        /// Tensorflow documentation for more details:
        /// https://www.tensorflow.org/api_docs/python/tf/sparse/SparseTensor.
        #[prost(string, tag = "6")]
        pub indices_tensor_name: ::prost::alloc::string::String,
        /// Specifies the shape of the values of the input if the input is a sparse
        /// representation. Refer to Tensorflow documentation for more details:
        /// https://www.tensorflow.org/api_docs/python/tf/sparse/SparseTensor.
        #[prost(string, tag = "7")]
        pub dense_shape_tensor_name: ::prost::alloc::string::String,
        /// A list of feature names for each index in the input tensor.
        /// Required when the input [InputMetadata.encoding][google.cloud.aiplatform.v1beta1.ExplanationMetadata.InputMetadata.encoding] is BAG_OF_FEATURES,
        /// BAG_OF_FEATURES_SPARSE, INDICATOR.
        #[prost(string, repeated, tag = "8")]
        pub index_feature_mapping: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Encoded tensor is a transformation of the input tensor. Must be provided
        /// if choosing [Integrated Gradients
        /// attribution][ExplanationParameters.integrated_gradients_attribution] or
        /// [XRAI attribution][google.cloud.aiplatform.v1beta1.ExplanationParameters.xrai_attribution]
        /// and the input tensor is not differentiable.
        ///
        /// An encoded tensor is generated if the input tensor is encoded by a lookup
        /// table.
        #[prost(string, tag = "9")]
        pub encoded_tensor_name: ::prost::alloc::string::String,
        /// A list of baselines for the encoded tensor.
        ///
        /// The shape of each baseline should match the shape of the encoded tensor.
        /// If a scalar is provided, AI Platform broadcast to the same shape as the
        /// encoded tensor.
        #[prost(message, repeated, tag = "10")]
        pub encoded_baselines: ::prost::alloc::vec::Vec<::prost_types::Value>,
        /// Visualization configurations for image explanation.
        #[prost(message, optional, tag = "11")]
        pub visualization: ::core::option::Option<input_metadata::Visualization>,
        /// Name of the group that the input belongs to. Features with the same group
        /// name will be treated as one feature when computing attributions. Features
        /// grouped together can have different shapes in value. If provided, there
        /// will be one single attribution generated in [
        /// featureAttributions][Attribution.feature_attributions], keyed by the
        /// group name.
        #[prost(string, tag = "12")]
        pub group_name: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `InputMetadata`.
    pub mod input_metadata {
        /// Domain details of the input feature value. Provides numeric information
        /// about the feature, such as its range (min, max). If the feature has been
        /// pre-processed, for example with z-scoring, then it provides information
        /// about how to recover the original feature. For example, if the input
        /// feature is an image and it has been pre-processed to obtain 0-mean and
        /// stddev = 1 values, then original_mean, and original_stddev refer to the
        /// mean and stddev of the original feature (e.g. image tensor) from which
        /// input feature (with mean = 0 and stddev = 1) was obtained.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FeatureValueDomain {
            /// The minimum permissible value for this feature.
            #[prost(float, tag = "1")]
            pub min_value: f32,
            /// The maximum permissible value for this feature.
            #[prost(float, tag = "2")]
            pub max_value: f32,
            /// If this input feature has been normalized to a mean value of 0,
            /// the original_mean specifies the mean value of the domain prior to
            /// normalization.
            #[prost(float, tag = "3")]
            pub original_mean: f32,
            /// If this input feature has been normalized to a standard deviation of
            /// 1.0, the original_stddev specifies the standard deviation of the domain
            /// prior to normalization.
            #[prost(float, tag = "4")]
            pub original_stddev: f32,
        }
        /// Visualization configurations for image explanation.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Visualization {
            /// Type of the image visualization. Only applicable to [Integrated
            /// Gradients attribution]
            /// [ExplanationParameters.integrated_gradients_attribution]. OUTLINES
            /// shows regions of attribution, while PIXELS shows per-pixel attribution.
            /// Defaults to OUTLINES.
            #[prost(enumeration = "visualization::Type", tag = "1")]
            pub r#type: i32,
            /// Whether to only highlight pixels with positive contributions, negative
            /// or both. Defaults to POSITIVE.
            #[prost(enumeration = "visualization::Polarity", tag = "2")]
            pub polarity: i32,
            /// The color scheme used for the highlighted areas.
            ///
            /// Defaults to PINK_GREEN for [Integrated Gradients
            /// attribution][ExplanationParameters.integrated_gradients_attribution],
            /// which shows positive attributions in green and negative in pink.
            ///
            /// Defaults to VIRIDIS for
            /// [XRAI attribution][google.cloud.aiplatform.v1beta1.ExplanationParameters.xrai_attribution], which
            /// highlights the most influential regions in yellow and the least
            /// influential in blue.
            #[prost(enumeration = "visualization::ColorMap", tag = "3")]
            pub color_map: i32,
            /// Excludes attributions above the specified percentile from the
            /// highlighted areas. Using the clip_percent_upperbound and
            /// clip_percent_lowerbound together can be useful for filtering out noise
            /// and making it easier to see areas of strong attribution. Defaults to
            /// 99.9.
            #[prost(float, tag = "4")]
            pub clip_percent_upperbound: f32,
            /// Excludes attributions below the specified percentile, from the
            /// highlighted areas. Defaults to 35.
            #[prost(float, tag = "5")]
            pub clip_percent_lowerbound: f32,
            /// How the original image is displayed in the visualization.
            /// Adjusting the overlay can help increase visual clarity if the original
            /// image makes it difficult to view the visualization. Defaults to NONE.
            #[prost(enumeration = "visualization::OverlayType", tag = "6")]
            pub overlay_type: i32,
        }
        /// Nested message and enum types in `Visualization`.
        pub mod visualization {
            /// Type of the image visualization. Only applicable to [Integrated
            /// Gradients attribution]
            /// [ExplanationParameters.integrated_gradients_attribution].
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum Type {
                /// Should not be used.
                Unspecified = 0,
                /// Shows which pixel contributed to the image prediction.
                Pixels = 1,
                /// Shows which region contributed to the image prediction by outlining
                /// the region.
                Outlines = 2,
            }
            /// Whether to only highlight pixels with positive contributions, negative
            /// or both. Defaults to POSITIVE.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum Polarity {
                /// Default value. This is the same as POSITIVE.
                Unspecified = 0,
                /// Highlights the pixels/outlines that were most influential to the
                /// model's prediction.
                Positive = 1,
                /// Setting polarity to negative highlights areas that does not lead to
                /// the models's current prediction.
                Negative = 2,
                /// Shows both positive and negative attributions.
                Both = 3,
            }
            /// The color scheme used for highlighting areas.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum ColorMap {
                /// Should not be used.
                Unspecified = 0,
                /// Positive: green. Negative: pink.
                PinkGreen = 1,
                /// Viridis color map: A perceptually uniform color mapping which is
                /// easier to see by those with colorblindness and progresses from yellow
                /// to green to blue. Positive: yellow. Negative: blue.
                Viridis = 2,
                /// Positive: red. Negative: red.
                Red = 3,
                /// Positive: green. Negative: green.
                Green = 4,
                /// Positive: green. Negative: red.
                RedGreen = 6,
                /// PiYG palette.
                PinkWhiteGreen = 5,
            }
            /// How the original image is displayed in the visualization.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum OverlayType {
                /// Default value. This is the same as NONE.
                Unspecified = 0,
                /// No overlay.
                None = 1,
                /// The attributions are shown on top of the original image.
                Original = 2,
                /// The attributions are shown on top of grayscaled version of the
                /// original image.
                Grayscale = 3,
                /// The attributions are used as a mask to reveal predictive parts of
                /// the image and hide the un-predictive parts.
                MaskBlack = 4,
            }
        }
        /// Defines how the feature is encoded to [encoded_tensor][]. Defaults to
        /// IDENTITY.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Encoding {
            /// Default value. This is the same as IDENTITY.
            Unspecified = 0,
            /// The tensor represents one feature.
            Identity = 1,
            /// The tensor represents a bag of features where each index maps to
            /// a feature. [InputMetadata.index_feature_mapping][google.cloud.aiplatform.v1beta1.ExplanationMetadata.InputMetadata.index_feature_mapping] must be provided for
            /// this encoding. For example:
            /// ```
            /// input = [27, 6.0, 150]
            /// index_feature_mapping = ["age", "height", "weight"]
            /// ```
            BagOfFeatures = 2,
            /// The tensor represents a bag of features where each index maps to a
            /// feature. Zero values in the tensor indicates feature being
            /// non-existent. [InputMetadata.index_feature_mapping][google.cloud.aiplatform.v1beta1.ExplanationMetadata.InputMetadata.index_feature_mapping] must be provided
            /// for this encoding. For example:
            /// ```
            /// input = [2, 0, 5, 0, 1]
            /// index_feature_mapping = ["a", "b", "c", "d", "e"]
            /// ```
            BagOfFeaturesSparse = 3,
            /// The tensor is a list of binaries representing whether a feature exists
            /// or not (1 indicates existence). [InputMetadata.index_feature_mapping][google.cloud.aiplatform.v1beta1.ExplanationMetadata.InputMetadata.index_feature_mapping]
            /// must be provided for this encoding. For example:
            /// ```
            /// input = [1, 0, 1, 0, 1]
            /// index_feature_mapping = ["a", "b", "c", "d", "e"]
            /// ```
            Indicator = 4,
            /// The tensor is encoded into a 1-dimensional array represented by an
            /// encoded tensor. [InputMetadata.encoded_tensor_name][google.cloud.aiplatform.v1beta1.ExplanationMetadata.InputMetadata.encoded_tensor_name] must be provided
            /// for this encoding. For example:
            /// ```
            /// input = ["This", "is", "a", "test", "."]
            /// encoded = [0.1, 0.2, 0.3, 0.4, 0.5]
            /// ```
            CombinedEmbedding = 5,
            /// Select this encoding when the input tensor is encoded into a
            /// 2-dimensional array represented by an encoded tensor.
            /// [InputMetadata.encoded_tensor_name][google.cloud.aiplatform.v1beta1.ExplanationMetadata.InputMetadata.encoded_tensor_name] must be provided for this
            /// encoding. The first dimension of the encoded tensor's shape is the same
            /// as the input tensor's shape. For example:
            /// ```
            /// input = ["This", "is", "a", "test", "."]
            /// encoded = [[0.1, 0.2, 0.3, 0.4, 0.5],
            ///            [0.2, 0.1, 0.4, 0.3, 0.5],
            ///            [0.5, 0.1, 0.3, 0.5, 0.4],
            ///            [0.5, 0.3, 0.1, 0.2, 0.4],
            ///            [0.4, 0.3, 0.2, 0.5, 0.1]]
            /// ```
            ConcatEmbedding = 6,
        }
    }
    /// Metadata of the prediction output to be explained.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputMetadata {
        /// Name of the output tensor. Required and is only applicable to AI
        /// Platform provided images for Tensorflow.
        #[prost(string, tag = "3")]
        pub output_tensor_name: ::prost::alloc::string::String,
        /// Defines how to map [Attribution.output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index] to
        /// [Attribution.output_display_name][google.cloud.aiplatform.v1beta1.Attribution.output_display_name].
        ///
        /// If neither of the fields are specified,
        /// [Attribution.output_display_name][google.cloud.aiplatform.v1beta1.Attribution.output_display_name] will not be populated.
        #[prost(oneof = "output_metadata::DisplayNameMapping", tags = "1, 2")]
        pub display_name_mapping: ::core::option::Option<output_metadata::DisplayNameMapping>,
    }
    /// Nested message and enum types in `OutputMetadata`.
    pub mod output_metadata {
        /// Defines how to map [Attribution.output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index] to
        /// [Attribution.output_display_name][google.cloud.aiplatform.v1beta1.Attribution.output_display_name].
        ///
        /// If neither of the fields are specified,
        /// [Attribution.output_display_name][google.cloud.aiplatform.v1beta1.Attribution.output_display_name] will not be populated.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DisplayNameMapping {
            /// Static mapping between the index and display name.
            ///
            /// Use this if the outputs are a deterministic n-dimensional array, e.g. a
            /// list of scores of all the classes in a pre-defined order for a
            /// multi-classification Model. It's not feasible if the outputs are
            /// non-deterministic, e.g. the Model produces top-k classes or sort the
            /// outputs by their values.
            ///
            /// The shape of the value must be an n-dimensional array of strings. The
            /// number of dimensions must match that of the outputs to be explained.
            /// The [Attribution.output_display_name][google.cloud.aiplatform.v1beta1.Attribution.output_display_name] is populated by locating in the
            /// mapping with [Attribution.output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index].
            #[prost(message, tag = "1")]
            IndexDisplayNameMapping(::prost_types::Value),
            /// Specify a field name in the prediction to look for the display name.
            ///
            /// Use this if the prediction contains the display names for the outputs.
            ///
            /// The display names in the prediction must have the same shape of the
            /// outputs, so that it can be located by [Attribution.output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index] for
            /// a specific output.
            #[prost(string, tag = "2")]
            DisplayNameMappingKey(::prost::alloc::string::String),
        }
    }
}
/// The Google Cloud Storage location for the input content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. Google Cloud Storage URI(-s) to the input file(s). May contain
    /// wildcards. For more information on wildcards, see
    /// https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames.
    #[prost(string, repeated, tag = "1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The Google Cloud Storage location where the output is to be written to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required. Google Cloud Storage URI to output directory. If the uri doesn't end with
    /// '/', a '/' will be automatically appended. The directory is created if it
    /// doesn't exist.
    #[prost(string, tag = "1")]
    pub output_uri_prefix: ::prost::alloc::string::String,
}
/// The BigQuery location for the input content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQuerySource {
    /// Required. BigQuery URI to a table, up to 2000 characters long.
    /// Accepted forms:
    ///
    /// *  BigQuery path. For example: `bq://projectId.bqDatasetId.bqTableId`.
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
}
/// The BigQuery location for the output content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDestination {
    /// Required. BigQuery URI to a project or table, up to 2000 characters long.
    ///
    /// When only the project is specified, the Dataset and Table is created.
    /// When the full table reference is specified, the Dataset must exist and
    /// table must not exist.
    ///
    /// Accepted forms:
    ///
    /// *  BigQuery path. For example:
    /// `bq://projectId` or `bq://projectId.bqDatasetId.bqTableId`.
    #[prost(string, tag = "1")]
    pub output_uri: ::prost::alloc::string::String,
}
/// The Container Registry location for the container image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerRegistryDestination {
    /// Required. Container Registry URI of a container image.
    /// Only Google Container Registry and Artifact Registry are supported now.
    /// Accepted forms:
    ///
    /// *  Google Container Registry path. For example:
    ///    `gcr.io/projectId/imageName:tag`.
    ///
    /// *  Artifact Registry path. For example:
    ///    `us-central1-docker.pkg.dev/projectId/repoName/imageName:tag`.
    ///
    /// If a tag is not specified, "latest" will be used as the default tag.
    #[prost(string, tag = "1")]
    pub output_uri: ::prost::alloc::string::String,
}
/// Explanation of a prediction (provided in [PredictResponse.predictions][google.cloud.aiplatform.v1beta1.PredictResponse.predictions])
/// produced by the Model on a given [instance][google.cloud.aiplatform.v1beta1.ExplainRequest.instances].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Explanation {
    /// Output only. Feature attributions grouped by predicted outputs.
    ///
    /// For Models that predict only one output, such as regression Models that
    /// predict only one score, there is only one attibution that explains the
    /// predicted output. For Models that predict multiple outputs, such as
    /// multiclass Models that predict multiple classes, each element explains one
    /// specific item. [Attribution.output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index] can be used to identify which
    /// output this attribution is explaining.
    ///
    /// If users set [ExplanationParameters.top_k][google.cloud.aiplatform.v1beta1.ExplanationParameters.top_k], the attributions are sorted
    /// by [instance_output_value][Attributions.instance_output_value] in
    /// descending order. If [ExplanationParameters.output_indices][google.cloud.aiplatform.v1beta1.ExplanationParameters.output_indices] is specified,
    /// the attributions are stored by [Attribution.output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index] in the same
    /// order as they appear in the output_indices.
    #[prost(message, repeated, tag = "1")]
    pub attributions: ::prost::alloc::vec::Vec<Attribution>,
}
/// Aggregated explanation metrics for a Model over a set of instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelExplanation {
    /// Output only. Aggregated attributions explaining the Model's prediction outputs over the
    /// set of instances. The attributions are grouped by outputs.
    ///
    /// For Models that predict only one output, such as regression Models that
    /// predict only one score, there is only one attibution that explains the
    /// predicted output. For Models that predict multiple outputs, such as
    /// multiclass Models that predict multiple classes, each element explains one
    /// specific item. [Attribution.output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index] can be used to identify which
    /// output this attribution is explaining.
    ///
    /// The [baselineOutputValue][google.cloud.aiplatform.v1beta1.Attribution.baseline_output_value],
    /// [instanceOutputValue][google.cloud.aiplatform.v1beta1.Attribution.instance_output_value] and
    /// [featureAttributions][google.cloud.aiplatform.v1beta1.Attribution.feature_attributions] fields are
    /// averaged over the test data.
    ///
    /// NOTE: Currently AutoML tabular classification Models produce only one
    /// attribution, which averages attributions over all the classes it predicts.
    /// [Attribution.approximation_error][google.cloud.aiplatform.v1beta1.Attribution.approximation_error] is not populated.
    #[prost(message, repeated, tag = "1")]
    pub mean_attributions: ::prost::alloc::vec::Vec<Attribution>,
}
/// Attribution that explains a particular prediction output.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribution {
    /// Output only. Model predicted output if the input instance is constructed from the
    /// baselines of all the features defined in [ExplanationMetadata.inputs][google.cloud.aiplatform.v1beta1.ExplanationMetadata.inputs].
    /// The field name of the output is determined by the key in
    /// [ExplanationMetadata.outputs][google.cloud.aiplatform.v1beta1.ExplanationMetadata.outputs].
    ///
    /// If the Model's predicted output has multiple dimensions (rank > 1), this is
    /// the value in the output located by [output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index].
    ///
    /// If there are multiple baselines, their output values are averaged.
    #[prost(double, tag = "1")]
    pub baseline_output_value: f64,
    /// Output only. Model predicted output on the corresponding [explanation
    /// instance][ExplainRequest.instances]. The field name of the output is
    /// determined by the key in [ExplanationMetadata.outputs][google.cloud.aiplatform.v1beta1.ExplanationMetadata.outputs].
    ///
    /// If the Model predicted output has multiple dimensions, this is the value in
    /// the output located by [output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index].
    #[prost(double, tag = "2")]
    pub instance_output_value: f64,
    /// Output only. Attributions of each explained feature. Features are extracted from
    /// the [prediction instances][google.cloud.aiplatform.v1beta1.ExplainRequest.instances] according to
    /// [explanation metadata for inputs][google.cloud.aiplatform.v1beta1.ExplanationMetadata.inputs].
    ///
    /// The value is a struct, whose keys are the name of the feature. The values
    /// are how much the feature in the [instance][google.cloud.aiplatform.v1beta1.ExplainRequest.instances]
    /// contributed to the predicted result.
    ///
    /// The format of the value is determined by the feature's input format:
    ///
    ///   * If the feature is a scalar value, the attribution value is a
    ///     [floating number][google.protobuf.Value.number_value].
    ///
    ///   * If the feature is an array of scalar values, the attribution value is
    ///     an [array][google.protobuf.Value.list_value].
    ///
    ///   * If the feature is a struct, the attribution value is a
    ///     [struct][google.protobuf.Value.struct_value]. The keys in the
    ///     attribution value struct are the same as the keys in the feature
    ///     struct. The formats of the values in the attribution struct are
    ///     determined by the formats of the values in the feature struct.
    ///
    /// The [ExplanationMetadata.feature_attributions_schema_uri][google.cloud.aiplatform.v1beta1.ExplanationMetadata.feature_attributions_schema_uri] field,
    /// pointed to by the [ExplanationSpec][google.cloud.aiplatform.v1beta1.ExplanationSpec] field of the
    /// [Endpoint.deployed_models][google.cloud.aiplatform.v1beta1.Endpoint.deployed_models] object, points to the schema file that
    /// describes the features and their attribution values (if it is populated).
    #[prost(message, optional, tag = "3")]
    pub feature_attributions: ::core::option::Option<::prost_types::Value>,
    /// Output only. The index that locates the explained prediction output.
    ///
    /// If the prediction output is a scalar value, output_index is not populated.
    /// If the prediction output has multiple dimensions, the length of the
    /// output_index list is the same as the number of dimensions of the output.
    /// The i-th element in output_index is the element index of the i-th dimension
    /// of the output vector. Indices start from 0.
    #[prost(int32, repeated, packed = "false", tag = "4")]
    pub output_index: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The display name of the output identified by [output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index]. For example,
    /// the predicted class name by a multi-classification Model.
    ///
    /// This field is only populated iff the Model predicts display names as a
    /// separate field along with the explained output. The predicted display name
    /// must has the same shape of the explained output, and can be located using
    /// output_index.
    #[prost(string, tag = "5")]
    pub output_display_name: ::prost::alloc::string::String,
    /// Output only. Error of [feature_attributions][google.cloud.aiplatform.v1beta1.Attribution.feature_attributions] caused by approximation used in the
    /// explanation method. Lower value means more precise attributions.
    ///
    /// * For Sampled Shapley
    /// [attribution][google.cloud.aiplatform.v1beta1.ExplanationParameters.sampled_shapley_attribution],
    /// increasing [path_count][google.cloud.aiplatform.v1beta1.SampledShapleyAttribution.path_count] might reduce
    /// the error.
    /// * For Integrated Gradients
    /// [attribution][google.cloud.aiplatform.v1beta1.ExplanationParameters.integrated_gradients_attribution],
    /// increasing [step_count][google.cloud.aiplatform.v1beta1.IntegratedGradientsAttribution.step_count] might
    /// reduce the error.
    /// * For [XRAI attribution][google.cloud.aiplatform.v1beta1.ExplanationParameters.xrai_attribution],
    /// increasing
    /// [step_count][google.cloud.aiplatform.v1beta1.XraiAttribution.step_count] might reduce the error.
    ///
    /// See [this introduction](/ai-platform-unified/docs/explainable-ai/overview)
    /// for more information.
    #[prost(double, tag = "6")]
    pub approximation_error: f64,
    /// Output only. Name of the explain output. Specified as the key in
    /// [ExplanationMetadata.outputs][google.cloud.aiplatform.v1beta1.ExplanationMetadata.outputs].
    #[prost(string, tag = "7")]
    pub output_name: ::prost::alloc::string::String,
}
/// Specification of Model explanation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplanationSpec {
    /// Required. Parameters that configure explaining of the Model's predictions.
    #[prost(message, optional, tag = "1")]
    pub parameters: ::core::option::Option<ExplanationParameters>,
    /// Required. Metadata describing the Model's input and output for explanation.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<ExplanationMetadata>,
}
/// Parameters to configure explaining for Model's predictions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplanationParameters {
    /// If populated, returns attributions for top K indices of outputs
    /// (defaults to 1). Only applies to Models that predicts more than one outputs
    /// (e,g, multi-class Models). When set to -1, returns explanations for all
    /// outputs.
    #[prost(int32, tag = "4")]
    pub top_k: i32,
    /// If populated, only returns attributions that have
    /// [output_index][google.cloud.aiplatform.v1beta1.Attribution.output_index] contained in output_indices. It
    /// must be an ndarray of integers, with the same shape of the output it's
    /// explaining.
    ///
    /// If not populated, returns attributions for [top_k][google.cloud.aiplatform.v1beta1.ExplanationParameters.top_k] indices of outputs.
    /// If neither top_k nor output_indeices is populated, returns the argmax
    /// index of the outputs.
    ///
    /// Only applicable to Models that predict multiple outputs (e,g, multi-class
    /// Models that predict multiple classes).
    #[prost(message, optional, tag = "5")]
    pub output_indices: ::core::option::Option<::prost_types::ListValue>,
    #[prost(oneof = "explanation_parameters::Method", tags = "1, 2, 3")]
    pub method: ::core::option::Option<explanation_parameters::Method>,
}
/// Nested message and enum types in `ExplanationParameters`.
pub mod explanation_parameters {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        /// An attribution method that approximates Shapley values for features that
        /// contribute to the label being predicted. A sampling strategy is used to
        /// approximate the value rather than considering all subsets of features.
        /// Refer to this paper for model details: https://arxiv.org/abs/1306.4265.
        #[prost(message, tag = "1")]
        SampledShapleyAttribution(super::SampledShapleyAttribution),
        /// An attribution method that computes Aumann-Shapley values taking
        /// advantage of the model's fully differentiable structure. Refer to this
        /// paper for more details: https://arxiv.org/abs/1703.01365
        #[prost(message, tag = "2")]
        IntegratedGradientsAttribution(super::IntegratedGradientsAttribution),
        /// An attribution method that redistributes Integrated Gradients
        /// attribution to segmented regions, taking advantage of the model's fully
        /// differentiable structure. Refer to this paper for
        /// more details: https://arxiv.org/abs/1906.02825
        ///
        /// XRAI currently performs better on natural images, like a picture of a
        /// house or an animal. If the images are taken in artificial environments,
        /// like a lab or manufacturing line, or from diagnostic equipment, like
        /// x-rays or quality-control cameras, use Integrated Gradients instead.
        #[prost(message, tag = "3")]
        XraiAttribution(super::XraiAttribution),
    }
}
/// An attribution method that approximates Shapley values for features that
/// contribute to the label being predicted. A sampling strategy is used to
/// approximate the value rather than considering all subsets of features.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampledShapleyAttribution {
    /// Required. The number of feature permutations to consider when approximating the
    /// Shapley values.
    ///
    /// Valid range of its value is [1, 50], inclusively.
    #[prost(int32, tag = "1")]
    pub path_count: i32,
}
/// An attribution method that computes the Aumann-Shapley value taking advantage
/// of the model's fully differentiable structure. Refer to this paper for
/// more details: https://arxiv.org/abs/1703.01365
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegratedGradientsAttribution {
    /// Required. The number of steps for approximating the path integral.
    /// A good value to start is 50 and gradually increase until the
    /// sum to diff property is within the desired error range.
    ///
    /// Valid range of its value is [1, 100], inclusively.
    #[prost(int32, tag = "1")]
    pub step_count: i32,
    /// Config for SmoothGrad approximation of gradients.
    ///
    /// When enabled, the gradients are approximated by averaging the gradients
    /// from noisy samples in the vicinity of the inputs. Adding
    /// noise can help improve the computed gradients. Refer to this paper for more
    /// details: https://arxiv.org/pdf/1706.03825.pdf
    #[prost(message, optional, tag = "2")]
    pub smooth_grad_config: ::core::option::Option<SmoothGradConfig>,
}
/// An explanation method that redistributes Integrated Gradients
/// attributions to segmented regions, taking advantage of the model's fully
/// differentiable structure. Refer to this paper for more details:
/// https://arxiv.org/abs/1906.02825
///
/// Supported only by image Models.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XraiAttribution {
    /// Required. The number of steps for approximating the path integral.
    /// A good value to start is 50 and gradually increase until the
    /// sum to diff property is met within the desired error range.
    ///
    /// Valid range of its value is [1, 100], inclusively.
    #[prost(int32, tag = "1")]
    pub step_count: i32,
    /// Config for SmoothGrad approximation of gradients.
    ///
    /// When enabled, the gradients are approximated by averaging the gradients
    /// from noisy samples in the vicinity of the inputs. Adding
    /// noise can help improve the computed gradients. Refer to this paper for more
    /// details: https://arxiv.org/pdf/1706.03825.pdf
    #[prost(message, optional, tag = "2")]
    pub smooth_grad_config: ::core::option::Option<SmoothGradConfig>,
}
/// Config for SmoothGrad approximation of gradients.
///
/// When enabled, the gradients are approximated by averaging the gradients from
/// noisy samples in the vicinity of the inputs. Adding noise can help improve
/// the computed gradients. Refer to this paper for more details:
/// https://arxiv.org/pdf/1706.03825.pdf
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmoothGradConfig {
    /// The number of gradient samples to use for
    /// approximation. The higher this number, the more accurate the gradient
    /// is, but the runtime complexity increases by this factor as well.
    /// Valid range of its value is [1, 50]. Defaults to 3.
    #[prost(int32, tag = "3")]
    pub noisy_sample_count: i32,
    /// Represents the standard deviation of the gaussian kernel
    /// that will be used to add noise to the interpolated inputs
    /// prior to computing gradients.
    #[prost(oneof = "smooth_grad_config::GradientNoiseSigma", tags = "1, 2")]
    pub gradient_noise_sigma: ::core::option::Option<smooth_grad_config::GradientNoiseSigma>,
}
/// Nested message and enum types in `SmoothGradConfig`.
pub mod smooth_grad_config {
    /// Represents the standard deviation of the gaussian kernel
    /// that will be used to add noise to the interpolated inputs
    /// prior to computing gradients.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GradientNoiseSigma {
        /// This is a single float value and will be used to add noise to all the
        /// features. Use this field when all features are normalized to have the
        /// same distribution: scale to range [0, 1], [-1, 1] or z-scoring, where
        /// features are normalized to have 0-mean and 1-variance. For more details
        /// about normalization:
        /// https://tinyurl.com/dgc-normalization.
        ///
        /// For best results the recommended value is about 10% - 20% of the standard
        /// deviation of the input feature. Refer to section 3.2 of the SmoothGrad
        /// paper: https://arxiv.org/pdf/1706.03825.pdf. Defaults to 0.1.
        ///
        /// If the distribution is different per feature, set
        /// [feature_noise_sigma][google.cloud.aiplatform.v1beta1.SmoothGradConfig.feature_noise_sigma] instead
        /// for each feature.
        #[prost(float, tag = "1")]
        NoiseSigma(f32),
        /// This is similar to [noise_sigma][google.cloud.aiplatform.v1beta1.SmoothGradConfig.noise_sigma], but
        /// provides additional flexibility. A separate noise sigma can be provided
        /// for each feature, which is useful if their distributions are different.
        /// No noise is added to features that are not set. If this field is unset,
        /// [noise_sigma][google.cloud.aiplatform.v1beta1.SmoothGradConfig.noise_sigma] will be used for all
        /// features.
        #[prost(message, tag = "2")]
        FeatureNoiseSigma(super::FeatureNoiseSigma),
    }
}
/// Noise sigma by features. Noise sigma represents the standard deviation of the
/// gaussian kernel that will be used to add noise to interpolated inputs prior
/// to computing gradients.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureNoiseSigma {
    /// Noise sigma per feature. No noise is added to features that are not set.
    #[prost(message, repeated, tag = "1")]
    pub noise_sigma: ::prost::alloc::vec::Vec<feature_noise_sigma::NoiseSigmaForFeature>,
}
/// Nested message and enum types in `FeatureNoiseSigma`.
pub mod feature_noise_sigma {
    /// Noise sigma for a single feature.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoiseSigmaForFeature {
        /// The name of the input feature for which noise sigma is provided. The
        /// features are defined in
        /// [explanation metadata inputs][google.cloud.aiplatform.v1beta1.ExplanationMetadata.inputs].
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// This represents the standard deviation of the Gaussian kernel that will
        /// be used to add noise to the feature prior to computing gradients. Similar
        /// to [noise_sigma][google.cloud.aiplatform.v1beta1.SmoothGradConfig.noise_sigma] but represents the
        /// noise added to the current feature. Defaults to 0.1.
        #[prost(float, tag = "2")]
        pub sigma: f32,
    }
}
/// The [ExplanationSpec][google.cloud.aiplatform.v1beta1.ExplanationSpec] entries that can be overridden at [online
/// explanation][PredictionService.Explain][google.cloud.aiplatform.v1beta1.PredictionService.Explain] time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplanationSpecOverride {
    /// The parameters to be overridden. Note that the
    /// [method][google.cloud.aiplatform.v1beta1.ExplanationParameters.method] cannot be changed. If not specified,
    /// no parameter is overridden.
    #[prost(message, optional, tag = "1")]
    pub parameters: ::core::option::Option<ExplanationParameters>,
    /// The metadata to be overridden. If not specified, no metadata is overridden.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<ExplanationMetadataOverride>,
}
/// The [ExplanationMetadata][google.cloud.aiplatform.v1beta1.ExplanationMetadata] entries that can be overridden at
/// [online explanation][google.cloud.aiplatform.v1beta1.PredictionService.Explain] time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplanationMetadataOverride {
    /// Required. Overrides the [input metadata][google.cloud.aiplatform.v1beta1.ExplanationMetadata.inputs] of the features.
    /// The key is the name of the feature to be overridden. The keys specified
    /// here must exist in the input metadata to be overridden. If a feature is
    /// not specified here, the corresponding feature's input metadata is not
    /// overridden.
    #[prost(map = "string, message", tag = "1")]
    pub inputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        explanation_metadata_override::InputMetadataOverride,
    >,
}
/// Nested message and enum types in `ExplanationMetadataOverride`.
pub mod explanation_metadata_override {
    /// The [input metadata][google.cloud.aiplatform.v1beta1.ExplanationMetadata.InputMetadata] entries to be
    /// overridden.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputMetadataOverride {
        /// Baseline inputs for this feature.
        ///
        /// This overrides the `input_baseline` field of the
        /// [ExplanationMetadata.InputMetadata][google.cloud.aiplatform.v1beta1.ExplanationMetadata.InputMetadata]
        /// object of the corresponding feature's input metadata. If it's not
        /// specified, the original baselines are not overridden.
        #[prost(message, repeated, tag = "1")]
        pub input_baselines: ::prost::alloc::vec::Vec<::prost_types::Value>,
    }
}
/// Describes the state of a job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobState {
    /// The job state is unspecified.
    Unspecified = 0,
    /// The job has been just created or resumed and processing has not yet begun.
    Queued = 1,
    /// The service is preparing to run the job.
    Pending = 2,
    /// The job is in progress.
    Running = 3,
    /// The job completed successfully.
    Succeeded = 4,
    /// The job failed.
    Failed = 5,
    /// The job is being cancelled. From this state the job may only go to
    /// either `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`.
    Cancelling = 6,
    /// The job has been cancelled.
    Cancelled = 7,
    /// The job has been stopped, and can be resumed.
    Paused = 8,
    /// The job has expired.
    Expired = 9,
}
/// Specification of a single machine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MachineSpec {
    /// Immutable. The type of the machine. For the machine types supported for prediction,
    /// see https://tinyurl.com/aip-docs/predictions/machine-types.
    /// For machine types supported for creating a custom training job, see
    /// https://tinyurl.com/aip-docs/training/configure-compute.
    ///
    /// For [DeployedModel][google.cloud.aiplatform.v1beta1.DeployedModel] this field is optional, and the default
    /// value is `n1-standard-2`. For [BatchPredictionJob][google.cloud.aiplatform.v1beta1.BatchPredictionJob] or as part of
    /// [WorkerPoolSpec][google.cloud.aiplatform.v1beta1.WorkerPoolSpec] this field is required.
    #[prost(string, tag = "1")]
    pub machine_type: ::prost::alloc::string::String,
    /// Immutable. The type of accelerator(s) that may be attached to the machine as per
    /// [accelerator_count][google.cloud.aiplatform.v1beta1.MachineSpec.accelerator_count].
    #[prost(enumeration = "AcceleratorType", tag = "2")]
    pub accelerator_type: i32,
    /// The number of accelerators to attach to the machine.
    #[prost(int32, tag = "3")]
    pub accelerator_count: i32,
}
/// A description of resources that are dedicated to a DeployedModel, and
/// that need a higher degree of manual configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DedicatedResources {
    /// Required. Immutable. The specification of a single machine used by the prediction.
    #[prost(message, optional, tag = "1")]
    pub machine_spec: ::core::option::Option<MachineSpec>,
    /// Required. Immutable. The minimum number of machine replicas this DeployedModel will be always
    /// deployed on. If traffic against it increases, it may dynamically be
    /// deployed onto more replicas, and as traffic decreases, some of these extra
    /// replicas may be freed.
    /// Note: if [machine_spec.accelerator_count][google.cloud.aiplatform.v1beta1.MachineSpec.accelerator_count] is
    /// above 0, currently the model will be always deployed precisely on
    /// [min_replica_count][google.cloud.aiplatform.v1beta1.DedicatedResources.min_replica_count].
    #[prost(int32, tag = "2")]
    pub min_replica_count: i32,
    /// Immutable. The maximum number of replicas this DeployedModel may be deployed on when
    /// the traffic against it increases. If the requested value is too large,
    /// the deployment will error, but if deployment succeeds then the ability
    /// to scale the model to that many replicas is guaranteed (barring service
    /// outages). If traffic against the DeployedModel increases beyond what its
    /// replicas at maximum may handle, a portion of the traffic will be dropped.
    /// If this value is not provided, will use [min_replica_count][google.cloud.aiplatform.v1beta1.DedicatedResources.min_replica_count] as the
    /// default value.
    #[prost(int32, tag = "3")]
    pub max_replica_count: i32,
    /// Immutable. The metric specifications that overrides a resource
    /// utilization metric (CPU utilization, accelerator's duty cycle, and so on)
    /// target value (default to 60 if not set). At most one entry is allowed per
    /// metric.
    ///
    /// If [machine_spec.accelerator_count][google.cloud.aiplatform.v1beta1.MachineSpec.accelerator_count] is
    /// above 0, the autoscaling will be based on both CPU utilization and
    /// accelerator's duty cycle metrics and scale up when either metrics exceeds
    /// its target value while scale down if both metrics are under their target
    /// value. The default target value is 60 for both metrics.
    ///
    /// If [machine_spec.accelerator_count][google.cloud.aiplatform.v1beta1.MachineSpec.accelerator_count] is
    /// 0, the autoscaling will be based on CPU utilization metric only with
    /// default target value 60 if not explicitly set.
    ///
    /// For example, in the case of Online Prediction, if you want to override
    /// target CPU utilization to 80, you should set
    /// [autoscaling_metric_specs.metric_name][google.cloud.aiplatform.v1beta1.AutoscalingMetricSpec.metric_name]
    /// to `aiplatform.googleapis.com/prediction/online/cpu/utilization` and
    /// [autoscaling_metric_specs.target][google.cloud.aiplatform.v1beta1.AutoscalingMetricSpec.target] to `80`.
    #[prost(message, repeated, tag = "4")]
    pub autoscaling_metric_specs: ::prost::alloc::vec::Vec<AutoscalingMetricSpec>,
}
/// A description of resources that to large degree are decided by AI Platform,
/// and require only a modest additional configuration.
/// Each Model supporting these resources documents its specific guidelines.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutomaticResources {
    /// Immutable. The minimum number of replicas this DeployedModel will be always deployed
    /// on. If traffic against it increases, it may dynamically be deployed onto
    /// more replicas up to [max_replica_count][google.cloud.aiplatform.v1beta1.AutomaticResources.max_replica_count], and as traffic decreases, some
    /// of these extra replicas may be freed.
    /// If the requested value is too large, the deployment will error.
    #[prost(int32, tag = "1")]
    pub min_replica_count: i32,
    /// Immutable. The maximum number of replicas this DeployedModel may be deployed on when
    /// the traffic against it increases. If the requested value is too large,
    /// the deployment will error, but if deployment succeeds then the ability
    /// to scale the model to that many replicas is guaranteed (barring service
    /// outages). If traffic against the DeployedModel increases beyond what its
    /// replicas at maximum may handle, a portion of the traffic will be dropped.
    /// If this value is not provided, a no upper bound for scaling under heavy
    /// traffic will be assume, though AI Platform may be unable to scale beyond
    /// certain replica number.
    #[prost(int32, tag = "2")]
    pub max_replica_count: i32,
}
/// A description of resources that are used for performing batch operations, are
/// dedicated to a Model, and need manual configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDedicatedResources {
    /// Required. Immutable. The specification of a single machine.
    #[prost(message, optional, tag = "1")]
    pub machine_spec: ::core::option::Option<MachineSpec>,
    /// Immutable. The number of machine replicas used at the start of the batch operation.
    /// If not set, AI Platform decides starting number, not greater than
    /// [max_replica_count][google.cloud.aiplatform.v1beta1.BatchDedicatedResources.max_replica_count]
    #[prost(int32, tag = "2")]
    pub starting_replica_count: i32,
    /// Immutable. The maximum number of machine replicas the batch operation may be scaled
    /// to. The default value is 10.
    #[prost(int32, tag = "3")]
    pub max_replica_count: i32,
}
/// Statistics information about resource consumption.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourcesConsumed {
    /// Output only. The number of replica hours used. Note that many replicas may run in
    /// parallel, and additionally any given work may be queued for some time.
    /// Therefore this value is not strictly related to wall time.
    #[prost(double, tag = "1")]
    pub replica_hours: f64,
}
/// Represents the spec of disk options.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskSpec {
    /// Type of the boot disk (default is "pd-ssd").
    /// Valid values: "pd-ssd" (Persistent Disk Solid State Drive) or
    /// "pd-standard" (Persistent Disk Hard Disk Drive).
    #[prost(string, tag = "1")]
    pub boot_disk_type: ::prost::alloc::string::String,
    /// Size in GB of the boot disk (default is 100GB).
    #[prost(int32, tag = "2")]
    pub boot_disk_size_gb: i32,
}
/// The metric specification that defines the target resource utilization
/// (CPU utilization, accelerator's duty cycle, and so on) for calculating the
/// desired replica count.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalingMetricSpec {
    /// Required. The resource metric name.
    /// Supported metrics:
    ///
    /// * For Online Prediction:
    /// * `aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle`
    /// * `aiplatform.googleapis.com/prediction/online/cpu/utilization`
    #[prost(string, tag = "1")]
    pub metric_name: ::prost::alloc::string::String,
    /// The target resource utilization in percentage (1% - 100%) for the given
    /// metric; once the real usage deviates from the target by a certain
    /// percentage, the machine replicas change. The default value is 60
    /// (representing 60%) if not provided.
    #[prost(int32, tag = "2")]
    pub target: i32,
}
/// Manual batch tuning parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualBatchTuningParameters {
    /// Immutable. The number of the records (e.g. instances) of the operation given in
    /// each batch to a machine replica. Machine type, and size of a single
    /// record should be considered when setting this parameter, higher value
    /// speeds up the batch operation's execution, but too high value will result
    /// in a whole batch not fitting in a machine's memory, and the whole
    /// operation will fail.
    /// The default value is 4.
    #[prost(int32, tag = "1")]
    pub batch_size: i32,
}
/// A job that uses a [Model][google.cloud.aiplatform.v1beta1.BatchPredictionJob.model] to produce predictions
/// on multiple [input instances][google.cloud.aiplatform.v1beta1.BatchPredictionJob.input_config]. If
/// predictions for significant portion of the instances fail, the job may finish
/// without attempting predictions for all remaining instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchPredictionJob {
    /// Output only. Resource name of the BatchPredictionJob.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The user-defined name of this BatchPredictionJob.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The name of the Model that produces the predictions via this job,
    /// must share the same ancestor Location.
    /// Starting this job has no impact on any existing deployments of the Model
    /// and their resources.
    #[prost(string, tag = "3")]
    pub model: ::prost::alloc::string::String,
    /// Required. Input configuration of the instances on which predictions are performed.
    /// The schema of any single instance may be specified via
    /// the [Model's][google.cloud.aiplatform.v1beta1.BatchPredictionJob.model]
    /// [PredictSchemata's][google.cloud.aiplatform.v1beta1.Model.predict_schemata]
    /// [instance_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.instance_schema_uri].
    #[prost(message, optional, tag = "4")]
    pub input_config: ::core::option::Option<batch_prediction_job::InputConfig>,
    /// The parameters that govern the predictions. The schema of the parameters
    /// may be specified via the [Model's][google.cloud.aiplatform.v1beta1.BatchPredictionJob.model]
    /// [PredictSchemata's][google.cloud.aiplatform.v1beta1.Model.predict_schemata]
    /// [parameters_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.parameters_schema_uri].
    #[prost(message, optional, tag = "5")]
    pub model_parameters: ::core::option::Option<::prost_types::Value>,
    /// Required. The Configuration specifying where output predictions should
    /// be written.
    /// The schema of any single prediction may be specified as a concatenation
    /// of [Model's][google.cloud.aiplatform.v1beta1.BatchPredictionJob.model]
    /// [PredictSchemata's][google.cloud.aiplatform.v1beta1.Model.predict_schemata]
    /// [instance_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.instance_schema_uri]
    /// and
    /// [prediction_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.prediction_schema_uri].
    #[prost(message, optional, tag = "6")]
    pub output_config: ::core::option::Option<batch_prediction_job::OutputConfig>,
    /// The config of resources used by the Model during the batch prediction. If
    /// the Model [supports][google.cloud.aiplatform.v1beta1.Model.supported_deployment_resources_types]
    /// DEDICATED_RESOURCES this config may be provided (and the job will use these
    /// resources), if the Model doesn't support AUTOMATIC_RESOURCES, this config
    /// must be provided.
    #[prost(message, optional, tag = "7")]
    pub dedicated_resources: ::core::option::Option<BatchDedicatedResources>,
    /// Immutable. Parameters configuring the batch behavior. Currently only applicable when
    /// [dedicated_resources][google.cloud.aiplatform.v1beta1.BatchPredictionJob.dedicated_resources] are used (in other cases AI Platform does
    /// the tuning itself).
    #[prost(message, optional, tag = "8")]
    pub manual_batch_tuning_parameters: ::core::option::Option<ManualBatchTuningParameters>,
    /// Generate explanation with the batch prediction results.
    ///
    /// When set to `true`, the batch prediction output changes based on the
    /// `predictions_format` field of the
    /// [BatchPredictionJob.output_config][google.cloud.aiplatform.v1beta1.BatchPredictionJob.output_config] object:
    ///
    ///  * `bigquery`: output includes a column named `explanation`. The value
    ///    is a struct that conforms to the [Explanation][google.cloud.aiplatform.v1beta1.Explanation] object.
    ///  * `jsonl`: The JSON objects on each line include an additional entry
    ///    keyed `explanation`. The value of the entry is a JSON object that
    ///    conforms to the [Explanation][google.cloud.aiplatform.v1beta1.Explanation] object.
    ///  * `csv`: Generating explanations for CSV format is not supported.
    ///
    /// If this field is set to true, either the [Model.explanation_spec][google.cloud.aiplatform.v1beta1.Model.explanation_spec] or
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.BatchPredictionJob.explanation_spec] must be populated.
    #[prost(bool, tag = "23")]
    pub generate_explanation: bool,
    /// Explanation configuration for this BatchPredictionJob. Can be
    /// specified only if [generate_explanation][google.cloud.aiplatform.v1beta1.BatchPredictionJob.generate_explanation] is set to `true`.
    ///
    /// This value overrides the value of [Model.explanation_spec][google.cloud.aiplatform.v1beta1.Model.explanation_spec]. All fields of
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.BatchPredictionJob.explanation_spec] are optional in the request. If a field of the
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.BatchPredictionJob.explanation_spec] object is not populated, the corresponding field of
    /// the [Model.explanation_spec][google.cloud.aiplatform.v1beta1.Model.explanation_spec] object is inherited.
    #[prost(message, optional, tag = "25")]
    pub explanation_spec: ::core::option::Option<ExplanationSpec>,
    /// Output only. Information further describing the output of this job.
    #[prost(message, optional, tag = "9")]
    pub output_info: ::core::option::Option<batch_prediction_job::OutputInfo>,
    /// Output only. The detailed state of the job.
    #[prost(enumeration = "JobState", tag = "10")]
    pub state: i32,
    /// Output only. Only populated when the job's state is JOB_STATE_FAILED or
    /// JOB_STATE_CANCELLED.
    #[prost(message, optional, tag = "11")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. Partial failures encountered.
    /// For example, single files that can't be read.
    /// This field never exceeds 20 entries.
    /// Status details fields contain standard GCP error details.
    #[prost(message, repeated, tag = "12")]
    pub partial_failures: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Output only. Information about resources that had been consumed by this job.
    /// Provided in real time at best effort basis, as well as a final value
    /// once the job completes.
    ///
    /// Note: This field currently may be not populated for batch predictions that
    /// use AutoML Models.
    #[prost(message, optional, tag = "13")]
    pub resources_consumed: ::core::option::Option<ResourcesConsumed>,
    /// Output only. Statistics on completed and failed prediction instances.
    #[prost(message, optional, tag = "14")]
    pub completion_stats: ::core::option::Option<CompletionStats>,
    /// Output only. Time when the BatchPredictionJob was created.
    #[prost(message, optional, tag = "15")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the BatchPredictionJob for the first time entered the
    /// `JOB_STATE_RUNNING` state.
    #[prost(message, optional, tag = "16")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the BatchPredictionJob entered any of the following states:
    /// `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`.
    #[prost(message, optional, tag = "17")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the BatchPredictionJob was most recently updated.
    #[prost(message, optional, tag = "18")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The labels with user-defined metadata to organize BatchPredictionJobs.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    ///
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    #[prost(map = "string, string", tag = "19")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Customer-managed encryption key options for a BatchPredictionJob. If this
    /// is set, then all resources created by the BatchPredictionJob will be
    /// encrypted with the provided encryption key.
    #[prost(message, optional, tag = "24")]
    pub encryption_spec: ::core::option::Option<EncryptionSpec>,
}
/// Nested message and enum types in `BatchPredictionJob`.
pub mod batch_prediction_job {
    /// Configures the input to [BatchPredictionJob][google.cloud.aiplatform.v1beta1.BatchPredictionJob].
    /// See [Model.supported_input_storage_formats][google.cloud.aiplatform.v1beta1.Model.supported_input_storage_formats] for Model's supported input
    /// formats, and how instances should be expressed via any of them.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InputConfig {
        /// Required. The format in which instances are given, must be one of the
        /// [Model's][google.cloud.aiplatform.v1beta1.BatchPredictionJob.model]
        /// [supported_input_storage_formats][google.cloud.aiplatform.v1beta1.Model.supported_input_storage_formats].
        #[prost(string, tag = "1")]
        pub instances_format: ::prost::alloc::string::String,
        /// Required. The source of the input.
        #[prost(oneof = "input_config::Source", tags = "2, 3")]
        pub source: ::core::option::Option<input_config::Source>,
    }
    /// Nested message and enum types in `InputConfig`.
    pub mod input_config {
        /// Required. The source of the input.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            /// The Cloud Storage location for the input instances.
            #[prost(message, tag = "2")]
            GcsSource(super::super::GcsSource),
            /// The BigQuery location of the input table.
            /// The schema of the table should be in the format described by the given
            /// context OpenAPI Schema, if one is provided. The table may contain
            /// additional columns that are not described by the schema, and they will
            /// be ignored.
            #[prost(message, tag = "3")]
            BigquerySource(super::super::BigQuerySource),
        }
    }
    /// Configures the output of [BatchPredictionJob][google.cloud.aiplatform.v1beta1.BatchPredictionJob].
    /// See [Model.supported_output_storage_formats][google.cloud.aiplatform.v1beta1.Model.supported_output_storage_formats] for supported output
    /// formats, and how predictions are expressed via any of them.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputConfig {
        /// Required. The format in which AI Platform gives the predictions, must be one of the
        /// [Model's][google.cloud.aiplatform.v1beta1.BatchPredictionJob.model]
        /// [supported_output_storage_formats][google.cloud.aiplatform.v1beta1.Model.supported_output_storage_formats].
        #[prost(string, tag = "1")]
        pub predictions_format: ::prost::alloc::string::String,
        /// Required. The destination of the output.
        #[prost(oneof = "output_config::Destination", tags = "2, 3")]
        pub destination: ::core::option::Option<output_config::Destination>,
    }
    /// Nested message and enum types in `OutputConfig`.
    pub mod output_config {
        /// Required. The destination of the output.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Destination {
            /// The Cloud Storage location of the directory where the output is
            /// to be written to. In the given directory a new directory is created.
            /// Its name is `prediction-<model-display-name>-<job-create-time>`,
            /// where timestamp is in YYYY-MM-DDThh:mm:ss.sssZ ISO-8601 format.
            /// Inside of it files `predictions_0001.<extension>`,
            /// `predictions_0002.<extension>`, ..., `predictions_N.<extension>`
            /// are created where `<extension>` depends on chosen
            /// [predictions_format][google.cloud.aiplatform.v1beta1.BatchPredictionJob.OutputConfig.predictions_format], and N may equal 0001 and depends on the total
            /// number of successfully predicted instances.
            /// If the Model has both [instance][google.cloud.aiplatform.v1beta1.PredictSchemata.instance_schema_uri]
            /// and [prediction][google.cloud.aiplatform.v1beta1.PredictSchemata.parameters_schema_uri] schemata
            /// defined then each such file contains predictions as per the
            /// [predictions_format][google.cloud.aiplatform.v1beta1.BatchPredictionJob.OutputConfig.predictions_format].
            /// If prediction for any instance failed (partially or completely), then
            /// an additional `errors_0001.<extension>`, `errors_0002.<extension>`,...,
            /// `errors_N.<extension>` files are created (N depends on total number
            /// of failed predictions). These files contain the failed instances,
            /// as per their schema, followed by an additional `error` field which as
            /// value has
            /// [`google.rpc.Status`](Status)
            /// containing only `code` and `message` fields.
            #[prost(message, tag = "2")]
            GcsDestination(super::super::GcsDestination),
            /// The BigQuery project location where the output is to be written to.
            /// In the given project a new dataset is created with name
            /// `prediction_<model-display-name>_<job-create-time>`
            /// where <model-display-name> is made
            /// BigQuery-dataset-name compatible (for example, most special characters
            /// become underscores), and timestamp is in
            /// YYYY_MM_DDThh_mm_ss_sssZ "based on ISO-8601" format. In the dataset
            /// two tables will be created, `predictions`, and `errors`.
            /// If the Model has both [instance][google.cloud.aiplatform.v1beta1.PredictSchemata.instance_schema_uri]
            /// and [prediction][google.cloud.aiplatform.v1beta1.PredictSchemata.parameters_schema_uri] schemata
            /// defined then the tables have columns as follows: The `predictions`
            /// table contains instances for which the prediction succeeded, it
            /// has columns as per a concatenation of the Model's instance and
            /// prediction schemata. The `errors` table contains rows for which the
            /// prediction has failed, it has instance columns, as per the
            /// instance schema, followed by a single "errors" column, which as values
            /// has [`google.rpc.Status`](Status)
            /// represented as a STRUCT, and containing only `code` and `message`.
            #[prost(message, tag = "3")]
            BigqueryDestination(super::super::BigQueryDestination),
        }
    }
    /// Further describes this job's output.
    /// Supplements [output_config][google.cloud.aiplatform.v1beta1.BatchPredictionJob.output_config].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputInfo {
        /// The output location into which prediction output is written.
        #[prost(oneof = "output_info::OutputLocation", tags = "1, 2")]
        pub output_location: ::core::option::Option<output_info::OutputLocation>,
    }
    /// Nested message and enum types in `OutputInfo`.
    pub mod output_info {
        /// The output location into which prediction output is written.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OutputLocation {
            /// Output only. The full path of the Cloud Storage directory created, into which
            /// the prediction output is written.
            #[prost(string, tag = "1")]
            GcsOutputDirectory(::prost::alloc::string::String),
            /// Output only. The path of the BigQuery dataset created, in
            /// `bq://projectId.bqDatasetId`
            /// format, into which the prediction output is written.
            #[prost(string, tag = "2")]
            BigqueryOutputDataset(::prost::alloc::string::String),
        }
    }
}
/// Represents an environment variable present in a Container or Python Module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvVar {
    /// Required. Name of the environment variable. Must be a valid C identifier.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Variables that reference a $(VAR_NAME) are expanded
    /// using the previous defined environment variables in the container and
    /// any service environment variables. If a variable cannot be resolved,
    /// the reference in the input string will be unchanged. The $(VAR_NAME)
    /// syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped
    /// references will never be expanded, regardless of whether the variable
    /// exists or not.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Represents a job that runs custom workloads such as a Docker container or a
/// Python package. A CustomJob can have multiple worker pools and each worker
/// pool can have its own machine and input spec. A CustomJob will be cleaned up
/// once the job enters terminal state (failed or succeeded).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomJob {
    /// Output only. Resource name of a CustomJob.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the CustomJob.
    /// The name can be up to 128 characters long and can be consist of any UTF-8
    /// characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Job spec.
    #[prost(message, optional, tag = "4")]
    pub job_spec: ::core::option::Option<CustomJobSpec>,
    /// Output only. The detailed state of the job.
    #[prost(enumeration = "JobState", tag = "5")]
    pub state: i32,
    /// Output only. Time when the CustomJob was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the CustomJob for the first time entered the
    /// `JOB_STATE_RUNNING` state.
    #[prost(message, optional, tag = "7")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the CustomJob entered any of the following states:
    /// `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`.
    #[prost(message, optional, tag = "8")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the CustomJob was most recently updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Only populated when job's state is `JOB_STATE_FAILED` or
    /// `JOB_STATE_CANCELLED`.
    #[prost(message, optional, tag = "10")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The labels with user-defined metadata to organize CustomJobs.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    ///
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    #[prost(map = "string, string", tag = "11")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Customer-managed encryption key options for a CustomJob. If this is set,
    /// then all resources created by the CustomJob will be encrypted with the
    /// provided encryption key.
    #[prost(message, optional, tag = "12")]
    pub encryption_spec: ::core::option::Option<EncryptionSpec>,
}
/// Represents the spec of a CustomJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomJobSpec {
    /// Required. The spec of the worker pools including machine type and Docker image.
    /// All worker pools except the first one are optional and can be skipped by
    /// providing an empty value.
    #[prost(message, repeated, tag = "1")]
    pub worker_pool_specs: ::prost::alloc::vec::Vec<WorkerPoolSpec>,
    /// Scheduling options for a CustomJob.
    #[prost(message, optional, tag = "3")]
    pub scheduling: ::core::option::Option<Scheduling>,
    /// Specifies the service account for workload run-as account.
    /// Users submitting jobs must have act-as permission on this run-as account.
    /// If unspecified, the AI Platform Custom Code Service Agent for the
    /// CustomJob's project is used.
    #[prost(string, tag = "4")]
    pub service_account: ::prost::alloc::string::String,
    /// The full name of the Compute Engine
    /// [network](/compute/docs/networks-and-firewalls#networks) to which the Job
    /// should be peered. For example, `projects/12345/global/networks/myVPC`.
    /// [Format](/compute/docs/reference/rest/v1/networks/insert)
    /// is of the form `projects/{project}/global/networks/{network}`.
    /// Where {project} is a project number, as in `12345`, and {network} is a
    /// network name.
    ///
    /// Private services access must already be configured for the network. If left
    /// unspecified, the job is not peered with any network.
    #[prost(string, tag = "5")]
    pub network: ::prost::alloc::string::String,
    /// The Cloud Storage location to store the output of this CustomJob or
    /// HyperparameterTuningJob. For HyperparameterTuningJob,
    /// the baseOutputDirectory of
    /// each child CustomJob backing a Trial is set to a subdirectory of name
    /// [id][google.cloud.aiplatform.v1beta1.Trial.id] under its parent HyperparameterTuningJob's
    /// baseOutputDirectory.
    ///
    /// The following AI Platform environment variables will be passed to
    /// containers or python modules when this field is set:
    ///
    ///   For CustomJob:
    ///
    ///   * AIP_MODEL_DIR = `<base_output_directory>/model/`
    ///   * AIP_CHECKPOINT_DIR = `<base_output_directory>/checkpoints/`
    ///   * AIP_TENSORBOARD_LOG_DIR = `<base_output_directory>/logs/`
    ///
    ///   For CustomJob backing a Trial of HyperparameterTuningJob:
    ///
    ///   * AIP_MODEL_DIR = `<base_output_directory>/<trial_id>/model/`
    ///   * AIP_CHECKPOINT_DIR = `<base_output_directory>/<trial_id>/checkpoints/`
    ///   * AIP_TENSORBOARD_LOG_DIR = `<base_output_directory>/<trial_id>/logs/`
    #[prost(message, optional, tag = "6")]
    pub base_output_directory: ::core::option::Option<GcsDestination>,
}
/// Represents the spec of a worker pool in a job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkerPoolSpec {
    /// Optional. Immutable. The specification of a single machine.
    #[prost(message, optional, tag = "1")]
    pub machine_spec: ::core::option::Option<MachineSpec>,
    /// Optional. The number of worker replicas to use for this worker pool.
    #[prost(int64, tag = "2")]
    pub replica_count: i64,
    /// Disk spec.
    #[prost(message, optional, tag = "5")]
    pub disk_spec: ::core::option::Option<DiskSpec>,
    /// The custom task to be executed in this worker pool.
    #[prost(oneof = "worker_pool_spec::Task", tags = "6, 7")]
    pub task: ::core::option::Option<worker_pool_spec::Task>,
}
/// Nested message and enum types in `WorkerPoolSpec`.
pub mod worker_pool_spec {
    /// The custom task to be executed in this worker pool.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Task {
        /// The custom container task.
        #[prost(message, tag = "6")]
        ContainerSpec(super::ContainerSpec),
        /// The Python packaged task.
        #[prost(message, tag = "7")]
        PythonPackageSpec(super::PythonPackageSpec),
    }
}
/// The spec of a Container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerSpec {
    /// Required. The URI of a container image in the Container Registry that is to be run on
    /// each worker replica.
    #[prost(string, tag = "1")]
    pub image_uri: ::prost::alloc::string::String,
    /// The command to be invoked when the container is started.
    /// It overrides the entrypoint instruction in Dockerfile when provided.
    #[prost(string, repeated, tag = "2")]
    pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The arguments to be passed when starting the container.
    #[prost(string, repeated, tag = "3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The spec of a Python packaged code.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PythonPackageSpec {
    /// Required. The URI of a container image in Artifact Registry that will run the
    /// provided Python package. AI Platform provides a wide range of executor
    /// images with pre-installed packages to meet users' various use cases. See
    /// the list of [pre-built containers for
    /// training](https://cloud.google.com/ai-platform-unified/docs/training/pre-built-containers).
    /// You must use an image from this list.
    #[prost(string, tag = "1")]
    pub executor_image_uri: ::prost::alloc::string::String,
    /// Required. The Google Cloud Storage location of the Python package files which are
    /// the training program and its dependent packages.
    /// The maximum number of package URIs is 100.
    #[prost(string, repeated, tag = "2")]
    pub package_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The Python module name to run after installing the packages.
    #[prost(string, tag = "3")]
    pub python_module: ::prost::alloc::string::String,
    /// Command line arguments to be passed to the Python task.
    #[prost(string, repeated, tag = "4")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// All parameters related to queuing and scheduling of custom jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scheduling {
    /// The maximum job running time. The default is 7 days.
    #[prost(message, optional, tag = "1")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Restarts the entire CustomJob if a worker gets restarted.
    /// This feature can be used by distributed training jobs that are not
    /// resilient to workers leaving and joining a job.
    #[prost(bool, tag = "3")]
    pub restart_job_on_worker_restart: bool,
}
/// A piece of data in a Dataset. Could be an image, a video, a document or plain
/// text.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataItem {
    /// Output only. The resource name of the DataItem.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Timestamp when this DataItem was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when this DataItem was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The labels with user-defined metadata to organize your DataItems.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// No more than 64 user labels can be associated with one DataItem(System
    /// labels are excluded).
    ///
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    /// System reserved label keys are prefixed with "aiplatform.googleapis.com/"
    /// and are immutable.
    #[prost(map = "string, string", tag = "3")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. The data that the DataItem represents (for example, an image or a text
    /// snippet). The schema of the payload is stored in the parent Dataset's
    /// [metadata schema's][google.cloud.aiplatform.v1beta1.Dataset.metadata_schema_uri] dataItemSchemaUri field.
    #[prost(message, optional, tag = "4")]
    pub payload: ::core::option::Option<::prost_types::Value>,
    /// Optional. Used to perform consistent read-modify-write updates. If not set, a blind
    /// "overwrite" update happens.
    #[prost(string, tag = "7")]
    pub etag: ::prost::alloc::string::String,
}
/// SpecialistPool represents customers' own workforce to work on their data
/// labeling jobs. It includes a group of specialist managers who are responsible
/// for managing the labelers in this pool as well as customers' data labeling
/// jobs associated with this pool.
/// Customers create specialist pool as well as start data labeling jobs on
/// Cloud, managers and labelers work with the jobs using CrowdCompute console.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecialistPool {
    /// Required. The resource name of the SpecialistPool.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The user-defined name of the SpecialistPool.
    /// The name can be up to 128 characters long and can be consist of any UTF-8
    /// characters.
    /// This field should be unique on project-level.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The number of Specialists in this SpecialistPool.
    #[prost(int32, tag = "3")]
    pub specialist_managers_count: i32,
    /// The email addresses of the specialists in the SpecialistPool.
    #[prost(string, repeated, tag = "4")]
    pub specialist_manager_emails: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The resource name of the pending data labeling jobs.
    #[prost(string, repeated, tag = "5")]
    pub pending_data_labeling_jobs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DataLabelingJob is used to trigger a human labeling job on unlabeled data
/// from the following Dataset:
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataLabelingJob {
    /// Output only. Resource name of the DataLabelingJob.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The user-defined name of the DataLabelingJob.
    /// The name can be up to 128 characters long and can be consist of any UTF-8
    /// characters.
    /// Display name of a DataLabelingJob.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Dataset resource names. Right now we only support labeling from a single
    /// Dataset.
    /// Format:
    /// `projects/{project}/locations/{location}/datasets/{dataset}`
    #[prost(string, repeated, tag = "3")]
    pub datasets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Labels to assign to annotations generated by this DataLabelingJob.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    /// System reserved label keys are prefixed with "aiplatform.googleapis.com/"
    /// and are immutable.
    #[prost(map = "string, string", tag = "12")]
    pub annotation_labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. Number of labelers to work on each DataItem.
    #[prost(int32, tag = "4")]
    pub labeler_count: i32,
    /// Required. The Google Cloud Storage location of the instruction pdf. This pdf is
    /// shared with labelers, and provides detailed description on how to label
    /// DataItems in Datasets.
    #[prost(string, tag = "5")]
    pub instruction_uri: ::prost::alloc::string::String,
    /// Required. Points to a YAML file stored on Google Cloud Storage describing the
    /// config for a specific type of DataLabelingJob.
    /// The schema files that can be used here are found in the
    /// https://storage.googleapis.com/google-cloud-aiplatform bucket in the
    /// /schema/datalabelingjob/inputs/ folder.
    #[prost(string, tag = "6")]
    pub inputs_schema_uri: ::prost::alloc::string::String,
    /// Required. Input config parameters for the DataLabelingJob.
    #[prost(message, optional, tag = "7")]
    pub inputs: ::core::option::Option<::prost_types::Value>,
    /// Output only. The detailed state of the job.
    #[prost(enumeration = "JobState", tag = "8")]
    pub state: i32,
    /// Output only. Current labeling job progress percentage scaled in interval [0, 100],
    /// indicating the percentage of DataItems that has been finished.
    #[prost(int32, tag = "13")]
    pub labeling_progress: i32,
    /// Output only. Estimated cost(in US dollars) that the DataLabelingJob has incurred to
    /// date.
    #[prost(message, optional, tag = "14")]
    pub current_spend: ::core::option::Option<super::super::super::r#type::Money>,
    /// Output only. Timestamp when this DataLabelingJob was created.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when this DataLabelingJob was updated most recently.
    #[prost(message, optional, tag = "10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. DataLabelingJob errors. It is only populated when job's state is
    /// `JOB_STATE_FAILED` or `JOB_STATE_CANCELLED`.
    #[prost(message, optional, tag = "22")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The labels with user-defined metadata to organize your DataLabelingJobs.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    ///
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    /// System reserved label keys are prefixed with "aiplatform.googleapis.com/"
    /// and are immutable. Following system labels exist for each DataLabelingJob:
    ///
    /// * "aiplatform.googleapis.com/schema": output only, its value is the
    ///   [inputs_schema][google.cloud.aiplatform.v1beta1.DataLabelingJob.inputs_schema_uri]'s title.
    #[prost(map = "string, string", tag = "11")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The SpecialistPools' resource names associated with this job.
    #[prost(string, repeated, tag = "16")]
    pub specialist_pools: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Customer-managed encryption key spec for a DataLabelingJob. If set, this
    /// DataLabelingJob will be secured by this key.
    ///
    /// Note: Annotations created in the DataLabelingJob are associated with
    /// the EncryptionSpec of the Dataset they are exported to.
    #[prost(message, optional, tag = "20")]
    pub encryption_spec: ::core::option::Option<EncryptionSpec>,
    /// Parameters that configure the active learning pipeline. Active learning
    /// will label the data incrementally via several iterations. For every
    /// iteration, it will select a batch of data based on the sampling strategy.
    #[prost(message, optional, tag = "21")]
    pub active_learning_config: ::core::option::Option<ActiveLearningConfig>,
}
/// Parameters that configure the active learning pipeline. Active learning will
///  label the data incrementally by several iterations. For every iteration, it
///  will select a batch of data based on the sampling strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActiveLearningConfig {
    /// Active learning data sampling config. For every active learning labeling
    /// iteration, it will select a batch of data based on the sampling strategy.
    #[prost(message, optional, tag = "3")]
    pub sample_config: ::core::option::Option<SampleConfig>,
    /// CMLE training config. For every active learning labeling iteration, system
    /// will train a machine learning model on CMLE. The trained model will be used
    /// by data sampling algorithm to select DataItems.
    #[prost(message, optional, tag = "4")]
    pub training_config: ::core::option::Option<TrainingConfig>,
    /// Required. Max human labeling DataItems. The rest part will be labeled by
    /// machine.
    #[prost(oneof = "active_learning_config::HumanLabelingBudget", tags = "1, 2")]
    pub human_labeling_budget: ::core::option::Option<active_learning_config::HumanLabelingBudget>,
}
/// Nested message and enum types in `ActiveLearningConfig`.
pub mod active_learning_config {
    /// Required. Max human labeling DataItems. The rest part will be labeled by
    /// machine.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HumanLabelingBudget {
        /// Max number of human labeled DataItems.
        #[prost(int64, tag = "1")]
        MaxDataItemCount(i64),
        /// Max percent of total DataItems for human labeling.
        #[prost(int32, tag = "2")]
        MaxDataItemPercentage(i32),
    }
}
/// Active learning data sampling config. For every active learning labeling
/// iteration, it will select a batch of data based on the sampling strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleConfig {
    /// Field to choose sampling strategy. Sampling strategy will decide which data
    /// should be selected for human labeling in every batch.
    #[prost(enumeration = "sample_config::SampleStrategy", tag = "5")]
    pub sample_strategy: i32,
    /// Decides sample size for the initial batch. initial_batch_sample_percentage
    /// is used by default.
    #[prost(oneof = "sample_config::InitialBatchSampleSize", tags = "1")]
    pub initial_batch_sample_size: ::core::option::Option<sample_config::InitialBatchSampleSize>,
    /// Decides sample size for the following batches.
    /// following_batch_sample_percentage is used by default.
    #[prost(oneof = "sample_config::FollowingBatchSampleSize", tags = "3")]
    pub following_batch_sample_size:
        ::core::option::Option<sample_config::FollowingBatchSampleSize>,
}
/// Nested message and enum types in `SampleConfig`.
pub mod sample_config {
    /// Sample strategy decides which subset of DataItems should be selected for
    /// human labeling in every batch.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SampleStrategy {
        /// Default will be treated as UNCERTAINTY.
        Unspecified = 0,
        /// Sample the most uncertain data to label.
        Uncertainty = 1,
    }
    /// Decides sample size for the initial batch. initial_batch_sample_percentage
    /// is used by default.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InitialBatchSampleSize {
        /// The percentage of data needed to be labeled in the first batch.
        #[prost(int32, tag = "1")]
        InitialBatchSamplePercentage(i32),
    }
    /// Decides sample size for the following batches.
    /// following_batch_sample_percentage is used by default.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FollowingBatchSampleSize {
        /// The percentage of data needed to be labeled in each following batch
        /// (except the first batch).
        #[prost(int32, tag = "3")]
        FollowingBatchSamplePercentage(i32),
    }
}
/// CMLE training config. For every active learning labeling iteration, system
/// will train a machine learning model on CMLE. The trained model will be used
/// by data sampling algorithm to select DataItems.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainingConfig {
    /// The timeout hours for the CMLE training job, expressed in milli hours
    /// i.e. 1,000 value in this field means 1 hour.
    #[prost(int64, tag = "1")]
    pub timeout_training_milli_hours: i64,
}
/// A collection of DataItems and Annotations on them.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    /// Output only. The resource name of the Dataset.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The user-defined name of the Dataset.
    /// The name can be up to 128 characters long and can be consist of any UTF-8
    /// characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Points to a YAML file stored on Google Cloud Storage describing additional
    /// information about the Dataset.
    /// The schema is defined as an OpenAPI 3.0.2 Schema Object.
    /// The schema files that can be used here are found in
    /// gs://google-cloud-aiplatform/schema/dataset/metadata/.
    #[prost(string, tag = "3")]
    pub metadata_schema_uri: ::prost::alloc::string::String,
    /// Required. Additional information about the Dataset.
    #[prost(message, optional, tag = "8")]
    pub metadata: ::core::option::Option<::prost_types::Value>,
    /// Output only. Timestamp when this Dataset was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when this Dataset was last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Used to perform consistent read-modify-write updates. If not set, a blind
    /// "overwrite" update happens.
    #[prost(string, tag = "6")]
    pub etag: ::prost::alloc::string::String,
    /// The labels with user-defined metadata to organize your Datasets.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// No more than 64 user labels can be associated with one Dataset (System
    /// labels are excluded).
    ///
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    /// System reserved label keys are prefixed with "aiplatform.googleapis.com/"
    /// and are immutable. Following system labels exist for each Dataset:
    ///
    /// * "aiplatform.googleapis.com/dataset_metadata_schema": output only, its
    ///   value is the [metadata_schema's][google.cloud.aiplatform.v1beta1.Dataset.metadata_schema_uri] title.
    #[prost(map = "string, string", tag = "7")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Customer-managed encryption key spec for a Dataset. If set, this Dataset
    /// and all sub-resources of this Dataset will be secured by this key.
    #[prost(message, optional, tag = "11")]
    pub encryption_spec: ::core::option::Option<EncryptionSpec>,
}
/// Describes the location from where we import data into a Dataset, together
/// with the labels that will be applied to the DataItems and the Annotations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataConfig {
    /// Labels that will be applied to newly imported DataItems. If an identical
    /// DataItem as one being imported already exists in the Dataset, then these
    /// labels will be appended to these of the already existing one, and if labels
    /// with identical key is imported before, the old label value will be
    /// overwritten. If two DataItems are identical in the same import data
    /// operation, the labels will be combined and if key collision happens in this
    /// case, one of the values will be picked randomly. Two DataItems are
    /// considered identical if their content bytes are identical (e.g. image bytes
    /// or pdf bytes).
    /// These labels will be overridden by Annotation labels specified inside index
    /// file referenced by [import_schema_uri][google.cloud.aiplatform.v1beta1.ImportDataConfig.import_schema_uri], e.g. jsonl file.
    #[prost(map = "string, string", tag = "2")]
    pub data_item_labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. Points to a YAML file stored on Google Cloud Storage describing the import
    /// format. Validation will be done against the schema. The schema is defined
    /// as an [OpenAPI 3.0.2 Schema Object](https://tinyurl.com/y538mdwt).
    #[prost(string, tag = "4")]
    pub import_schema_uri: ::prost::alloc::string::String,
    /// The source of the input.
    #[prost(oneof = "import_data_config::Source", tags = "1")]
    pub source: ::core::option::Option<import_data_config::Source>,
}
/// Nested message and enum types in `ImportDataConfig`.
pub mod import_data_config {
    /// The source of the input.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Google Cloud Storage location for the input content.
        #[prost(message, tag = "1")]
        GcsSource(super::GcsSource),
    }
}
/// Describes what part of the Dataset is to be exported, the destination of
/// the export and how to export.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataConfig {
    /// A filter on Annotations of the Dataset. Only Annotations on to-be-exported
    /// DataItems(specified by [data_items_filter][]) that match this filter will
    /// be exported. The filter syntax is the same as in
    /// [ListAnnotations][google.cloud.aiplatform.v1beta1.DatasetService.ListAnnotations].
    #[prost(string, tag = "2")]
    pub annotations_filter: ::prost::alloc::string::String,
    /// The destination of the output.
    #[prost(oneof = "export_data_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<export_data_config::Destination>,
}
/// Nested message and enum types in `ExportDataConfig`.
pub mod export_data_config {
    /// The destination of the output.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// The Google Cloud Storage location where the output is to be written to.
        /// In the given directory a new directory will be created with name:
        /// `export-data-<dataset-display-name>-<timestamp-of-export-call>` where
        /// timestamp is in YYYY-MM-DDThh:mm:ss.sssZ ISO-8601 format. All export
        /// output will be written into that directory. Inside that directory,
        /// annotations with the same schema will be grouped into sub directories
        /// which are named with the corresponding annotations' schema title. Inside
        /// these sub directories, a schema.yaml will be created to describe the
        /// output format.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// Generic Metadata shared by all operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenericOperationMetadata {
    /// Output only. Partial failures encountered.
    /// E.g. single files that couldn't be read.
    /// This field should never exceed 20 entries.
    /// Status details field will contain standard GCP error details.
    #[prost(message, repeated, tag = "1")]
    pub partial_failures: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Output only. Time when the operation was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the operation was updated for the last time.
    /// If the operation has finished (successfully or not), this is the finish
    /// time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Details of operations that perform deletes of any entities.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOperationMetadata {
    /// The common part of the operation metadata.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
}
/// Points to a DeployedModel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployedModelRef {
    /// Immutable. A resource name of an Endpoint.
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    /// Immutable. An ID of a DeployedModel in the above Endpoint.
    #[prost(string, tag = "2")]
    pub deployed_model_id: ::prost::alloc::string::String,
}
/// A trained machine learning Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// The resource name of the Model.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the Model.
    /// The name can be up to 128 characters long and can be consist of any UTF-8
    /// characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The description of the Model.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The schemata that describe formats of the Model's predictions and
    /// explanations as given and returned via
    /// [PredictionService.Predict][google.cloud.aiplatform.v1beta1.PredictionService.Predict] and [PredictionService.Explain][google.cloud.aiplatform.v1beta1.PredictionService.Explain].
    #[prost(message, optional, tag = "4")]
    pub predict_schemata: ::core::option::Option<PredictSchemata>,
    /// Immutable. Points to a YAML file stored on Google Cloud Storage describing additional
    /// information about the Model, that is specific to it. Unset if the Model
    /// does not have any additional information.
    /// The schema is defined as an OpenAPI 3.0.2
    /// [Schema Object](https://tinyurl.com/y538mdwt#schema-object).
    /// AutoML Models always have this field populated by AI Platform, if no
    /// additional metadata is needed, this field is set to an empty string.
    /// Note: The URI given on output will be immutable and probably different,
    /// including the URI scheme, than the one given on input. The output URI will
    /// point to a location where the user only has a read access.
    #[prost(string, tag = "5")]
    pub metadata_schema_uri: ::prost::alloc::string::String,
    /// Immutable. An additional information about the Model; the schema of the metadata can
    /// be found in [metadata_schema][google.cloud.aiplatform.v1beta1.Model.metadata_schema_uri].
    /// Unset if the Model does not have any additional information.
    #[prost(message, optional, tag = "6")]
    pub metadata: ::core::option::Option<::prost_types::Value>,
    /// Output only. The formats in which this Model may be exported. If empty, this Model is
    /// not available for export.
    #[prost(message, repeated, tag = "20")]
    pub supported_export_formats: ::prost::alloc::vec::Vec<model::ExportFormat>,
    /// Output only. The resource name of the TrainingPipeline that uploaded this Model, if any.
    #[prost(string, tag = "7")]
    pub training_pipeline: ::prost::alloc::string::String,
    /// Input only. The specification of the container that is to be used when deploying
    /// this Model. The specification is ingested upon
    /// [ModelService.UploadModel][google.cloud.aiplatform.v1beta1.ModelService.UploadModel], and all binaries it contains are copied
    /// and stored internally by AI Platform.
    /// Not present for AutoML Models.
    #[prost(message, optional, tag = "9")]
    pub container_spec: ::core::option::Option<ModelContainerSpec>,
    /// Immutable. The path to the directory containing the Model artifact and any of its
    /// supporting files.
    /// Not present for AutoML Models.
    #[prost(string, tag = "26")]
    pub artifact_uri: ::prost::alloc::string::String,
    /// Output only. When this Model is deployed, its prediction resources are described by the
    /// `prediction_resources` field of the [Endpoint.deployed_models][google.cloud.aiplatform.v1beta1.Endpoint.deployed_models] object.
    /// Because not all Models support all resource configuration types, the
    /// configuration types this Model supports are listed here. If no
    /// configuration types are listed, the Model cannot be deployed to an
    /// [Endpoint][google.cloud.aiplatform.v1beta1.Endpoint] and does not support
    /// online predictions ([PredictionService.Predict][google.cloud.aiplatform.v1beta1.PredictionService.Predict] or
    /// [PredictionService.Explain][google.cloud.aiplatform.v1beta1.PredictionService.Explain]). Such a Model can serve predictions by
    /// using a [BatchPredictionJob][google.cloud.aiplatform.v1beta1.BatchPredictionJob], if it has at least one entry each in
    /// [supported_input_storage_formats][google.cloud.aiplatform.v1beta1.Model.supported_input_storage_formats] and
    /// [supported_output_storage_formats][google.cloud.aiplatform.v1beta1.Model.supported_output_storage_formats].
    #[prost(
        enumeration = "model::DeploymentResourcesType",
        repeated,
        packed = "false",
        tag = "10"
    )]
    pub supported_deployment_resources_types: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The formats this Model supports in
    /// [BatchPredictionJob.input_config][google.cloud.aiplatform.v1beta1.BatchPredictionJob.input_config]. If
    /// [PredictSchemata.instance_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.instance_schema_uri] exists, the instances
    /// should be given as per that schema.
    ///
    /// The possible formats are:
    ///
    /// * `jsonl`
    /// The JSON Lines format, where each instance is a single line. Uses
    /// [GcsSource][google.cloud.aiplatform.v1beta1.BatchPredictionJob.InputConfig.gcs_source].
    ///
    /// * `csv`
    /// The CSV format, where each instance is a single comma-separated line.
    /// The first line in the file is the header, containing comma-separated field
    /// names. Uses [GcsSource][google.cloud.aiplatform.v1beta1.BatchPredictionJob.InputConfig.gcs_source].
    ///
    /// * `tf-record`
    /// The TFRecord format, where each instance is a single record in tfrecord
    /// syntax. Uses [GcsSource][google.cloud.aiplatform.v1beta1.BatchPredictionJob.InputConfig.gcs_source].
    ///
    /// * `tf-record-gzip`
    /// Similar to `tf-record`, but the file is gzipped. Uses
    /// [GcsSource][google.cloud.aiplatform.v1beta1.BatchPredictionJob.InputConfig.gcs_source].
    ///
    /// * `bigquery`
    /// Each instance is a single row in BigQuery. Uses
    /// [BigQuerySource][google.cloud.aiplatform.v1beta1.BatchPredictionJob.InputConfig.bigquery_source].
    ///
    /// * `file-list`
    /// Each line of the file is the location of an instance to process, uses
    /// `gcs_source` field of the
    /// [InputConfig][google.cloud.aiplatform.v1beta1.BatchPredictionJob.InputConfig] object.
    ///
    ///
    /// If this Model doesn't support any of these formats it means it cannot be
    /// used with a [BatchPredictionJob][google.cloud.aiplatform.v1beta1.BatchPredictionJob]. However, if it has
    /// [supported_deployment_resources_types][google.cloud.aiplatform.v1beta1.Model.supported_deployment_resources_types], it could serve online
    /// predictions by using [PredictionService.Predict][google.cloud.aiplatform.v1beta1.PredictionService.Predict] or
    /// [PredictionService.Explain][google.cloud.aiplatform.v1beta1.PredictionService.Explain].
    #[prost(string, repeated, tag = "11")]
    pub supported_input_storage_formats: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The formats this Model supports in
    /// [BatchPredictionJob.output_config][google.cloud.aiplatform.v1beta1.BatchPredictionJob.output_config]. If both
    /// [PredictSchemata.instance_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.instance_schema_uri] and
    /// [PredictSchemata.prediction_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.prediction_schema_uri] exist, the predictions
    /// are returned together with their instances. In other words, the
    /// prediction has the original instance data first, followed
    /// by the actual prediction content (as per the schema).
    ///
    /// The possible formats are:
    ///
    /// * `jsonl`
    /// The JSON Lines format, where each prediction is a single line. Uses
    /// [GcsDestination][google.cloud.aiplatform.v1beta1.BatchPredictionJob.OutputConfig.gcs_destination].
    ///
    /// * `csv`
    /// The CSV format, where each prediction is a single comma-separated line.
    /// The first line in the file is the header, containing comma-separated field
    /// names. Uses
    /// [GcsDestination][google.cloud.aiplatform.v1beta1.BatchPredictionJob.OutputConfig.gcs_destination].
    ///
    /// * `bigquery`
    /// Each prediction is a single row in a BigQuery table, uses
    /// [BigQueryDestination][google.cloud.aiplatform.v1beta1.BatchPredictionJob.OutputConfig.bigquery_destination]
    /// .
    ///
    ///
    /// If this Model doesn't support any of these formats it means it cannot be
    /// used with a [BatchPredictionJob][google.cloud.aiplatform.v1beta1.BatchPredictionJob]. However, if it has
    /// [supported_deployment_resources_types][google.cloud.aiplatform.v1beta1.Model.supported_deployment_resources_types], it could serve online
    /// predictions by using [PredictionService.Predict][google.cloud.aiplatform.v1beta1.PredictionService.Predict] or
    /// [PredictionService.Explain][google.cloud.aiplatform.v1beta1.PredictionService.Explain].
    #[prost(string, repeated, tag = "12")]
    pub supported_output_storage_formats: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Timestamp when this Model was uploaded into AI Platform.
    #[prost(message, optional, tag = "13")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when this Model was most recently updated.
    #[prost(message, optional, tag = "14")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The pointers to DeployedModels created from this Model. Note that
    /// Model could have been deployed to Endpoints in different Locations.
    #[prost(message, repeated, tag = "15")]
    pub deployed_models: ::prost::alloc::vec::Vec<DeployedModelRef>,
    /// The default explanation specification for this Model.
    ///
    /// The Model can be used for [requesting
    /// explanation][PredictionService.Explain] after being
    /// [deployed][google.cloud.aiplatform.v1beta1.EndpointService.DeployModel] if it is populated.
    /// The Model can be used for [batch
    /// explanation][BatchPredictionJob.generate_explanation] if it is populated.
    ///
    /// All fields of the explanation_spec can be overridden by
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.DeployedModel.explanation_spec] of
    /// [DeployModelRequest.deployed_model][google.cloud.aiplatform.v1beta1.DeployModelRequest.deployed_model], or
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.BatchPredictionJob.explanation_spec] of
    /// [BatchPredictionJob][google.cloud.aiplatform.v1beta1.BatchPredictionJob].
    ///
    /// If the default explanation specification is not set for this Model, this
    /// Model can still be used for [requesting
    /// explanation][PredictionService.Explain] by setting
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.DeployedModel.explanation_spec] of
    /// [DeployModelRequest.deployed_model][google.cloud.aiplatform.v1beta1.DeployModelRequest.deployed_model] and for [batch
    /// explanation][BatchPredictionJob.generate_explanation] by setting
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.BatchPredictionJob.explanation_spec] of
    /// [BatchPredictionJob][google.cloud.aiplatform.v1beta1.BatchPredictionJob].
    #[prost(message, optional, tag = "23")]
    pub explanation_spec: ::core::option::Option<ExplanationSpec>,
    /// Used to perform consistent read-modify-write updates. If not set, a blind
    /// "overwrite" update happens.
    #[prost(string, tag = "16")]
    pub etag: ::prost::alloc::string::String,
    /// The labels with user-defined metadata to organize your Models.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    ///
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    #[prost(map = "string, string", tag = "17")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Customer-managed encryption key spec for a Model. If set, this
    /// Model and all sub-resources of this Model will be secured by this key.
    #[prost(message, optional, tag = "24")]
    pub encryption_spec: ::core::option::Option<EncryptionSpec>,
}
/// Nested message and enum types in `Model`.
pub mod model {
    /// Represents export format supported by the Model.
    /// All formats export to Google Cloud Storage.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExportFormat {
        /// Output only. The ID of the export format.
        /// The possible format IDs are:
        ///
        /// * `tflite`
        /// Used for Android mobile devices.
        ///
        /// * `edgetpu-tflite`
        /// Used for [Edge TPU](https://cloud.google.com/edge-tpu/) devices.
        ///
        /// * `tf-saved-model`
        /// A tensorflow model in SavedModel format.
        ///
        /// * `tf-js`
        /// A [TensorFlow.js](https://www.tensorflow.org/js) model that can be used
        /// in the browser and in Node.js using JavaScript.
        ///
        /// * `core-ml`
        /// Used for iOS mobile devices.
        ///
        /// * `custom-trained`
        /// A Model that was uploaded or trained by custom code.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Output only. The content of this Model that may be exported.
        #[prost(
            enumeration = "export_format::ExportableContent",
            repeated,
            packed = "false",
            tag = "2"
        )]
        pub exportable_contents: ::prost::alloc::vec::Vec<i32>,
    }
    /// Nested message and enum types in `ExportFormat`.
    pub mod export_format {
        /// The Model content that can be exported.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum ExportableContent {
            /// Should not be used.
            Unspecified = 0,
            /// Model artifact and any of its supported files. Will be exported to the
            /// location specified by the `artifactDestination` field of the
            /// [ExportModelRequest.output_config][google.cloud.aiplatform.v1beta1.ExportModelRequest.output_config] object.
            Artifact = 1,
            /// The container image that is to be used when deploying this Model. Will
            /// be exported to the location specified by the `imageDestination` field
            /// of the [ExportModelRequest.output_config][google.cloud.aiplatform.v1beta1.ExportModelRequest.output_config] object.
            Image = 2,
        }
    }
    /// Identifies a type of Model's prediction resources.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DeploymentResourcesType {
        /// Should not be used.
        Unspecified = 0,
        /// Resources that are dedicated to the [DeployedModel][google.cloud.aiplatform.v1beta1.DeployedModel], and that need a
        /// higher degree of manual configuration.
        DedicatedResources = 1,
        /// Resources that to large degree are decided by AI Platform, and require
        /// only a modest additional configuration.
        AutomaticResources = 2,
    }
}
/// Contains the schemata used in Model's predictions and explanations via
/// [PredictionService.Predict][google.cloud.aiplatform.v1beta1.PredictionService.Predict], [PredictionService.Explain][google.cloud.aiplatform.v1beta1.PredictionService.Explain] and
/// [BatchPredictionJob][google.cloud.aiplatform.v1beta1.BatchPredictionJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictSchemata {
    /// Immutable. Points to a YAML file stored on Google Cloud Storage describing the format
    /// of a single instance, which are used in [PredictRequest.instances][google.cloud.aiplatform.v1beta1.PredictRequest.instances],
    /// [ExplainRequest.instances][google.cloud.aiplatform.v1beta1.ExplainRequest.instances] and
    /// [BatchPredictionJob.input_config][google.cloud.aiplatform.v1beta1.BatchPredictionJob.input_config].
    /// The schema is defined as an OpenAPI 3.0.2
    /// [Schema Object](https://tinyurl.com/y538mdwt#schema-object).
    /// AutoML Models always have this field populated by AI Platform.
    /// Note: The URI given on output will be immutable and probably different,
    /// including the URI scheme, than the one given on input. The output URI will
    /// point to a location where the user only has a read access.
    #[prost(string, tag = "1")]
    pub instance_schema_uri: ::prost::alloc::string::String,
    /// Immutable. Points to a YAML file stored on Google Cloud Storage describing the
    /// parameters of prediction and explanation via
    /// [PredictRequest.parameters][google.cloud.aiplatform.v1beta1.PredictRequest.parameters], [ExplainRequest.parameters][google.cloud.aiplatform.v1beta1.ExplainRequest.parameters] and
    /// [BatchPredictionJob.model_parameters][google.cloud.aiplatform.v1beta1.BatchPredictionJob.model_parameters].
    /// The schema is defined as an OpenAPI 3.0.2
    /// [Schema Object](https://tinyurl.com/y538mdwt#schema-object).
    /// AutoML Models always have this field populated by AI Platform, if no
    /// parameters are supported, then it is set to an empty string.
    /// Note: The URI given on output will be immutable and probably different,
    /// including the URI scheme, than the one given on input. The output URI will
    /// point to a location where the user only has a read access.
    #[prost(string, tag = "2")]
    pub parameters_schema_uri: ::prost::alloc::string::String,
    /// Immutable. Points to a YAML file stored on Google Cloud Storage describing the format
    /// of a single prediction produced by this Model, which are returned via
    /// [PredictResponse.predictions][google.cloud.aiplatform.v1beta1.PredictResponse.predictions], [ExplainResponse.explanations][google.cloud.aiplatform.v1beta1.ExplainResponse.explanations], and
    /// [BatchPredictionJob.output_config][google.cloud.aiplatform.v1beta1.BatchPredictionJob.output_config].
    /// The schema is defined as an OpenAPI 3.0.2
    /// [Schema Object](https://tinyurl.com/y538mdwt#schema-object).
    /// AutoML Models always have this field populated by AI Platform.
    /// Note: The URI given on output will be immutable and probably different,
    /// including the URI scheme, than the one given on input. The output URI will
    /// point to a location where the user only has a read access.
    #[prost(string, tag = "3")]
    pub prediction_schema_uri: ::prost::alloc::string::String,
}
/// Specification of a container for serving predictions. This message is a
/// subset of the Kubernetes Container v1 core
/// [specification](https://tinyurl.com/k8s-io-api/v1.18/#container-v1-core).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelContainerSpec {
    /// Required. Immutable. URI of the Docker image to be used as the custom container for serving
    /// predictions. This URI must identify an image in Artifact Registry or
    /// Container Registry. Learn more about the container publishing
    /// requirements, including permissions requirements for the AI Platform
    /// Service Agent,
    /// [here](https://tinyurl.com/cust-cont-reqs#publishing).
    ///
    /// The container image is ingested upon [ModelService.UploadModel][google.cloud.aiplatform.v1beta1.ModelService.UploadModel], stored
    /// internally, and this original path is afterwards not used.
    ///
    /// To learn about the requirements for the Docker image itself, see
    /// [Custom container requirements](https://tinyurl.com/cust-cont-reqs).
    ///
    /// You can use the URI to one of AI Platform's [pre-built container images for
    /// prediction](https://cloud.google.com/ai-platform-unified/docs/predictions/pre-built-containers)
    /// in this field.
    #[prost(string, tag = "1")]
    pub image_uri: ::prost::alloc::string::String,
    /// Immutable. Specifies the command that runs when the container starts. This overrides
    /// the container's
    /// [ENTRYPOINT](https://docs.docker.com/engine/reference/builder/#entrypoint).
    /// Specify this field as an array of executable and arguments, similar to a
    /// Docker `ENTRYPOINT`'s "exec" form, not its "shell" form.
    ///
    /// If you do not specify this field, then the container's `ENTRYPOINT` runs,
    /// in conjunction with the [args][google.cloud.aiplatform.v1beta1.ModelContainerSpec.args] field or the
    /// container's [`CMD`](https://docs.docker.com/engine/reference/builder/#cmd),
    /// if either exists. If this field is not specified and the container does not
    /// have an `ENTRYPOINT`, then refer to the Docker documentation about how
    /// `CMD` and `ENTRYPOINT`
    /// [interact](https://tinyurl.com/h3kdcgs).
    ///
    /// If you specify this field, then you can also specify the `args` field to
    /// provide additional arguments for this command. However, if you specify this
    /// field, then the container's `CMD` is ignored. See the
    /// [Kubernetes documentation](https://tinyurl.com/y8bvllf4) about how the
    /// `command` and `args` fields interact with a container's `ENTRYPOINT` and
    /// `CMD`.
    ///
    /// In this field, you can reference environment variables
    /// [set by AI Platform](https://tinyurl.com/cust-cont-reqs#aip-variables)
    /// and environment variables set in the [env][google.cloud.aiplatform.v1beta1.ModelContainerSpec.env] field.
    /// You cannot reference environment variables set in the Docker image. In
    /// order for environment variables to be expanded, reference them by using the
    /// following syntax:
    /// <code>$(<var>VARIABLE_NAME</var>)</code>
    /// Note that this differs from Bash variable expansion, which does not use
    /// parentheses. If a variable cannot be resolved, the reference in the input
    /// string is used unchanged. To avoid variable expansion, you can escape this
    /// syntax with `$$`; for example:
    /// <code>$$(<var>VARIABLE_NAME</var>)</code>
    /// This field corresponds to the `command` field of the Kubernetes Containers
    /// [v1 core API](https://tinyurl.com/k8s-io-api/v1.18/#container-v1-core).
    #[prost(string, repeated, tag = "2")]
    pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Immutable. Specifies arguments for the command that runs when the container starts.
    /// This overrides the container's
    /// [`CMD`](https://docs.docker.com/engine/reference/builder/#cmd). Specify
    /// this field as an array of executable and arguments, similar to a Docker
    /// `CMD`'s "default parameters" form.
    ///
    /// If you don't specify this field but do specify the
    /// [command][google.cloud.aiplatform.v1beta1.ModelContainerSpec.command] field, then the command from the
    /// `command` field runs without any additional arguments. See the
    /// [Kubernetes documentation](https://tinyurl.com/y8bvllf4) about how the
    /// `command` and `args` fields interact with a container's `ENTRYPOINT` and
    /// `CMD`.
    ///
    /// If you don't specify this field and don't specify the `command` field,
    /// then the container's
    /// [`ENTRYPOINT`](https://docs.docker.com/engine/reference/builder/#cmd) and
    /// `CMD` determine what runs based on their default behavior. See the Docker
    /// documentation about how `CMD` and `ENTRYPOINT`
    /// [interact](https://tinyurl.com/h3kdcgs).
    ///
    /// In this field, you can reference environment variables
    /// [set by AI Platform](https://tinyurl.com/cust-cont-reqs#aip-variables)
    /// and environment variables set in the [env][google.cloud.aiplatform.v1beta1.ModelContainerSpec.env] field.
    /// You cannot reference environment variables set in the Docker image. In
    /// order for environment variables to be expanded, reference them by using the
    /// following syntax:
    /// <code>$(<var>VARIABLE_NAME</var>)</code>
    /// Note that this differs from Bash variable expansion, which does not use
    /// parentheses. If a variable cannot be resolved, the reference in the input
    /// string is used unchanged. To avoid variable expansion, you can escape this
    /// syntax with `$$`; for example:
    /// <code>$$(<var>VARIABLE_NAME</var>)</code>
    /// This field corresponds to the `args` field of the Kubernetes Containers
    /// [v1 core API](https://tinyurl.com/k8s-io-api/v1.18/#container-v1-core).
    #[prost(string, repeated, tag = "3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Immutable. List of environment variables to set in the container. After the container
    /// starts running, code running in the container can read these environment
    /// variables.
    ///
    /// Additionally, the [command][google.cloud.aiplatform.v1beta1.ModelContainerSpec.command] and
    /// [args][google.cloud.aiplatform.v1beta1.ModelContainerSpec.args] fields can reference these variables. Later
    /// entries in this list can also reference earlier entries. For example, the
    /// following example sets the variable `VAR_2` to have the value `foo bar`:
    ///
    /// ```json
    /// [
    ///   {
    ///     "name": "VAR_1",
    ///     "value": "foo"
    ///   },
    ///   {
    ///     "name": "VAR_2",
    ///     "value": "$(VAR_1) bar"
    ///   }
    /// ]
    /// ```
    ///
    /// If you switch the order of the variables in the example, then the expansion
    /// does not occur.
    ///
    /// This field corresponds to the `env` field of the Kubernetes Containers
    /// [v1 core API](https://tinyurl.com/k8s-io-api/v1.18/#container-v1-core).
    #[prost(message, repeated, tag = "4")]
    pub env: ::prost::alloc::vec::Vec<EnvVar>,
    /// Immutable. List of ports to expose from the container. AI Platform sends any
    /// prediction requests that it receives to the first port on this list. AI
    /// Platform also sends
    /// [liveness and health checks](https://tinyurl.com/cust-cont-reqs#health)
    /// to this port.
    ///
    /// If you do not specify this field, it defaults to following value:
    ///
    /// ```json
    /// [
    ///   {
    ///     "containerPort": 8080
    ///   }
    /// ]
    /// ```
    ///
    /// AI Platform does not use ports other than the first one listed. This field
    /// corresponds to the `ports` field of the Kubernetes Containers
    /// [v1 core API](https://tinyurl.com/k8s-io-api/v1.18/#container-v1-core).
    #[prost(message, repeated, tag = "5")]
    pub ports: ::prost::alloc::vec::Vec<Port>,
    /// Immutable. HTTP path on the container to send prediction requests to. AI Platform
    /// forwards requests sent using
    /// [projects.locations.endpoints.predict][google.cloud.aiplatform.v1beta1.PredictionService.Predict] to this
    /// path on the container's IP address and port. AI Platform then returns the
    /// container's response in the API response.
    ///
    /// For example, if you set this field to `/foo`, then when AI Platform
    /// receives a prediction request, it forwards the request body in a POST
    /// request to the `/foo` path on the port of your container specified by the
    /// first value of this `ModelContainerSpec`'s
    /// [ports][google.cloud.aiplatform.v1beta1.ModelContainerSpec.ports] field.
    ///
    /// If you don't specify this field, it defaults to the following value when
    /// you [deploy this Model to an Endpoint][google.cloud.aiplatform.v1beta1.EndpointService.DeployModel]:
    /// <code>/v1/endpoints/<var>ENDPOINT</var>/deployedModels/<var>DEPLOYED_MODEL</var>:predict</code>
    /// The placeholders in this value are replaced as follows:
    ///
    /// * <var>ENDPOINT</var>: The last segment (following `endpoints/`)of the
    ///   Endpoint.name][] field of the Endpoint where this Model has been
    ///   deployed. (AI Platform makes this value available to your container code
    ///   as the
    ///  [`AIP_ENDPOINT_ID`](https://tinyurl.com/cust-cont-reqs#aip-variables)
    ///  environment variable.)
    ///
    /// * <var>DEPLOYED_MODEL</var>: [DeployedModel.id][google.cloud.aiplatform.v1beta1.DeployedModel.id] of the `DeployedModel`.
    ///   (AI Platform makes this value available to your container code
    ///   as the [`AIP_DEPLOYED_MODEL_ID` environment
    ///   variable](https://tinyurl.com/cust-cont-reqs#aip-variables).)
    #[prost(string, tag = "6")]
    pub predict_route: ::prost::alloc::string::String,
    /// Immutable. HTTP path on the container to send health checks to. AI Platform
    /// intermittently sends GET requests to this path on the container's IP
    /// address and port to check that the container is healthy. Read more about
    /// [health
    /// checks](https://tinyurl.com/cust-cont-reqs#checks).
    ///
    /// For example, if you set this field to `/bar`, then AI Platform
    /// intermittently sends a GET request to the `/bar` path on the port of your
    /// container specified by the first value of this `ModelContainerSpec`'s
    /// [ports][google.cloud.aiplatform.v1beta1.ModelContainerSpec.ports] field.
    ///
    /// If you don't specify this field, it defaults to the following value when
    /// you [deploy this Model to an Endpoint][google.cloud.aiplatform.v1beta1.EndpointService.DeployModel]:
    /// <code>/v1/endpoints/<var>ENDPOINT</var>/deployedModels/<var>DEPLOYED_MODEL</var>:predict</code>
    /// The placeholders in this value are replaced as follows:
    ///
    /// * <var>ENDPOINT</var>: The last segment (following `endpoints/`)of the
    ///   Endpoint.name][] field of the Endpoint where this Model has been
    ///   deployed. (AI Platform makes this value available to your container code
    ///   as the
    ///   [`AIP_ENDPOINT_ID`](https://tinyurl.com/cust-cont-reqs#aip-variables)
    ///   environment variable.)
    ///
    /// * <var>DEPLOYED_MODEL</var>: [DeployedModel.id][google.cloud.aiplatform.v1beta1.DeployedModel.id] of the `DeployedModel`.
    ///   (AI Platform makes this value available to your container code as the
    /// [`AIP_DEPLOYED_MODEL_ID`](https://tinyurl.com/cust-cont-reqs#aip-variables)
    ///   environment variable.)
    #[prost(string, tag = "7")]
    pub health_route: ::prost::alloc::string::String,
}
/// Represents a network port in a container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Port {
    /// The number of the port to expose on the pod's IP address.
    /// Must be a valid port number, between 1 and 65535 inclusive.
    #[prost(int32, tag = "3")]
    pub container_port: i32,
}
/// Describes the state of a pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PipelineState {
    /// The pipeline state is unspecified.
    Unspecified = 0,
    /// The pipeline has been just created or resumed and processing has not yet
    /// begun.
    Queued = 1,
    /// The service is preparing to run the pipeline.
    Pending = 2,
    /// The pipeline is in progress.
    Running = 3,
    /// The pipeline completed successfully.
    Succeeded = 4,
    /// The pipeline failed.
    Failed = 5,
    /// The pipeline is being cancelled. From this state the pipeline may only go
    /// to either PIPELINE_STATE_SUCCEEDED, PIPELINE_STATE_FAILED or
    /// PIPELINE_STATE_CANCELLED.
    Cancelling = 6,
    /// The pipeline has been cancelled.
    Cancelled = 7,
    /// The pipeline has been stopped, and can be resumed.
    Paused = 8,
}
/// The TrainingPipeline orchestrates tasks associated with training a Model. It
/// always executes the training task, and optionally may also
/// export data from AI Platform's Dataset which becomes the training input,
/// [upload][google.cloud.aiplatform.v1beta1.ModelService.UploadModel] the Model to AI Platform, and evaluate the
/// Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainingPipeline {
    /// Output only. Resource name of the TrainingPipeline.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The user-defined name of this TrainingPipeline.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Specifies AI Platform owned input data that may be used for training the
    /// Model. The TrainingPipeline's [training_task_definition][google.cloud.aiplatform.v1beta1.TrainingPipeline.training_task_definition] should make
    /// clear whether this config is used and if there are any special requirements
    /// on how it should be filled. If nothing about this config is mentioned in
    /// the [training_task_definition][google.cloud.aiplatform.v1beta1.TrainingPipeline.training_task_definition], then it should be assumed that the
    /// TrainingPipeline does not depend on this configuration.
    #[prost(message, optional, tag = "3")]
    pub input_data_config: ::core::option::Option<InputDataConfig>,
    /// Required. A Google Cloud Storage path to the YAML file that defines the training task
    /// which is responsible for producing the model artifact, and may also include
    /// additional auxiliary work.
    /// The definition files that can be used here are found in
    /// gs://google-cloud-aiplatform/schema/trainingjob/definition/.
    /// Note: The URI given on output will be immutable and probably different,
    /// including the URI scheme, than the one given on input. The output URI will
    /// point to a location where the user only has a read access.
    #[prost(string, tag = "4")]
    pub training_task_definition: ::prost::alloc::string::String,
    /// Required. The training task's parameter(s), as specified in the
    /// [training_task_definition][google.cloud.aiplatform.v1beta1.TrainingPipeline.training_task_definition]'s `inputs`.
    #[prost(message, optional, tag = "5")]
    pub training_task_inputs: ::core::option::Option<::prost_types::Value>,
    /// Output only. The metadata information as specified in the [training_task_definition][google.cloud.aiplatform.v1beta1.TrainingPipeline.training_task_definition]'s
    /// `metadata`. This metadata is an auxiliary runtime and final information
    /// about the training task. While the pipeline is running this information is
    /// populated only at a best effort basis. Only present if the
    /// pipeline's [training_task_definition][google.cloud.aiplatform.v1beta1.TrainingPipeline.training_task_definition] contains `metadata` object.
    #[prost(message, optional, tag = "6")]
    pub training_task_metadata: ::core::option::Option<::prost_types::Value>,
    /// Describes the Model that may be uploaded (via [ModelService.UploadModel][google.cloud.aiplatform.v1beta1.ModelService.UploadModel])
    /// by this TrainingPipeline. The TrainingPipeline's
    /// [training_task_definition][google.cloud.aiplatform.v1beta1.TrainingPipeline.training_task_definition] should make clear whether this Model
    /// description should be populated, and if there are any special requirements
    /// regarding how it should be filled. If nothing is mentioned in the
    /// [training_task_definition][google.cloud.aiplatform.v1beta1.TrainingPipeline.training_task_definition], then it should be assumed that this field
    /// should not be filled and the training task either uploads the Model without
    /// a need of this information, or that training task does not support
    /// uploading a Model as part of the pipeline.
    /// When the Pipeline's state becomes `PIPELINE_STATE_SUCCEEDED` and
    /// the trained Model had been uploaded into AI Platform, then the
    /// model_to_upload's resource [name][google.cloud.aiplatform.v1beta1.Model.name] is populated. The Model
    /// is always uploaded into the Project and Location in which this pipeline
    /// is.
    #[prost(message, optional, tag = "7")]
    pub model_to_upload: ::core::option::Option<Model>,
    /// Output only. The detailed state of the pipeline.
    #[prost(enumeration = "PipelineState", tag = "9")]
    pub state: i32,
    /// Output only. Only populated when the pipeline's state is `PIPELINE_STATE_FAILED` or
    /// `PIPELINE_STATE_CANCELLED`.
    #[prost(message, optional, tag = "10")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. Time when the TrainingPipeline was created.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the TrainingPipeline for the first time entered the
    /// `PIPELINE_STATE_RUNNING` state.
    #[prost(message, optional, tag = "12")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the TrainingPipeline entered any of the following states:
    /// `PIPELINE_STATE_SUCCEEDED`, `PIPELINE_STATE_FAILED`,
    /// `PIPELINE_STATE_CANCELLED`.
    #[prost(message, optional, tag = "13")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the TrainingPipeline was most recently updated.
    #[prost(message, optional, tag = "14")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The labels with user-defined metadata to organize TrainingPipelines.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    ///
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    #[prost(map = "string, string", tag = "15")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Customer-managed encryption key spec for a TrainingPipeline. If set, this
    /// TrainingPipeline will be secured by this key.
    ///
    /// Note: Model trained by this TrainingPipeline is also secured by this key if
    /// [model_to_upload][google.cloud.aiplatform.v1beta1.TrainingPipeline.encryption_spec] is not set separately.
    #[prost(message, optional, tag = "18")]
    pub encryption_spec: ::core::option::Option<EncryptionSpec>,
}
/// Specifies AI Platform owned input data to be used for training, and
/// possibly evaluating, the Model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputDataConfig {
    /// Required. The ID of the Dataset in the same Project and Location which data will be
    /// used to train the Model. The Dataset must use schema compatible with
    /// Model being trained, and what is compatible should be described in the
    /// used TrainingPipeline's [training_task_definition]
    /// [google.cloud.aiplatform.v1beta1.TrainingPipeline.training_task_definition].
    /// For tabular Datasets, all their data is exported to training, to pick
    /// and choose from.
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Applicable only to Datasets that have DataItems and Annotations.
    ///
    /// A filter on Annotations of the Dataset. Only Annotations that both
    /// match this filter and belong to DataItems not ignored by the split method
    /// are used in respectively training, validation or test role, depending on
    /// the role of the DataItem they are on (for the auto-assigned that role is
    /// decided by AI Platform). A filter with same syntax as the one used in
    /// [ListAnnotations][google.cloud.aiplatform.v1beta1.DatasetService.ListAnnotations] may be used, but note
    /// here it filters across all Annotations of the Dataset, and not just within
    /// a single DataItem.
    #[prost(string, tag = "6")]
    pub annotations_filter: ::prost::alloc::string::String,
    /// Applicable only to custom training with Datasets that have DataItems and
    /// Annotations.
    ///
    /// Cloud Storage URI that points to a YAML file describing the annotation
    /// schema. The schema is defined as an OpenAPI 3.0.2
    /// [Schema Object](https://tinyurl.com/y538mdwt#schema-object).
    /// The schema files that can be used here are found in
    /// gs://google-cloud-aiplatform/schema/dataset/annotation/ , note that the
    /// chosen schema must be consistent with
    /// [metadata][google.cloud.aiplatform.v1beta1.Dataset.metadata_schema_uri] of the Dataset specified by
    /// [dataset_id][google.cloud.aiplatform.v1beta1.InputDataConfig.dataset_id].
    ///
    /// Only Annotations that both match this schema and belong to DataItems not
    /// ignored by the split method are used in respectively training, validation
    /// or test role, depending on the role of the DataItem they are on.
    ///
    /// When used in conjunction with [annotations_filter][google.cloud.aiplatform.v1beta1.InputDataConfig.annotations_filter], the Annotations used
    /// for training are filtered by both [annotations_filter][google.cloud.aiplatform.v1beta1.InputDataConfig.annotations_filter] and
    /// [annotation_schema_uri][google.cloud.aiplatform.v1beta1.InputDataConfig.annotation_schema_uri].
    #[prost(string, tag = "9")]
    pub annotation_schema_uri: ::prost::alloc::string::String,
    /// The instructions how the input data should be split between the
    /// training, validation and test sets.
    /// If no split type is provided, the [fraction_split][google.cloud.aiplatform.v1beta1.InputDataConfig.fraction_split] is used by default.
    #[prost(oneof = "input_data_config::Split", tags = "2, 3, 4, 5")]
    pub split: ::core::option::Option<input_data_config::Split>,
    /// Only applicable to Custom and Hyperparameter Tuning TrainingPipelines.
    ///
    /// The destination of the training data to be written to.
    ///
    /// Supported destination file formats:
    ///   * For non-tabular data: "jsonl".
    ///   * For tabular data: "csv" and "bigquery".
    ///
    /// The following AI Platform environment variables are passed to containers
    /// or python modules of the training task when this field is set:
    ///
    /// * AIP_DATA_FORMAT : Exported data format.
    /// * AIP_TRAINING_DATA_URI : Sharded exported training data uris.
    /// * AIP_VALIDATION_DATA_URI : Sharded exported validation data uris.
    /// * AIP_TEST_DATA_URI : Sharded exported test data uris.
    #[prost(oneof = "input_data_config::Destination", tags = "8, 10")]
    pub destination: ::core::option::Option<input_data_config::Destination>,
}
/// Nested message and enum types in `InputDataConfig`.
pub mod input_data_config {
    /// The instructions how the input data should be split between the
    /// training, validation and test sets.
    /// If no split type is provided, the [fraction_split][google.cloud.aiplatform.v1beta1.InputDataConfig.fraction_split] is used by default.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Split {
        /// Split based on fractions defining the size of each set.
        #[prost(message, tag = "2")]
        FractionSplit(super::FractionSplit),
        /// Split based on the provided filters for each set.
        #[prost(message, tag = "3")]
        FilterSplit(super::FilterSplit),
        /// Supported only for tabular Datasets.
        ///
        /// Split based on a predefined key.
        #[prost(message, tag = "4")]
        PredefinedSplit(super::PredefinedSplit),
        /// Supported only for tabular Datasets.
        ///
        /// Split based on the timestamp of the input data pieces.
        #[prost(message, tag = "5")]
        TimestampSplit(super::TimestampSplit),
    }
    /// Only applicable to Custom and Hyperparameter Tuning TrainingPipelines.
    ///
    /// The destination of the training data to be written to.
    ///
    /// Supported destination file formats:
    ///   * For non-tabular data: "jsonl".
    ///   * For tabular data: "csv" and "bigquery".
    ///
    /// The following AI Platform environment variables are passed to containers
    /// or python modules of the training task when this field is set:
    ///
    /// * AIP_DATA_FORMAT : Exported data format.
    /// * AIP_TRAINING_DATA_URI : Sharded exported training data uris.
    /// * AIP_VALIDATION_DATA_URI : Sharded exported validation data uris.
    /// * AIP_TEST_DATA_URI : Sharded exported test data uris.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// The Cloud Storage location where the training data is to be
        /// written to. In the given directory a new directory is created with
        /// name:
        /// `dataset-<dataset-id>-<annotation-type>-<timestamp-of-training-call>`
        /// where timestamp is in YYYY-MM-DDThh:mm:ss.sssZ ISO-8601 format.
        /// All training input data is written into that directory.
        ///
        /// The AI Platform environment variables representing Cloud Storage
        /// data URIs are represented in the Cloud Storage wildcard
        /// format to support sharded data. e.g.: "gs://.../training-*.jsonl"
        ///
        /// * AIP_DATA_FORMAT = "jsonl" for non-tabular data, "csv" for tabular data
        /// * AIP_TRAINING_DATA_URI =
        /// "gcs_destination/dataset-<dataset-id>-<annotation-type>-<time>/training-*.${AIP_DATA_FORMAT}"
        ///
        /// * AIP_VALIDATION_DATA_URI =
        /// "gcs_destination/dataset-<dataset-id>-<annotation-type>-<time>/validation-*.${AIP_DATA_FORMAT}"
        ///
        /// * AIP_TEST_DATA_URI =
        /// "gcs_destination/dataset-<dataset-id>-<annotation-type>-<time>/test-*.${AIP_DATA_FORMAT}"
        #[prost(message, tag = "8")]
        GcsDestination(super::GcsDestination),
        /// Only applicable to custom training with tabular Dataset with BigQuery
        /// source.
        ///
        /// The BigQuery project location where the training data is to be written
        /// to. In the given project a new dataset is created with name
        /// `dataset_<dataset-id>_<annotation-type>_<timestamp-of-training-call>`
        /// where timestamp is in YYYY_MM_DDThh_mm_ss_sssZ format. All training
        /// input data is written into that dataset. In the dataset three
        /// tables are created, `training`, `validation` and `test`.
        ///
        /// * AIP_DATA_FORMAT = "bigquery".
        /// * AIP_TRAINING_DATA_URI  =
        /// "bigquery_destination.dataset_<dataset-id>_<annotation-type>_<time>.training"
        ///
        /// * AIP_VALIDATION_DATA_URI =
        /// "bigquery_destination.dataset_<dataset-id>_<annotation-type>_<time>.validation"
        ///
        /// * AIP_TEST_DATA_URI =
        /// "bigquery_destination.dataset_<dataset-id>_<annotation-type>_<time>.test"
        #[prost(message, tag = "10")]
        BigqueryDestination(super::BigQueryDestination),
    }
}
/// Assigns the input data to training, validation, and test sets as per the
/// given fractions. Any of `training_fraction`, `validation_fraction` and
/// `test_fraction` may optionally be provided, they must sum to up to 1. If the
/// provided ones sum to less than 1, the remainder is assigned to sets as
/// decided by AI Platform. If none of the fractions are set, by default roughly
/// 80% of data is used for training, 10% for validation, and 10% for test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FractionSplit {
    /// The fraction of the input data that is to be used to train the Model.
    #[prost(double, tag = "1")]
    pub training_fraction: f64,
    /// The fraction of the input data that is to be used to validate the Model.
    #[prost(double, tag = "2")]
    pub validation_fraction: f64,
    /// The fraction of the input data that is to be used to evaluate the Model.
    #[prost(double, tag = "3")]
    pub test_fraction: f64,
}
/// Assigns input data to training, validation, and test sets based on the given
/// filters, data pieces not matched by any filter are ignored. Currently only
/// supported for Datasets containing DataItems.
/// If any of the filters in this message are to match nothing, then they can be
/// set as '-' (the minus sign).
///
/// Supported only for unstructured Datasets.
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterSplit {
    /// Required. A filter on DataItems of the Dataset. DataItems that match
    /// this filter are used to train the Model. A filter with same syntax
    /// as the one used in [DatasetService.ListDataItems][google.cloud.aiplatform.v1beta1.DatasetService.ListDataItems] may be used. If a
    /// single DataItem is matched by more than one of the FilterSplit filters,
    /// then it is assigned to the first set that applies to it in the
    /// training, validation, test order.
    #[prost(string, tag = "1")]
    pub training_filter: ::prost::alloc::string::String,
    /// Required. A filter on DataItems of the Dataset. DataItems that match
    /// this filter are used to validate the Model. A filter with same syntax
    /// as the one used in [DatasetService.ListDataItems][google.cloud.aiplatform.v1beta1.DatasetService.ListDataItems] may be used. If a
    /// single DataItem is matched by more than one of the FilterSplit filters,
    /// then it is assigned to the first set that applies to it in the
    /// training, validation, test order.
    #[prost(string, tag = "2")]
    pub validation_filter: ::prost::alloc::string::String,
    /// Required. A filter on DataItems of the Dataset. DataItems that match
    /// this filter are used to test the Model. A filter with same syntax
    /// as the one used in [DatasetService.ListDataItems][google.cloud.aiplatform.v1beta1.DatasetService.ListDataItems] may be used. If a
    /// single DataItem is matched by more than one of the FilterSplit filters,
    /// then it is assigned to the first set that applies to it in the
    /// training, validation, test order.
    #[prost(string, tag = "3")]
    pub test_filter: ::prost::alloc::string::String,
}
/// Assigns input data to training, validation, and test sets based on the
/// value of a provided key.
///
/// Supported only for tabular Datasets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredefinedSplit {
    /// Required. The key is a name of one of the Dataset's data columns.
    /// The value of the key (either the label's value or value in the column)
    /// must be one of {`training`, `validation`, `test`}, and it defines to which
    /// set the given piece of data is assigned. If for a piece of data the key
    /// is not present or has an invalid value, that piece is ignored by the
    /// pipeline.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// Assigns input data to training, validation, and test sets based on a
/// provided timestamps. The youngest data pieces are assigned to training set,
/// next to validation set, and the oldest to the test set.
///
/// Supported only for tabular Datasets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampSplit {
    /// The fraction of the input data that is to be used to train the Model.
    #[prost(double, tag = "1")]
    pub training_fraction: f64,
    /// The fraction of the input data that is to be used to validate the Model.
    #[prost(double, tag = "2")]
    pub validation_fraction: f64,
    /// The fraction of the input data that is to be used to evaluate the Model.
    #[prost(double, tag = "3")]
    pub test_fraction: f64,
    /// Required. The key is a name of one of the Dataset's data columns.
    /// The values of the key (the values in the column) must be in RFC 3339
    /// `date-time` format, where `time-offset` = `"Z"`
    /// (e.g. 1985-04-12T23:20:50.52Z). If for a piece of data the key is not
    /// present or has an invalid value, that piece is ignored by the pipeline.
    #[prost(string, tag = "4")]
    pub key: ::prost::alloc::string::String,
}
/// Request message for [DatasetService.CreateDataset][google.cloud.aiplatform.v1beta1.DatasetService.CreateDataset].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    /// Required. The resource name of the Location to create the Dataset in.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Dataset to create.
    #[prost(message, optional, tag = "2")]
    pub dataset: ::core::option::Option<Dataset>,
}
/// Runtime operation information for [DatasetService.CreateDataset][google.cloud.aiplatform.v1beta1.DatasetService.CreateDataset].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetOperationMetadata {
    /// The operation generic information.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
}
/// Request message for [DatasetService.GetDataset][google.cloud.aiplatform.v1beta1.DatasetService.GetDataset].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatasetRequest {
    /// Required. The name of the Dataset resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for [DatasetService.UpdateDataset][google.cloud.aiplatform.v1beta1.DatasetService.UpdateDataset].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatasetRequest {
    /// Required. The Dataset which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub dataset: ::core::option::Option<Dataset>,
    /// Required. The update mask applies to the resource.
    /// For the `FieldMask` definition, see
    /// [FieldMask](https://tinyurl.com/protobufs/google.protobuf#fieldmask).
    /// Updatable fields:
    ///
    ///   * `display_name`
    ///   * `description`
    ///   * `labels`
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for [DatasetService.ListDatasets][google.cloud.aiplatform.v1beta1.DatasetService.ListDatasets].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsRequest {
    /// Required. The name of the Dataset's parent resource.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// An expression for filtering the results of the request. For field names
    /// both snake_case and camelCase are supported.
    ///
    ///   * `display_name`: supports = and !=
    ///   * `metadata_schema_uri`: supports = and !=
    ///   * `labels` supports general map functions that is:
    ///     * `labels.key=value` - key:value equality
    ///     * `labels.key:* or labels:key - key existence
    ///     * A key including a space must be quoted. `labels."a key"`.
    ///
    /// Some examples:
    ///   * `displayName="myDisplayName"`
    ///   * `labels.myKey="myValue"`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// A comma-separated list of fields to order by, sorted in ascending order.
    /// Use "desc" after a field name for descending.
    /// Supported fields:
    ///   * `display_name`
    ///   * `create_time`
    ///   * `update_time`
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for [DatasetService.ListDatasets][google.cloud.aiplatform.v1beta1.DatasetService.ListDatasets].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsResponse {
    /// A list of Datasets that matches the specified filter in the request.
    #[prost(message, repeated, tag = "1")]
    pub datasets: ::prost::alloc::vec::Vec<Dataset>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [DatasetService.DeleteDataset][google.cloud.aiplatform.v1beta1.DatasetService.DeleteDataset].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetRequest {
    /// Required. The resource name of the Dataset to delete.
    /// Format:
    /// `projects/{project}/locations/{location}/datasets/{dataset}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [DatasetService.ImportData][google.cloud.aiplatform.v1beta1.DatasetService.ImportData].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataRequest {
    /// Required. The name of the Dataset resource.
    /// Format:
    /// `projects/{project}/locations/{location}/datasets/{dataset}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired input locations. The contents of all input locations will be
    /// imported in one batch.
    #[prost(message, repeated, tag = "2")]
    pub import_configs: ::prost::alloc::vec::Vec<ImportDataConfig>,
}
/// Response message for [DatasetService.ImportData][google.cloud.aiplatform.v1beta1.DatasetService.ImportData].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataResponse {}
/// Runtime operation information for [DatasetService.ImportData][google.cloud.aiplatform.v1beta1.DatasetService.ImportData].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportDataOperationMetadata {
    /// The common part of the operation metadata.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
}
/// Request message for [DatasetService.ExportData][google.cloud.aiplatform.v1beta1.DatasetService.ExportData].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataRequest {
    /// Required. The name of the Dataset resource.
    /// Format:
    /// `projects/{project}/locations/{location}/datasets/{dataset}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired output location.
    #[prost(message, optional, tag = "2")]
    pub export_config: ::core::option::Option<ExportDataConfig>,
}
/// Response message for [DatasetService.ExportData][google.cloud.aiplatform.v1beta1.DatasetService.ExportData].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataResponse {
    /// All of the files that are exported in this export operation.
    #[prost(string, repeated, tag = "1")]
    pub exported_files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Runtime operation information for [DatasetService.ExportData][google.cloud.aiplatform.v1beta1.DatasetService.ExportData].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportDataOperationMetadata {
    /// The common part of the operation metadata.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
    /// A Google Cloud Storage directory which path ends with '/'. The exported
    /// data is stored in the directory.
    #[prost(string, tag = "2")]
    pub gcs_output_directory: ::prost::alloc::string::String,
}
/// Request message for [DatasetService.ListDataItems][google.cloud.aiplatform.v1beta1.DatasetService.ListDataItems].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataItemsRequest {
    /// Required. The resource name of the Dataset to list DataItems from.
    /// Format:
    /// `projects/{project}/locations/{location}/datasets/{dataset}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard list filter.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// A comma-separated list of fields to order by, sorted in ascending order.
    /// Use "desc" after a field name for descending.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for [DatasetService.ListDataItems][google.cloud.aiplatform.v1beta1.DatasetService.ListDataItems].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataItemsResponse {
    /// A list of DataItems that matches the specified filter in the request.
    #[prost(message, repeated, tag = "1")]
    pub data_items: ::prost::alloc::vec::Vec<DataItem>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [DatasetService.GetAnnotationSpec][google.cloud.aiplatform.v1beta1.DatasetService.GetAnnotationSpec].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAnnotationSpecRequest {
    /// Required. The name of the AnnotationSpec resource.
    /// Format:
    /// `projects/{project}/locations/{location}/datasets/{dataset}/annotationSpecs/{annotation_spec}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "2")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for [DatasetService.ListAnnotations][google.cloud.aiplatform.v1beta1.DatasetService.ListAnnotations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotationsRequest {
    /// Required. The resource name of the DataItem to list Annotations from.
    /// Format:
    /// `projects/{project}/locations/{location}/datasets/{dataset}/dataItems/{data_item}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard list filter.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// A comma-separated list of fields to order by, sorted in ascending order.
    /// Use "desc" after a field name for descending.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for [DatasetService.ListAnnotations][google.cloud.aiplatform.v1beta1.DatasetService.ListAnnotations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAnnotationsResponse {
    /// A list of Annotations that matches the specified filter in the request.
    #[prost(message, repeated, tag = "1")]
    pub annotations: ::prost::alloc::vec::Vec<Annotation>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod dataset_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct DatasetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DatasetServiceClient<T>
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
        #[doc = " Creates a Dataset."]
        pub async fn create_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatasetRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.DatasetService/CreateDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a Dataset."]
        pub async fn get_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.DatasetService/GetDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a Dataset."]
        pub async fn update_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatasetRequest>,
        ) -> Result<tonic::Response<super::Dataset>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.DatasetService/UpdateDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Datasets in a Location."]
        pub async fn list_datasets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatasetsRequest>,
        ) -> Result<tonic::Response<super::ListDatasetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.DatasetService/ListDatasets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a Dataset."]
        pub async fn delete_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatasetRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.DatasetService/DeleteDataset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Imports data into a Dataset."]
        pub async fn import_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportDataRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.DatasetService/ImportData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports data from a Dataset."]
        pub async fn export_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportDataRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.DatasetService/ExportData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists DataItems in a Dataset."]
        pub async fn list_data_items(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataItemsRequest>,
        ) -> Result<tonic::Response<super::ListDataItemsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.DatasetService/ListDataItems",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets an AnnotationSpec."]
        pub async fn get_annotation_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAnnotationSpecRequest>,
        ) -> Result<tonic::Response<super::AnnotationSpec>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.DatasetService/GetAnnotationSpec",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Annotations belongs to a dataitem"]
        pub async fn list_annotations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAnnotationsRequest>,
        ) -> Result<tonic::Response<super::ListAnnotationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.DatasetService/ListAnnotations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DatasetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DatasetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DatasetServiceClient {{ ... }}")
        }
    }
}
/// Models are deployed into it, and afterwards Endpoint is called to obtain
/// predictions and explanations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    /// Output only. The resource name of the Endpoint.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the Endpoint.
    /// The name can be up to 128 characters long and can be consist of any UTF-8
    /// characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The description of the Endpoint.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The models deployed in this Endpoint.
    /// To add or remove DeployedModels use [EndpointService.DeployModel][google.cloud.aiplatform.v1beta1.EndpointService.DeployModel] and
    /// [EndpointService.UndeployModel][google.cloud.aiplatform.v1beta1.EndpointService.UndeployModel] respectively.
    #[prost(message, repeated, tag = "4")]
    pub deployed_models: ::prost::alloc::vec::Vec<DeployedModel>,
    /// A map from a DeployedModel's ID to the percentage of this Endpoint's
    /// traffic that should be forwarded to that DeployedModel.
    ///
    /// If a DeployedModel's ID is not listed in this map, then it receives no
    /// traffic.
    ///
    /// The traffic percentage values must add up to 100, or map must be empty if
    /// the Endpoint is to not accept any traffic at a moment.
    #[prost(map = "string, int32", tag = "5")]
    pub traffic_split: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    /// Used to perform consistent read-modify-write updates. If not set, a blind
    /// "overwrite" update happens.
    #[prost(string, tag = "6")]
    pub etag: ::prost::alloc::string::String,
    /// The labels with user-defined metadata to organize your Endpoints.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    ///
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    #[prost(map = "string, string", tag = "7")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Timestamp when this Endpoint was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when this Endpoint was last updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Customer-managed encryption key spec for an Endpoint. If set, this
    /// Endpoint and all sub-resources of this Endpoint will be secured by
    /// this key.
    #[prost(message, optional, tag = "10")]
    pub encryption_spec: ::core::option::Option<EncryptionSpec>,
}
/// A deployment of a Model. Endpoints contain one or more DeployedModels.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployedModel {
    /// Output only. The ID of the DeployedModel.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Required. The name of the Model that this is the deployment of. Note that the Model
    /// may be in a different location than the DeployedModel's Endpoint.
    #[prost(string, tag = "2")]
    pub model: ::prost::alloc::string::String,
    /// The display name of the DeployedModel. If not provided upon creation,
    /// the Model's display_name is used.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Timestamp when the DeployedModel was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Explanation configuration for this DeployedModel.
    ///
    /// When deploying a Model using [EndpointService.DeployModel][google.cloud.aiplatform.v1beta1.EndpointService.DeployModel], this value
    /// overrides the value of [Model.explanation_spec][google.cloud.aiplatform.v1beta1.Model.explanation_spec]. All fields of
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.DeployedModel.explanation_spec] are optional in the request. If a field of
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.DeployedModel.explanation_spec] is not populated, the value of the same field of
    /// [Model.explanation_spec][google.cloud.aiplatform.v1beta1.Model.explanation_spec] is inherited. If the corresponding
    /// [Model.explanation_spec][google.cloud.aiplatform.v1beta1.Model.explanation_spec] is not populated, all fields of the
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.DeployedModel.explanation_spec] will be used for the explanation configuration.
    #[prost(message, optional, tag = "9")]
    pub explanation_spec: ::core::option::Option<ExplanationSpec>,
    /// The service account that the DeployedModel's container runs as. Specify the
    /// email address of the service account. If this service account is not
    /// specified, the container runs as a service account that doesn't have access
    /// to the resource project.
    ///
    /// Users deploying the Model must have the `iam.serviceAccounts.actAs`
    /// permission on this service account.
    #[prost(string, tag = "11")]
    pub service_account: ::prost::alloc::string::String,
    /// If true, the container of the DeployedModel instances will send `stderr`
    /// and `stdout` streams to Stackdriver Logging.
    ///
    /// Only supported for custom-trained Models and AutoML Tabular Models.
    #[prost(bool, tag = "12")]
    pub enable_container_logging: bool,
    /// These logs are like standard server access logs, containing
    /// information like timestamp and latency for each prediction request.
    ///
    /// Note that Stackdriver logs may incur a cost, especially if your project
    /// receives prediction requests at a high queries per second rate (QPS).
    /// Estimate your costs before enabling this option.
    #[prost(bool, tag = "13")]
    pub enable_access_logging: bool,
    /// The prediction (for example, the machine) resources that the DeployedModel
    /// uses. The user is billed for the resources (at least their minimal amount)
    /// even if the DeployedModel receives no traffic.
    /// Not all Models support all resources types. See
    /// [Model.supported_deployment_resources_types][google.cloud.aiplatform.v1beta1.Model.supported_deployment_resources_types].
    #[prost(oneof = "deployed_model::PredictionResources", tags = "7, 8")]
    pub prediction_resources: ::core::option::Option<deployed_model::PredictionResources>,
}
/// Nested message and enum types in `DeployedModel`.
pub mod deployed_model {
    /// The prediction (for example, the machine) resources that the DeployedModel
    /// uses. The user is billed for the resources (at least their minimal amount)
    /// even if the DeployedModel receives no traffic.
    /// Not all Models support all resources types. See
    /// [Model.supported_deployment_resources_types][google.cloud.aiplatform.v1beta1.Model.supported_deployment_resources_types].
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PredictionResources {
        /// A description of resources that are dedicated to the DeployedModel, and
        /// that need a higher degree of manual configuration.
        #[prost(message, tag = "7")]
        DedicatedResources(super::DedicatedResources),
        /// A description of resources that to large degree are decided by AI
        /// Platform, and require only a modest additional configuration.
        #[prost(message, tag = "8")]
        AutomaticResources(super::AutomaticResources),
    }
}
/// Request message for [EndpointService.CreateEndpoint][google.cloud.aiplatform.v1beta1.EndpointService.CreateEndpoint].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEndpointRequest {
    /// Required. The resource name of the Location to create the Endpoint in.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Endpoint to create.
    #[prost(message, optional, tag = "2")]
    pub endpoint: ::core::option::Option<Endpoint>,
}
/// Runtime operation information for [EndpointService.CreateEndpoint][google.cloud.aiplatform.v1beta1.EndpointService.CreateEndpoint].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEndpointOperationMetadata {
    /// The operation generic information.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
}
/// Request message for [EndpointService.GetEndpoint][google.cloud.aiplatform.v1beta1.EndpointService.GetEndpoint]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndpointRequest {
    /// Required. The name of the Endpoint resource.
    /// Format:
    /// `projects/{project}/locations/{location}/endpoints/{endpoint}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [EndpointService.ListEndpoints][google.cloud.aiplatform.v1beta1.EndpointService.ListEndpoints].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointsRequest {
    /// Required. The resource name of the Location from which to list the Endpoints.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. An expression for filtering the results of the request. For field names
    /// both snake_case and camelCase are supported.
    ///
    ///   * `endpoint` supports = and !=. `endpoint` represents the Endpoint ID,
    ///     i.e. the last segment of the Endpoint's [resource name][google.cloud.aiplatform.v1beta1.Endpoint.name].
    ///   * `display_name` supports = and, !=
    ///   * `labels` supports general map functions that is:
    ///     * `labels.key=value` - key:value equality
    ///     * `labels.key:* or labels:key - key existence
    ///     * A key including a space must be quoted. `labels."a key"`.
    ///
    /// Some examples:
    ///   * `endpoint=1`
    ///   * `displayName="myDisplayName"`
    ///   * `labels.myKey="myValue"`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. The standard list page token.
    /// Typically obtained via
    /// [ListEndpointsResponse.next_page_token][google.cloud.aiplatform.v1beta1.ListEndpointsResponse.next_page_token] of the previous
    /// [EndpointService.ListEndpoints][google.cloud.aiplatform.v1beta1.EndpointService.ListEndpoints] call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for [EndpointService.ListEndpoints][google.cloud.aiplatform.v1beta1.EndpointService.ListEndpoints].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEndpointsResponse {
    /// List of Endpoints in the requested page.
    #[prost(message, repeated, tag = "1")]
    pub endpoints: ::prost::alloc::vec::Vec<Endpoint>,
    /// A token to retrieve the next page of results.
    /// Pass to [ListEndpointsRequest.page_token][google.cloud.aiplatform.v1beta1.ListEndpointsRequest.page_token] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [EndpointService.UpdateEndpoint][google.cloud.aiplatform.v1beta1.EndpointService.UpdateEndpoint].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEndpointRequest {
    /// Required. The Endpoint which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub endpoint: ::core::option::Option<Endpoint>,
    /// Required. The update mask applies to the resource.
    /// See
    /// [FieldMask](https://tinyurl.com/protobufs/google.protobuf#fieldmask).
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for [EndpointService.DeleteEndpoint][google.cloud.aiplatform.v1beta1.EndpointService.DeleteEndpoint].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEndpointRequest {
    /// Required. The name of the Endpoint resource to be deleted.
    /// Format:
    /// `projects/{project}/locations/{location}/endpoints/{endpoint}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [EndpointService.DeployModel][google.cloud.aiplatform.v1beta1.EndpointService.DeployModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployModelRequest {
    /// Required. The name of the Endpoint resource into which to deploy a Model.
    /// Format:
    /// `projects/{project}/locations/{location}/endpoints/{endpoint}`
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    /// Required. The DeployedModel to be created within the Endpoint. Note that
    /// [Endpoint.traffic_split][google.cloud.aiplatform.v1beta1.Endpoint.traffic_split] must be updated for the DeployedModel to start
    /// receiving traffic, either as part of this call, or via
    /// [EndpointService.UpdateEndpoint][google.cloud.aiplatform.v1beta1.EndpointService.UpdateEndpoint].
    #[prost(message, optional, tag = "2")]
    pub deployed_model: ::core::option::Option<DeployedModel>,
    /// A map from a DeployedModel's ID to the percentage of this Endpoint's
    /// traffic that should be forwarded to that DeployedModel.
    ///
    /// If this field is non-empty, then the Endpoint's
    /// [traffic_split][google.cloud.aiplatform.v1beta1.Endpoint.traffic_split] will be overwritten with it.
    /// To refer to the ID of the just being deployed Model, a "0" should be used,
    /// and the actual ID of the new DeployedModel will be filled in its place by
    /// this method. The traffic percentage values must add up to 100.
    ///
    /// If this field is empty, then the Endpoint's
    /// [traffic_split][google.cloud.aiplatform.v1beta1.Endpoint.traffic_split] is not updated.
    #[prost(map = "string, int32", tag = "3")]
    pub traffic_split: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
/// Response message for [EndpointService.DeployModel][google.cloud.aiplatform.v1beta1.EndpointService.DeployModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployModelResponse {
    /// The DeployedModel that had been deployed in the Endpoint.
    #[prost(message, optional, tag = "1")]
    pub deployed_model: ::core::option::Option<DeployedModel>,
}
/// Runtime operation information for [EndpointService.DeployModel][google.cloud.aiplatform.v1beta1.EndpointService.DeployModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployModelOperationMetadata {
    /// The operation generic information.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
}
/// Request message for [EndpointService.UndeployModel][google.cloud.aiplatform.v1beta1.EndpointService.UndeployModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployModelRequest {
    /// Required. The name of the Endpoint resource from which to undeploy a Model.
    /// Format:
    /// `projects/{project}/locations/{location}/endpoints/{endpoint}`
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    /// Required. The ID of the DeployedModel to be undeployed from the Endpoint.
    #[prost(string, tag = "2")]
    pub deployed_model_id: ::prost::alloc::string::String,
    /// If this field is provided, then the Endpoint's
    /// [traffic_split][google.cloud.aiplatform.v1beta1.Endpoint.traffic_split] will be overwritten with it. If
    /// last DeployedModel is being undeployed from the Endpoint, the
    /// [Endpoint.traffic_split] will always end up empty when this call returns.
    /// A DeployedModel will be successfully undeployed only if it doesn't have
    /// any traffic assigned to it when this method executes, or if this field
    /// unassigns any traffic to it.
    #[prost(map = "string, int32", tag = "3")]
    pub traffic_split: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
/// Response message for [EndpointService.UndeployModel][google.cloud.aiplatform.v1beta1.EndpointService.UndeployModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployModelResponse {}
/// Runtime operation information for [EndpointService.UndeployModel][google.cloud.aiplatform.v1beta1.EndpointService.UndeployModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployModelOperationMetadata {
    /// The operation generic information.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
}
#[doc = r" Generated client implementations."]
pub mod endpoint_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct EndpointServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EndpointServiceClient<T>
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
        #[doc = " Creates an Endpoint."]
        pub async fn create_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEndpointRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.EndpointService/CreateEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets an Endpoint."]
        pub async fn get_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndpointRequest>,
        ) -> Result<tonic::Response<super::Endpoint>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.EndpointService/GetEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Endpoints in a Location."]
        pub async fn list_endpoints(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEndpointsRequest>,
        ) -> Result<tonic::Response<super::ListEndpointsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.EndpointService/ListEndpoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an Endpoint."]
        pub async fn update_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEndpointRequest>,
        ) -> Result<tonic::Response<super::Endpoint>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.EndpointService/UpdateEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an Endpoint."]
        pub async fn delete_endpoint(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEndpointRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.EndpointService/DeleteEndpoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deploys a Model into this Endpoint, creating a DeployedModel within it."]
        pub async fn deploy_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeployModelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.EndpointService/DeployModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Undeploys a Model from an Endpoint, removing a DeployedModel from it, and"]
        #[doc = " freeing all resources it's using."]
        pub async fn undeploy_model(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeployModelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.EndpointService/UndeployModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for EndpointServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for EndpointServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "EndpointServiceClient {{ ... }}")
        }
    }
}
/// A message representing a Study.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Study {
    /// Output only. The name of a study. The study's globally unique identifier.
    /// Format: `projects/{project}/locations/{location}/studies/{study}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Describes the Study, default value is empty string.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Configuration of the Study.
    #[prost(message, optional, tag = "3")]
    pub study_spec: ::core::option::Option<StudySpec>,
    /// Output only. The detailed state of a Study.
    #[prost(enumeration = "study::State", tag = "4")]
    pub state: i32,
    /// Output only. Time at which the study was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A human readable reason why the Study is inactive.
    /// This should be empty if a study is ACTIVE or COMPLETED.
    #[prost(string, tag = "6")]
    pub inactive_reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Study`.
pub mod study {
    /// Describes the Study state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The study state is unspecified.
        Unspecified = 0,
        /// The study is active.
        Active = 1,
        /// The study is stopped due to an internal error.
        Inactive = 2,
        /// The study is done when the service exhausts the parameter search space
        /// or max_trial_count is reached.
        Completed = 3,
    }
}
/// A message representing a Trial. A Trial contains a unique set of Parameters
/// that has been or will be evaluated, along with the objective metrics got by
/// running the Trial.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trial {
    /// Output only. Resource name of the Trial assigned by the service.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The identifier of the Trial assigned by the service.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Output only. The detailed state of the Trial.
    #[prost(enumeration = "trial::State", tag = "3")]
    pub state: i32,
    /// Output only. The parameters of the Trial.
    #[prost(message, repeated, tag = "4")]
    pub parameters: ::prost::alloc::vec::Vec<trial::Parameter>,
    /// Output only. The final measurement containing the objective value.
    #[prost(message, optional, tag = "5")]
    pub final_measurement: ::core::option::Option<Measurement>,
    /// Output only. Time when the Trial was started.
    #[prost(message, optional, tag = "7")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the Trial's status changed to `SUCCEEDED` or `INFEASIBLE`.
    #[prost(message, optional, tag = "8")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The CustomJob name linked to the Trial.
    /// It's set for a HyperparameterTuningJob's Trial.
    #[prost(string, tag = "11")]
    pub custom_job: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Trial`.
pub mod trial {
    /// A message representing a parameter to be tuned.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Parameter {
        /// Output only. The ID of the parameter. The parameter should be defined in
        /// [StudySpec's Parameters][google.cloud.aiplatform.v1beta1.StudySpec.parameters].
        #[prost(string, tag = "1")]
        pub parameter_id: ::prost::alloc::string::String,
        /// Output only. The value of the parameter.
        /// `number_value` will be set if a parameter defined in StudySpec is
        /// in type 'INTEGER', 'DOUBLE' or 'DISCRETE'.
        /// `string_value` will be set if a parameter defined in StudySpec is
        /// in type 'CATEGORICAL'.
        #[prost(message, optional, tag = "2")]
        pub value: ::core::option::Option<::prost_types::Value>,
    }
    /// Describes a Trial state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The Trial state is unspecified.
        Unspecified = 0,
        /// Indicates that a specific Trial has been requested, but it has not yet
        /// been suggested by the service.
        Requested = 1,
        /// Indicates that the Trial has been suggested.
        Active = 2,
        /// Indicates that the Trial should stop according to the service.
        Stopping = 3,
        /// Indicates that the Trial is completed successfully.
        Succeeded = 4,
        /// Indicates that the Trial should not be attempted again.
        /// The service will set a Trial to INFEASIBLE when it's done but missing
        /// the final_measurement.
        Infeasible = 5,
    }
}
/// Represents specification of a Study.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StudySpec {
    /// Required. Metric specs for the Study.
    #[prost(message, repeated, tag = "1")]
    pub metrics: ::prost::alloc::vec::Vec<study_spec::MetricSpec>,
    /// Required. The set of parameters to tune.
    #[prost(message, repeated, tag = "2")]
    pub parameters: ::prost::alloc::vec::Vec<study_spec::ParameterSpec>,
    /// The search algorithm specified for the Study.
    #[prost(enumeration = "study_spec::Algorithm", tag = "3")]
    pub algorithm: i32,
    /// The observation noise level of the study.
    /// Currently only supported by the Vizier service. Not supported by
    /// HyperparamterTuningJob or TrainingPipeline.
    #[prost(enumeration = "study_spec::ObservationNoise", tag = "6")]
    pub observation_noise: i32,
    /// Describe which measurement selection type will be used
    #[prost(enumeration = "study_spec::MeasurementSelectionType", tag = "7")]
    pub measurement_selection_type: i32,
    #[prost(oneof = "study_spec::AutomatedStoppingSpec", tags = "4, 5, 8")]
    pub automated_stopping_spec: ::core::option::Option<study_spec::AutomatedStoppingSpec>,
}
/// Nested message and enum types in `StudySpec`.
pub mod study_spec {
    /// Represents a metric to optimize.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetricSpec {
        /// Required. The ID of the metric. Must not contain whitespaces and must be unique
        /// amongst all MetricSpecs.
        #[prost(string, tag = "1")]
        pub metric_id: ::prost::alloc::string::String,
        /// Required. The optimization goal of the metric.
        #[prost(enumeration = "metric_spec::GoalType", tag = "2")]
        pub goal: i32,
    }
    /// Nested message and enum types in `MetricSpec`.
    pub mod metric_spec {
        /// The available types of optimization goals.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum GoalType {
            /// Goal Type will default to maximize.
            Unspecified = 0,
            /// Maximize the goal metric.
            Maximize = 1,
            /// Minimize the goal metric.
            Minimize = 2,
        }
    }
    /// Represents a single parameter to optimize.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ParameterSpec {
        /// Required. The ID of the parameter. Must not contain whitespaces and must be unique
        /// amongst all ParameterSpecs.
        #[prost(string, tag = "1")]
        pub parameter_id: ::prost::alloc::string::String,
        /// How the parameter should be scaled.
        /// Leave unset for `CATEGORICAL` parameters.
        #[prost(enumeration = "parameter_spec::ScaleType", tag = "6")]
        pub scale_type: i32,
        /// A conditional parameter node is active if the parameter's value matches
        /// the conditional node's parent_value_condition.
        ///
        /// If two items in conditional_parameter_specs have the same name, they
        /// must have disjoint parent_value_condition.
        #[prost(message, repeated, tag = "10")]
        pub conditional_parameter_specs:
            ::prost::alloc::vec::Vec<parameter_spec::ConditionalParameterSpec>,
        #[prost(oneof = "parameter_spec::ParameterValueSpec", tags = "2, 3, 4, 5")]
        pub parameter_value_spec: ::core::option::Option<parameter_spec::ParameterValueSpec>,
    }
    /// Nested message and enum types in `ParameterSpec`.
    pub mod parameter_spec {
        /// Value specification for a parameter in `DOUBLE` type.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DoubleValueSpec {
            /// Required. Inclusive minimum value of the parameter.
            #[prost(double, tag = "1")]
            pub min_value: f64,
            /// Required. Inclusive maximum value of the parameter.
            #[prost(double, tag = "2")]
            pub max_value: f64,
        }
        /// Value specification for a parameter in `INTEGER` type.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IntegerValueSpec {
            /// Required. Inclusive minimum value of the parameter.
            #[prost(int64, tag = "1")]
            pub min_value: i64,
            /// Required. Inclusive maximum value of the parameter.
            #[prost(int64, tag = "2")]
            pub max_value: i64,
        }
        /// Value specification for a parameter in `CATEGORICAL` type.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CategoricalValueSpec {
            /// Required. The list of possible categories.
            #[prost(string, repeated, tag = "1")]
            pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        /// Value specification for a parameter in `DISCRETE` type.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DiscreteValueSpec {
            /// Required. A list of possible values.
            /// The list should be in increasing order and at least 1e-10 apart.
            /// For instance, this parameter might have possible settings of 1.5, 2.5,
            /// and 4.0. This list should not contain more than 1,000 values.
            #[prost(double, repeated, packed = "false", tag = "1")]
            pub values: ::prost::alloc::vec::Vec<f64>,
        }
        /// Represents a parameter spec with condition from its parent parameter.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConditionalParameterSpec {
            /// Required. The spec for a conditional parameter.
            #[prost(message, optional, tag = "1")]
            pub parameter_spec: ::core::option::Option<super::ParameterSpec>,
            /// A set of parameter values from the parent ParameterSpec's feasible
            /// space.
            #[prost(
                oneof = "conditional_parameter_spec::ParentValueCondition",
                tags = "2, 3, 4"
            )]
            pub parent_value_condition:
                ::core::option::Option<conditional_parameter_spec::ParentValueCondition>,
        }
        /// Nested message and enum types in `ConditionalParameterSpec`.
        pub mod conditional_parameter_spec {
            /// Represents the spec to match discrete values from parent parameter.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DiscreteValueCondition {
                /// Required. Matches values of the parent parameter of 'DISCRETE' type.
                /// All values must exist in `discrete_value_spec` of parent parameter.
                ///
                /// The Epsilon of the value matching is 1e-10.
                #[prost(double, repeated, packed = "false", tag = "1")]
                pub values: ::prost::alloc::vec::Vec<f64>,
            }
            /// Represents the spec to match integer values from parent parameter.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct IntValueCondition {
                /// Required. Matches values of the parent parameter of 'INTEGER' type.
                /// All values must lie in `integer_value_spec` of parent parameter.
                #[prost(int64, repeated, packed = "false", tag = "1")]
                pub values: ::prost::alloc::vec::Vec<i64>,
            }
            /// Represents the spec to match categorical values from parent parameter.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct CategoricalValueCondition {
                /// Required. Matches values of the parent parameter of 'CATEGORICAL' type.
                /// All values must exist in `categorical_value_spec` of parent
                /// parameter.
                #[prost(string, repeated, tag = "1")]
                pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            }
            /// A set of parameter values from the parent ParameterSpec's feasible
            /// space.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum ParentValueCondition {
                /// The spec for matching values from a parent parameter of
                /// `DISCRETE` type.
                #[prost(message, tag = "2")]
                ParentDiscreteValues(DiscreteValueCondition),
                /// The spec for matching values from a parent parameter of `INTEGER`
                /// type.
                #[prost(message, tag = "3")]
                ParentIntValues(IntValueCondition),
                /// The spec for matching values from a parent parameter of
                /// `CATEGORICAL` type.
                #[prost(message, tag = "4")]
                ParentCategoricalValues(CategoricalValueCondition),
            }
        }
        /// The type of scaling that should be applied to this parameter.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum ScaleType {
            /// By default, no scaling is applied.
            Unspecified = 0,
            /// Scales the feasible space to (0, 1) linearly.
            UnitLinearScale = 1,
            /// Scales the feasible space logarithmically to (0, 1). The entire
            /// feasible space must be strictly positive.
            UnitLogScale = 2,
            /// Scales the feasible space "reverse" logarithmically to (0, 1). The
            /// result is that values close to the top of the feasible space are spread
            /// out more than points near the bottom. The entire feasible space must be
            /// strictly positive.
            UnitReverseLogScale = 3,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ParameterValueSpec {
            /// The value spec for a 'DOUBLE' parameter.
            #[prost(message, tag = "2")]
            DoubleValueSpec(DoubleValueSpec),
            /// The value spec for an 'INTEGER' parameter.
            #[prost(message, tag = "3")]
            IntegerValueSpec(IntegerValueSpec),
            /// The value spec for a 'CATEGORICAL' parameter.
            #[prost(message, tag = "4")]
            CategoricalValueSpec(CategoricalValueSpec),
            /// The value spec for a 'DISCRETE' parameter.
            #[prost(message, tag = "5")]
            DiscreteValueSpec(DiscreteValueSpec),
        }
    }
    /// The decay curve automated stopping rule builds a Gaussian Process
    /// Regressor to predict the final objective value of a Trial based on the
    /// already completed Trials and the intermediate measurements of the current
    /// Trial. Early stopping is requested for the current Trial if there is very
    /// low probability to exceed the optimal value found so far.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DecayCurveAutomatedStoppingSpec {
        /// True if [Measurement.elapsed_duration][google.cloud.aiplatform.v1beta1.Measurement.elapsed_duration] is used as the x-axis of each
        /// Trials Decay Curve. Otherwise, [Measurement.step_count][google.cloud.aiplatform.v1beta1.Measurement.step_count] will be used
        /// as the x-axis.
        #[prost(bool, tag = "1")]
        pub use_elapsed_duration: bool,
    }
    /// The median automated stopping rule stops a pending Trial if the Trial's
    /// best objective_value is strictly below the median 'performance' of all
    /// completed Trials reported up to the Trial's last measurement.
    /// Currently, 'performance' refers to the running average of the objective
    /// values reported by the Trial in each measurement.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MedianAutomatedStoppingSpec {
        /// True if median automated stopping rule applies on
        /// [Measurement.elapsed_duration][google.cloud.aiplatform.v1beta1.Measurement.elapsed_duration]. It means that elapsed_duration
        /// field of latest measurement of current Trial is used to compute median
        /// objective value for each completed Trials.
        #[prost(bool, tag = "1")]
        pub use_elapsed_duration: bool,
    }
    /// Configuration for ConvexStopPolicy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConvexStopConfig {
        /// Steps used in predicting the final objective for early stopped trials. In
        /// general, it's set to be the same as the defined steps in training /
        /// tuning. When use_steps is false, this field is set to the maximum elapsed
        /// seconds.
        #[prost(int64, tag = "1")]
        pub max_num_steps: i64,
        /// Minimum number of steps for a trial to complete. Trials which do not have
        /// a measurement with num_steps > min_num_steps won't be considered for
        /// early stopping. It's ok to set it to 0, and a trial can be early stopped
        /// at any stage. By default, min_num_steps is set to be one-tenth of the
        /// max_num_steps.
        /// When use_steps is false, this field is set to the minimum elapsed
        /// seconds.
        #[prost(int64, tag = "2")]
        pub min_num_steps: i64,
        /// The number of Trial measurements used in autoregressive model for
        /// value prediction. A trial won't be considered early stopping if has fewer
        /// measurement points.
        #[prost(int64, tag = "3")]
        pub autoregressive_order: i64,
        /// The hyper-parameter name used in the tuning job that stands for learning
        /// rate. Leave it blank if learning rate is not in a parameter in tuning.
        /// The learning_rate is used to estimate the objective value of the ongoing
        /// trial.
        #[prost(string, tag = "4")]
        pub learning_rate_parameter_name: ::prost::alloc::string::String,
        /// This bool determines whether or not the rule is applied based on
        /// elapsed_secs or steps. If use_seconds==false, the early stopping decision
        /// is made according to the predicted objective values according to the
        /// target steps. If use_seconds==true, elapsed_secs is used instead of
        /// steps. Also, in this case, the parameters max_num_steps and min_num_steps
        /// are overloaded to contain max_elapsed_seconds and min_elapsed_seconds.
        #[prost(bool, tag = "5")]
        pub use_seconds: bool,
    }
    /// The available search algorithms for the Study.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Algorithm {
        /// The default algorithm used by AI Platform Optimization service.
        Unspecified = 0,
        /// Simple grid search within the feasible space. To use grid search,
        /// all parameters must be `INTEGER`, `CATEGORICAL`, or `DISCRETE`.
        GridSearch = 2,
        /// Simple random search within the feasible space.
        RandomSearch = 3,
    }
    /// Describes the noise level of the repeated observations.
    ///
    /// "Noisy" means that the repeated observations with the same Trial parameters
    /// may lead to different metric evaluations.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ObservationNoise {
        /// The default noise level chosen by the AI Platform service.
        Unspecified = 0,
        /// AI Platform Vizier assumes that the objective function is (nearly)
        /// perfectly reproducible, and will never repeat the same Trial
        /// parameters.
        Low = 1,
        /// AI Platform Vizier will estimate the amount of noise in metric
        /// evaluations, it may repeat the same Trial parameters more than once.
        High = 2,
    }
    /// This indicates which measurement to use if/when the service automatically
    /// selects the final measurement from previously reported intermediate
    /// measurements. Choose this based on two considerations:
    ///  A) Do you expect your measurements to monotonically improve?
    ///     If so, choose LAST_MEASUREMENT. On the other hand, if you're in a
    ///     situation where your system can "over-train" and you expect the
    ///     performance to get better for a while but then start declining,
    ///     choose BEST_MEASUREMENT.
    ///  B) Are your measurements significantly noisy and/or irreproducible?
    ///     If so, BEST_MEASUREMENT will tend to be over-optimistic, and it
    ///     may be better to choose LAST_MEASUREMENT.
    ///  If both or neither of (A) and (B) apply, it doesn't matter which
    ///  selection type is chosen.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MeasurementSelectionType {
        /// Will be treated as LAST_MEASUREMENT.
        Unspecified = 0,
        /// Use the last measurement reported.
        LastMeasurement = 1,
        /// Use the best measurement reported.
        BestMeasurement = 2,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AutomatedStoppingSpec {
        /// The automated early stopping spec using decay curve rule.
        #[prost(message, tag = "4")]
        DecayCurveStoppingSpec(DecayCurveAutomatedStoppingSpec),
        /// The automated early stopping spec using median rule.
        #[prost(message, tag = "5")]
        MedianAutomatedStoppingSpec(MedianAutomatedStoppingSpec),
        /// The automated early stopping using convex stopping rule.
        #[prost(message, tag = "8")]
        ConvexStopConfig(ConvexStopConfig),
    }
}
/// A message representing a Measurement of a Trial. A Measurement contains
/// the Metrics got by executing a Trial using suggested hyperparameter
/// values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Measurement {
    /// Output only. The number of steps the machine learning model has been trained for.
    /// Must be non-negative.
    #[prost(int64, tag = "2")]
    pub step_count: i64,
    /// Output only. A list of metrics got by evaluating the objective functions using suggested
    /// Parameter values.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<measurement::Metric>,
}
/// Nested message and enum types in `Measurement`.
pub mod measurement {
    /// A message representing a metric in the measurement.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metric {
        /// Output only. The ID of the Metric. The Metric should be defined in
        /// [StudySpec's Metrics][google.cloud.aiplatform.v1beta1.StudySpec.metrics].
        #[prost(string, tag = "1")]
        pub metric_id: ::prost::alloc::string::String,
        /// Output only. The value for this metric.
        #[prost(double, tag = "2")]
        pub value: f64,
    }
}
/// Represents a HyperparameterTuningJob. A HyperparameterTuningJob
/// has a Study specification and multiple CustomJobs with identical
/// CustomJob specification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HyperparameterTuningJob {
    /// Output only. Resource name of the HyperparameterTuningJob.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The display name of the HyperparameterTuningJob.
    /// The name can be up to 128 characters long and can be consist of any UTF-8
    /// characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Study configuration of the HyperparameterTuningJob.
    #[prost(message, optional, tag = "4")]
    pub study_spec: ::core::option::Option<StudySpec>,
    /// Required. The desired total number of Trials.
    #[prost(int32, tag = "5")]
    pub max_trial_count: i32,
    /// Required. The desired number of Trials to run in parallel.
    #[prost(int32, tag = "6")]
    pub parallel_trial_count: i32,
    /// The number of failed Trials that need to be seen before failing
    /// the HyperparameterTuningJob.
    ///
    /// If set to 0, AI Platform decides how many Trials must fail
    /// before the whole job fails.
    #[prost(int32, tag = "7")]
    pub max_failed_trial_count: i32,
    /// Required. The spec of a trial job. The same spec applies to the CustomJobs created
    /// in all the trials.
    #[prost(message, optional, tag = "8")]
    pub trial_job_spec: ::core::option::Option<CustomJobSpec>,
    /// Output only. Trials of the HyperparameterTuningJob.
    #[prost(message, repeated, tag = "9")]
    pub trials: ::prost::alloc::vec::Vec<Trial>,
    /// Output only. The detailed state of the job.
    #[prost(enumeration = "JobState", tag = "10")]
    pub state: i32,
    /// Output only. Time when the HyperparameterTuningJob was created.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the HyperparameterTuningJob for the first time entered the
    /// `JOB_STATE_RUNNING` state.
    #[prost(message, optional, tag = "12")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the HyperparameterTuningJob entered any of the following states:
    /// `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`, `JOB_STATE_CANCELLED`.
    #[prost(message, optional, tag = "13")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the HyperparameterTuningJob was most recently updated.
    #[prost(message, optional, tag = "14")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Only populated when job's state is JOB_STATE_FAILED or
    /// JOB_STATE_CANCELLED.
    #[prost(message, optional, tag = "15")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The labels with user-defined metadata to organize HyperparameterTuningJobs.
    ///
    /// Label keys and values can be no longer than 64 characters
    /// (Unicode codepoints), can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    ///
    /// See https://goo.gl/xmQnxf for more information and examples of labels.
    #[prost(map = "string, string", tag = "16")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Customer-managed encryption key options for a HyperparameterTuningJob.
    /// If this is set, then all resources created by the HyperparameterTuningJob
    /// will be encrypted with the provided encryption key.
    #[prost(message, optional, tag = "17")]
    pub encryption_spec: ::core::option::Option<EncryptionSpec>,
}
/// Request message for [JobService.CreateCustomJob][google.cloud.aiplatform.v1beta1.JobService.CreateCustomJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomJobRequest {
    /// Required. The resource name of the Location to create the CustomJob in.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The CustomJob to create.
    #[prost(message, optional, tag = "2")]
    pub custom_job: ::core::option::Option<CustomJob>,
}
/// Request message for [JobService.GetCustomJob][google.cloud.aiplatform.v1beta1.JobService.GetCustomJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomJobRequest {
    /// Required. The name of the CustomJob resource.
    /// Format:
    /// `projects/{project}/locations/{location}/customJobs/{custom_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [JobService.ListCustomJobs][google.cloud.aiplatform.v1beta1.JobService.ListCustomJobs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomJobsRequest {
    /// Required. The resource name of the Location to list the CustomJobs from.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard list filter.
    ///
    /// Supported fields:
    ///
    ///   * `display_name` supports = and !=.
    ///
    ///   * `state` supports = and !=.
    ///
    /// Some examples of using the filter are:
    ///
    ///  * `state="JOB_STATE_SUCCEEDED" AND display_name="my_job"`
    ///
    ///  * `state="JOB_STATE_RUNNING" OR display_name="my_job"`
    ///
    ///  * `NOT display_name="my_job"`
    ///
    ///  * `state="JOB_STATE_FAILED"`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    /// Typically obtained via
    /// [ListCustomJobsResponse.next_page_token][google.cloud.aiplatform.v1beta1.ListCustomJobsResponse.next_page_token] of the previous
    /// [JobService.ListCustomJobs][google.cloud.aiplatform.v1beta1.JobService.ListCustomJobs] call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for [JobService.ListCustomJobs][google.cloud.aiplatform.v1beta1.JobService.ListCustomJobs]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomJobsResponse {
    /// List of CustomJobs in the requested page.
    #[prost(message, repeated, tag = "1")]
    pub custom_jobs: ::prost::alloc::vec::Vec<CustomJob>,
    /// A token to retrieve the next page of results.
    /// Pass to [ListCustomJobsRequest.page_token][google.cloud.aiplatform.v1beta1.ListCustomJobsRequest.page_token] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [JobService.DeleteCustomJob][google.cloud.aiplatform.v1beta1.JobService.DeleteCustomJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCustomJobRequest {
    /// Required. The name of the CustomJob resource to be deleted.
    /// Format:
    /// `projects/{project}/locations/{location}/customJobs/{custom_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [JobService.CancelCustomJob][google.cloud.aiplatform.v1beta1.JobService.CancelCustomJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelCustomJobRequest {
    /// Required. The name of the CustomJob to cancel.
    /// Format:
    /// `projects/{project}/locations/{location}/customJobs/{custom_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [DataLabelingJobService.CreateDataLabelingJob][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataLabelingJobRequest {
    /// Required. The parent of the DataLabelingJob.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The DataLabelingJob to create.
    #[prost(message, optional, tag = "2")]
    pub data_labeling_job: ::core::option::Option<DataLabelingJob>,
}
/// Request message for [DataLabelingJobService.GetDataLabelingJob][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataLabelingJobRequest {
    /// Required. The name of the DataLabelingJob.
    /// Format:
    /// `projects/{project}/locations/{location}/dataLabelingJobs/{data_labeling_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [DataLabelingJobService.ListDataLabelingJobs][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataLabelingJobsRequest {
    /// Required. The parent of the DataLabelingJob.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard list filter.
    ///
    /// Supported fields:
    ///
    ///   * `display_name` supports = and !=.
    ///
    ///   * `state` supports = and !=.
    ///
    /// Some examples of using the filter are:
    ///
    ///  * `state="JOB_STATE_SUCCEEDED" AND display_name="my_job"`
    ///
    ///  * `state="JOB_STATE_RUNNING" OR display_name="my_job"`
    ///
    ///  * `NOT display_name="my_job"`
    ///
    ///  * `state="JOB_STATE_FAILED"`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read. FieldMask represents a set of
    /// symbolic field paths. For example, the mask can be `paths: "name"`. The
    /// "name" here is a field in DataLabelingJob.
    /// If this field is not set, all fields of the DataLabelingJob are returned.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// A comma-separated list of fields to order by, sorted in ascending order by
    /// default.
    /// Use `desc` after a field name for descending.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for [JobService.ListDataLabelingJobs][google.cloud.aiplatform.v1beta1.JobService.ListDataLabelingJobs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataLabelingJobsResponse {
    /// A list of DataLabelingJobs that matches the specified filter in the
    /// request.
    #[prost(message, repeated, tag = "1")]
    pub data_labeling_jobs: ::prost::alloc::vec::Vec<DataLabelingJob>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [JobService.DeleteDataLabelingJob][google.cloud.aiplatform.v1beta1.JobService.DeleteDataLabelingJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataLabelingJobRequest {
    /// Required. The name of the DataLabelingJob to be deleted.
    /// Format:
    /// `projects/{project}/locations/{location}/dataLabelingJobs/{data_labeling_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [DataLabelingJobService.CancelDataLabelingJob][].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelDataLabelingJobRequest {
    /// Required. The name of the DataLabelingJob.
    /// Format:
    /// `projects/{project}/locations/{location}/dataLabelingJobs/{data_labeling_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [JobService.CreateHyperparameterTuningJob][google.cloud.aiplatform.v1beta1.JobService.CreateHyperparameterTuningJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateHyperparameterTuningJobRequest {
    /// Required. The resource name of the Location to create the HyperparameterTuningJob in.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The HyperparameterTuningJob to create.
    #[prost(message, optional, tag = "2")]
    pub hyperparameter_tuning_job: ::core::option::Option<HyperparameterTuningJob>,
}
/// Request message for [JobService.GetHyperparameterTuningJob][google.cloud.aiplatform.v1beta1.JobService.GetHyperparameterTuningJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHyperparameterTuningJobRequest {
    /// Required. The name of the HyperparameterTuningJob resource.
    /// Format:
    /// `projects/{project}/locations/{location}/hyperparameterTuningJobs/{hyperparameter_tuning_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [JobService.ListHyperparameterTuningJobs][google.cloud.aiplatform.v1beta1.JobService.ListHyperparameterTuningJobs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHyperparameterTuningJobsRequest {
    /// Required. The resource name of the Location to list the HyperparameterTuningJobs
    /// from. Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard list filter.
    ///
    /// Supported fields:
    ///
    ///   * `display_name` supports = and !=.
    ///
    ///   * `state` supports = and !=.
    ///
    /// Some examples of using the filter are:
    ///
    ///  * `state="JOB_STATE_SUCCEEDED" AND display_name="my_job"`
    ///
    ///  * `state="JOB_STATE_RUNNING" OR display_name="my_job"`
    ///
    ///  * `NOT display_name="my_job"`
    ///
    ///  * `state="JOB_STATE_FAILED"`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    /// Typically obtained via
    /// [ListHyperparameterTuningJobsResponse.next_page_token][google.cloud.aiplatform.v1beta1.ListHyperparameterTuningJobsResponse.next_page_token] of the previous
    /// [JobService.ListHyperparameterTuningJobs][google.cloud.aiplatform.v1beta1.JobService.ListHyperparameterTuningJobs] call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for [JobService.ListHyperparameterTuningJobs][google.cloud.aiplatform.v1beta1.JobService.ListHyperparameterTuningJobs]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListHyperparameterTuningJobsResponse {
    /// List of HyperparameterTuningJobs in the requested page.
    /// [HyperparameterTuningJob.trials][google.cloud.aiplatform.v1beta1.HyperparameterTuningJob.trials] of the jobs will be not be returned.
    #[prost(message, repeated, tag = "1")]
    pub hyperparameter_tuning_jobs: ::prost::alloc::vec::Vec<HyperparameterTuningJob>,
    /// A token to retrieve the next page of results.
    /// Pass to [ListHyperparameterTuningJobsRequest.page_token][google.cloud.aiplatform.v1beta1.ListHyperparameterTuningJobsRequest.page_token] to obtain that
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [JobService.DeleteHyperparameterTuningJob][google.cloud.aiplatform.v1beta1.JobService.DeleteHyperparameterTuningJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteHyperparameterTuningJobRequest {
    /// Required. The name of the HyperparameterTuningJob resource to be deleted.
    /// Format:
    /// `projects/{project}/locations/{location}/hyperparameterTuningJobs/{hyperparameter_tuning_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [JobService.CancelHyperparameterTuningJob][google.cloud.aiplatform.v1beta1.JobService.CancelHyperparameterTuningJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelHyperparameterTuningJobRequest {
    /// Required. The name of the HyperparameterTuningJob to cancel.
    /// Format:
    /// `projects/{project}/locations/{location}/hyperparameterTuningJobs/{hyperparameter_tuning_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [JobService.CreateBatchPredictionJob][google.cloud.aiplatform.v1beta1.JobService.CreateBatchPredictionJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBatchPredictionJobRequest {
    /// Required. The resource name of the Location to create the BatchPredictionJob in.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The BatchPredictionJob to create.
    #[prost(message, optional, tag = "2")]
    pub batch_prediction_job: ::core::option::Option<BatchPredictionJob>,
}
/// Request message for [JobService.GetBatchPredictionJob][google.cloud.aiplatform.v1beta1.JobService.GetBatchPredictionJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBatchPredictionJobRequest {
    /// Required. The name of the BatchPredictionJob resource.
    /// Format:
    /// `projects/{project}/locations/{location}/batchPredictionJobs/{batch_prediction_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [JobService.ListBatchPredictionJobs][google.cloud.aiplatform.v1beta1.JobService.ListBatchPredictionJobs].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBatchPredictionJobsRequest {
    /// Required. The resource name of the Location to list the BatchPredictionJobs
    /// from. Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard list filter.
    ///
    /// Supported fields:
    ///
    ///   * `display_name` supports = and !=.
    ///
    ///   * `state` supports = and !=.
    ///
    ///   * `model_display_name` supports = and !=
    ///
    /// Some examples of using the filter are:
    ///
    ///  * `state="JOB_STATE_SUCCEEDED" AND display_name="my_job"`
    ///
    ///  * `state="JOB_STATE_RUNNING" OR display_name="my_job"`
    ///
    ///  * `NOT display_name="my_job"`
    ///
    ///  * `state="JOB_STATE_FAILED"`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    /// Typically obtained via
    /// [ListBatchPredictionJobsResponse.next_page_token][google.cloud.aiplatform.v1beta1.ListBatchPredictionJobsResponse.next_page_token] of the previous
    /// [JobService.ListBatchPredictionJobs][google.cloud.aiplatform.v1beta1.JobService.ListBatchPredictionJobs] call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for [JobService.ListBatchPredictionJobs][google.cloud.aiplatform.v1beta1.JobService.ListBatchPredictionJobs]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBatchPredictionJobsResponse {
    /// List of BatchPredictionJobs in the requested page.
    #[prost(message, repeated, tag = "1")]
    pub batch_prediction_jobs: ::prost::alloc::vec::Vec<BatchPredictionJob>,
    /// A token to retrieve the next page of results.
    /// Pass to [ListBatchPredictionJobsRequest.page_token][google.cloud.aiplatform.v1beta1.ListBatchPredictionJobsRequest.page_token] to obtain that
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [JobService.DeleteBatchPredictionJob][google.cloud.aiplatform.v1beta1.JobService.DeleteBatchPredictionJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBatchPredictionJobRequest {
    /// Required. The name of the BatchPredictionJob resource to be deleted.
    /// Format:
    /// `projects/{project}/locations/{location}/batchPredictionJobs/{batch_prediction_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [JobService.CancelBatchPredictionJob][google.cloud.aiplatform.v1beta1.JobService.CancelBatchPredictionJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelBatchPredictionJobRequest {
    /// Required. The name of the BatchPredictionJob to cancel.
    /// Format:
    /// `projects/{project}/locations/{location}/batchPredictionJobs/{batch_prediction_job}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod job_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service for creating and managing AI Platform's jobs."]
    pub struct JobServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> JobServiceClient<T>
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
        #[doc = " Creates a CustomJob. A created CustomJob right away"]
        #[doc = " will be attempted to be run."]
        pub async fn create_custom_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomJobRequest>,
        ) -> Result<tonic::Response<super::CustomJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/CreateCustomJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a CustomJob."]
        pub async fn get_custom_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomJobRequest>,
        ) -> Result<tonic::Response<super::CustomJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/GetCustomJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists CustomJobs in a Location."]
        pub async fn list_custom_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomJobsRequest>,
        ) -> Result<tonic::Response<super::ListCustomJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/ListCustomJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a CustomJob."]
        pub async fn delete_custom_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCustomJobRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/DeleteCustomJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels a CustomJob."]
        #[doc = " Starts asynchronous cancellation on the CustomJob. The server"]
        #[doc = " makes a best effort to cancel the job, but success is not"]
        #[doc = " guaranteed. Clients can use [JobService.GetCustomJob][google.cloud.aiplatform.v1beta1.JobService.GetCustomJob] or"]
        #[doc = " other methods to check whether the cancellation succeeded or whether the"]
        #[doc = " job completed despite cancellation. On successful cancellation,"]
        #[doc = " the CustomJob is not deleted; instead it becomes a job with"]
        #[doc = " a [CustomJob.error][google.cloud.aiplatform.v1beta1.CustomJob.error] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,"]
        #[doc = " corresponding to `Code.CANCELLED`, and [CustomJob.state][google.cloud.aiplatform.v1beta1.CustomJob.state] is set to"]
        #[doc = " `CANCELLED`."]
        pub async fn cancel_custom_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelCustomJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/CancelCustomJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a DataLabelingJob."]
        pub async fn create_data_labeling_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDataLabelingJobRequest>,
        ) -> Result<tonic::Response<super::DataLabelingJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/CreateDataLabelingJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a DataLabelingJob."]
        pub async fn get_data_labeling_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataLabelingJobRequest>,
        ) -> Result<tonic::Response<super::DataLabelingJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/GetDataLabelingJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists DataLabelingJobs in a Location."]
        pub async fn list_data_labeling_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataLabelingJobsRequest>,
        ) -> Result<tonic::Response<super::ListDataLabelingJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/ListDataLabelingJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a DataLabelingJob."]
        pub async fn delete_data_labeling_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataLabelingJobRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/DeleteDataLabelingJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels a DataLabelingJob. Success of cancellation is not guaranteed."]
        pub async fn cancel_data_labeling_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelDataLabelingJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/CancelDataLabelingJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a HyperparameterTuningJob"]
        pub async fn create_hyperparameter_tuning_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateHyperparameterTuningJobRequest>,
        ) -> Result<tonic::Response<super::HyperparameterTuningJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/CreateHyperparameterTuningJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a HyperparameterTuningJob"]
        pub async fn get_hyperparameter_tuning_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHyperparameterTuningJobRequest>,
        ) -> Result<tonic::Response<super::HyperparameterTuningJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/GetHyperparameterTuningJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists HyperparameterTuningJobs in a Location."]
        pub async fn list_hyperparameter_tuning_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListHyperparameterTuningJobsRequest>,
        ) -> Result<tonic::Response<super::ListHyperparameterTuningJobsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/ListHyperparameterTuningJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a HyperparameterTuningJob."]
        pub async fn delete_hyperparameter_tuning_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteHyperparameterTuningJobRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/DeleteHyperparameterTuningJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels a HyperparameterTuningJob."]
        #[doc = " Starts asynchronous cancellation on the HyperparameterTuningJob. The server"]
        #[doc = " makes a best effort to cancel the job, but success is not"]
        #[doc = " guaranteed. Clients can use [JobService.GetHyperparameterTuningJob][google.cloud.aiplatform.v1beta1.JobService.GetHyperparameterTuningJob] or"]
        #[doc = " other methods to check whether the cancellation succeeded or whether the"]
        #[doc = " job completed despite cancellation. On successful cancellation,"]
        #[doc = " the HyperparameterTuningJob is not deleted; instead it becomes a job with"]
        #[doc = " a [HyperparameterTuningJob.error][google.cloud.aiplatform.v1beta1.HyperparameterTuningJob.error] value with a [google.rpc.Status.code][google.rpc.Status.code]"]
        #[doc = " of 1, corresponding to `Code.CANCELLED`, and"]
        #[doc = " [HyperparameterTuningJob.state][google.cloud.aiplatform.v1beta1.HyperparameterTuningJob.state] is set to `CANCELLED`."]
        pub async fn cancel_hyperparameter_tuning_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelHyperparameterTuningJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/CancelHyperparameterTuningJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a BatchPredictionJob. A BatchPredictionJob once created will"]
        #[doc = " right away be attempted to start."]
        pub async fn create_batch_prediction_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBatchPredictionJobRequest>,
        ) -> Result<tonic::Response<super::BatchPredictionJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/CreateBatchPredictionJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a BatchPredictionJob"]
        pub async fn get_batch_prediction_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBatchPredictionJobRequest>,
        ) -> Result<tonic::Response<super::BatchPredictionJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/GetBatchPredictionJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists BatchPredictionJobs in a Location."]
        pub async fn list_batch_prediction_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBatchPredictionJobsRequest>,
        ) -> Result<tonic::Response<super::ListBatchPredictionJobsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/ListBatchPredictionJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a BatchPredictionJob. Can only be called on jobs that already"]
        #[doc = " finished."]
        pub async fn delete_batch_prediction_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBatchPredictionJobRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/DeleteBatchPredictionJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels a BatchPredictionJob."]
        #[doc = ""]
        #[doc = " Starts asynchronous cancellation on the BatchPredictionJob. The server"]
        #[doc = " makes the best effort to cancel the job, but success is not"]
        #[doc = " guaranteed. Clients can use [JobService.GetBatchPredictionJob][google.cloud.aiplatform.v1beta1.JobService.GetBatchPredictionJob] or"]
        #[doc = " other methods to check whether the cancellation succeeded or whether the"]
        #[doc = " job completed despite cancellation. On a successful cancellation,"]
        #[doc = " the BatchPredictionJob is not deleted;instead its"]
        #[doc = " [BatchPredictionJob.state][google.cloud.aiplatform.v1beta1.BatchPredictionJob.state] is set to `CANCELLED`. Any files already"]
        #[doc = " outputted by the job are not deleted."]
        pub async fn cancel_batch_prediction_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelBatchPredictionJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.JobService/CancelBatchPredictionJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for JobServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for JobServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "JobServiceClient {{ ... }}")
        }
    }
}
/// Represents one resource that exists in automl.googleapis.com,
/// datalabeling.googleapis.com or ml.googleapis.com.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigratableResource {
    /// Output only. Timestamp when the last migration attempt on this MigratableResource
    /// started. Will not be set if there's no migration attempt on this
    /// MigratableResource.
    #[prost(message, optional, tag = "5")]
    pub last_migrate_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when this MigratableResource was last updated.
    #[prost(message, optional, tag = "6")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "migratable_resource::Resource", tags = "1, 2, 3, 4")]
    pub resource: ::core::option::Option<migratable_resource::Resource>,
}
/// Nested message and enum types in `MigratableResource`.
pub mod migratable_resource {
    /// Represents one model Version in ml.googleapis.com.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MlEngineModelVersion {
        /// The ml.googleapis.com endpoint that this model Version currently lives
        /// in.
        /// Example values:
        ///
        /// * ml.googleapis.com
        /// * us-centrall-ml.googleapis.com
        /// * europe-west4-ml.googleapis.com
        /// * asia-east1-ml.googleapis.com
        #[prost(string, tag = "1")]
        pub endpoint: ::prost::alloc::string::String,
        /// Full resource name of ml engine model Version.
        /// Format: `projects/{project}/models/{model}/versions/{version}`.
        #[prost(string, tag = "2")]
        pub version: ::prost::alloc::string::String,
    }
    /// Represents one Model in automl.googleapis.com.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutomlModel {
        /// Full resource name of automl Model.
        /// Format:
        /// `projects/{project}/locations/{location}/models/{model}`.
        #[prost(string, tag = "1")]
        pub model: ::prost::alloc::string::String,
        /// The Model's display name in automl.googleapis.com.
        #[prost(string, tag = "3")]
        pub model_display_name: ::prost::alloc::string::String,
    }
    /// Represents one Dataset in automl.googleapis.com.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutomlDataset {
        /// Full resource name of automl Dataset.
        /// Format:
        /// `projects/{project}/locations/{location}/datasets/{dataset}`.
        #[prost(string, tag = "1")]
        pub dataset: ::prost::alloc::string::String,
        /// The Dataset's display name in automl.googleapis.com.
        #[prost(string, tag = "4")]
        pub dataset_display_name: ::prost::alloc::string::String,
    }
    /// Represents one Dataset in datalabeling.googleapis.com.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataLabelingDataset {
        /// Full resource name of data labeling Dataset.
        /// Format:
        /// `projects/{project}/datasets/{dataset}`.
        #[prost(string, tag = "1")]
        pub dataset: ::prost::alloc::string::String,
        /// The Dataset's display name in datalabeling.googleapis.com.
        #[prost(string, tag = "4")]
        pub dataset_display_name: ::prost::alloc::string::String,
        /// The migratable AnnotatedDataset in datalabeling.googleapis.com belongs to
        /// the data labeling Dataset.
        #[prost(message, repeated, tag = "3")]
        pub data_labeling_annotated_datasets:
            ::prost::alloc::vec::Vec<data_labeling_dataset::DataLabelingAnnotatedDataset>,
    }
    /// Nested message and enum types in `DataLabelingDataset`.
    pub mod data_labeling_dataset {
        /// Represents one AnnotatedDataset in datalabeling.googleapis.com.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DataLabelingAnnotatedDataset {
            /// Full resource name of data labeling AnnotatedDataset.
            /// Format:
            /// `projects/{project}/datasets/{dataset}/annotatedDatasets/{annotated_dataset}`.
            #[prost(string, tag = "1")]
            pub annotated_dataset: ::prost::alloc::string::String,
            /// The AnnotatedDataset's display name in datalabeling.googleapis.com.
            #[prost(string, tag = "3")]
            pub annotated_dataset_display_name: ::prost::alloc::string::String,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        /// Output only. Represents one Version in ml.googleapis.com.
        #[prost(message, tag = "1")]
        MlEngineModelVersion(MlEngineModelVersion),
        /// Output only. Represents one Model in automl.googleapis.com.
        #[prost(message, tag = "2")]
        AutomlModel(AutomlModel),
        /// Output only. Represents one Dataset in automl.googleapis.com.
        #[prost(message, tag = "3")]
        AutomlDataset(AutomlDataset),
        /// Output only. Represents one Dataset in datalabeling.googleapis.com.
        #[prost(message, tag = "4")]
        DataLabelingDataset(DataLabelingDataset),
    }
}
/// Request message for [MigrationService.SearchMigratableResources][google.cloud.aiplatform.v1beta1.MigrationService.SearchMigratableResources].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchMigratableResourcesRequest {
    /// Required. The location that the migratable resources should be searched from.
    /// It's the AI Platform location that the resources can be migrated to, not
    /// the resources' original location.
    /// Format:
    /// `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard page size.
    /// The default and maximum value is 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The standard page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Supported filters are:
    /// * Resource type: For a specific type of MigratableResource.
    ///   * `ml_engine_model_version:*`
    ///   * `automl_model:*`,
    ///   * `automl_dataset:*`
    ///   * `data_labeling_dataset:*`.
    /// * Migrated or not: Filter migrated resource or not by last_migrate_time.
    ///   * `last_migrate_time:*` will filter migrated resources.
    ///   * `NOT last_migrate_time:*` will filter not yet migrated resources.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for [MigrationService.SearchMigratableResources][google.cloud.aiplatform.v1beta1.MigrationService.SearchMigratableResources].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchMigratableResourcesResponse {
    /// All migratable resources that can be migrated to the
    /// location specified in the request.
    #[prost(message, repeated, tag = "1")]
    pub migratable_resources: ::prost::alloc::vec::Vec<MigratableResource>,
    /// The standard next-page token.
    /// The migratable_resources may not fill page_size in
    /// SearchMigratableResourcesRequest even when there are subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [MigrationService.BatchMigrateResources][google.cloud.aiplatform.v1beta1.MigrationService.BatchMigrateResources].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchMigrateResourcesRequest {
    /// Required. The location of the migrated resource will live in.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The request messages specifying the resources to migrate.
    /// They must be in the same location as the destination.
    /// Up to 50 resources can be migrated in one batch.
    #[prost(message, repeated, tag = "2")]
    pub migrate_resource_requests: ::prost::alloc::vec::Vec<MigrateResourceRequest>,
}
/// Config of migrating one resource from automl.googleapis.com,
/// datalabeling.googleapis.com and ml.googleapis.com to AI Platform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateResourceRequest {
    #[prost(oneof = "migrate_resource_request::Request", tags = "1, 2, 3, 4")]
    pub request: ::core::option::Option<migrate_resource_request::Request>,
}
/// Nested message and enum types in `MigrateResourceRequest`.
pub mod migrate_resource_request {
    /// Config for migrating version in ml.googleapis.com to AI Platform's Model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MigrateMlEngineModelVersionConfig {
        /// Required. The ml.googleapis.com endpoint that this model version should be migrated
        /// from.
        /// Example values:
        ///
        /// * ml.googleapis.com
        ///
        /// * us-centrall-ml.googleapis.com
        ///
        /// * europe-west4-ml.googleapis.com
        ///
        /// * asia-east1-ml.googleapis.com
        #[prost(string, tag = "1")]
        pub endpoint: ::prost::alloc::string::String,
        /// Required. Full resource name of ml engine model version.
        /// Format: `projects/{project}/models/{model}/versions/{version}`.
        #[prost(string, tag = "2")]
        pub model_version: ::prost::alloc::string::String,
        /// Required. Display name of the model in AI Platform.
        /// System will pick a display name if unspecified.
        #[prost(string, tag = "3")]
        pub model_display_name: ::prost::alloc::string::String,
    }
    /// Config for migrating Model in automl.googleapis.com to AI Platform's Model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MigrateAutomlModelConfig {
        /// Required. Full resource name of automl Model.
        /// Format:
        /// `projects/{project}/locations/{location}/models/{model}`.
        #[prost(string, tag = "1")]
        pub model: ::prost::alloc::string::String,
        /// Optional. Display name of the model in AI Platform.
        /// System will pick a display name if unspecified.
        #[prost(string, tag = "2")]
        pub model_display_name: ::prost::alloc::string::String,
    }
    /// Config for migrating Dataset in automl.googleapis.com to AI Platform's
    /// Dataset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MigrateAutomlDatasetConfig {
        /// Required. Full resource name of automl Dataset.
        /// Format:
        /// `projects/{project}/locations/{location}/datasets/{dataset}`.
        #[prost(string, tag = "1")]
        pub dataset: ::prost::alloc::string::String,
        /// Required. Display name of the Dataset in AI Platform.
        /// System will pick a display name if unspecified.
        #[prost(string, tag = "2")]
        pub dataset_display_name: ::prost::alloc::string::String,
    }
    /// Config for migrating Dataset in datalabeling.googleapis.com to AI
    /// Platform's Dataset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MigrateDataLabelingDatasetConfig {
        /// Required. Full resource name of data labeling Dataset.
        /// Format:
        /// `projects/{project}/datasets/{dataset}`.
        #[prost(string, tag = "1")]
        pub dataset: ::prost::alloc::string::String,
        /// Optional. Display name of the Dataset in AI Platform.
        /// System will pick a display name if unspecified.
        #[prost(string, tag = "2")]
        pub dataset_display_name: ::prost::alloc::string::String,
        /// Optional. Configs for migrating AnnotatedDataset in datalabeling.googleapis.com to
        /// AI Platform's SavedQuery. The specified AnnotatedDatasets have to belong
        /// to the datalabeling Dataset.
        #[prost(message, repeated, tag = "3")]
        pub migrate_data_labeling_annotated_dataset_configs: ::prost::alloc::vec::Vec<
            migrate_data_labeling_dataset_config::MigrateDataLabelingAnnotatedDatasetConfig,
        >,
    }
    /// Nested message and enum types in `MigrateDataLabelingDatasetConfig`.
    pub mod migrate_data_labeling_dataset_config {
        /// Config for migrating AnnotatedDataset in datalabeling.googleapis.com to
        /// AI Platform's SavedQuery.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MigrateDataLabelingAnnotatedDatasetConfig {
            /// Required. Full resource name of data labeling AnnotatedDataset.
            /// Format:
            /// `projects/{project}/datasets/{dataset}/annotatedDatasets/{annotated_dataset}`.
            #[prost(string, tag = "1")]
            pub annotated_dataset: ::prost::alloc::string::String,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Config for migrating Version in ml.googleapis.com to AI Platform's Model.
        #[prost(message, tag = "1")]
        MigrateMlEngineModelVersionConfig(MigrateMlEngineModelVersionConfig),
        /// Config for migrating Model in automl.googleapis.com to AI Platform's
        /// Model.
        #[prost(message, tag = "2")]
        MigrateAutomlModelConfig(MigrateAutomlModelConfig),
        /// Config for migrating Dataset in automl.googleapis.com to AI Platform's
        /// Dataset.
        #[prost(message, tag = "3")]
        MigrateAutomlDatasetConfig(MigrateAutomlDatasetConfig),
        /// Config for migrating Dataset in datalabeling.googleapis.com to
        /// AI Platform's Dataset.
        #[prost(message, tag = "4")]
        MigrateDataLabelingDatasetConfig(MigrateDataLabelingDatasetConfig),
    }
}
/// Response message for [MigrationService.BatchMigrateResources][google.cloud.aiplatform.v1beta1.MigrationService.BatchMigrateResources].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchMigrateResourcesResponse {
    /// Successfully migrated resources.
    #[prost(message, repeated, tag = "1")]
    pub migrate_resource_responses: ::prost::alloc::vec::Vec<MigrateResourceResponse>,
}
/// Describes a successfully migrated resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateResourceResponse {
    /// Before migration, the identifier in ml.googleapis.com,
    /// automl.googleapis.com or datalabeling.googleapis.com.
    #[prost(message, optional, tag = "3")]
    pub migratable_resource: ::core::option::Option<MigratableResource>,
    /// After migration, the resource name in AI Platform.
    #[prost(oneof = "migrate_resource_response::MigratedResource", tags = "1, 2")]
    pub migrated_resource: ::core::option::Option<migrate_resource_response::MigratedResource>,
}
/// Nested message and enum types in `MigrateResourceResponse`.
pub mod migrate_resource_response {
    /// After migration, the resource name in AI Platform.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MigratedResource {
        /// Migrated Dataset's resource name.
        #[prost(string, tag = "1")]
        Dataset(::prost::alloc::string::String),
        /// Migrated Model's resource name.
        #[prost(string, tag = "2")]
        Model(::prost::alloc::string::String),
    }
}
/// Runtime operation information for [MigrationService.BatchMigrateResources][google.cloud.aiplatform.v1beta1.MigrationService.BatchMigrateResources].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchMigrateResourcesOperationMetadata {
    /// The common part of the operation metadata.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
    /// Partial results that reflect the latest migration operation progress.
    #[prost(message, repeated, tag = "2")]
    pub partial_results:
        ::prost::alloc::vec::Vec<batch_migrate_resources_operation_metadata::PartialResult>,
}
/// Nested message and enum types in `BatchMigrateResourcesOperationMetadata`.
pub mod batch_migrate_resources_operation_metadata {
    /// Represents a partial result in batch migration operation for one
    /// [MigrateResourceRequest][google.cloud.aiplatform.v1beta1.MigrateResourceRequest].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PartialResult {
        /// It's the same as the value in
        /// [MigrateResourceRequest.migrate_resource_requests][].
        #[prost(message, optional, tag = "1")]
        pub request: ::core::option::Option<super::MigrateResourceRequest>,
        /// If the resource's migration is ongoing, none of the result will be set.
        /// If the resource's migration is finished, either error or one of the
        /// migrated resource name will be filled.
        #[prost(oneof = "partial_result::Result", tags = "2, 3, 4")]
        pub result: ::core::option::Option<partial_result::Result>,
    }
    /// Nested message and enum types in `PartialResult`.
    pub mod partial_result {
        /// If the resource's migration is ongoing, none of the result will be set.
        /// If the resource's migration is finished, either error or one of the
        /// migrated resource name will be filled.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Result {
            /// The error result of the migration request in case of failure.
            #[prost(message, tag = "2")]
            Error(super::super::super::super::super::rpc::Status),
            /// Migrated model resource name.
            #[prost(string, tag = "3")]
            Model(::prost::alloc::string::String),
            /// Migrated dataset resource name.
            #[prost(string, tag = "4")]
            Dataset(::prost::alloc::string::String),
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod migration_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service that migrates resources from automl.googleapis.com,"]
    #[doc = " datalabeling.googleapis.com and ml.googleapis.com to AI Platform."]
    pub struct MigrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MigrationServiceClient<T>
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
        #[doc = " Searches all of the resources in automl.googleapis.com,"]
        #[doc = " datalabeling.googleapis.com and ml.googleapis.com that can be migrated to"]
        #[doc = " AI Platform's given location."]
        pub async fn search_migratable_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchMigratableResourcesRequest>,
        ) -> Result<tonic::Response<super::SearchMigratableResourcesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.MigrationService/SearchMigratableResources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Batch migrates resources from ml.googleapis.com, automl.googleapis.com,"]
        #[doc = " and datalabeling.googleapis.com to AI Platform (Unified)."]
        pub async fn batch_migrate_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchMigrateResourcesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.MigrationService/BatchMigrateResources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MigrationServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MigrationServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MigrationServiceClient {{ ... }}")
        }
    }
}
/// A collection of metrics calculated by comparing Model's predictions on all of
/// the test data against annotations from the test data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelEvaluation {
    /// Output only. The resource name of the ModelEvaluation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Points to a YAML file stored on Google Cloud Storage describing the
    /// [metrics][google.cloud.aiplatform.v1beta1.ModelEvaluation.metrics] of this ModelEvaluation. The schema is
    /// defined as an OpenAPI 3.0.2
    /// [Schema Object](https://tinyurl.com/y538mdwt#schema-object).
    #[prost(string, tag = "2")]
    pub metrics_schema_uri: ::prost::alloc::string::String,
    /// Output only. Evaluation metrics of the Model. The schema of the metrics is stored in
    /// [metrics_schema_uri][google.cloud.aiplatform.v1beta1.ModelEvaluation.metrics_schema_uri]
    #[prost(message, optional, tag = "3")]
    pub metrics: ::core::option::Option<::prost_types::Value>,
    /// Output only. Timestamp when this ModelEvaluation was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. All possible [dimensions][ModelEvaluationSlice.slice.dimension] of
    /// ModelEvaluationSlices. The dimensions can be used as the filter of the
    /// [ModelService.ListModelEvaluationSlices][google.cloud.aiplatform.v1beta1.ModelService.ListModelEvaluationSlices] request, in the form of
    /// `slice.dimension = <dimension>`.
    #[prost(string, repeated, tag = "5")]
    pub slice_dimensions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Aggregated explanation metrics for the Model's prediction output over the
    /// data this ModelEvaluation uses. This field is populated only if the Model
    /// is evaluated with explanations, and only for AutoML tabular Models.
    ///
    #[prost(message, optional, tag = "8")]
    pub model_explanation: ::core::option::Option<ModelExplanation>,
    /// Output only. Describes the values of [ExplanationSpec][google.cloud.aiplatform.v1beta1.ExplanationSpec] that are used for explaining
    /// the predicted values on the evaluated data.
    #[prost(message, repeated, tag = "9")]
    pub explanation_specs:
        ::prost::alloc::vec::Vec<model_evaluation::ModelEvaluationExplanationSpec>,
}
/// Nested message and enum types in `ModelEvaluation`.
pub mod model_evaluation {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelEvaluationExplanationSpec {
        /// Explanation type.
        ///
        /// For AutoML Image Classification models, possible values are:
        ///
        ///   * `image-integrated-gradients`
        ///   * `image-xrai`
        #[prost(string, tag = "1")]
        pub explanation_type: ::prost::alloc::string::String,
        /// Explanation spec details.
        #[prost(message, optional, tag = "2")]
        pub explanation_spec: ::core::option::Option<super::ExplanationSpec>,
    }
}
/// A collection of metrics calculated by comparing Model's predictions on a
/// slice of the test data against ground truth annotations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelEvaluationSlice {
    /// Output only. The resource name of the ModelEvaluationSlice.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The slice of the test data that is used to evaluate the Model.
    #[prost(message, optional, tag = "2")]
    pub slice: ::core::option::Option<model_evaluation_slice::Slice>,
    /// Output only. Points to a YAML file stored on Google Cloud Storage describing the
    /// [metrics][google.cloud.aiplatform.v1beta1.ModelEvaluationSlice.metrics] of this ModelEvaluationSlice. The
    /// schema is defined as an OpenAPI 3.0.2
    /// [Schema Object](https://tinyurl.com/y538mdwt#schema-object).
    #[prost(string, tag = "3")]
    pub metrics_schema_uri: ::prost::alloc::string::String,
    /// Output only. Sliced evaluation metrics of the Model. The schema of the metrics is stored
    /// in [metrics_schema_uri][google.cloud.aiplatform.v1beta1.ModelEvaluationSlice.metrics_schema_uri]
    #[prost(message, optional, tag = "4")]
    pub metrics: ::core::option::Option<::prost_types::Value>,
    /// Output only. Timestamp when this ModelEvaluationSlice was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `ModelEvaluationSlice`.
pub mod model_evaluation_slice {
    /// Definition of a slice.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Slice {
        /// Output only. The dimension of the slice.
        /// Well-known dimensions are:
        ///   * `annotationSpec`: This slice is on the test data that has either
        ///     ground truth or prediction with [AnnotationSpec.display_name][google.cloud.aiplatform.v1beta1.AnnotationSpec.display_name]
        ///     equals to [value][google.cloud.aiplatform.v1beta1.ModelEvaluationSlice.Slice.value].
        #[prost(string, tag = "1")]
        pub dimension: ::prost::alloc::string::String,
        /// Output only. The value of the dimension in this slice.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
    }
}
/// Request message for [ModelService.UploadModel][google.cloud.aiplatform.v1beta1.ModelService.UploadModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadModelRequest {
    /// Required. The resource name of the Location into which to upload the Model.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Model to create.
    #[prost(message, optional, tag = "2")]
    pub model: ::core::option::Option<Model>,
}
/// Details of [ModelService.UploadModel][google.cloud.aiplatform.v1beta1.ModelService.UploadModel] operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadModelOperationMetadata {
    /// The common part of the operation metadata.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
}
/// Response message of [ModelService.UploadModel][google.cloud.aiplatform.v1beta1.ModelService.UploadModel] operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadModelResponse {
    /// The name of the uploaded Model resource.
    /// Format: `projects/{project}/locations/{location}/models/{model}`
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
}
/// Request message for [ModelService.GetModel][google.cloud.aiplatform.v1beta1.ModelService.GetModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelRequest {
    /// Required. The name of the Model resource.
    /// Format: `projects/{project}/locations/{location}/models/{model}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [ModelService.ListModels][google.cloud.aiplatform.v1beta1.ModelService.ListModels].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// Required. The resource name of the Location to list the Models from.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// An expression for filtering the results of the request. For field names
    /// both snake_case and camelCase are supported.
    ///
    ///   * `model` supports = and !=. `model` represents the Model ID,
    ///     i.e. the last segment of the Model's [resource name][google.cloud.aiplatform.v1beta1.Model.name].
    ///   * `display_name` supports = and !=
    ///   * `labels` supports general map functions that is:
    ///     * `labels.key=value` - key:value equality
    ///     * `labels.key:* or labels:key - key existence
    ///     * A key including a space must be quoted. `labels."a key"`.
    ///
    /// Some examples:
    ///   * `model=1234`
    ///   * `displayName="myDisplayName"`
    ///   * `labels.myKey="myValue"`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    /// Typically obtained via
    /// [ListModelsResponse.next_page_token][google.cloud.aiplatform.v1beta1.ListModelsResponse.next_page_token] of the previous
    /// [ModelService.ListModels][google.cloud.aiplatform.v1beta1.ModelService.ListModels] call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for [ModelService.ListModels][google.cloud.aiplatform.v1beta1.ModelService.ListModels]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// List of Models in the requested page.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// A token to retrieve next page of results.
    /// Pass to [ListModelsRequest.page_token][google.cloud.aiplatform.v1beta1.ListModelsRequest.page_token] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [ModelService.UpdateModel][google.cloud.aiplatform.v1beta1.ModelService.UpdateModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateModelRequest {
    /// Required. The Model which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub model: ::core::option::Option<Model>,
    /// Required. The update mask applies to the resource.
    /// For the `FieldMask` definition, see
    /// [FieldMask](https://tinyurl.com/protobufs/google.protobuf#fieldmask).
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for [ModelService.DeleteModel][google.cloud.aiplatform.v1beta1.ModelService.DeleteModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteModelRequest {
    /// Required. The name of the Model resource to be deleted.
    /// Format: `projects/{project}/locations/{location}/models/{model}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [ModelService.ExportModel][google.cloud.aiplatform.v1beta1.ModelService.ExportModel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportModelRequest {
    /// Required. The resource name of the Model to export.
    /// Format: `projects/{project}/locations/{location}/models/{model}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The desired output location and configuration.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<export_model_request::OutputConfig>,
}
/// Nested message and enum types in `ExportModelRequest`.
pub mod export_model_request {
    /// Output configuration for the Model export.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputConfig {
        /// The ID of the format in which the Model must be exported. Each Model
        /// lists the [export formats it supports][google.cloud.aiplatform.v1beta1.Model.supported_export_formats].
        /// If no value is provided here, then the first from the list of the Model's
        /// supported formats is used by default.
        #[prost(string, tag = "1")]
        pub export_format_id: ::prost::alloc::string::String,
        /// The Cloud Storage location where the Model artifact is to be
        /// written to. Under the directory given as the destination a new one with
        /// name "`model-export-<model-display-name>-<timestamp-of-export-call>`",
        /// where timestamp is in YYYY-MM-DDThh:mm:ss.sssZ ISO-8601 format,
        /// will be created. Inside, the Model and any of its supporting files
        /// will be written.
        /// This field should only be set when the `exportableContent` field of the
        /// [Model.supported_export_formats] object contains `ARTIFACT`.
        #[prost(message, optional, tag = "3")]
        pub artifact_destination: ::core::option::Option<super::GcsDestination>,
        /// The Google Container Registry or Artifact Registry uri where the
        /// Model container image will be copied to.
        /// This field should only be set when the `exportableContent` field of the
        /// [Model.supported_export_formats] object contains `IMAGE`.
        #[prost(message, optional, tag = "4")]
        pub image_destination: ::core::option::Option<super::ContainerRegistryDestination>,
    }
}
/// Details of [ModelService.ExportModel][google.cloud.aiplatform.v1beta1.ModelService.ExportModel] operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportModelOperationMetadata {
    /// The common part of the operation metadata.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
    /// Output only. Information further describing the output of this Model export.
    #[prost(message, optional, tag = "2")]
    pub output_info: ::core::option::Option<export_model_operation_metadata::OutputInfo>,
}
/// Nested message and enum types in `ExportModelOperationMetadata`.
pub mod export_model_operation_metadata {
    /// Further describes the output of the ExportModel. Supplements
    /// [ExportModelRequest.OutputConfig][google.cloud.aiplatform.v1beta1.ExportModelRequest.OutputConfig].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OutputInfo {
        /// Output only. If the Model artifact is being exported to Google Cloud Storage this is
        /// the full path of the directory created, into which the Model files are
        /// being written to.
        #[prost(string, tag = "2")]
        pub artifact_output_uri: ::prost::alloc::string::String,
        /// Output only. If the Model image is being exported to Google Container Registry or
        /// Artifact Registry this is the full path of the image created.
        #[prost(string, tag = "3")]
        pub image_output_uri: ::prost::alloc::string::String,
    }
}
/// Response message of [ModelService.ExportModel][google.cloud.aiplatform.v1beta1.ModelService.ExportModel] operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportModelResponse {}
/// Request message for [ModelService.GetModelEvaluation][google.cloud.aiplatform.v1beta1.ModelService.GetModelEvaluation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelEvaluationRequest {
    /// Required. The name of the ModelEvaluation resource.
    /// Format:
    /// `projects/{project}/locations/{location}/models/{model}/evaluations/{evaluation}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [ModelService.ListModelEvaluations][google.cloud.aiplatform.v1beta1.ModelService.ListModelEvaluations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelEvaluationsRequest {
    /// Required. The resource name of the Model to list the ModelEvaluations from.
    /// Format: `projects/{project}/locations/{location}/models/{model}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard list filter.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    /// Typically obtained via
    /// [ListModelEvaluationsResponse.next_page_token][google.cloud.aiplatform.v1beta1.ListModelEvaluationsResponse.next_page_token] of the previous
    /// [ModelService.ListModelEvaluations][google.cloud.aiplatform.v1beta1.ModelService.ListModelEvaluations] call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for [ModelService.ListModelEvaluations][google.cloud.aiplatform.v1beta1.ModelService.ListModelEvaluations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelEvaluationsResponse {
    /// List of ModelEvaluations in the requested page.
    #[prost(message, repeated, tag = "1")]
    pub model_evaluations: ::prost::alloc::vec::Vec<ModelEvaluation>,
    /// A token to retrieve next page of results.
    /// Pass to [ListModelEvaluationsRequest.page_token][google.cloud.aiplatform.v1beta1.ListModelEvaluationsRequest.page_token] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [ModelService.GetModelEvaluationSlice][google.cloud.aiplatform.v1beta1.ModelService.GetModelEvaluationSlice].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelEvaluationSliceRequest {
    /// Required. The name of the ModelEvaluationSlice resource.
    /// Format:
    /// `projects/{project}/locations/{location}/models/{model}/evaluations/{evaluation}/slices/{slice}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [ModelService.ListModelEvaluationSlices][google.cloud.aiplatform.v1beta1.ModelService.ListModelEvaluationSlices].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelEvaluationSlicesRequest {
    /// Required. The resource name of the ModelEvaluation to list the ModelEvaluationSlices
    /// from. Format:
    /// `projects/{project}/locations/{location}/models/{model}/evaluations/{evaluation}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard list filter.
    ///
    ///   * `slice.dimension` - for =.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    /// Typically obtained via
    /// [ListModelEvaluationSlicesResponse.next_page_token][google.cloud.aiplatform.v1beta1.ListModelEvaluationSlicesResponse.next_page_token] of the previous
    /// [ModelService.ListModelEvaluationSlices][google.cloud.aiplatform.v1beta1.ModelService.ListModelEvaluationSlices] call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for [ModelService.ListModelEvaluationSlices][google.cloud.aiplatform.v1beta1.ModelService.ListModelEvaluationSlices].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelEvaluationSlicesResponse {
    /// List of ModelEvaluations in the requested page.
    #[prost(message, repeated, tag = "1")]
    pub model_evaluation_slices: ::prost::alloc::vec::Vec<ModelEvaluationSlice>,
    /// A token to retrieve next page of results.
    /// Pass to [ListModelEvaluationSlicesRequest.page_token][google.cloud.aiplatform.v1beta1.ListModelEvaluationSlicesRequest.page_token] to obtain that
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod model_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service for managing AI Platform's machine learning Models."]
    pub struct ModelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ModelServiceClient<T>
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
        #[doc = " Uploads a Model artifact into AI Platform."]
        pub async fn upload_model(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadModelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.ModelService/UploadModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a Model."]
        pub async fn get_model(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.ModelService/GetModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Models in a Location."]
        pub async fn list_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelsRequest>,
        ) -> Result<tonic::Response<super::ListModelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.ModelService/ListModels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a Model."]
        pub async fn update_model(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.ModelService/UpdateModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a Model."]
        #[doc = " Note: Model can only be deleted if there are no DeployedModels created"]
        #[doc = " from it."]
        pub async fn delete_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteModelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.ModelService/DeleteModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports a trained, exportable, Model to a location specified by the"]
        #[doc = " user. A Model is considered to be exportable if it has at least one"]
        #[doc = " [supported export format][google.cloud.aiplatform.v1beta1.Model.supported_export_formats]."]
        pub async fn export_model(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportModelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.ModelService/ExportModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a ModelEvaluation."]
        pub async fn get_model_evaluation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelEvaluationRequest>,
        ) -> Result<tonic::Response<super::ModelEvaluation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.ModelService/GetModelEvaluation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists ModelEvaluations in a Model."]
        pub async fn list_model_evaluations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelEvaluationsRequest>,
        ) -> Result<tonic::Response<super::ListModelEvaluationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.ModelService/ListModelEvaluations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a ModelEvaluationSlice."]
        pub async fn get_model_evaluation_slice(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModelEvaluationSliceRequest>,
        ) -> Result<tonic::Response<super::ModelEvaluationSlice>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.ModelService/GetModelEvaluationSlice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists ModelEvaluationSlices in a ModelEvaluation."]
        pub async fn list_model_evaluation_slices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelEvaluationSlicesRequest>,
        ) -> Result<tonic::Response<super::ListModelEvaluationSlicesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.ModelService/ListModelEvaluationSlices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ModelServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ModelServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ModelServiceClient {{ ... }}")
        }
    }
}
/// Request message for [PipelineService.CreateTrainingPipeline][google.cloud.aiplatform.v1beta1.PipelineService.CreateTrainingPipeline].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTrainingPipelineRequest {
    /// Required. The resource name of the Location to create the TrainingPipeline in.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The TrainingPipeline to create.
    #[prost(message, optional, tag = "2")]
    pub training_pipeline: ::core::option::Option<TrainingPipeline>,
}
/// Request message for [PipelineService.GetTrainingPipeline][google.cloud.aiplatform.v1beta1.PipelineService.GetTrainingPipeline].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTrainingPipelineRequest {
    /// Required. The name of the TrainingPipeline resource.
    /// Format:
    /// `projects/{project}/locations/{location}/trainingPipelines/{training_pipeline}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [PipelineService.ListTrainingPipelines][google.cloud.aiplatform.v1beta1.PipelineService.ListTrainingPipelines].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTrainingPipelinesRequest {
    /// Required. The resource name of the Location to list the TrainingPipelines from.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard list filter.
    /// Supported fields:
    ///
    ///   * `display_name` supports = and !=.
    ///
    ///   * `state` supports = and !=.
    ///
    /// Some examples of using the filter are:
    ///
    ///  * `state="PIPELINE_STATE_SUCCEEDED" AND display_name="my_pipeline"`
    ///
    ///  * `state="PIPELINE_STATE_RUNNING" OR display_name="my_pipeline"`
    ///
    ///  * `NOT display_name="my_pipeline"`
    ///
    ///  * `state="PIPELINE_STATE_FAILED"`
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    /// Typically obtained via
    /// [ListTrainingPipelinesResponse.next_page_token][google.cloud.aiplatform.v1beta1.ListTrainingPipelinesResponse.next_page_token] of the previous
    /// [PipelineService.ListTrainingPipelines][google.cloud.aiplatform.v1beta1.PipelineService.ListTrainingPipelines] call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for [PipelineService.ListTrainingPipelines][google.cloud.aiplatform.v1beta1.PipelineService.ListTrainingPipelines]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTrainingPipelinesResponse {
    /// List of TrainingPipelines in the requested page.
    #[prost(message, repeated, tag = "1")]
    pub training_pipelines: ::prost::alloc::vec::Vec<TrainingPipeline>,
    /// A token to retrieve the next page of results.
    /// Pass to [ListTrainingPipelinesRequest.page_token][google.cloud.aiplatform.v1beta1.ListTrainingPipelinesRequest.page_token] to obtain that page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [PipelineService.DeleteTrainingPipeline][google.cloud.aiplatform.v1beta1.PipelineService.DeleteTrainingPipeline].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTrainingPipelineRequest {
    /// Required. The name of the TrainingPipeline resource to be deleted.
    /// Format:
    /// `projects/{project}/locations/{location}/trainingPipelines/{training_pipeline}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [PipelineService.CancelTrainingPipeline][google.cloud.aiplatform.v1beta1.PipelineService.CancelTrainingPipeline].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTrainingPipelineRequest {
    /// Required. The name of the TrainingPipeline to cancel.
    /// Format:
    /// `projects/{project}/locations/{location}/trainingPipelines/{training_pipeline}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod pipeline_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service for creating and managing AI Platform's pipelines."]
    pub struct PipelineServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PipelineServiceClient<T>
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
        #[doc = " Creates a TrainingPipeline. A created TrainingPipeline right away will be"]
        #[doc = " attempted to be run."]
        pub async fn create_training_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTrainingPipelineRequest>,
        ) -> Result<tonic::Response<super::TrainingPipeline>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.PipelineService/CreateTrainingPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a TrainingPipeline."]
        pub async fn get_training_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTrainingPipelineRequest>,
        ) -> Result<tonic::Response<super::TrainingPipeline>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.PipelineService/GetTrainingPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists TrainingPipelines in a Location."]
        pub async fn list_training_pipelines(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTrainingPipelinesRequest>,
        ) -> Result<tonic::Response<super::ListTrainingPipelinesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.PipelineService/ListTrainingPipelines",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a TrainingPipeline."]
        pub async fn delete_training_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTrainingPipelineRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.PipelineService/DeleteTrainingPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels a TrainingPipeline."]
        #[doc = " Starts asynchronous cancellation on the TrainingPipeline. The server"]
        #[doc = " makes a best effort to cancel the pipeline, but success is not"]
        #[doc = " guaranteed. Clients can use [PipelineService.GetTrainingPipeline][google.cloud.aiplatform.v1beta1.PipelineService.GetTrainingPipeline] or"]
        #[doc = " other methods to check whether the cancellation succeeded or whether the"]
        #[doc = " pipeline completed despite cancellation. On successful cancellation,"]
        #[doc = " the TrainingPipeline is not deleted; instead it becomes a pipeline with"]
        #[doc = " a [TrainingPipeline.error][google.cloud.aiplatform.v1beta1.TrainingPipeline.error] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,"]
        #[doc = " corresponding to `Code.CANCELLED`, and [TrainingPipeline.state][google.cloud.aiplatform.v1beta1.TrainingPipeline.state] is set to"]
        #[doc = " `CANCELLED`."]
        pub async fn cancel_training_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelTrainingPipelineRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.PipelineService/CancelTrainingPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PipelineServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PipelineServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PipelineServiceClient {{ ... }}")
        }
    }
}
/// Request message for [PredictionService.Predict][google.cloud.aiplatform.v1beta1.PredictionService.Predict].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictRequest {
    /// Required. The name of the Endpoint requested to serve the prediction.
    /// Format:
    /// `projects/{project}/locations/{location}/endpoints/{endpoint}`
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    /// Required. The instances that are the input to the prediction call.
    /// A DeployedModel may have an upper limit on the number of instances it
    /// supports per request, and when it is exceeded the prediction call errors
    /// in case of AutoML Models, or, in case of customer created Models, the
    /// behaviour is as documented by that Model.
    /// The schema of any single instance may be specified via Endpoint's
    /// DeployedModels' [Model's][google.cloud.aiplatform.v1beta1.DeployedModel.model]
    /// [PredictSchemata's][google.cloud.aiplatform.v1beta1.Model.predict_schemata]
    /// [instance_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.instance_schema_uri].
    #[prost(message, repeated, tag = "2")]
    pub instances: ::prost::alloc::vec::Vec<::prost_types::Value>,
    /// The parameters that govern the prediction. The schema of the parameters may
    /// be specified via Endpoint's DeployedModels' [Model's ][google.cloud.aiplatform.v1beta1.DeployedModel.model]
    /// [PredictSchemata's][google.cloud.aiplatform.v1beta1.Model.predict_schemata]
    /// [parameters_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.parameters_schema_uri].
    #[prost(message, optional, tag = "3")]
    pub parameters: ::core::option::Option<::prost_types::Value>,
}
/// Response message for [PredictionService.Predict][google.cloud.aiplatform.v1beta1.PredictionService.Predict].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictResponse {
    /// The predictions that are the output of the predictions call.
    /// The schema of any single prediction may be specified via Endpoint's
    /// DeployedModels' [Model's ][google.cloud.aiplatform.v1beta1.DeployedModel.model]
    /// [PredictSchemata's][google.cloud.aiplatform.v1beta1.Model.predict_schemata]
    /// [prediction_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.prediction_schema_uri].
    #[prost(message, repeated, tag = "1")]
    pub predictions: ::prost::alloc::vec::Vec<::prost_types::Value>,
    /// ID of the Endpoint's DeployedModel that served this prediction.
    #[prost(string, tag = "2")]
    pub deployed_model_id: ::prost::alloc::string::String,
}
/// Request message for [PredictionService.Explain][google.cloud.aiplatform.v1beta1.PredictionService.Explain].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainRequest {
    /// Required. The name of the Endpoint requested to serve the explanation.
    /// Format:
    /// `projects/{project}/locations/{location}/endpoints/{endpoint}`
    #[prost(string, tag = "1")]
    pub endpoint: ::prost::alloc::string::String,
    /// Required. The instances that are the input to the explanation call.
    /// A DeployedModel may have an upper limit on the number of instances it
    /// supports per request, and when it is exceeded the explanation call errors
    /// in case of AutoML Models, or, in case of customer created Models, the
    /// behaviour is as documented by that Model.
    /// The schema of any single instance may be specified via Endpoint's
    /// DeployedModels' [Model's][google.cloud.aiplatform.v1beta1.DeployedModel.model]
    /// [PredictSchemata's][google.cloud.aiplatform.v1beta1.Model.predict_schemata]
    /// [instance_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.instance_schema_uri].
    #[prost(message, repeated, tag = "2")]
    pub instances: ::prost::alloc::vec::Vec<::prost_types::Value>,
    /// The parameters that govern the prediction. The schema of the parameters may
    /// be specified via Endpoint's DeployedModels' [Model's ][google.cloud.aiplatform.v1beta1.DeployedModel.model]
    /// [PredictSchemata's][google.cloud.aiplatform.v1beta1.Model.predict_schemata]
    /// [parameters_schema_uri][google.cloud.aiplatform.v1beta1.PredictSchemata.parameters_schema_uri].
    #[prost(message, optional, tag = "4")]
    pub parameters: ::core::option::Option<::prost_types::Value>,
    /// If specified, overrides the
    /// [explanation_spec][google.cloud.aiplatform.v1beta1.DeployedModel.explanation_spec] of the DeployedModel.
    /// Can be used for explaining prediction results with different
    /// configurations, such as:
    ///  - Explaining top-5 predictions results as opposed to top-1;
    ///  - Increasing path count or step count of the attribution methods to reduce
    ///    approximate errors;
    ///  - Using different baselines for explaining the prediction results.
    #[prost(message, optional, tag = "5")]
    pub explanation_spec_override: ::core::option::Option<ExplanationSpecOverride>,
    /// If specified, this ExplainRequest will be served by the chosen
    /// DeployedModel, overriding [Endpoint.traffic_split][google.cloud.aiplatform.v1beta1.Endpoint.traffic_split].
    #[prost(string, tag = "3")]
    pub deployed_model_id: ::prost::alloc::string::String,
}
/// Response message for [PredictionService.Explain][google.cloud.aiplatform.v1beta1.PredictionService.Explain].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplainResponse {
    /// The explanations of the Model's [PredictResponse.predictions][google.cloud.aiplatform.v1beta1.PredictResponse.predictions].
    ///
    /// It has the same number of elements as [instances][google.cloud.aiplatform.v1beta1.ExplainRequest.instances]
    /// to be explained.
    #[prost(message, repeated, tag = "1")]
    pub explanations: ::prost::alloc::vec::Vec<Explanation>,
    /// ID of the Endpoint's DeployedModel that served this explanation.
    #[prost(string, tag = "2")]
    pub deployed_model_id: ::prost::alloc::string::String,
    /// The predictions that are the output of the predictions call.
    /// Same as [PredictResponse.predictions][google.cloud.aiplatform.v1beta1.PredictResponse.predictions].
    #[prost(message, repeated, tag = "3")]
    pub predictions: ::prost::alloc::vec::Vec<::prost_types::Value>,
}
#[doc = r" Generated client implementations."]
pub mod prediction_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service for online predictions and explanations."]
    pub struct PredictionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PredictionServiceClient<T>
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
        #[doc = " Perform an online prediction."]
        pub async fn predict(
            &mut self,
            request: impl tonic::IntoRequest<super::PredictRequest>,
        ) -> Result<tonic::Response<super::PredictResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.PredictionService/Predict",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Perform an online explanation."]
        #[doc = ""]
        #[doc = " If [deployed_model_id][google.cloud.aiplatform.v1beta1.ExplainRequest.deployed_model_id] is specified,"]
        #[doc = " the corresponding DeployModel must have"]
        #[doc = " [explanation_spec][google.cloud.aiplatform.v1beta1.DeployedModel.explanation_spec]"]
        #[doc = " populated. If [deployed_model_id][google.cloud.aiplatform.v1beta1.ExplainRequest.deployed_model_id]"]
        #[doc = " is not specified, all DeployedModels must have"]
        #[doc = " [explanation_spec][google.cloud.aiplatform.v1beta1.DeployedModel.explanation_spec]"]
        #[doc = " populated. Only deployed AutoML tabular Models have"]
        #[doc = " explanation_spec."]
        pub async fn explain(
            &mut self,
            request: impl tonic::IntoRequest<super::ExplainRequest>,
        ) -> Result<tonic::Response<super::ExplainResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.PredictionService/Explain",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PredictionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PredictionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PredictionServiceClient {{ ... }}")
        }
    }
}
/// Request message for [SpecialistPoolService.CreateSpecialistPool][google.cloud.aiplatform.v1beta1.SpecialistPoolService.CreateSpecialistPool].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpecialistPoolRequest {
    /// Required. The parent Project name for the new SpecialistPool.
    /// The form is `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The SpecialistPool to create.
    #[prost(message, optional, tag = "2")]
    pub specialist_pool: ::core::option::Option<SpecialistPool>,
}
/// Runtime operation information for
/// [SpecialistPoolService.CreateSpecialistPool][google.cloud.aiplatform.v1beta1.SpecialistPoolService.CreateSpecialistPool].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSpecialistPoolOperationMetadata {
    /// The operation generic information.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
}
/// Request message for [SpecialistPoolService.GetSpecialistPool][google.cloud.aiplatform.v1beta1.SpecialistPoolService.GetSpecialistPool].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSpecialistPoolRequest {
    /// Required. The name of the SpecialistPool resource.
    /// The form is
    /// `projects/{project}/locations/{location}/specialistPools/{specialist_pool}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [SpecialistPoolService.ListSpecialistPools][google.cloud.aiplatform.v1beta1.SpecialistPoolService.ListSpecialistPools].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecialistPoolsRequest {
    /// Required. The name of the SpecialistPool's parent resource.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The standard list page token.
    /// Typically obtained by [ListSpecialistPoolsResponse.next_page_token][google.cloud.aiplatform.v1beta1.ListSpecialistPoolsResponse.next_page_token] of
    /// the previous [SpecialistPoolService.ListSpecialistPools][google.cloud.aiplatform.v1beta1.SpecialistPoolService.ListSpecialistPools] call. Return
    /// first page if empty.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Mask specifying which fields to read. FieldMask represents a set of
    #[prost(message, optional, tag = "4")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for [SpecialistPoolService.ListSpecialistPools][google.cloud.aiplatform.v1beta1.SpecialistPoolService.ListSpecialistPools].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecialistPoolsResponse {
    /// A list of SpecialistPools that matches the specified filter in the request.
    #[prost(message, repeated, tag = "1")]
    pub specialist_pools: ::prost::alloc::vec::Vec<SpecialistPool>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [SpecialistPoolService.DeleteSpecialistPool][google.cloud.aiplatform.v1beta1.SpecialistPoolService.DeleteSpecialistPool].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSpecialistPoolRequest {
    /// Required. The resource name of the SpecialistPool to delete. Format:
    /// `projects/{project}/locations/{location}/specialistPools/{specialist_pool}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, any specialist managers in this SpecialistPool will also be
    /// deleted. (Otherwise, the request will only work if the SpecialistPool has
    /// no specialist managers.)
    #[prost(bool, tag = "2")]
    pub force: bool,
}
/// Request message for [SpecialistPoolService.UpdateSpecialistPool][google.cloud.aiplatform.v1beta1.SpecialistPoolService.UpdateSpecialistPool].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSpecialistPoolRequest {
    /// Required. The SpecialistPool which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub specialist_pool: ::core::option::Option<SpecialistPool>,
    /// Required. The update mask applies to the resource.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Runtime operation metadata for
/// [SpecialistPoolService.UpdateSpecialistPool][google.cloud.aiplatform.v1beta1.SpecialistPoolService.UpdateSpecialistPool].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSpecialistPoolOperationMetadata {
    /// Output only. The name of the SpecialistPool to which the specialists are being added.
    /// Format:
    /// `projects/{project_id}/locations/{location_id}/specialistPools/{specialist_pool}`
    #[prost(string, tag = "1")]
    pub specialist_pool: ::prost::alloc::string::String,
    /// The operation generic information.
    #[prost(message, optional, tag = "2")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
}
#[doc = r" Generated client implementations."]
pub mod specialist_pool_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service for creating and managing Customer SpecialistPools."]
    #[doc = " When customers start Data Labeling jobs, they can reuse/create Specialist"]
    #[doc = " Pools to bring their own Specialists to label the data."]
    #[doc = " Customers can add/remove Managers for the Specialist Pool on Cloud console,"]
    #[doc = " then Managers will get email notifications to manage Specialists and tasks on"]
    #[doc = " CrowdCompute console."]
    pub struct SpecialistPoolServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SpecialistPoolServiceClient<T>
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
        #[doc = " Creates a SpecialistPool."]
        pub async fn create_specialist_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSpecialistPoolRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.SpecialistPoolService/CreateSpecialistPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a SpecialistPool."]
        pub async fn get_specialist_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSpecialistPoolRequest>,
        ) -> Result<tonic::Response<super::SpecialistPool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.SpecialistPoolService/GetSpecialistPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists SpecialistPools in a Location."]
        pub async fn list_specialist_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSpecialistPoolsRequest>,
        ) -> Result<tonic::Response<super::ListSpecialistPoolsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.SpecialistPoolService/ListSpecialistPools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a SpecialistPool as well as all Specialists in the pool."]
        pub async fn delete_specialist_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSpecialistPoolRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.SpecialistPoolService/DeleteSpecialistPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a SpecialistPool."]
        pub async fn update_specialist_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSpecialistPoolRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.SpecialistPoolService/UpdateSpecialistPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SpecialistPoolServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SpecialistPoolServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SpecialistPoolServiceClient {{ ... }}")
        }
    }
}
/// Request message for [VizierService.GetStudy][google.cloud.aiplatform.v1beta1.VizierService.GetStudy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStudyRequest {
    /// Required. The name of the Study resource.
    /// Format: `projects/{project}/locations/{location}/studies/{study}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [VizierService.CreateStudy][google.cloud.aiplatform.v1beta1.VizierService.CreateStudy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStudyRequest {
    /// Required. The resource name of the Location to create the CustomJob in.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Study configuration used to create the Study.
    #[prost(message, optional, tag = "2")]
    pub study: ::core::option::Option<Study>,
}
/// Request message for [VizierService.ListStudies][google.cloud.aiplatform.v1beta1.VizierService.ListStudies].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStudiesRequest {
    /// Required. The resource name of the Location to list the Study from.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. A page token to request the next page of results.
    /// If unspecified, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The maximum number of studies to return per "page" of results.
    /// If unspecified, service will pick an appropriate default.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for [VizierService.ListStudies][google.cloud.aiplatform.v1beta1.VizierService.ListStudies].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStudiesResponse {
    /// The studies associated with the project.
    #[prost(message, repeated, tag = "1")]
    pub studies: ::prost::alloc::vec::Vec<Study>,
    /// Passes this token as the `page_token` field of the request for a
    /// subsequent call.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [VizierService.DeleteStudy][google.cloud.aiplatform.v1beta1.VizierService.DeleteStudy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteStudyRequest {
    /// Required. The name of the Study resource to be deleted.
    /// Format: `projects/{project}/locations/{location}/studies/{study}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [VizierService.LookupStudy][google.cloud.aiplatform.v1beta1.VizierService.LookupStudy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupStudyRequest {
    /// Required. The resource name of the Location to get the Study from.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The user-defined display name of the Study
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
}
/// Request message for [VizierService.SuggestTrials][google.cloud.aiplatform.v1beta1.VizierService.SuggestTrials].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestTrialsRequest {
    /// Required. The project and location that the Study belongs to.
    /// Format: `projects/{project}/locations/{location}/studies/{study}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The number of suggestions requested.
    #[prost(int32, tag = "2")]
    pub suggestion_count: i32,
    /// Required. The identifier of the client that is requesting the suggestion.
    ///
    /// If multiple SuggestTrialsRequests have the same `client_id`,
    /// the service will return the identical suggested Trial if the Trial is
    /// pending, and provide a new Trial if the last suggested Trial was completed.
    #[prost(string, tag = "3")]
    pub client_id: ::prost::alloc::string::String,
}
/// Response message for [VizierService.SuggestTrials][google.cloud.aiplatform.v1beta1.VizierService.SuggestTrials].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestTrialsResponse {
    /// A list of Trials.
    #[prost(message, repeated, tag = "1")]
    pub trials: ::prost::alloc::vec::Vec<Trial>,
    /// The state of the Study.
    #[prost(enumeration = "study::State", tag = "2")]
    pub study_state: i32,
    /// The time at which the operation was started.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time at which operation processing completed.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Details of operations that perform Trials suggestion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestTrialsMetadata {
    /// Operation metadata for suggesting Trials.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
    /// The identifier of the client that is requesting the suggestion.
    ///
    /// If multiple SuggestTrialsRequests have the same `client_id`,
    /// the service will return the identical suggested Trial if the Trial is
    /// pending, and provide a new Trial if the last suggested Trial was completed.
    #[prost(string, tag = "2")]
    pub client_id: ::prost::alloc::string::String,
}
/// Request message for [VizierService.CreateTrial][google.cloud.aiplatform.v1beta1.VizierService.CreateTrial].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTrialRequest {
    /// Required. The resource name of the Study to create the Trial in.
    /// Format: `projects/{project}/locations/{location}/studies/{study}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Trial to create.
    #[prost(message, optional, tag = "2")]
    pub trial: ::core::option::Option<Trial>,
}
/// Request message for [VizierService.GetTrial][google.cloud.aiplatform.v1beta1.VizierService.GetTrial].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTrialRequest {
    /// Required. The name of the Trial resource.
    /// Format:
    /// `projects/{project}/locations/{location}/studies/{study}/trials/{trial}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [VizierService.ListTrials][google.cloud.aiplatform.v1beta1.VizierService.ListTrials].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTrialsRequest {
    /// Required. The resource name of the Study to list the Trial from.
    /// Format: `projects/{project}/locations/{location}/studies/{study}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. A page token to request the next page of results.
    /// If unspecified, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The number of Trials to retrieve per "page" of results.
    /// If unspecified, the service will pick an appropriate default.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for [VizierService.ListTrials][google.cloud.aiplatform.v1beta1.VizierService.ListTrials].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTrialsResponse {
    /// The Trials associated with the Study.
    #[prost(message, repeated, tag = "1")]
    pub trials: ::prost::alloc::vec::Vec<Trial>,
    /// Pass this token as the `page_token` field of the request for a
    /// subsequent call.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for [VizierService.AddTrialMeasurement][google.cloud.aiplatform.v1beta1.VizierService.AddTrialMeasurement].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTrialMeasurementRequest {
    /// Required. The name of the trial to add measurement.
    /// Format:
    /// `projects/{project}/locations/{location}/studies/{study}/trials/{trial}`
    #[prost(string, tag = "1")]
    pub trial_name: ::prost::alloc::string::String,
    /// Required. The measurement to be added to a Trial.
    #[prost(message, optional, tag = "3")]
    pub measurement: ::core::option::Option<Measurement>,
}
/// Request message for [VizierService.CompleteTrial][google.cloud.aiplatform.v1beta1.VizierService.CompleteTrial].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteTrialRequest {
    /// Required. The Trial's name.
    /// Format:
    /// `projects/{project}/locations/{location}/studies/{study}/trials/{trial}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. If provided, it will be used as the completed Trial's
    /// final_measurement; Otherwise, the service will auto-select a
    /// previously reported measurement as the final-measurement
    #[prost(message, optional, tag = "2")]
    pub final_measurement: ::core::option::Option<Measurement>,
    /// Optional. True if the Trial cannot be run with the given Parameter, and
    /// final_measurement will be ignored.
    #[prost(bool, tag = "3")]
    pub trial_infeasible: bool,
    /// Optional. A human readable reason why the trial was infeasible. This should
    /// only be provided if `trial_infeasible` is true.
    #[prost(string, tag = "4")]
    pub infeasible_reason: ::prost::alloc::string::String,
}
/// Request message for [VizierService.DeleteTrial][google.cloud.aiplatform.v1beta1.VizierService.DeleteTrial].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTrialRequest {
    /// Required. The Trial's name.
    /// Format:
    /// `projects/{project}/locations/{location}/studies/{study}/trials/{trial}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [VizierService.CheckTrialEarlyStoppingState][google.cloud.aiplatform.v1beta1.VizierService.CheckTrialEarlyStoppingState].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckTrialEarlyStoppingStateRequest {
    /// Required. The Trial's name.
    /// Format:
    /// `projects/{project}/locations/{location}/studies/{study}/trials/{trial}`
    #[prost(string, tag = "1")]
    pub trial_name: ::prost::alloc::string::String,
}
/// Response message for [VizierService.CheckTrialEarlyStoppingState][google.cloud.aiplatform.v1beta1.VizierService.CheckTrialEarlyStoppingState].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckTrialEarlyStoppingStateResponse {
    /// True if the Trial should stop.
    #[prost(bool, tag = "1")]
    pub should_stop: bool,
}
/// This message will be placed in the metadata field of a
/// google.longrunning.Operation associated with a CheckTrialEarlyStoppingState
/// request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckTrialEarlyStoppingStateMetatdata {
    /// Operation metadata for suggesting Trials.
    #[prost(message, optional, tag = "1")]
    pub generic_metadata: ::core::option::Option<GenericOperationMetadata>,
    /// The name of the Study that the Trial belongs to.
    #[prost(string, tag = "2")]
    pub study: ::prost::alloc::string::String,
    /// The Trial name.
    #[prost(string, tag = "3")]
    pub trial: ::prost::alloc::string::String,
}
/// Request message for [VizierService.StopTrial][google.cloud.aiplatform.v1beta1.VizierService.StopTrial].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopTrialRequest {
    /// Required. The Trial's name.
    /// Format:
    /// `projects/{project}/locations/{location}/studies/{study}/trials/{trial}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [VizierService.ListOptimalTrials][google.cloud.aiplatform.v1beta1.VizierService.ListOptimalTrials].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOptimalTrialsRequest {
    /// Required. The name of the Study that the optimal Trial belongs to.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response message for [VizierService.ListOptimalTrials][google.cloud.aiplatform.v1beta1.VizierService.ListOptimalTrials].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOptimalTrialsResponse {
    /// The pareto-optimal Trials for multiple objective Study or the
    /// optimal trial for single objective Study. The definition of
    /// pareto-optimal can be checked in wiki page.
    /// https://en.wikipedia.org/wiki/Pareto_efficiency
    #[prost(message, repeated, tag = "1")]
    pub optimal_trials: ::prost::alloc::vec::Vec<Trial>,
}
#[doc = r" Generated client implementations."]
pub mod vizier_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Cloud AI Platform Vizier API."]
    #[doc = ""]
    #[doc = " Vizier service is a GCP service to solve blackbox optimization problems,"]
    #[doc = " such as tuning machine learning hyperparameters and searching over deep"]
    #[doc = " learning architectures."]
    pub struct VizierServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VizierServiceClient<T>
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
        #[doc = " Creates a Study. A resource name will be generated after creation of the"]
        #[doc = " Study."]
        pub async fn create_study(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStudyRequest>,
        ) -> Result<tonic::Response<super::Study>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/CreateStudy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a Study by name."]
        pub async fn get_study(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStudyRequest>,
        ) -> Result<tonic::Response<super::Study>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/GetStudy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all the studies in a region for an associated project."]
        pub async fn list_studies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStudiesRequest>,
        ) -> Result<tonic::Response<super::ListStudiesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/ListStudies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a Study."]
        pub async fn delete_study(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStudyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/DeleteStudy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Looks a study up using the user-defined display_name field instead of the"]
        #[doc = " fully qualified resource name."]
        pub async fn lookup_study(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupStudyRequest>,
        ) -> Result<tonic::Response<super::Study>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/LookupStudy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds one or more Trials to a Study, with parameter values"]
        #[doc = " suggested by AI Platform Vizier. Returns a long-running"]
        #[doc = " operation associated with the generation of Trial suggestions."]
        #[doc = " When this long-running operation succeeds, it will contain"]
        #[doc = " a [SuggestTrialsResponse][google.cloud.ml.v1.SuggestTrialsResponse]."]
        pub async fn suggest_trials(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestTrialsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/SuggestTrials",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds a user provided Trial to a Study."]
        pub async fn create_trial(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTrialRequest>,
        ) -> Result<tonic::Response<super::Trial>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/CreateTrial",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a Trial."]
        pub async fn get_trial(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTrialRequest>,
        ) -> Result<tonic::Response<super::Trial>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/GetTrial",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the Trials associated with a Study."]
        pub async fn list_trials(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTrialsRequest>,
        ) -> Result<tonic::Response<super::ListTrialsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/ListTrials",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds a measurement of the objective metrics to a Trial. This measurement"]
        #[doc = " is assumed to have been taken before the Trial is complete."]
        pub async fn add_trial_measurement(
            &mut self,
            request: impl tonic::IntoRequest<super::AddTrialMeasurementRequest>,
        ) -> Result<tonic::Response<super::Trial>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/AddTrialMeasurement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Marks a Trial as complete."]
        pub async fn complete_trial(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteTrialRequest>,
        ) -> Result<tonic::Response<super::Trial>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/CompleteTrial",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a Trial."]
        pub async fn delete_trial(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTrialRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/DeleteTrial",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Checks  whether a Trial should stop or not. Returns a"]
        #[doc = " long-running operation. When the operation is successful,"]
        #[doc = " it will contain a"]
        #[doc = " [CheckTrialEarlyStoppingStateResponse][google.cloud.ml.v1.CheckTrialEarlyStoppingStateResponse]."]
        pub async fn check_trial_early_stopping_state(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckTrialEarlyStoppingStateRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/CheckTrialEarlyStoppingState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops a Trial."]
        pub async fn stop_trial(
            &mut self,
            request: impl tonic::IntoRequest<super::StopTrialRequest>,
        ) -> Result<tonic::Response<super::Trial>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/StopTrial",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the pareto-optimal Trials for multi-objective Study or the"]
        #[doc = " optimal Trials for single-objective Study. The definition of"]
        #[doc = " pareto-optimal can be checked in wiki page."]
        #[doc = " https://en.wikipedia.org/wiki/Pareto_efficiency"]
        pub async fn list_optimal_trials(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOptimalTrialsRequest>,
        ) -> Result<tonic::Response<super::ListOptimalTrialsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.aiplatform.v1beta1.VizierService/ListOptimalTrials",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VizierServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VizierServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VizierServiceClient {{ ... }}")
        }
    }
}
