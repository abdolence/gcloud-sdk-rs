#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionConfiguration {
    /// Optional. Describes the Cloud KMS encryption key that will be used to
    /// protect destination BigQuery table. The BigQuery Service Account associated
    /// with your project requires access to this encryption key.
    #[prost(message, optional, tag = "1")]
    pub kms_key_name: ::std::option::Option<::std::string::String>,
}
/// Id path of a model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModelReference {
    /// Required. The ID of the project containing this model.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. The ID of the dataset containing this model.
    #[prost(string, tag = "2")]
    pub dataset_id: std::string::String,
    /// Required. The ID of the model. The ID must contain only
    /// letters (a-z, A-Z), numbers (0-9), or underscores (_). The maximum
    /// length is 1,024 characters.
    #[prost(string, tag = "3")]
    pub model_id: std::string::String,
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
    pub sub_type: ::std::option::Option<standard_sql_data_type::SubType>,
}
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
        /// Encoded as WKT
        Geography = 22,
        /// Encoded as a decimal string.
        Numeric = 23,
        /// Encoded as a list with types matching Type.array_type.
        Array = 16,
        /// Encoded as a list with fields of type Type.struct_type[i]. List is used
        /// because a JSON object cannot have duplicate field names.
        Struct = 17,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SubType {
        /// The type of the array's elements, if type_kind = "ARRAY".
        #[prost(message, tag = "2")]
        ArrayElementType(Box<super::StandardSqlDataType>),
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
    pub name: std::string::String,
    /// Optional. The type of this parameter. Absent if not explicitly
    /// specified (e.g., CREATE FUNCTION statement can omit the return type;
    /// in this case the output parameter does not have this "type" field).
    #[prost(message, optional, tag = "2")]
    pub r#type: ::std::option::Option<StandardSqlDataType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardSqlStructType {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::std::vec::Vec<StandardSqlField>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// Output only. A hash of this resource.
    #[prost(string, tag = "1")]
    pub etag: std::string::String,
    /// Required. Unique identifier for this model.
    #[prost(message, optional, tag = "2")]
    pub model_reference: ::std::option::Option<ModelReference>,
    /// Output only. The time when this model was created, in millisecs since the epoch.
    #[prost(int64, tag = "5")]
    pub creation_time: i64,
    /// Output only. The time when this model was last modified, in millisecs since the epoch.
    #[prost(int64, tag = "6")]
    pub last_modified_time: i64,
    /// Optional. A user-friendly description of this model.
    #[prost(string, tag = "12")]
    pub description: std::string::String,
    /// Optional. A descriptive name for this model.
    #[prost(string, tag = "14")]
    pub friendly_name: std::string::String,
    /// The labels associated with this model. You can use these to organize
    /// and group your models. Label keys and values can be no longer
    /// than 63 characters, can only contain lowercase letters, numeric
    /// characters, underscores and dashes. International characters are allowed.
    /// Label values are optional. Label keys must start with a letter and each
    /// label in the list must have a different key.
    #[prost(map = "string, string", tag = "15")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
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
    pub location: std::string::String,
    /// Custom encryption configuration (e.g., Cloud KMS keys). This shows the
    /// encryption configuration of the model data while stored in BigQuery
    /// storage.
    #[prost(message, optional, tag = "17")]
    pub encryption_configuration: ::std::option::Option<EncryptionConfiguration>,
    /// Output only. Type of the model resource.
    #[prost(enumeration = "model::ModelType", tag = "7")]
    pub model_type: i32,
    /// Output only. Information for all training runs in increasing order of start_time.
    #[prost(message, repeated, tag = "9")]
    pub training_runs: ::std::vec::Vec<model::TrainingRun>,
    /// Output only. Input feature columns that were used to train this model.
    #[prost(message, repeated, tag = "10")]
    pub feature_columns: ::std::vec::Vec<StandardSqlField>,
    /// Output only. Label columns that were used to train this model.
    /// The output of the model will have a "predicted_" prefix to these columns.
    #[prost(message, repeated, tag = "11")]
    pub label_columns: ::std::vec::Vec<StandardSqlField>,
}
pub mod model {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KmeansEnums {}
    pub mod kmeans_enums {
        /// Indicates the method used to initialize the centroids for KMeans
        /// clustering algorithm.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum KmeansInitializationMethod {
            Unspecified = 0,
            /// Initializes the centroids randomly.
            Random = 1,
            /// Initializes the centroids using data specified in
            /// kmeans_initialization_column.
            Custom = 2,
        }
    }
    /// Evaluation metrics for regression and explicit feedback type matrix
    /// factorization models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RegressionMetrics {
        /// Mean absolute error.
        #[prost(message, optional, tag = "1")]
        pub mean_absolute_error: ::std::option::Option<f64>,
        /// Mean squared error.
        #[prost(message, optional, tag = "2")]
        pub mean_squared_error: ::std::option::Option<f64>,
        /// Mean squared log error.
        #[prost(message, optional, tag = "3")]
        pub mean_squared_log_error: ::std::option::Option<f64>,
        /// Median absolute error.
        #[prost(message, optional, tag = "4")]
        pub median_absolute_error: ::std::option::Option<f64>,
        /// R^2 score.
        #[prost(message, optional, tag = "5")]
        pub r_squared: ::std::option::Option<f64>,
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
        pub precision: ::std::option::Option<f64>,
        /// Recall is the fraction of actual positive labels that were given a
        /// positive prediction. For multiclass this is a macro-averaged metric.
        #[prost(message, optional, tag = "2")]
        pub recall: ::std::option::Option<f64>,
        /// Accuracy is the fraction of predictions given the correct label. For
        /// multiclass this is a micro-averaged metric.
        #[prost(message, optional, tag = "3")]
        pub accuracy: ::std::option::Option<f64>,
        /// Threshold at which the metrics are computed. For binary
        /// classification models this is the positive class threshold.
        /// For multi-class classfication models this is the confidence
        /// threshold.
        #[prost(message, optional, tag = "4")]
        pub threshold: ::std::option::Option<f64>,
        /// The F1 score is an average of recall and precision. For multiclass
        /// this is a macro-averaged metric.
        #[prost(message, optional, tag = "5")]
        pub f1_score: ::std::option::Option<f64>,
        /// Logarithmic Loss. For multiclass this is a macro-averaged metric.
        #[prost(message, optional, tag = "6")]
        pub log_loss: ::std::option::Option<f64>,
        /// Area Under a ROC Curve. For multiclass this is a macro-averaged
        /// metric.
        #[prost(message, optional, tag = "7")]
        pub roc_auc: ::std::option::Option<f64>,
    }
    /// Evaluation metrics for binary classification/classifier models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BinaryClassificationMetrics {
        /// Aggregate classification metrics.
        #[prost(message, optional, tag = "1")]
        pub aggregate_classification_metrics: ::std::option::Option<AggregateClassificationMetrics>,
        /// Binary confusion matrix at multiple thresholds.
        #[prost(message, repeated, tag = "2")]
        pub binary_confusion_matrix_list:
            ::std::vec::Vec<binary_classification_metrics::BinaryConfusionMatrix>,
        /// Label representing the positive class.
        #[prost(string, tag = "3")]
        pub positive_label: std::string::String,
        /// Label representing the negative class.
        #[prost(string, tag = "4")]
        pub negative_label: std::string::String,
    }
    pub mod binary_classification_metrics {
        /// Confusion matrix for binary classification models.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BinaryConfusionMatrix {
            /// Threshold value used when computing each of the following metric.
            #[prost(message, optional, tag = "1")]
            pub positive_class_threshold: ::std::option::Option<f64>,
            /// Number of true samples predicted as true.
            #[prost(message, optional, tag = "2")]
            pub true_positives: ::std::option::Option<i64>,
            /// Number of false samples predicted as true.
            #[prost(message, optional, tag = "3")]
            pub false_positives: ::std::option::Option<i64>,
            /// Number of true samples predicted as false.
            #[prost(message, optional, tag = "4")]
            pub true_negatives: ::std::option::Option<i64>,
            /// Number of false samples predicted as false.
            #[prost(message, optional, tag = "5")]
            pub false_negatives: ::std::option::Option<i64>,
            /// The fraction of actual positive predictions that had positive actual
            /// labels.
            #[prost(message, optional, tag = "6")]
            pub precision: ::std::option::Option<f64>,
            /// The fraction of actual positive labels that were given a positive
            /// prediction.
            #[prost(message, optional, tag = "7")]
            pub recall: ::std::option::Option<f64>,
            /// The equally weighted average of recall and precision.
            #[prost(message, optional, tag = "8")]
            pub f1_score: ::std::option::Option<f64>,
            /// The fraction of predictions given the correct label.
            #[prost(message, optional, tag = "9")]
            pub accuracy: ::std::option::Option<f64>,
        }
    }
    /// Evaluation metrics for multi-class classification/classifier models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultiClassClassificationMetrics {
        /// Aggregate classification metrics.
        #[prost(message, optional, tag = "1")]
        pub aggregate_classification_metrics: ::std::option::Option<AggregateClassificationMetrics>,
        /// Confusion matrix at different thresholds.
        #[prost(message, repeated, tag = "2")]
        pub confusion_matrix_list:
            ::std::vec::Vec<multi_class_classification_metrics::ConfusionMatrix>,
    }
    pub mod multi_class_classification_metrics {
        /// Confusion matrix for multi-class classification models.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConfusionMatrix {
            /// Confidence threshold used when computing the entries of the
            /// confusion matrix.
            #[prost(message, optional, tag = "1")]
            pub confidence_threshold: ::std::option::Option<f64>,
            /// One row per actual label.
            #[prost(message, repeated, tag = "2")]
            pub rows: ::std::vec::Vec<confusion_matrix::Row>,
        }
        pub mod confusion_matrix {
            /// A single entry in the confusion matrix.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Entry {
                /// The predicted label. For confidence_threshold > 0, we will
                /// also add an entry indicating the number of items under the
                /// confidence threshold.
                #[prost(string, tag = "1")]
                pub predicted_label: std::string::String,
                /// Number of items being predicted as this label.
                #[prost(message, optional, tag = "2")]
                pub item_count: ::std::option::Option<i64>,
            }
            /// A single row in the confusion matrix.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Row {
                /// The original label of this row.
                #[prost(string, tag = "1")]
                pub actual_label: std::string::String,
                /// Info describing predicted label distribution.
                #[prost(message, repeated, tag = "2")]
                pub entries: ::std::vec::Vec<Entry>,
            }
        }
    }
    /// Evaluation metrics for clustering models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusteringMetrics {
        /// Davies-Bouldin index.
        #[prost(message, optional, tag = "1")]
        pub davies_bouldin_index: ::std::option::Option<f64>,
        /// Mean of squared distances between each sample to its cluster centroid.
        #[prost(message, optional, tag = "2")]
        pub mean_squared_distance: ::std::option::Option<f64>,
        /// [Beta] Information for all clusters.
        #[prost(message, repeated, tag = "3")]
        pub clusters: ::std::vec::Vec<clustering_metrics::Cluster>,
    }
    pub mod clustering_metrics {
        /// Message containing the information about one cluster.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Cluster {
            /// Centroid id.
            #[prost(int64, tag = "1")]
            pub centroid_id: i64,
            /// Values of highly variant features for this cluster.
            #[prost(message, repeated, tag = "2")]
            pub feature_values: ::std::vec::Vec<cluster::FeatureValue>,
            /// Count of training data rows that were assigned to this cluster.
            #[prost(message, optional, tag = "3")]
            pub count: ::std::option::Option<i64>,
        }
        pub mod cluster {
            /// Representative value of a single feature within the cluster.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct FeatureValue {
                /// The feature column name.
                #[prost(string, tag = "1")]
                pub feature_column: std::string::String,
                #[prost(oneof = "feature_value::Value", tags = "2, 3")]
                pub value: ::std::option::Option<feature_value::Value>,
            }
            pub mod feature_value {
                /// Representative value of a categorical feature.
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct CategoricalValue {
                    /// Counts of all categories for the categorical feature. If there are
                    /// more than ten categories, we return top ten (by count) and return
                    /// one more CategoryCount with category "_OTHER_" and count as
                    /// aggregate counts of remaining categories.
                    #[prost(message, repeated, tag = "1")]
                    pub category_counts: ::std::vec::Vec<categorical_value::CategoryCount>,
                }
                pub mod categorical_value {
                    /// Represents the count of a single category within the cluster.
                    #[derive(Clone, PartialEq, ::prost::Message)]
                    pub struct CategoryCount {
                        /// The name of category.
                        #[prost(string, tag = "1")]
                        pub category: std::string::String,
                        /// The count of training samples matching the category within the
                        /// cluster.
                        #[prost(message, optional, tag = "2")]
                        pub count: ::std::option::Option<i64>,
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
    /// Evaluation metrics of a model. These are either computed on all training
    /// data or just the eval data based on whether eval data was used during
    /// training. These are not present for imported models.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EvaluationMetrics {
        #[prost(oneof = "evaluation_metrics::Metrics", tags = "1, 2, 3, 4")]
        pub metrics: ::std::option::Option<evaluation_metrics::Metrics>,
    }
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
        }
    }
    /// Information about a single training query run for the model.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrainingRun {
        /// Options that were used for this training run, includes
        /// user specified and default options that were used.
        #[prost(message, optional, tag = "1")]
        pub training_options: ::std::option::Option<training_run::TrainingOptions>,
        /// The start time of this training run.
        #[prost(message, optional, tag = "8")]
        pub start_time: ::std::option::Option<::prost_types::Timestamp>,
        /// Output of each iteration run, results.size() <= max_iterations.
        #[prost(message, repeated, tag = "6")]
        pub results: ::std::vec::Vec<training_run::IterationResult>,
        /// The evaluation metrics over training/eval data that were computed at the
        /// end of training.
        #[prost(message, optional, tag = "7")]
        pub evaluation_metrics: ::std::option::Option<EvaluationMetrics>,
    }
    pub mod training_run {
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
            pub l1_regularization: ::std::option::Option<f64>,
            /// L2 regularization coefficient.
            #[prost(message, optional, tag = "5")]
            pub l2_regularization: ::std::option::Option<f64>,
            /// When early_stop is true, stops training when accuracy improvement is
            /// less than 'min_relative_progress'. Used only for iterative training
            /// algorithms.
            #[prost(message, optional, tag = "6")]
            pub min_relative_progress: ::std::option::Option<f64>,
            /// Whether to train a model from the last checkpoint.
            #[prost(message, optional, tag = "7")]
            pub warm_start: ::std::option::Option<bool>,
            /// Whether to stop early when the loss doesn't improve significantly
            /// any more (compared to min_relative_progress). Used only for iterative
            /// training algorithms.
            #[prost(message, optional, tag = "8")]
            pub early_stop: ::std::option::Option<bool>,
            /// Name of input label columns in training data.
            #[prost(string, repeated, tag = "9")]
            pub input_label_columns: ::std::vec::Vec<std::string::String>,
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
            /// https://cloud.google.com/bigquery/docs/reference/standard-sql/data-types#data-type-properties
            #[prost(string, tag = "12")]
            pub data_split_column: std::string::String,
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
            pub label_class_weights: ::std::collections::HashMap<std::string::String, f64>,
            /// Distance type for clustering models.
            #[prost(enumeration = "super::DistanceType", tag = "20")]
            pub distance_type: i32,
            /// Number of clusters for clustering models.
            #[prost(int64, tag = "21")]
            pub num_clusters: i64,
            /// [Beta] Google Cloud Storage URI from which the model was imported. Only
            /// applicable for imported models.
            #[prost(string, tag = "22")]
            pub model_uri: std::string::String,
            /// Optimization strategy for training linear regression models.
            #[prost(enumeration = "super::OptimizationStrategy", tag = "23")]
            pub optimization_strategy: i32,
            /// The method used to initialize the centroids for kmeans algorithm.
            #[prost(
                enumeration = "super::kmeans_enums::KmeansInitializationMethod",
                tag = "33"
            )]
            pub kmeans_initialization_method: i32,
            /// The column used to provide the initial centroids for kmeans algorithm
            /// when kmeans_initialization_method is CUSTOM.
            #[prost(string, tag = "34")]
            pub kmeans_initialization_column: std::string::String,
        }
        /// Information about a single iteration of the training run.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IterationResult {
            /// Index of the iteration, 0 based.
            #[prost(message, optional, tag = "1")]
            pub index: ::std::option::Option<i32>,
            /// Time taken to run the iteration in milliseconds.
            #[prost(message, optional, tag = "4")]
            pub duration_ms: ::std::option::Option<i64>,
            /// Loss computed on the training data at the end of iteration.
            #[prost(message, optional, tag = "5")]
            pub training_loss: ::std::option::Option<f64>,
            /// Loss computed on the eval data at the end of iteration.
            #[prost(message, optional, tag = "6")]
            pub eval_loss: ::std::option::Option<f64>,
            /// Learn rate used for this iteration.
            #[prost(double, tag = "7")]
            pub learn_rate: f64,
            /// Information about top clusters for clustering models.
            #[prost(message, repeated, tag = "8")]
            pub cluster_infos: ::std::vec::Vec<iteration_result::ClusterInfo>,
        }
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
                pub cluster_radius: ::std::option::Option<f64>,
                /// Cluster size, the total number of points assigned to the cluster.
                #[prost(message, optional, tag = "3")]
                pub cluster_size: ::std::option::Option<i64>,
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
        /// [Beta] An imported TensorFlow model.
        Tensorflow = 6,
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
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelRequest {
    /// Required. Project ID of the requested model.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Dataset ID of the requested model.
    #[prost(string, tag = "2")]
    pub dataset_id: std::string::String,
    /// Required. Model ID of the requested model.
    #[prost(string, tag = "3")]
    pub model_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchModelRequest {
    /// Required. Project ID of the model to patch.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Dataset ID of the model to patch.
    #[prost(string, tag = "2")]
    pub dataset_id: std::string::String,
    /// Required. Model ID of the model to patch.
    #[prost(string, tag = "3")]
    pub model_id: std::string::String,
    /// Required. Patched model.
    /// Follows RFC5789 patch semantics. Missing fields are not updated.
    /// To clear a field, explicitly set to default value.
    #[prost(message, optional, tag = "4")]
    pub model: ::std::option::Option<Model>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteModelRequest {
    /// Required. Project ID of the model to delete.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Dataset ID of the model to delete.
    #[prost(string, tag = "2")]
    pub dataset_id: std::string::String,
    /// Required. Model ID of the model to delete.
    #[prost(string, tag = "3")]
    pub model_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// Required. Project ID of the models to list.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Dataset ID of the models to list.
    #[prost(string, tag = "2")]
    pub dataset_id: std::string::String,
    /// The maximum number of results to return in a single response page.
    /// Leverage the page tokens to iterate through the entire collection.
    #[prost(message, optional, tag = "3")]
    pub max_results: ::std::option::Option<u32>,
    /// Page token, returned by a previous call to request the next page of
    /// results
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// Models in the requested dataset. Only the following fields are populated:
    /// model_reference, model_type, creation_time, last_modified_time and
    /// labels.
    #[prost(message, repeated, tag = "1")]
    pub models: ::std::vec::Vec<Model>,
    /// A token to request the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod model_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ModelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ModelServiceClient<tonic::transport::Channel> {
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
        #[doc = " role."]
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
#[doc = r" Generated server implementations."]
pub mod model_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ModelServiceServer."]
    #[async_trait]
    pub trait ModelService: Send + Sync + 'static {
        #[doc = " Gets the specified model resource by model ID."]
        async fn get_model(
            &self,
            request: tonic::Request<super::GetModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>;
        #[doc = " Lists all models in the specified dataset. Requires the READER dataset"]
        #[doc = " role."]
        async fn list_models(
            &self,
            request: tonic::Request<super::ListModelsRequest>,
        ) -> Result<tonic::Response<super::ListModelsResponse>, tonic::Status>;
        #[doc = " Patch specific fields in the specified model."]
        async fn patch_model(
            &self,
            request: tonic::Request<super::PatchModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>;
        #[doc = " Deletes the model specified by modelId from the dataset."]
        async fn delete_model(
            &self,
            request: tonic::Request<super::DeleteModelRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ModelServiceServer<T: ModelService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ModelService> ModelServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ModelServiceServer<T>
    where
        T: ModelService,
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
                "/google.cloud.bigquery.v2.ModelService/GetModel" => {
                    #[allow(non_camel_case_types)]
                    struct GetModelSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService> tonic::server::UnaryService<super::GetModelRequest> for GetModelSvc<T> {
                        type Response = super::Model;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetModelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_model(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetModelSvc(inner);
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
                "/google.cloud.bigquery.v2.ModelService/ListModels" => {
                    #[allow(non_camel_case_types)]
                    struct ListModelsSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService> tonic::server::UnaryService<super::ListModelsRequest> for ListModelsSvc<T> {
                        type Response = super::ListModelsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListModelsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_models(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListModelsSvc(inner);
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
                "/google.cloud.bigquery.v2.ModelService/PatchModel" => {
                    #[allow(non_camel_case_types)]
                    struct PatchModelSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService> tonic::server::UnaryService<super::PatchModelRequest> for PatchModelSvc<T> {
                        type Response = super::Model;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PatchModelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.patch_model(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PatchModelSvc(inner);
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
                "/google.cloud.bigquery.v2.ModelService/DeleteModel" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteModelSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService> tonic::server::UnaryService<super::DeleteModelRequest> for DeleteModelSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteModelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_model(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteModelSvc(inner);
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
    impl<T: ModelService> Clone for ModelServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ModelService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ModelService> tonic::transport::NamedService for ModelServiceServer<T> {
        const NAME: &'static str = "google.cloud.bigquery.v2.ModelService";
    }
}
