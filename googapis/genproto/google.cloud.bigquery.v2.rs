#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionConfiguration {
    /// Optional. Describes the Cloud KMS encryption key that will be used to
    /// protect destination BigQuery table. The BigQuery Service Account associated
    /// with your project requires access to this encryption key.
    #[prost(message, optional, tag = "1")]
    pub kms_key_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Id path of a model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelReference {
    /// Required. The ID of the project containing this model.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The ID of the dataset containing this model.
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Required. The ID of the model. The ID must contain only
    /// letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum
    /// length is 1,024 characters.
    #[prost(string, tag = "3")]
    pub model_id: ::prost::alloc::string::String,
}
/// The type of a variable, e.g., a function argument.
/// Examples:
/// INT64: {type_kind="INT64"}
/// ARRAY<STRING>: {type_kind="ARRAY", array_element_type="STRING"}
/// STRUCT<x STRING, y ARRAY<DATE>>:
///   {type_kind="STRUCT",
///    struct_type={fields=[
///      {name="x", type={type_kind="STRING"}},
///      {name="y", type={type_kind="ARRAY", array_element_type="DATE"}}
///    ]}}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardSqlDataType {
    /// Required. The top level type of this field.
    /// Can be any standard SQL data type (e.g., "INT64", "DATE", "ARRAY").
    #[prost(enumeration = "standard_sql_data_type::TypeKind", tag = "1")]
    pub type_kind: i32,
    #[prost(oneof = "standard_sql_data_type::SubType", tags = "2, 3")]
    pub sub_type: ::core::option::Option<standard_sql_data_type::SubType>,
}
/// Nested message and enum types in `StandardSqlDataType`.
pub mod standard_sql_data_type {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TypeKind {
        /// Invalid type.
        Unspecified = 0,
        /// Encoded as a string in decimal format.
        Int64 = 2,
        /// Encoded as a boolean "false" or "true".
        Bool = 5,
        /// Encoded as a number, or string "NaN", "Infinity" or "-Infinity".
        Float64 = 7,
        /// Encoded as a string value.
        String = 8,
        /// Encoded as a base64 string per RFC 4648, section 4.
        Bytes = 9,
        /// Encoded as an RFC 3339 timestamp with mandatory "Z" time zone string:
        /// 1985-04-12T23:20:50.52Z
        Timestamp = 19,
        /// Encoded as RFC 3339 full-date format string: 1985-04-12
        Date = 10,
        /// Encoded as RFC 3339 partial-time format string: 23:20:50.52
        Time = 20,
        /// Encoded as RFC 3339 full-date "T" partial-time: 1985-04-12T23:20:50.52
        Datetime = 21,
        /// Encoded as fully qualified 3 part: 0-5 15 2:30:45.6
        Interval = 26,
        /// Encoded as WKT
        Geography = 22,
        /// Encoded as a decimal string.
        Numeric = 23,
        /// Encoded as a decimal string.
        Bignumeric = 24,
        /// Encoded as a string.
        Json = 25,
        /// Encoded as a list with types matching Type.array_type.
        Array = 16,
        /// Encoded as a list with fields of type Type.struct_type\[i\]. List is used
        /// because a JSON object cannot have duplicate field names.
        Struct = 17,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SubType {
        /// The type of the array's elements, if type_kind = "ARRAY".
        #[prost(message, tag = "2")]
        ArrayElementType(::prost::alloc::boxed::Box<super::StandardSqlDataType>),
        /// The fields of this struct, in order, if type_kind = "STRUCT".
        #[prost(message, tag = "3")]
        StructType(super::StandardSqlStructType),
    }
}
/// A field or a column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardSqlField {
    /// Optional. The name of this field. Can be absent for struct fields.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The type of this parameter. Absent if not explicitly
    /// specified (e.g., CREATE FUNCTION statement can omit the return type;
    /// in this case the output parameter does not have this "type" field).
    #[prost(message, optional, tag = "2")]
    pub r#type: ::core::option::Option<StandardSqlDataType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardSqlStructType {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<StandardSqlField>,
}
/// A table type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardSqlTableType {
    /// The columns in this table type
    #[prost(message, repeated, tag = "1")]
    pub columns: ::prost::alloc::vec::Vec<StandardSqlField>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableReference {
    /// Required. The ID of the project containing this table.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The ID of the dataset containing this table.
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Required. The ID of the table. The ID must contain only
    /// letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum
    /// length is 1,024 characters.  Certain operations allow
    /// suffixing of the table ID with a partition decorator, such as
    /// `sample_table$20190123`.
    #[prost(string, tag = "3")]
    pub table_id: ::prost::alloc::string::String,
    /// The alternative field that will be used when ESF is not able to translate
    /// the received data to the project_id field.
    #[prost(string, repeated, tag = "4")]
    pub project_id_alternative: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The alternative field that will be used when ESF is not able to translate
    /// the received data to the project_id field.
    #[prost(string, repeated, tag = "5")]
    pub dataset_id_alternative: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The alternative field that will be used when ESF is not able to translate
    /// the received data to the project_id field.
    #[prost(string, repeated, tag = "6")]
    pub table_id_alternative: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// Output only. A hash of this resource.
    #[prost(string, tag = "1")]
    pub etag: ::prost::alloc::string::String,
    /// Required. Unique identifier for this model.
    #[prost(message, optional, tag = "2")]
    pub model_reference: ::core::option::Option<ModelReference>,
    /// Output only. The time when this model was created, in millisecs since the epoch.
    #[prost(int64, tag = "5")]
    pub creation_time: i64,
    /// Output only. The time when this model was last modified, in millisecs since the epoch.
    #[prost(int64, tag = "6")]
    pub last_modified_time: i64,
    /// Optional. A user-friendly description of this model.
    #[prost(string, tag = "12")]
    pub description: ::prost::alloc::string::String,
    /// Optional. A descriptive name for this model.
    #[prost(string, tag = "14")]
    pub friendly_name: ::prost::alloc::string::String,
    /// The labels associated with this model. You can use these to organize
    /// and group your models. Label keys and values can be no longer
    /// than 63 characters, can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// Label values are optional. Label keys must start with a letter and each
    /// label in the list must have a different key.
    #[prost(map = "string, string", tag = "15")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. The time when this model expires, in milliseconds since the epoch.
    /// If not present, the model will persist indefinitely. Expired models
    /// will be deleted and their storage reclaimed.  The defaultTableExpirationMs
    /// property of the encapsulating dataset can be used to set a default
    /// expirationTime on newly created models.
    #[prost(int64, tag = "16")]
    pub expiration_time: i64,
    /// Output only. The geographic location where the model resides. This value
    /// is inherited from the dataset.
    #[prost(string, tag = "13")]
    pub location: ::prost::alloc::string::String,
    /// Custom encryption configuration (e.g., Cloud KMS keys). This shows the
    /// encryption configuration of the model data while stored in BigQuery
    /// storage. This field can be used with PatchModel to update encryption key
    /// for an already encrypted model.
    #[prost(message, optional, tag = "17")]
    pub encryption_configuration: ::core::option::Option<EncryptionConfiguration>,
    /// Output only. Type of the model resource.
    #[prost(enumeration = "model::ModelType", tag = "7")]
    pub model_type: i32,
    /// Output only. Information for all training runs in increasing order of start_time.
    #[prost(message, repeated, tag = "9")]
    pub training_runs: ::prost::alloc::vec::Vec<model::TrainingRun>,
    /// Output only. Input feature columns that were used to train this model.
    #[prost(message, repeated, tag = "10")]
    pub feature_columns: ::prost::alloc::vec::Vec<StandardSqlField>,
    /// Output only. Label columns that were used to train this model.
    /// The output of the model will have a "predicted_" prefix to these columns.
    #[prost(message, repeated, tag = "11")]
    pub label_columns: ::prost::alloc::vec::Vec<StandardSqlField>,
    /// The best trial_id across all training runs.
    #[deprecated]
    #[prost(int64, tag = "19")]
    pub best_trial_id: i64,
}
/// Nested message and enum types in `Model`.
pub mod model {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SeasonalPeriod {}
    /// Nested message and enum types in `SeasonalPeriod`.
    pub mod seasonal_period {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum SeasonalPeriodType {
            Unspecified = 0,
            /// No seasonality
            NoSeasonality = 1,
            /// Daily period, 24 hours.
            Daily = 2,
            /// Weekly period, 7 days.
            Weekly = 3,
            /// Monthly period, 30 days or irregular.
            Monthly = 4,
            /// Quarterly period, 90 days or irregular.
            Quarterly = 5,
            /// Yearly period, 365 days or irregular.
            Yearly = 6,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KmeansEnums {}
    /// Nested message and enum types in `KmeansEnums`.
    pub mod kmeans_enums {
        /// Indicates the method used to initialize the centroids for KMeans
        /// clustering algorithm.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum KmeansInitializationMethod {
            /// Unspecified initialization method.
            Unspecified = 0,
            /// Initializes the centroids randomly.
            Random = 1,
            /// Initializes the centroids using data specified in
            /// kmeans_initialization_column.
            Custom = 2,
            /// Initializes with kmeans++.
            KmeansPlusPlus = 3,
        }
    }
    /// Evaluation metrics for regression and explicit feedback type matrix
    /// factorization models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RegressionMetrics {
        /// Mean absolute error.
        #[prost(message, optional, tag = "1")]
        pub mean_absolute_error: ::core::option::Option<f64>,
        /// Mean squared error.
        #[prost(message, optional, tag = "2")]
        pub mean_squared_error: ::core::option::Option<f64>,
        /// Mean squared log error.
        #[prost(message, optional, tag = "3")]
        pub mean_squared_log_error: ::core::option::Option<f64>,
        /// Median absolute error.
        #[prost(message, optional, tag = "4")]
        pub median_absolute_error: ::core::option::Option<f64>,
        /// R^2 score. This corresponds to r2_score in ML.EVALUATE.
        #[prost(message, optional, tag = "5")]
        pub r_squared: ::core::option::Option<f64>,
    }
    /// Aggregate metrics for classification/classifier models. For multi-class
    /// models, the metrics are either macro-averaged or micro-averaged. When
    /// macro-averaged, the metrics are calculated for each label and then an
    /// unweighted average is taken of those values. When micro-averaged, the
    /// metric is calculated globally by counting the total number of correctly
    /// predicted rows.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AggregateClassificationMetrics {
        /// Precision is the fraction of actual positive predictions that had
        /// positive actual labels. For multiclass this is a macro-averaged
        /// metric treating each class as a binary classifier.
        #[prost(message, optional, tag = "1")]
        pub precision: ::core::option::Option<f64>,
        /// Recall is the fraction of actual positive labels that were given a
        /// positive prediction. For multiclass this is a macro-averaged metric.
        #[prost(message, optional, tag = "2")]
        pub recall: ::core::option::Option<f64>,
        /// Accuracy is the fraction of predictions given the correct label. For
        /// multiclass this is a micro-averaged metric.
        #[prost(message, optional, tag = "3")]
        pub accuracy: ::core::option::Option<f64>,
        /// Threshold at which the metrics are computed. For binary
        /// classification models this is the positive class threshold.
        /// For multi-class classfication models this is the confidence
        /// threshold.
        #[prost(message, optional, tag = "4")]
        pub threshold: ::core::option::Option<f64>,
        /// The F1 score is an average of recall and precision. For multiclass
        /// this is a macro-averaged metric.
        #[prost(message, optional, tag = "5")]
        pub f1_score: ::core::option::Option<f64>,
        /// Logarithmic Loss. For multiclass this is a macro-averaged metric.
        #[prost(message, optional, tag = "6")]
        pub log_loss: ::core::option::Option<f64>,
        /// Area Under a ROC Curve. For multiclass this is a macro-averaged
        /// metric.
        #[prost(message, optional, tag = "7")]
        pub roc_auc: ::core::option::Option<f64>,
    }
    /// Evaluation metrics for binary classification/classifier models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BinaryClassificationMetrics {
        /// Aggregate classification metrics.
        #[prost(message, optional, tag = "1")]
        pub aggregate_classification_metrics:
            ::core::option::Option<AggregateClassificationMetrics>,
        /// Binary confusion matrix at multiple thresholds.
        #[prost(message, repeated, tag = "2")]
        pub binary_confusion_matrix_list:
            ::prost::alloc::vec::Vec<binary_classification_metrics::BinaryConfusionMatrix>,
        /// Label representing the positive class.
        #[prost(string, tag = "3")]
        pub positive_label: ::prost::alloc::string::String,
        /// Label representing the negative class.
        #[prost(string, tag = "4")]
        pub negative_label: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `BinaryClassificationMetrics`.
    pub mod binary_classification_metrics {
        /// Confusion matrix for binary classification models.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BinaryConfusionMatrix {
            /// Threshold value used when computing each of the following metric.
            #[prost(message, optional, tag = "1")]
            pub positive_class_threshold: ::core::option::Option<f64>,
            /// Number of true samples predicted as true.
            #[prost(message, optional, tag = "2")]
            pub true_positives: ::core::option::Option<i64>,
            /// Number of false samples predicted as true.
            #[prost(message, optional, tag = "3")]
            pub false_positives: ::core::option::Option<i64>,
            /// Number of true samples predicted as false.
            #[prost(message, optional, tag = "4")]
            pub true_negatives: ::core::option::Option<i64>,
            /// Number of false samples predicted as false.
            #[prost(message, optional, tag = "5")]
            pub false_negatives: ::core::option::Option<i64>,
            /// The fraction of actual positive predictions that had positive actual
            /// labels.
            #[prost(message, optional, tag = "6")]
            pub precision: ::core::option::Option<f64>,
            /// The fraction of actual positive labels that were given a positive
            /// prediction.
            #[prost(message, optional, tag = "7")]
            pub recall: ::core::option::Option<f64>,
            /// The equally weighted average of recall and precision.
            #[prost(message, optional, tag = "8")]
            pub f1_score: ::core::option::Option<f64>,
            /// The fraction of predictions given the correct label.
            #[prost(message, optional, tag = "9")]
            pub accuracy: ::core::option::Option<f64>,
        }
    }
    /// Evaluation metrics for multi-class classification/classifier models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultiClassClassificationMetrics {
        /// Aggregate classification metrics.
        #[prost(message, optional, tag = "1")]
        pub aggregate_classification_metrics:
            ::core::option::Option<AggregateClassificationMetrics>,
        /// Confusion matrix at different thresholds.
        #[prost(message, repeated, tag = "2")]
        pub confusion_matrix_list:
            ::prost::alloc::vec::Vec<multi_class_classification_metrics::ConfusionMatrix>,
    }
    /// Nested message and enum types in `MultiClassClassificationMetrics`.
    pub mod multi_class_classification_metrics {
        /// Confusion matrix for multi-class classification models.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConfusionMatrix {
            /// Confidence threshold used when computing the entries of the
            /// confusion matrix.
            #[prost(message, optional, tag = "1")]
            pub confidence_threshold: ::core::option::Option<f64>,
            /// One row per actual label.
            #[prost(message, repeated, tag = "2")]
            pub rows: ::prost::alloc::vec::Vec<confusion_matrix::Row>,
        }
        /// Nested message and enum types in `ConfusionMatrix`.
        pub mod confusion_matrix {
            /// A single entry in the confusion matrix.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Entry {
                /// The predicted label. For confidence_threshold > 0, we will
                /// also add an entry indicating the number of items under the
                /// confidence threshold.
                #[prost(string, tag = "1")]
                pub predicted_label: ::prost::alloc::string::String,
                /// Number of items being predicted as this label.
                #[prost(message, optional, tag = "2")]
                pub item_count: ::core::option::Option<i64>,
            }
            /// A single row in the confusion matrix.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Row {
                /// The original label of this row.
                #[prost(string, tag = "1")]
                pub actual_label: ::prost::alloc::string::String,
                /// Info describing predicted label distribution.
                #[prost(message, repeated, tag = "2")]
                pub entries: ::prost::alloc::vec::Vec<Entry>,
            }
        }
    }
    /// Evaluation metrics for clustering models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusteringMetrics {
        /// Davies-Bouldin index.
        #[prost(message, optional, tag = "1")]
        pub davies_bouldin_index: ::core::option::Option<f64>,
        /// Mean of squared distances between each sample to its cluster centroid.
        #[prost(message, optional, tag = "2")]
        pub mean_squared_distance: ::core::option::Option<f64>,
        /// Information for all clusters.
        #[prost(message, repeated, tag = "3")]
        pub clusters: ::prost::alloc::vec::Vec<clustering_metrics::Cluster>,
    }
    /// Nested message and enum types in `ClusteringMetrics`.
    pub mod clustering_metrics {
        /// Message containing the information about one cluster.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Cluster {
            /// Centroid id.
            #[prost(int64, tag = "1")]
            pub centroid_id: i64,
            /// Values of highly variant features for this cluster.
            #[prost(message, repeated, tag = "2")]
            pub feature_values: ::prost::alloc::vec::Vec<cluster::FeatureValue>,
            /// Count of training data rows that were assigned to this cluster.
            #[prost(message, optional, tag = "3")]
            pub count: ::core::option::Option<i64>,
        }
        /// Nested message and enum types in `Cluster`.
        pub mod cluster {
            /// Representative value of a single feature within the cluster.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct FeatureValue {
                /// The feature column name.
                #[prost(string, tag = "1")]
                pub feature_column: ::prost::alloc::string::String,
                #[prost(oneof = "feature_value::Value", tags = "2, 3")]
                pub value: ::core::option::Option<feature_value::Value>,
            }
            /// Nested message and enum types in `FeatureValue`.
            pub mod feature_value {
                /// Representative value of a categorical feature.
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct CategoricalValue {
                    /// Counts of all categories for the categorical feature. If there are
                    /// more than ten categories, we return top ten (by count) and return
                    /// one more CategoryCount with category "_OTHER_" and count as
                    /// aggregate counts of remaining categories.
                    #[prost(message, repeated, tag = "1")]
                    pub category_counts: ::prost::alloc::vec::Vec<categorical_value::CategoryCount>,
                }
                /// Nested message and enum types in `CategoricalValue`.
                pub mod categorical_value {
                    /// Represents the count of a single category within the cluster.
                    #[derive(Clone, PartialEq, ::prost::Message)]
                    pub struct CategoryCount {
                        /// The name of category.
                        #[prost(string, tag = "1")]
                        pub category: ::prost::alloc::string::String,
                        /// The count of training samples matching the category within the
                        /// cluster.
                        #[prost(message, optional, tag = "2")]
                        pub count: ::core::option::Option<i64>,
                    }
                }
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Value {
                    /// The numerical feature value. This is the centroid value for this
                    /// feature.
                    #[prost(message, tag = "2")]
                    NumericalValue(f64),
                    /// The categorical feature value.
                    #[prost(message, tag = "3")]
                    CategoricalValue(CategoricalValue),
                }
            }
        }
    }
    /// Evaluation metrics used by weighted-ALS models specified by
    /// feedback_type=implicit.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RankingMetrics {
        /// Calculates a precision per user for all the items by ranking them and
        /// then averages all the precisions across all the users.
        #[prost(message, optional, tag = "1")]
        pub mean_average_precision: ::core::option::Option<f64>,
        /// Similar to the mean squared error computed in regression and explicit
        /// recommendation models except instead of computing the rating directly,
        /// the output from evaluate is computed against a preference which is 1 or 0
        /// depending on if the rating exists or not.
        #[prost(message, optional, tag = "2")]
        pub mean_squared_error: ::core::option::Option<f64>,
        /// A metric to determine the goodness of a ranking calculated from the
        /// predicted confidence by comparing it to an ideal rank measured by the
        /// original ratings.
        #[prost(message, optional, tag = "3")]
        pub normalized_discounted_cumulative_gain: ::core::option::Option<f64>,
        /// Determines the goodness of a ranking by computing the percentile rank
        /// from the predicted confidence and dividing it by the original rank.
        #[prost(message, optional, tag = "4")]
        pub average_rank: ::core::option::Option<f64>,
    }
    /// Model evaluation metrics for ARIMA forecasting models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ArimaForecastingMetrics {
        /// Non-seasonal order.
        #[deprecated]
        #[prost(message, repeated, tag = "1")]
        pub non_seasonal_order: ::prost::alloc::vec::Vec<ArimaOrder>,
        /// Arima model fitting metrics.
        #[deprecated]
        #[prost(message, repeated, tag = "2")]
        pub arima_fitting_metrics: ::prost::alloc::vec::Vec<ArimaFittingMetrics>,
        /// Seasonal periods. Repeated because multiple periods are supported for one
        /// time series.
        #[deprecated]
        #[prost(
            enumeration = "seasonal_period::SeasonalPeriodType",
            repeated,
            packed = "false",
            tag = "3"
        )]
        pub seasonal_periods: ::prost::alloc::vec::Vec<i32>,
        /// Whether Arima model fitted with drift or not. It is always false when d
        /// is not 1.
        #[deprecated]
        #[prost(bool, repeated, packed = "false", tag = "4")]
        pub has_drift: ::prost::alloc::vec::Vec<bool>,
        /// Id to differentiate different time series for the large-scale case.
        #[deprecated]
        #[prost(string, repeated, tag = "5")]
        pub time_series_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Repeated as there can be many metric sets (one for each model) in
        /// auto-arima and the large-scale case.
        #[prost(message, repeated, tag = "6")]
        pub arima_single_model_forecasting_metrics:
            ::prost::alloc::vec::Vec<arima_forecasting_metrics::ArimaSingleModelForecastingMetrics>,
    }
    /// Nested message and enum types in `ArimaForecastingMetrics`.
    pub mod arima_forecasting_metrics {
        /// Model evaluation metrics for a single ARIMA forecasting model.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ArimaSingleModelForecastingMetrics {
            /// Non-seasonal order.
            #[prost(message, optional, tag = "1")]
            pub non_seasonal_order: ::core::option::Option<super::ArimaOrder>,
            /// Arima fitting metrics.
            #[prost(message, optional, tag = "2")]
            pub arima_fitting_metrics: ::core::option::Option<super::ArimaFittingMetrics>,
            /// Is arima model fitted with drift or not. It is always false when d
            /// is not 1.
            #[prost(bool, tag = "3")]
            pub has_drift: bool,
            /// The time_series_id value for this time series. It will be one of
            /// the unique values from the time_series_id_column specified during
            /// ARIMA model training. Only present when time_series_id_column
            /// training option was used.
            #[prost(string, tag = "4")]
            pub time_series_id: ::prost::alloc::string::String,
            /// The tuple of time_series_ids identifying this time series. It will
            /// be one of the unique tuples of values present in the
            /// time_series_id_columns specified during ARIMA model training. Only
            /// present when time_series_id_columns training option was used and
            /// the order of values here are same as the order of
            /// time_series_id_columns.
            #[prost(string, repeated, tag = "9")]
            pub time_series_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Seasonal periods. Repeated because multiple periods are supported
            /// for one time series.
            #[prost(
                enumeration = "super::seasonal_period::SeasonalPeriodType",
                repeated,
                tag = "5"
            )]
            pub seasonal_periods: ::prost::alloc::vec::Vec<i32>,
            /// If true, holiday_effect is a part of time series decomposition result.
            #[prost(message, optional, tag = "6")]
            pub has_holiday_effect: ::core::option::Option<bool>,
            /// If true, spikes_and_dips is a part of time series decomposition result.
            #[prost(message, optional, tag = "7")]
            pub has_spikes_and_dips: ::core::option::Option<bool>,
            /// If true, step_changes is a part of time series decomposition result.
            #[prost(message, optional, tag = "8")]
            pub has_step_changes: ::core::option::Option<bool>,
        }
    }
    /// Evaluation metrics of a model. These are either computed on all training
    /// data or just the eval data based on whether eval data was used during
    /// training. These are not present for imported models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EvaluationMetrics {
        #[prost(oneof = "evaluation_metrics::Metrics", tags = "1, 2, 3, 4, 5, 6")]
        pub metrics: ::core::option::Option<evaluation_metrics::Metrics>,
    }
    /// Nested message and enum types in `EvaluationMetrics`.
    pub mod evaluation_metrics {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Metrics {
            /// Populated for regression models and explicit feedback type matrix
            /// factorization models.
            #[prost(message, tag = "1")]
            RegressionMetrics(super::RegressionMetrics),
            /// Populated for binary classification/classifier models.
            #[prost(message, tag = "2")]
            BinaryClassificationMetrics(super::BinaryClassificationMetrics),
            /// Populated for multi-class classification/classifier models.
            #[prost(message, tag = "3")]
            MultiClassClassificationMetrics(super::MultiClassClassificationMetrics),
            /// Populated for clustering models.
            #[prost(message, tag = "4")]
            ClusteringMetrics(super::ClusteringMetrics),
            /// Populated for implicit feedback type matrix factorization models.
            #[prost(message, tag = "5")]
            RankingMetrics(super::RankingMetrics),
            /// Populated for ARIMA models.
            #[prost(message, tag = "6")]
            ArimaForecastingMetrics(super::ArimaForecastingMetrics),
        }
    }
    /// Data split result. This contains references to the training and evaluation
    /// data tables that were used to train the model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataSplitResult {
        /// Table reference of the training data after split.
        #[prost(message, optional, tag = "1")]
        pub training_table: ::core::option::Option<super::TableReference>,
        /// Table reference of the evaluation data after split.
        #[prost(message, optional, tag = "2")]
        pub evaluation_table: ::core::option::Option<super::TableReference>,
    }
    /// Arima order, can be used for both non-seasonal and seasonal parts.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ArimaOrder {
        /// Order of the autoregressive part.
        #[prost(int64, tag = "1")]
        pub p: i64,
        /// Order of the differencing part.
        #[prost(int64, tag = "2")]
        pub d: i64,
        /// Order of the moving-average part.
        #[prost(int64, tag = "3")]
        pub q: i64,
    }
    /// ARIMA model fitting metrics.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ArimaFittingMetrics {
        /// Log-likelihood.
        #[prost(double, tag = "1")]
        pub log_likelihood: f64,
        /// AIC.
        #[prost(double, tag = "2")]
        pub aic: f64,
        /// Variance.
        #[prost(double, tag = "3")]
        pub variance: f64,
    }
    /// Global explanations containing the top most important features
    /// after training.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GlobalExplanation {
        /// A list of the top global explanations. Sorted by absolute value of
        /// attribution in descending order.
        #[prost(message, repeated, tag = "1")]
        pub explanations: ::prost::alloc::vec::Vec<global_explanation::Explanation>,
        /// Class label for this set of global explanations. Will be empty/null for
        /// binary logistic and linear regression models. Sorted alphabetically in
        /// descending order.
        #[prost(string, tag = "2")]
        pub class_label: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `GlobalExplanation`.
    pub mod global_explanation {
        /// Explanation for a single feature.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Explanation {
            /// Full name of the feature. For non-numerical features, will be
            /// formatted like <column_name>.<encoded_feature_name>. Overall size of
            /// feature name will always be truncated to first 120 characters.
            #[prost(string, tag = "1")]
            pub feature_name: ::prost::alloc::string::String,
            /// Attribution of feature.
            #[prost(message, optional, tag = "2")]
            pub attribution: ::core::option::Option<f64>,
        }
    }
    /// Information about a single training query run for the model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrainingRun {
        /// Options that were used for this training run, includes
        /// user specified and default options that were used.
        #[prost(message, optional, tag = "1")]
        pub training_options: ::core::option::Option<training_run::TrainingOptions>,
        /// The start time of this training run.
        #[prost(message, optional, tag = "8")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Output of each iteration run, results.size() <= max_iterations.
        #[prost(message, repeated, tag = "6")]
        pub results: ::prost::alloc::vec::Vec<training_run::IterationResult>,
        /// The evaluation metrics over training/eval data that were computed at the
        /// end of training.
        #[prost(message, optional, tag = "7")]
        pub evaluation_metrics: ::core::option::Option<EvaluationMetrics>,
        /// Data split result of the training run. Only set when the input data is
        /// actually split.
        #[prost(message, optional, tag = "9")]
        pub data_split_result: ::core::option::Option<DataSplitResult>,
        /// Global explanations for important features of the model. For multi-class
        /// models, there is one entry for each label class. For other models, there
        /// is only one entry in the list.
        #[prost(message, repeated, tag = "10")]
        pub global_explanations: ::prost::alloc::vec::Vec<GlobalExplanation>,
    }
    /// Nested message and enum types in `TrainingRun`.
    pub mod training_run {
        /// Options used in model training.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TrainingOptions {
            /// The maximum number of iterations in training. Used only for iterative
            /// training algorithms.
            #[prost(int64, tag = "1")]
            pub max_iterations: i64,
            /// Type of loss function used during training run.
            #[prost(enumeration = "super::LossType", tag = "2")]
            pub loss_type: i32,
            /// Learning rate in training. Used only for iterative training algorithms.
            #[prost(double, tag = "3")]
            pub learn_rate: f64,
            /// L1 regularization coefficient.
            #[prost(message, optional, tag = "4")]
            pub l1_regularization: ::core::option::Option<f64>,
            /// L2 regularization coefficient.
            #[prost(message, optional, tag = "5")]
            pub l2_regularization: ::core::option::Option<f64>,
            /// When early_stop is true, stops training when accuracy improvement is
            /// less than 'min_relative_progress'. Used only for iterative training
            /// algorithms.
            #[prost(message, optional, tag = "6")]
            pub min_relative_progress: ::core::option::Option<f64>,
            /// Whether to train a model from the last checkpoint.
            #[prost(message, optional, tag = "7")]
            pub warm_start: ::core::option::Option<bool>,
            /// Whether to stop early when the loss doesn't improve significantly
            /// any more (compared to min_relative_progress). Used only for iterative
            /// training algorithms.
            #[prost(message, optional, tag = "8")]
            pub early_stop: ::core::option::Option<bool>,
            /// Name of input label columns in training data.
            #[prost(string, repeated, tag = "9")]
            pub input_label_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// The data split type for training and evaluation, e.g. RANDOM.
            #[prost(enumeration = "super::DataSplitMethod", tag = "10")]
            pub data_split_method: i32,
            /// The fraction of evaluation data over the whole input data. The rest
            /// of data will be used as training data. The format should be double.
            /// Accurate to two decimal places.
            /// Default value is 0.2.
            #[prost(double, tag = "11")]
            pub data_split_eval_fraction: f64,
            /// The column to split data with. This column won't be used as a
            /// feature.
            /// 1. When data_split_method is CUSTOM, the corresponding column should
            /// be boolean. The rows with true value tag are eval data, and the false
            /// are training data.
            /// 2. When data_split_method is SEQ, the first DATA_SPLIT_EVAL_FRACTION
            /// rows (from smallest to largest) in the corresponding column are used
            /// as training data, and the rest are eval data. It respects the order
            /// in Orderable data types:
            /// <https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#data-type-properties>
            #[prost(string, tag = "12")]
            pub data_split_column: ::prost::alloc::string::String,
            /// The strategy to determine learn rate for the current iteration.
            #[prost(enumeration = "super::LearnRateStrategy", tag = "13")]
            pub learn_rate_strategy: i32,
            /// Specifies the initial learning rate for the line search learn rate
            /// strategy.
            #[prost(double, tag = "16")]
            pub initial_learn_rate: f64,
            /// Weights associated with each label class, for rebalancing the
            /// training data. Only applicable for classification models.
            #[prost(map = "string, double", tag = "17")]
            pub label_class_weights:
                ::std::collections::HashMap<::prost::alloc::string::String, f64>,
            /// User column specified for matrix factorization models.
            #[prost(string, tag = "18")]
            pub user_column: ::prost::alloc::string::String,
            /// Item column specified for matrix factorization models.
            #[prost(string, tag = "19")]
            pub item_column: ::prost::alloc::string::String,
            /// Distance type for clustering models.
            #[prost(enumeration = "super::DistanceType", tag = "20")]
            pub distance_type: i32,
            /// Number of clusters for clustering models.
            #[prost(int64, tag = "21")]
            pub num_clusters: i64,
            /// Google Cloud Storage URI from which the model was imported. Only
            /// applicable for imported models.
            #[prost(string, tag = "22")]
            pub model_uri: ::prost::alloc::string::String,
            /// Optimization strategy for training linear regression models.
            #[prost(enumeration = "super::OptimizationStrategy", tag = "23")]
            pub optimization_strategy: i32,
            /// Hidden units for dnn models.
            #[prost(int64, repeated, tag = "24")]
            pub hidden_units: ::prost::alloc::vec::Vec<i64>,
            /// Batch size for dnn models.
            #[prost(int64, tag = "25")]
            pub batch_size: i64,
            /// Dropout probability for dnn models.
            #[prost(message, optional, tag = "26")]
            pub dropout: ::core::option::Option<f64>,
            /// Maximum depth of a tree for boosted tree models.
            #[prost(int64, tag = "27")]
            pub max_tree_depth: i64,
            /// Subsample fraction of the training data to grow tree to prevent
            /// overfitting for boosted tree models.
            #[prost(double, tag = "28")]
            pub subsample: f64,
            /// Minimum split loss for boosted tree models.
            #[prost(message, optional, tag = "29")]
            pub min_split_loss: ::core::option::Option<f64>,
            /// Num factors specified for matrix factorization models.
            #[prost(int64, tag = "30")]
            pub num_factors: i64,
            /// Feedback type that specifies which algorithm to run for matrix
            /// factorization.
            #[prost(enumeration = "super::FeedbackType", tag = "31")]
            pub feedback_type: i32,
            /// Hyperparameter for matrix factoration when implicit feedback type is
            /// specified.
            #[prost(message, optional, tag = "32")]
            pub wals_alpha: ::core::option::Option<f64>,
            /// The method used to initialize the centroids for kmeans algorithm.
            #[prost(
                enumeration = "super::kmeans_enums::KmeansInitializationMethod",
                tag = "33"
            )]
            pub kmeans_initialization_method: i32,
            /// The column used to provide the initial centroids for kmeans algorithm
            /// when kmeans_initialization_method is CUSTOM.
            #[prost(string, tag = "34")]
            pub kmeans_initialization_column: ::prost::alloc::string::String,
            /// Column to be designated as time series timestamp for ARIMA model.
            #[prost(string, tag = "35")]
            pub time_series_timestamp_column: ::prost::alloc::string::String,
            /// Column to be designated as time series data for ARIMA model.
            #[prost(string, tag = "36")]
            pub time_series_data_column: ::prost::alloc::string::String,
            /// Whether to enable auto ARIMA or not.
            #[prost(bool, tag = "37")]
            pub auto_arima: bool,
            /// A specification of the non-seasonal part of the ARIMA model: the three
            /// components (p, d, q) are the AR order, the degree of differencing, and
            /// the MA order.
            #[prost(message, optional, tag = "38")]
            pub non_seasonal_order: ::core::option::Option<super::ArimaOrder>,
            /// The data frequency of a time series.
            #[prost(enumeration = "super::DataFrequency", tag = "39")]
            pub data_frequency: i32,
            /// Include drift when fitting an ARIMA model.
            #[prost(bool, tag = "41")]
            pub include_drift: bool,
            /// The geographical region based on which the holidays are considered in
            /// time series modeling. If a valid value is specified, then holiday
            /// effects modeling is enabled.
            #[prost(enumeration = "super::HolidayRegion", tag = "42")]
            pub holiday_region: i32,
            /// The time series id column that was used during ARIMA model training.
            #[prost(string, tag = "43")]
            pub time_series_id_column: ::prost::alloc::string::String,
            /// The time series id columns that were used during ARIMA model training.
            #[prost(string, repeated, tag = "51")]
            pub time_series_id_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// The number of periods ahead that need to be forecasted.
            #[prost(int64, tag = "44")]
            pub horizon: i64,
            /// Whether to preserve the input structs in output feature names.
            /// Suppose there is a struct A with field b.
            /// When false (default), the output feature name is A_b.
            /// When true, the output feature name is A.b.
            #[prost(bool, tag = "45")]
            pub preserve_input_structs: bool,
            /// The max value of non-seasonal p and q.
            #[prost(int64, tag = "46")]
            pub auto_arima_max_order: i64,
            /// If true, perform decompose time series and save the results.
            #[prost(message, optional, tag = "50")]
            pub decompose_time_series: ::core::option::Option<bool>,
            /// If true, clean spikes and dips in the input time series.
            #[prost(message, optional, tag = "52")]
            pub clean_spikes_and_dips: ::core::option::Option<bool>,
            /// If true, detect step changes and make data adjustment in the input time
            /// series.
            #[prost(message, optional, tag = "53")]
            pub adjust_step_changes: ::core::option::Option<bool>,
        }
        /// Information about a single iteration of the training run.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IterationResult {
            /// Index of the iteration, 0 based.
            #[prost(message, optional, tag = "1")]
            pub index: ::core::option::Option<i32>,
            /// Time taken to run the iteration in milliseconds.
            #[prost(message, optional, tag = "4")]
            pub duration_ms: ::core::option::Option<i64>,
            /// Loss computed on the training data at the end of iteration.
            #[prost(message, optional, tag = "5")]
            pub training_loss: ::core::option::Option<f64>,
            /// Loss computed on the eval data at the end of iteration.
            #[prost(message, optional, tag = "6")]
            pub eval_loss: ::core::option::Option<f64>,
            /// Learn rate used for this iteration.
            #[prost(double, tag = "7")]
            pub learn_rate: f64,
            /// Information about top clusters for clustering models.
            #[prost(message, repeated, tag = "8")]
            pub cluster_infos: ::prost::alloc::vec::Vec<iteration_result::ClusterInfo>,
            #[prost(message, optional, tag = "9")]
            pub arima_result: ::core::option::Option<iteration_result::ArimaResult>,
        }
        /// Nested message and enum types in `IterationResult`.
        pub mod iteration_result {
            /// Information about a single cluster for clustering model.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ClusterInfo {
                /// Centroid id.
                #[prost(int64, tag = "1")]
                pub centroid_id: i64,
                /// Cluster radius, the average distance from centroid
                /// to each point assigned to the cluster.
                #[prost(message, optional, tag = "2")]
                pub cluster_radius: ::core::option::Option<f64>,
                /// Cluster size, the total number of points assigned to the cluster.
                #[prost(message, optional, tag = "3")]
                pub cluster_size: ::core::option::Option<i64>,
            }
            /// (Auto-)arima fitting result. Wrap everything in ArimaResult for easier
            /// refactoring if we want to use model-specific iteration results.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ArimaResult {
                /// This message is repeated because there are multiple arima models
                /// fitted in auto-arima. For non-auto-arima model, its size is one.
                #[prost(message, repeated, tag = "1")]
                pub arima_model_info: ::prost::alloc::vec::Vec<arima_result::ArimaModelInfo>,
                /// Seasonal periods. Repeated because multiple periods are supported for
                /// one time series.
                #[prost(
                    enumeration = "super::super::seasonal_period::SeasonalPeriodType",
                    repeated,
                    tag = "2"
                )]
                pub seasonal_periods: ::prost::alloc::vec::Vec<i32>,
            }
            /// Nested message and enum types in `ArimaResult`.
            pub mod arima_result {
                /// Arima coefficients.
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct ArimaCoefficients {
                    /// Auto-regressive coefficients, an array of double.
                    #[prost(double, repeated, tag = "1")]
                    pub auto_regressive_coefficients: ::prost::alloc::vec::Vec<f64>,
                    /// Moving-average coefficients, an array of double.
                    #[prost(double, repeated, tag = "2")]
                    pub moving_average_coefficients: ::prost::alloc::vec::Vec<f64>,
                    /// Intercept coefficient, just a double not an array.
                    #[prost(double, tag = "3")]
                    pub intercept_coefficient: f64,
                }
                /// Arima model information.
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct ArimaModelInfo {
                    /// Non-seasonal order.
                    #[prost(message, optional, tag = "1")]
                    pub non_seasonal_order: ::core::option::Option<super::super::super::ArimaOrder>,
                    /// Arima coefficients.
                    #[prost(message, optional, tag = "2")]
                    pub arima_coefficients: ::core::option::Option<ArimaCoefficients>,
                    /// Arima fitting metrics.
                    #[prost(message, optional, tag = "3")]
                    pub arima_fitting_metrics:
                        ::core::option::Option<super::super::super::ArimaFittingMetrics>,
                    /// Whether Arima model fitted with drift or not. It is always false
                    /// when d is not 1.
                    #[prost(bool, tag = "4")]
                    pub has_drift: bool,
                    /// The time_series_id value for this time series. It will be one of
                    /// the unique values from the time_series_id_column specified during
                    /// ARIMA model training. Only present when time_series_id_column
                    /// training option was used.
                    #[prost(string, tag = "5")]
                    pub time_series_id: ::prost::alloc::string::String,
                    /// The tuple of time_series_ids identifying this time series. It will
                    /// be one of the unique tuples of values present in the
                    /// time_series_id_columns specified during ARIMA model training. Only
                    /// present when time_series_id_columns training option was used and
                    /// the order of values here are same as the order of
                    /// time_series_id_columns.
                    #[prost(string, repeated, tag = "10")]
                    pub time_series_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                    /// Seasonal periods. Repeated because multiple periods are supported
                    /// for one time series.
                    #[prost(
                        enumeration = "super::super::super::seasonal_period::SeasonalPeriodType",
                        repeated,
                        tag = "6"
                    )]
                    pub seasonal_periods: ::prost::alloc::vec::Vec<i32>,
                    /// If true, holiday_effect is a part of time series decomposition
                    /// result.
                    #[prost(message, optional, tag = "7")]
                    pub has_holiday_effect: ::core::option::Option<bool>,
                    /// If true, spikes_and_dips is a part of time series decomposition
                    /// result.
                    #[prost(message, optional, tag = "8")]
                    pub has_spikes_and_dips: ::core::option::Option<bool>,
                    /// If true, step_changes is a part of time series decomposition
                    /// result.
                    #[prost(message, optional, tag = "9")]
                    pub has_step_changes: ::core::option::Option<bool>,
                }
            }
        }
    }
    /// Indicates the type of the Model.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ModelType {
        Unspecified = 0,
        /// Linear regression model.
        LinearRegression = 1,
        /// Logistic regression based classification model.
        LogisticRegression = 2,
        /// K-means clustering model.
        Kmeans = 3,
        /// Matrix factorization model.
        MatrixFactorization = 4,
        /// DNN classifier model.
        DnnClassifier = 5,
        /// An imported TensorFlow model.
        Tensorflow = 6,
        /// DNN regressor model.
        DnnRegressor = 7,
        /// Boosted tree regressor model.
        BoostedTreeRegressor = 9,
        /// Boosted tree classifier model.
        BoostedTreeClassifier = 10,
        /// ARIMA model.
        Arima = 11,
        /// \[Beta\] AutoML Tables regression model.
        AutomlRegressor = 12,
        /// \[Beta\] AutoML Tables classification model.
        AutomlClassifier = 13,
        /// New name for the ARIMA model.
        ArimaPlus = 19,
    }
    /// Loss metric to evaluate model training performance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LossType {
        Unspecified = 0,
        /// Mean squared loss, used for linear regression.
        MeanSquaredLoss = 1,
        /// Mean log loss, used for logistic regression.
        MeanLogLoss = 2,
    }
    /// Distance metric used to compute the distance between two points.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DistanceType {
        Unspecified = 0,
        /// Eculidean distance.
        Euclidean = 1,
        /// Cosine distance.
        Cosine = 2,
    }
    /// Indicates the method to split input data into multiple tables.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataSplitMethod {
        Unspecified = 0,
        /// Splits data randomly.
        Random = 1,
        /// Splits data with the user provided tags.
        Custom = 2,
        /// Splits data sequentially.
        Sequential = 3,
        /// Data split will be skipped.
        NoSplit = 4,
        /// Splits data automatically: Uses NO_SPLIT if the data size is small.
        /// Otherwise uses RANDOM.
        AutoSplit = 5,
    }
    /// Type of supported data frequency for time series forecasting models.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataFrequency {
        Unspecified = 0,
        /// Automatically inferred from timestamps.
        AutoFrequency = 1,
        /// Yearly data.
        Yearly = 2,
        /// Quarterly data.
        Quarterly = 3,
        /// Monthly data.
        Monthly = 4,
        /// Weekly data.
        Weekly = 5,
        /// Daily data.
        Daily = 6,
        /// Hourly data.
        Hourly = 7,
        /// Per-minute data.
        PerMinute = 8,
    }
    /// Type of supported holiday regions for time series forecasting models.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HolidayRegion {
        /// Holiday region unspecified.
        Unspecified = 0,
        /// Global.
        Global = 1,
        /// North America.
        Na = 2,
        /// Japan and Asia Pacific: Korea, Greater China, India, Australia, and New
        /// Zealand.
        Japac = 3,
        /// Europe, the Middle East and Africa.
        Emea = 4,
        /// Latin America and the Caribbean.
        Lac = 5,
        /// United Arab Emirates
        Ae = 6,
        /// Argentina
        Ar = 7,
        /// Austria
        At = 8,
        /// Australia
        Au = 9,
        /// Belgium
        Be = 10,
        /// Brazil
        Br = 11,
        /// Canada
        Ca = 12,
        /// Switzerland
        Ch = 13,
        /// Chile
        Cl = 14,
        /// China
        Cn = 15,
        /// Colombia
        Co = 16,
        /// Czechoslovakia
        Cs = 17,
        /// Czech Republic
        Cz = 18,
        /// Germany
        De = 19,
        /// Denmark
        Dk = 20,
        /// Algeria
        Dz = 21,
        /// Ecuador
        Ec = 22,
        /// Estonia
        Ee = 23,
        /// Egypt
        Eg = 24,
        /// Spain
        Es = 25,
        /// Finland
        Fi = 26,
        /// France
        Fr = 27,
        /// Great Britain (United Kingdom)
        Gb = 28,
        /// Greece
        Gr = 29,
        /// Hong Kong
        Hk = 30,
        /// Hungary
        Hu = 31,
        /// Indonesia
        Id = 32,
        /// Ireland
        Ie = 33,
        /// Israel
        Il = 34,
        /// India
        In = 35,
        /// Iran
        Ir = 36,
        /// Italy
        It = 37,
        /// Japan
        Jp = 38,
        /// Korea (South)
        Kr = 39,
        /// Latvia
        Lv = 40,
        /// Morocco
        Ma = 41,
        /// Mexico
        Mx = 42,
        /// Malaysia
        My = 43,
        /// Nigeria
        Ng = 44,
        /// Netherlands
        Nl = 45,
        /// Norway
        No = 46,
        /// New Zealand
        Nz = 47,
        /// Peru
        Pe = 48,
        /// Philippines
        Ph = 49,
        /// Pakistan
        Pk = 50,
        /// Poland
        Pl = 51,
        /// Portugal
        Pt = 52,
        /// Romania
        Ro = 53,
        /// Serbia
        Rs = 54,
        /// Russian Federation
        Ru = 55,
        /// Saudi Arabia
        Sa = 56,
        /// Sweden
        Se = 57,
        /// Singapore
        Sg = 58,
        /// Slovenia
        Si = 59,
        /// Slovakia
        Sk = 60,
        /// Thailand
        Th = 61,
        /// Turkey
        Tr = 62,
        /// Taiwan
        Tw = 63,
        /// Ukraine
        Ua = 64,
        /// United States
        Us = 65,
        /// Venezuela
        Ve = 66,
        /// Viet Nam
        Vn = 67,
        /// South Africa
        Za = 68,
    }
    /// Indicates the learning rate optimization strategy to use.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LearnRateStrategy {
        Unspecified = 0,
        /// Use line search to determine learning rate.
        LineSearch = 1,
        /// Use a constant learning rate.
        Constant = 2,
    }
    /// Indicates the optimization strategy used for training.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OptimizationStrategy {
        Unspecified = 0,
        /// Uses an iterative batch gradient descent algorithm.
        BatchGradientDescent = 1,
        /// Uses a normal equation to solve linear regression problem.
        NormalEquation = 2,
    }
    /// Indicates the training algorithm to use for matrix factorization models.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedbackType {
        Unspecified = 0,
        /// Use weighted-als for implicit feedback problems.
        Implicit = 1,
        /// Use nonweighted-als for explicit feedback problems.
        Explicit = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelRequest {
    /// Required. Project ID of the requested model.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Dataset ID of the requested model.
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Required. Model ID of the requested model.
    #[prost(string, tag = "3")]
    pub model_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchModelRequest {
    /// Required. Project ID of the model to patch.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Dataset ID of the model to patch.
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Required. Model ID of the model to patch.
    #[prost(string, tag = "3")]
    pub model_id: ::prost::alloc::string::String,
    /// Required. Patched model.
    /// Follows RFC5789 patch semantics. Missing fields are not updated.
    /// To clear a field, explicitly set to default value.
    #[prost(message, optional, tag = "4")]
    pub model: ::core::option::Option<Model>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteModelRequest {
    /// Required. Project ID of the model to delete.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Dataset ID of the model to delete.
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Required. Model ID of the model to delete.
    #[prost(string, tag = "3")]
    pub model_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// Required. Project ID of the models to list.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Dataset ID of the models to list.
    #[prost(string, tag = "2")]
    pub dataset_id: ::prost::alloc::string::String,
    /// The maximum number of results to return in a single response page.
    /// Leverage the page tokens to iterate through the entire collection.
    #[prost(message, optional, tag = "3")]
    pub max_results: ::core::option::Option<u32>,
    /// Page token, returned by a previous call to request the next page of
    /// results
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// Models in the requested dataset. Only the following fields are populated:
    /// model_reference, model_type, creation_time, last_modified_time and
    /// labels.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// A token to request the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod model_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ModelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ModelServiceClient<T>
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
        ) -> ModelServiceClient<InterceptedService<T, F>>
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
            ModelServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Gets the specified model resource by model ID."]
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
                "/google.cloud.bigquery.v2.ModelService/GetModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all models in the specified dataset. Requires the READER dataset"]
        #[doc = " role. After retrieving the list of models, you can get information about a"]
        #[doc = " particular model by calling the models.get method."]
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
                "/google.cloud.bigquery.v2.ModelService/ListModels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Patch specific fields in the specified model."]
        pub async fn patch_model(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.v2.ModelService/PatchModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the model specified by modelId from the dataset."]
        pub async fn delete_model(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteModelRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.v2.ModelService/DeleteModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
