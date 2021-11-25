/// Layer holds metadata specific to a layer of a Docker image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Layer {
    /// Required. The recovered Dockerfile directive used to construct this layer.
    #[prost(enumeration = "layer::Directive", tag = "1")]
    pub directive: i32,
    /// The recovered arguments to the Dockerfile directive.
    #[prost(string, tag = "2")]
    pub arguments: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Layer`.
pub mod layer {
    /// Instructions from Dockerfile.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Directive {
        /// Default value for unsupported/missing directive.
        Unspecified = 0,
        /// <https://docs.docker.com/engine/reference/builder/>
        Maintainer = 1,
        /// <https://docs.docker.com/engine/reference/builder/>
        Run = 2,
        /// <https://docs.docker.com/engine/reference/builder/>
        Cmd = 3,
        /// <https://docs.docker.com/engine/reference/builder/>
        Label = 4,
        /// <https://docs.docker.com/engine/reference/builder/>
        Expose = 5,
        /// <https://docs.docker.com/engine/reference/builder/>
        Env = 6,
        /// <https://docs.docker.com/engine/reference/builder/>
        Add = 7,
        /// <https://docs.docker.com/engine/reference/builder/>
        Copy = 8,
        /// <https://docs.docker.com/engine/reference/builder/>
        Entrypoint = 9,
        /// <https://docs.docker.com/engine/reference/builder/>
        Volume = 10,
        /// <https://docs.docker.com/engine/reference/builder/>
        User = 11,
        /// <https://docs.docker.com/engine/reference/builder/>
        Workdir = 12,
        /// <https://docs.docker.com/engine/reference/builder/>
        Arg = 13,
        /// <https://docs.docker.com/engine/reference/builder/>
        Onbuild = 14,
        /// <https://docs.docker.com/engine/reference/builder/>
        Stopsignal = 15,
        /// <https://docs.docker.com/engine/reference/builder/>
        Healthcheck = 16,
        /// <https://docs.docker.com/engine/reference/builder/>
        Shell = 17,
    }
}
/// A set of properties that uniquely identify a given Docker image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fingerprint {
    /// Required. The layer ID of the final layer in the Docker image's v1
    /// representation.
    #[prost(string, tag = "1")]
    pub v1_name: ::prost::alloc::string::String,
    /// Required. The ordered list of v2 blobs that represent a given image.
    #[prost(string, repeated, tag = "2")]
    pub v2_blob: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The name of the image's v2 blobs computed via:
    ///   \[bottom\] := v2_blob\[bottom\]
    ///   \[N\] := sha256(v2_blob\[N\] + " " + v2_name\[N+1\])
    /// Only the name of the final blob is kept.
    #[prost(string, tag = "3")]
    pub v2_name: ::prost::alloc::string::String,
}
/// Basis describes the base image portion (Note) of the DockerImage
/// relationship. Linked occurrences are derived from this or an
/// equivalent image via:
///   FROM <Basis.resource_url>
/// Or an equivalent reference, e.g. a tag of the resource_url.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Basis {
    /// Required. Immutable. The resource_url for the resource representing the
    /// basis of associated occurrence images.
    #[prost(string, tag = "1")]
    pub resource_url: ::prost::alloc::string::String,
    /// Required. Immutable. The fingerprint of the base image.
    #[prost(message, optional, tag = "2")]
    pub fingerprint: ::core::option::Option<Fingerprint>,
}
/// Details of an image occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Details {
    /// Required. Immutable. The child image derived from the base image.
    #[prost(message, optional, tag = "1")]
    pub derived_image: ::core::option::Option<Derived>,
}
/// Derived describes the derived image portion (Occurrence) of the DockerImage
/// relationship. This image would be produced from a Dockerfile with FROM
/// <DockerImage.Basis in attached Note>.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Derived {
    /// Required. The fingerprint of the derived image.
    #[prost(message, optional, tag = "1")]
    pub fingerprint: ::core::option::Option<Fingerprint>,
    /// Output only. The number of layers by which this image differs from the
    /// associated image basis.
    #[prost(int32, tag = "2")]
    pub distance: i32,
    /// This contains layer-specific metadata, if populated it has length
    /// "distance" and is ordered with \[distance\] being the layer immediately
    /// following the base image and \[1\] being the final layer.
    #[prost(message, repeated, tag = "3")]
    pub layer_info: ::prost::alloc::vec::Vec<Layer>,
    /// Output only. This contains the base image URL for the derived image
    /// occurrence.
    #[prost(string, tag = "4")]
    pub base_resource_url: ::prost::alloc::string::String,
}
