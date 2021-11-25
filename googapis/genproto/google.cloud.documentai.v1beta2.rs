/// A vertex represents a 2D point in the image.
/// NOTE: the vertex coordinates are in the same scale as the original image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    /// X coordinate.
    #[prost(int32, tag = "1")]
    pub x: i32,
    /// Y coordinate.
    #[prost(int32, tag = "2")]
    pub y: i32,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedVertex {
    /// X coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Y coordinate.
    #[prost(float, tag = "2")]
    pub y: f32,
}
/// A bounding polygon for the detected image annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingPoly {
    /// The bounding polygon vertices.
    #[prost(message, repeated, tag = "1")]
    pub vertices: ::prost::alloc::vec::Vec<Vertex>,
    /// The bounding polygon normalized vertices.
    #[prost(message, repeated, tag = "2")]
    pub normalized_vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// Document represents the canonical document resource in Document Understanding
/// AI.
/// It is an interchange format that provides insights into documents and allows
/// for collaboration between users and Document Understanding AI to iterate and
/// optimize for quality.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// An IANA published MIME type (also referred to as media type). For more
    /// information, see
    /// <https://www.iana.org/assignments/media-types/media-types.xhtml.>
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
    /// UTF-8 encoded text in reading order from the document.
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
    /// Styles for the \[Document.text][google.cloud.documentai.v1beta2.Document.text\].
    #[prost(message, repeated, tag = "5")]
    pub text_styles: ::prost::alloc::vec::Vec<document::Style>,
    /// Visual page layout for the \[Document][google.cloud.documentai.v1beta2.Document\].
    #[prost(message, repeated, tag = "6")]
    pub pages: ::prost::alloc::vec::Vec<document::Page>,
    /// A list of entities detected on \[Document.text][google.cloud.documentai.v1beta2.Document.text\]. For document shards,
    /// entities in this list may cross shard boundaries.
    #[prost(message, repeated, tag = "7")]
    pub entities: ::prost::alloc::vec::Vec<document::Entity>,
    /// Relationship among \[Document.entities][google.cloud.documentai.v1beta2.Document.entities\].
    #[prost(message, repeated, tag = "8")]
    pub entity_relations: ::prost::alloc::vec::Vec<document::EntityRelation>,
    /// Information about the sharding if this document is sharded part of a larger
    /// document. If the document is not sharded, this message is not specified.
    #[prost(message, optional, tag = "9")]
    pub shard_info: ::core::option::Option<document::ShardInfo>,
    /// \[Label][google.cloud.documentai.v1beta2.Document.Label\]s for this document.
    #[prost(message, repeated, tag = "11")]
    pub labels: ::prost::alloc::vec::Vec<document::Label>,
    /// Any error that occurred while processing this document.
    #[prost(message, optional, tag = "10")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Original source document from the user.
    #[prost(oneof = "document::Source", tags = "1, 2")]
    pub source: ::core::option::Option<document::Source>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// For a large document, sharding may be performed to produce several
    /// document shards. Each document shard contains this field to detail which
    /// shard it is.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShardInfo {
        /// The 0-based index of this shard.
        #[prost(int64, tag = "1")]
        pub shard_index: i64,
        /// Total number of shards.
        #[prost(int64, tag = "2")]
        pub shard_count: i64,
        /// The index of the first character in \[Document.text][google.cloud.documentai.v1beta2.Document.text\] in the overall
        /// document global text.
        #[prost(int64, tag = "3")]
        pub text_offset: i64,
    }
    /// Label attaches schema information and/or other metadata to segments within
    /// a \[Document][google.cloud.documentai.v1beta2.Document\]. Multiple \[Label][google.cloud.documentai.v1beta2.Document.Label\]s on a single field can denote either
    /// different labels, different instances of the same label created at
    /// different times, or some combination of both.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Label {
        /// Name of the label.
        ///
        /// When the label is generated from AutoML Text Classification model, this
        /// field represents the name of the category.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Confidence score between 0 and 1 for label assignment.
        #[prost(float, tag = "3")]
        pub confidence: f32,
        /// Provenance of the label.
        #[prost(oneof = "label::Source", tags = "2")]
        pub source: ::core::option::Option<label::Source>,
    }
    /// Nested message and enum types in `Label`.
    pub mod label {
        /// Provenance of the label.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            /// Label is generated AutoML model. This field stores the full resource
            /// name of the AutoML model.
            ///
            /// Format:
            /// `projects/{project-id}/locations/{location-id}/models/{model-id}`
            #[prost(string, tag = "2")]
            AutomlModel(::prost::alloc::string::String),
        }
    }
    /// Annotation for common text style attributes. This adheres to CSS
    /// conventions as much as possible.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Style {
        /// Text anchor indexing into the \[Document.text][google.cloud.documentai.v1beta2.Document.text\].
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// Text color.
        #[prost(message, optional, tag = "2")]
        pub color: ::core::option::Option<super::super::super::super::r#type::Color>,
        /// Text background color.
        #[prost(message, optional, tag = "3")]
        pub background_color: ::core::option::Option<super::super::super::super::r#type::Color>,
        /// Font weight. Possible values are normal, bold, bolder, and lighter.
        /// <https://www.w3schools.com/cssref/pr_font_weight.asp>
        #[prost(string, tag = "4")]
        pub font_weight: ::prost::alloc::string::String,
        /// Text style. Possible values are normal, italic, and oblique.
        /// <https://www.w3schools.com/cssref/pr_font_font-style.asp>
        #[prost(string, tag = "5")]
        pub text_style: ::prost::alloc::string::String,
        /// Text decoration. Follows CSS standard.
        /// <text-decoration-line> <text-decoration-color> <text-decoration-style>
        /// <https://www.w3schools.com/cssref/pr_text_text-decoration.asp>
        #[prost(string, tag = "6")]
        pub text_decoration: ::prost::alloc::string::String,
        /// Font size.
        #[prost(message, optional, tag = "7")]
        pub font_size: ::core::option::Option<style::FontSize>,
    }
    /// Nested message and enum types in `Style`.
    pub mod style {
        /// Font size with unit.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FontSize {
            /// Font size for the text.
            #[prost(float, tag = "1")]
            pub size: f32,
            /// Unit for the font size. Follows CSS naming (in, px, pt, etc.).
            #[prost(string, tag = "2")]
            pub unit: ::prost::alloc::string::String,
        }
    }
    /// A page in a \[Document][google.cloud.documentai.v1beta2.Document\].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Page {
        /// 1-based index for current \[Page][google.cloud.documentai.v1beta2.Document.Page\] in a parent \[Document][google.cloud.documentai.v1beta2.Document\].
        /// Useful when a page is taken out of a \[Document][google.cloud.documentai.v1beta2.Document\] for individual
        /// processing.
        #[prost(int32, tag = "1")]
        pub page_number: i32,
        /// Physical dimension of the page.
        #[prost(message, optional, tag = "2")]
        pub dimension: ::core::option::Option<page::Dimension>,
        /// \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] for the page.
        #[prost(message, optional, tag = "3")]
        pub layout: ::core::option::Option<page::Layout>,
        /// A list of detected languages together with confidence.
        #[prost(message, repeated, tag = "4")]
        pub detected_languages: ::prost::alloc::vec::Vec<page::DetectedLanguage>,
        /// A list of visually detected text blocks on the page.
        /// A block has a set of lines (collected into paragraphs) that have a common
        /// line-spacing and orientation.
        #[prost(message, repeated, tag = "5")]
        pub blocks: ::prost::alloc::vec::Vec<page::Block>,
        /// A list of visually detected text paragraphs on the page.
        /// A collection of lines that a human would perceive as a paragraph.
        #[prost(message, repeated, tag = "6")]
        pub paragraphs: ::prost::alloc::vec::Vec<page::Paragraph>,
        /// A list of visually detected text lines on the page.
        /// A collection of tokens that a human would perceive as a line.
        #[prost(message, repeated, tag = "7")]
        pub lines: ::prost::alloc::vec::Vec<page::Line>,
        /// A list of visually detected tokens on the page.
        #[prost(message, repeated, tag = "8")]
        pub tokens: ::prost::alloc::vec::Vec<page::Token>,
        /// A list of detected non-text visual elements e.g. checkbox,
        /// signature etc. on the page.
        #[prost(message, repeated, tag = "9")]
        pub visual_elements: ::prost::alloc::vec::Vec<page::VisualElement>,
        /// A list of visually detected tables on the page.
        #[prost(message, repeated, tag = "10")]
        pub tables: ::prost::alloc::vec::Vec<page::Table>,
        /// A list of visually detected form fields on the page.
        #[prost(message, repeated, tag = "11")]
        pub form_fields: ::prost::alloc::vec::Vec<page::FormField>,
    }
    /// Nested message and enum types in `Page`.
    pub mod page {
        /// Dimension for the page.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Dimension {
            /// Page width.
            #[prost(float, tag = "1")]
            pub width: f32,
            /// Page height.
            #[prost(float, tag = "2")]
            pub height: f32,
            /// Dimension unit.
            #[prost(string, tag = "3")]
            pub unit: ::prost::alloc::string::String,
        }
        /// Visual element describing a layout unit on a page.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Layout {
            /// Text anchor indexing into the \[Document.text][google.cloud.documentai.v1beta2.Document.text\].
            #[prost(message, optional, tag = "1")]
            pub text_anchor: ::core::option::Option<super::TextAnchor>,
            /// Confidence of the current \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] within context of the object this
            /// layout is for. e.g. confidence can be for a single token, a table,
            /// a visual element, etc. depending on context. Range [0, 1].
            #[prost(float, tag = "2")]
            pub confidence: f32,
            /// The bounding polygon for the \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\].
            #[prost(message, optional, tag = "3")]
            pub bounding_poly: ::core::option::Option<super::super::BoundingPoly>,
            /// Detected orientation for the \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\].
            #[prost(enumeration = "layout::Orientation", tag = "4")]
            pub orientation: i32,
            /// Optional. This is the identifier used by referencing \[PageAnchor][google.cloud.documentai.v1beta2.Document.PageAnchor\]s.
            #[prost(string, tag = "5")]
            pub id: ::prost::alloc::string::String,
        }
        /// Nested message and enum types in `Layout`.
        pub mod layout {
            /// Detected human reading orientation.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum Orientation {
                /// Unspecified orientation.
                Unspecified = 0,
                /// Orientation is aligned with page up.
                PageUp = 1,
                /// Orientation is aligned with page right.
                /// Turn the head 90 degrees clockwise from upright to read.
                PageRight = 2,
                /// Orientation is aligned with page down.
                /// Turn the head 180 degrees from upright to read.
                PageDown = 3,
                /// Orientation is aligned with page left.
                /// Turn the head 90 degrees counterclockwise from upright to read.
                PageLeft = 4,
            }
        }
        /// A block has a set of lines (collected into paragraphs) that have a
        /// common line-spacing and orientation.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Block {
            /// \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] for \[Block][google.cloud.documentai.v1beta2.Document.Page.Block\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// A collection of lines that a human would perceive as a paragraph.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Paragraph {
            /// \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] for \[Paragraph][google.cloud.documentai.v1beta2.Document.Page.Paragraph\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// A collection of tokens that a human would perceive as a line.
        /// Does not cross column boundaries, can be horizontal, vertical, etc.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Line {
            /// \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] for \[Line][google.cloud.documentai.v1beta2.Document.Page.Line\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// A detected token.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Token {
            /// \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] for \[Token][google.cloud.documentai.v1beta2.Document.Page.Token\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Detected break at the end of a \[Token][google.cloud.documentai.v1beta2.Document.Page.Token\].
            #[prost(message, optional, tag = "2")]
            pub detected_break: ::core::option::Option<token::DetectedBreak>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// Nested message and enum types in `Token`.
        pub mod token {
            /// Detected break at the end of a \[Token][google.cloud.documentai.v1beta2.Document.Page.Token\].
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DetectedBreak {
                /// Detected break type.
                #[prost(enumeration = "detected_break::Type", tag = "1")]
                pub r#type: i32,
            }
            /// Nested message and enum types in `DetectedBreak`.
            pub mod detected_break {
                /// Enum to denote the type of break found.
                #[derive(
                    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
                )]
                #[repr(i32)]
                pub enum Type {
                    /// Unspecified break type.
                    Unspecified = 0,
                    /// A single whitespace.
                    Space = 1,
                    /// A wider whitespace.
                    WideSpace = 2,
                    /// A hyphen that indicates that a token has been split across lines.
                    Hyphen = 3,
                }
            }
        }
        /// Detected non-text visual elements e.g. checkbox, signature etc. on the
        /// page.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct VisualElement {
            /// \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] for \[VisualElement][google.cloud.documentai.v1beta2.Document.Page.VisualElement\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Type of the \[VisualElement][google.cloud.documentai.v1beta2.Document.Page.VisualElement\].
            #[prost(string, tag = "2")]
            pub r#type: ::prost::alloc::string::String,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// A table representation similar to HTML table structure.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Table {
            /// \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] for \[Table][google.cloud.documentai.v1beta2.Document.Page.Table\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Header rows of the table.
            #[prost(message, repeated, tag = "2")]
            pub header_rows: ::prost::alloc::vec::Vec<table::TableRow>,
            /// Body rows of the table.
            #[prost(message, repeated, tag = "3")]
            pub body_rows: ::prost::alloc::vec::Vec<table::TableRow>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "4")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// Nested message and enum types in `Table`.
        pub mod table {
            /// A row of table cells.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct TableRow {
                /// Cells that make up this row.
                #[prost(message, repeated, tag = "1")]
                pub cells: ::prost::alloc::vec::Vec<TableCell>,
            }
            /// A cell representation inside the table.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct TableCell {
                /// \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] for \[TableCell][google.cloud.documentai.v1beta2.Document.Page.Table.TableCell\].
                #[prost(message, optional, tag = "1")]
                pub layout: ::core::option::Option<super::Layout>,
                /// How many rows this cell spans.
                #[prost(int32, tag = "2")]
                pub row_span: i32,
                /// How many columns this cell spans.
                #[prost(int32, tag = "3")]
                pub col_span: i32,
                /// A list of detected languages together with confidence.
                #[prost(message, repeated, tag = "4")]
                pub detected_languages: ::prost::alloc::vec::Vec<super::DetectedLanguage>,
            }
        }
        /// A form field detected on the page.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FormField {
            /// \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] for the \[FormField][google.cloud.documentai.v1beta2.Document.Page.FormField\] name. e.g. `Address`, `Email`,
            /// `Grand total`, `Phone number`, etc.
            #[prost(message, optional, tag = "1")]
            pub field_name: ::core::option::Option<Layout>,
            /// \[Layout][google.cloud.documentai.v1beta2.Document.Page.Layout\] for the \[FormField][google.cloud.documentai.v1beta2.Document.Page.FormField\] value.
            #[prost(message, optional, tag = "2")]
            pub field_value: ::core::option::Option<Layout>,
            /// A list of detected languages for name together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub name_detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// A list of detected languages for value together with confidence.
            #[prost(message, repeated, tag = "4")]
            pub value_detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// If the value is non-textual, this field represents the type. Current
            /// valid values are:
            /// - blank (this indicates the field_value is normal text)
            /// - "unfilled_checkbox"
            /// - "filled_checkbox"
            #[prost(string, tag = "5")]
            pub value_type: ::prost::alloc::string::String,
            /// An internal field, created for Labeling UI to export key text.
            #[prost(string, tag = "6")]
            pub corrected_key_text: ::prost::alloc::string::String,
            /// An internal field, created for Labeling UI to export value text.
            #[prost(string, tag = "7")]
            pub corrected_value_text: ::prost::alloc::string::String,
        }
        /// Detected language for a structural component.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DetectedLanguage {
            /// The BCP-47 language code, such as "en-US" or "sr-Latn". For more
            /// information, see
            /// <http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
            #[prost(string, tag = "1")]
            pub language_code: ::prost::alloc::string::String,
            /// Confidence of detected language. Range [0, 1].
            #[prost(float, tag = "2")]
            pub confidence: f32,
        }
    }
    /// A phrase in the text that is a known entity type, such as a person, an
    /// organization, or location.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Provenance of the entity.
        /// Text anchor indexing into the \[Document.text][google.cloud.documentai.v1beta2.Document.text\].
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// Entity type from a schema e.g. `Address`.
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// Text value in the document e.g. `1600 Amphitheatre Pkwy`.
        #[prost(string, tag = "3")]
        pub mention_text: ::prost::alloc::string::String,
        /// Deprecated.  Use `id` field instead.
        #[prost(string, tag = "4")]
        pub mention_id: ::prost::alloc::string::String,
        /// Optional. Confidence of detected Schema entity. Range [0, 1].
        #[prost(float, tag = "5")]
        pub confidence: f32,
        /// Optional. Represents the provenance of this entity wrt. the location on the
        /// page where it was found.
        #[prost(message, optional, tag = "6")]
        pub page_anchor: ::core::option::Option<PageAnchor>,
        /// Optional. Canonical id. This will be a unique value in the entity list
        /// for this document.
        #[prost(string, tag = "7")]
        pub id: ::prost::alloc::string::String,
        /// Optional. Temporary field to store the bounding poly for short-term POCs. Used by
        /// the frontend only. Do not use before you talk to ybo@ and lukasr@.
        #[prost(message, optional, tag = "8")]
        pub bounding_poly_for_demo_frontend: ::core::option::Option<super::BoundingPoly>,
    }
    /// Relationship between \[Entities][google.cloud.documentai.v1beta2.Document.Entity\].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EntityRelation {
        /// Subject entity id.
        #[prost(string, tag = "1")]
        pub subject_id: ::prost::alloc::string::String,
        /// Object entity id.
        #[prost(string, tag = "2")]
        pub object_id: ::prost::alloc::string::String,
        /// Relationship description.
        #[prost(string, tag = "3")]
        pub relation: ::prost::alloc::string::String,
    }
    /// Text reference indexing into the \[Document.text][google.cloud.documentai.v1beta2.Document.text\].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextAnchor {
        /// The text segments from the \[Document.text][google.cloud.documentai.v1beta2.Document.text\].
        #[prost(message, repeated, tag = "1")]
        pub text_segments: ::prost::alloc::vec::Vec<text_anchor::TextSegment>,
    }
    /// Nested message and enum types in `TextAnchor`.
    pub mod text_anchor {
        /// A text segment in the \[Document.text][google.cloud.documentai.v1beta2.Document.text\]. The indices may be out of bounds
        /// which indicate that the text extends into another document shard for
        /// large sharded documents. See \[ShardInfo.text_offset][google.cloud.documentai.v1beta2.Document.ShardInfo.text_offset\]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TextSegment {
            /// \[TextSegment][google.cloud.documentai.v1beta2.Document.TextAnchor.TextSegment\] start UTF-8 char index in the \[Document.text][google.cloud.documentai.v1beta2.Document.text\].
            #[prost(int64, tag = "1")]
            pub start_index: i64,
            /// \[TextSegment][google.cloud.documentai.v1beta2.Document.TextAnchor.TextSegment\] half open end UTF-8 char index in the
            /// \[Document.text][google.cloud.documentai.v1beta2.Document.text\].
            #[prost(int64, tag = "2")]
            pub end_index: i64,
        }
    }
    /// Referencing elements in \[Document.pages][google.cloud.documentai.v1beta2.Document.pages\].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PageAnchor {
        /// One or more references to visual page elements
        #[prost(message, repeated, tag = "1")]
        pub page_refs: ::prost::alloc::vec::Vec<page_anchor::PageRef>,
    }
    /// Nested message and enum types in `PageAnchor`.
    pub mod page_anchor {
        /// Represents a weak reference to a page element within a document.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PageRef {
            /// Required. Index into the \[Document.pages][google.cloud.documentai.v1beta2.Document.pages\] element
            #[prost(int64, tag = "1")]
            pub page: i64,
            /// Optional. The type of the layout element that is being referenced.  If not
            /// specified the whole page is assumed to be referenced.
            #[prost(enumeration = "page_ref::LayoutType", tag = "2")]
            pub layout_type: i32,
            /// Optional. The \[Page.Layout.id][google.cloud.documentai.v1beta2.Document.Page.Layout.id\] on the page that this element
            /// references.  If \[LayoutRef.type][\] is specified this id must also be
            /// specified.
            #[prost(string, tag = "3")]
            pub layout_id: ::prost::alloc::string::String,
        }
        /// Nested message and enum types in `PageRef`.
        pub mod page_ref {
            /// The type of layout that is being referenced.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum LayoutType {
                /// Layout Unspecified.
                Unspecified = 0,
                /// References a \[Page.blocks][google.cloud.documentai.v1beta2.Document.Page.blocks\] element.
                Block = 1,
                /// References a \[Page.paragraphs][google.cloud.documentai.v1beta2.Document.Page.paragraphs\] element.
                Paragraph = 2,
                /// References a \[Page.lines][google.cloud.documentai.v1beta2.Document.Page.lines\] element.
                Line = 3,
                /// References a \[Page.tokens][google.cloud.documentai.v1beta2.Document.Page.tokens\] element.
                Token = 4,
                /// References a \[Page.visual_elements][google.cloud.documentai.v1beta2.Document.Page.visual_elements\] element.
                VisualElement = 5,
                /// Refrrences a \[Page.tables][google.cloud.documentai.v1beta2.Document.Page.tables\] element.
                Table = 6,
                /// References a \[Page.form_fields][google.cloud.documentai.v1beta2.Document.Page.form_fields\] element.
                FormField = 7,
            }
        }
    }
    /// Original source document from the user.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Currently supports Google Cloud Storage URI of the form
        ///    `gs://bucket_name/object_name`. Object versioning is not supported.
        ///    See [Google Cloud Storage Request
        ///    URIs](<https://cloud.google.com/storage/docs/reference-uris>) for more
        ///    info.
        #[prost(string, tag = "1")]
        Uri(::prost::alloc::string::String),
        /// Inline document content, represented as a stream of bytes.
        /// Note: As with all `bytes` fields, protobuffers use a pure binary
        /// representation, whereas JSON representations use base64.
        #[prost(bytes, tag = "2")]
        Content(::prost::alloc::vec::Vec<u8>),
    }
}
/// Request to batch process documents as an asynchronous operation. The output
/// is written to Cloud Storage as JSON in the \[Document\] format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchProcessDocumentsRequest {
    /// Required. Individual requests for each document.
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<ProcessDocumentRequest>,
    /// Target project and location to make a call.
    ///
    /// Format: `projects/{project-id}/locations/{location-id}`.
    ///
    /// If no location is specified, a region will be chosen automatically.
    #[prost(string, tag = "2")]
    pub parent: ::prost::alloc::string::String,
}
/// Request to process one document.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessDocumentRequest {
    /// Target project and location to make a call.
    ///
    /// Format: `projects/{project-id}/locations/{location-id}`.
    ///
    /// If no location is specified, a region will be chosen automatically.
    /// This field is only populated when used in ProcessDocument method.
    #[prost(string, tag = "9")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Information about the input file.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// Optional. The desired output location. This field is only needed in
    /// BatchProcessDocumentsRequest.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputConfig>,
    /// Specifies a known document type for deeper structure detection. Valid
    /// values are currently "general" and "invoice". If not provided, "general"\
    /// is used as default. If any other value is given, the request is rejected.
    #[prost(string, tag = "3")]
    pub document_type: ::prost::alloc::string::String,
    /// Controls table extraction behavior. If not specified, the system will
    /// decide reasonable defaults.
    #[prost(message, optional, tag = "4")]
    pub table_extraction_params: ::core::option::Option<TableExtractionParams>,
    /// Controls form extraction behavior. If not specified, the system will
    /// decide reasonable defaults.
    #[prost(message, optional, tag = "5")]
    pub form_extraction_params: ::core::option::Option<FormExtractionParams>,
    /// Controls entity extraction behavior. If not specified, the system will
    /// decide reasonable defaults.
    #[prost(message, optional, tag = "6")]
    pub entity_extraction_params: ::core::option::Option<EntityExtractionParams>,
    /// Controls OCR behavior. If not specified, the system will decide reasonable
    /// defaults.
    #[prost(message, optional, tag = "7")]
    pub ocr_params: ::core::option::Option<OcrParams>,
    /// Controls AutoML model prediction behavior. AutoMlParams cannot be used
    /// together with other Params.
    #[prost(message, optional, tag = "8")]
    pub automl_params: ::core::option::Option<AutoMlParams>,
}
/// Response to an batch document processing request. This is returned in
/// the LRO Operation after the operation is complete.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchProcessDocumentsResponse {
    /// Responses for each individual document.
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<ProcessDocumentResponse>,
}
/// Response to a single document processing request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessDocumentResponse {
    /// Information about the input file. This is the same as the corresponding
    /// input config in the request.
    #[prost(message, optional, tag = "1")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// The output location of the parsed responses. The responses are written to
    /// this location as JSON-serialized `Document` objects.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// Parameters to control Optical Character Recognition (OCR) behavior.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OcrParams {
    /// List of languages to use for OCR. In most cases, an empty value
    /// yields the best results since it enables automatic language detection. For
    /// languages based on the Latin alphabet, setting `language_hints` is not
    /// needed. In rare cases, when the language of the text in the image is known,
    /// setting a hint will help get better results (although it will be a
    /// significant hindrance if the hint is wrong). Document processing returns an
    /// error if one or more of the specified languages is not one of the
    /// supported languages.
    #[prost(string, repeated, tag = "1")]
    pub language_hints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Parameters to control table extraction behavior.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableExtractionParams {
    /// Whether to enable table extraction.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Optional. Table bounding box hints that can be provided to complex cases
    /// which our algorithm cannot locate the table(s) in.
    #[prost(message, repeated, tag = "2")]
    pub table_bound_hints: ::prost::alloc::vec::Vec<TableBoundHint>,
    /// Optional. Table header hints. The extraction will bias towards producing
    /// these terms as table headers, which may improve accuracy.
    #[prost(string, repeated, tag = "3")]
    pub header_hints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Model version of the table extraction system. Default is "builtin/stable".
    /// Specify "builtin/latest" for the latest model.
    #[prost(string, tag = "4")]
    pub model_version: ::prost::alloc::string::String,
}
/// A hint for a table bounding box on the page for table parsing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableBoundHint {
    /// Optional. Page number for multi-paged inputs this hint applies to. If not
    /// provided, this hint will apply to all pages by default. This value is
    /// 1-based.
    #[prost(int32, tag = "1")]
    pub page_number: i32,
    /// Bounding box hint for a table on this page. The coordinates must be
    /// normalized to \[0,1\] and the bounding box must be an axis-aligned rectangle.
    #[prost(message, optional, tag = "2")]
    pub bounding_box: ::core::option::Option<BoundingPoly>,
}
/// Parameters to control form extraction behavior.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FormExtractionParams {
    /// Whether to enable form extraction.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// User can provide pairs of (key text, value type) to improve the parsing
    /// result.
    ///
    /// For example, if a document has a field called "Date" that holds a date
    /// value and a field called "Amount" that may hold either a currency value
    /// (e.g., "$500.00") or a simple number value (e.g., "20"), you could use the
    /// following hints: [ {"key": "Date", value_types: [ "DATE"]}, {"key":
    /// "Amount", "value_types": [ "PRICE", "NUMBER" ]} ]
    ///
    /// If the value type is unknown, but you want to provide hints for the keys,
    /// you can leave the value_types field blank. e.g. {"key": "Date",
    /// "value_types": []}
    #[prost(message, repeated, tag = "2")]
    pub key_value_pair_hints: ::prost::alloc::vec::Vec<KeyValuePairHint>,
    /// Model version of the form extraction system. Default is
    /// "builtin/stable". Specify "builtin/latest" for the latest model.
    /// For custom form models, specify: “custom/{model_name}". Model name
    /// format is "bucket_name/path/to/modeldir" corresponding to
    /// "gs://bucket_name/path/to/modeldir" where annotated examples are stored.
    #[prost(string, tag = "3")]
    pub model_version: ::prost::alloc::string::String,
}
/// User-provided hint for key value pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValuePairHint {
    /// The key text for the hint.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Type of the value. This is case-insensitive, and could be one of:
    /// ADDRESS, LOCATION, ORGANIZATION, PERSON, PHONE_NUMBER,
    /// ID, NUMBER, EMAIL, PRICE, TERMS, DATE, NAME. Types not in this list will
    /// be ignored.
    #[prost(string, repeated, tag = "2")]
    pub value_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Parameters to control entity extraction behavior.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityExtractionParams {
    /// Whether to enable entity extraction.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Model version of the entity extraction. Default is
    /// "builtin/stable". Specify "builtin/latest" for the latest model.
    #[prost(string, tag = "2")]
    pub model_version: ::prost::alloc::string::String,
}
/// Parameters to control AutoML model prediction behavior.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoMlParams {
    /// Resource name of the AutoML model.
    ///
    /// Format: `projects/{project-id}/locations/{location-id}/models/{model-id}`.
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
}
/// The desired input location and metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// Required. Mimetype of the input. Current supported mimetypes are application/pdf,
    /// image/tiff, and image/gif.
    /// In addition, application/json type is supported for requests with
    /// \[ProcessDocumentRequest.automl_params][google.cloud.documentai.v1beta2.ProcessDocumentRequest.automl_params\] field set. The JSON file needs to
    /// be in \[Document][google.cloud.documentai.v1beta2.Document\] format.
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
    /// Required.
    #[prost(oneof = "input_config::Source", tags = "1, 3")]
    pub source: ::core::option::Option<input_config::Source>,
}
/// Nested message and enum types in `InputConfig`.
pub mod input_config {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Google Cloud Storage location to read the input from. This must be a
        /// single file.
        #[prost(message, tag = "1")]
        GcsSource(super::GcsSource),
        /// Content in bytes, represented as a stream of bytes.
        /// Note: As with all `bytes` fields, proto buffer messages use a pure binary
        /// representation, whereas JSON representations use base64.
        ///
        /// This field only works for synchronous ProcessDocument method.
        #[prost(bytes, tag = "3")]
        Contents(::prost::alloc::vec::Vec<u8>),
    }
}
/// The desired output location and metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// The max number of pages to include into each output Document shard JSON on
    /// Google Cloud Storage.
    ///
    /// The valid range is [1, 100]. If not specified, the default value is 20.
    ///
    /// For example, for one pdf file with 100 pages, 100 parsed pages will be
    /// produced. If `pages_per_shard` = 20, then 5 Document shard JSON files each
    /// containing 20 parsed pages will be written under the prefix
    /// \[OutputConfig.gcs_destination.uri][\] and suffix pages-x-to-y.json where
    /// x and y are 1-indexed page numbers.
    ///
    /// Example GCS outputs with 157 pages and pages_per_shard = 50:
    ///
    /// <prefix>pages-001-to-050.json
    /// <prefix>pages-051-to-100.json
    /// <prefix>pages-101-to-150.json
    /// <prefix>pages-151-to-157.json
    #[prost(int32, tag = "2")]
    pub pages_per_shard: i32,
    /// Required.
    #[prost(oneof = "output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// The Google Cloud Storage location to write the output to.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// The Google Cloud Storage location where the input file will be read from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// The Google Cloud Storage location where the output file will be written to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
/// Contains metadata for the BatchProcessDocuments operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The state of the current batch processing.
    #[prost(enumeration = "operation_metadata::State", tag = "1")]
    pub state: i32,
    /// A message providing more details about the current state of processing.
    #[prost(string, tag = "2")]
    pub state_message: ::prost::alloc::string::String,
    /// The creation time of the operation.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last update time of the operation.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// Request is received.
        Accepted = 1,
        /// Request operation is waiting for scheduling.
        Waiting = 2,
        /// Request is being processed.
        Running = 3,
        /// The batch processing completed successfully.
        Succeeded = 4,
        /// The batch processing was cancelled.
        Cancelled = 5,
        /// The batch processing has failed.
        Failed = 6,
    }
}
#[doc = r" Generated client implementations."]
pub mod document_understanding_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to parse structured information from unstructured or semi-structured"]
    #[doc = " documents using state-of-the-art Google AI such as natural language,"]
    #[doc = " computer vision, and translation."]
    #[derive(Debug, Clone)]
    pub struct DocumentUnderstandingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DocumentUnderstandingServiceClient<T>
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
        ) -> DocumentUnderstandingServiceClient<InterceptedService<T, F>>
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
            DocumentUnderstandingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " LRO endpoint to batch process many documents. The output is written"]
        #[doc = " to Cloud Storage as JSON in the [Document] format."]
        pub async fn batch_process_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchProcessDocumentsRequest>,
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
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.documentai.v1beta2.DocumentUnderstandingService/BatchProcessDocuments") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Processes a single document."]
        pub async fn process_document(
            &mut self,
            request: impl tonic::IntoRequest<super::ProcessDocumentRequest>,
        ) -> Result<tonic::Response<super::Document>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1beta2.DocumentUnderstandingService/ProcessDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
