/// The encryption state of the device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceEncryptionStatus {
    /// The encryption status of the device is not specified or not known.
    EncryptionUnspecified = 0,
    /// The device does not support encryption.
    EncryptionUnsupported = 1,
    /// The device supports encryption, but is currently unencrypted.
    Unencrypted = 2,
    /// The device is encrypted.
    Encrypted = 3,
}
/// The operating system type of the device.
/// Next id: 7
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OsType {
    /// The operating system of the device is not specified or not known.
    OsUnspecified = 0,
    /// A desktop Mac operating system.
    DesktopMac = 1,
    /// A desktop Windows operating system.
    DesktopWindows = 2,
    /// A desktop Linux operating system.
    DesktopLinux = 3,
    /// A desktop ChromeOS operating system.
    DesktopChromeOs = 6,
    /// An Android operating system.
    Android = 4,
    /// An iOS operating system.
    Ios = 5,
}
/// The degree to which the device is managed by the Cloud organization.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceManagementLevel {
    /// The device's management level is not specified or not known.
    ManagementUnspecified = 0,
    /// The device is not managed.
    None = 1,
    /// Basic management is enabled, which is generally limited to monitoring and
    /// wiping the corporate account.
    Basic = 2,
    /// Complete device management. This includes more thorough monitoring and the
    /// ability to directly manage the device (such as remote wiping). This can be
    /// enabled through the Android Enterprise Platform.
    Complete = 3,
}
