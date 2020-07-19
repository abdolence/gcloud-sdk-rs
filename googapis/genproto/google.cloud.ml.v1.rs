/// Represents input parameters for a training job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainingInput {
    /// Required. Specifies the machine types, the number of replicas for workers
    /// and parameter servers.
    #[prost(enumeration = "training_input::ScaleTier", tag = "1")]
    pub scale_tier: i32,
    /// Optional. Specifies the type of virtual machine to use for your training
    /// job's master worker.
    ///
    /// The following types are supported:
    ///
    /// <dl>
    ///   <dt>standard</dt>
    ///   <dd>
    ///   A basic machine configuration suitable for training simple models with
    ///   small to moderate datasets.
    ///   </dd>
    ///   <dt>large_model</dt>
    ///   <dd>
    ///   A machine with a lot of memory, specially suited for parameter servers
    ///   when your model is large (having many hidden layers or layers with very
    ///   large numbers of nodes).
    ///   </dd>
    ///   <dt>complex_model_s</dt>
    ///   <dd>
    ///   A machine suitable for the master and workers of the cluster when your
    ///   model requires more computation than the standard machine can handle
    ///   satisfactorily.
    ///   </dd>
    ///   <dt>complex_model_m</dt>
    ///   <dd>
    ///   A machine with roughly twice the number of cores and roughly double the
    ///   memory of <code suppresswarning="true">complex_model_s</code>.
    ///   </dd>
    ///   <dt>complex_model_l</dt>
    ///   <dd>
    ///   A machine with roughly twice the number of cores and roughly double the
    ///   memory of <code suppresswarning="true">complex_model_m</code>.
    ///   </dd>
    ///   <dt>standard_gpu</dt>
    ///   <dd>
    ///   A machine equivalent to <code suppresswarning="true">standard</code> that
    ///   also includes a
    ///   <a href="ml/docs/how-tos/using-gpus">
    ///   GPU that you can use in your trainer</a>.
    ///   </dd>
    ///   <dt>complex_model_m_gpu</dt>
    ///   <dd>
    ///   A machine equivalent to
    ///   <code suppresswarning="true">coplex_model_m</code> that also includes
    ///   four GPUs.
    ///   </dd>
    /// </dl>
    ///
    /// You must set this value when `scaleTier` is set to `CUSTOM`.
    #[prost(string, tag = "2")]
    pub master_type: std::string::String,
    /// Optional. Specifies the type of virtual machine to use for your training
    /// job's worker nodes.
    ///
    /// The supported values are the same as those described in the entry for
    /// `masterType`.
    ///
    /// This value must be present when `scaleTier` is set to `CUSTOM` and
    /// `workerCount` is greater than zero.
    #[prost(string, tag = "3")]
    pub worker_type: std::string::String,
    /// Optional. Specifies the type of virtual machine to use for your training
    /// job's parameter server.
    ///
    /// The supported values are the same as those described in the entry for
    /// `master_type`.
    ///
    /// This value must be present when `scaleTier` is set to `CUSTOM` and
    /// `parameter_server_count` is greater than zero.
    #[prost(string, tag = "4")]
    pub parameter_server_type: std::string::String,
    /// Optional. The number of worker replicas to use for the training job. Each
    /// replica in the cluster will be of the type specified in `worker_type`.
    ///
    /// This value can only be used when `scale_tier` is set to `CUSTOM`. If you
    /// set this value, you must also set `worker_type`.
    #[prost(int64, tag = "5")]
    pub worker_count: i64,
    /// Optional. The number of parameter server replicas to use for the training
    /// job. Each replica in the cluster will be of the type specified in
    /// `parameter_server_type`.
    ///
    /// This value can only be used when `scale_tier` is set to `CUSTOM`.If you
    /// set this value, you must also set `parameter_server_type`.
    #[prost(int64, tag = "6")]
    pub parameter_server_count: i64,
    /// Required. The Google Cloud Storage location of the packages with
    /// the training program and any additional dependencies.
    #[prost(string, repeated, tag = "7")]
    pub package_uris: ::std::vec::Vec<std::string::String>,
    /// Required. The Python module name to run after installing the packages.
    #[prost(string, tag = "8")]
    pub python_module: std::string::String,
    /// Optional. Command line arguments to pass to the program.
    #[prost(string, repeated, tag = "10")]
    pub args: ::std::vec::Vec<std::string::String>,
    /// Optional. The set of Hyperparameters to tune.
    #[prost(message, optional, tag = "12")]
    pub hyperparameters: ::std::option::Option<HyperparameterSpec>,
    /// Required. The Google Compute Engine region to run the training job in.
    #[prost(string, tag = "14")]
    pub region: std::string::String,
    /// Optional. A Google Cloud Storage path in which to store training outputs
    /// and other data needed for training. This path is passed to your TensorFlow
    /// program as the 'job_dir' command-line argument. The benefit of specifying
    /// this field is that Cloud ML validates the path for use in training.
    #[prost(string, tag = "16")]
    pub job_dir: std::string::String,
    /// Optional. The Google Cloud ML runtime version to use for training.  If not
    /// set, Google Cloud ML will choose the latest stable version.
    #[prost(string, tag = "15")]
    pub runtime_version: std::string::String,
}
pub mod training_input {
    /// A scale tier is an abstract representation of the resources Cloud ML
    /// will allocate to a training job. When selecting a scale tier for your
    /// training job, you should consider the size of your training dataset and
    /// the complexity of your model. As the tiers increase, virtual machines are
    /// added to handle your job, and the individual machines in the cluster
    /// generally have more memory and greater processing power than they do at
    /// lower tiers. The number of training units charged per hour of processing
    /// increases as tiers get more advanced. Refer to the
    /// [pricing guide](/ml/pricing) for more details. Note that in addition to
    /// incurring costs, your use of training resources is constrained by the
    /// [quota policy](/ml/quota).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ScaleTier {
        /// A single worker instance. This tier is suitable for learning how to use
        /// Cloud ML, and for experimenting with new models using small datasets.
        Basic = 0,
        /// Many workers and a few parameter servers.
        Standard1 = 1,
        /// A large number of workers with many parameter servers.
        Premium1 = 3,
        /// A single worker instance [with a GPU](ml/docs/how-tos/using-gpus).
        BasicGpu = 6,
        /// The CUSTOM tier is not a set tier, but rather enables you to use your
        /// own cluster specification. When you use this tier, set values to
        /// configure your processing cluster according to these guidelines:
        ///
        /// *   You _must_ set `TrainingInput.masterType` to specify the type
        ///     of machine to use for your master node. This is the only required
        ///     setting.
        ///
        /// *   You _may_ set `TrainingInput.workerCount` to specify the number of
        ///     workers to use. If you specify one or more workers, you _must_ also
        ///     set `TrainingInput.workerType` to specify the type of machine to use
        ///     for your worker nodes.
        ///
        /// *   You _may_ set `TrainingInput.parameterServerCount` to specify the
        ///     number of parameter servers to use. If you specify one or more
        ///     parameter servers, you _must_ also set
        ///     `TrainingInput.parameterServerType` to specify the type of machine to
        ///     use for your parameter servers.
        ///
        /// Note that all of your workers must use the same machine type, which can
        /// be different from your parameter server type and master type. Your
        /// parameter servers must likewise use the same machine type, which can be
        /// different from your worker type and master type.
        Custom = 5,
    }
}
/// Represents a set of hyperparameters to optimize.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HyperparameterSpec {
    /// Required. The type of goal to use for tuning. Available types are
    /// `MAXIMIZE` and `MINIMIZE`.
    ///
    /// Defaults to `MAXIMIZE`.
    #[prost(enumeration = "hyperparameter_spec::GoalType", tag = "1")]
    pub goal: i32,
    /// Required. The set of parameters to tune.
    #[prost(message, repeated, tag = "2")]
    pub params: ::std::vec::Vec<ParameterSpec>,
    /// Optional. How many training trials should be attempted to optimize
    /// the specified hyperparameters.
    ///
    /// Defaults to one.
    #[prost(int32, tag = "3")]
    pub max_trials: i32,
    /// Optional. The number of training trials to run concurrently.
    /// You can reduce the time it takes to perform hyperparameter tuning by adding
    /// trials in parallel. However, each trail only benefits from the information
    /// gained in completed trials. That means that a trial does not get access to
    /// the results of trials running at the same time, which could reduce the
    /// quality of the overall optimization.
    ///
    /// Each trial will use the same scale tier and machine types.
    ///
    /// Defaults to one.
    #[prost(int32, tag = "4")]
    pub max_parallel_trials: i32,
    /// Optional. The Tensorflow summary tag name to use for optimizing trials. For
    /// current versions of Tensorflow, this tag name should exactly match what is
    /// shown in Tensorboard, including all scopes.  For versions of Tensorflow
    /// prior to 0.12, this should be only the tag passed to tf.Summary.
    /// By default, "training/hptuning/metric" will be used.
    #[prost(string, tag = "5")]
    pub hyperparameter_metric_tag: std::string::String,
}
pub mod hyperparameter_spec {
    /// The available types of optimization goals.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
/// Represents a single hyperparameter to optimize.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParameterSpec {
    /// Required. The parameter name must be unique amongst all ParameterConfigs in
    /// a HyperparameterSpec message. E.g., "learning_rate".
    #[prost(string, tag = "1")]
    pub parameter_name: std::string::String,
    /// Required. The type of the parameter.
    #[prost(enumeration = "parameter_spec::ParameterType", tag = "4")]
    pub r#type: i32,
    /// Required if type is `DOUBLE` or `INTEGER`. This field
    /// should be unset if type is `CATEGORICAL`. This value should be integers if
    /// type is INTEGER.
    #[prost(double, tag = "2")]
    pub min_value: f64,
    /// Required if typeis `DOUBLE` or `INTEGER`. This field
    /// should be unset if type is `CATEGORICAL`. This value should be integers if
    /// type is `INTEGER`.
    #[prost(double, tag = "3")]
    pub max_value: f64,
    /// Required if type is `CATEGORICAL`. The list of possible categories.
    #[prost(string, repeated, tag = "5")]
    pub categorical_values: ::std::vec::Vec<std::string::String>,
    /// Required if type is `DISCRETE`.
    /// A list of feasible points.
    /// The list should be in strictly increasing order. For instance, this
    /// parameter might have possible settings of 1.5, 2.5, and 4.0. This list
    /// should not contain more than 1,000 values.
    #[prost(double, repeated, tag = "6")]
    pub discrete_values: ::std::vec::Vec<f64>,
    /// Optional. How the parameter should be scaled to the hypercube.
    /// Leave unset for categorical parameters.
    /// Some kind of scaling is strongly recommended for real or integral
    /// parameters (e.g., `UNIT_LINEAR_SCALE`).
    #[prost(enumeration = "parameter_spec::ScaleType", tag = "7")]
    pub scale_type: i32,
}
pub mod parameter_spec {
    /// The type of the parameter.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ParameterType {
        /// You must specify a valid type. Using this unspecified type will result in
        /// an error.
        Unspecified = 0,
        /// Type for real-valued parameters.
        Double = 1,
        /// Type for integral parameters.
        Integer = 2,
        /// The parameter is categorical, with a value chosen from the categories
        /// field.
        Categorical = 3,
        /// The parameter is real valued, with a fixed set of feasible points. If
        /// `type==DISCRETE`, feasible_points must be provided, and
        /// {`min_value`, `max_value`} will be ignored.
        Discrete = 4,
    }
    /// The type of scaling that should be applied to this parameter.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ScaleType {
        /// By default, no scaling is applied.
        None = 0,
        /// Scales the feasible space to (0, 1) linearly.
        UnitLinearScale = 1,
        /// Scales the feasible space logarithmically to (0, 1). The entire feasible
        /// space must be strictly positive.
        UnitLogScale = 2,
        /// Scales the feasible space "reverse" logarithmically to (0, 1). The result
        /// is that values close to the top of the feasible space are spread out more
        /// than points near the bottom. The entire feasible space must be strictly
        /// positive.
        UnitReverseLogScale = 3,
    }
}
/// Represents the result of a single hyperparameter tuning trial from a
/// training job. The TrainingOutput object that is returned on successful
/// completion of a training job with hyperparameter tuning includes a list
/// of HyperparameterOutput objects, one for each successful trial.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HyperparameterOutput {
    /// The trial id for these results.
    #[prost(string, tag = "1")]
    pub trial_id: std::string::String,
    /// The hyperparameters given to this trial.
    #[prost(map = "string, string", tag = "2")]
    pub hyperparameters: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The final objective metric seen for this trial.
    #[prost(message, optional, tag = "3")]
    pub final_metric: ::std::option::Option<hyperparameter_output::HyperparameterMetric>,
    /// All recorded object metrics for this trial.
    #[prost(message, repeated, tag = "4")]
    pub all_metrics: ::std::vec::Vec<hyperparameter_output::HyperparameterMetric>,
}
pub mod hyperparameter_output {
    /// An observed value of a metric.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HyperparameterMetric {
        /// The global training step for this metric.
        #[prost(int64, tag = "1")]
        pub training_step: i64,
        /// The objective value at this training step.
        #[prost(double, tag = "2")]
        pub objective_value: f64,
    }
}
/// Represents results of a training job. Output only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainingOutput {
    /// The number of hyperparameter tuning trials that completed successfully.
    /// Only set for hyperparameter tuning jobs.
    #[prost(int64, tag = "1")]
    pub completed_trial_count: i64,
    /// Results for individual Hyperparameter trials.
    /// Only set for hyperparameter tuning jobs.
    #[prost(message, repeated, tag = "2")]
    pub trials: ::std::vec::Vec<HyperparameterOutput>,
    /// The amount of ML units consumed by the job.
    #[prost(double, tag = "3")]
    pub consumed_ml_units: f64,
    /// Whether this job is a hyperparameter tuning job.
    #[prost(bool, tag = "4")]
    pub is_hyperparameter_tuning_job: bool,
}
/// Represents input parameters for a prediction job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictionInput {
    /// Required. The format of the input data files.
    #[prost(enumeration = "prediction_input::DataFormat", tag = "3")]
    pub data_format: i32,
    /// Required. The Google Cloud Storage location of the input data files.
    /// May contain wildcards.
    #[prost(string, repeated, tag = "4")]
    pub input_paths: ::std::vec::Vec<std::string::String>,
    /// Required. The output Google Cloud Storage location.
    #[prost(string, tag = "5")]
    pub output_path: std::string::String,
    /// Optional. The maximum number of workers to be used for parallel processing.
    /// Defaults to 10 if not specified.
    #[prost(int64, tag = "6")]
    pub max_worker_count: i64,
    /// Required. The Google Compute Engine region to run the prediction job in.
    #[prost(string, tag = "7")]
    pub region: std::string::String,
    /// Optional. The Google Cloud ML runtime version to use for this batch
    /// prediction. If not set, Google Cloud ML will pick the runtime version used
    /// during the CreateVersion request for this model version, or choose the
    /// latest stable version when model version information is not available
    /// such as when the model is specified by uri.
    #[prost(string, tag = "8")]
    pub runtime_version: std::string::String,
    /// Required. The model or the version to use for prediction.
    #[prost(oneof = "prediction_input::ModelVersion", tags = "1, 2, 9")]
    pub model_version: ::std::option::Option<prediction_input::ModelVersion>,
}
pub mod prediction_input {
    /// The format used to separate data instances in the source files.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataFormat {
        /// Unspecified format.
        Unspecified = 0,
        /// The source file is a text file with instances separated by the
        /// new-line character.
        Text = 1,
        /// The source file is a TFRecord file.
        TfRecord = 2,
        /// The source file is a GZIP-compressed TFRecord file.
        TfRecordGzip = 3,
    }
    /// Required. The model or the version to use for prediction.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModelVersion {
        /// Use this field if you want to use the default version for the specified
        /// model. The string must use the following format:
        ///
        /// `"projects/<var>[YOUR_PROJECT]</var>/models/<var>[YOUR_MODEL]</var>"`
        #[prost(string, tag = "1")]
        ModelName(std::string::String),
        /// Use this field if you want to specify a version of the model to use. The
        /// string is formatted the same way as `model_version`, with the addition
        /// of the version information:
        ///
        /// `"projects/<var>[YOUR_PROJECT]</var>/models/<var>YOUR_MODEL/versions/<var>[YOUR_VERSION]</var>"`
        #[prost(string, tag = "2")]
        VersionName(std::string::String),
        /// Use this field if you want to specify a Google Cloud Storage path for
        /// the model to use.
        #[prost(string, tag = "9")]
        Uri(std::string::String),
    }
}
/// Represents results of a prediction job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictionOutput {
    /// The output Google Cloud Storage location provided at the job creation time.
    #[prost(string, tag = "1")]
    pub output_path: std::string::String,
    /// The number of generated predictions.
    #[prost(int64, tag = "2")]
    pub prediction_count: i64,
    /// The number of data instances which resulted in errors.
    #[prost(int64, tag = "3")]
    pub error_count: i64,
    /// Node hours used by the batch prediction job.
    #[prost(double, tag = "4")]
    pub node_hours: f64,
}
/// Represents a training or prediction job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Required. The user-specified id of the job.
    #[prost(string, tag = "1")]
    pub job_id: std::string::String,
    /// Output only. When the job was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. When the job processing was started.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. When the job processing was completed.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The detailed state of a job.
    #[prost(enumeration = "job::State", tag = "7")]
    pub state: i32,
    /// Output only. The details of a failure or a cancellation.
    #[prost(string, tag = "8")]
    pub error_message: std::string::String,
    /// Required. Parameters to create a job.
    #[prost(oneof = "job::Input", tags = "2, 3")]
    pub input: ::std::option::Option<job::Input>,
    /// Output only. The current result of the job.
    #[prost(oneof = "job::Output", tags = "9, 10")]
    pub output: ::std::option::Option<job::Output>,
}
pub mod job {
    /// Describes the job state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The job state is unspecified.
        Unspecified = 0,
        /// The job has been just created and processing has not yet begun.
        Queued = 1,
        /// The service is preparing to run the job.
        Preparing = 2,
        /// The job is in progress.
        Running = 3,
        /// The job completed successfully.
        Succeeded = 4,
        /// The job failed.
        /// `error_message` should contain the details of the failure.
        Failed = 5,
        /// The job is being cancelled.
        /// `error_message` should describe the reason for the cancellation.
        Cancelling = 6,
        /// The job has been cancelled.
        /// `error_message` should describe the reason for the cancellation.
        Cancelled = 7,
    }
    /// Required. Parameters to create a job.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        /// Input parameters to create a training job.
        #[prost(message, tag = "2")]
        TrainingInput(super::TrainingInput),
        /// Input parameters to create a prediction job.
        #[prost(message, tag = "3")]
        PredictionInput(super::PredictionInput),
    }
    /// Output only. The current result of the job.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        /// The current training job result.
        #[prost(message, tag = "9")]
        TrainingOutput(super::TrainingOutput),
        /// The current prediction job result.
        #[prost(message, tag = "10")]
        PredictionOutput(super::PredictionOutput),
    }
}
/// Request message for the CreateJob method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobRequest {
    /// Required. The project name.
    ///
    /// Authorization: requires `Editor` role on the specified project.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The job to create.
    #[prost(message, optional, tag = "2")]
    pub job: ::std::option::Option<Job>,
}
/// Request message for the ListJobs method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// Required. The name of the project for which to list jobs.
    ///
    /// Authorization: requires `Viewer` role on the specified project.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Specifies the subset of jobs to retrieve.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Optional. A page token to request the next page of results.
    ///
    /// You get the token from the `next_page_token` field of the response from
    /// the previous call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
    /// Optional. The number of jobs to retrieve per "page" of results. If there
    /// are more remaining results than this number, the response message will
    /// contain a valid value in the `next_page_token` field.
    ///
    /// The default value is 20, and the maximum page size is 100.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
}
/// Response message for the ListJobs method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// The list of jobs.
    #[prost(message, repeated, tag = "1")]
    pub jobs: ::std::vec::Vec<Job>,
    /// Optional. Pass this token as the `page_token` field of the request for a
    /// subsequent call.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for the GetJob method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// Required. The name of the job to get the description of.
    ///
    /// Authorization: requires `Viewer` role on the parent project.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for the CancelJob method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelJobRequest {
    /// Required. The name of the job to cancel.
    ///
    /// Authorization: requires `Editor` role on the parent project.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod job_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to create and manage training and batch prediction jobs."]
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
        #[doc = " Creates a training or a batch prediction job."]
        pub async fn create_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.ml.v1.JobService/CreateJob");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the jobs in the project."]
        pub async fn list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.ml.v1.JobService/ListJobs");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Describes a job."]
        pub async fn get_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.ml.v1.JobService/GetJob");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels a running job."]
        pub async fn cancel_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.ml.v1.JobService/CancelJob");
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
#[doc = r" Generated server implementations."]
pub mod job_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with JobServiceServer."]
    #[async_trait]
    pub trait JobService: Send + Sync + 'static {
        #[doc = " Creates a training or a batch prediction job."]
        async fn create_job(
            &self,
            request: tonic::Request<super::CreateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
        #[doc = " Lists the jobs in the project."]
        async fn list_jobs(
            &self,
            request: tonic::Request<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status>;
        #[doc = " Describes a job."]
        async fn get_job(
            &self,
            request: tonic::Request<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
        #[doc = " Cancels a running job."]
        async fn cancel_job(
            &self,
            request: tonic::Request<super::CancelJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " Service to create and manage training and batch prediction jobs."]
    #[derive(Debug)]
    pub struct JobServiceServer<T: JobService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: JobService> JobServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for JobServiceServer<T>
    where
        T: JobService,
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
                "/google.cloud.ml.v1.JobService/CreateJob" => {
                    #[allow(non_camel_case_types)]
                    struct CreateJobSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::CreateJobRequest> for CreateJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateJobSvc(inner);
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
                "/google.cloud.ml.v1.JobService/ListJobs" => {
                    #[allow(non_camel_case_types)]
                    struct ListJobsSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::ListJobsRequest> for ListJobsSvc<T> {
                        type Response = super::ListJobsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListJobsSvc(inner);
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
                "/google.cloud.ml.v1.JobService/GetJob" => {
                    #[allow(non_camel_case_types)]
                    struct GetJobSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::GetJobRequest> for GetJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetJobSvc(inner);
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
                "/google.cloud.ml.v1.JobService/CancelJob" => {
                    #[allow(non_camel_case_types)]
                    struct CancelJobSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::CancelJobRequest> for CancelJobSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CancelJobSvc(inner);
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
    impl<T: JobService> Clone for JobServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: JobService> Clone for _Inner<T> {
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
/// Represents a machine learning solution.
///
/// A model can have multiple versions, each of which is a deployed, trained
/// model ready to receive prediction requests. The model itself is just a
/// container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// Required. The name specified for the model when it was created.
    ///
    /// The model name must be unique within the project it is created in.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The description specified for the model when it was created.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Output only. The default version of the model. This version will be used to
    /// handle prediction requests that do not specify a version.
    ///
    /// You can change the default version by calling
    /// [projects.methods.versions.setDefault](/ml/reference/rest/v1/projects.models.versions/setDefault).
    #[prost(message, optional, tag = "3")]
    pub default_version: ::std::option::Option<Version>,
    /// Optional. The list of regions where the model is going to be deployed.
    /// Currently only one region per model is supported.
    /// Defaults to 'us-central1' if nothing is set.
    #[prost(string, repeated, tag = "4")]
    pub regions: ::std::vec::Vec<std::string::String>,
    /// Optional. If true, enables StackDriver Logging for online prediction.
    /// Default is false.
    #[prost(bool, tag = "5")]
    pub online_prediction_logging: bool,
}
/// Represents a version of the model.
///
/// Each version is a trained model deployed in the cloud, ready to handle
/// prediction requests. A model can have multiple versions. You can get
/// information about all of the versions of a given model by calling
/// [projects.models.versions.list](/ml/reference/rest/v1/projects.models.versions/list).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Required.The name specified for the version when it was created.
    ///
    /// The version name must be unique within the model it is created in.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The description specified for the version when it was created.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Output only. If true, this version will be used to handle prediction
    /// requests that do not specify a version.
    ///
    /// You can change the default version by calling
    /// [projects.methods.versions.setDefault](/ml/reference/rest/v1/projects.models.versions/setDefault).
    #[prost(bool, tag = "3")]
    pub is_default: bool,
    /// Required. The Google Cloud Storage location of the trained model used to
    /// create the version. See the
    /// [overview of model deployment](/ml/docs/concepts/deployment-overview) for
    /// more informaiton.
    ///
    /// When passing Version to
    /// [projects.models.versions.create](/ml/reference/rest/v1/projects.models.versions/create)
    /// the model service uses the specified location as the source of the model.
    /// Once deployed, the model version is hosted by the prediction service, so
    /// this location is useful only as a historical record.
    #[prost(string, tag = "4")]
    pub deployment_uri: std::string::String,
    /// Output only. The time the version was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the version was last used for prediction.
    #[prost(message, optional, tag = "6")]
    pub last_use_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. The Google Cloud ML runtime version to use for this deployment.
    /// If not set, Google Cloud ML will choose a version.
    #[prost(string, tag = "8")]
    pub runtime_version: std::string::String,
    /// Optional. Manually select the number of nodes to use for serving the
    /// model. If unset (i.e., by default), the number of nodes used to serve
    /// the model automatically scales with traffic. However, care should be
    /// taken to ramp up traffic according to the model's ability to scale. If
    /// your model needs to handle bursts of traffic beyond it's ability to
    /// scale, it is recommended you set this field appropriately.
    #[prost(message, optional, tag = "9")]
    pub manual_scaling: ::std::option::Option<ManualScaling>,
}
/// Options for manually scaling a model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualScaling {
    /// The number of nodes to allocate for this model. These nodes are always up,
    /// starting from the time the model is deployed, so the cost of operating
    /// this model will be proportional to nodes * number of hours since
    /// deployment.
    #[prost(int32, tag = "1")]
    pub nodes: i32,
}
/// Request message for the CreateModel method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModelRequest {
    /// Required. The project name.
    ///
    /// Authorization: requires `Editor` role on the specified project.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The model to create.
    #[prost(message, optional, tag = "2")]
    pub model: ::std::option::Option<Model>,
}
/// Request message for the ListModels method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// Required. The name of the project whose models are to be listed.
    ///
    /// Authorization: requires `Viewer` role on the specified project.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. A page token to request the next page of results.
    ///
    /// You get the token from the `next_page_token` field of the response from
    /// the previous call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
    /// Optional. The number of models to retrieve per "page" of results. If there
    /// are more remaining results than this number, the response message will
    /// contain a valid value in the `next_page_token` field.
    ///
    /// The default value is 20, and the maximum page size is 100.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
}
/// Response message for the ListModels method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// The list of models.
    #[prost(message, repeated, tag = "1")]
    pub models: ::std::vec::Vec<Model>,
    /// Optional. Pass this token as the `page_token` field of the request for a
    /// subsequent call.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for the GetModel method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModelRequest {
    /// Required. The name of the model.
    ///
    /// Authorization: requires `Viewer` role on the parent project.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for the DeleteModel method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteModelRequest {
    /// Required. The name of the model.
    ///
    /// Authorization: requires `Editor` role on the parent project.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Uploads the provided trained model version to Cloud Machine Learning.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateVersionRequest {
    /// Required. The name of the model.
    ///
    /// Authorization: requires `Editor` role on the parent project.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The version details.
    #[prost(message, optional, tag = "2")]
    pub version: ::std::option::Option<Version>,
}
/// Request message for the ListVersions method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsRequest {
    /// Required. The name of the model for which to list the version.
    ///
    /// Authorization: requires `Viewer` role on the parent project.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. A page token to request the next page of results.
    ///
    /// You get the token from the `next_page_token` field of the response from
    /// the previous call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
    /// Optional. The number of versions to retrieve per "page" of results. If
    /// there are more remaining results than this number, the response message
    /// will contain a valid value in the `next_page_token` field.
    ///
    /// The default value is 20, and the maximum page size is 100.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
}
/// Response message for the ListVersions method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionsResponse {
    /// The list of versions.
    #[prost(message, repeated, tag = "1")]
    pub versions: ::std::vec::Vec<Version>,
    /// Optional. Pass this token as the `page_token` field of the request for a
    /// subsequent call.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for the GetVersion method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    /// Required. The name of the version.
    ///
    /// Authorization: requires `Viewer` role on the parent project.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for the DeleteVerionRequest method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVersionRequest {
    /// Required. The name of the version. You can get the names of all the
    /// versions of a model by calling
    /// [projects.models.versions.list](/ml/reference/rest/v1/projects.models.versions/list).
    ///
    /// Authorization: requires `Editor` role on the parent project.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for the SetDefaultVersion request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDefaultVersionRequest {
    /// Required. The name of the version to make the default for the model. You
    /// can get the names of all the versions of a model by calling
    /// [projects.models.versions.list](/ml/reference/rest/v1/projects.models.versions/list).
    ///
    /// Authorization: requires `Editor` role on the parent project.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod model_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Provides methods that create and manage machine learning models and their"]
    #[doc = " versions."]
    #[doc = ""]
    #[doc = " A model in this context is a container for versions. The model can't provide"]
    #[doc = " predictions without first having a version created for it."]
    #[doc = ""]
    #[doc = " Each version is a trained machine learning model, and each is assumed to be"]
    #[doc = " an iteration of the same machine learning problem as the other versions of"]
    #[doc = " the same model."]
    #[doc = ""]
    #[doc = " Your project can define multiple models, each with multiple versions."]
    #[doc = ""]
    #[doc = " The basic life cycle of a model is:"]
    #[doc = ""]
    #[doc = " *   Create and train the machine learning model and save it to a"]
    #[doc = "     Google Cloud Storage location."]
    #[doc = " *   Use"]
    #[doc = "     [projects.models.create](/ml/reference/rest/v1/projects.models/create)"]
    #[doc = "     to make a new model in your project."]
    #[doc = " *   Use"]
    #[doc = "     [projects.models.versions.create](/ml/reference/rest/v1/projects.models.versions/create)"]
    #[doc = "     to deploy your saved model."]
    #[doc = " *   Use [projects.predict](/ml/reference/rest/v1/projects/predict to"]
    #[doc = "     request predictions of a version of your model, or use"]
    #[doc = "     [projects.jobs.create](/ml/reference/rest/v1/projects.jobs/create)"]
    #[doc = "     to start a batch prediction job."]
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
        #[doc = " Creates a model which will later contain one or more versions."]
        #[doc = ""]
        #[doc = " You must add at least one version before you can request predictions from"]
        #[doc = " the model. Add versions by calling"]
        #[doc = " [projects.models.versions.create](/ml/reference/rest/v1/projects.models.versions/create)."]
        pub async fn create_model(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.ml.v1.ModelService/CreateModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the models in a project."]
        #[doc = ""]
        #[doc = " Each project can contain multiple models, and each model can have multiple"]
        #[doc = " versions."]
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
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.ml.v1.ModelService/ListModels");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a model, including its name, the description (if"]
        #[doc = " set), and the default version (if at least one version of the model has"]
        #[doc = " been deployed)."]
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
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.ml.v1.ModelService/GetModel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a model."]
        #[doc = ""]
        #[doc = " You can only delete a model if there are no versions in it. You can delete"]
        #[doc = " versions by calling"]
        #[doc = " [projects.models.versions.delete](/ml/reference/rest/v1/projects.models.versions/delete)."]
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
                "/google.cloud.ml.v1.ModelService/DeleteModel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new version of a model from a trained TensorFlow model."]
        #[doc = ""]
        #[doc = " If the version created in the cloud by this call is the first deployed"]
        #[doc = " version of the specified model, it will be made the default version of the"]
        #[doc = " model. When you add a version to a model that already has one or more"]
        #[doc = " versions, the default version does not automatically change. If you want a"]
        #[doc = " new version to be the default, you must call"]
        #[doc = " [projects.models.versions.setDefault](/ml/reference/rest/v1/projects.models.versions/setDefault)."]
        pub async fn create_version(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateVersionRequest>,
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
                "/google.cloud.ml.v1.ModelService/CreateVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets basic information about all the versions of a model."]
        #[doc = ""]
        #[doc = " If you expect that a model has a lot of versions, or if you need to handle"]
        #[doc = " only a limited number of results at a time, you can request that the list"]
        #[doc = " be retrieved in batches (called pages):"]
        pub async fn list_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVersionsRequest>,
        ) -> Result<tonic::Response<super::ListVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.ml.v1.ModelService/ListVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a model version."]
        #[doc = ""]
        #[doc = " Models can have multiple versions. You can call"]
        #[doc = " [projects.models.versions.list](/ml/reference/rest/v1/projects.models.versions/list)"]
        #[doc = " to get the same information that this method returns for all of the"]
        #[doc = " versions of a model."]
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.ml.v1.ModelService/GetVersion");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a model version."]
        #[doc = ""]
        #[doc = " Each model can have multiple versions deployed and in use at any given"]
        #[doc = " time. Use this method to remove a single version."]
        #[doc = ""]
        #[doc = " Note: You cannot delete the version that is set as the default version"]
        #[doc = " of the model unless it is the only remaining version."]
        pub async fn delete_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteVersionRequest>,
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
                "/google.cloud.ml.v1.ModelService/DeleteVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Designates a version to be the default for the model."]
        #[doc = ""]
        #[doc = " The default version is used for prediction requests made against the model"]
        #[doc = " that don't specify a version."]
        #[doc = ""]
        #[doc = " The first version to be created for a model is automatically set as the"]
        #[doc = " default. You must make any subsequent changes to the default version"]
        #[doc = " setting manually using this method."]
        pub async fn set_default_version(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDefaultVersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.ml.v1.ModelService/SetDefaultVersion",
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
        #[doc = " Creates a model which will later contain one or more versions."]
        #[doc = ""]
        #[doc = " You must add at least one version before you can request predictions from"]
        #[doc = " the model. Add versions by calling"]
        #[doc = " [projects.models.versions.create](/ml/reference/rest/v1/projects.models.versions/create)."]
        async fn create_model(
            &self,
            request: tonic::Request<super::CreateModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>;
        #[doc = " Lists the models in a project."]
        #[doc = ""]
        #[doc = " Each project can contain multiple models, and each model can have multiple"]
        #[doc = " versions."]
        async fn list_models(
            &self,
            request: tonic::Request<super::ListModelsRequest>,
        ) -> Result<tonic::Response<super::ListModelsResponse>, tonic::Status>;
        #[doc = " Gets information about a model, including its name, the description (if"]
        #[doc = " set), and the default version (if at least one version of the model has"]
        #[doc = " been deployed)."]
        async fn get_model(
            &self,
            request: tonic::Request<super::GetModelRequest>,
        ) -> Result<tonic::Response<super::Model>, tonic::Status>;
        #[doc = " Deletes a model."]
        #[doc = ""]
        #[doc = " You can only delete a model if there are no versions in it. You can delete"]
        #[doc = " versions by calling"]
        #[doc = " [projects.models.versions.delete](/ml/reference/rest/v1/projects.models.versions/delete)."]
        async fn delete_model(
            &self,
            request: tonic::Request<super::DeleteModelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Creates a new version of a model from a trained TensorFlow model."]
        #[doc = ""]
        #[doc = " If the version created in the cloud by this call is the first deployed"]
        #[doc = " version of the specified model, it will be made the default version of the"]
        #[doc = " model. When you add a version to a model that already has one or more"]
        #[doc = " versions, the default version does not automatically change. If you want a"]
        #[doc = " new version to be the default, you must call"]
        #[doc = " [projects.models.versions.setDefault](/ml/reference/rest/v1/projects.models.versions/setDefault)."]
        async fn create_version(
            &self,
            request: tonic::Request<super::CreateVersionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Gets basic information about all the versions of a model."]
        #[doc = ""]
        #[doc = " If you expect that a model has a lot of versions, or if you need to handle"]
        #[doc = " only a limited number of results at a time, you can request that the list"]
        #[doc = " be retrieved in batches (called pages):"]
        async fn list_versions(
            &self,
            request: tonic::Request<super::ListVersionsRequest>,
        ) -> Result<tonic::Response<super::ListVersionsResponse>, tonic::Status>;
        #[doc = " Gets information about a model version."]
        #[doc = ""]
        #[doc = " Models can have multiple versions. You can call"]
        #[doc = " [projects.models.versions.list](/ml/reference/rest/v1/projects.models.versions/list)"]
        #[doc = " to get the same information that this method returns for all of the"]
        #[doc = " versions of a model."]
        async fn get_version(
            &self,
            request: tonic::Request<super::GetVersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status>;
        #[doc = " Deletes a model version."]
        #[doc = ""]
        #[doc = " Each model can have multiple versions deployed and in use at any given"]
        #[doc = " time. Use this method to remove a single version."]
        #[doc = ""]
        #[doc = " Note: You cannot delete the version that is set as the default version"]
        #[doc = " of the model unless it is the only remaining version."]
        async fn delete_version(
            &self,
            request: tonic::Request<super::DeleteVersionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Designates a version to be the default for the model."]
        #[doc = ""]
        #[doc = " The default version is used for prediction requests made against the model"]
        #[doc = " that don't specify a version."]
        #[doc = ""]
        #[doc = " The first version to be created for a model is automatically set as the"]
        #[doc = " default. You must make any subsequent changes to the default version"]
        #[doc = " setting manually using this method."]
        async fn set_default_version(
            &self,
            request: tonic::Request<super::SetDefaultVersionRequest>,
        ) -> Result<tonic::Response<super::Version>, tonic::Status>;
    }
    #[doc = " Provides methods that create and manage machine learning models and their"]
    #[doc = " versions."]
    #[doc = ""]
    #[doc = " A model in this context is a container for versions. The model can't provide"]
    #[doc = " predictions without first having a version created for it."]
    #[doc = ""]
    #[doc = " Each version is a trained machine learning model, and each is assumed to be"]
    #[doc = " an iteration of the same machine learning problem as the other versions of"]
    #[doc = " the same model."]
    #[doc = ""]
    #[doc = " Your project can define multiple models, each with multiple versions."]
    #[doc = ""]
    #[doc = " The basic life cycle of a model is:"]
    #[doc = ""]
    #[doc = " *   Create and train the machine learning model and save it to a"]
    #[doc = "     Google Cloud Storage location."]
    #[doc = " *   Use"]
    #[doc = "     [projects.models.create](/ml/reference/rest/v1/projects.models/create)"]
    #[doc = "     to make a new model in your project."]
    #[doc = " *   Use"]
    #[doc = "     [projects.models.versions.create](/ml/reference/rest/v1/projects.models.versions/create)"]
    #[doc = "     to deploy your saved model."]
    #[doc = " *   Use [projects.predict](/ml/reference/rest/v1/projects/predict to"]
    #[doc = "     request predictions of a version of your model, or use"]
    #[doc = "     [projects.jobs.create](/ml/reference/rest/v1/projects.jobs/create)"]
    #[doc = "     to start a batch prediction job."]
    #[derive(Debug)]
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
                "/google.cloud.ml.v1.ModelService/CreateModel" => {
                    #[allow(non_camel_case_types)]
                    struct CreateModelSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService> tonic::server::UnaryService<super::CreateModelRequest> for CreateModelSvc<T> {
                        type Response = super::Model;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateModelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_model(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateModelSvc(inner);
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
                "/google.cloud.ml.v1.ModelService/ListModels" => {
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
                            let fut = async move { (*inner).list_models(request).await };
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
                "/google.cloud.ml.v1.ModelService/GetModel" => {
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
                            let fut = async move { (*inner).get_model(request).await };
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
                "/google.cloud.ml.v1.ModelService/DeleteModel" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteModelSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService> tonic::server::UnaryService<super::DeleteModelRequest> for DeleteModelSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteModelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_model(request).await };
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
                "/google.cloud.ml.v1.ModelService/CreateVersion" => {
                    #[allow(non_camel_case_types)]
                    struct CreateVersionSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService> tonic::server::UnaryService<super::CreateVersionRequest>
                        for CreateVersionSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateVersionSvc(inner);
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
                "/google.cloud.ml.v1.ModelService/ListVersions" => {
                    #[allow(non_camel_case_types)]
                    struct ListVersionsSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService> tonic::server::UnaryService<super::ListVersionsRequest>
                        for ListVersionsSvc<T>
                    {
                        type Response = super::ListVersionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListVersionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_versions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListVersionsSvc(inner);
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
                "/google.cloud.ml.v1.ModelService/GetVersion" => {
                    #[allow(non_camel_case_types)]
                    struct GetVersionSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService> tonic::server::UnaryService<super::GetVersionRequest> for GetVersionSvc<T> {
                        type Response = super::Version;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetVersionSvc(inner);
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
                "/google.cloud.ml.v1.ModelService/DeleteVersion" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteVersionSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService> tonic::server::UnaryService<super::DeleteVersionRequest>
                        for DeleteVersionSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteVersionSvc(inner);
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
                "/google.cloud.ml.v1.ModelService/SetDefaultVersion" => {
                    #[allow(non_camel_case_types)]
                    struct SetDefaultVersionSvc<T: ModelService>(pub Arc<T>);
                    impl<T: ModelService>
                        tonic::server::UnaryService<super::SetDefaultVersionRequest>
                        for SetDefaultVersionSvc<T>
                    {
                        type Response = super::Version;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetDefaultVersionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_default_version(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetDefaultVersionSvc(inner);
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
} // Copyright 2017 Google Inc. All Rights Reserved.
  //
  // Proto file for the Google Cloud Machine Learning Engine.
  // Describes the metadata for longrunning operations.

/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was submitted.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time operation processing started.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time operation processing completed.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Indicates whether a request to cancel this operation has been made.
    #[prost(bool, tag = "4")]
    pub is_cancellation_requested: bool,
    /// The operation type.
    #[prost(enumeration = "operation_metadata::OperationType", tag = "5")]
    pub operation_type: i32,
    /// Contains the name of the model associated with the operation.
    #[prost(string, tag = "6")]
    pub model_name: std::string::String,
    /// Contains the version associated with the operation.
    #[prost(message, optional, tag = "7")]
    pub version: ::std::option::Option<Version>,
}
pub mod operation_metadata {
    /// The operation type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OperationType {
        /// Unspecified operation type.
        Unspecified = 0,
        /// An operation to create a new version.
        CreateVersion = 1,
        /// An operation to delete an existing version.
        DeleteVersion = 2,
        /// An operation to delete an existing model.
        DeleteModel = 3,
    }
}
/// Request for predictions to be issued against a trained model.
///
/// The body of the request is a single JSON object with a single top-level
/// field:
///
/// <dl>
///   <dt>instances</dt>
///   <dd>A JSON array containing values representing the instances to use for
///       prediction.</dd>
/// </dl>
///
/// The structure of each element of the instances list is determined by your
/// model's input definition. Instances can include named inputs or can contain
/// only unlabeled values.
///
/// Not all data includes named inputs. Some instances will be simple
/// JSON values (boolean, number, or string). However, instances are often lists
/// of simple values, or complex nested lists. Here are some examples of request
/// bodies:
///
/// CSV data with each row encoded as a string value:
/// <pre>
/// {"instances": ["1.0,true,\\"x\\"", "-2.0,false,\\"y\\""]}
/// </pre>
/// Plain text:
/// <pre>
/// {"instances": ["the quick brown fox", "la bruja le dio"]}
/// </pre>
/// Sentences encoded as lists of words (vectors of strings):
/// <pre>
/// {
///   "instances": [
///     ["the","quick","brown"],
///     ["la","bruja","le"],
///     ...
///   ]
/// }
/// </pre>
/// Floating point scalar values:
/// <pre>
/// {"instances": [0.0, 1.1, 2.2]}
/// </pre>
/// Vectors of integers:
/// <pre>
/// {
///   "instances": [
///     [0, 1, 2],
///     [3, 4, 5],
///     ...
///   ]
/// }
/// </pre>
/// Tensors (in this case, two-dimensional tensors):
/// <pre>
/// {
///   "instances": [
///     [
///       [0, 1, 2],
///       [3, 4, 5]
///     ],
///     ...
///   ]
/// }
/// </pre>
/// Images can be represented different ways. In this encoding scheme the first
/// two dimensions represent the rows and columns of the image, and the third
/// contains lists (vectors) of the R, G, and B values for each pixel.
/// <pre>
/// {
///   "instances": [
///     [
///       [
///         [138, 30, 66],
///         [130, 20, 56],
///         ...
///       ],
///       [
///         [126, 38, 61],
///         [122, 24, 57],
///         ...
///       ],
///       ...
///     ],
///     ...
///   ]
/// }
/// </pre>
/// JSON strings must be encoded as UTF-8. To send binary data, you must
/// base64-encode the data and mark it as binary. To mark a JSON string
/// as binary, replace it with a JSON object with a single attribute named `b64`:
/// <pre>{"b64": "..."} </pre>
/// For example:
///
/// Two Serialized tf.Examples (fake data, for illustrative purposes only):
/// <pre>
/// {"instances": [{"b64": "X5ad6u"}, {"b64": "IA9j4nx"}]}
/// </pre>
/// Two JPEG image byte strings (fake data, for illustrative purposes only):
/// <pre>
/// {"instances": [{"b64": "ASa8asdf"}, {"b64": "JLK7ljk3"}]}
/// </pre>
/// If your data includes named references, format each instance as a JSON object
/// with the named references as the keys:
///
/// JSON input data to be preprocessed:
/// <pre>
/// {
///   "instances": [
///     {
///       "a": 1.0,
///       "b": true,
///       "c": "x"
///     },
///     {
///       "a": -2.0,
///       "b": false,
///       "c": "y"
///     }
///   ]
/// }
/// </pre>
/// Some models have an underlying TensorFlow graph that accepts multiple input
/// tensors. In this case, you should use the names of JSON name/value pairs to
/// identify the input tensors, as shown in the following exmaples:
///
/// For a graph with input tensor aliases "tag" (string) and "image"
/// (base64-encoded string):
/// <pre>
/// {
///   "instances": [
///     {
///       "tag": "beach",
///       "image": {"b64": "ASa8asdf"}
///     },
///     {
///       "tag": "car",
///       "image": {"b64": "JLK7ljk3"}
///     }
///   ]
/// }
/// </pre>
/// For a graph with input tensor aliases "tag" (string) and "image"
/// (3-dimensional array of 8-bit ints):
/// <pre>
/// {
///   "instances": [
///     {
///       "tag": "beach",
///       "image": [
///         [
///           [138, 30, 66],
///           [130, 20, 56],
///           ...
///         ],
///         [
///           [126, 38, 61],
///           [122, 24, 57],
///           ...
///         ],
///         ...
///       ]
///     },
///     {
///       "tag": "car",
///       "image": [
///         [
///           [255, 0, 102],
///           [255, 0, 97],
///           ...
///         ],
///         [
///           [254, 1, 101],
///           [254, 2, 93],
///           ...
///         ],
///         ...
///       ]
///     },
///     ...
///   ]
/// }
/// </pre>
/// If the call is successful, the response body will contain one prediction
/// entry per instance in the request body. If prediction fails for any
/// instance, the response body will contain no predictions and will contian
/// a single error entry instead.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictRequest {
    /// Required. The resource name of a model or a version.
    ///
    /// Authorization: requires `Viewer` role on the parent project.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    ///
    /// Required. The prediction request body.
    #[prost(message, optional, tag = "2")]
    pub http_body: ::std::option::Option<super::super::super::api::HttpBody>,
}
#[doc = r" Generated client implementations."]
pub mod online_prediction_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Prediction API, which serves predictions for models managed by"]
    #[doc = " ModelService."]
    pub struct OnlinePredictionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OnlinePredictionServiceClient<T>
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
        #[doc = " Performs prediction on the data in the request."]
        #[doc = ""]
        #[doc = " **** REMOVE FROM GENERATED DOCUMENTATION"]
        pub async fn predict(
            &mut self,
            request: impl tonic::IntoRequest<super::PredictRequest>,
        ) -> Result<tonic::Response<super::super::super::super::api::HttpBody>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.ml.v1.OnlinePredictionService/Predict",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for OnlinePredictionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for OnlinePredictionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OnlinePredictionServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod online_prediction_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with OnlinePredictionServiceServer."]
    #[async_trait]
    pub trait OnlinePredictionService: Send + Sync + 'static {
        #[doc = " Performs prediction on the data in the request."]
        #[doc = ""]
        #[doc = " **** REMOVE FROM GENERATED DOCUMENTATION"]
        async fn predict(
            &self,
            request: tonic::Request<super::PredictRequest>,
        ) -> Result<tonic::Response<super::super::super::super::api::HttpBody>, tonic::Status>;
    }
    #[doc = " The Prediction API, which serves predictions for models managed by"]
    #[doc = " ModelService."]
    #[derive(Debug)]
    pub struct OnlinePredictionServiceServer<T: OnlinePredictionService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: OnlinePredictionService> OnlinePredictionServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for OnlinePredictionServiceServer<T>
    where
        T: OnlinePredictionService,
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
                "/google.cloud.ml.v1.OnlinePredictionService/Predict" => {
                    #[allow(non_camel_case_types)]
                    struct PredictSvc<T: OnlinePredictionService>(pub Arc<T>);
                    impl<T: OnlinePredictionService>
                        tonic::server::UnaryService<super::PredictRequest> for PredictSvc<T>
                    {
                        type Response = super::super::super::super::api::HttpBody;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PredictRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).predict(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PredictSvc(inner);
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
    impl<T: OnlinePredictionService> Clone for OnlinePredictionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: OnlinePredictionService> Clone for _Inner<T> {
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
/// Requests service account information associated with a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigRequest {
    /// Required. The project name.
    ///
    /// Authorization: requires `Viewer` role on the specified project.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Returns service account information associated with a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigResponse {
    /// The service account Cloud ML uses to access resources in the project.
    #[prost(string, tag = "1")]
    pub service_account: std::string::String,
    /// The project number for `service_account`.
    #[prost(int64, tag = "2")]
    pub service_account_project: i64,
}
#[doc = r" Generated client implementations."]
pub mod project_management_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Allows retrieving project related information."]
    pub struct ProjectManagementServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ProjectManagementServiceClient<T>
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
        #[doc = " Get the service account information associated with your project. You need"]
        #[doc = " this information in order to grant the service account persmissions for"]
        #[doc = " the Google Cloud Storage location where you put your model training code"]
        #[doc = " for training the model with Google Cloud Machine Learning."]
        pub async fn get_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfigRequest>,
        ) -> Result<tonic::Response<super::GetConfigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.ml.v1.ProjectManagementService/GetConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ProjectManagementServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ProjectManagementServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ProjectManagementServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod project_management_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ProjectManagementServiceServer."]
    #[async_trait]
    pub trait ProjectManagementService: Send + Sync + 'static {
        #[doc = " Get the service account information associated with your project. You need"]
        #[doc = " this information in order to grant the service account persmissions for"]
        #[doc = " the Google Cloud Storage location where you put your model training code"]
        #[doc = " for training the model with Google Cloud Machine Learning."]
        async fn get_config(
            &self,
            request: tonic::Request<super::GetConfigRequest>,
        ) -> Result<tonic::Response<super::GetConfigResponse>, tonic::Status>;
    }
    #[doc = " Allows retrieving project related information."]
    #[derive(Debug)]
    pub struct ProjectManagementServiceServer<T: ProjectManagementService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ProjectManagementService> ProjectManagementServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ProjectManagementServiceServer<T>
    where
        T: ProjectManagementService,
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
                "/google.cloud.ml.v1.ProjectManagementService/GetConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetConfigSvc<T: ProjectManagementService>(pub Arc<T>);
                    impl<T: ProjectManagementService>
                        tonic::server::UnaryService<super::GetConfigRequest> for GetConfigSvc<T>
                    {
                        type Response = super::GetConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetConfigSvc(inner);
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
    impl<T: ProjectManagementService> Clone for ProjectManagementServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ProjectManagementService> Clone for _Inner<T> {
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
