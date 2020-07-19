/// Specifies the complete (requested) contents of a single row of a table.
/// Rows which exceed 256MiB in size cannot be read in full.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    /// The unique key which identifies this row within its table. This is the same
    /// key that's used to identify the row in, for example, a MutateRowRequest.
    /// May contain any non-empty byte string up to 4KiB in length.
    #[prost(bytes, tag = "1")]
    pub key: std::vec::Vec<u8>,
    /// May be empty, but only if the entire row is empty.
    /// The mutual ordering of column families is not specified.
    #[prost(message, repeated, tag = "2")]
    pub families: ::std::vec::Vec<Family>,
}
/// Specifies (some of) the contents of a single row/column family of a table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Family {
    /// The unique key which identifies this family within its row. This is the
    /// same key that's used to identify the family in, for example, a RowFilter
    /// which sets its "family_name_regex_filter" field.
    /// Must match [-_.a-zA-Z0-9]+, except that AggregatingRowProcessors may
    /// produce cells in a sentinel family with an empty name.
    /// Must be no greater than 64 characters in length.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Must not be empty. Sorted in order of increasing "qualifier".
    #[prost(message, repeated, tag = "2")]
    pub columns: ::std::vec::Vec<Column>,
}
/// Specifies (some of) the contents of a single row/column of a table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Column {
    /// The unique key which identifies this column within its family. This is the
    /// same key that's used to identify the column in, for example, a RowFilter
    /// which sets its "column_qualifier_regex_filter" field.
    /// May contain any byte string, including the empty string, up to 16kiB in
    /// length.
    #[prost(bytes, tag = "1")]
    pub qualifier: std::vec::Vec<u8>,
    /// Must not be empty. Sorted in order of decreasing "timestamp_micros".
    #[prost(message, repeated, tag = "2")]
    pub cells: ::std::vec::Vec<Cell>,
}
/// Specifies (some of) the contents of a single row/column/timestamp of a table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cell {
    /// The cell's stored timestamp, which also uniquely identifies it within
    /// its column.
    /// Values are always expressed in microseconds, but individual tables may set
    /// a coarser "granularity" to further restrict the allowed values. For
    /// example, a table which specifies millisecond granularity will only allow
    /// values of "timestamp_micros" which are multiples of 1000.
    #[prost(int64, tag = "1")]
    pub timestamp_micros: i64,
    /// The value stored in the cell.
    /// May contain any byte string, including the empty string, up to 100MiB in
    /// length.
    #[prost(bytes, tag = "2")]
    pub value: std::vec::Vec<u8>,
    /// Labels applied to the cell by a [RowFilter][google.bigtable.v1.RowFilter].
    #[prost(string, repeated, tag = "3")]
    pub labels: ::std::vec::Vec<std::string::String>,
}
/// Specifies a contiguous range of rows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowRange {
    /// Inclusive lower bound. If left empty, interpreted as the empty string.
    #[prost(bytes, tag = "2")]
    pub start_key: std::vec::Vec<u8>,
    /// Exclusive upper bound. If left empty, interpreted as infinity.
    #[prost(bytes, tag = "3")]
    pub end_key: std::vec::Vec<u8>,
}
/// Specifies a non-contiguous set of rows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowSet {
    /// Single rows included in the set.
    #[prost(bytes, repeated, tag = "1")]
    pub row_keys: ::std::vec::Vec<std::vec::Vec<u8>>,
    /// Contiguous row ranges included in the set.
    #[prost(message, repeated, tag = "2")]
    pub row_ranges: ::std::vec::Vec<RowRange>,
}
/// Specifies a contiguous range of columns within a single column family.
/// The range spans from <column_family>:<start_qualifier> to
/// <column_family>:<end_qualifier>, where both bounds can be either inclusive or
/// exclusive.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnRange {
    /// The name of the column family within which this range falls.
    #[prost(string, tag = "1")]
    pub family_name: std::string::String,
    /// The column qualifier at which to start the range (within 'column_family').
    /// If neither field is set, interpreted as the empty string, inclusive.
    #[prost(oneof = "column_range::StartQualifier", tags = "2, 3")]
    pub start_qualifier: ::std::option::Option<column_range::StartQualifier>,
    /// The column qualifier at which to end the range (within 'column_family').
    /// If neither field is set, interpreted as the infinite string, exclusive.
    #[prost(oneof = "column_range::EndQualifier", tags = "4, 5")]
    pub end_qualifier: ::std::option::Option<column_range::EndQualifier>,
}
pub mod column_range {
    /// The column qualifier at which to start the range (within 'column_family').
    /// If neither field is set, interpreted as the empty string, inclusive.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StartQualifier {
        /// Used when giving an inclusive lower bound for the range.
        #[prost(bytes, tag = "2")]
        StartQualifierInclusive(std::vec::Vec<u8>),
        /// Used when giving an exclusive lower bound for the range.
        #[prost(bytes, tag = "3")]
        StartQualifierExclusive(std::vec::Vec<u8>),
    }
    /// The column qualifier at which to end the range (within 'column_family').
    /// If neither field is set, interpreted as the infinite string, exclusive.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EndQualifier {
        /// Used when giving an inclusive upper bound for the range.
        #[prost(bytes, tag = "4")]
        EndQualifierInclusive(std::vec::Vec<u8>),
        /// Used when giving an exclusive upper bound for the range.
        #[prost(bytes, tag = "5")]
        EndQualifierExclusive(std::vec::Vec<u8>),
    }
}
/// Specified a contiguous range of microsecond timestamps.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampRange {
    /// Inclusive lower bound. If left empty, interpreted as 0.
    #[prost(int64, tag = "1")]
    pub start_timestamp_micros: i64,
    /// Exclusive upper bound. If left empty, interpreted as infinity.
    #[prost(int64, tag = "2")]
    pub end_timestamp_micros: i64,
}
/// Specifies a contiguous range of raw byte values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRange {
    /// The value at which to start the range.
    /// If neither field is set, interpreted as the empty string, inclusive.
    #[prost(oneof = "value_range::StartValue", tags = "1, 2")]
    pub start_value: ::std::option::Option<value_range::StartValue>,
    /// The value at which to end the range.
    /// If neither field is set, interpreted as the infinite string, exclusive.
    #[prost(oneof = "value_range::EndValue", tags = "3, 4")]
    pub end_value: ::std::option::Option<value_range::EndValue>,
}
pub mod value_range {
    /// The value at which to start the range.
    /// If neither field is set, interpreted as the empty string, inclusive.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum StartValue {
        /// Used when giving an inclusive lower bound for the range.
        #[prost(bytes, tag = "1")]
        StartValueInclusive(std::vec::Vec<u8>),
        /// Used when giving an exclusive lower bound for the range.
        #[prost(bytes, tag = "2")]
        StartValueExclusive(std::vec::Vec<u8>),
    }
    /// The value at which to end the range.
    /// If neither field is set, interpreted as the infinite string, exclusive.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EndValue {
        /// Used when giving an inclusive upper bound for the range.
        #[prost(bytes, tag = "3")]
        EndValueInclusive(std::vec::Vec<u8>),
        /// Used when giving an exclusive upper bound for the range.
        #[prost(bytes, tag = "4")]
        EndValueExclusive(std::vec::Vec<u8>),
    }
}
/// Takes a row as input and produces an alternate view of the row based on
/// specified rules. For example, a RowFilter might trim down a row to include
/// just the cells from columns matching a given regular expression, or might
/// return all the cells of a row but not their values. More complicated filters
/// can be composed out of these components to express requests such as, "within
/// every column of a particular family, give just the two most recent cells
/// which are older than timestamp X."
///
/// There are two broad categories of RowFilters (true filters and transformers),
/// as well as two ways to compose simple filters into more complex ones
/// (chains and interleaves). They work as follows:
///
/// * True filters alter the input row by excluding some of its cells wholesale
/// from the output row. An example of a true filter is the "value_regex_filter",
/// which excludes cells whose values don't match the specified pattern. All
/// regex true filters use RE2 syntax (https://github.com/google/re2/wiki/Syntax)
/// in raw byte mode (RE2::Latin1), and are evaluated as full matches. An
/// important point to keep in mind is that RE2(.) is equivalent by default to
/// RE2([^\n]), meaning that it does not match newlines. When attempting to match
/// an arbitrary byte, you should therefore use the escape sequence '\C', which
/// may need to be further escaped as '\\C' in your client language.
///
/// * Transformers alter the input row by changing the values of some of its
/// cells in the output, without excluding them completely. Currently, the only
/// supported transformer is the "strip_value_transformer", which replaces every
/// cell's value with the empty string.
///
/// * Chains and interleaves are described in more detail in the
/// RowFilter.Chain and RowFilter.Interleave documentation.
///
/// The total serialized size of a RowFilter message must not
/// exceed 4096 bytes, and RowFilters may not be nested within each other
/// (in Chains or Interleaves) to a depth of more than 20.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowFilter {
    /// Which of the possible RowFilter types to apply. If none are set, this
    /// RowFilter returns all cells in the input row.
    #[prost(
        oneof = "row_filter::Filter",
        tags = "1, 2, 3, 16, 17, 18, 4, 14, 5, 6, 7, 8, 9, 15, 10, 11, 12, 13, 19"
    )]
    pub filter: ::std::option::Option<row_filter::Filter>,
}
pub mod row_filter {
    /// A RowFilter which sends rows through several RowFilters in sequence.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Chain {
        /// The elements of "filters" are chained together to process the input row:
        /// in row -> f(0) -> intermediate row -> f(1) -> ... -> f(N) -> out row
        /// The full chain is executed atomically.
        #[prost(message, repeated, tag = "1")]
        pub filters: ::std::vec::Vec<super::RowFilter>,
    }
    /// A RowFilter which sends each row to each of several component
    /// RowFilters and interleaves the results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Interleave {
        /// The elements of "filters" all process a copy of the input row, and the
        /// results are pooled, sorted, and combined into a single output row.
        /// If multiple cells are produced with the same column and timestamp,
        /// they will all appear in the output row in an unspecified mutual order.
        /// Consider the following example, with three filters:
        ///
        ///                              input row
        ///                                  |
        ///        -----------------------------------------------------
        ///        |                         |                         |
        ///       f(0)                      f(1)                      f(2)
        ///        |                         |                         |
        /// 1: foo,bar,10,x             foo,bar,10,z              far,bar,7,a
        /// 2: foo,blah,11,z            far,blah,5,x              far,blah,5,x
        ///        |                         |                         |
        ///        -----------------------------------------------------
        ///                                  |
        /// 1:                        foo,bar,10,z     // could have switched with #2
        /// 2:                        foo,bar,10,x     // could have switched with #1
        /// 3:                        foo,blah,11,z
        /// 4:                        far,bar,7,a
        /// 5:                        far,blah,5,x     // identical to #6
        /// 6:                        far,blah,5,x     // identical to #5
        /// All interleaved filters are executed atomically.
        #[prost(message, repeated, tag = "1")]
        pub filters: ::std::vec::Vec<super::RowFilter>,
    }
    /// A RowFilter which evaluates one of two possible RowFilters, depending on
    /// whether or not a predicate RowFilter outputs any cells from the input row.
    ///
    /// IMPORTANT NOTE: The predicate filter does not execute atomically with the
    /// true and false filters, which may lead to inconsistent or unexpected
    /// results. Additionally, Condition filters have poor performance, especially
    /// when filters are set for the false condition.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Condition {
        /// If "predicate_filter" outputs any cells, then "true_filter" will be
        /// evaluated on the input row. Otherwise, "false_filter" will be evaluated.
        #[prost(message, optional, boxed, tag = "1")]
        pub predicate_filter: ::std::option::Option<::std::boxed::Box<super::RowFilter>>,
        /// The filter to apply to the input row if "predicate_filter" returns any
        /// results. If not provided, no results will be returned in the true case.
        #[prost(message, optional, boxed, tag = "2")]
        pub true_filter: ::std::option::Option<::std::boxed::Box<super::RowFilter>>,
        /// The filter to apply to the input row if "predicate_filter" does not
        /// return any results. If not provided, no results will be returned in the
        /// false case.
        #[prost(message, optional, boxed, tag = "3")]
        pub false_filter: ::std::option::Option<::std::boxed::Box<super::RowFilter>>,
    }
    /// Which of the possible RowFilter types to apply. If none are set, this
    /// RowFilter returns all cells in the input row.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// Applies several RowFilters to the data in sequence, progressively
        /// narrowing the results.
        #[prost(message, tag = "1")]
        Chain(Chain),
        /// Applies several RowFilters to the data in parallel and combines the
        /// results.
        #[prost(message, tag = "2")]
        Interleave(Interleave),
        /// Applies one of two possible RowFilters to the data based on the output of
        /// a predicate RowFilter.
        #[prost(message, tag = "3")]
        Condition(Box<Condition>),
        /// ADVANCED USE ONLY.
        /// Hook for introspection into the RowFilter. Outputs all cells directly to
        /// the output of the read rather than to any parent filter. Consider the
        /// following example:
        ///
        /// Chain(
        ///   FamilyRegex("A"),
        ///   Interleave(
        ///     All(),
        ///     Chain(Label("foo"), Sink())
        ///   ),
        ///   QualifierRegex("B")
        /// )
        ///
        ///                         A,A,1,w
        ///                         A,B,2,x
        ///                         B,B,4,z
        ///                            |
        ///                     FamilyRegex("A")
        ///                            |
        ///                         A,A,1,w
        ///                         A,B,2,x
        ///                            |
        ///               +------------+-------------+
        ///               |                          |
        ///             All()                    Label(foo)
        ///               |                          |
        ///            A,A,1,w              A,A,1,w,labels:[foo]
        ///            A,B,2,x              A,B,2,x,labels:[foo]
        ///               |                          |
        ///               |                        Sink() --------------+
        ///               |                          |                  |
        ///               +------------+      x------+          A,A,1,w,labels:[foo]
        ///                            |                        A,B,2,x,labels:[foo]
        ///                         A,A,1,w                             |
        ///                         A,B,2,x                             |
        ///                            |                                |
        ///                    QualifierRegex("B")                      |
        ///                            |                                |
        ///                         A,B,2,x                             |
        ///                            |                                |
        ///                            +--------------------------------+
        ///                            |
        ///                         A,A,1,w,labels:[foo]
        ///                         A,B,2,x,labels:[foo]  // could be switched
        ///                         A,B,2,x               // could be switched
        ///
        /// Despite being excluded by the qualifier filter, a copy of every cell
        /// that reaches the sink is present in the final result.
        ///
        /// As with an [Interleave][google.bigtable.v1.RowFilter.Interleave],
        /// duplicate cells are possible, and appear in an unspecified mutual order.
        /// In this case we have a duplicate with column "A:B" and timestamp 2,
        /// because one copy passed through the all filter while the other was
        /// passed through the label and sink. Note that one copy has label "foo",
        /// while the other does not.
        ///
        /// Cannot be used within the `predicate_filter`, `true_filter`, or
        /// `false_filter` of a [Condition][google.bigtable.v1.RowFilter.Condition].
        #[prost(bool, tag = "16")]
        Sink(bool),
        /// Matches all cells, regardless of input. Functionally equivalent to
        /// leaving `filter` unset, but included for completeness.
        #[prost(bool, tag = "17")]
        PassAllFilter(bool),
        /// Does not match any cells, regardless of input. Useful for temporarily
        /// disabling just part of a filter.
        #[prost(bool, tag = "18")]
        BlockAllFilter(bool),
        /// Matches only cells from rows whose keys satisfy the given RE2 regex. In
        /// other words, passes through the entire row when the key matches, and
        /// otherwise produces an empty row.
        /// Note that, since row keys can contain arbitrary bytes, the '\C' escape
        /// sequence must be used if a true wildcard is desired. The '.' character
        /// will not match the new line character '\n', which may be present in a
        /// binary key.
        #[prost(bytes, tag = "4")]
        RowKeyRegexFilter(std::vec::Vec<u8>),
        /// Matches all cells from a row with probability p, and matches no cells
        /// from the row with probability 1-p.
        #[prost(double, tag = "14")]
        RowSampleFilter(f64),
        /// Matches only cells from columns whose families satisfy the given RE2
        /// regex. For technical reasons, the regex must not contain the ':'
        /// character, even if it is not being used as a literal.
        /// Note that, since column families cannot contain the new line character
        /// '\n', it is sufficient to use '.' as a full wildcard when matching
        /// column family names.
        #[prost(string, tag = "5")]
        FamilyNameRegexFilter(std::string::String),
        /// Matches only cells from columns whose qualifiers satisfy the given RE2
        /// regex.
        /// Note that, since column qualifiers can contain arbitrary bytes, the '\C'
        /// escape sequence must be used if a true wildcard is desired. The '.'
        /// character will not match the new line character '\n', which may be
        /// present in a binary qualifier.
        #[prost(bytes, tag = "6")]
        ColumnQualifierRegexFilter(std::vec::Vec<u8>),
        /// Matches only cells from columns within the given range.
        #[prost(message, tag = "7")]
        ColumnRangeFilter(super::ColumnRange),
        /// Matches only cells with timestamps within the given range.
        #[prost(message, tag = "8")]
        TimestampRangeFilter(super::TimestampRange),
        /// Matches only cells with values that satisfy the given regular expression.
        /// Note that, since cell values can contain arbitrary bytes, the '\C' escape
        /// sequence must be used if a true wildcard is desired. The '.' character
        /// will not match the new line character '\n', which may be present in a
        /// binary value.
        #[prost(bytes, tag = "9")]
        ValueRegexFilter(std::vec::Vec<u8>),
        /// Matches only cells with values that fall within the given range.
        #[prost(message, tag = "15")]
        ValueRangeFilter(super::ValueRange),
        /// Skips the first N cells of each row, matching all subsequent cells.
        /// If duplicate cells are present, as is possible when using an Interleave,
        /// each copy of the cell is counted separately.
        #[prost(int32, tag = "10")]
        CellsPerRowOffsetFilter(i32),
        /// Matches only the first N cells of each row.
        /// If duplicate cells are present, as is possible when using an Interleave,
        /// each copy of the cell is counted separately.
        #[prost(int32, tag = "11")]
        CellsPerRowLimitFilter(i32),
        /// Matches only the most recent N cells within each column. For example,
        /// if N=2, this filter would match column "foo:bar" at timestamps 10 and 9,
        /// skip all earlier cells in "foo:bar", and then begin matching again in
        /// column "foo:bar2".
        /// If duplicate cells are present, as is possible when using an Interleave,
        /// each copy of the cell is counted separately.
        #[prost(int32, tag = "12")]
        CellsPerColumnLimitFilter(i32),
        /// Replaces each cell's value with the empty string.
        #[prost(bool, tag = "13")]
        StripValueTransformer(bool),
        /// Applies the given label to all cells in the output row. This allows
        /// the client to determine which results were produced from which part of
        /// the filter.
        ///
        /// Values must be at most 15 characters in length, and match the RE2
        /// pattern [a-z0-9\\-]+
        ///
        /// Due to a technical limitation, it is not currently possible to apply
        /// multiple labels to a cell. As a result, a Chain may have no more than
        /// one sub-filter which contains a apply_label_transformer. It is okay for
        /// an Interleave to contain multiple apply_label_transformers, as they will
        /// be applied to separate copies of the input. This may be relaxed in the
        /// future.
        #[prost(string, tag = "19")]
        ApplyLabelTransformer(std::string::String),
    }
}
/// Specifies a particular change to be made to the contents of a row.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutation {
    /// Which of the possible Mutation types to apply.
    #[prost(oneof = "mutation::Mutation", tags = "1, 2, 3, 4")]
    pub mutation: ::std::option::Option<mutation::Mutation>,
}
pub mod mutation {
    /// A Mutation which sets the value of the specified cell.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetCell {
        /// The name of the family into which new data should be written.
        /// Must match [-_.a-zA-Z0-9]+
        #[prost(string, tag = "1")]
        pub family_name: std::string::String,
        /// The qualifier of the column into which new data should be written.
        /// Can be any byte string, including the empty string.
        #[prost(bytes, tag = "2")]
        pub column_qualifier: std::vec::Vec<u8>,
        /// The timestamp of the cell into which new data should be written.
        /// Use -1 for current Bigtable server time.
        /// Otherwise, the client should set this value itself, noting that the
        /// default value is a timestamp of zero if the field is left unspecified.
        /// Values must match the "granularity" of the table (e.g. micros, millis).
        #[prost(int64, tag = "3")]
        pub timestamp_micros: i64,
        /// The value to be written into the specified cell.
        #[prost(bytes, tag = "4")]
        pub value: std::vec::Vec<u8>,
    }
    /// A Mutation which deletes cells from the specified column, optionally
    /// restricting the deletions to a given timestamp range.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeleteFromColumn {
        /// The name of the family from which cells should be deleted.
        /// Must match [-_.a-zA-Z0-9]+
        #[prost(string, tag = "1")]
        pub family_name: std::string::String,
        /// The qualifier of the column from which cells should be deleted.
        /// Can be any byte string, including the empty string.
        #[prost(bytes, tag = "2")]
        pub column_qualifier: std::vec::Vec<u8>,
        /// The range of timestamps within which cells should be deleted.
        #[prost(message, optional, tag = "3")]
        pub time_range: ::std::option::Option<super::TimestampRange>,
    }
    /// A Mutation which deletes all cells from the specified column family.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeleteFromFamily {
        /// The name of the family from which cells should be deleted.
        /// Must match [-_.a-zA-Z0-9]+
        #[prost(string, tag = "1")]
        pub family_name: std::string::String,
    }
    /// A Mutation which deletes all cells from the containing row.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeleteFromRow {}
    /// Which of the possible Mutation types to apply.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mutation {
        /// Set a cell's value.
        #[prost(message, tag = "1")]
        SetCell(SetCell),
        /// Deletes cells from a column.
        #[prost(message, tag = "2")]
        DeleteFromColumn(DeleteFromColumn),
        /// Deletes cells from a column family.
        #[prost(message, tag = "3")]
        DeleteFromFamily(DeleteFromFamily),
        /// Deletes cells from the entire row.
        #[prost(message, tag = "4")]
        DeleteFromRow(DeleteFromRow),
    }
}
/// Specifies an atomic read/modify/write operation on the latest value of the
/// specified column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadModifyWriteRule {
    /// The name of the family to which the read/modify/write should be applied.
    /// Must match [-_.a-zA-Z0-9]+
    #[prost(string, tag = "1")]
    pub family_name: std::string::String,
    /// The qualifier of the column to which the read/modify/write should be
    /// applied.
    /// Can be any byte string, including the empty string.
    #[prost(bytes, tag = "2")]
    pub column_qualifier: std::vec::Vec<u8>,
    /// The rule used to determine the column's new latest value from its current
    /// latest value.
    #[prost(oneof = "read_modify_write_rule::Rule", tags = "3, 4")]
    pub rule: ::std::option::Option<read_modify_write_rule::Rule>,
}
pub mod read_modify_write_rule {
    /// The rule used to determine the column's new latest value from its current
    /// latest value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rule {
        /// Rule specifying that "append_value" be appended to the existing value.
        /// If the targeted cell is unset, it will be treated as containing the
        /// empty string.
        #[prost(bytes, tag = "3")]
        AppendValue(std::vec::Vec<u8>),
        /// Rule specifying that "increment_amount" be added to the existing value.
        /// If the targeted cell is unset, it will be treated as containing a zero.
        /// Otherwise, the targeted cell must contain an 8-byte value (interpreted
        /// as a 64-bit big-endian signed integer), or the entire request will fail.
        #[prost(int64, tag = "4")]
        IncrementAmount(i64),
    }
}
/// Request message for BigtableServer.ReadRows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsRequest {
    /// The unique name of the table from which to read.
    #[prost(string, tag = "1")]
    pub table_name: std::string::String,
    /// The filter to apply to the contents of the specified row(s). If unset,
    /// reads the entire table.
    #[prost(message, optional, tag = "5")]
    pub filter: ::std::option::Option<RowFilter>,
    /// By default, rows are read sequentially, producing results which are
    /// guaranteed to arrive in increasing row order. Setting
    /// "allow_row_interleaving" to true allows multiple rows to be interleaved in
    /// the response stream, which increases throughput but breaks this guarantee,
    /// and may force the client to use more memory to buffer partially-received
    /// rows. Cannot be set to true when specifying "num_rows_limit".
    #[prost(bool, tag = "6")]
    pub allow_row_interleaving: bool,
    /// The read will terminate after committing to N rows' worth of results. The
    /// default (zero) is to return all results.
    /// Note that "allow_row_interleaving" cannot be set to true when this is set.
    #[prost(int64, tag = "7")]
    pub num_rows_limit: i64,
    /// If neither row_key nor row_range is set, reads from all rows.
    #[prost(oneof = "read_rows_request::Target", tags = "2, 3, 8")]
    pub target: ::std::option::Option<read_rows_request::Target>,
}
pub mod read_rows_request {
    /// If neither row_key nor row_range is set, reads from all rows.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The key of a single row from which to read.
        #[prost(bytes, tag = "2")]
        RowKey(std::vec::Vec<u8>),
        /// A range of rows from which to read.
        #[prost(message, tag = "3")]
        RowRange(super::RowRange),
        /// A set of rows from which to read. Entries need not be in order, and will
        /// be deduplicated before reading.
        /// The total serialized size of the set must not exceed 1MB.
        #[prost(message, tag = "8")]
        RowSet(super::RowSet),
    }
}
/// Response message for BigtableService.ReadRows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRowsResponse {
    /// The key of the row for which we're receiving data.
    /// Results will be received in increasing row key order, unless
    /// "allow_row_interleaving" was specified in the request.
    #[prost(bytes, tag = "1")]
    pub row_key: std::vec::Vec<u8>,
    /// One or more chunks of the row specified by "row_key".
    #[prost(message, repeated, tag = "2")]
    pub chunks: ::std::vec::Vec<read_rows_response::Chunk>,
}
pub mod read_rows_response {
    /// Specifies a piece of a row's contents returned as part of the read
    /// response stream.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Chunk {
        #[prost(oneof = "chunk::Chunk", tags = "1, 2, 3")]
        pub chunk: ::std::option::Option<chunk::Chunk>,
    }
    pub mod chunk {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Chunk {
            /// A subset of the data from a particular row. As long as no "reset_row"
            /// is received in between, multiple "row_contents" from the same row are
            /// from the same atomic view of that row, and will be received in the
            /// expected family/column/timestamp order.
            #[prost(message, tag = "1")]
            RowContents(super::super::Family),
            /// Indicates that the client should drop all previous chunks for
            /// "row_key", as it will be re-read from the beginning.
            #[prost(bool, tag = "2")]
            ResetRow(bool),
            /// Indicates that the client can safely process all previous chunks for
            /// "row_key", as its data has been fully read.
            #[prost(bool, tag = "3")]
            CommitRow(bool),
        }
    }
}
/// Request message for BigtableService.SampleRowKeys.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleRowKeysRequest {
    /// The unique name of the table from which to sample row keys.
    #[prost(string, tag = "1")]
    pub table_name: std::string::String,
}
/// Response message for BigtableService.SampleRowKeys.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SampleRowKeysResponse {
    /// Sorted streamed sequence of sample row keys in the table. The table might
    /// have contents before the first row key in the list and after the last one,
    /// but a key containing the empty string indicates "end of table" and will be
    /// the last response given, if present.
    /// Note that row keys in this list may not have ever been written to or read
    /// from, and users should therefore not make any assumptions about the row key
    /// structure that are specific to their use case.
    #[prost(bytes, tag = "1")]
    pub row_key: std::vec::Vec<u8>,
    /// Approximate total storage space used by all rows in the table which precede
    /// "row_key". Buffering the contents of all rows between two subsequent
    /// samples would require space roughly equal to the difference in their
    /// "offset_bytes" fields.
    #[prost(int64, tag = "2")]
    pub offset_bytes: i64,
}
/// Request message for BigtableService.MutateRow.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateRowRequest {
    /// The unique name of the table to which the mutation should be applied.
    #[prost(string, tag = "1")]
    pub table_name: std::string::String,
    /// The key of the row to which the mutation should be applied.
    #[prost(bytes, tag = "2")]
    pub row_key: std::vec::Vec<u8>,
    /// Changes to be atomically applied to the specified row. Entries are applied
    /// in order, meaning that earlier mutations can be masked by later ones.
    /// Must contain at least one entry and at most 100000.
    #[prost(message, repeated, tag = "3")]
    pub mutations: ::std::vec::Vec<Mutation>,
}
/// Request message for BigtableService.MutateRows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateRowsRequest {
    /// The unique name of the table to which the mutations should be applied.
    #[prost(string, tag = "1")]
    pub table_name: std::string::String,
    /// The row keys/mutations to be applied in bulk.
    /// Each entry is applied as an atomic mutation, but the entries may be
    /// applied in arbitrary order (even between entries for the same row).
    /// At least one entry must be specified, and in total the entries may
    /// contain at most 100000 mutations.
    #[prost(message, repeated, tag = "2")]
    pub entries: ::std::vec::Vec<mutate_rows_request::Entry>,
}
pub mod mutate_rows_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        /// The key of the row to which the `mutations` should be applied.
        #[prost(bytes, tag = "1")]
        pub row_key: std::vec::Vec<u8>,
        /// Changes to be atomically applied to the specified row. Mutations are
        /// applied in order, meaning that earlier mutations can be masked by
        /// later ones.
        /// At least one mutation must be specified.
        #[prost(message, repeated, tag = "2")]
        pub mutations: ::std::vec::Vec<super::Mutation>,
    }
}
/// Response message for BigtableService.MutateRows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateRowsResponse {
    /// The results for each Entry from the request, presented in the order
    /// in which the entries were originally given.
    /// Depending on how requests are batched during execution, it is possible
    /// for one Entry to fail due to an error with another Entry. In the event
    /// that this occurs, the same error will be reported for both entries.
    #[prost(message, repeated, tag = "1")]
    pub statuses: ::std::vec::Vec<super::super::rpc::Status>,
}
/// Request message for BigtableService.CheckAndMutateRowRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckAndMutateRowRequest {
    /// The unique name of the table to which the conditional mutation should be
    /// applied.
    #[prost(string, tag = "1")]
    pub table_name: std::string::String,
    /// The key of the row to which the conditional mutation should be applied.
    #[prost(bytes, tag = "2")]
    pub row_key: std::vec::Vec<u8>,
    /// The filter to be applied to the contents of the specified row. Depending
    /// on whether or not any results are yielded, either "true_mutations" or
    /// "false_mutations" will be executed. If unset, checks that the row contains
    /// any values at all.
    #[prost(message, optional, tag = "6")]
    pub predicate_filter: ::std::option::Option<RowFilter>,
    /// Changes to be atomically applied to the specified row if "predicate_filter"
    /// yields at least one cell when applied to "row_key". Entries are applied in
    /// order, meaning that earlier mutations can be masked by later ones.
    /// Must contain at least one entry if "false_mutations" is empty, and at most
    /// 100000.
    #[prost(message, repeated, tag = "4")]
    pub true_mutations: ::std::vec::Vec<Mutation>,
    /// Changes to be atomically applied to the specified row if "predicate_filter"
    /// does not yield any cells when applied to "row_key". Entries are applied in
    /// order, meaning that earlier mutations can be masked by later ones.
    /// Must contain at least one entry if "true_mutations" is empty, and at most
    /// 100000.
    #[prost(message, repeated, tag = "5")]
    pub false_mutations: ::std::vec::Vec<Mutation>,
}
/// Response message for BigtableService.CheckAndMutateRowRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckAndMutateRowResponse {
    /// Whether or not the request's "predicate_filter" yielded any results for
    /// the specified row.
    #[prost(bool, tag = "1")]
    pub predicate_matched: bool,
}
/// Request message for BigtableService.ReadModifyWriteRowRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadModifyWriteRowRequest {
    /// The unique name of the table to which the read/modify/write rules should be
    /// applied.
    #[prost(string, tag = "1")]
    pub table_name: std::string::String,
    /// The key of the row to which the read/modify/write rules should be applied.
    #[prost(bytes, tag = "2")]
    pub row_key: std::vec::Vec<u8>,
    /// Rules specifying how the specified row's contents are to be transformed
    /// into writes. Entries are applied in order, meaning that earlier rules will
    /// affect the results of later ones.
    #[prost(message, repeated, tag = "3")]
    pub rules: ::std::vec::Vec<ReadModifyWriteRule>,
}
#[doc = r" Generated client implementations."]
pub mod bigtable_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for reading from and writing to existing Bigtables."]
    pub struct BigtableServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigtableServiceClient<T>
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
        #[doc = " Streams back the contents of all requested rows, optionally applying"]
        #[doc = " the same Reader filter to each. Depending on their size, rows may be"]
        #[doc = " broken up across multiple responses, but atomicity of each row will still"]
        #[doc = " be preserved."]
        pub async fn read_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRowsRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ReadRowsResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v1.BigtableService/ReadRows",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Returns a sample of row keys in the table. The returned row keys will"]
        #[doc = " delimit contiguous sections of the table of approximately equal size,"]
        #[doc = " which can be used to break up the data for distributed tasks like"]
        #[doc = " mapreduces."]
        pub async fn sample_row_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::SampleRowKeysRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SampleRowKeysResponse>>,
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
                "/google.bigtable.v1.BigtableService/SampleRowKeys",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Mutates a row atomically. Cells already present in the row are left"]
        #[doc = " unchanged unless explicitly changed by 'mutation'."]
        pub async fn mutate_row(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateRowRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v1.BigtableService/MutateRow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Mutates multiple rows in a batch. Each individual row is mutated"]
        #[doc = " atomically as in MutateRow, but the entire batch is not executed"]
        #[doc = " atomically."]
        pub async fn mutate_rows(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateRowsRequest>,
        ) -> Result<tonic::Response<super::MutateRowsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v1.BigtableService/MutateRows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Mutates a row atomically based on the output of a predicate Reader filter."]
        pub async fn check_and_mutate_row(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckAndMutateRowRequest>,
        ) -> Result<tonic::Response<super::CheckAndMutateRowResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v1.BigtableService/CheckAndMutateRow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Modifies a row atomically, reading the latest existing timestamp/value from"]
        #[doc = " the specified columns and writing a new value at"]
        #[doc = " max(existing timestamp, current server time) based on pre-defined"]
        #[doc = " read/modify/write rules. Returns the new contents of all modified cells."]
        pub async fn read_modify_write_row(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadModifyWriteRowRequest>,
        ) -> Result<tonic::Response<super::Row>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.v1.BigtableService/ReadModifyWriteRow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BigtableServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BigtableServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BigtableServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod bigtable_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BigtableServiceServer."]
    #[async_trait]
    pub trait BigtableService: Send + Sync + 'static {
        #[doc = "Server streaming response type for the ReadRows method."]
        type ReadRowsStream: Stream<Item = Result<super::ReadRowsResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Streams back the contents of all requested rows, optionally applying"]
        #[doc = " the same Reader filter to each. Depending on their size, rows may be"]
        #[doc = " broken up across multiple responses, but atomicity of each row will still"]
        #[doc = " be preserved."]
        async fn read_rows(
            &self,
            request: tonic::Request<super::ReadRowsRequest>,
        ) -> Result<tonic::Response<Self::ReadRowsStream>, tonic::Status>;
        #[doc = "Server streaming response type for the SampleRowKeys method."]
        type SampleRowKeysStream: Stream<Item = Result<super::SampleRowKeysResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Returns a sample of row keys in the table. The returned row keys will"]
        #[doc = " delimit contiguous sections of the table of approximately equal size,"]
        #[doc = " which can be used to break up the data for distributed tasks like"]
        #[doc = " mapreduces."]
        async fn sample_row_keys(
            &self,
            request: tonic::Request<super::SampleRowKeysRequest>,
        ) -> Result<tonic::Response<Self::SampleRowKeysStream>, tonic::Status>;
        #[doc = " Mutates a row atomically. Cells already present in the row are left"]
        #[doc = " unchanged unless explicitly changed by 'mutation'."]
        async fn mutate_row(
            &self,
            request: tonic::Request<super::MutateRowRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Mutates multiple rows in a batch. Each individual row is mutated"]
        #[doc = " atomically as in MutateRow, but the entire batch is not executed"]
        #[doc = " atomically."]
        async fn mutate_rows(
            &self,
            request: tonic::Request<super::MutateRowsRequest>,
        ) -> Result<tonic::Response<super::MutateRowsResponse>, tonic::Status>;
        #[doc = " Mutates a row atomically based on the output of a predicate Reader filter."]
        async fn check_and_mutate_row(
            &self,
            request: tonic::Request<super::CheckAndMutateRowRequest>,
        ) -> Result<tonic::Response<super::CheckAndMutateRowResponse>, tonic::Status>;
        #[doc = " Modifies a row atomically, reading the latest existing timestamp/value from"]
        #[doc = " the specified columns and writing a new value at"]
        #[doc = " max(existing timestamp, current server time) based on pre-defined"]
        #[doc = " read/modify/write rules. Returns the new contents of all modified cells."]
        async fn read_modify_write_row(
            &self,
            request: tonic::Request<super::ReadModifyWriteRowRequest>,
        ) -> Result<tonic::Response<super::Row>, tonic::Status>;
    }
    #[doc = " Service for reading from and writing to existing Bigtables."]
    #[derive(Debug)]
    pub struct BigtableServiceServer<T: BigtableService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: BigtableService> BigtableServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for BigtableServiceServer<T>
    where
        T: BigtableService,
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
                "/google.bigtable.v1.BigtableService/ReadRows" => {
                    #[allow(non_camel_case_types)]
                    struct ReadRowsSvc<T: BigtableService>(pub Arc<T>);
                    impl<T: BigtableService>
                        tonic::server::ServerStreamingService<super::ReadRowsRequest>
                        for ReadRowsSvc<T>
                    {
                        type Response = super::ReadRowsResponse;
                        type ResponseStream = T::ReadRowsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadRowsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_rows(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ReadRowsSvc(inner);
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
                "/google.bigtable.v1.BigtableService/SampleRowKeys" => {
                    #[allow(non_camel_case_types)]
                    struct SampleRowKeysSvc<T: BigtableService>(pub Arc<T>);
                    impl<T: BigtableService>
                        tonic::server::ServerStreamingService<super::SampleRowKeysRequest>
                        for SampleRowKeysSvc<T>
                    {
                        type Response = super::SampleRowKeysResponse;
                        type ResponseStream = T::SampleRowKeysStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SampleRowKeysRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sample_row_keys(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = SampleRowKeysSvc(inner);
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
                "/google.bigtable.v1.BigtableService/MutateRow" => {
                    #[allow(non_camel_case_types)]
                    struct MutateRowSvc<T: BigtableService>(pub Arc<T>);
                    impl<T: BigtableService> tonic::server::UnaryService<super::MutateRowRequest> for MutateRowSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MutateRowRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mutate_row(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MutateRowSvc(inner);
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
                "/google.bigtable.v1.BigtableService/MutateRows" => {
                    #[allow(non_camel_case_types)]
                    struct MutateRowsSvc<T: BigtableService>(pub Arc<T>);
                    impl<T: BigtableService> tonic::server::UnaryService<super::MutateRowsRequest>
                        for MutateRowsSvc<T>
                    {
                        type Response = super::MutateRowsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MutateRowsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mutate_rows(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MutateRowsSvc(inner);
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
                "/google.bigtable.v1.BigtableService/CheckAndMutateRow" => {
                    #[allow(non_camel_case_types)]
                    struct CheckAndMutateRowSvc<T: BigtableService>(pub Arc<T>);
                    impl<T: BigtableService>
                        tonic::server::UnaryService<super::CheckAndMutateRowRequest>
                        for CheckAndMutateRowSvc<T>
                    {
                        type Response = super::CheckAndMutateRowResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckAndMutateRowRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).check_and_mutate_row(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CheckAndMutateRowSvc(inner);
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
                "/google.bigtable.v1.BigtableService/ReadModifyWriteRow" => {
                    #[allow(non_camel_case_types)]
                    struct ReadModifyWriteRowSvc<T: BigtableService>(pub Arc<T>);
                    impl<T: BigtableService>
                        tonic::server::UnaryService<super::ReadModifyWriteRowRequest>
                        for ReadModifyWriteRowSvc<T>
                    {
                        type Response = super::Row;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadModifyWriteRowRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_modify_write_row(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ReadModifyWriteRowSvc(inner);
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
    impl<T: BigtableService> Clone for BigtableServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: BigtableService> Clone for _Inner<T> {
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
