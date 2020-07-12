/// Describes the status of a resource in both enum and string form.
/// Only use description when conveying additional info not captured in the enum
/// name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusAttributes {
    /// Enum representation of the status.
    #[prost(enumeration = "Status", tag = "1")]
    pub status: i32,
    /// A longer description about the status.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
}
/// A generic key-value property definition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// The key.
    #[prost(string, tag = "1")]
    pub key: std::string::String,
    /// The value.
    #[prost(string, tag = "2")]
    pub value: std::string::String,
}
/// The timing of a particular Invocation, Action, etc. The start_time is
/// specified, stop time can be calculated by adding duration to start_time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timing {
    /// The time the resource started running. This is in UTC Epoch time.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The duration for which the resource ran.
    #[prost(message, optional, tag = "2")]
    pub duration: ::std::option::Option<::prost_types::Duration>,
}
/// Represents a dependency of a resource on another resource. This can be used
/// to define a graph or a workflow paradigm through resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dependency {
    /// A label describing this dependency.
    /// The label "Root Cause" is handled specially. It is used to point to the
    /// exact resource that caused a resource to fail.
    #[prost(string, tag = "4")]
    pub label: std::string::String,
    /// The resource depended upon. It may be a Target, ConfiguredTarget, or
    /// Action.
    #[prost(oneof = "dependency::Resource", tags = "1, 2, 3")]
    pub resource: ::std::option::Option<dependency::Resource>,
}
pub mod dependency {
    /// The resource depended upon. It may be a Target, ConfiguredTarget, or
    /// Action.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        /// The name of a target.  Its format must be:
        /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}
        /// This must point to an target under the same invocation.
        #[prost(string, tag = "1")]
        Target(std::string::String),
        /// The name of a configured target.  Its format must be:
        /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIG_ID)}
        /// This must point to an configured target under the same invocation.
        #[prost(string, tag = "2")]
        ConfiguredTarget(std::string::String),
        /// The name of an action.  Its format must be:
        /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIG_ID)}/actions/${url_encode(ACTION_ID)}
        /// This must point to an action under the same invocation.
        #[prost(string, tag = "3")]
        Action(std::string::String),
    }
}
/// These correspond to the prefix of the rule name. Eg cc_test has language CC.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Language {
    /// Language unspecified or not listed here.
    Unspecified = 0,
    /// Not related to any particular language
    None = 1,
    /// Android
    Android = 2,
    /// ActionScript (Flash)
    As = 3,
    /// C++ or C
    Cc = 4,
    /// Cascading-Style-Sheets
    Css = 5,
    /// Dart
    Dart = 6,
    /// Go
    Go = 7,
    /// Google-Web-Toolkit
    Gwt = 8,
    /// Haskell
    Haskell = 9,
    /// Java
    Java = 10,
    /// Javascript
    Js = 11,
    /// Lisp
    Lisp = 12,
    /// Objective-C
    Objc = 13,
    /// Python
    Py = 14,
    /// Shell (Typically Bash)
    Sh = 15,
    /// Swift
    Swift = 16,
    /// Typescript
    Ts = 18,
    /// Webtesting
    Web = 19,
    /// Scala
    Scala = 20,
    /// Protocol Buffer
    Proto = 21,
    /// Extensible Markup Language
    Xml = 22,
}
/// Status of a resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    /// The implicit default enum value. Should never be set.
    Unspecified = 0,
    /// Displays as "Building". Means the target is compiling, linking, etc.
    Building = 1,
    /// Displays as "Built". Means the target was built successfully.
    /// If testing was requested, it should never reach this status: it should go
    /// straight from BUILDING to TESTING.
    Built = 2,
    /// Displays as "Broken". Means build failure such as compile error.
    FailedToBuild = 3,
    /// Displays as "Testing". Means the test is running.
    Testing = 4,
    /// Displays as "Passed". Means the test was run and passed.
    Passed = 5,
    /// Displays as "Failed". Means the test was run and failed.
    Failed = 6,
    /// Displays as "Timed out". Means the test didn't finish in time.
    TimedOut = 7,
    /// Displays as "Cancelled". Means the build or test was cancelled.
    /// E.g. User hit control-C.
    Cancelled = 8,
    /// Displays as "Tool Failed". Means the build or test had internal tool
    /// failure.
    ToolFailed = 9,
    /// Displays as "Incomplete". Means the build or test did not complete.  This
    /// might happen when a build breakage or test failure causes the tool to stop
    /// trying to build anything more or run any more tests, with the default
    /// bazel --nokeep_going option or the --notest_keep_going option.
    Incomplete = 10,
    /// Displays as "Flaky". Means the aggregate status contains some runs that
    /// were successful, and some that were not.
    Flaky = 11,
    /// Displays as "Unknown". Means the tool uploading to the server died
    /// mid-upload or does not know the state.
    Unknown = 12,
    /// Displays as "Skipped". Means building and testing were skipped.
    /// (E.g. Restricted to a different configuration.)
    Skipped = 13,
}
/// Indicates the upload status of the invocation, whether it is
/// post-processing, or immutable, etc.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UploadStatus {
    /// The implicit default enum value. Should never be set.
    Unspecified = 0,
    /// The invocation is still uploading to the ResultStore.
    Uploading = 1,
    /// The invocation upload is complete. The ResultStore is still post-processing
    /// the invocation.
    PostProcessing = 2,
    /// All post-processing is complete, and the invocation is now immutable.
    Immutable = 3,
}
/// Describes line coverage for a file
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineCoverage {
    /// Which source lines in the file represent the start of a statement that was
    /// instrumented to detect whether it was executed by the test.
    ///
    /// This is a bitfield where i-th bit corresponds to the i-th line. Divide line
    /// number by 8 to get index into byte array. Mod line number by 8 to get bit
    /// number (0 = LSB, 7 = MSB).
    ///
    /// A 1 denotes the line was instrumented.
    /// A 0 denotes the line was not instrumented.
    #[prost(bytes, tag = "1")]
    pub instrumented_lines: std::vec::Vec<u8>,
    /// Which of the instrumented source lines were executed by the test. Should
    /// include lines that were not instrumented.
    ///
    /// This is a bitfield where i-th bit corresponds to the i-th line. Divide line
    /// number by 8 to get index into byte array. Mod line number by 8 to get bit
    /// number (0 = LSB, 7 = MSB).
    ///
    /// A 1 denotes the line was executed.
    /// A 0 denotes the line was not executed.
    #[prost(bytes, tag = "2")]
    pub executed_lines: std::vec::Vec<u8>,
}
/// Describes branch coverage for a file
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BranchCoverage {
    /// The field branch_present denotes the lines containing at least one branch.
    ///
    /// This is a bitfield where i-th bit corresponds to the i-th line. Divide line
    /// number by 8 to get index into byte array. Mod line number by 8 to get bit
    /// number (0 = LSB, 7 = MSB).
    ///
    /// A 1 denotes the line contains at least one branch.
    /// A 0 denotes the line contains no branches.
    #[prost(bytes, tag = "1")]
    pub branch_present: std::vec::Vec<u8>,
    /// Contains the number of branches present, only for the lines which have the
    /// corresponding bit set in branch_present, in a relative order ignoring
    /// lines which do not have any branches.
    #[prost(int32, repeated, tag = "2")]
    pub branches_in_line: ::std::vec::Vec<i32>,
    /// As each branch can have any one of the following three states: not
    /// executed, executed but not taken, executed and taken.
    ///
    /// This is a bitfield where i-th bit corresponds to the i-th branch. Divide
    /// branch number by 8 to get index into byte array. Mod branch number by 8 to
    /// get bit number (0 = LSB, 7 = MSB).
    ///
    /// i-th bit of the following two byte arrays are used to denote the above
    /// mentioned states.
    ///
    /// not executed: i-th bit of executed == 0 && i-th bit of taken == 0
    /// executed but not taken: i-th bit of executed == 1 && i-th bit of taken == 0
    /// executed and taken: i-th bit of executed == 1 && i-th bit of taken == 1
    #[prost(bytes, tag = "3")]
    pub executed: std::vec::Vec<u8>,
    /// Described above.
    #[prost(bytes, tag = "4")]
    pub taken: std::vec::Vec<u8>,
}
/// Describes code coverage for a particular file under test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileCoverage {
    /// Path of source file within the SourceContext of this Invocation.
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// Details of lines in a file required to calculate line coverage.
    #[prost(message, optional, tag = "2")]
    pub line_coverage: ::std::option::Option<LineCoverage>,
    /// Details of branches in a file required to calculate branch coverage.
    #[prost(message, optional, tag = "3")]
    pub branch_coverage: ::std::option::Option<BranchCoverage>,
}
/// Describes code coverage for a build or test Action. This is used to store
/// baseline coverage for build Actions and test coverage for test Actions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionCoverage {
    /// List of coverage info for all source files that the TestResult covers.
    #[prost(message, repeated, tag = "2")]
    pub file_coverages: ::std::vec::Vec<FileCoverage>,
}
/// Describes aggregate code coverage for a collection of build or test Actions.
/// A line or branch is covered if and only if it is covered in any of the build
/// or test actions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateCoverage {
    /// Aggregated coverage info for all source files that the actions cover.
    #[prost(message, repeated, tag = "1")]
    pub file_coverages: ::std::vec::Vec<FileCoverage>,
}
/// The metadata for a file or an archive file entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    /// The identifier of the file or archive entry.
    /// User-provided, must be unique for the repeated field it is in. When an
    /// Append RPC is called with a Files field populated, if a File already exists
    /// with this ID, that File will be overwritten with the new File proto.
    #[prost(string, tag = "1")]
    pub uid: std::string::String,
    /// The URI of a file.
    /// This could also be the URI of an entire archive.
    /// Most log data doesn't need to be stored forever, so a ttl is suggested.
    /// Note that if you ever move or delete the file at this URI, the link from
    /// the server will be broken.
    #[prost(string, tag = "2")]
    pub uri: std::string::String,
    /// (Optional) The length of the file in bytes.  Allows the filesize to be
    /// shown in the UI.  Omit if file is still being written or length is
    /// not known.  This could also be the length of an entire archive.
    #[prost(message, optional, tag = "3")]
    pub length: ::std::option::Option<i64>,
    /// (Optional) The content-type (aka MIME-type) of the file.  This is sent to
    /// the web browser so it knows how to handle the file. (e.g. text/plain,
    /// image/jpeg, text/html, etc). For zip archives, use "application/zip".
    #[prost(string, tag = "4")]
    pub content_type: std::string::String,
    /// (Optional) If the above path, length, and content_type are referring to an
    /// archive, and you wish to refer to a particular entry within that archive,
    /// put the particular archive entry data here.
    #[prost(message, optional, tag = "5")]
    pub archive_entry: ::std::option::Option<ArchiveEntry>,
    /// (Optional) A url to a content display app/site for this file or archive
    /// entry.
    #[prost(string, tag = "6")]
    pub content_viewer: std::string::String,
    /// (Optional) Whether to hide this file or archive entry in the UI.  Defaults
    /// to false. A checkbox lets users see hidden files, but they're hidden by
    /// default.
    #[prost(bool, tag = "7")]
    pub hidden: bool,
    /// (Optional) A short description of what this file or archive entry
    /// contains. This description should help someone viewing the list of these
    /// files to understand the purpose of this file and what they would want to
    /// view it for.
    #[prost(string, tag = "8")]
    pub description: std::string::String,
    /// (Optional) digest of this file in hexadecimal-like string if known.
    #[prost(string, tag = "9")]
    pub digest: std::string::String,
    /// (Optional) The algorithm corresponding to the digest if known.
    #[prost(enumeration = "file::HashType", tag = "10")]
    pub hash_type: i32,
}
pub mod file {
    /// If known, the hash function used to compute this digest.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HashType {
        /// Unknown
        Unspecified = 0,
        /// MD5
        Md5 = 1,
        /// SHA-1
        Sha1 = 2,
        /// SHA-256
        Sha256 = 3,
    }
}
/// Information specific to an entry in an archive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArchiveEntry {
    /// The relative path of the entry within the archive.
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// (Optional) The uncompressed length of the archive entry in bytes.  Allows
    /// the entry size to be shown in the UI.  Omit if the length is not known.
    #[prost(message, optional, tag = "2")]
    pub length: ::std::option::Option<i64>,
    /// (Optional) The content-type (aka MIME-type) of the archive entry. (e.g.
    /// text/plain, image/jpeg, text/html, etc). This is sent to the web browser
    /// so it knows how to handle the entry.
    #[prost(string, tag = "3")]
    pub content_type: std::string::String,
}
/// Stores errors reading or parsing a file during post-processing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileProcessingErrors {
    /// The uid of the File being read or parsed.
    #[prost(string, tag = "1")]
    pub file_uid: std::string::String,
    /// What went wrong.
    #[prost(message, repeated, tag = "3")]
    pub file_processing_errors: ::std::vec::Vec<FileProcessingError>,
}
/// Stores an error reading or parsing a file during post-processing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileProcessingError {
    /// The type of error that occurred.
    #[prost(enumeration = "FileProcessingErrorType", tag = "1")]
    pub r#type: i32,
    /// Error message describing the problem.
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
/// Errors in file post-processing are categorized using this enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FileProcessingErrorType {
    /// Type unspecified or not listed here.
    Unspecified = 0,
    /// A read error occurred trying to read the file.
    GenericReadError = 1,
    /// There was an error trying to parse the file.
    GenericParseError = 2,
    /// File is exceeds size limit.
    FileTooLarge = 3,
    /// The result of parsing the file exceeded size limit.
    OutputTooLarge = 4,
    /// Read access to the file was denied by file system.
    AccessDenied = 5,
    /// Deadline exceeded trying to read the file.
    DeadlineExceeded = 6,
    /// File not found.
    NotFound = 7,
    /// File is empty but was expected to have content.
    FileEmpty = 8,
}
/// The result of running a test suite, as reported in a <testsuite> element of
/// an XML log.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestSuite {
    /// The full name of this suite, as reported in the name attribute. For Java
    /// tests, this is normally the fully qualified class name. Eg.
    /// "com.google.common.hash.BloomFilterTest".
    #[prost(string, tag = "1")]
    pub suite_name: std::string::String,
    /// The results of the test cases and test suites contained in this suite,
    /// as reported in the <testcase> and <testsuite> elements contained within
    /// this <testsuite>.
    #[prost(message, repeated, tag = "2")]
    pub tests: ::std::vec::Vec<Test>,
    /// Failures reported in <failure> elements within this <testsuite>.
    #[prost(message, repeated, tag = "3")]
    pub failures: ::std::vec::Vec<TestFailure>,
    /// Errors reported in <error> elements within this <testsuite>.
    #[prost(message, repeated, tag = "4")]
    pub errors: ::std::vec::Vec<TestError>,
    /// The timing for the entire TestSuite, as reported by the time attribute.
    #[prost(message, optional, tag = "6")]
    pub timing: ::std::option::Option<Timing>,
    /// Arbitrary name-value pairs, as reported in custom attributes or in a
    /// <properties> element within this <testsuite>. Multiple properties are
    /// allowed with the same key. Properties will be returned in lexicographical
    /// order by key.
    #[prost(message, repeated, tag = "7")]
    pub properties: ::std::vec::Vec<Property>,
    /// Files produced by this test suite, as reported by undeclared output
    /// annotations.
    /// The file IDs must be unique within this list. Duplicate file IDs will
    /// result in an error. Files will be returned in lexicographical order by ID.
    #[prost(message, repeated, tag = "8")]
    pub files: ::std::vec::Vec<File>,
}
/// The result of running a test case or test suite. JUnit3 TestDecorators are
/// represented as a TestSuite with a single test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Test {
    /// Either a TestCase of a TestSuite
    #[prost(oneof = "test::TestType", tags = "1, 2")]
    pub test_type: ::std::option::Option<test::TestType>,
}
pub mod test {
    /// Either a TestCase of a TestSuite
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TestType {
        /// When this contains just a single TestCase
        #[prost(message, tag = "1")]
        TestCase(super::TestCase),
        /// When this contains a TestSuite of test cases.
        #[prost(message, tag = "2")]
        TestSuite(super::TestSuite),
    }
}
/// The result of running a test case, as reported in a <testcase> element of
/// an XML log.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestCase {
    /// The name of the test case, as reported in the name attribute. For Java,
    /// this is normally the method name. Eg. "testBasic".
    #[prost(string, tag = "1")]
    pub case_name: std::string::String,
    /// The name of the class in which the test case was defined, as reported in
    /// the classname attribute. For Java, this is normally the fully qualified
    /// class name. Eg. "com.google.common.hash.BloomFilterTest".
    #[prost(string, tag = "2")]
    pub class_name: std::string::String,
    /// An enum reported in the result attribute that is used in conjunction with
    /// failures and errors below to report the outcome.
    #[prost(enumeration = "test_case::Result", tag = "3")]
    pub result: i32,
    /// Failures reported in <failure> elements within this <testcase>.
    #[prost(message, repeated, tag = "4")]
    pub failures: ::std::vec::Vec<TestFailure>,
    /// Errors reported in <error> elements within this <testcase>.
    #[prost(message, repeated, tag = "5")]
    pub errors: ::std::vec::Vec<TestError>,
    /// The timing for the TestCase, as reported by the time attribute.
    #[prost(message, optional, tag = "7")]
    pub timing: ::std::option::Option<Timing>,
    /// Arbitrary name-value pairs, as reported in custom attributes or in a
    /// <properties> element within this <testcase>. Multiple properties are
    /// allowed with the same key. Properties will be returned in lexicographical
    /// order by key.
    #[prost(message, repeated, tag = "8")]
    pub properties: ::std::vec::Vec<Property>,
    /// Files produced by this test case, as reported by undeclared output
    /// annotations.
    /// The file IDs must be unique within this list. Duplicate file IDs will
    /// result in an error. Files will be returned in lexicographical order by ID.
    #[prost(message, repeated, tag = "9")]
    pub files: ::std::vec::Vec<File>,
}
pub mod test_case {
    /// The result of running a test case.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        /// The implicit default enum value. Do not use.
        Unspecified = 0,
        /// Test case ran to completion. Look for failures or errors to determine
        /// whether it passed, failed, or errored.
        Completed = 1,
        /// Test case started but did not complete because the test harness received
        /// a signal and decided to stop running tests.
        Interrupted = 2,
        /// Test case was not started because the test harness received a SIGINT or
        /// timed out.
        Cancelled = 3,
        /// Test case was not run because the user or process running the test
        /// specified a filter that excluded this test case.
        Filtered = 4,
        /// Test case was not run to completion because the test case decided it
        /// should not be run (eg. due to a failed assumption in a JUnit4 test).
        /// Per-test setup or tear-down may or may not have run.
        Skipped = 5,
        /// The test framework did not run the test case because it was labeled as
        /// suppressed.  Eg. if someone temporarily disables a failing test.
        Suppressed = 6,
    }
}
/// Represents a violated assertion, as reported in a <failure> element within a
/// <testcase>. Some languages allow assertions to be made without stopping the
/// test case when they're violated, leading to multiple TestFailures. For Java,
/// multiple TestFailures are used to represent a chained exception.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestFailure {
    /// The exception message reported in the message attribute. Typically short,
    /// but may be multi-line. Eg. "Expected 'foo' but was 'bar'".
    #[prost(string, tag = "1")]
    pub failure_message: std::string::String,
    /// The type of the exception being thrown, reported in the type attribute.
    /// Eg: "org.junit.ComparisonFailure"
    #[prost(string, tag = "2")]
    pub exception_type: std::string::String,
    /// The stack trace reported as the content of the <failure> element, often in
    /// a CDATA block. This contains one line for each stack frame, each including
    /// a method/function name, a class/file name, and a line number. Most recent
    /// call is usually first, but not for Python stack traces. May contain the
    /// exception_type and message.
    #[prost(string, tag = "3")]
    pub stack_trace: std::string::String,
    /// The expected values.
    ///
    /// These values can be diffed against the actual values. Often, there is just
    /// one actual and one expected value. If there is more than one, they should
    /// be compared as an unordered collection.
    #[prost(string, repeated, tag = "4")]
    pub expected: ::std::vec::Vec<std::string::String>,
    /// The actual values.
    ///
    /// These values can be diffed against the expected values. Often, there is
    /// just one actual and one expected value. If there is more than one, they
    /// should be compared as an unordered collection.
    #[prost(string, repeated, tag = "5")]
    pub actual: ::std::vec::Vec<std::string::String>,
}
/// Represents an exception that prevented a test case from completing, as
/// reported in an <error> element within a <testcase>. For Java, multiple
/// TestErrors are used to represent a chained exception.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestError {
    /// The exception message, as reported in the message attribute. Typically
    /// short, but may be multi-line. Eg. "argument cannot be null".
    #[prost(string, tag = "1")]
    pub error_message: std::string::String,
    /// The type of the exception being thrown, reported in the type attribute.
    /// For Java, this is a fully qualified Throwable class name.
    /// Eg: "java.lang.IllegalArgumentException"
    #[prost(string, tag = "2")]
    pub exception_type: std::string::String,
    /// The stack trace reported as the content of the <error> element, often in
    /// a CDATA block. This contains one line for each stack frame, each including
    /// a method/function name, a class/file name, and a line number. Most recent
    /// call is usually first, but not for Python stack traces. May contain the
    /// exception_type and message.
    #[prost(string, tag = "3")]
    pub stack_trace: std::string::String,
}
/// An action that happened as part of a configured target. This action could be
/// a build, a test, or another type of action, as specified in action_type
/// oneof.
///
/// Each parent ConfiguredTarget resource should have at least one Action as its
/// child resource before the invocation is finalized. For a simple build, at
/// least one build action should be created to represent the build result, and
/// at at least one test action should be created to represent the test result,
/// if any.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    /// The resource name.  Its format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/url_encode(${CONFIG_ID})/actions/${url_encode(ACTION_ID)}
    ///
    /// See CreateActionRequest proto for more information.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource ID components that identify the Action. They must match the
    /// resource name after proper encoding.
    #[prost(message, optional, tag = "2")]
    pub id: ::std::option::Option<action::Id>,
    /// The status of the action.
    #[prost(message, optional, tag = "3")]
    pub status_attributes: ::std::option::Option<StatusAttributes>,
    /// The timing of the whole action. For TestActions, the start time may be
    /// before the test actually started, and the duration may last until after the
    /// test actually finished.
    #[prost(message, optional, tag = "4")]
    pub timing: ::std::option::Option<Timing>,
    /// General attributes of the action.
    #[prost(message, optional, tag = "5")]
    pub action_attributes: ::std::option::Option<ActionAttributes>,
    /// A list of resources that this action depended upon. May be used to provide
    /// the cause of a build failure in the case of a failed build action.
    #[prost(message, repeated, tag = "14")]
    pub action_dependencies: ::std::vec::Vec<Dependency>,
    /// Arbitrary name-value pairs.
    /// This is implemented as a multi-map. Multiple properties are allowed with
    /// the same key. Properties will be returned in lexicographical order by key.
    #[prost(message, repeated, tag = "7")]
    pub properties: ::std::vec::Vec<Property>,
    /// A list of file references for action level files.
    /// The file IDs must be unique within this list. Duplicate file IDs will
    /// result in an error. Files will be returned in lexicographical order by ID.
    ///
    /// Files with the following reserved file IDs cause specific post-processing
    /// or have special handling. These files must be immediately available to
    /// ResultStore for processing when the reference is uploaded.
    ///
    /// For build actions:
    /// stdout: The stdout of the action
    /// stderr: The stderr of the action
    /// baseline.lcov: Baseline coverage file to be parsed by the server. This
    ///     uses a stripped down implementation of the LCOV standard.
    ///     http://ltp.sourceforge.net/coverage/lcov/geninfo.1.php
    ///
    /// For test actions:
    /// test.xml: The test suite / test case data in XML format.
    /// test.log: The combined stdout and stderr of the test process.
    /// test.lcov: Coverage file to be parsed by the server. This uses a stripped
    ///     down implementation of the LCOV standard.
    ///     http://ltp.sourceforge.net/coverage/lcov/geninfo.1.php
    #[prost(message, repeated, tag = "8")]
    pub files: ::std::vec::Vec<File>,
    /// List of names of file sets that are referenced from this Action.
    /// Each name must point to a file set under the same Invocation. The name
    /// format must be: invocations/${INVOCATION_ID}/fileSets/${FILE_SET_ID}
    #[prost(string, repeated, tag = "15")]
    pub file_sets: ::std::vec::Vec<std::string::String>,
    /// Coverage data was collected while running the build or test action. This
    /// usually includes line coverage, and may also include branch coverage.
    /// For test actions, this is usually only for the source files which were
    /// actually executed by that particular action.
    /// For build actions, this is the baseline coverage, which captures the
    /// instrumented files and lines, without any lines being executed. This
    /// ensures files that are never covered at all are included.
    #[prost(message, optional, tag = "11")]
    pub coverage: ::std::option::Option<ActionCoverage>,
    /// ResultStore will read and parse Files with reserved IDs listed above. Read
    /// and parse errors for all these Files are reported here.
    /// This is implemented as a map, with one FileProcessingErrors for each file.
    /// Typically produced when parsing Files, but may also be provided directly
    /// by clients.
    #[prost(message, repeated, tag = "13")]
    pub file_processing_errors: ::std::vec::Vec<FileProcessingErrors>,
    /// The type of the action. The type of an action may not change over the
    /// lifetime of the invocation. If one of these fields is to be set, it must be
    /// set in the CreateAction method. It may be set to an empty message that is
    /// populated in later methods or post-processing. A generic "untyped" action
    /// can be created by not setting any of these fields. An untyped action will
    /// be untyped for the lifetime of the invocation.
    #[prost(oneof = "action::ActionType", tags = "9, 10")]
    pub action_type: ::std::option::Option<action::ActionType>,
}
pub mod action {
    /// The resource ID components that identify the Action.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Id {
        /// The Invocation ID.
        #[prost(string, tag = "1")]
        pub invocation_id: std::string::String,
        /// The Target ID.
        #[prost(string, tag = "2")]
        pub target_id: std::string::String,
        /// The Configuration ID.
        #[prost(string, tag = "3")]
        pub configuration_id: std::string::String,
        /// The Action ID.
        #[prost(string, tag = "4")]
        pub action_id: std::string::String,
    }
    /// The type of the action. The type of an action may not change over the
    /// lifetime of the invocation. If one of these fields is to be set, it must be
    /// set in the CreateAction method. It may be set to an empty message that is
    /// populated in later methods or post-processing. A generic "untyped" action
    /// can be created by not setting any of these fields. An untyped action will
    /// be untyped for the lifetime of the invocation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ActionType {
        /// Used only when this action represents a build action.
        #[prost(message, tag = "9")]
        BuildAction(super::BuildAction),
        /// Only for test actions.
        #[prost(message, tag = "10")]
        TestAction(super::TestAction),
    }
}
/// A build action, such as building a java library.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildAction {
    /// The type of the action.  This is intended to be a clue as to how the output
    /// of the action should be parsed. For example "javac" for a Java compile
    /// action.
    #[prost(string, tag = "1")]
    pub r#type: std::string::String,
    /// The "primary" input artifact processed by this action.  E.g., the .cc file
    /// of a C++ compile action.  Empty string ("") if the action has no input
    /// artifacts or no "primary" input artifact.
    #[prost(string, tag = "2")]
    pub primary_input_path: std::string::String,
    /// The "primary" output artifact processed by this action.  E.g., the .o file
    /// of a C++ compile action.  Empty string ("") if the action has no output
    /// artifacts or no "primary" output artifact.
    #[prost(string, tag = "3")]
    pub primary_output_path: std::string::String,
}
/// A test action, such as running a JUnit4 test binary.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestAction {
    /// Timing data for execution of the test action.
    #[prost(message, optional, tag = "1")]
    pub test_timing: ::std::option::Option<TestTiming>,
    /// If the test is divided up into shards to improve performance, set this to
    /// indicate which shard this test action is for. Value must be in interval
    /// [0, total_shard_count). Defaults to 0, which is appropriate if all test
    /// cases are run in the same process.
    #[prost(int32, tag = "2")]
    pub shard_number: i32,
    /// If the user requested that every test be run multiple times, as is often
    /// done to measure flakiness, set this to indicate which run this test action
    /// is for. Value must be in interval [0, total_run_count). Defaults to 0,
    /// which is appropriate if multiple runs were not requested.
    #[prost(int32, tag = "3")]
    pub run_number: i32,
    /// If flaky tests are automatically retried, set this to indicate which
    /// attempt this test action is for. (e.g. 0 for the first attempt, 1 for
    /// second, and so on). Defaults to 0, which is appropriate if this is only
    /// attempt.
    #[prost(int32, tag = "4")]
    pub attempt_number: i32,
    /// A tree of test suites and test cases that were run by this test action.
    /// Each test case has its own status information, including stack traces.
    /// Typically produced by parsing an XML Log, but may also be provided directly
    /// by clients.
    #[prost(message, optional, tag = "5")]
    pub test_suite: ::std::option::Option<TestSuite>,
    /// Warnings for this test action.
    #[prost(message, repeated, tag = "8")]
    pub warnings: ::std::vec::Vec<TestWarning>,
    /// Estimated memory consumption of the test action, in bytes. A default value
    /// of 0 means there is no memory consumption estimate specified.
    #[prost(int64, tag = "10")]
    pub estimated_memory_bytes: i64,
}
/// General attributes of an action
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionAttributes {
    /// Strategy used for executing the action.
    #[prost(enumeration = "ExecutionStrategy", tag = "1")]
    pub execution_strategy: i32,
    /// Exit code of the process that ran the action. A non-zero value means
    /// failure.
    #[prost(int32, tag = "2")]
    pub exit_code: i32,
    /// Where the action was run.
    #[prost(string, tag = "3")]
    pub hostname: std::string::String,
    /// Information about the input files used in all actions under this configured
    /// target.
    #[prost(message, optional, tag = "4")]
    pub input_file_info: ::std::option::Option<InputFileInfo>,
}
/// File count and size information for the input files to a configured target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputFileInfo {
    /// The number of input files (counting every file, even if a duplicate).
    #[prost(int64, tag = "1")]
    pub count: i64,
    /// The number of distinct input files.
    #[prost(int64, tag = "2")]
    pub distinct_count: i64,
    /// The max number of input files allowed by the build system (counting every
    /// file, even if a duplicate).
    #[prost(int64, tag = "3")]
    pub count_limit: i64,
    /// The total size of the distinct input files.
    #[prost(int64, tag = "4")]
    pub distinct_bytes: i64,
    /// The max allowed total size of the distinct input files.
    #[prost(int64, tag = "5")]
    pub distinct_byte_limit: i64,
}
/// Timing data for tests executed locally on the machine running the build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalTestTiming {
    /// Time taken by the test process, typically surrounded by a small wrapper
    /// script.
    #[prost(message, optional, tag = "1")]
    pub test_process_duration: ::std::option::Option<::prost_types::Duration>,
}
/// Timing data for one attempt to execute a test action remotely.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteTestAttemptTiming {
    /// Idle period before the test process is invoked on the remote machine.
    #[prost(message, optional, tag = "1")]
    pub queue_duration: ::std::option::Option<::prost_types::Duration>,
    /// Time to upload data dependencies from the local machine to the remote
    /// machine running the test, or to the distributed cache.
    #[prost(message, optional, tag = "2")]
    pub upload_duration: ::std::option::Option<::prost_types::Duration>,
    /// Time to set up the remote machine.
    /// Not to be confused with setup time in
    /// xUnit test frameworks, which falls within the test_process_time.
    #[prost(message, optional, tag = "3")]
    pub machine_setup_duration: ::std::option::Option<::prost_types::Duration>,
    /// Time taken by the test process, typically surrounded by a small wrapper
    /// script.
    /// For Java tests, this includes JVM setup, flag parsing, class path setup,
    /// parsing files to setup the suite, and finally running your test methods.
    /// In many cases, only a small fraction of the test process time is spent
    /// running the test methods.
    #[prost(message, optional, tag = "4")]
    pub test_process_duration: ::std::option::Option<::prost_types::Duration>,
    /// Time spent retrieving test logs and any other test outputs, back to the
    /// local machine.
    #[prost(message, optional, tag = "5")]
    pub download_duration: ::std::option::Option<::prost_types::Duration>,
}
/// Timing data for the part of the test execution that is done remotely.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteTestTiming {
    /// Time taken locally to determine what to do.
    #[prost(message, optional, tag = "1")]
    pub local_analysis_duration: ::std::option::Option<::prost_types::Duration>,
    /// Normally there is only one attempt, but the system may retry on internal
    /// errors, leading to multiple attempts.
    #[prost(message, repeated, tag = "2")]
    pub attempts: ::std::vec::Vec<RemoteTestAttemptTiming>,
}
/// Timing data for execution of a test action. The action may be performed
/// locally, on the machine running the build, or remotely.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestTiming {
    /// The amount of CPU time spent by the test process executing system calls
    /// within the kernel, as opposed to library code. Time the test process spent
    /// blocked does not count towards this figure.
    #[prost(message, optional, tag = "3")]
    pub system_time_duration: ::std::option::Option<::prost_types::Duration>,
    /// The amount of CPU time spent by the test process executing user-mode code
    /// outside the kernel, as opposed to library code. Time the test process
    /// spent blocked does not count towards this figure. You can add user_time to
    /// system_time to get total CPU time taken by the test process.
    #[prost(message, optional, tag = "4")]
    pub user_time_duration: ::std::option::Option<::prost_types::Duration>,
    /// Most build systems cache build results to speed up incremental builds.
    /// Some also cache test results too. This indicates whether the test results
    /// were found in a cache, and where that cache was located.
    #[prost(enumeration = "TestCaching", tag = "5")]
    pub test_caching: i32,
    /// Test timing for either a local or remote execution.
    #[prost(oneof = "test_timing::Location", tags = "1, 2")]
    pub location: ::std::option::Option<test_timing::Location>,
}
pub mod test_timing {
    /// Test timing for either a local or remote execution.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        /// Used for local test actions.
        #[prost(message, tag = "1")]
        Local(super::LocalTestTiming),
        /// Used for remote test actions.
        #[prost(message, tag = "2")]
        Remote(super::RemoteTestTiming),
    }
}
/// A warning from a test execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestWarning {
    /// Contains the message detailing the warning.
    #[prost(string, tag = "1")]
    pub warning_message: std::string::String,
}
/// Indicates how/where this Action was executed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionStrategy {
    /// The action did not indicate how it was executed.
    Unspecified = 0,
    /// The action was executed in some other form.
    OtherEnvironment = 1,
    /// The action used a remote build service.
    RemoteService = 2,
    /// The action was executed locally, in parallel with other actions.
    LocalParallel = 3,
    /// The action was executed locally, without parallelism.
    LocalSequential = 4,
}
/// Most build systems cache build results to speed up incremental builds.
/// Some also cache test results too. This indicates whether the test results
/// were found in a cache, and where that cache was located.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TestCaching {
    /// The implicit default enum value. Should never be set.
    Unspecified = 0,
    /// The test result was found in a local cache, so it wasn't run again.
    LocalCacheHit = 1,
    /// The test result was found in a remote cache, so it wasn't run again.
    RemoteCacheHit = 2,
    /// The test result was not found in any cache, so it had to be run again.
    CacheMiss = 3,
}
/// Represents a configuration within an Invocation associated with one or more
/// ConfiguredTargets. It captures the environment and other settings that
/// were used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Configuration {
    /// The format of this Configuration resource name must be:
    /// invocations/${INVOCATION_ID}/configs/${url_encode(CONFIG_ID)}
    /// The configuration ID of "default" should be preferred for the default
    /// configuration in a single-config invocation.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource ID components that identify the Configuration. They must match
    /// the resource name after proper encoding.
    #[prost(message, optional, tag = "2")]
    pub id: ::std::option::Option<configuration::Id>,
    /// The aggregate status for this configuration.
    #[prost(message, optional, tag = "3")]
    pub status_attributes: ::std::option::Option<StatusAttributes>,
    /// Attributes that apply only to this configuration.
    #[prost(message, optional, tag = "5")]
    pub configuration_attributes: ::std::option::Option<ConfigurationAttributes>,
    /// Arbitrary name-value pairs.
    /// This is implemented as a multi-map. Multiple properties are allowed with
    /// the same key. Properties will be returned in lexicographical order by key.
    #[prost(message, repeated, tag = "6")]
    pub properties: ::std::vec::Vec<Property>,
    /// A human-readable name for Configuration.
    /// It is recommended that this name be unique.
    /// If omitted, the configuration_id should be used as display_name instead.
    #[prost(string, tag = "8")]
    pub display_name: std::string::String,
}
pub mod configuration {
    /// The resource ID components that identify the Configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Id {
        /// The Invocation ID.
        #[prost(string, tag = "1")]
        pub invocation_id: std::string::String,
        /// The Configuration ID.
        #[prost(string, tag = "2")]
        pub configuration_id: std::string::String,
    }
}
/// Attributes that apply only to the configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigurationAttributes {
    /// The type of cpu. (e.g. "x86", "powerpc")
    #[prost(string, tag = "1")]
    pub cpu: std::string::String,
}
/// Each ConfiguredTarget represents data for a given configuration of a given
/// target in a given Invocation.
/// Every ConfiguredTarget should have at least one Action as a child resource
/// before the invocation is finalized. Refer to the Action's documentation for
/// more info on this.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfiguredTarget {
    /// The resource name.  Its format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIG_ID)}
    /// where ${CONFIG_ID} must match the ID of an existing Configuration under
    /// this Invocation.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource ID components that identify the ConfiguredTarget. They must
    /// match the resource name after proper encoding.
    #[prost(message, optional, tag = "2")]
    pub id: ::std::option::Option<configured_target::Id>,
    /// The aggregate status for this configuration of this target. If testing
    /// was not requested, set this to the build status (e.g. BUILT or
    /// FAILED_TO_BUILD).
    #[prost(message, optional, tag = "3")]
    pub status_attributes: ::std::option::Option<StatusAttributes>,
    /// Captures the start time and duration of this configured target.
    #[prost(message, optional, tag = "4")]
    pub timing: ::std::option::Option<Timing>,
    /// Test specific attributes for this ConfiguredTarget.
    #[prost(message, optional, tag = "6")]
    pub test_attributes: ::std::option::Option<ConfiguredTestAttributes>,
    /// Arbitrary name-value pairs.
    /// This is implemented as a multi-map. Multiple properties are allowed with
    /// the same key. Properties will be returned in lexicographical order by key.
    #[prost(message, repeated, tag = "7")]
    pub properties: ::std::vec::Vec<Property>,
    /// A list of file references for configured target level files.
    /// The file IDs must be unique within this list. Duplicate file IDs will
    /// result in an error. Files will be returned in lexicographical order by ID.
    #[prost(message, repeated, tag = "8")]
    pub files: ::std::vec::Vec<File>,
}
pub mod configured_target {
    /// The resource ID components that identify the ConfiguredTarget.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Id {
        /// The Invocation ID.
        #[prost(string, tag = "1")]
        pub invocation_id: std::string::String,
        /// The Target ID.
        #[prost(string, tag = "2")]
        pub target_id: std::string::String,
        /// The Configuration ID.
        #[prost(string, tag = "3")]
        pub configuration_id: std::string::String,
    }
}
/// Attributes that apply only to test actions under this configured target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfiguredTestAttributes {
    /// Total number of test runs. For example, in bazel this is specified with
    /// --runs_per_test. Zero if runs_per_test is not used.
    #[prost(int32, tag = "2")]
    pub total_run_count: i32,
    /// Total number of test shards. Zero if shard count was not specified.
    #[prost(int32, tag = "3")]
    pub total_shard_count: i32,
    /// How long test is allowed to run.
    #[prost(message, optional, tag = "5")]
    pub timeout_duration: ::std::option::Option<::prost_types::Duration>,
}
/// Summary of line coverage
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LineCoverageSummary {
    /// Number of lines instrumented for coverage.
    #[prost(int32, tag = "1")]
    pub instrumented_line_count: i32,
    /// Number of instrumented lines that were executed by the test.
    #[prost(int32, tag = "2")]
    pub executed_line_count: i32,
}
/// Summary of branch coverage
/// A branch may be:
///  * not executed.  Counted only in total.
///  * executed but not taken.  Appears in total and executed.
///  * executed and taken.  Appears in all three fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BranchCoverageSummary {
    /// The number of branches present in the file.
    #[prost(int32, tag = "1")]
    pub total_branch_count: i32,
    /// The number of branches executed out of the total branches present.
    /// A branch is executed when its condition is evaluated.
    /// This is <= total_branch_count as not all branches are executed.
    #[prost(int32, tag = "2")]
    pub executed_branch_count: i32,
    /// The number of branches taken out of the total branches executed.
    /// A branch is taken when its condition is satisfied.
    /// This is <= executed_branch_count as not all executed branches are taken.
    #[prost(int32, tag = "3")]
    pub taken_branch_count: i32,
}
/// Summary of coverage in each language
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageCoverageSummary {
    /// This summary is for all files written in this programming language.
    #[prost(enumeration = "Language", tag = "1")]
    pub language: i32,
    /// Summary of lines covered vs instrumented.
    #[prost(message, optional, tag = "2")]
    pub line_summary: ::std::option::Option<LineCoverageSummary>,
    /// Summary of branch coverage.
    #[prost(message, optional, tag = "3")]
    pub branch_summary: ::std::option::Option<BranchCoverageSummary>,
}
/// The download metadata for an invocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadMetadata {
    /// The name of the download metadata.  Its format will be:
    /// invocations/${INVOCATION_ID}/downloadMetadata
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Indicates the upload status of the invocation, whether it is
    /// post-processing, or immutable, etc.
    #[prost(enumeration = "UploadStatus", tag = "2")]
    pub upload_status: i32,
}
/// This resource represents a set of Files and other (nested) FileSets.
/// A FileSet is a node in the graph, and the file_sets field represents the
/// outgoing edges. A resource may reference various nodes in the graph to
/// represent the transitive closure of all files from those nodes.
/// The FileSets must form a directed acyclic graph. The Upload API is unable to
/// enforce that the graph is acyclic at write time, and if cycles are written,
/// it may cause issues at read time.
///
/// A FileSet may be referenced by other resources in conjunction with Files. A
/// File is preferred for something that can only be ever referenced by one
/// resource, and a FileSet is preferred if it can be reference by multiple
/// resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileSet {
    /// The format of this FileSet resource name must be:
    /// invocations/${INVOCATION_ID}/fileSets/${url_encode(FILE_SET_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource ID components that identify the file set. They must match the
    /// resource name after proper encoding.
    #[prost(message, optional, tag = "2")]
    pub id: ::std::option::Option<file_set::Id>,
    /// List of names of other file sets that are referenced from this one.
    /// Each name must point to a file set under the same invocation. The name
    /// format must be: invocations/${INVOCATION_ID}/fileSets/${FILE_SET_ID}
    #[prost(string, repeated, tag = "3")]
    pub file_sets: ::std::vec::Vec<std::string::String>,
    /// Files that are contained within this file set.
    /// The uid field in the file should be unique for the Invocation.
    #[prost(message, repeated, tag = "4")]
    pub files: ::std::vec::Vec<File>,
}
pub mod file_set {
    /// The resource ID components that identify the FileSet.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Id {
        /// The Invocation ID.
        #[prost(string, tag = "1")]
        pub invocation_id: std::string::String,
        /// The FileSet ID.
        #[prost(string, tag = "2")]
        pub file_set_id: std::string::String,
    }
}
/// An Invocation typically represents the result of running a tool. Each has a
/// unique ID, typically generated by the server. Target resources under each
/// Invocation contain the bulk of the data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invocation {
    /// The resource name.  Its format must be:
    /// invocations/${INVOCATION_ID}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource ID components that identify the Invocation. They must match
    /// the resource name after proper encoding.
    #[prost(message, optional, tag = "2")]
    pub id: ::std::option::Option<invocation::Id>,
    /// The aggregate status of the invocation.
    #[prost(message, optional, tag = "3")]
    pub status_attributes: ::std::option::Option<StatusAttributes>,
    /// When this invocation started and its duration.
    #[prost(message, optional, tag = "4")]
    pub timing: ::std::option::Option<Timing>,
    /// Attributes of this invocation.
    #[prost(message, optional, tag = "5")]
    pub invocation_attributes: ::std::option::Option<InvocationAttributes>,
    /// The workspace the tool was run in.
    #[prost(message, optional, tag = "6")]
    pub workspace_info: ::std::option::Option<WorkspaceInfo>,
    /// Arbitrary name-value pairs.
    /// This is implemented as a multi-map. Multiple properties are allowed with
    /// the same key. Properties will be returned in lexicographical order by key.
    #[prost(message, repeated, tag = "7")]
    pub properties: ::std::vec::Vec<Property>,
    /// A list of file references for invocation level files.
    /// The file IDs must be unique within this list. Duplicate file IDs will
    /// result in an error. Files will be returned in lexicographical order by ID.
    /// Use this field to specify build logs, and other invocation level logs.
    ///
    /// Files with the following reserved file IDs cause specific post-processing
    /// or have special handling. These files must be immediately available to
    /// ResultStore for processing when the reference is uploaded.
    ///
    /// build.log: The primary log for the Invocation.
    /// coverage_report.lcov: Aggregate coverage report for the invocation.
    #[prost(message, repeated, tag = "8")]
    pub files: ::std::vec::Vec<File>,
    /// Summary of aggregate coverage across all Actions in this Invocation.
    /// If missing, this data will be populated by the server from the
    /// coverage_report.lcov file or the union of all ActionCoverages under this
    /// invocation (in that order).
    #[prost(message, repeated, tag = "9")]
    pub coverage_summaries: ::std::vec::Vec<LanguageCoverageSummary>,
    /// Aggregate code coverage for all build and test Actions within this
    /// Invocation. If missing, this data will be populated by the server
    /// from the coverage_report.lcov file or the union of all ActionCoverages
    /// under this invocation (in that order).
    #[prost(message, optional, tag = "10")]
    pub aggregate_coverage: ::std::option::Option<AggregateCoverage>,
    /// NOT IMPLEMENTED.
    /// ResultStore will read and parse Files with reserved IDs listed above. Read
    /// and parse errors for all these Files are reported here.
    /// This is implemented as a map, with one FileProcessingErrors for each file.
    /// Typically produced when parsing Files, but may also be provided directly
    /// by clients.
    #[prost(message, repeated, tag = "11")]
    pub file_processing_errors: ::std::vec::Vec<FileProcessingErrors>,
}
pub mod invocation {
    /// The resource ID components that identify the Invocation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Id {
        /// The Invocation ID.
        #[prost(string, tag = "1")]
        pub invocation_id: std::string::String,
    }
}
/// If known, represents the state of the user/build-system workspace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkspaceContext {}
/// Describes the workspace under which the tool was invoked, this includes
/// information that was fed into the command, the source code referenced, and
/// the tool itself.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkspaceInfo {
    /// Data about the workspace that might be useful for debugging.
    #[prost(message, optional, tag = "1")]
    pub workspace_context: ::std::option::Option<WorkspaceContext>,
    /// Where the tool was invoked
    #[prost(string, tag = "3")]
    pub hostname: std::string::String,
    /// The client's working directory where the build/test was run from.
    #[prost(string, tag = "4")]
    pub working_directory: std::string::String,
    /// Tools should set tool_tag to the name of the tool or use case.
    #[prost(string, tag = "5")]
    pub tool_tag: std::string::String,
    /// The command lines invoked. The first command line is the one typed by the
    /// user, then each one after that should be an expansion of the previous
    /// command line.
    #[prost(message, repeated, tag = "7")]
    pub command_lines: ::std::vec::Vec<CommandLine>,
}
/// The command and arguments that produced this Invocation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandLine {
    /// A label describing this command line.
    #[prost(string, tag = "1")]
    pub label: std::string::String,
    /// The command-line tool that is run: argv[0].
    #[prost(string, tag = "2")]
    pub tool: std::string::String,
    /// The arguments to the above tool: argv[1]...argv[N].
    #[prost(string, repeated, tag = "3")]
    pub args: ::std::vec::Vec<std::string::String>,
    /// The actual command that was run with the tool.  (e.g. "build", or "test")
    /// Omit if the tool doesn't accept a command.
    /// This is a duplicate of one of the fields in args.
    #[prost(string, tag = "4")]
    pub command: std::string::String,
}
/// Attributes that apply to all invocations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvocationAttributes {
    /// Immutable. The Cloud Project that owns this invocation (this is different than the
    /// Consumer Cloud Project that calls this API).
    /// This must be set in the CreateInvocation call, and can't be changed.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// The list of users in the command chain.  The first user in this sequence
    /// is the one who instigated the first command in the chain. For example,
    /// this might contain just the user that ran a Bazel command, or a robot
    /// that tested a change as part of a CI system. It could also contain the user
    /// that manually triggered a CI test, then the robot that ran the test.
    #[prost(string, repeated, tag = "2")]
    pub users: ::std::vec::Vec<std::string::String>,
    /// Labels to categorize this invocation.
    /// This is implemented as a set. All labels will be unique. Any duplicate
    /// labels added will be ignored. Labels will be returned in lexicographical
    /// order. Labels should be a list of words describing the Invocation. Labels
    /// should be short, easy to read, and you shouldn't have more than a handful.
    /// Labels should not be used for unique properties such as unique IDs. Use
    /// properties in cases that don't meet these conditions.
    #[prost(string, repeated, tag = "3")]
    pub labels: ::std::vec::Vec<std::string::String>,
    /// This field describes the overall context or purpose of this invocation.
    /// It will be used in the UI to give users more information about
    /// how or why this invocation was run.
    #[prost(string, tag = "4")]
    pub description: std::string::String,
    /// If this Invocation was run in the context of a larger Continuous
    /// Integration build or other automated system, this field may contain more
    /// information about the greater context.
    #[prost(message, repeated, tag = "6")]
    pub invocation_contexts: ::std::vec::Vec<InvocationContext>,
}
/// Describes the invocation context which includes a display name and URL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvocationContext {
    /// A human readable name for the context under which this Invocation was run.
    #[prost(string, tag = "1")]
    pub display_name: std::string::String,
    /// A URL pointing to a UI containing more information
    #[prost(string, tag = "2")]
    pub url: std::string::String,
}
/// Each Target represents data for a given target in a given Invocation.
/// ConfiguredTarget and Action resources under each Target contain the bulk of
/// the data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Target {
    /// The resource name.  Its format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource ID components that identify the Target. They must match the
    /// resource name after proper encoding.
    #[prost(message, optional, tag = "2")]
    pub id: ::std::option::Option<target::Id>,
    /// This is the aggregate status of the target.
    #[prost(message, optional, tag = "3")]
    pub status_attributes: ::std::option::Option<StatusAttributes>,
    /// When this target started and its duration.
    #[prost(message, optional, tag = "4")]
    pub timing: ::std::option::Option<Timing>,
    /// Attributes that apply to all targets.
    #[prost(message, optional, tag = "5")]
    pub target_attributes: ::std::option::Option<TargetAttributes>,
    /// Attributes that apply to all test actions under this target.
    #[prost(message, optional, tag = "6")]
    pub test_attributes: ::std::option::Option<TestAttributes>,
    /// Arbitrary name-value pairs.
    /// This is implemented as a multi-map. Multiple properties are allowed with
    /// the same key. Properties will be returned in lexicographical order by key.
    #[prost(message, repeated, tag = "7")]
    pub properties: ::std::vec::Vec<Property>,
    /// A list of file references for target level files.
    /// The file IDs must be unique within this list. Duplicate file IDs will
    /// result in an error. Files will be returned in lexicographical order by ID.
    /// Use this field to specify outputs not related to a configuration.
    #[prost(message, repeated, tag = "8")]
    pub files: ::std::vec::Vec<File>,
    /// Provides a hint to clients as to whether to display the Target to users.
    /// If true then clients likely want to display the Target by default.
    /// Once set to true, this may not be set back to false.
    #[prost(bool, tag = "10")]
    pub visible: bool,
}
pub mod target {
    /// The resource ID components that identify the Target.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Id {
        /// The Invocation ID.
        #[prost(string, tag = "1")]
        pub invocation_id: std::string::String,
        /// The Target ID.
        #[prost(string, tag = "2")]
        pub target_id: std::string::String,
    }
}
/// Attributes that apply to all targets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetAttributes {
    /// If known, indicates the type of this target.  In bazel this corresponds
    /// to the rule-suffix.
    #[prost(enumeration = "TargetType", tag = "1")]
    pub r#type: i32,
    /// If known, the main language of this target, e.g. java, cc, python, etc.
    #[prost(enumeration = "Language", tag = "2")]
    pub language: i32,
    /// The tags attribute of the build rule. These should be short, descriptive
    /// words, and there should only be a few of them.
    /// This is implemented as a set. All tags will be unique. Any duplicate tags
    /// will be ignored. Tags will be returned in lexicographical order.
    #[prost(string, repeated, tag = "3")]
    pub tags: ::std::vec::Vec<std::string::String>,
}
/// Attributes that apply only to test actions under this target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestAttributes {
    /// Indicates how big the user indicated the test action was.
    #[prost(enumeration = "TestSize", tag = "1")]
    pub size: i32,
}
/// These correspond to the suffix of the rule name. Eg cc_test has type TEST.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TargetType {
    /// Unspecified by the build system.
    Unspecified = 0,
    /// An application e.g. ios_application.
    Application = 1,
    /// A binary target e.g. cc_binary.
    Binary = 2,
    /// A library target e.g. java_library
    Library = 3,
    /// A package
    Package = 4,
    /// Any test target, in bazel that means a rule with a '_test' suffix.
    Test = 5,
}
/// Indicates how big the user indicated the test action was.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TestSize {
    /// Unspecified by the user.
    Unspecified = 0,
    /// Unit test taking less than 1 minute.
    Small = 1,
    /// Integration tests taking less than 5 minutes.
    Medium = 2,
    /// End-to-end tests taking less than 15 minutes.
    Large = 3,
    /// Even bigger than LARGE.
    Enormous = 4,
    /// Something that doesn't fit into the above categories.
    OtherSize = 5,
}
/// Request passed into GetInvocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvocationRequest {
    /// The name of the invocation to retrieve. It must match this format:
    /// invocations/${INVOCATION_ID}
    /// where INVOCATION_ID must be an RFC 4122-compliant random UUID.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request passed into SearchInvocations
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchInvocationsRequest {
    /// The maximum number of items to return. Zero means all, but may be capped by
    /// the server.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A filtering query string.
    ///
    /// Only a limited number of fields and operators are supported. Not every
    /// field supports every operator.
    ///
    /// Fields that support equals ("=") restrictions:
    ///
    /// name
    /// status_attributes.status
    /// workspace_info.hostname
    ///
    /// Fields that support contains (":") restrictions:
    ///
    /// invocation_attributes.users
    /// invocation_attributes.labels
    ///
    /// Fields that support comparison ("<", "<=", ">", ">=") restrictions;
    ///
    /// timing.start_time
    ///
    /// Supported custom function global restrictions:
    ///
    /// propertyEquals("key", "value")
    #[prost(string, tag = "4")]
    pub query: std::string::String,
    /// The project id to search under.
    #[prost(string, tag = "5")]
    pub project_id: std::string::String,
    /// If true, all equals or contains restrictions on string fields in query will
    /// require exact match. Otherwise, a string field restriction may ignore case
    /// and punctuation.
    #[prost(bool, tag = "7")]
    pub exact_match: bool,
    /// Options for pagination.
    #[prost(oneof = "search_invocations_request::PageStart", tags = "2, 3")]
    pub page_start: ::std::option::Option<search_invocations_request::PageStart>,
}
pub mod search_invocations_request {
    /// Options for pagination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PageStart {
        /// The next_page_token value returned from a previous Search request, if
        /// any.
        #[prost(string, tag = "2")]
        PageToken(std::string::String),
        /// Absolute number of results to skip. May be rejected if too high.
        #[prost(int64, tag = "3")]
        Offset(i64),
    }
}
/// Response from calling SearchInvocations
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchInvocationsResponse {
    /// Invocations matching the search, possibly capped at request.page_size or a
    /// server limit.
    #[prost(message, repeated, tag = "1")]
    pub invocations: ::std::vec::Vec<Invocation>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request passed into GetInvocationDownloadMetadata
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvocationDownloadMetadataRequest {
    /// The name of the download metadata to retrieve. It must match this format:
    /// invocations/${INVOCATION_ID}/downloadMetadata
    /// where INVOCATION_ID must be an RFC 4122-compliant random UUID.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request passed into GetConfiguration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigurationRequest {
    /// The name of the configuration to retrieve. It must match this format:
    /// invocations/${INVOCATION_ID}/configs/${url_encode(CONFIGURATION_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request passed into ListConfigurations
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConfigurationsRequest {
    /// The invocation name of the configurations to retrieve.
    /// It must match this format: invocations/${INVOCATION_ID}
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    /// Zero means all, but may be capped by the server.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A filter to return only resources that match it.
    /// Any fields used in the filter must be also specified in the field mask.
    /// May cause pages with 0 results and a next_page_token to be returned.
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// Options for pagination.
    #[prost(oneof = "list_configurations_request::PageStart", tags = "3, 4")]
    pub page_start: ::std::option::Option<list_configurations_request::PageStart>,
}
pub mod list_configurations_request {
    /// Options for pagination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PageStart {
        /// The next_page_token value returned from a previous List request, if any.
        #[prost(string, tag = "3")]
        PageToken(std::string::String),
        /// Absolute number of results to skip.
        #[prost(int64, tag = "4")]
        Offset(i64),
    }
}
/// Response from calling ListConfigurations
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConfigurationsResponse {
    /// Configurations matching the request invocation,
    /// possibly capped at request.page_size or a server limit.
    #[prost(message, repeated, tag = "1")]
    pub configurations: ::std::vec::Vec<Configuration>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request passed into GetTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTargetRequest {
    /// The name of the target to retrieve. It must match this format:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request passed into ListTargets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTargetsRequest {
    /// The invocation name of the targets to retrieve. It must match this format:
    /// invocations/${INVOCATION_ID}
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    /// Zero means all, but may be capped by the server.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A filter to return only resources that match it.
    /// Any fields used in the filter must be also specified in the field mask.
    /// May cause pages with 0 results and a next_page_token to be returned.
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// Options for pagination.
    #[prost(oneof = "list_targets_request::PageStart", tags = "3, 4")]
    pub page_start: ::std::option::Option<list_targets_request::PageStart>,
}
pub mod list_targets_request {
    /// Options for pagination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PageStart {
        /// The next_page_token value returned from a previous List request, if any.
        #[prost(string, tag = "3")]
        PageToken(std::string::String),
        /// Absolute number of results to skip.
        #[prost(int64, tag = "4")]
        Offset(i64),
    }
}
/// Response from calling ListTargetsResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTargetsResponse {
    /// Targets matching the request invocation,
    /// possibly capped at request.page_size or a server limit.
    #[prost(message, repeated, tag = "1")]
    pub targets: ::std::vec::Vec<Target>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request passed into GetConfiguredTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfiguredTargetRequest {
    /// The name of the configured target to retrieve. It must match this format:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIGURATION_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request passed into ListConfiguredTargets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConfiguredTargetsRequest {
    /// The invocation and target name of the configured targets to retrieve.
    /// It must match this format:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}
    /// Supports '-' for ${TARGET_ID} meaning all targets.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    /// Zero means all, but may be capped by the server.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A filter to return only resources that match it.
    /// Any fields used in the filter must be also specified in the field mask.
    /// May cause pages with 0 results and a next_page_token to be returned.
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// Options for pagination.
    #[prost(oneof = "list_configured_targets_request::PageStart", tags = "3, 4")]
    pub page_start: ::std::option::Option<list_configured_targets_request::PageStart>,
}
pub mod list_configured_targets_request {
    /// Options for pagination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PageStart {
        /// The next_page_token value returned from a previous List request, if any.
        #[prost(string, tag = "3")]
        PageToken(std::string::String),
        /// Absolute number of results to skip.
        #[prost(int64, tag = "4")]
        Offset(i64),
    }
}
/// Response from calling ListConfiguredTargets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConfiguredTargetsResponse {
    /// ConfiguredTargets matching the request,
    /// possibly capped at request.page_size or a server limit.
    #[prost(message, repeated, tag = "1")]
    pub configured_targets: ::std::vec::Vec<ConfiguredTarget>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request passed into GetAction
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActionRequest {
    /// The name of the action to retrieve. It must match this format:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIGURATION_ID)}/actions/${url_encode(ACTION_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request passed into ListActions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActionsRequest {
    /// The invocation, target, and configuration name of the action to retrieve.
    /// It must match this format:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIGURATION_ID)}
    /// Supports '-' for ${CONFIGURATION_ID} to mean all Actions for all
    /// Configurations for a Target, or '-' for ${TARGET_ID} and
    /// ${CONFIGURATION_ID} to mean all Actions for all Configurations and all
    /// Targets. Does not support ${TARGET_ID} '-' with a specified configuration.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    /// Zero means all, but may be capped by the server.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A filter to return only resources that match it.
    /// Any fields used in the filter must be also specified in the field mask.
    /// May cause pages with 0 results and a next_page_token to be returned.
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// Options for pagination.
    #[prost(oneof = "list_actions_request::PageStart", tags = "3, 4")]
    pub page_start: ::std::option::Option<list_actions_request::PageStart>,
}
pub mod list_actions_request {
    /// Options for pagination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PageStart {
        /// The next_page_token value returned from a previous List request, if any.
        #[prost(string, tag = "3")]
        PageToken(std::string::String),
        /// Absolute number of results to skip.
        #[prost(int64, tag = "4")]
        Offset(i64),
    }
}
/// Response from calling ListActions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListActionsResponse {
    /// Actions matching the request,
    /// possibly capped at request.page_size or a server limit.
    #[prost(message, repeated, tag = "1")]
    pub actions: ::std::vec::Vec<Action>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request passed into GetFileSet
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileSetRequest {
    /// The name of the file set to retrieve. It must match this format:
    /// invocations/${INVOCATION_ID}/fileSets/${url_encode(FILE_SET_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request passed into ListFileSets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFileSetsRequest {
    /// The invocation name of the file sets to retrieve.
    /// It must match this format: invocations/${INVOCATION_ID}
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    /// Zero means all, but may be capped by the server.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A filter to return only resources that match it.
    /// Any fields used in the filter must be also specified in the field mask.
    /// May cause pages with 0 results and a next_page_token to be returned.
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// Options for pagination.
    #[prost(oneof = "list_file_sets_request::PageStart", tags = "3, 4")]
    pub page_start: ::std::option::Option<list_file_sets_request::PageStart>,
}
pub mod list_file_sets_request {
    /// Options for pagination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PageStart {
        /// The next_page_token value returned from a previous List request, if any.
        #[prost(string, tag = "3")]
        PageToken(std::string::String),
        /// Absolute number of results to skip.
        #[prost(int64, tag = "4")]
        Offset(i64),
    }
}
/// Response from calling ListFileSets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFileSetsResponse {
    /// File sets matching the request,
    /// possibly capped at request.page_size or a server limit.
    #[prost(message, repeated, tag = "1")]
    pub file_sets: ::std::vec::Vec<FileSet>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request passed into TraverseFileSets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraverseFileSetsRequest {
    /// The name of the resource to traverse.
    /// It must match one of the following formats:
    ///
    /// invocations/${INVOCATION_ID}/fileSets/${url_encode(FILE_SET_ID)}
    /// This returns the transitive closure of FileSets referenced by the given
    /// FileSet, including itself.
    ///
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIGURATION_ID)}/actions/${url_encode(ACTION_ID)}
    /// This returns the transitive closure of FileSets referenced by the given
    /// Action. If ${ACTION_ID} is "-", this returns the transitive closure of
    /// FileSets referenced by all Actions under the given ConfiguredTarget.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The maximum number of items to return.
    /// Zero means all, but may be capped by the server.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Options for pagination.
    #[prost(oneof = "traverse_file_sets_request::PageStart", tags = "3, 4")]
    pub page_start: ::std::option::Option<traverse_file_sets_request::PageStart>,
}
pub mod traverse_file_sets_request {
    /// Options for pagination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PageStart {
        /// The next_page_token value returned from a previous List request, if any.
        /// Page tokens will become larger with every page returned, and if a page
        /// token becomes too large, it will no longer be possible to continue to
        /// calculate the transitive dependencies. The API will return a 400
        /// Bad request (HTTPS), or a INVALID_ARGUMENT (gRPC ) when
        /// this happens.
        #[prost(string, tag = "3")]
        PageToken(std::string::String),
        /// Absolute number of results to skip.
        /// Not yet implemented. 0 for default.
        #[prost(int64, tag = "4")]
        Offset(i64),
    }
}
/// Response from calling TraverseFileSets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraverseFileSetsResponse {
    /// File sets matching the request.
    /// The order in which results are returned is undefined, but stable.
    #[prost(message, repeated, tag = "1")]
    pub file_sets: ::std::vec::Vec<FileSet>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod result_store_download_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " This is the interface used to download information from the ResultStore"]
    #[doc = " database."]
    #[doc = ""]
    #[doc = " Most APIs require setting a response FieldMask via the 'fields' URL query"]
    #[doc = " parameter or the X-Goog-FieldMask HTTP/gRPC header."]
    pub struct ResultStoreDownloadClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ResultStoreDownloadClient<T>
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
        #[doc = " Retrieves the invocation with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation is not found."]
        #[doc = " - If the given invocation name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn get_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInvocationRequest>,
        ) -> Result<tonic::Response<super::Invocation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/GetInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches for invocations matching the given query parameters. Results will"]
        #[doc = " be ordered by timing.start_time with most recent first, but total ordering"]
        #[doc = " of results is not guaranteed when difference in timestamps is very small."]
        #[doc = " Results may be stale."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If a query string is not provided"]
        #[doc = " - If no field mask was given."]
        pub async fn search_invocations(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchInvocationsRequest>,
        ) -> Result<tonic::Response<super::SearchInvocationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/SearchInvocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the metadata for an invocation with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation is not found."]
        #[doc = " - If the given invocation name is badly formatted."]
        pub async fn get_invocation_download_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInvocationDownloadMetadataRequest>,
        ) -> Result<tonic::Response<super::DownloadMetadata>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/GetInvocationDownloadMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the configuration with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configuration or its parent invocation is not found."]
        #[doc = " - If the given configuration name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn get_configuration(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfigurationRequest>,
        ) -> Result<tonic::Response<super::Configuration>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/GetConfiguration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves all configurations for a parent invocation."]
        #[doc = " This might be limited by user or server,"]
        #[doc = " in which case a continuation token is provided."]
        #[doc = " The order in which results are returned is undefined, but stable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the parent invocation is not found."]
        #[doc = " - If the given parent invocation name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn list_configurations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConfigurationsRequest>,
        ) -> Result<tonic::Response<super::ListConfigurationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/ListConfigurations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the target with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the target or its parent invocation is not found."]
        #[doc = " - If the given target name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn get_target(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTargetRequest>,
        ) -> Result<tonic::Response<super::Target>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/GetTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves all targets for a parent invocation.  This might be limited by"]
        #[doc = " user or server, in which case a continuation token is provided."]
        #[doc = " The order in which results are returned is undefined, but stable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the parent is not found."]
        #[doc = " - If the given parent name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn list_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTargetsRequest>,
        ) -> Result<tonic::Response<super::ListTargetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/ListTargets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the configured target with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configured target is not found."]
        #[doc = " - If the given name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn get_configured_target(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfiguredTargetRequest>,
        ) -> Result<tonic::Response<super::ConfiguredTarget>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/GetConfiguredTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves all configured targets for a parent invocation/target."]
        #[doc = " This might be limited by user or server, in which case a continuation"]
        #[doc = " token is provided.  Supports '-' for targetId meaning all targets."]
        #[doc = " The order in which results are returned is undefined, but stable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the parent is not found."]
        #[doc = " - If the given parent name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn list_configured_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConfiguredTargetsRequest>,
        ) -> Result<tonic::Response<super::ListConfiguredTargetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/ListConfiguredTargets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the action with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the action is not found."]
        #[doc = " - If the given name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn get_action(
            &mut self,
            request: impl tonic::IntoRequest<super::GetActionRequest>,
        ) -> Result<tonic::Response<super::Action>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/GetAction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves all actions for a parent invocation/target/configuration."]
        #[doc = " This might be limited by user or server, in which case a continuation"]
        #[doc = " token is provided.  Supports '-' for configurationId to mean all"]
        #[doc = " actions for all configurations for a target, or '-' for targetId and"]
        #[doc = " configurationId to mean all actions for all configurations and all targets."]
        #[doc = " Does not support targetId '-' with a specified configuration."]
        #[doc = " The order in which results are returned is undefined, but stable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the parent is not found."]
        #[doc = " - If the given parent name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn list_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListActionsRequest>,
        ) -> Result<tonic::Response<super::ListActionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/ListActions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the file set with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the file set or its parent invocation is not found."]
        #[doc = " - If the given file set name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn get_file_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFileSetRequest>,
        ) -> Result<tonic::Response<super::FileSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/GetFileSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves all file sets for a parent invocation."]
        #[doc = " This might be limited by user or server,"]
        #[doc = " in which case a continuation token is provided."]
        #[doc = " The order in which results are returned is undefined, but stable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the parent invocation is not found."]
        #[doc = " - If the given parent invocation name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn list_file_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFileSetsRequest>,
        ) -> Result<tonic::Response<super::ListFileSetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/ListFileSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the transitive closure of FileSets. This might be limited by user"]
        #[doc = " or server, in which case a continuation token is provided."]
        #[doc = " The order in which results are returned is undefined, and unstable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If page_token is too large to continue the calculation."]
        #[doc = " - If the resource is not found."]
        #[doc = " - If the given resource name is badly formatted."]
        #[doc = " - If no field mask was given."]
        pub async fn traverse_file_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::TraverseFileSetsRequest>,
        ) -> Result<tonic::Response<super::TraverseFileSetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreDownload/TraverseFileSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ResultStoreDownloadClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ResultStoreDownloadClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ResultStoreDownloadClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod result_store_download_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ResultStoreDownloadServer."]
    #[async_trait]
    pub trait ResultStoreDownload: Send + Sync + 'static {
        #[doc = " Retrieves the invocation with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation is not found."]
        #[doc = " - If the given invocation name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn get_invocation(
            &self,
            request: tonic::Request<super::GetInvocationRequest>,
        ) -> Result<tonic::Response<super::Invocation>, tonic::Status>;
        #[doc = " Searches for invocations matching the given query parameters. Results will"]
        #[doc = " be ordered by timing.start_time with most recent first, but total ordering"]
        #[doc = " of results is not guaranteed when difference in timestamps is very small."]
        #[doc = " Results may be stale."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If a query string is not provided"]
        #[doc = " - If no field mask was given."]
        async fn search_invocations(
            &self,
            request: tonic::Request<super::SearchInvocationsRequest>,
        ) -> Result<tonic::Response<super::SearchInvocationsResponse>, tonic::Status>;
        #[doc = " Retrieves the metadata for an invocation with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation is not found."]
        #[doc = " - If the given invocation name is badly formatted."]
        async fn get_invocation_download_metadata(
            &self,
            request: tonic::Request<super::GetInvocationDownloadMetadataRequest>,
        ) -> Result<tonic::Response<super::DownloadMetadata>, tonic::Status>;
        #[doc = " Retrieves the configuration with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configuration or its parent invocation is not found."]
        #[doc = " - If the given configuration name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn get_configuration(
            &self,
            request: tonic::Request<super::GetConfigurationRequest>,
        ) -> Result<tonic::Response<super::Configuration>, tonic::Status>;
        #[doc = " Retrieves all configurations for a parent invocation."]
        #[doc = " This might be limited by user or server,"]
        #[doc = " in which case a continuation token is provided."]
        #[doc = " The order in which results are returned is undefined, but stable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the parent invocation is not found."]
        #[doc = " - If the given parent invocation name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn list_configurations(
            &self,
            request: tonic::Request<super::ListConfigurationsRequest>,
        ) -> Result<tonic::Response<super::ListConfigurationsResponse>, tonic::Status>;
        #[doc = " Retrieves the target with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the target or its parent invocation is not found."]
        #[doc = " - If the given target name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn get_target(
            &self,
            request: tonic::Request<super::GetTargetRequest>,
        ) -> Result<tonic::Response<super::Target>, tonic::Status>;
        #[doc = " Retrieves all targets for a parent invocation.  This might be limited by"]
        #[doc = " user or server, in which case a continuation token is provided."]
        #[doc = " The order in which results are returned is undefined, but stable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the parent is not found."]
        #[doc = " - If the given parent name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn list_targets(
            &self,
            request: tonic::Request<super::ListTargetsRequest>,
        ) -> Result<tonic::Response<super::ListTargetsResponse>, tonic::Status>;
        #[doc = " Retrieves the configured target with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configured target is not found."]
        #[doc = " - If the given name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn get_configured_target(
            &self,
            request: tonic::Request<super::GetConfiguredTargetRequest>,
        ) -> Result<tonic::Response<super::ConfiguredTarget>, tonic::Status>;
        #[doc = " Retrieves all configured targets for a parent invocation/target."]
        #[doc = " This might be limited by user or server, in which case a continuation"]
        #[doc = " token is provided.  Supports '-' for targetId meaning all targets."]
        #[doc = " The order in which results are returned is undefined, but stable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the parent is not found."]
        #[doc = " - If the given parent name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn list_configured_targets(
            &self,
            request: tonic::Request<super::ListConfiguredTargetsRequest>,
        ) -> Result<tonic::Response<super::ListConfiguredTargetsResponse>, tonic::Status>;
        #[doc = " Retrieves the action with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the action is not found."]
        #[doc = " - If the given name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn get_action(
            &self,
            request: tonic::Request<super::GetActionRequest>,
        ) -> Result<tonic::Response<super::Action>, tonic::Status>;
        #[doc = " Retrieves all actions for a parent invocation/target/configuration."]
        #[doc = " This might be limited by user or server, in which case a continuation"]
        #[doc = " token is provided.  Supports '-' for configurationId to mean all"]
        #[doc = " actions for all configurations for a target, or '-' for targetId and"]
        #[doc = " configurationId to mean all actions for all configurations and all targets."]
        #[doc = " Does not support targetId '-' with a specified configuration."]
        #[doc = " The order in which results are returned is undefined, but stable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the parent is not found."]
        #[doc = " - If the given parent name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn list_actions(
            &self,
            request: tonic::Request<super::ListActionsRequest>,
        ) -> Result<tonic::Response<super::ListActionsResponse>, tonic::Status>;
        #[doc = " Retrieves the file set with the given name."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the file set or its parent invocation is not found."]
        #[doc = " - If the given file set name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn get_file_set(
            &self,
            request: tonic::Request<super::GetFileSetRequest>,
        ) -> Result<tonic::Response<super::FileSet>, tonic::Status>;
        #[doc = " Retrieves all file sets for a parent invocation."]
        #[doc = " This might be limited by user or server,"]
        #[doc = " in which case a continuation token is provided."]
        #[doc = " The order in which results are returned is undefined, but stable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the parent invocation is not found."]
        #[doc = " - If the given parent invocation name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn list_file_sets(
            &self,
            request: tonic::Request<super::ListFileSetsRequest>,
        ) -> Result<tonic::Response<super::ListFileSetsResponse>, tonic::Status>;
        #[doc = " Returns the transitive closure of FileSets. This might be limited by user"]
        #[doc = " or server, in which case a continuation token is provided."]
        #[doc = " The order in which results are returned is undefined, and unstable."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If page_token is too large to continue the calculation."]
        #[doc = " - If the resource is not found."]
        #[doc = " - If the given resource name is badly formatted."]
        #[doc = " - If no field mask was given."]
        async fn traverse_file_sets(
            &self,
            request: tonic::Request<super::TraverseFileSetsRequest>,
        ) -> Result<tonic::Response<super::TraverseFileSetsResponse>, tonic::Status>;
    }
    #[doc = " This is the interface used to download information from the ResultStore"]
    #[doc = " database."]
    #[doc = ""]
    #[doc = " Most APIs require setting a response FieldMask via the 'fields' URL query"]
    #[doc = " parameter or the X-Goog-FieldMask HTTP/gRPC header."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ResultStoreDownloadServer<T: ResultStoreDownload> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ResultStoreDownload> ResultStoreDownloadServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ResultStoreDownloadServer<T>
    where
        T: ResultStoreDownload,
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
            match req . uri ( ) . path ( ) { "/google.devtools.resultstore.v2.ResultStoreDownload/GetInvocation" => { # [ allow ( non_camel_case_types ) ] struct GetInvocationSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: GetInvocationRequest > for GetInvocationSvc < T > { type Response = super :: Invocation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetInvocationRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_invocation ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetInvocationSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/SearchInvocations" => { # [ allow ( non_camel_case_types ) ] struct SearchInvocationsSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: SearchInvocationsRequest > for SearchInvocationsSvc < T > { type Response = super :: SearchInvocationsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: SearchInvocationsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . search_invocations ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = SearchInvocationsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/GetInvocationDownloadMetadata" => { # [ allow ( non_camel_case_types ) ] struct GetInvocationDownloadMetadataSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: GetInvocationDownloadMetadataRequest > for GetInvocationDownloadMetadataSvc < T > { type Response = super :: DownloadMetadata ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetInvocationDownloadMetadataRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_invocation_download_metadata ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetInvocationDownloadMetadataSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/GetConfiguration" => { # [ allow ( non_camel_case_types ) ] struct GetConfigurationSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: GetConfigurationRequest > for GetConfigurationSvc < T > { type Response = super :: Configuration ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetConfigurationRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_configuration ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetConfigurationSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/ListConfigurations" => { # [ allow ( non_camel_case_types ) ] struct ListConfigurationsSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: ListConfigurationsRequest > for ListConfigurationsSvc < T > { type Response = super :: ListConfigurationsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListConfigurationsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_configurations ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListConfigurationsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/GetTarget" => { # [ allow ( non_camel_case_types ) ] struct GetTargetSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: GetTargetRequest > for GetTargetSvc < T > { type Response = super :: Target ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetTargetRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_target ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetTargetSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/ListTargets" => { # [ allow ( non_camel_case_types ) ] struct ListTargetsSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: ListTargetsRequest > for ListTargetsSvc < T > { type Response = super :: ListTargetsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListTargetsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_targets ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListTargetsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/GetConfiguredTarget" => { # [ allow ( non_camel_case_types ) ] struct GetConfiguredTargetSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: GetConfiguredTargetRequest > for GetConfiguredTargetSvc < T > { type Response = super :: ConfiguredTarget ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetConfiguredTargetRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_configured_target ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetConfiguredTargetSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/ListConfiguredTargets" => { # [ allow ( non_camel_case_types ) ] struct ListConfiguredTargetsSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: ListConfiguredTargetsRequest > for ListConfiguredTargetsSvc < T > { type Response = super :: ListConfiguredTargetsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListConfiguredTargetsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_configured_targets ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListConfiguredTargetsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/GetAction" => { # [ allow ( non_camel_case_types ) ] struct GetActionSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: GetActionRequest > for GetActionSvc < T > { type Response = super :: Action ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetActionRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_action ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetActionSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/ListActions" => { # [ allow ( non_camel_case_types ) ] struct ListActionsSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: ListActionsRequest > for ListActionsSvc < T > { type Response = super :: ListActionsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListActionsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_actions ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListActionsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/GetFileSet" => { # [ allow ( non_camel_case_types ) ] struct GetFileSetSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: GetFileSetRequest > for GetFileSetSvc < T > { type Response = super :: FileSet ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetFileSetRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_file_set ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetFileSetSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/ListFileSets" => { # [ allow ( non_camel_case_types ) ] struct ListFileSetsSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: ListFileSetsRequest > for ListFileSetsSvc < T > { type Response = super :: ListFileSetsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListFileSetsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_file_sets ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListFileSetsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.resultstore.v2.ResultStoreDownload/TraverseFileSets" => { # [ allow ( non_camel_case_types ) ] struct TraverseFileSetsSvc < T : ResultStoreDownload > ( pub Arc < T > ) ; impl < T : ResultStoreDownload > tonic :: server :: UnaryService < super :: TraverseFileSetsRequest > for TraverseFileSetsSvc < T > { type Response = super :: TraverseFileSetsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: TraverseFileSetsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . traverse_file_sets ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = TraverseFileSetsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: ResultStoreDownload> Clone for ResultStoreDownloadServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ResultStoreDownload> Clone for _Inner<T> {
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
/// Request object for GetFile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileRequest {
    /// This corresponds to the uri field in the File message.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
    /// The offset for the first byte to return in the read, relative to the start
    /// of the resource.
    ///
    /// A `read_offset` that is negative or greater than the size of the resource
    /// will cause an `OUT_OF_RANGE` error.
    #[prost(int64, tag = "2")]
    pub read_offset: i64,
    /// The maximum number of `data` bytes the server is allowed to return in the
    /// sum of all `ReadResponse` messages. A `read_limit` of zero indicates that
    /// there is no limit, and a negative `read_limit` will cause an error.
    ///
    /// If the stream returns fewer bytes than allowed by the `read_limit` and no
    /// error occurred, the stream includes all data from the `read_offset` to the
    /// end of the resource.
    #[prost(int64, tag = "3")]
    pub read_limit: i64,
    /// Only applies if the referenced file is a known archive type (ar, jar, zip)
    /// The above read_offset and read_limit fields are applied to this entry.
    /// If this file is not an archive, INVALID_ARGUMENT is thrown.
    #[prost(string, tag = "4")]
    pub archive_entry: std::string::String,
}
/// Response object for GetFile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileResponse {
    /// The file data.
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
}
/// Request object for GetFileTail
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileTailRequest {
    /// This corresponds to the uri field in the File message.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
    /// The offset for the first byte to return in the read, relative to the end
    /// of the resource.
    ///
    /// A `read_offset` that is negative or greater than the size of the resource
    /// will cause an `OUT_OF_RANGE` error.
    #[prost(int64, tag = "2")]
    pub read_offset: i64,
    /// The maximum number of `data` bytes the server is allowed to return. The
    /// server will return bytes starting from the tail of the file.
    ///
    /// A `read_limit` of zero indicates that there is no limit, and a negative
    /// `read_limit` will cause an error.
    #[prost(int64, tag = "3")]
    pub read_limit: i64,
    /// Only applies if the referenced file is a known archive type (ar, jar, zip)
    /// The above read_offset and read_limit fields are applied to this entry.
    /// If this file is not an archive, INVALID_ARGUMENT is thrown.
    #[prost(string, tag = "4")]
    pub archive_entry: std::string::String,
}
/// Response object for GetFileTail
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileTailResponse {
    /// The file data, encoded with UTF-8.
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod result_store_file_download_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " This API allows download of File messages referenced in"]
    #[doc = " ResultStore resources."]
    pub struct ResultStoreFileDownloadClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ResultStoreFileDownloadClient<T>
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
        #[doc = " Retrieves the File with the given uri."]
        #[doc = " returns a stream of bytes to be stitched together in order."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the File is not found."]
        #[doc = " - If the given File uri is badly formatted."]
        pub async fn get_file(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFileRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::GetFileResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreFileDownload/GetFile",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Retrieves the tail of a File with the given uri."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the File is not found."]
        #[doc = " - If the given File uri is badly formatted."]
        pub async fn get_file_tail(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFileTailRequest>,
        ) -> Result<tonic::Response<super::GetFileTailResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreFileDownload/GetFileTail",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ResultStoreFileDownloadClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ResultStoreFileDownloadClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ResultStoreFileDownloadClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod result_store_file_download_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ResultStoreFileDownloadServer."]
    #[async_trait]
    pub trait ResultStoreFileDownload: Send + Sync + 'static {
        #[doc = "Server streaming response type for the GetFile method."]
        type GetFileStream: Stream<Item = Result<super::GetFileResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Retrieves the File with the given uri."]
        #[doc = " returns a stream of bytes to be stitched together in order."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the File is not found."]
        #[doc = " - If the given File uri is badly formatted."]
        async fn get_file(
            &self,
            request: tonic::Request<super::GetFileRequest>,
        ) -> Result<tonic::Response<Self::GetFileStream>, tonic::Status>;
        #[doc = " Retrieves the tail of a File with the given uri."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the File is not found."]
        #[doc = " - If the given File uri is badly formatted."]
        async fn get_file_tail(
            &self,
            request: tonic::Request<super::GetFileTailRequest>,
        ) -> Result<tonic::Response<super::GetFileTailResponse>, tonic::Status>;
    }
    #[doc = " This API allows download of File messages referenced in"]
    #[doc = " ResultStore resources."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ResultStoreFileDownloadServer<T: ResultStoreFileDownload> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ResultStoreFileDownload> ResultStoreFileDownloadServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ResultStoreFileDownloadServer<T>
    where
        T: ResultStoreFileDownload,
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
                "/google.devtools.resultstore.v2.ResultStoreFileDownload/GetFile" => {
                    #[allow(non_camel_case_types)]
                    struct GetFileSvc<T: ResultStoreFileDownload>(pub Arc<T>);
                    impl<T: ResultStoreFileDownload>
                        tonic::server::ServerStreamingService<super::GetFileRequest>
                        for GetFileSvc<T>
                    {
                        type Response = super::GetFileResponse;
                        type ResponseStream = T::GetFileStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_file(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetFileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.devtools.resultstore.v2.ResultStoreFileDownload/GetFileTail" => {
                    #[allow(non_camel_case_types)]
                    struct GetFileTailSvc<T: ResultStoreFileDownload>(pub Arc<T>);
                    impl<T: ResultStoreFileDownload>
                        tonic::server::UnaryService<super::GetFileTailRequest>
                        for GetFileTailSvc<T>
                    {
                        type Response = super::GetFileTailResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFileTailRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_file_tail(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetFileTailSvc(inner);
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
    impl<T: ResultStoreFileDownload> Clone for ResultStoreFileDownloadServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ResultStoreFileDownload> Clone for _Inner<T> {
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
/// The upload metadata for an invocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadMetadata {
    /// The name of the upload metadata.  Its format will be:
    /// invocations/${INVOCATION_ID}/uploadMetadata
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resume token of the last batch that was committed in the most recent
    /// batch upload.
    /// More information with resume_token could be found in
    /// resultstore_upload.proto
    #[prost(string, tag = "2")]
    pub resume_token: std::string::String,
    /// Client-specific data used to resume batch upload if an error occurs and
    /// retry action is needed.
    #[prost(bytes, tag = "3")]
    pub uploader_state: std::vec::Vec<u8>,
}
/// Request passed into CreateInvocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInvocationRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID. If set, invocation_id must also be provided.
    /// Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// The invocation ID. It is optional, but strongly recommended.
    ///
    /// If left empty then a new unique ID will be assigned by the server. If
    /// populated, a RFC 4122-compliant v4 UUID is preferred, but v3 or v5 UUIDs
    /// are allowed too.
    #[prost(string, tag = "2")]
    pub invocation_id: std::string::String,
    /// The invocation to create.  Its name field will be ignored, since the name
    /// will be derived from the id field above and assigned by the server.
    #[prost(message, optional, tag = "3")]
    pub invocation: ::std::option::Option<Invocation>,
    /// This is a token to authorize upload access to this invocation. It must be
    /// set to a RFC 4122-compliant v3, v4, or v5 UUID. Once this is set in
    /// CreateInvocation, all other upload RPCs for that Invocation and any of its
    /// child resources must also include the exact same token, or they will be
    /// rejected. The generated token should be unique to this invocation, and it
    /// should be kept secret.
    ///
    /// The purpose of this field is to prevent other users and tools from
    /// clobbering your upload intentionally or accidentally. The standard way of
    /// using this token is to create a second v4 UUID when the invocation_id is
    /// created, and storing them together during the upload. Essentially, this is
    /// a "password" to the invocation.
    #[prost(string, tag = "4")]
    pub authorization_token: std::string::String,
    /// By default, Invocations are auto-finalized if they are not modified for 24
    /// hours. If you need auto-finalize to happen sooner, set this field to the
    /// time you'd like auto-finalize to occur.
    #[prost(message, optional, tag = "6")]
    pub auto_finalize_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Client provided unique token for batch upload to ensure data integrity and
    /// to provide a way to resume batch upload in case of a distributed failure on
    /// the client side. The standard uploading client is presumed to have many
    /// machines uploading to ResultStore, and that any given machine could process
    /// any given Invocation at any time. This field is used to coordinate between
    /// the client's machines, resolve concurrency issues, and enforce "exactly
    /// once" semantics on each batch within the upload.
    ///
    /// The typical usage of the resume_token is that it should contain a "key"
    /// indicating to the client where it is in the upload process, so that the
    /// client can use it to resume the upload by reconstructing the state of
    /// upload from the point where it was interrupted.
    ///
    /// If this matches the previously uploaded resume_token, then this request
    /// will silently do nothing, making CreateInvocation idempotent.
    /// If this token is provided, all further upload RPCs must be done through
    /// UploadBatch. This token must not be combined with request_id.
    /// Must be web safe Base64 encoded bytes.
    #[prost(string, tag = "7")]
    pub initial_resume_token: std::string::String,
    /// Client-specific data used to resume batch upload if an error occurs and
    /// retry is needed. This serves a role closely related to resume_token, as
    /// both fields may be used to provide state required to restore a Batch
    /// Upload, but they differ in two important aspects:
    ///  - it is not compared to previous values, and as such does not provide
    ///    concurrency control;
    ///  - it allows for a larger payload, since the contents are never
    ///    inspected/compared;
    /// The size of the message must be within 1 MiB. Too large requests will be
    /// rejected.
    #[prost(bytes, tag = "8")]
    pub uploader_state: std::vec::Vec<u8>,
}
/// Request passed into UpdateInvocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInvocationRequest {
    /// Contains the name and the fields of the invocation to be updated.  The
    /// name format must be: invocations/${INVOCATION_ID}
    #[prost(message, optional, tag = "3")]
    pub invocation: ::std::option::Option<Invocation>,
    /// Indicates which fields to update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
}
/// Request passed into MergeInvocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeInvocationRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID.  Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Contains the name and the fields of the invocation to be merged.  The
    /// name format must be: invocations/${INVOCATION_ID}
    #[prost(message, optional, tag = "3")]
    pub invocation: ::std::option::Option<Invocation>,
    /// Indicates which fields to merge.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
}
/// Request passed into TouchInvocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TouchInvocationRequest {
    /// The name of the invocation.  Its format must be:
    /// invocations/${INVOCATION_ID}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "2")]
    pub authorization_token: std::string::String,
}
/// Response returned from TouchInvocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TouchInvocationResponse {
    /// The name of the invocation.  Its format will be:
    /// invocations/${INVOCATION_ID}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource ID components that identify the Invocation.
    #[prost(message, optional, tag = "2")]
    pub id: ::std::option::Option<invocation::Id>,
}
/// Request passed into DeleteInvocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInvocationRequest {
    /// The name of the invocation.  Its format must be:
    /// invocations/${INVOCATION_ID}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request passed into FinalizeInvocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeInvocationRequest {
    /// The name of the invocation.  Its format must be:
    /// invocations/${INVOCATION_ID}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "3")]
    pub authorization_token: std::string::String,
}
/// Response returned from FinalizeInvocation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeInvocationResponse {
    /// The name of the invocation.  Its format will be:
    /// invocations/${INVOCATION_ID}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource ID components that identify the Invocation.
    #[prost(message, optional, tag = "2")]
    pub id: ::std::option::Option<invocation::Id>,
}
/// Request passed into CreateTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTargetRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID.  Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// The name of the parent invocation in which the target is created.
    /// Its format must be invocations/${INVOCATION_ID}
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
    /// The target identifier.  It can be any string up to 1024 Unicode characters
    /// long except for the reserved id '-'.
    #[prost(string, tag = "3")]
    pub target_id: std::string::String,
    /// The target to create.  Its name field will be ignored, since the name will
    /// be derived from the id field above and assigned by the server.
    #[prost(message, optional, tag = "4")]
    pub target: ::std::option::Option<Target>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
}
/// Request passed into UpdateTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTargetRequest {
    /// Contains the name and the fields of the target to be updated.  The name
    /// format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}
    #[prost(message, optional, tag = "3")]
    pub target: ::std::option::Option<Target>,
    /// Indicates which fields to update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
    /// If true then the Update operation will become a Create operation if the
    /// Target is NOT_FOUND.
    #[prost(bool, tag = "6")]
    pub create_if_not_found: bool,
}
/// Request passed into MergeTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeTargetRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID.  Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Contains the name and the fields of the target to be merged.  The name
    /// format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}
    #[prost(message, optional, tag = "3")]
    pub target: ::std::option::Option<Target>,
    /// Indicates which fields to merge.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
    /// If true then the Merge operation will become a Create operation if the
    /// Target is NOT_FOUND.
    #[prost(bool, tag = "6")]
    pub create_if_not_found: bool,
}
/// Request passed into FinalizeTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeTargetRequest {
    /// The name of the target.  Its format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "3")]
    pub authorization_token: std::string::String,
}
/// Response returned from FinalizeTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeTargetResponse {
    /// The name of the target.  Its format will be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource ID components that identify the Target.
    #[prost(message, optional, tag = "2")]
    pub id: ::std::option::Option<target::Id>,
}
/// Request passed into CreateConfiguredTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConfiguredTargetRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID.  Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// The name of the parent target in which the configured target is created.
    /// Its format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
    /// The configuration identifier. This must match the ID of an existing
    /// Configuration under this Invocation. Cannot be the reserved id '-'.
    #[prost(string, tag = "3")]
    pub config_id: std::string::String,
    /// The configured target to create. Its name field will be ignored, since the
    /// name will be derived from the id field above and assigned by the server.
    #[prost(message, optional, tag = "4")]
    pub configured_target: ::std::option::Option<ConfiguredTarget>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
}
/// Request passed into UpdateConfiguredTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConfiguredTargetRequest {
    /// Contains the name and the fields of the configured target to be updated.
    /// The name format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIG_ID)}
    #[prost(message, optional, tag = "3")]
    pub configured_target: ::std::option::Option<ConfiguredTarget>,
    /// Indicates which fields to update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
    /// If true then the Update operation will become a Create operation if the
    /// ConfiguredTarget is NOT_FOUND.
    #[prost(bool, tag = "6")]
    pub create_if_not_found: bool,
}
/// Request passed into MergeConfiguredTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeConfiguredTargetRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID.  Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Contains the name and the fields of the configured target to be merged.
    /// The name format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIG_ID)}
    #[prost(message, optional, tag = "3")]
    pub configured_target: ::std::option::Option<ConfiguredTarget>,
    /// Indicates which fields to merge.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
    /// If true then the Merge operation will become a Create operation if the
    /// ConfiguredTarget is NOT_FOUND.
    #[prost(bool, tag = "6")]
    pub create_if_not_found: bool,
}
/// Request passed into FinalizeConfiguredTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeConfiguredTargetRequest {
    /// The name of the configured target. Its format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIG_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "3")]
    pub authorization_token: std::string::String,
}
/// Response returned from FinalizeConfiguredTarget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeConfiguredTargetResponse {
    /// The name of the configured target. Its format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIG_ID)}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource ID components that identify the ConfiguredTarget.
    #[prost(message, optional, tag = "2")]
    pub id: ::std::option::Option<configured_target::Id>,
}
/// Request passed into CreateAction
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateActionRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID.  Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// The name of the parent configured target in which the action is created.
    /// Its format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIG_ID)}
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
    /// The action identifier. It can be any string up to 512 Unicode characters
    /// long, except for the reserved id '-'.
    ///
    /// Recommended IDs for Test Actions:
    /// "test": For a single test action.
    /// "test_shard0_run0_attempt0" ... "test_shard9_run9_attempt9": For tests with
    ///  shard/run/attempts.
    ///
    /// Recommended IDs for Build Actions:
    /// "build": If you only have a single build action.
    #[prost(string, tag = "3")]
    pub action_id: std::string::String,
    /// The action to create.  Its name field will be ignored, since the
    /// name will be derived from the id field above and assigned by the server.
    #[prost(message, optional, tag = "4")]
    pub action: ::std::option::Option<Action>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
}
/// Request passed into UpdateAction
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateActionRequest {
    /// Contains the name and the fields of the action to be updated.  The
    /// name format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIG_ID)}/actions/${url_encode(ACTION_ID)}
    #[prost(message, optional, tag = "3")]
    pub action: ::std::option::Option<Action>,
    /// Indicates which fields to update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
    /// If true then the Update operation will become a Create operation if the
    /// Action is NOT_FOUND.
    #[prost(bool, tag = "6")]
    pub create_if_not_found: bool,
}
/// Request passed into MergeAction
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeActionRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID.  Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Contains the name and the fields of the action to be merged.  The
    /// name format must be:
    /// invocations/${INVOCATION_ID}/targets/${url_encode(TARGET_ID)}/configuredTargets/${url_encode(CONFIG_ID)}/actions/${url_encode(ACTION_ID)}
    #[prost(message, optional, tag = "3")]
    pub action: ::std::option::Option<Action>,
    /// Indicates which fields to merge.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
    /// If true then the Merge operation will become a Create operation if the
    /// Action is NOT_FOUND.
    #[prost(bool, tag = "6")]
    pub create_if_not_found: bool,
}
/// Request passed into CreateConfiguration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConfigurationRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID.  Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// The name of the parent invocation in which the configuration is created.
    /// Its format must be invocations/${INVOCATION_ID}
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
    /// The configuration identifier. It can be any string up to 256 Unicode
    /// characters long. The configuration ID of "default" should be preferred for
    /// the default configuration in a single-config invocation. Cannot be the
    /// reserved id '-'.
    #[prost(string, tag = "3")]
    pub config_id: std::string::String,
    /// The configuration to create. Its name field will be ignored, since the name
    /// will be derived from the id field above and assigned by the server.
    #[prost(message, optional, tag = "4")]
    pub configuration: ::std::option::Option<Configuration>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
}
/// Request passed into UpdateConfiguration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConfigurationRequest {
    /// Contains the name and fields of the configuration to be updated. The name
    /// format must be:
    /// invocations/${INVOCATION_ID}/configs/${url_encode(CONFIG_ID)}
    #[prost(message, optional, tag = "3")]
    pub configuration: ::std::option::Option<Configuration>,
    /// Indicates which fields to update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
    /// If true then the Update operation will become a Create operation if the
    /// Configuration is NOT_FOUND.
    #[prost(bool, tag = "6")]
    pub create_if_not_found: bool,
}
/// Request passed into CreateFileSet
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFileSetRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID.  Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// The name of the parent invocation in which the file set is created.
    /// Its format must be invocations/${INVOCATION_ID}
    #[prost(string, tag = "2")]
    pub parent: std::string::String,
    /// The file set identifier. It can be any string up to 256 Unicode characters
    /// long.
    #[prost(string, tag = "3")]
    pub file_set_id: std::string::String,
    /// The file set to create. Its name field will be ignored, since the name will
    /// be derived from the id field above and assigned by the server.
    #[prost(message, optional, tag = "4")]
    pub file_set: ::std::option::Option<FileSet>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "5")]
    pub authorization_token: std::string::String,
}
/// Request passed into UpdateFileSet
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFileSetRequest {
    /// Contains the name and fields of the file set to be updated. The name format
    /// must be: invocations/${INVOCATION_ID}/fileSets/${url_encode(FILE_SET_ID)}
    #[prost(message, optional, tag = "1")]
    pub file_set: ::std::option::Option<FileSet>,
    /// Indicates which fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "3")]
    pub authorization_token: std::string::String,
    /// If true then the Update operation will become a Create operation if the
    /// FileSet is NOT_FOUND.
    #[prost(bool, tag = "4")]
    pub create_if_not_found: bool,
}
/// Request passed into MergeFileSet
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeFileSetRequest {
    /// A unique identifier for this request. Must be set to a different value for
    /// each request that affects a given resource (eg. a random UUID). Required
    /// for the operation to be idempotent. This is achieved by ignoring this
    /// request if the last successful operation on the resource had the same
    /// request ID.  Restricted to 36 Unicode characters.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Contains the name and fields of the file set to be merged. The name
    /// format must be:
    /// invocations/${INVOCATION_ID}/fileSets/${url_encode(FILE_SET_ID)}
    #[prost(message, optional, tag = "2")]
    pub file_set: ::std::option::Option<FileSet>,
    /// Indicates which fields to merge.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// This is a token to authorize access to this invocation. It must be set to
    /// the same value that was provided in the CreateInvocationRequest.
    #[prost(string, tag = "4")]
    pub authorization_token: std::string::String,
    /// If true then the Merge operation will become a Create operation if the
    /// FileSet is NOT_FOUND.
    #[prost(bool, tag = "5")]
    pub create_if_not_found: bool,
}
/// Request passed into UploadBatch
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadBatchRequest {
    /// Required. The name of the invocation being modified.
    /// The name format must be: invocations/${INVOCATION_ID}
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. A UUID that must match the value provided in CreateInvocationRequest.
    #[prost(string, tag = "2")]
    pub authorization_token: std::string::String,
    /// Required. The token of this batch, that will be committed in this UploadBatchRequest.
    /// If this matches the previously uploaded resume_token, then this request
    /// will silently do nothing.
    /// See CreateInvocationRequest.initial_resume_token for more information.
    /// Must be web safe Base64 encoded bytes.
    #[prost(string, tag = "3")]
    pub next_resume_token: std::string::String,
    /// Required. The token of the previous batch that was committed in a UploadBatchRequest.
    /// This will be checked after next_resume_token match is checked. If this does
    /// not match the previously uploaded resume_token, a 409 Conflict (HTTPS) or
    /// ABORTED (gRPC ) error code indicating a concurrency
    /// failure will be returned, and that the user should call
    /// GetInvocationUploadMetadata to fetch the current resume_token to
    /// reconstruct the state of the upload to resume it.
    /// See CreateInvocationRequest.initial_resume_token for more information.
    /// Must be web safe Base64 encoded bytes.
    #[prost(string, tag = "4")]
    pub resume_token: std::string::String,
    /// Client-specific data used to resume batch upload if an error occurs and
    /// retry is needed. This serves a role closely related to resume_token, as
    /// both fields may be used to provide state required to restore a Batch
    /// Upload, but they differ in two important aspects:
    ///  - it is not compared to previous values, and as such does not provide
    ///    concurrency control;
    ///  - it allows for a larger payload, since the contents are never
    ///    inspected/compared;
    /// The size of the message must be within 1 MiB. Too large requests will be
    /// rejected.
    #[prost(bytes, tag = "6")]
    pub uploader_state: std::vec::Vec<u8>,
    /// The individual upload requests for this batch.
    /// The recommend total size for a batch is 10 MiB. Too large requests may be
    /// rejected.
    /// This field may be empty, allowing this RPC to be used like TouchInvocation.
    #[prost(message, repeated, tag = "5")]
    pub upload_requests: ::std::vec::Vec<UploadRequest>,
}
/// Response for UploadBatch
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadBatchResponse {}
/// The individual upload requests for this batch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadRequest {
    /// The resource ID components that identify the resource being uploaded.
    #[prost(message, optional, tag = "1")]
    pub id: ::std::option::Option<upload_request::Id>,
    /// The operation for the request (e.g. Create(), Update(), etc.)
    #[prost(enumeration = "upload_request::UploadOperation", tag = "2")]
    pub upload_operation: i32,
    /// Required for Update and Merge operations.
    /// Ignored for Create and Finalize operations.
    /// Masks the fields of the resource being uploaded. Provides support for a
    /// more granular upload.
    /// FieldMask must match one of the follow patterns, where * means any single
    /// field name:
    /// Invocation: [*, status_attributes.*, timing.*, invocation_attributes.*,
    /// workspace_info.*].
    /// Target: [*, status_attributes.*, timing.*].
    /// Configuration: [*, status_attributes.*].
    /// ConfiguredTarget: [*, status_attributes.*].
    /// Action: [*, status_attributes.*, timing.*, test_action.test_suite,
    /// test_action.infrastructure_failure_info].
    /// FileSet: [*].
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// If true then the Update, Merge operation will become a Create operation if
    /// the resource is NOT_FOUND. Not supported for Invocation resource.
    #[prost(bool, tag = "10")]
    pub create_if_not_found: bool,
    /// The proto of the resource being uploaded.
    #[prost(oneof = "upload_request::Resource", tags = "4, 5, 6, 7, 8, 9")]
    pub resource: ::std::option::Option<upload_request::Resource>,
}
pub mod upload_request {
    /// The resource ID components that identify the resource being uploaded.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Id {
        /// Required for Target, ConfiguredTarget, or Action.
        /// The Target ID.
        #[prost(string, tag = "1")]
        pub target_id: std::string::String,
        /// Required for Configuration, ConfiguredTarget, or Action.
        /// The Configuration ID.
        #[prost(string, tag = "2")]
        pub configuration_id: std::string::String,
        /// Required for Action.
        /// The Action ID.
        #[prost(string, tag = "3")]
        pub action_id: std::string::String,
        /// Required for FileSet.
        /// The FileSet ID.
        #[prost(string, tag = "4")]
        pub file_set_id: std::string::String,
    }
    /// The operation for the request (e.g. Create(), Update(), etc.)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UploadOperation {
        /// Unspecified
        Unspecified = 0,
        /// Create the given resources.
        /// For more information, check the Create APIs.
        Create = 1,
        /// Applies a standard update to the resource identified by the given
        /// proto's name. For more information, see the Update APIs.
        /// UploadBatch does not support arbitrary field masks. The list of allowed
        /// field masks can be found below.
        Update = 2,
        /// Applies an merge update to the resource identified by the given
        /// proto's name. For more information, see the Merge APIs.
        /// Currently, only the "files" and "file_processing_errors" fields are
        /// supported by this operation.
        Merge = 3,
        /// Declares the resource with the given name as finalized and immutable by
        /// the uploader. Only supported for Invocation, Target, ConfiguredTarget.
        /// There must be no operation on child resources after parent resource is
        /// Finalized. If there is a Finalize of Invocation, it must be the final
        /// UploadRequest. For more information, see the Finalize APIs.
        /// An empty resource should be provided below.
        Finalize = 4,
    }
    /// The proto of the resource being uploaded.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        /// The Invocation Resource
        #[prost(message, tag = "4")]
        Invocation(super::Invocation),
        /// The Target Resource
        #[prost(message, tag = "5")]
        Target(super::Target),
        /// The Configuration Resource
        #[prost(message, tag = "6")]
        Configuration(super::Configuration),
        /// The ConfiguredTarget Resource
        #[prost(message, tag = "7")]
        ConfiguredTarget(super::ConfiguredTarget),
        /// The Action Resource
        #[prost(message, tag = "8")]
        Action(super::Action),
        /// The FileSet Resource
        #[prost(message, tag = "9")]
        FileSet(super::FileSet),
    }
}
/// Request passed into GetInvocationUploadMetadata
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvocationUploadMetadataRequest {
    /// Required
    /// The name of the UploadMetadata being requested.
    /// The name format must be: invocations/${INVOCATION_ID}/uploadMetadata
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. A UUID that must match the value provided in CreateInvocationRequest.
    #[prost(string, tag = "2")]
    pub authorization_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod result_store_upload_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " This is the interface used to upload information to the ResultStore database,"]
    #[doc = " to update that information as necessary, and to make it immutable at the end."]
    #[doc = ""]
    #[doc = " This interface intentionally does not support user read-modify-write"]
    #[doc = " operations. They may corrupt data, and are too expensive. For the same"]
    #[doc = " reason, all upload RPCs will return no resource fields except name and ID. An"]
    #[doc = " uploader should hold as little state as possible in memory to avoid running"]
    #[doc = " out of memory."]
    pub struct ResultStoreUploadClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ResultStoreUploadClient<T>
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
        #[doc = " Creates the given invocation."]
        #[doc = ""]
        #[doc = " This is not an implicitly idempotent API, so a request id is required to"]
        #[doc = " make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Invocation proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If an invocation with the same ID already exists."]
        pub async fn create_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInvocationRequest>,
        ) -> Result<tonic::Response<super::Invocation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a standard update to the invocation identified by the given proto's"]
        #[doc = " name.  For all types of fields (primitive, message, or repeated), replaces"]
        #[doc = " them with the given proto fields if they are under the given field mask"]
        #[doc = " paths.  Fields that match the mask but aren't populated in the given"]
        #[doc = " invocation are cleared. This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty Invocation proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation does not exist."]
        #[doc = " - If the invocation is finalized."]
        #[doc = " - If no field mask was given."]
        pub async fn update_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInvocationRequest>,
        ) -> Result<tonic::Response<super::Invocation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a merge update to the invocation identified by the given proto's"]
        #[doc = " name.  For primitive and message fields, replaces them with the ones in"]
        #[doc = " the given proto if they are covered under the field mask paths.  For"]
        #[doc = " repeated fields, merges to them with the given ones if they are covered"]
        #[doc = " under the field mask paths. This is not an implicitly idempotent API, so a"]
        #[doc = " request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Invocation proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation does not exist."]
        #[doc = " - If the invocation is finalized."]
        #[doc = " - If no field mask was given."]
        pub async fn merge_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeInvocationRequest>,
        ) -> Result<tonic::Response<super::Invocation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/MergeInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Touches the invocation identified by the given proto's name."]
        #[doc = ""]
        #[doc = " This is useful when you need to notify ResultStore that you haven't"]
        #[doc = " abandoned the upload, since abandoned uploads will be automatically"]
        #[doc = " finalized after a set period."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation does not exist."]
        #[doc = " - If the invocation is finalized."]
        pub async fn touch_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::TouchInvocationRequest>,
        ) -> Result<tonic::Response<super::TouchInvocationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/TouchInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Declares the invocation with the given name as finalized and immutable by"]
        #[doc = " the user. It may still be mutated by post-processing. This is an implicitly"]
        #[doc = " idempotent API."]
        #[doc = ""]
        #[doc = " If an Invocation is not updated for 24 hours, some time after that"]
        #[doc = " this will be called automatically."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation does not exist."]
        pub async fn finalize_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeInvocationRequest>,
        ) -> Result<tonic::Response<super::FinalizeInvocationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/FinalizeInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an immutable invocation (permanently)"]
        #[doc = " Note: this does not delete indirect data, e.g. files stored in other"]
        #[doc = " services."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation does not exist."]
        #[doc = " - If the invocation is not finalized.  This can be retried until it is."]
        pub async fn delete_invocation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInvocationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/DeleteInvocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates the given target under the given parent invocation. The given"]
        #[doc = " target ID is URL encoded, converted to the full resource name, and assigned"]
        #[doc = " to the target's name field. This is not an implicitly idempotent API, so a"]
        #[doc = " request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Target proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If no target ID is provided."]
        #[doc = " - If the parent invocation does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        #[doc = " - If a target with the same name already exists."]
        pub async fn create_target(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTargetRequest>,
        ) -> Result<tonic::Response<super::Target>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a standard update to the target identified by the given proto's"]
        #[doc = " name. For all types of fields (primitive, message, or repeated), replaces"]
        #[doc = " them with the given proto fields if they are under the given field mask"]
        #[doc = " paths. Fields that match the mask but aren't populated in the given"]
        #[doc = " target are cleared. This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty Target proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the target does not exist."]
        #[doc = " - If the target or parent invocation is finalized."]
        #[doc = " - If no field mask was given."]
        pub async fn update_target(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTargetRequest>,
        ) -> Result<tonic::Response<super::Target>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a merge update to the target identified by the given proto's"]
        #[doc = " name. For primitive and message fields, replaces them with the ones in the"]
        #[doc = " given proto if they are covered under the field mask paths.  For repeated"]
        #[doc = " fields, merges to them with the given ones if they are covered under the"]
        #[doc = " field mask paths. This is not an implicitly idempotent API, so a request"]
        #[doc = " id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Target proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the target does not exist."]
        #[doc = " - If the target or parent invocation is finalized."]
        #[doc = " - If no field mask was given."]
        pub async fn merge_target(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeTargetRequest>,
        ) -> Result<tonic::Response<super::Target>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/MergeTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Declares the target with the given name as finalized and immutable by the"]
        #[doc = " user. It may still be mutated by post-processing. This is an implicitly"]
        #[doc = " idempotent API."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the target does not exist."]
        pub async fn finalize_target(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeTargetRequest>,
        ) -> Result<tonic::Response<super::FinalizeTargetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/FinalizeTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates the given configured target under the given parent target."]
        #[doc = " The given configured target ID is URL encoded, converted to the full"]
        #[doc = " resource name, and assigned to the configured target's name field."]
        #[doc = " This is not an implicitly idempotent API, so a request id is required"]
        #[doc = " to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty ConfiguredTarget proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If no config ID is provided."]
        #[doc = " - If a configured target with the same ID already exists."]
        #[doc = " - If the parent target does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        pub async fn create_configured_target(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConfiguredTargetRequest>,
        ) -> Result<tonic::Response<super::ConfiguredTarget>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateConfiguredTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a standard update to the configured target identified by the given"]
        #[doc = " proto's name. For all types of fields (primitive, message, or repeated),"]
        #[doc = " replaces them with the given proto fields if they are under the given"]
        #[doc = " field mask paths. Fields that match the mask but aren't populated in the"]
        #[doc = " given configured target are cleared. This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty ConfiguredTarget proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configured target does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        #[doc = " - If no field mask was given."]
        pub async fn update_configured_target(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConfiguredTargetRequest>,
        ) -> Result<tonic::Response<super::ConfiguredTarget>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateConfiguredTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a merge update to the configured target identified by the given"]
        #[doc = " proto's name. For primitive and message fields, replaces them with the"]
        #[doc = " ones in the given proto if they are covered under the field mask paths."]
        #[doc = " For repeated fields, merges to them with the given ones if they are"]
        #[doc = " covered under the field mask paths. This is not an implicitly idempotent"]
        #[doc = " API, so a request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty ConfiguredTarget proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configured target does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        #[doc = " - If no field mask was given."]
        pub async fn merge_configured_target(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeConfiguredTargetRequest>,
        ) -> Result<tonic::Response<super::ConfiguredTarget>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/MergeConfiguredTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Declares the configured target with the given name as finalized and"]
        #[doc = " immutable by the user. It may still be mutated by post-processing. This is"]
        #[doc = " an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configured target does not exist."]
        pub async fn finalize_configured_target(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeConfiguredTargetRequest>,
        ) -> Result<tonic::Response<super::FinalizeConfiguredTargetResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/FinalizeConfiguredTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates the given action under the given configured target. The given"]
        #[doc = " action ID is URL encoded, converted to the full resource name, and"]
        #[doc = " assigned to the action's name field. This is not an implicitly"]
        #[doc = " idempotent API, so a request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Action proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If no action ID provided."]
        #[doc = " - If the parent configured target does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        #[doc = " - If an action  with the same name already exists."]
        pub async fn create_action(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateActionRequest>,
        ) -> Result<tonic::Response<super::Action>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateAction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a standard update to the action identified by the given"]
        #[doc = " proto's name.  For all types of fields (primitive, message, or repeated),"]
        #[doc = " replaces them with the given proto fields if they are under the given"]
        #[doc = " field mask paths.  Fields that match the mask but aren't populated in the"]
        #[doc = " given action are cleared.  This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty Action proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the action does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        #[doc = " - If no field mask was given."]
        pub async fn update_action(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateActionRequest>,
        ) -> Result<tonic::Response<super::Action>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateAction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a merge update to the action identified by the given"]
        #[doc = " proto's name.  For primitive and message fields, replaces them with the"]
        #[doc = " ones in the given proto if they are covered under the field mask paths."]
        #[doc = " For repeated fields, merges to them with the given ones if they are"]
        #[doc = " covered under the field mask paths. This is not an implicitly idempotent"]
        #[doc = " API, so a request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Action proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the action does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        #[doc = " - If no field mask was given."]
        pub async fn merge_action(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeActionRequest>,
        ) -> Result<tonic::Response<super::Action>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/MergeAction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates the given configuration under the given parent invocation. The"]
        #[doc = " given configuration ID is URL encoded, converted to the full resource name,"]
        #[doc = " and assigned to the configuration's name field. The configuration ID of"]
        #[doc = " \"default\" should be preferred for the default configuration in a"]
        #[doc = " single-config invocation. This is not an implicitly idempotent API, so a"]
        #[doc = " request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Configuration proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If no configuration ID is provided."]
        #[doc = " - If the parent invocation does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        #[doc = " - If a configuration with the same name already exists."]
        pub async fn create_configuration(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConfigurationRequest>,
        ) -> Result<tonic::Response<super::Configuration>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateConfiguration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a standard update to the configuration identified by the given"]
        #[doc = " proto's name. For all types of fields (primitive, message, or repeated),"]
        #[doc = " replaces them with the given proto fields if they are under the given field"]
        #[doc = " mask paths. Fields that match the mask but aren't populated in the given"]
        #[doc = " configuration are cleared. This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty Configuration proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configuration does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        #[doc = " - If no field mask was given."]
        #[doc = " - If a given field mask path is not valid."]
        pub async fn update_configuration(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConfigurationRequest>,
        ) -> Result<tonic::Response<super::Configuration>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateConfiguration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates the given file set under the given parent invocation. The given"]
        #[doc = " file set ID is URL encoded, converted to the full resource name, and"]
        #[doc = " assigned to the file set's name field. This is not an implicitly idempotent"]
        #[doc = " API, so a request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty FileSet proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If no file set ID is provided."]
        #[doc = " - If a file set with the same name already exists."]
        #[doc = " - If the parent invocation does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        pub async fn create_file_set(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFileSetRequest>,
        ) -> Result<tonic::Response<super::FileSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateFileSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a standard update to the file set identified by the given proto's"]
        #[doc = " name. For all types of fields (primitive, message, or repeated), replaces"]
        #[doc = " them with the given proto fields if they are under the given field mask"]
        #[doc = " paths. Fields that match the mask but aren't populated in the given"]
        #[doc = " configuration are cleared. This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty FileSet proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the file set does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        #[doc = " - If no field mask was given."]
        #[doc = " - If a given field mask path is not valid."]
        pub async fn update_file_set(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFileSetRequest>,
        ) -> Result<tonic::Response<super::FileSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateFileSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies a merge update to the file set identified by the given proto's"]
        #[doc = " name. For primitive and message fields, updates them with the ones in the"]
        #[doc = " given proto if they are covered under the field mask paths. For repeated"]
        #[doc = " fields, merges to them with the given ones if they are covered under the"]
        #[doc = " field mask paths. This is not an implicitly idempotent API, so a request"]
        #[doc = " id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty FileSet proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the file set does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        #[doc = " - If a given field mask path is not valid."]
        #[doc = " - If no field mask was given."]
        pub async fn merge_file_set(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeFileSetRequest>,
        ) -> Result<tonic::Response<super::FileSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/MergeFileSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This is the RPC used for batch upload. It supports uploading multiple"]
        #[doc = " resources for an invocation in a transaction safe manner."]
        #[doc = ""]
        #[doc = " To use this RPC, the CreateInvocationRequest must have been provided a"]
        #[doc = " resume_token."]
        #[doc = ""]
        #[doc = " Combining batch upload with normal upload on a single Invocation is not"]
        #[doc = " supported. If an Invocation is created with a resume_token, all further"]
        #[doc = " calls must be through UploadBatch. If an Invocation is created without"]
        #[doc = " resume_token normal upload, all further upload calls must be through normal"]
        #[doc = " upload RPCs."]
        pub async fn upload_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadBatchRequest>,
        ) -> Result<tonic::Response<super::UploadBatchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/UploadBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Provides a way to read the metadata for an invocation."]
        #[doc = " The UploadMetadata could still be retrieved by this RPC even the Invocation"]
        #[doc = " has been finalized."]
        #[doc = " This API requires setting a response FieldMask via 'fields' URL query"]
        #[doc = " parameter or X-Goog-FieldMask HTTP/gRPC header."]
        #[doc = ""]
        #[doc = " An error will be reported in the following case:"]
        #[doc = " - If the invocation does not exist."]
        #[doc = " - If no field mask was given."]
        pub async fn get_invocation_upload_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInvocationUploadMetadataRequest>,
        ) -> Result<tonic::Response<super::UploadMetadata>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.resultstore.v2.ResultStoreUpload/GetInvocationUploadMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ResultStoreUploadClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ResultStoreUploadClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ResultStoreUploadClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod result_store_upload_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ResultStoreUploadServer."]
    #[async_trait]
    pub trait ResultStoreUpload: Send + Sync + 'static {
        #[doc = " Creates the given invocation."]
        #[doc = ""]
        #[doc = " This is not an implicitly idempotent API, so a request id is required to"]
        #[doc = " make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Invocation proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If an invocation with the same ID already exists."]
        async fn create_invocation(
            &self,
            request: tonic::Request<super::CreateInvocationRequest>,
        ) -> Result<tonic::Response<super::Invocation>, tonic::Status>;
        #[doc = " Applies a standard update to the invocation identified by the given proto's"]
        #[doc = " name.  For all types of fields (primitive, message, or repeated), replaces"]
        #[doc = " them with the given proto fields if they are under the given field mask"]
        #[doc = " paths.  Fields that match the mask but aren't populated in the given"]
        #[doc = " invocation are cleared. This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty Invocation proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation does not exist."]
        #[doc = " - If the invocation is finalized."]
        #[doc = " - If no field mask was given."]
        async fn update_invocation(
            &self,
            request: tonic::Request<super::UpdateInvocationRequest>,
        ) -> Result<tonic::Response<super::Invocation>, tonic::Status>;
        #[doc = " Applies a merge update to the invocation identified by the given proto's"]
        #[doc = " name.  For primitive and message fields, replaces them with the ones in"]
        #[doc = " the given proto if they are covered under the field mask paths.  For"]
        #[doc = " repeated fields, merges to them with the given ones if they are covered"]
        #[doc = " under the field mask paths. This is not an implicitly idempotent API, so a"]
        #[doc = " request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Invocation proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation does not exist."]
        #[doc = " - If the invocation is finalized."]
        #[doc = " - If no field mask was given."]
        async fn merge_invocation(
            &self,
            request: tonic::Request<super::MergeInvocationRequest>,
        ) -> Result<tonic::Response<super::Invocation>, tonic::Status>;
        #[doc = " Touches the invocation identified by the given proto's name."]
        #[doc = ""]
        #[doc = " This is useful when you need to notify ResultStore that you haven't"]
        #[doc = " abandoned the upload, since abandoned uploads will be automatically"]
        #[doc = " finalized after a set period."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation does not exist."]
        #[doc = " - If the invocation is finalized."]
        async fn touch_invocation(
            &self,
            request: tonic::Request<super::TouchInvocationRequest>,
        ) -> Result<tonic::Response<super::TouchInvocationResponse>, tonic::Status>;
        #[doc = " Declares the invocation with the given name as finalized and immutable by"]
        #[doc = " the user. It may still be mutated by post-processing. This is an implicitly"]
        #[doc = " idempotent API."]
        #[doc = ""]
        #[doc = " If an Invocation is not updated for 24 hours, some time after that"]
        #[doc = " this will be called automatically."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation does not exist."]
        async fn finalize_invocation(
            &self,
            request: tonic::Request<super::FinalizeInvocationRequest>,
        ) -> Result<tonic::Response<super::FinalizeInvocationResponse>, tonic::Status>;
        #[doc = " Deletes an immutable invocation (permanently)"]
        #[doc = " Note: this does not delete indirect data, e.g. files stored in other"]
        #[doc = " services."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the invocation does not exist."]
        #[doc = " - If the invocation is not finalized.  This can be retried until it is."]
        async fn delete_invocation(
            &self,
            request: tonic::Request<super::DeleteInvocationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Creates the given target under the given parent invocation. The given"]
        #[doc = " target ID is URL encoded, converted to the full resource name, and assigned"]
        #[doc = " to the target's name field. This is not an implicitly idempotent API, so a"]
        #[doc = " request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Target proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If no target ID is provided."]
        #[doc = " - If the parent invocation does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        #[doc = " - If a target with the same name already exists."]
        async fn create_target(
            &self,
            request: tonic::Request<super::CreateTargetRequest>,
        ) -> Result<tonic::Response<super::Target>, tonic::Status>;
        #[doc = " Applies a standard update to the target identified by the given proto's"]
        #[doc = " name. For all types of fields (primitive, message, or repeated), replaces"]
        #[doc = " them with the given proto fields if they are under the given field mask"]
        #[doc = " paths. Fields that match the mask but aren't populated in the given"]
        #[doc = " target are cleared. This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty Target proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the target does not exist."]
        #[doc = " - If the target or parent invocation is finalized."]
        #[doc = " - If no field mask was given."]
        async fn update_target(
            &self,
            request: tonic::Request<super::UpdateTargetRequest>,
        ) -> Result<tonic::Response<super::Target>, tonic::Status>;
        #[doc = " Applies a merge update to the target identified by the given proto's"]
        #[doc = " name. For primitive and message fields, replaces them with the ones in the"]
        #[doc = " given proto if they are covered under the field mask paths.  For repeated"]
        #[doc = " fields, merges to them with the given ones if they are covered under the"]
        #[doc = " field mask paths. This is not an implicitly idempotent API, so a request"]
        #[doc = " id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Target proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the target does not exist."]
        #[doc = " - If the target or parent invocation is finalized."]
        #[doc = " - If no field mask was given."]
        async fn merge_target(
            &self,
            request: tonic::Request<super::MergeTargetRequest>,
        ) -> Result<tonic::Response<super::Target>, tonic::Status>;
        #[doc = " Declares the target with the given name as finalized and immutable by the"]
        #[doc = " user. It may still be mutated by post-processing. This is an implicitly"]
        #[doc = " idempotent API."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the target does not exist."]
        async fn finalize_target(
            &self,
            request: tonic::Request<super::FinalizeTargetRequest>,
        ) -> Result<tonic::Response<super::FinalizeTargetResponse>, tonic::Status>;
        #[doc = " Creates the given configured target under the given parent target."]
        #[doc = " The given configured target ID is URL encoded, converted to the full"]
        #[doc = " resource name, and assigned to the configured target's name field."]
        #[doc = " This is not an implicitly idempotent API, so a request id is required"]
        #[doc = " to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty ConfiguredTarget proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If no config ID is provided."]
        #[doc = " - If a configured target with the same ID already exists."]
        #[doc = " - If the parent target does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        async fn create_configured_target(
            &self,
            request: tonic::Request<super::CreateConfiguredTargetRequest>,
        ) -> Result<tonic::Response<super::ConfiguredTarget>, tonic::Status>;
        #[doc = " Applies a standard update to the configured target identified by the given"]
        #[doc = " proto's name. For all types of fields (primitive, message, or repeated),"]
        #[doc = " replaces them with the given proto fields if they are under the given"]
        #[doc = " field mask paths. Fields that match the mask but aren't populated in the"]
        #[doc = " given configured target are cleared. This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty ConfiguredTarget proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configured target does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        #[doc = " - If no field mask was given."]
        async fn update_configured_target(
            &self,
            request: tonic::Request<super::UpdateConfiguredTargetRequest>,
        ) -> Result<tonic::Response<super::ConfiguredTarget>, tonic::Status>;
        #[doc = " Applies a merge update to the configured target identified by the given"]
        #[doc = " proto's name. For primitive and message fields, replaces them with the"]
        #[doc = " ones in the given proto if they are covered under the field mask paths."]
        #[doc = " For repeated fields, merges to them with the given ones if they are"]
        #[doc = " covered under the field mask paths. This is not an implicitly idempotent"]
        #[doc = " API, so a request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty ConfiguredTarget proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configured target does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        #[doc = " - If no field mask was given."]
        async fn merge_configured_target(
            &self,
            request: tonic::Request<super::MergeConfiguredTargetRequest>,
        ) -> Result<tonic::Response<super::ConfiguredTarget>, tonic::Status>;
        #[doc = " Declares the configured target with the given name as finalized and"]
        #[doc = " immutable by the user. It may still be mutated by post-processing. This is"]
        #[doc = " an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configured target does not exist."]
        async fn finalize_configured_target(
            &self,
            request: tonic::Request<super::FinalizeConfiguredTargetRequest>,
        ) -> Result<tonic::Response<super::FinalizeConfiguredTargetResponse>, tonic::Status>;
        #[doc = " Creates the given action under the given configured target. The given"]
        #[doc = " action ID is URL encoded, converted to the full resource name, and"]
        #[doc = " assigned to the action's name field. This is not an implicitly"]
        #[doc = " idempotent API, so a request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Action proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If no action ID provided."]
        #[doc = " - If the parent configured target does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        #[doc = " - If an action  with the same name already exists."]
        async fn create_action(
            &self,
            request: tonic::Request<super::CreateActionRequest>,
        ) -> Result<tonic::Response<super::Action>, tonic::Status>;
        #[doc = " Applies a standard update to the action identified by the given"]
        #[doc = " proto's name.  For all types of fields (primitive, message, or repeated),"]
        #[doc = " replaces them with the given proto fields if they are under the given"]
        #[doc = " field mask paths.  Fields that match the mask but aren't populated in the"]
        #[doc = " given action are cleared.  This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty Action proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the action does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        #[doc = " - If no field mask was given."]
        async fn update_action(
            &self,
            request: tonic::Request<super::UpdateActionRequest>,
        ) -> Result<tonic::Response<super::Action>, tonic::Status>;
        #[doc = " Applies a merge update to the action identified by the given"]
        #[doc = " proto's name.  For primitive and message fields, replaces them with the"]
        #[doc = " ones in the given proto if they are covered under the field mask paths."]
        #[doc = " For repeated fields, merges to them with the given ones if they are"]
        #[doc = " covered under the field mask paths. This is not an implicitly idempotent"]
        #[doc = " API, so a request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Action proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the action does not exist."]
        #[doc = " - If the parent target or invocation is finalized."]
        #[doc = " - If no field mask was given."]
        async fn merge_action(
            &self,
            request: tonic::Request<super::MergeActionRequest>,
        ) -> Result<tonic::Response<super::Action>, tonic::Status>;
        #[doc = " Creates the given configuration under the given parent invocation. The"]
        #[doc = " given configuration ID is URL encoded, converted to the full resource name,"]
        #[doc = " and assigned to the configuration's name field. The configuration ID of"]
        #[doc = " \"default\" should be preferred for the default configuration in a"]
        #[doc = " single-config invocation. This is not an implicitly idempotent API, so a"]
        #[doc = " request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty Configuration proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If no configuration ID is provided."]
        #[doc = " - If the parent invocation does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        #[doc = " - If a configuration with the same name already exists."]
        async fn create_configuration(
            &self,
            request: tonic::Request<super::CreateConfigurationRequest>,
        ) -> Result<tonic::Response<super::Configuration>, tonic::Status>;
        #[doc = " Applies a standard update to the configuration identified by the given"]
        #[doc = " proto's name. For all types of fields (primitive, message, or repeated),"]
        #[doc = " replaces them with the given proto fields if they are under the given field"]
        #[doc = " mask paths. Fields that match the mask but aren't populated in the given"]
        #[doc = " configuration are cleared. This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty Configuration proto with only the name and ID fields"]
        #[doc = " populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the configuration does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        #[doc = " - If no field mask was given."]
        #[doc = " - If a given field mask path is not valid."]
        async fn update_configuration(
            &self,
            request: tonic::Request<super::UpdateConfigurationRequest>,
        ) -> Result<tonic::Response<super::Configuration>, tonic::Status>;
        #[doc = " Creates the given file set under the given parent invocation. The given"]
        #[doc = " file set ID is URL encoded, converted to the full resource name, and"]
        #[doc = " assigned to the file set's name field. This is not an implicitly idempotent"]
        #[doc = " API, so a request id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty FileSet proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If no file set ID is provided."]
        #[doc = " - If a file set with the same name already exists."]
        #[doc = " - If the parent invocation does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        async fn create_file_set(
            &self,
            request: tonic::Request<super::CreateFileSetRequest>,
        ) -> Result<tonic::Response<super::FileSet>, tonic::Status>;
        #[doc = " Applies a standard update to the file set identified by the given proto's"]
        #[doc = " name. For all types of fields (primitive, message, or repeated), replaces"]
        #[doc = " them with the given proto fields if they are under the given field mask"]
        #[doc = " paths. Fields that match the mask but aren't populated in the given"]
        #[doc = " configuration are cleared. This is an implicitly idempotent API."]
        #[doc = ""]
        #[doc = " Returns an empty FileSet proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the file set does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        #[doc = " - If no field mask was given."]
        #[doc = " - If a given field mask path is not valid."]
        async fn update_file_set(
            &self,
            request: tonic::Request<super::UpdateFileSetRequest>,
        ) -> Result<tonic::Response<super::FileSet>, tonic::Status>;
        #[doc = " Applies a merge update to the file set identified by the given proto's"]
        #[doc = " name. For primitive and message fields, updates them with the ones in the"]
        #[doc = " given proto if they are covered under the field mask paths. For repeated"]
        #[doc = " fields, merges to them with the given ones if they are covered under the"]
        #[doc = " field mask paths. This is not an implicitly idempotent API, so a request"]
        #[doc = " id is required to make it idempotent."]
        #[doc = ""]
        #[doc = " Returns an empty FileSet proto with only the name and ID fields populated."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " An error will be reported in the following cases:"]
        #[doc = " - If the file set does not exist."]
        #[doc = " - If the parent invocation is finalized."]
        #[doc = " - If a given field mask path is not valid."]
        #[doc = " - If no field mask was given."]
        async fn merge_file_set(
            &self,
            request: tonic::Request<super::MergeFileSetRequest>,
        ) -> Result<tonic::Response<super::FileSet>, tonic::Status>;
        #[doc = " This is the RPC used for batch upload. It supports uploading multiple"]
        #[doc = " resources for an invocation in a transaction safe manner."]
        #[doc = ""]
        #[doc = " To use this RPC, the CreateInvocationRequest must have been provided a"]
        #[doc = " resume_token."]
        #[doc = ""]
        #[doc = " Combining batch upload with normal upload on a single Invocation is not"]
        #[doc = " supported. If an Invocation is created with a resume_token, all further"]
        #[doc = " calls must be through UploadBatch. If an Invocation is created without"]
        #[doc = " resume_token normal upload, all further upload calls must be through normal"]
        #[doc = " upload RPCs."]
        async fn upload_batch(
            &self,
            request: tonic::Request<super::UploadBatchRequest>,
        ) -> Result<tonic::Response<super::UploadBatchResponse>, tonic::Status>;
        #[doc = " Provides a way to read the metadata for an invocation."]
        #[doc = " The UploadMetadata could still be retrieved by this RPC even the Invocation"]
        #[doc = " has been finalized."]
        #[doc = " This API requires setting a response FieldMask via 'fields' URL query"]
        #[doc = " parameter or X-Goog-FieldMask HTTP/gRPC header."]
        #[doc = ""]
        #[doc = " An error will be reported in the following case:"]
        #[doc = " - If the invocation does not exist."]
        #[doc = " - If no field mask was given."]
        async fn get_invocation_upload_metadata(
            &self,
            request: tonic::Request<super::GetInvocationUploadMetadataRequest>,
        ) -> Result<tonic::Response<super::UploadMetadata>, tonic::Status>;
    }
    #[doc = " This is the interface used to upload information to the ResultStore database,"]
    #[doc = " to update that information as necessary, and to make it immutable at the end."]
    #[doc = ""]
    #[doc = " This interface intentionally does not support user read-modify-write"]
    #[doc = " operations. They may corrupt data, and are too expensive. For the same"]
    #[doc = " reason, all upload RPCs will return no resource fields except name and ID. An"]
    #[doc = " uploader should hold as little state as possible in memory to avoid running"]
    #[doc = " out of memory."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ResultStoreUploadServer<T: ResultStoreUpload> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ResultStoreUpload> ResultStoreUploadServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ResultStoreUploadServer<T>
    where
        T: ResultStoreUpload,
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateInvocation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInvocationSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::CreateInvocationRequest>
                        for CreateInvocationSvc<T>
                    {
                        type Response = super::Invocation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateInvocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_invocation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateInvocationSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateInvocation" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateInvocationSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::UpdateInvocationRequest>
                        for UpdateInvocationSvc<T>
                    {
                        type Response = super::Invocation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateInvocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_invocation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateInvocationSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/MergeInvocation" => {
                    #[allow(non_camel_case_types)]
                    struct MergeInvocationSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::MergeInvocationRequest>
                        for MergeInvocationSvc<T>
                    {
                        type Response = super::Invocation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MergeInvocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.merge_invocation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MergeInvocationSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/TouchInvocation" => {
                    #[allow(non_camel_case_types)]
                    struct TouchInvocationSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::TouchInvocationRequest>
                        for TouchInvocationSvc<T>
                    {
                        type Response = super::TouchInvocationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TouchInvocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.touch_invocation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TouchInvocationSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/FinalizeInvocation" => {
                    #[allow(non_camel_case_types)]
                    struct FinalizeInvocationSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::FinalizeInvocationRequest>
                        for FinalizeInvocationSvc<T>
                    {
                        type Response = super::FinalizeInvocationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinalizeInvocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.finalize_invocation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FinalizeInvocationSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/DeleteInvocation" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteInvocationSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::DeleteInvocationRequest>
                        for DeleteInvocationSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteInvocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_invocation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteInvocationSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateTarget" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTargetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::CreateTargetRequest>
                        for CreateTargetSvc<T>
                    {
                        type Response = super::Target;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTargetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_target(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateTargetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateTarget" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTargetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::UpdateTargetRequest>
                        for UpdateTargetSvc<T>
                    {
                        type Response = super::Target;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTargetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_target(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateTargetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/MergeTarget" => {
                    #[allow(non_camel_case_types)]
                    struct MergeTargetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::MergeTargetRequest>
                        for MergeTargetSvc<T>
                    {
                        type Response = super::Target;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MergeTargetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.merge_target(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MergeTargetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/FinalizeTarget" => {
                    #[allow(non_camel_case_types)]
                    struct FinalizeTargetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::FinalizeTargetRequest>
                        for FinalizeTargetSvc<T>
                    {
                        type Response = super::FinalizeTargetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinalizeTargetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.finalize_target(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FinalizeTargetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateConfiguredTarget" => {
                    #[allow(non_camel_case_types)]
                    struct CreateConfiguredTargetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::CreateConfiguredTargetRequest>
                        for CreateConfiguredTargetSvc<T>
                    {
                        type Response = super::ConfiguredTarget;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateConfiguredTargetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_configured_target(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateConfiguredTargetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateConfiguredTarget" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateConfiguredTargetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::UpdateConfiguredTargetRequest>
                        for UpdateConfiguredTargetSvc<T>
                    {
                        type Response = super::ConfiguredTarget;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateConfiguredTargetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_configured_target(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateConfiguredTargetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/MergeConfiguredTarget" => {
                    #[allow(non_camel_case_types)]
                    struct MergeConfiguredTargetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::MergeConfiguredTargetRequest>
                        for MergeConfiguredTargetSvc<T>
                    {
                        type Response = super::ConfiguredTarget;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MergeConfiguredTargetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.merge_configured_target(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MergeConfiguredTargetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/FinalizeConfiguredTarget" => {
                    #[allow(non_camel_case_types)]
                    struct FinalizeConfiguredTargetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::FinalizeConfiguredTargetRequest>
                        for FinalizeConfiguredTargetSvc<T>
                    {
                        type Response = super::FinalizeConfiguredTargetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FinalizeConfiguredTargetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.finalize_configured_target(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FinalizeConfiguredTargetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateAction" => {
                    #[allow(non_camel_case_types)]
                    struct CreateActionSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::CreateActionRequest>
                        for CreateActionSvc<T>
                    {
                        type Response = super::Action;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateActionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_action(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateActionSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateAction" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateActionSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::UpdateActionRequest>
                        for UpdateActionSvc<T>
                    {
                        type Response = super::Action;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateActionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_action(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateActionSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/MergeAction" => {
                    #[allow(non_camel_case_types)]
                    struct MergeActionSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::MergeActionRequest>
                        for MergeActionSvc<T>
                    {
                        type Response = super::Action;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MergeActionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.merge_action(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MergeActionSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateConfiguration" => {
                    #[allow(non_camel_case_types)]
                    struct CreateConfigurationSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::CreateConfigurationRequest>
                        for CreateConfigurationSvc<T>
                    {
                        type Response = super::Configuration;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateConfigurationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_configuration(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateConfigurationSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateConfiguration" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateConfigurationSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::UpdateConfigurationRequest>
                        for UpdateConfigurationSvc<T>
                    {
                        type Response = super::Configuration;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateConfigurationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_configuration(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateConfigurationSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/CreateFileSet" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFileSetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::CreateFileSetRequest>
                        for CreateFileSetSvc<T>
                    {
                        type Response = super::FileSet;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFileSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_file_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateFileSetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/UpdateFileSet" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFileSetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::UpdateFileSetRequest>
                        for UpdateFileSetSvc<T>
                    {
                        type Response = super::FileSet;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateFileSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_file_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateFileSetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/MergeFileSet" => {
                    #[allow(non_camel_case_types)]
                    struct MergeFileSetSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::MergeFileSetRequest>
                        for MergeFileSetSvc<T>
                    {
                        type Response = super::FileSet;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MergeFileSetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.merge_file_set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MergeFileSetSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/UploadBatch" => {
                    #[allow(non_camel_case_types)]
                    struct UploadBatchSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::UploadBatchRequest>
                        for UploadBatchSvc<T>
                    {
                        type Response = super::UploadBatchResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UploadBatchRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.upload_batch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UploadBatchSvc(inner);
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
                "/google.devtools.resultstore.v2.ResultStoreUpload/GetInvocationUploadMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct GetInvocationUploadMetadataSvc<T: ResultStoreUpload>(pub Arc<T>);
                    impl<T: ResultStoreUpload>
                        tonic::server::UnaryService<super::GetInvocationUploadMetadataRequest>
                        for GetInvocationUploadMetadataSvc<T>
                    {
                        type Response = super::UploadMetadata;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInvocationUploadMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.get_invocation_upload_metadata(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetInvocationUploadMetadataSvc(inner);
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
    impl<T: ResultStoreUpload> Clone for ResultStoreUploadServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ResultStoreUpload> Clone for _Inner<T> {
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
