/// TestMatrix captures all details about a test. It contains the environment
/// configuration, test specification, test executions and overall state and
/// outcome.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestMatrix {
    /// Output only. Unique id set by the service.
    #[prost(string, tag = "1")]
    pub test_matrix_id: ::prost::alloc::string::String,
    /// The cloud project that owns the test matrix.
    #[prost(string, tag = "7")]
    pub project_id: ::prost::alloc::string::String,
    /// Information about the client which invoked the test.
    #[prost(message, optional, tag = "10")]
    pub client_info: ::core::option::Option<ClientInfo>,
    /// Required. How to run the test.
    #[prost(message, optional, tag = "3")]
    pub test_specification: ::core::option::Option<TestSpecification>,
    /// Required. The devices the tests are being executed on.
    #[prost(message, optional, tag = "4")]
    pub environment_matrix: ::core::option::Option<EnvironmentMatrix>,
    /// Output only. The list of test executions that the service creates for
    /// this matrix.
    #[prost(message, repeated, tag = "5")]
    pub test_executions: ::prost::alloc::vec::Vec<TestExecution>,
    /// Required. Where the results for the matrix are written.
    #[prost(message, optional, tag = "6")]
    pub result_storage: ::core::option::Option<ResultStorage>,
    /// Output only. Indicates the current progress of the test matrix.
    #[prost(enumeration = "TestState", tag = "8")]
    pub state: i32,
    /// Output only. The time this test matrix was initially created.
    #[prost(message, optional, tag = "9")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Describes why the matrix is considered invalid.
    /// Only useful for matrices in the INVALID state.
    #[prost(enumeration = "InvalidMatrixDetails", tag = "11")]
    pub invalid_matrix_details: i32,
    /// The number of times a TestExecution should be re-attempted if one or more
    /// of its test cases fail for any reason.
    /// The maximum number of reruns allowed is 10.
    ///
    /// Default is 0, which implies no reruns.
    #[prost(int32, tag = "13")]
    pub flaky_test_attempts: i32,
    /// Output Only. The overall outcome of the test.
    /// Only set when the test matrix state is FINISHED.
    #[prost(enumeration = "OutcomeSummary", tag = "14")]
    pub outcome_summary: i32,
    /// If true, only a single attempt at most will be made to run each
    /// execution/shard in the matrix. Flaky test attempts are not affected.
    ///
    /// Normally, 2 or more attempts are made if a potential infrastructure issue
    /// is detected.
    ///
    /// This feature is for latency sensitive workloads. The incidence of
    /// execution failures may be significantly greater for fail-fast matrices
    /// and support is more limited because of that expectation.
    #[prost(bool, tag = "17")]
    pub fail_fast: bool,
}
/// A single test executed in a single environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestExecution {
    /// Output only. Unique id set by the service.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Output only. Id of the containing TestMatrix.
    #[prost(string, tag = "9")]
    pub matrix_id: ::prost::alloc::string::String,
    /// Output only. The cloud project that owns the test execution.
    #[prost(string, tag = "10")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. How to run the test.
    #[prost(message, optional, tag = "3")]
    pub test_specification: ::core::option::Option<TestSpecification>,
    /// Output only. Details about the shard.
    #[prost(message, optional, tag = "12")]
    pub shard: ::core::option::Option<Shard>,
    /// Output only. How the host machine(s) are configured.
    #[prost(message, optional, tag = "4")]
    pub environment: ::core::option::Option<Environment>,
    /// Output only. Indicates the current progress of the test execution
    /// (e.g., FINISHED).
    #[prost(enumeration = "TestState", tag = "5")]
    pub state: i32,
    /// Output only. Where the results for this execution are written.
    #[prost(message, optional, tag = "11")]
    pub tool_results_step: ::core::option::Option<ToolResultsStep>,
    /// Output only. The time this test execution was initially created.
    #[prost(message, optional, tag = "7")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Additional details about the running test.
    #[prost(message, optional, tag = "8")]
    pub test_details: ::core::option::Option<TestDetails>,
}
/// A description of how to run the test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSpecification {
    /// Max time a test execution is allowed to run before it is
    /// automatically cancelled.
    /// The default value is 5 min.
    #[prost(message, optional, tag = "1")]
    pub test_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Disables video recording. May reduce test latency.
    #[prost(bool, tag = "10")]
    pub disable_video_recording: bool,
    /// Disables performance metrics recording. May reduce test latency.
    #[prost(bool, tag = "11")]
    pub disable_performance_metrics: bool,
    /// Test setup requirements.
    #[prost(oneof = "test_specification::Setup", tags = "6, 14")]
    pub setup: ::core::option::Option<test_specification::Setup>,
    /// Required. The type of test to run.
    #[prost(oneof = "test_specification::Test", tags = "2, 3, 9, 13, 15")]
    pub test: ::core::option::Option<test_specification::Test>,
}
/// Nested message and enum types in `TestSpecification`.
pub mod test_specification {
    /// Test setup requirements.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Setup {
        /// Test setup requirements for Android e.g. files to install, bootstrap
        /// scripts.
        #[prost(message, tag = "6")]
        TestSetup(super::TestSetup),
        /// Test setup requirements for iOS.
        #[prost(message, tag = "14")]
        IosTestSetup(super::IosTestSetup),
    }
    /// Required. The type of test to run.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Test {
        /// An Android instrumentation test.
        #[prost(message, tag = "2")]
        AndroidInstrumentationTest(super::AndroidInstrumentationTest),
        /// An Android robo test.
        #[prost(message, tag = "3")]
        AndroidRoboTest(super::AndroidRoboTest),
        /// An Android Application with a Test Loop.
        #[prost(message, tag = "9")]
        AndroidTestLoop(super::AndroidTestLoop),
        /// An iOS XCTest, via an .xctestrun file.
        #[prost(message, tag = "13")]
        IosXcTest(super::IosXcTest),
        /// An iOS application with a test loop.
        #[prost(message, tag = "15")]
        IosTestLoop(super::IosTestLoop),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystraceSetup {
    /// Systrace duration in seconds.
    /// Should be between 1 and 30 seconds. 0 disables systrace.
    #[prost(int32, tag = "1")]
    pub duration_seconds: i32,
}
/// A description of how to set up the Android device prior to running the test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSetup {
    /// List of files to push to the device before starting the test.
    #[prost(message, repeated, tag = "1")]
    pub files_to_push: ::prost::alloc::vec::Vec<DeviceFile>,
    /// List of directories on the device to upload to GCS at the end of the test;
    /// they must be absolute paths under /sdcard, /storage or /data/local/tmp.
    /// Path names are restricted to characters a-z A-Z 0-9 _ - . + and /
    ///
    /// Note: The paths /sdcard and /data will be made available and treated as
    /// implicit path substitutions. E.g. if /sdcard on a particular device does
    /// not map to external storage, the system will replace it with the external
    /// storage path prefix for that device.
    #[prost(string, repeated, tag = "2")]
    pub directories_to_pull: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// APKs to install in addition to those being directly tested.
    /// Currently capped at 100.
    #[prost(message, repeated, tag = "3")]
    pub additional_apks: ::prost::alloc::vec::Vec<Apk>,
    /// The device will be logged in on this account for the duration of the test.
    #[prost(message, optional, tag = "4")]
    pub account: ::core::option::Option<Account>,
    /// The network traffic profile used for running the test.
    /// Available network profiles can be queried by using the
    /// NETWORK_CONFIGURATION environment type when calling
    /// TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.
    #[prost(string, tag = "5")]
    pub network_profile: ::prost::alloc::string::String,
    /// Environment variables to set for the test (only applicable for
    /// instrumentation tests).
    #[prost(message, repeated, tag = "6")]
    pub environment_variables: ::prost::alloc::vec::Vec<EnvironmentVariable>,
    /// Systrace configuration for the run.
    /// If set a systrace will be taken, starting on test start and lasting for the
    /// configured duration. The systrace file thus obtained is put in the results
    /// bucket together with the other artifacts from the run.
    #[prost(message, optional, tag = "9")]
    pub systrace: ::core::option::Option<SystraceSetup>,
    /// Whether to prevent all runtime permissions to be granted at app install
    #[prost(bool, tag = "23")]
    pub dont_autogrant_permissions: bool,
}
/// A description of how to set up an iOS device prior to running the test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosTestSetup {
    /// The network traffic profile used for running the test.
    /// Available network profiles can be queried by using the
    /// NETWORK_CONFIGURATION environment type when calling
    /// TestEnvironmentDiscoveryService.GetTestEnvironmentCatalog.
    #[prost(string, tag = "1")]
    pub network_profile: ::prost::alloc::string::String,
    /// iOS apps to install in addition to those being directly tested.
    #[prost(message, repeated, tag = "2")]
    pub additional_ipas: ::prost::alloc::vec::Vec<FileReference>,
    /// List of files to push to the device before starting the test.
    #[prost(message, repeated, tag = "3")]
    pub push_files: ::prost::alloc::vec::Vec<IosDeviceFile>,
    /// List of directories on the device to upload to Cloud Storage at the end of
    /// the test.
    ///
    /// Directories should either be in a shared directory
    /// (e.g. /private/var/mobile/Media) or within an accessible directory inside
    /// the app's filesystem (e.g. /Documents) by specifying the bundle id.
    #[prost(message, repeated, tag = "4")]
    pub pull_directories: ::prost::alloc::vec::Vec<IosDeviceFile>,
}
/// A key-value pair passed as an environment variable to the test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentVariable {
    /// Key for the environment variable.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Value for the environment variable.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Identifies an account and how to log into it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Required. The type of account, based what it's for (e.g. Google) and what
    /// its login mechanism is (e.g. username and password).
    #[prost(oneof = "account::AccountType", tags = "1")]
    pub account_type: ::core::option::Option<account::AccountType>,
}
/// Nested message and enum types in `Account`.
pub mod account {
    /// Required. The type of account, based what it's for (e.g. Google) and what
    /// its login mechanism is (e.g. username and password).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccountType {
        /// An automatic google login account.
        #[prost(message, tag = "1")]
        GoogleAuto(super::GoogleAuto),
    }
}
/// Enables automatic Google account login.
/// If set, the service automatically generates a Google test account and adds
/// it to the device, before executing the test. Note that test accounts might be
/// reused.
/// Many applications show their full set of functionalities when an account is
/// present on the device. Logging into the device with these generated accounts
/// allows testing more functionalities.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAuto {}
/// An Android package file to install.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Apk {
    /// The path to an APK to be installed on the device before the test begins.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<FileReference>,
    /// The java package for the APK to be installed.
    /// Value is determined by examining the application's manifest.
    #[prost(string, tag = "2")]
    pub package_name: ::prost::alloc::string::String,
}
/// An Android App Bundle file format, containing a BundleConfig.pb file,
/// a base module directory, zero or more dynamic feature module directories.
/// <p>See <https://developer.android.com/guide/app-bundle/build> for guidance on
/// building App Bundles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppBundle {
    /// Required. Bundle location information.
    #[prost(oneof = "app_bundle::Bundle", tags = "1")]
    pub bundle: ::core::option::Option<app_bundle::Bundle>,
}
/// Nested message and enum types in `AppBundle`.
pub mod app_bundle {
    /// Required. Bundle location information.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Bundle {
        /// .aab file representing the app bundle under test.
        #[prost(message, tag = "1")]
        BundleLocation(super::FileReference),
    }
}
/// A single device file description.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceFile {
    /// Required.
    #[prost(oneof = "device_file::DeviceFile", tags = "1, 2")]
    pub device_file: ::core::option::Option<device_file::DeviceFile>,
}
/// Nested message and enum types in `DeviceFile`.
pub mod device_file {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeviceFile {
        /// A reference to an opaque binary blob file.
        #[prost(message, tag = "1")]
        ObbFile(super::ObbFile),
        /// A reference to a regular file.
        #[prost(message, tag = "2")]
        RegularFile(super::RegularFile),
    }
}
/// An opaque binary blob file to install on the device before the test starts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObbFile {
    /// Required. OBB file name which must conform to the format as specified by
    /// Android
    /// e.g. \[main|patch\].0300110.com.example.android.obb
    /// which will be installed into
    ///   \<shared-storage\>/Android/obb/\<package-name\>/
    /// on the device.
    #[prost(string, tag = "1")]
    pub obb_file_name: ::prost::alloc::string::String,
    /// Required. Opaque Binary Blob (OBB) file(s) to install on the device.
    #[prost(message, optional, tag = "2")]
    pub obb: ::core::option::Option<FileReference>,
}
/// A file or directory to install on the device before the test starts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegularFile {
    /// Required. The source file.
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<FileReference>,
    /// Required. Where to put the content on the device. Must be an absolute,
    /// allowlisted path. If the file exists, it will be replaced.
    /// The following device-side directories and any of their subdirectories are
    /// allowlisted:
    /// <p>${EXTERNAL_STORAGE}, /sdcard, or /storage</p>
    /// <p>${ANDROID_DATA}/local/tmp, or /data/local/tmp</p>
    /// <p>Specifying a path outside of these directory trees is invalid.
    ///
    /// <p> The paths /sdcard and /data will be made available and treated as
    /// implicit path substitutions. E.g. if /sdcard on a particular device does
    /// not map to external storage, the system will replace it with the external
    /// storage path prefix for that device and copy the file there.
    ///
    /// <p> It is strongly advised to use the <a href=
    /// "<http://developer.android.com/reference/android/os/Environment.html">>
    /// Environment API</a> in app and test code to access files on the device in a
    /// portable way.
    #[prost(string, tag = "2")]
    pub device_path: ::prost::alloc::string::String,
}
/// A file or directory to install on the device before the test starts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosDeviceFile {
    /// The source file
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<FileReference>,
    /// The bundle id of the app where this file lives.
    ///
    /// iOS apps sandbox their own filesystem, so app files must specify which app
    /// installed on the device.
    #[prost(string, tag = "2")]
    pub bundle_id: ::prost::alloc::string::String,
    /// Location of the file on the device, inside the app's sandboxed filesystem
    #[prost(string, tag = "3")]
    pub device_path: ::prost::alloc::string::String,
}
/// A test of an Android Application with a Test Loop.
/// The intent \<intent-name\> will be implicitly added, since Games is the only
/// user of this api, for the time being.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidTestLoop {
    /// The java package for the application under test.
    /// The default is determined by examining the application's manifest.
    #[prost(string, tag = "2")]
    pub app_package_id: ::prost::alloc::string::String,
    /// The list of scenarios that should be run during the test.
    /// The default is all test loops, derived from the application's
    /// manifest.
    #[prost(int32, repeated, tag = "3")]
    pub scenarios: ::prost::alloc::vec::Vec<i32>,
    /// The list of scenario labels that should be run during the test.
    /// The scenario labels should map to labels defined in the application's
    /// manifest. For example, player_experience and
    /// com.google.test.loops.player_experience add all of the loops labeled in the
    /// manifest with the com.google.test.loops.player_experience name to the
    /// execution.
    /// Scenarios can also be specified in the scenarios field.
    #[prost(string, repeated, tag = "4")]
    pub scenario_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The Android package to test.
    #[prost(oneof = "android_test_loop::AppUnderTest", tags = "1, 5")]
    pub app_under_test: ::core::option::Option<android_test_loop::AppUnderTest>,
}
/// Nested message and enum types in `AndroidTestLoop`.
pub mod android_test_loop {
    /// Required. The Android package to test.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AppUnderTest {
        /// The APK for the application under test.
        #[prost(message, tag = "1")]
        AppApk(super::FileReference),
        /// A multi-apk app bundle for the application under test.
        #[prost(message, tag = "5")]
        AppBundle(super::AppBundle),
    }
}
/// A test of an iOS application that uses the XCTest framework.
/// Xcode supports the option to "build for testing", which generates an
/// .xctestrun file that contains a test specification (arguments, test methods,
/// etc). This test type accepts a zip file containing the .xctestrun file and
/// the corresponding contents of the Build/Products directory that contains all
/// the binaries needed to run the tests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosXcTest {
    /// Required. The .zip containing the .xctestrun file and the contents of the
    /// DerivedData/Build/Products directory.
    /// The .xctestrun file in this zip is ignored if the xctestrun field is
    /// specified.
    #[prost(message, optional, tag = "1")]
    pub tests_zip: ::core::option::Option<FileReference>,
    /// An .xctestrun file that will override the .xctestrun file in the
    /// tests zip. Because the .xctestrun file contains environment variables along
    /// with test methods to run and/or ignore, this can be useful for sharding
    /// tests. Default is taken from the tests zip.
    #[prost(message, optional, tag = "2")]
    pub xctestrun: ::core::option::Option<FileReference>,
    /// The Xcode version that should be used for the test.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    /// Defaults to the latest Xcode version Firebase Test Lab supports.
    #[prost(string, tag = "3")]
    pub xcode_version: ::prost::alloc::string::String,
    /// Output only. The bundle id for the application under test.
    #[prost(string, tag = "4")]
    pub app_bundle_id: ::prost::alloc::string::String,
    /// The option to test special app entitlements. Setting this would re-sign the
    /// app having special entitlements with an explicit application-identifier.
    /// Currently supports testing aps-environment entitlement.
    #[prost(bool, tag = "6")]
    pub test_special_entitlements: bool,
}
/// A test of an iOS application that implements one or more game loop scenarios.
/// This test type accepts an archived application (.ipa file) and a list of
/// integer scenarios that will be executed on the app sequentially.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosTestLoop {
    /// Required. The .ipa of the application to test.
    #[prost(message, optional, tag = "1")]
    pub app_ipa: ::core::option::Option<FileReference>,
    /// The list of scenarios that should be run during the test. Defaults to the
    /// single scenario 0 if unspecified.
    #[prost(int32, repeated, tag = "2")]
    pub scenarios: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The bundle id for the application under test.
    #[prost(string, tag = "3")]
    pub app_bundle_id: ::prost::alloc::string::String,
}
/// A test of an Android application that can control an Android component
/// independently of its normal lifecycle.
/// Android instrumentation tests run an application APK and test APK inside the
/// same process on a virtual or physical AndroidDevice.  They also specify
/// a test runner class, such as com.google.GoogleTestRunner, which can vary
/// on the specific instrumentation framework chosen.
///
/// See <<http://developer.android.com/tools/testing/testing_android.html>> for
/// more information on types of Android tests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidInstrumentationTest {
    /// Required. The APK containing the test code to be executed.
    #[prost(message, optional, tag = "2")]
    pub test_apk: ::core::option::Option<FileReference>,
    /// The java package for the application under test.
    /// The default value is determined by examining the application's manifest.
    #[prost(string, tag = "3")]
    pub app_package_id: ::prost::alloc::string::String,
    /// The java package for the test to be executed.
    /// The default value is determined by examining the application's manifest.
    #[prost(string, tag = "4")]
    pub test_package_id: ::prost::alloc::string::String,
    /// The InstrumentationTestRunner class.
    /// The default value is determined by examining the application's manifest.
    #[prost(string, tag = "5")]
    pub test_runner_class: ::prost::alloc::string::String,
    /// Each target must be fully qualified with the package name or class name,
    /// in one of these formats:
    ///  - "package package_name"
    ///  - "class package_name.class_name"
    ///  - "class package_name.class_name#method_name"
    ///
    /// If empty, all targets in the module will be run.
    #[prost(string, repeated, tag = "6")]
    pub test_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The option of whether running each test within its own invocation of
    /// instrumentation with Android Test Orchestrator or not.
    /// ** Orchestrator is only compatible with AndroidJUnitRunner version 1.0 or
    /// higher! **
    /// Orchestrator offers the following benefits:
    ///  - No shared state
    ///  - Crashes are isolated
    ///  - Logs are scoped per test
    ///
    /// See
    /// <<https://developer.android.com/training/testing/junit-runner.html#using-android-test-orchestrator>>
    /// for more information about Android Test Orchestrator.
    ///
    /// If not set, the test will be run without the orchestrator.
    #[prost(enumeration = "OrchestratorOption", tag = "7")]
    pub orchestrator_option: i32,
    /// The option to run tests in multiple shards in parallel.
    #[prost(message, optional, tag = "9")]
    pub sharding_option: ::core::option::Option<ShardingOption>,
    /// Required.
    #[prost(oneof = "android_instrumentation_test::AppUnderTest", tags = "1, 8")]
    pub app_under_test: ::core::option::Option<android_instrumentation_test::AppUnderTest>,
}
/// Nested message and enum types in `AndroidInstrumentationTest`.
pub mod android_instrumentation_test {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AppUnderTest {
        /// The APK for the application under test.
        #[prost(message, tag = "1")]
        AppApk(super::FileReference),
        /// A multi-apk app bundle for the application under test.
        #[prost(message, tag = "8")]
        AppBundle(super::AppBundle),
    }
}
/// A test of an android application that explores the application on a virtual
/// or physical Android Device, finding culprits and crashes as it goes.
/// Next tag: 30
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidRoboTest {
    /// The java package for the application under test.
    /// The default value is determined by examining the application's manifest.
    #[prost(string, tag = "2")]
    pub app_package_id: ::prost::alloc::string::String,
    /// The initial activity that should be used to start the app.
    #[prost(string, tag = "3")]
    pub app_initial_activity: ::prost::alloc::string::String,
    /// The max depth of the traversal stack Robo can explore. Needs to be at least
    /// 2 to make Robo explore the app beyond the first activity.
    /// Default is 50.
    #[deprecated]
    #[prost(int32, tag = "7")]
    pub max_depth: i32,
    /// The max number of steps Robo can execute.
    /// Default is no limit.
    #[deprecated]
    #[prost(int32, tag = "8")]
    pub max_steps: i32,
    /// A set of directives Robo should apply during the crawl.
    /// This allows users to customize the crawl. For example, the username and
    /// password for a test account can be provided.
    #[prost(message, repeated, tag = "11")]
    pub robo_directives: ::prost::alloc::vec::Vec<RoboDirective>,
    /// A JSON file with a sequence of actions Robo should perform as a prologue
    /// for the crawl.
    #[prost(message, optional, tag = "13")]
    pub robo_script: ::core::option::Option<FileReference>,
    /// The intents used to launch the app for the crawl.
    /// If none are provided, then the main launcher activity is launched.
    /// If some are provided, then only those provided are launched (the main
    /// launcher activity must be provided explicitly).
    #[prost(message, repeated, tag = "15")]
    pub starting_intents: ::prost::alloc::vec::Vec<RoboStartingIntent>,
    /// Required.
    #[prost(oneof = "android_robo_test::AppUnderTest", tags = "1, 16")]
    pub app_under_test: ::core::option::Option<android_robo_test::AppUnderTest>,
}
/// Nested message and enum types in `AndroidRoboTest`.
pub mod android_robo_test {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AppUnderTest {
        /// The APK for the application under test.
        #[prost(message, tag = "1")]
        AppApk(super::FileReference),
        /// A multi-apk app bundle for the application under test.
        #[prost(message, tag = "16")]
        AppBundle(super::AppBundle),
    }
}
/// Directs Robo to interact with a specific UI element if it is encountered
/// during the crawl. Currently, Robo can perform text entry or element click.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoboDirective {
    /// Required. The android resource name of the target UI element.
    /// For example,
    ///    in Java: R.string.foo
    ///    in xml: @string/foo
    /// Only the "foo" part is needed.
    /// Reference doc:
    /// <https://developer.android.com/guide/topics/resources/accessing-resources.html>
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The text that Robo is directed to set. If left empty, the directive will be
    /// treated as a CLICK on the element matching the resource_name.
    #[prost(string, tag = "2")]
    pub input_text: ::prost::alloc::string::String,
    /// Required. The type of action that Robo should perform on the specified
    /// element.
    #[prost(enumeration = "RoboActionType", tag = "3")]
    pub action_type: i32,
}
/// Message for specifying the start activities to crawl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoboStartingIntent {
    /// Timeout in seconds for each intent.
    #[prost(message, optional, tag = "3")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Required. Intent details to start an activity.
    #[prost(oneof = "robo_starting_intent::StartingIntent", tags = "1, 2")]
    pub starting_intent: ::core::option::Option<robo_starting_intent::StartingIntent>,
}
/// Nested message and enum types in `RoboStartingIntent`.
pub mod robo_starting_intent {
    /// Required. Intent details to start an activity.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StartingIntent {
        /// An intent that starts the main launcher activity.
        #[prost(message, tag = "1")]
        LauncherActivity(super::LauncherActivityIntent),
        /// An intent that starts an activity with specific details.
        #[prost(message, tag = "2")]
        StartActivity(super::StartActivityIntent),
    }
}
/// Specifies an intent that starts the main launcher activity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LauncherActivityIntent {}
/// A starting intent specified by an action, uri, and categories.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartActivityIntent {
    /// Action name.
    /// Required for START_ACTIVITY.
    #[prost(string, tag = "2")]
    pub action: ::prost::alloc::string::String,
    /// URI for the action.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// Intent categories to set on the intent.
    #[prost(string, repeated, tag = "4")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The matrix of environments in which the test is to be executed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentMatrix {
    /// Required. The environment matrix.
    #[prost(oneof = "environment_matrix::EnvironmentMatrix", tags = "1, 2, 3")]
    pub environment_matrix: ::core::option::Option<environment_matrix::EnvironmentMatrix>,
}
/// Nested message and enum types in `EnvironmentMatrix`.
pub mod environment_matrix {
    /// Required. The environment matrix.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EnvironmentMatrix {
        /// A matrix of Android devices.
        #[prost(message, tag = "1")]
        AndroidMatrix(super::AndroidMatrix),
        /// A list of Android devices; the test will be run only on the specified
        /// devices.
        #[prost(message, tag = "2")]
        AndroidDeviceList(super::AndroidDeviceList),
        /// A list of iOS devices.
        #[prost(message, tag = "3")]
        IosDeviceList(super::IosDeviceList),
    }
}
/// A list of Android device configurations in which the test is to be executed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidDeviceList {
    /// Required. A list of Android devices.
    #[prost(message, repeated, tag = "1")]
    pub android_devices: ::prost::alloc::vec::Vec<AndroidDevice>,
}
/// A list of iOS device configurations in which the test is to be executed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosDeviceList {
    /// Required. A list of iOS devices.
    #[prost(message, repeated, tag = "1")]
    pub ios_devices: ::prost::alloc::vec::Vec<IosDevice>,
}
/// A set of Android device configuration permutations is defined by the
/// the cross-product of the given axes. Internally, the given AndroidMatrix
/// will be expanded into a set of AndroidDevices.
///
/// Only supported permutations will be instantiated.  Invalid permutations
/// (e.g., incompatible models/versions) are ignored.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidMatrix {
    /// Required. The ids of the set of Android device to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, repeated, tag = "1")]
    pub android_model_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The ids of the set of Android OS version to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, repeated, tag = "2")]
    pub android_version_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The set of locales the test device will enable for testing.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, repeated, tag = "3")]
    pub locales: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The set of orientations to test with.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, repeated, tag = "4")]
    pub orientations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Information about the client which invoked the test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfo {
    /// Required. Client name, such as gcloud.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of detailed information about client.
    #[prost(message, repeated, tag = "2")]
    pub client_info_details: ::prost::alloc::vec::Vec<ClientInfoDetail>,
}
/// Key-value pair of detailed information about the client which invoked the
/// test. Examples: {'Version', '1.0'}, {'Release Track', 'BETA'}.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientInfoDetail {
    /// Required. The key of detailed client information.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Required. The value of detailed client information.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Locations where the results of running the test are stored.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultStorage {
    /// Required.
    #[prost(message, optional, tag = "1")]
    pub google_cloud_storage: ::core::option::Option<GoogleCloudStorage>,
    /// The tool results history that contains the tool results execution that
    /// results are written to.
    ///
    /// If not provided, the service will choose an appropriate value.
    #[prost(message, optional, tag = "5")]
    pub tool_results_history: ::core::option::Option<ToolResultsHistory>,
    /// Output only. The tool results execution that results are written to.
    #[prost(message, optional, tag = "6")]
    pub tool_results_execution: ::core::option::Option<ToolResultsExecution>,
    /// Output only. URL to the results in the Firebase Web Console.
    #[prost(string, tag = "7")]
    pub results_url: ::prost::alloc::string::String,
}
/// Represents a tool results history resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolResultsHistory {
    /// Required. The cloud project that owns the tool results history.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. A tool results history ID.
    #[prost(string, tag = "2")]
    pub history_id: ::prost::alloc::string::String,
}
/// Represents a tool results execution resource.
///
/// This has the results of a TestMatrix.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolResultsExecution {
    /// Output only. The cloud project that owns the tool results execution.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. A tool results history ID.
    #[prost(string, tag = "2")]
    pub history_id: ::prost::alloc::string::String,
    /// Output only. A tool results execution ID.
    #[prost(string, tag = "3")]
    pub execution_id: ::prost::alloc::string::String,
}
/// Represents a tool results step resource.
///
/// This has the results of a TestExecution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolResultsStep {
    /// Output only. The cloud project that owns the tool results step.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. A tool results history ID.
    #[prost(string, tag = "2")]
    pub history_id: ::prost::alloc::string::String,
    /// Output only. A tool results execution ID.
    #[prost(string, tag = "3")]
    pub execution_id: ::prost::alloc::string::String,
    /// Output only. A tool results step ID.
    #[prost(string, tag = "4")]
    pub step_id: ::prost::alloc::string::String,
}
/// A storage location within Google cloud storage (GCS).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleCloudStorage {
    /// Required. The path to a directory in GCS that will
    /// eventually contain the results for this test.
    /// The requesting user must have write access on the bucket in the supplied
    /// path.
    #[prost(string, tag = "1")]
    pub gcs_path: ::prost::alloc::string::String,
}
/// A reference to a file, used for user inputs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileReference {
    /// Required. The file reference.
    #[prost(oneof = "file_reference::File", tags = "1")]
    pub file: ::core::option::Option<file_reference::File>,
}
/// Nested message and enum types in `FileReference`.
pub mod file_reference {
    /// Required. The file reference.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum File {
        /// A path to a file in Google Cloud Storage.
        /// Example: gs://build-app-1414623860166/app%40debug-unaligned.apk
        /// These paths are expected to be url encoded (percent encoding)
        #[prost(string, tag = "1")]
        GcsPath(::prost::alloc::string::String),
    }
}
/// The environment in which the test is run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// Required. The environment.
    #[prost(oneof = "environment::Environment", tags = "1, 2")]
    pub environment: ::core::option::Option<environment::Environment>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    /// Required. The environment.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Environment {
        /// An Android device which must be used with an Android test.
        #[prost(message, tag = "1")]
        AndroidDevice(super::AndroidDevice),
        /// An iOS device which must be used with an iOS test.
        #[prost(message, tag = "2")]
        IosDevice(super::IosDevice),
    }
}
/// A single Android device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidDevice {
    /// Required. The id of the Android device to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "1")]
    pub android_model_id: ::prost::alloc::string::String,
    /// Required. The id of the Android OS version to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "2")]
    pub android_version_id: ::prost::alloc::string::String,
    /// Required. The locale the test device used for testing.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "3")]
    pub locale: ::prost::alloc::string::String,
    /// Required. How the device is oriented during the test.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "4")]
    pub orientation: ::prost::alloc::string::String,
}
/// A single iOS device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosDevice {
    /// Required. The id of the iOS device to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "1")]
    pub ios_model_id: ::prost::alloc::string::String,
    /// Required. The id of the iOS major software version to be used.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "2")]
    pub ios_version_id: ::prost::alloc::string::String,
    /// Required. The locale the test device used for testing.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "3")]
    pub locale: ::prost::alloc::string::String,
    /// Required. How the device is oriented during the test.
    /// Use the TestEnvironmentDiscoveryService to get supported options.
    #[prost(string, tag = "4")]
    pub orientation: ::prost::alloc::string::String,
}
/// Additional details about the progress of the running test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestDetails {
    /// Output only. Human-readable, detailed descriptions of the test's progress.
    /// For example: "Provisioning a device", "Starting Test".
    ///
    /// During the course of execution new data may be appended
    /// to the end of progress_messages.
    #[prost(string, repeated, tag = "3")]
    pub progress_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. If the TestState is ERROR, then this string will contain
    /// human-readable details about the error.
    #[prost(string, tag = "4")]
    pub error_message: ::prost::alloc::string::String,
}
/// Details behind an invalid request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvalidRequestDetail {
    /// The reason behind the error.
    #[prost(enumeration = "invalid_request_detail::Reason", tag = "1")]
    pub reason: i32,
}
/// Nested message and enum types in `InvalidRequestDetail`.
pub mod invalid_request_detail {
    /// Possible invalid request reasons.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reason {
        /// No reason has been specified - the default.
        Unspecified = 0,
        /// The request is not valid.
        RequestInvalid = 1,
        /// One or more of the resources specified in the request is too large.
        ResourceTooBig = 2,
        /// One or more resources specified in the request cannot be found.
        ResourceNotFound = 3,
        /// This request is not (currently) supported.
        Unsupported = 4,
        /// This request is not currently implemented.
        NotImplemented = 5,
    }
}
/// Options for enabling sharding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardingOption {
    #[prost(oneof = "sharding_option::Option", tags = "1, 2")]
    pub option: ::core::option::Option<sharding_option::Option>,
}
/// Nested message and enum types in `ShardingOption`.
pub mod sharding_option {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Option {
        /// Uniformly shards test cases given a total number of shards.
        #[prost(message, tag = "1")]
        UniformSharding(super::UniformSharding),
        /// Shards test cases into the specified groups of packages, classes, and/or
        /// methods.
        #[prost(message, tag = "2")]
        ManualSharding(super::ManualSharding),
    }
}
/// Uniformly shards test cases given a total number of shards.
///
/// For Instrumentation test, it will be translated to "-e numShard" "-e
/// shardIndex" AndroidJUnitRunner arguments. With uniform sharding enabled,
/// specifying these sharding arguments via environment_variables is invalid.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniformSharding {
    /// Required. Total number of shards. When any physical devices are selected,
    /// the number must be >= 1 and <= 50. When no physical devices are selected,
    /// the number must be >= 1 and <= 500.
    #[prost(int32, tag = "1")]
    pub num_shards: i32,
}
/// Shards test cases into the specified groups of packages, classes, and/or
/// methods.
///
/// With manual sharding enabled, specifying test targets via
/// environment_variables or in InstrumentationTest is invalid.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualSharding {
    /// Required. Group of packages, classes, and/or test methods to be run for
    /// each shard. When any physical devices are selected,  the number of
    /// test_targets_for_shard must be >= 1 and <= 50. When no physical devices are
    /// selected, the number must be >= 1 and <= 500.
    #[prost(message, repeated, tag = "1")]
    pub test_targets_for_shard: ::prost::alloc::vec::Vec<TestTargetsForShard>,
}
/// Test targets for a shard.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestTargetsForShard {
    /// Group of packages, classes, and/or test methods to be run for each shard.
    /// The targets need to be specified in AndroidJUnitRunner argument format. For
    /// example, "package com.my.packages" "class com.my.package.MyClass".
    ///
    /// The number of shard_test_targets must be greater than 0.
    #[prost(string, repeated, tag = "1")]
    pub test_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Output only. Details about the shard.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shard {
    /// Output only. The index of the shard among all the shards.
    #[prost(int32, tag = "1")]
    pub shard_index: i32,
    /// Output only. The total number of shards.
    #[prost(int32, tag = "2")]
    pub num_shards: i32,
    /// Output only. Test targets for each shard.
    #[prost(message, optional, tag = "3")]
    pub test_targets_for_shard: ::core::option::Option<TestTargetsForShard>,
}
/// Request to submit a matrix of tests for execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTestMatrixRequest {
    /// The GCE project under which this job will run.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The matrix of tests that the user wants to run.
    #[prost(message, optional, tag = "2")]
    pub test_matrix: ::core::option::Option<TestMatrix>,
    /// A string id used to detect duplicated requests.
    /// Ids are automatically scoped to a project, so
    /// users should ensure the ID is unique per-project.
    /// A UUID is recommended.
    ///
    /// Optional, but strongly recommended.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request to get the Test Matrix with the given id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTestMatrixRequest {
    /// Cloud project that owns the test matrix.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Unique test matrix id which was assigned by the service.
    #[prost(string, tag = "2")]
    pub test_matrix_id: ::prost::alloc::string::String,
}
/// Request to stop running all of the tests in the specified matrix.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTestMatrixRequest {
    /// Cloud project that owns the test.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Test matrix that will be canceled.
    #[prost(string, tag = "2")]
    pub test_matrix_id: ::prost::alloc::string::String,
}
/// Response containing the current state of the specified test matrix.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTestMatrixResponse {
    /// The current rolled-up state of the test matrix.
    /// If this state is already final, then the cancelation request will
    /// have no effect.
    #[prost(enumeration = "TestState", tag = "1")]
    pub test_state: i32,
}
/// Specifies how to execute the test.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrchestratorOption {
    /// Default value: the server will choose the mode. Currently implies that
    /// the test will run without the orchestrator. In the future,
    /// all instrumentation tests will be run with the orchestrator.
    /// Using the orchestrator is highly encouraged because of all the benefits it
    /// offers.
    Unspecified = 0,
    /// Run test using orchestrator.
    /// ** Only compatible with AndroidJUnitRunner version 1.0 or higher! **
    /// Recommended.
    UseOrchestrator = 1,
    /// Run test without using orchestrator.
    DoNotUseOrchestrator = 2,
}
/// Actions which Robo can perform on UI elements.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoboActionType {
    /// DO NOT USE. For proto versioning only.
    ActionTypeUnspecified = 0,
    /// Direct Robo to click on the specified element. No-op if specified element
    /// is not clickable.
    SingleClick = 1,
    /// Direct Robo to enter text on the specified element. No-op if specified
    /// element is not enabled or does not allow text entry.
    EnterText = 2,
    /// Direct Robo to ignore interactions with a specific element.
    Ignore = 3,
}
/// The detailed reason that a Matrix was deemed INVALID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InvalidMatrixDetails {
    /// Do not use. For proto versioning only.
    Unspecified = 0,
    /// The matrix is INVALID, but there are no further details available.
    DetailsUnavailable = 1,
    /// The input app APK could not be parsed.
    MalformedApk = 2,
    /// The input test APK could not be parsed.
    MalformedTestApk = 3,
    /// The AndroidManifest.xml could not be found.
    NoManifest = 4,
    /// The APK manifest does not declare a package name.
    NoPackageName = 5,
    /// The APK application ID (aka package name) is invalid.
    /// See also
    /// <https://developer.android.com/studio/build/application-id>
    InvalidPackageName = 31,
    /// The test package and app package are the same.
    TestSameAsApp = 6,
    /// The test apk does not declare an instrumentation.
    NoInstrumentation = 7,
    /// The input app apk does not have a signature.
    NoSignature = 20,
    /// The test runner class specified by user or in the test APK's manifest file
    /// is not compatible with Android Test Orchestrator.
    /// Orchestrator is only compatible with AndroidJUnitRunner version 1.0 or
    /// higher.
    /// Orchestrator can be disabled by using DO_NOT_USE_ORCHESTRATOR
    /// OrchestratorOption.
    InstrumentationOrchestratorIncompatible = 18,
    /// The test APK does not contain the test runner class specified by user or in
    /// the manifest file.
    /// This can be caused by either of the following reasons:
    /// - the user provided a runner class name that's incorrect, or
    /// - the test runner isn't built into the test APK (might be in the app APK
    /// instead).
    NoTestRunnerClass = 19,
    /// A main launcher activity could not be found.
    NoLauncherActivity = 8,
    /// The app declares one or more permissions that are not allowed.
    ForbiddenPermissions = 9,
    /// There is a conflict in the provided robo_directives.
    InvalidRoboDirectives = 10,
    /// There is at least one invalid resource name in the provided
    /// robo directives
    InvalidResourceName = 33,
    /// Invalid definition of action in the robo directives
    /// (e.g. a click or ignore action includes an input text field)
    InvalidDirectiveAction = 34,
    /// There is no test loop intent filter, or the one that is given is
    /// not formatted correctly.
    TestLoopIntentFilterNotFound = 12,
    /// The request contains a scenario label that was not declared in the
    /// manifest.
    ScenarioLabelNotDeclared = 13,
    /// There was an error when parsing a label's value.
    ScenarioLabelMalformed = 14,
    /// The request contains a scenario number that was not declared in the
    /// manifest.
    ScenarioNotDeclared = 15,
    /// Device administrator applications are not allowed.
    DeviceAdminReceiver = 17,
    /// The zipped XCTest was malformed. The zip did not contain a single
    /// .xctestrun file and the contents of the DerivedData/Build/Products
    /// directory.
    MalformedXcTestZip = 11,
    /// The zipped XCTest was built for the iOS simulator rather than for a
    /// physical device.
    BuiltForIosSimulator = 24,
    /// The .xctestrun file did not specify any test targets.
    NoTestsInXcTestZip = 25,
    /// One or more of the test targets defined in the .xctestrun file specifies
    /// "UseDestinationArtifacts", which is disallowed.
    UseDestinationArtifacts = 26,
    /// XC tests which run on physical devices must have
    /// "IsAppHostedTestBundle" == "true" in the xctestrun file.
    TestNotAppHosted = 28,
    /// An Info.plist file in the XCTest zip could not be parsed.
    PlistCannotBeParsed = 30,
    /// The APK is marked as "testOnly".
    /// Deprecated and not currently used.
    TestOnlyApk = 21,
    /// The input IPA could not be parsed.
    MalformedIpa = 22,
    /// The application doesn't register the game loop URL scheme.
    MissingUrlScheme = 35,
    /// The iOS application bundle (.app) couldn't be processed.
    MalformedAppBundle = 36,
    /// APK contains no code.
    /// See also
    /// <https://developer.android.com/guide/topics/manifest/application-element.html#code>
    NoCodeApk = 23,
    /// Either the provided input APK path was malformed,
    /// the APK file does not exist, or the user does not have permission to
    /// access the APK file.
    InvalidInputApk = 27,
    /// APK is built for a preview SDK which is unsupported
    InvalidApkPreviewSdk = 29,
}
/// The state (i.e., progress) of a test execution or matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TestState {
    /// Do not use.  For proto versioning only.
    Unspecified = 0,
    /// The execution or matrix is being validated.
    Validating = 8,
    /// The execution or matrix is waiting for resources to become available.
    Pending = 1,
    /// The execution is currently being processed.
    ///
    /// Can only be set on an execution.
    Running = 2,
    /// The execution or matrix has terminated normally.
    ///
    /// On a matrix this means that the matrix level processing completed normally,
    /// but individual executions may be in an ERROR state.
    Finished = 3,
    /// The execution or matrix has stopped because it encountered an
    /// infrastructure failure.
    Error = 4,
    /// The execution was not run because it corresponds to a unsupported
    /// environment.
    ///
    /// Can only be set on an execution.
    UnsupportedEnvironment = 5,
    /// The execution was not run because the provided inputs are incompatible with
    /// the requested environment.
    ///
    /// Example: requested AndroidVersion is lower than APK's minSdkVersion
    ///
    /// Can only be set on an execution.
    IncompatibleEnvironment = 9,
    /// The execution was not run because the provided inputs are incompatible with
    /// the requested architecture.
    ///
    /// Example: requested device does not support running the native code in
    /// the supplied APK
    ///
    /// Can only be set on an execution.
    IncompatibleArchitecture = 10,
    /// The user cancelled the execution.
    ///
    /// Can only be set on an execution.
    Cancelled = 6,
    /// The execution or matrix was not run because the provided inputs are not
    /// valid.
    ///
    /// Examples: input file is not of the expected type, is malformed/corrupt, or
    /// was flagged as malware
    Invalid = 7,
}
/// Outcome summary for a finished test matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutcomeSummary {
    /// Do not use. For proto versioning only.
    Unspecified = 0,
    /// The test matrix run was successful, for instance:
    /// - All the test cases passed.
    /// - Robo did not detect a crash of the application under test.
    Success = 1,
    /// A run failed, for instance:
    /// - One or more test case failed.
    /// - A test timed out.
    /// - The application under test crashed.
    Failure = 2,
    /// Something unexpected happened. The run should still be considered
    /// unsuccessful but this is likely a transient problem and re-running the
    /// test might be successful.
    Inconclusive = 3,
    /// All tests were skipped, for instance:
    /// - All device configurations were incompatible.
    Skipped = 4,
}
#[doc = r" Generated client implementations."]
pub mod test_execution_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service for requesting test executions and querying their status."]
    #[doc = ""]
    #[doc = " This service is part of Firebase Test Lab. To learn about how to use the"]
    #[doc = " product, and how to integrate it with your system,"]
    #[doc = " visit https://firebase.google.com/docs/test-lab."]
    #[doc = ""]
    #[doc = " Each test execution will wait for available capacity. It will then be"]
    #[doc = " invoked as described. The test may be invoked multiple times if an"]
    #[doc = " infrastructure failure is detected. Results and other files generated by"]
    #[doc = " the test will be stored in an external storage system."]
    #[doc = ""]
    #[doc = " The TestExecutionService models this behavior using two resource types:"]
    #[doc = ""]
    #[doc = " - TestMatrix: a group of one or more TestExecutions, built by taking a"]
    #[doc = "   product of values over a pre-defined set of axes. In the case of Android"]
    #[doc = "   Tests, for example, device model and OS version are two axes of the matrix."]
    #[doc = ""]
    #[doc = " - TestExecution: a single execution of one or more test targets on a"]
    #[doc = "   single device. These are created automatically when a TestMatrix is"]
    #[doc = "   created."]
    #[doc = ""]
    #[doc = " This service returns any error codes from the canonical error space (i.e."]
    #[doc = " google.rpc.Code). The errors which may be returned are specified on each"]
    #[doc = " method. In addition, any method may return UNAVAILABLE or INTERNAL."]
    #[derive(Debug, Clone)]
    pub struct TestExecutionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TestExecutionServiceClient<T>
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
        ) -> TestExecutionServiceClient<InterceptedService<T, F>>
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
            TestExecutionServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates and runs a matrix of tests according to the given specifications."]
        #[doc = " Unsupported environments will be returned in the state UNSUPPORTED."]
        #[doc = " A test matrix is limited to use at most 2000 devices in parallel."]
        #[doc = ""]
        #[doc = " May return any of the following canonical error codes:"]
        #[doc = ""]
        #[doc = " - PERMISSION_DENIED - if the user is not authorized to write to project"]
        #[doc = " - INVALID_ARGUMENT - if the request is malformed or if the matrix tries"]
        #[doc = "                      to use too many simultaneous devices."]
        pub async fn create_test_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTestMatrixRequest>,
        ) -> Result<tonic::Response<super::TestMatrix>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.testing.v1.TestExecutionService/CreateTestMatrix",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Checks the status of a test matrix."]
        #[doc = ""]
        #[doc = " May return any of the following canonical error codes:"]
        #[doc = ""]
        #[doc = " - PERMISSION_DENIED - if the user is not authorized to read project"]
        #[doc = " - INVALID_ARGUMENT - if the request is malformed"]
        #[doc = " - NOT_FOUND - if the Test Matrix does not exist"]
        pub async fn get_test_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTestMatrixRequest>,
        ) -> Result<tonic::Response<super::TestMatrix>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.testing.v1.TestExecutionService/GetTestMatrix",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels unfinished test executions in a test matrix."]
        #[doc = " This call returns immediately and cancellation proceeds asynchronously."]
        #[doc = " If the matrix is already final, this operation will have no effect."]
        #[doc = ""]
        #[doc = " May return any of the following canonical error codes:"]
        #[doc = ""]
        #[doc = " - PERMISSION_DENIED - if the user is not authorized to read project"]
        #[doc = " - INVALID_ARGUMENT - if the request is malformed"]
        #[doc = " - NOT_FOUND - if the Test Matrix does not exist"]
        pub async fn cancel_test_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelTestMatrixRequest>,
        ) -> Result<tonic::Response<super::CancelTestMatrixResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.testing.v1.TestExecutionService/CancelTestMatrix",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Android application details based on application manifest and apk archive
/// contents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApkDetail {
    #[prost(message, optional, tag = "1")]
    pub apk_manifest: ::core::option::Option<ApkManifest>,
}
/// An Android app manifest. See
/// <http://developer.android.com/guide/topics/manifest/manifest-intro.html>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApkManifest {
    /// Full Java-style package name for this application, e.g.
    /// "com.example.foo".
    #[prost(string, tag = "1")]
    pub package_name: ::prost::alloc::string::String,
    /// Minimum API level required for the application to run.
    #[prost(int32, tag = "2")]
    pub min_sdk_version: i32,
    /// Maximum API level on which the application is designed to run.
    #[prost(int32, tag = "3")]
    pub max_sdk_version: i32,
    /// Specifies the API Level on which the application is designed to run.
    #[prost(int32, tag = "6")]
    pub target_sdk_version: i32,
    /// User-readable name for the application.
    #[prost(string, tag = "4")]
    pub application_label: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub intent_filters: ::prost::alloc::vec::Vec<IntentFilter>,
    /// Permissions declared to be used by the application
    #[prost(string, repeated, tag = "7")]
    pub uses_permission: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The <intent-filter> section of an <activity> tag.
/// <https://developer.android.com/guide/topics/manifest/intent-filter-element.html>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentFilter {
    /// The android:name value of the <action> tag.
    #[prost(string, repeated, tag = "1")]
    pub action_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The android:name value of the <category> tag.
    #[prost(string, repeated, tag = "2")]
    pub category_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The android:mimeType value of the <data> tag.
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
}
/// A request to get the details of an Android application APK.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApkDetailsRequest {
    /// The APK to be parsed for details.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<FileReference>,
}
/// Response containing the details of the specified Android application APK.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApkDetailsResponse {
    /// Details of the Android APK.
    #[prost(message, optional, tag = "1")]
    pub apk_detail: ::core::option::Option<ApkDetail>,
}
#[doc = r" Generated client implementations."]
pub mod application_detail_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service which parses input applications and returns details that can be"]
    #[doc = " useful in the context of testing."]
    #[derive(Debug, Clone)]
    pub struct ApplicationDetailServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ApplicationDetailServiceClient<T>
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
        ) -> ApplicationDetailServiceClient<InterceptedService<T, F>>
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
            ApplicationDetailServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Gets the details of an Android application APK."]
        pub async fn get_apk_details(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApkDetailsRequest>,
        ) -> Result<tonic::Response<super::GetApkDetailsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.testing.v1.ApplicationDetailService/GetApkDetails",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A single device IP block
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceIpBlock {
    /// An IP address block in CIDR notation eg: 34.68.194.64/29
    #[prost(string, tag = "1")]
    pub block: ::prost::alloc::string::String,
    /// Whether this block is used by physical or virtual devices
    #[prost(enumeration = "DeviceForm", tag = "2")]
    pub form: i32,
    /// The date this block was added to Firebase Test Lab
    #[prost(message, optional, tag = "3")]
    pub added_date: ::core::option::Option<super::super::super::r#type::Date>,
}
/// Request to list the currently supported values for an environment type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTestEnvironmentCatalogRequest {
    /// Required. The type of environment that should be listed.
    #[prost(
        enumeration = "get_test_environment_catalog_request::EnvironmentType",
        tag = "1"
    )]
    pub environment_type: i32,
    /// For authorization, the cloud project requesting the TestEnvironmentCatalog.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `GetTestEnvironmentCatalogRequest`.
pub mod get_test_environment_catalog_request {
    /// Types of environments the Test API supports.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EnvironmentType {
        /// Do not use.  For proto versioning only.
        Unspecified = 0,
        /// A device running a version of the Android OS.
        Android = 1,
        /// A device running a version of iOS.
        Ios = 3,
        /// A network configuration to use when running a test.
        NetworkConfiguration = 4,
        /// The software environment provided by TestExecutionService.
        ProvidedSoftware = 5,
        /// The IP blocks used by devices in the test environment.
        DeviceIpBlocks = 6,
    }
}
/// A description of a test environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestEnvironmentCatalog {
    /// Output only.
    #[prost(
        oneof = "test_environment_catalog::EnvironmentCatalog",
        tags = "1, 3, 4, 5, 6"
    )]
    pub environment_catalog: ::core::option::Option<test_environment_catalog::EnvironmentCatalog>,
}
/// Nested message and enum types in `TestEnvironmentCatalog`.
pub mod test_environment_catalog {
    /// Output only.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EnvironmentCatalog {
        /// Supported Android devices.
        #[prost(message, tag = "1")]
        AndroidDeviceCatalog(super::AndroidDeviceCatalog),
        /// Supported iOS devices.
        #[prost(message, tag = "3")]
        IosDeviceCatalog(super::IosDeviceCatalog),
        /// Supported network configurations.
        #[prost(message, tag = "4")]
        NetworkConfigurationCatalog(super::NetworkConfigurationCatalog),
        /// The software test environment provided by TestExecutionService.
        #[prost(message, tag = "5")]
        SoftwareCatalog(super::ProvidedSoftwareCatalog),
        /// The IP blocks used by devices in the test environment.
        #[prost(message, tag = "6")]
        DeviceIpBlockCatalog(super::DeviceIpBlockCatalog),
    }
}
/// List of IP blocks used by the Firebase Test Lab
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceIpBlockCatalog {
    /// The device IP blocks used by Firebase Test Lab
    #[prost(message, repeated, tag = "1")]
    pub ip_blocks: ::prost::alloc::vec::Vec<DeviceIpBlock>,
}
/// The currently supported Android devices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidDeviceCatalog {
    /// The set of supported Android device models.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<AndroidModel>,
    /// The set of supported Android OS versions.
    #[prost(message, repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<AndroidVersion>,
    /// The set of supported runtime configurations.
    #[prost(message, optional, tag = "3")]
    pub runtime_configuration: ::core::option::Option<AndroidRuntimeConfiguration>,
}
/// Android configuration that can be selected at the time a test is run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidRuntimeConfiguration {
    /// The set of available locales.
    #[prost(message, repeated, tag = "1")]
    pub locales: ::prost::alloc::vec::Vec<Locale>,
    /// The set of available orientations.
    #[prost(message, repeated, tag = "2")]
    pub orientations: ::prost::alloc::vec::Vec<Orientation>,
}
/// A description of an Android device tests may be run on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidModel {
    /// The unique opaque id for this model.
    /// Use this for invoking the TestExecutionService.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The human-readable marketing name for this device model.
    /// Examples: "Nexus 5", "Galaxy S5".
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The manufacturer of this device.
    #[prost(string, tag = "3")]
    pub manufacturer: ::prost::alloc::string::String,
    /// The company that this device is branded with.
    /// Example: "Google", "Samsung".
    #[prost(string, tag = "9")]
    pub brand: ::prost::alloc::string::String,
    /// The name of the industrial design.
    /// This corresponds to android.os.Build.DEVICE.
    #[prost(string, tag = "10")]
    pub codename: ::prost::alloc::string::String,
    /// Whether this device is virtual or physical.
    #[prost(enumeration = "DeviceForm", tag = "4")]
    pub form: i32,
    /// Whether this device is a phone, tablet, wearable, etc.
    #[prost(enumeration = "DeviceFormFactor", tag = "16")]
    pub form_factor: i32,
    /// Screen size in the horizontal (X) dimension measured in pixels.
    #[prost(int32, tag = "5")]
    pub screen_x: i32,
    /// Screen size in the vertical (Y) dimension measured in pixels.
    #[prost(int32, tag = "6")]
    pub screen_y: i32,
    /// Screen density in DPI.
    /// This corresponds to ro.sf.lcd_density
    #[prost(int32, tag = "12")]
    pub screen_density: i32,
    /// True if and only if tests with this model are recorded by stitching
    /// together screenshots. See use_low_spec_video_recording in device config.
    #[prost(bool, tag = "17")]
    pub low_fps_video_recording: bool,
    /// The set of Android versions this device supports.
    #[prost(string, repeated, tag = "7")]
    pub supported_version_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of supported ABIs for this device.
    /// This corresponds to either android.os.Build.SUPPORTED_ABIS (for API level
    /// 21 and above) or android.os.Build.CPU_ABI/CPU_ABI2.
    /// The most preferred ABI is the first element in the list.
    ///
    /// Elements are optionally prefixed by "version_id:" (where version_id is
    /// the id of an AndroidVersion), denoting an ABI that is supported only on
    /// a particular version.
    #[prost(string, repeated, tag = "11")]
    pub supported_abis: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    #[prost(string, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL of a thumbnail image (photo) of the device.
    /// e.g. <https://lh3.googleusercontent.com/90WcauuJiCYABEl8U0lcZeuS5STUbf2yW...>
    #[prost(string, tag = "19")]
    pub thumbnail_url: ::prost::alloc::string::String,
}
/// A version of the Android OS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AndroidVersion {
    /// An opaque id for this Android version.
    /// Use this id to invoke the TestExecutionService.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// A string representing this version of the Android OS.
    /// Examples: "4.3", "4.4".
    #[prost(string, tag = "2")]
    pub version_string: ::prost::alloc::string::String,
    /// The API level for this Android version.
    /// Examples: 18, 19.
    #[prost(int32, tag = "3")]
    pub api_level: i32,
    /// The code name for this Android version.
    /// Examples: "JellyBean", "KitKat".
    #[prost(string, tag = "4")]
    pub code_name: ::prost::alloc::string::String,
    /// The date this Android version became available in the market.
    #[prost(message, optional, tag = "5")]
    pub release_date: ::core::option::Option<super::super::super::r#type::Date>,
    /// Market share for this version.
    #[prost(message, optional, tag = "6")]
    pub distribution: ::core::option::Option<Distribution>,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    #[prost(string, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Data about the relative number of devices running a
/// given configuration of the Android platform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Distribution {
    /// Output only. The time this distribution was measured.
    #[prost(message, optional, tag = "1")]
    pub measurement_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The estimated fraction (0-1) of the total market with this
    /// configuration.
    #[prost(double, tag = "2")]
    pub market_share: f64,
}
/// The currently supported iOS devices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosDeviceCatalog {
    /// The set of supported iOS device models.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<IosModel>,
    /// The set of supported iOS software versions.
    #[prost(message, repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<IosVersion>,
    /// The set of supported Xcode versions.
    #[prost(message, repeated, tag = "4")]
    pub xcode_versions: ::prost::alloc::vec::Vec<XcodeVersion>,
    /// The set of supported runtime configurations.
    #[prost(message, optional, tag = "3")]
    pub runtime_configuration: ::core::option::Option<IosRuntimeConfiguration>,
}
/// iOS configuration that can be selected at the time a test is run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosRuntimeConfiguration {
    /// The set of available locales.
    #[prost(message, repeated, tag = "1")]
    pub locales: ::prost::alloc::vec::Vec<Locale>,
    /// The set of available orientations.
    #[prost(message, repeated, tag = "2")]
    pub orientations: ::prost::alloc::vec::Vec<Orientation>,
}
/// A description of an iOS device tests may be run on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosModel {
    /// The unique opaque id for this model.
    /// Use this for invoking the TestExecutionService.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The human-readable name for this device model.
    /// Examples: "iPhone 4s", "iPad Mini 2".
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// The set of iOS major software versions this device supports.
    #[prost(string, repeated, tag = "3")]
    pub supported_version_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    #[prost(string, repeated, tag = "4")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Device capabilities.
    /// Copied from
    /// <https://developer.apple.com/library/archive/documentation/DeviceInformation/Reference/iOSDeviceCompatibility/DeviceCompatibilityMatrix/DeviceCompatibilityMatrix.html>
    #[prost(string, repeated, tag = "5")]
    pub device_capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Screen size in the horizontal (X) dimension measured in pixels.
    #[prost(int32, tag = "7")]
    pub screen_x: i32,
    /// Screen size in the vertical (Y) dimension measured in pixels.
    #[prost(int32, tag = "8")]
    pub screen_y: i32,
    /// Screen density in DPI.
    #[prost(int32, tag = "9")]
    pub screen_density: i32,
    /// Whether this device is a phone, tablet, wearable, etc.
    #[prost(enumeration = "DeviceFormFactor", tag = "6")]
    pub form_factor: i32,
}
/// An iOS version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IosVersion {
    /// An opaque id for this iOS version.
    /// Use this id to invoke the TestExecutionService.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// An integer representing the major iOS version.
    /// Examples: "8", "9".
    #[prost(int32, tag = "2")]
    pub major_version: i32,
    /// An integer representing the minor iOS version.
    /// Examples: "1", "2".
    #[prost(int32, tag = "4")]
    pub minor_version: i32,
    /// Tags for this dimension.
    /// Examples: "default", "preview", "deprecated".
    #[prost(string, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The available Xcode versions for this version.
    #[prost(string, repeated, tag = "5")]
    pub supported_xcode_version_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A location/region designation for language.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Locale {
    /// The id for this locale.
    /// Example: "en_US".
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// A human-friendly name for this language/locale.
    /// Example: "English".
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// A human-friendly string representing the region for this
    /// locale. Example: "United States". Not present for every locale.
    #[prost(string, tag = "3")]
    pub region: ::prost::alloc::string::String,
    /// Tags for this dimension.
    /// Example: "default".
    #[prost(string, repeated, tag = "4")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Screen orientation of the device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Orientation {
    /// The id for this orientation.
    /// Example: "portrait".
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// A human-friendly name for this orientation.
    /// Example: "portrait".
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Tags for this dimension.
    /// Example: "default".
    #[prost(string, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// An Xcode version that an iOS version is compatible with.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XcodeVersion {
    /// The id for this version.
    /// Example: "9.2".
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// Tags for this Xcode version.
    /// Example: "default".
    #[prost(string, repeated, tag = "2")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfigurationCatalog {
    #[prost(message, repeated, tag = "1")]
    pub configurations: ::prost::alloc::vec::Vec<NetworkConfiguration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfiguration {
    /// The unique opaque id for this network traffic configuration.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// The emulation rule applying to the upload traffic.
    #[prost(message, optional, tag = "2")]
    pub up_rule: ::core::option::Option<TrafficRule>,
    /// The emulation rule applying to the download traffic.
    #[prost(message, optional, tag = "3")]
    pub down_rule: ::core::option::Option<TrafficRule>,
}
/// Network emulation parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficRule {
    /// Packet delay, must be >= 0.
    #[prost(message, optional, tag = "1")]
    pub delay: ::core::option::Option<::prost_types::Duration>,
    /// Packet loss ratio (0.0 - 1.0).
    #[prost(float, tag = "2")]
    pub packet_loss_ratio: f32,
    /// Packet duplication ratio (0.0 - 1.0).
    #[prost(float, tag = "3")]
    pub packet_duplication_ratio: f32,
    /// Bandwidth in kbits/second.
    #[prost(float, tag = "4")]
    pub bandwidth: f32,
    /// Burst size in kbits.
    #[prost(float, tag = "5")]
    pub burst: f32,
}
/// The currently provided software environment on the devices under test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvidedSoftwareCatalog {
    /// A string representing the current version of Android Test Orchestrator
    /// that is used in the environment. The package is available at
    /// <https://maven.google.com/web/index.html#com.android.support.test:orchestrator.>
    #[prost(string, tag = "1")]
    pub orchestrator_version: ::prost::alloc::string::String,
    /// A string representing the current version of AndroidX Test Orchestrator
    /// that is used in the environment. The package is available at
    /// <https://maven.google.com/web/index.html#androidx.test:orchestrator.>
    #[prost(string, tag = "2")]
    pub androidx_orchestrator_version: ::prost::alloc::string::String,
}
/// Whether the device is physical or virtual.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceForm {
    /// Do not use.  For proto versioning only.
    Unspecified = 0,
    /// Android virtual device using Compute Engine native virtualization. Firebase
    /// Test Lab only.
    Virtual = 1,
    /// Actual hardware.
    Physical = 2,
    /// Android virtual device using emulator in nested virtualization. Equivalent
    /// to Android Studio.
    Emulator = 3,
}
/// The form factor of a device.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceFormFactor {
    /// Do not use. For proto versioning only.
    Unspecified = 0,
    /// This device has the shape of a phone.
    Phone = 1,
    /// This device has the shape of a tablet.
    Tablet = 2,
    /// This device has the shape of a watch or other wearable.
    Wearable = 3,
}
#[doc = r" Generated client implementations."]
pub mod test_environment_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for discovering environments supported by the TestExecutionService."]
    #[doc = ""]
    #[doc = " Over time the TestService may add or remove devices or configuration options"]
    #[doc = " (e.g., when new devices and APIs are released).  Clients should check here"]
    #[doc = " periodically to discover what options are supported."]
    #[doc = ""]
    #[doc = " It defines the following resource model:"]
    #[doc = ""]
    #[doc = " - The API a collection of [TestEnvironmentCatalog]"]
    #[doc = "   [google.devtools.test.v1.TestEnvironmentCatalog] resources, named"]
    #[doc = "   `testEnvironmentCatalog/*`"]
    #[doc = ""]
    #[doc = " - Each TestEnvironmentCatalog resource describes a set of supported"]
    #[doc = "   environments."]
    #[doc = ""]
    #[doc = " - An [AndroidDeviceCatalog][google.devtools.test.v1.AndroidDeviceCatalog]"]
    #[doc = "   describes supported Android devices. It contains lists of supported"]
    #[doc = "   [AndroidModels][google.devtools.test.v1.AndroidModel] and"]
    #[doc = "   [AndroidVersions][google.devtools.test.v1.AndroidVersion] along with a"]
    #[doc = "   [AndroidRuntimeConfiguration][google.devtools.test.v1.AndroidRuntimeConfiguration]."]
    #[doc = "   Each AndroidModel contains a list of Versions it supports. All"]
    #[doc = "   models support all locales and orientations described by the"]
    #[doc = "   AndroidRuntimeConfiguration"]
    #[doc = ""]
    #[doc = " - An [IosDeviceCatalog][google.devtools.test.v1.IosDeviceCatalog]"]
    #[doc = "   describes supported iOS devices. It contains lists of supported"]
    #[doc = "   [IosModels][google.devtools.test.v1.IosModel] and"]
    #[doc = "   [IosVersions][google.devtools.test.v1.IosVersion] along with a"]
    #[doc = "   [IosRuntimeConfiguration][google.devtools.test.v1.IosRuntimeConfiguration]."]
    #[doc = "   Each IosModel contains a list of Versions it supports. All"]
    #[doc = "   models support all locales and orientations described by the"]
    #[doc = "   IosRuntimeConfiguration."]
    #[derive(Debug, Clone)]
    pub struct TestEnvironmentDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TestEnvironmentDiscoveryServiceClient<T>
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
        ) -> TestEnvironmentDiscoveryServiceClient<InterceptedService<T, F>>
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
            TestEnvironmentDiscoveryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Gets the catalog of supported test environments."]
        #[doc = ""]
        #[doc = " May return any of the following canonical error codes:"]
        #[doc = ""]
        #[doc = " - INVALID_ARGUMENT - if the request is malformed"]
        #[doc = " - NOT_FOUND - if the environment type does not exist"]
        #[doc = " - INTERNAL - if an internal error occurred"]
        pub async fn get_test_environment_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTestEnvironmentCatalogRequest>,
        ) -> Result<tonic::Response<super::TestEnvironmentCatalog>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.devtools.testing.v1.TestEnvironmentDiscoveryService/GetTestEnvironmentCatalog") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
