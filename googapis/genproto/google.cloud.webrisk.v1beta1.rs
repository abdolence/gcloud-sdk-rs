/// Describes an API diff request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeThreatListDiffRequest {
    /// The ThreatList to update.
    #[prost(enumeration = "ThreatType", tag = "1")]
    pub threat_type: i32,
    /// The current version token of the client for the requested list (the
    /// client version that was received from the last successful diff).
    #[prost(bytes = "vec", tag = "2")]
    pub version_token: ::prost::alloc::vec::Vec<u8>,
    /// Required. The constraints associated with this request.
    #[prost(message, optional, tag = "3")]
    pub constraints: ::core::option::Option<compute_threat_list_diff_request::Constraints>,
}
/// Nested message and enum types in `ComputeThreatListDiffRequest`.
pub mod compute_threat_list_diff_request {
    /// The constraints for this diff.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Constraints {
        /// The maximum size in number of entries. The diff will not contain more
        /// entries than this value.  This should be a power of 2 between 2**10 and
        /// 2**20.  If zero, no diff size limit is set.
        #[prost(int32, tag = "1")]
        pub max_diff_entries: i32,
        /// Sets the maximum number of entries that the client is willing to have
        /// in the local database. This should be a power of 2 between 2**10 and
        /// 2**20. If zero, no database size limit is set.
        #[prost(int32, tag = "2")]
        pub max_database_entries: i32,
        /// The compression types supported by the client.
        #[prost(enumeration = "super::CompressionType", repeated, tag = "3")]
        pub supported_compressions: ::prost::alloc::vec::Vec<i32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeThreatListDiffResponse {
    /// The type of response. This may indicate that an action must be taken by the
    /// client when the response is received.
    #[prost(
        enumeration = "compute_threat_list_diff_response::ResponseType",
        tag = "4"
    )]
    pub response_type: i32,
    /// A set of entries to add to a local threat type's list.
    #[prost(message, optional, tag = "5")]
    pub additions: ::core::option::Option<ThreatEntryAdditions>,
    /// A set of entries to remove from a local threat type's list.
    /// This field may be empty.
    #[prost(message, optional, tag = "6")]
    pub removals: ::core::option::Option<ThreatEntryRemovals>,
    /// The new opaque client version token.
    #[prost(bytes = "vec", tag = "7")]
    pub new_version_token: ::prost::alloc::vec::Vec<u8>,
    /// The expected SHA256 hash of the client state; that is, of the sorted list
    /// of all hashes present in the database after applying the provided diff.
    /// If the client state doesn't match the expected state, the client must
    /// disregard this diff and retry later.
    #[prost(message, optional, tag = "8")]
    pub checksum: ::core::option::Option<compute_threat_list_diff_response::Checksum>,
    /// The soonest the client should wait before issuing any diff
    /// request. Querying sooner is unlikely to produce a meaningful diff.
    /// Waiting longer is acceptable considering the use case.
    /// If this field is not set clients may update as soon as they want.
    #[prost(message, optional, tag = "2")]
    pub recommended_next_diff: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `ComputeThreatListDiffResponse`.
pub mod compute_threat_list_diff_response {
    /// The expected state of a client's local database.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Checksum {
        /// The SHA256 hash of the client state; that is, of the sorted list of all
        /// hashes present in the database.
        #[prost(bytes = "vec", tag = "1")]
        pub sha256: ::prost::alloc::vec::Vec<u8>,
    }
    /// The type of response sent to the client.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseType {
        /// Unknown.
        Unspecified = 0,
        /// Partial updates are applied to the client's existing local database.
        Diff = 1,
        /// Full updates resets the client's entire local database. This means
        /// that either the client had no state, was seriously out-of-date,
        /// or the client is believed to be corrupt.
        Reset = 2,
    }
}
/// Request to check URI entries against threatLists.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchUrisRequest {
    /// Required. The URI to be checked for matches.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Required. The ThreatLists to search in.
    #[prost(enumeration = "ThreatType", repeated, packed = "false", tag = "2")]
    pub threat_types: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchUrisResponse {
    /// The threat list matches. This may be empty if the URI is on no list.
    #[prost(message, optional, tag = "1")]
    pub threat: ::core::option::Option<search_uris_response::ThreatUri>,
}
/// Nested message and enum types in `SearchUrisResponse`.
pub mod search_uris_response {
    /// Contains threat information on a matching uri.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThreatUri {
        /// The ThreatList this threat belongs to.
        #[prost(enumeration = "super::ThreatType", repeated, tag = "1")]
        pub threat_types: ::prost::alloc::vec::Vec<i32>,
        /// The cache lifetime for the returned match. Clients must not cache this
        /// response past this timestamp to avoid false positives.
        #[prost(message, optional, tag = "2")]
        pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// Request to return full hashes matched by the provided hash prefixes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchHashesRequest {
    /// A hash prefix, consisting of the most significant 4-32 bytes of a SHA256
    /// hash. For JSON requests, this field is base64-encoded.
    #[prost(bytes = "vec", tag = "1")]
    pub hash_prefix: ::prost::alloc::vec::Vec<u8>,
    /// Required. The ThreatLists to search in.
    #[prost(enumeration = "ThreatType", repeated, packed = "false", tag = "2")]
    pub threat_types: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchHashesResponse {
    /// The full hashes that matched the requested prefixes.
    /// The hash will be populated in the key.
    #[prost(message, repeated, tag = "1")]
    pub threats: ::prost::alloc::vec::Vec<search_hashes_response::ThreatHash>,
    /// For requested entities that did not match the threat list, how long to
    /// cache the response until.
    #[prost(message, optional, tag = "2")]
    pub negative_expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `SearchHashesResponse`.
pub mod search_hashes_response {
    /// Contains threat information on a matching hash.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThreatHash {
        /// The ThreatList this threat belongs to.
        /// This must contain at least one entry.
        #[prost(enumeration = "super::ThreatType", repeated, tag = "1")]
        pub threat_types: ::prost::alloc::vec::Vec<i32>,
        /// A 32 byte SHA256 hash. This field is in binary format. For JSON
        /// requests, hashes are base64-encoded.
        #[prost(bytes = "vec", tag = "2")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        /// The cache lifetime for the returned match. Clients must not cache this
        /// response past this timestamp to avoid false positives.
        #[prost(message, optional, tag = "3")]
        pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// Contains the set of entries to add to a local database.
/// May contain a combination of compressed and raw data in a single response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreatEntryAdditions {
    /// The raw SHA256-formatted entries.
    /// Repeated to allow returning sets of hashes with different prefix sizes.
    #[prost(message, repeated, tag = "1")]
    pub raw_hashes: ::prost::alloc::vec::Vec<RawHashes>,
    /// The encoded 4-byte prefixes of SHA256-formatted entries, using a
    /// Golomb-Rice encoding. The hashes are converted to uint32, sorted in
    /// ascending order, then delta encoded and stored as encoded_data.
    #[prost(message, optional, tag = "2")]
    pub rice_hashes: ::core::option::Option<RiceDeltaEncoding>,
}
/// Contains the set of entries to remove from a local database.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThreatEntryRemovals {
    /// The raw removal indices for a local list.
    #[prost(message, optional, tag = "1")]
    pub raw_indices: ::core::option::Option<RawIndices>,
    /// The encoded local, lexicographically-sorted list indices, using a
    /// Golomb-Rice encoding. Used for sending compressed removal indices. The
    /// removal indices (uint32) are sorted in ascending order, then delta encoded
    /// and stored as encoded_data.
    #[prost(message, optional, tag = "2")]
    pub rice_indices: ::core::option::Option<RiceDeltaEncoding>,
}
/// A set of raw indices to remove from a local list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawIndices {
    /// The indices to remove from a lexicographically-sorted local list.
    #[prost(int32, repeated, tag = "1")]
    pub indices: ::prost::alloc::vec::Vec<i32>,
}
/// The uncompressed threat entries in hash format.
/// Hashes can be anywhere from 4 to 32 bytes in size. A large majority are 4
/// bytes, but some hashes are lengthened if they collide with the hash of a
/// popular URI.
///
/// Used for sending ThreatEntryAdditons to clients that do not support
/// compression, or when sending non-4-byte hashes to clients that do support
/// compression.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawHashes {
    /// The number of bytes for each prefix encoded below.  This field can be
    /// anywhere from 4 (shortest prefix) to 32 (full SHA256 hash).
    #[prost(int32, tag = "1")]
    pub prefix_size: i32,
    /// The hashes, in binary format, concatenated into one long string. Hashes are
    /// sorted in lexicographic order. For JSON API users, hashes are
    /// base64-encoded.
    #[prost(bytes = "vec", tag = "2")]
    pub raw_hashes: ::prost::alloc::vec::Vec<u8>,
}
/// The Rice-Golomb encoded data. Used for sending compressed 4-byte hashes or
/// compressed removal indices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RiceDeltaEncoding {
    /// The offset of the first entry in the encoded data, or, if only a single
    /// integer was encoded, that single integer's value. If the field is empty or
    /// missing, assume zero.
    #[prost(int64, tag = "1")]
    pub first_value: i64,
    /// The Golomb-Rice parameter, which is a number between 2 and 28. This field
    /// is missing (that is, zero) if `num_entries` is zero.
    #[prost(int32, tag = "2")]
    pub rice_parameter: i32,
    /// The number of entries that are delta encoded in the encoded data. If only a
    /// single integer was encoded, this will be zero and the single value will be
    /// stored in `first_value`.
    #[prost(int32, tag = "3")]
    pub entry_count: i32,
    /// The encoded deltas that are encoded using the Golomb-Rice coder.
    #[prost(bytes = "vec", tag = "4")]
    pub encoded_data: ::prost::alloc::vec::Vec<u8>,
}
/// The type of threat. This maps dirrectly to the threat list a threat may
/// belong to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ThreatType {
    /// Unknown.
    Unspecified = 0,
    /// Malware targeting any platform.
    Malware = 1,
    /// Social engineering targeting any platform.
    SocialEngineering = 2,
    /// Unwanted software targeting any platform.
    UnwantedSoftware = 3,
}
/// The ways in which threat entry sets can be compressed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompressionType {
    /// Unknown.
    Unspecified = 0,
    /// Raw, uncompressed data.
    Raw = 1,
    /// Rice-Golomb encoded data.
    Rice = 2,
}
#[doc = r" Generated client implementations."]
pub mod web_risk_service_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Web Risk v1beta1 API defines an interface to detect malicious URLs on your"]
    #[doc = " website and in client applications."]
    #[derive(Debug, Clone)]
    pub struct WebRiskServiceV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> WebRiskServiceV1Beta1Client<T>
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
        ) -> WebRiskServiceV1Beta1Client<InterceptedService<T, F>>
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
            WebRiskServiceV1Beta1Client::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Gets the most recent threat list diffs."]
        pub async fn compute_threat_list_diff(
            &mut self,
            request: impl tonic::IntoRequest<super::ComputeThreatListDiffRequest>,
        ) -> Result<tonic::Response<super::ComputeThreatListDiffResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.webrisk.v1beta1.WebRiskServiceV1Beta1/ComputeThreatListDiff",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method is used to check whether a URI is on a given threatList."]
        pub async fn search_uris(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchUrisRequest>,
        ) -> Result<tonic::Response<super::SearchUrisResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.webrisk.v1beta1.WebRiskServiceV1Beta1/SearchUris",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the full hashes that match the requested hash prefix."]
        #[doc = " This is used after a hash prefix is looked up in a threatList"]
        #[doc = " and there is a match. The client side threatList only holds partial hashes"]
        #[doc = " so the client must query this method to determine if there is a full"]
        #[doc = " hash match of a threat."]
        pub async fn search_hashes(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchHashesRequest>,
        ) -> Result<tonic::Response<super::SearchHashesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.webrisk.v1beta1.WebRiskServiceV1Beta1/SearchHashes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
