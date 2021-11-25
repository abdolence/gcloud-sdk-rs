/// This represents a particular channel of distribution for a given package.
/// E.g., Debian's jessie-backports dpkg mirror.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Distribution {
    /// Required. The cpe_uri in [CPE format](<https://cpe.mitre.org/specification/>)
    /// denoting the package manager version distributing a package.
    #[prost(string, tag = "1")]
    pub cpe_uri: ::prost::alloc::string::String,
    /// The CPU architecture for which packages in this distribution channel were
    /// built.
    #[prost(enumeration = "Architecture", tag = "2")]
    pub architecture: i32,
    /// The latest available version of this package in this distribution channel.
    #[prost(message, optional, tag = "3")]
    pub latest_version: ::core::option::Option<Version>,
    /// A freeform string denoting the maintainer of this package.
    #[prost(string, tag = "4")]
    pub maintainer: ::prost::alloc::string::String,
    /// The distribution channel-specific homepage for this package.
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
    /// The distribution channel-specific description of this package.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
}
/// An occurrence of a particular package installation found within a system's
/// filesystem. E.g., glibc was found in `/var/lib/dpkg/status`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Required. The CPE URI in [CPE format](<https://cpe.mitre.org/specification/>)
    /// denoting the package manager version distributing a package.
    #[prost(string, tag = "1")]
    pub cpe_uri: ::prost::alloc::string::String,
    /// The version installed at this location.
    #[prost(message, optional, tag = "2")]
    pub version: ::core::option::Option<Version>,
    /// The path from which we gathered that this package/version is installed.
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
}
/// This represents a particular package that is distributed over various
/// channels. E.g., glibc (aka libc6) is distributed by many, at various
/// versions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    /// Required. Immutable. The name of the package.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The various channels by which a package is distributed.
    #[prost(message, repeated, tag = "10")]
    pub distribution: ::prost::alloc::vec::Vec<Distribution>,
}
/// Details of a package occurrence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Details {
    /// Required. Where the package was installed.
    #[prost(message, optional, tag = "1")]
    pub installation: ::core::option::Option<Installation>,
}
/// This represents how a particular software package may be installed on a
/// system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Installation {
    /// Output only. The name of the installed package.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. All of the places within the filesystem versions of this package
    /// have been found.
    #[prost(message, repeated, tag = "2")]
    pub location: ::prost::alloc::vec::Vec<Location>,
}
/// Version contains structured information about the version of a package.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// Used to correct mistakes in the version numbering scheme.
    #[prost(int32, tag = "1")]
    pub epoch: i32,
    /// Required only when version kind is NORMAL. The main part of the version
    /// name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The iteration of the package build from the above version.
    #[prost(string, tag = "3")]
    pub revision: ::prost::alloc::string::String,
    /// Required. Distinguishes between sentinel MIN/MAX versions and normal
    /// versions.
    #[prost(enumeration = "version::VersionKind", tag = "4")]
    pub kind: i32,
}
/// Nested message and enum types in `Version`.
pub mod version {
    /// Whether this is an ordinary package version or a sentinel MIN/MAX version.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VersionKind {
        /// Unknown.
        Unspecified = 0,
        /// A standard package version.
        Normal = 1,
        /// A special version representing negative infinity.
        Minimum = 2,
        /// A special version representing positive infinity.
        Maximum = 3,
    }
}
/// Instruction set architectures supported by various package managers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Architecture {
    /// Unknown architecture.
    Unspecified = 0,
    /// X86 architecture.
    X86 = 1,
    /// X64 architecture.
    X64 = 2,
}
