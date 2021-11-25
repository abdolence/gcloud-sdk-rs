/// Assessment task details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssessmentTaskDetails {
    /// Required. The Cloud Storage path for assessment input files.
    #[prost(string, tag = "1")]
    pub input_path: ::prost::alloc::string::String,
    /// Required. The BigQuery dataset for output.
    #[prost(string, tag = "2")]
    pub output_dataset: ::prost::alloc::string::String,
    /// Optional. An optional Cloud Storage path to write the query logs (which is then used
    /// as an input path on the translation task)
    #[prost(string, tag = "3")]
    pub querylogs_path: ::prost::alloc::string::String,
    /// Required. The data source or data warehouse type (eg: TERADATA/REDSHIFT) from which
    /// the input data is extracted.
    #[prost(string, tag = "4")]
    pub data_source: ::prost::alloc::string::String,
}
