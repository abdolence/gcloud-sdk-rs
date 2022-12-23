/// Encodes the detailed information of a barcode.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Barcode {
    /// Format of a barcode.
    /// The supported formats are:
    ///
    /// - `CODE_128`: Code 128 type.
    /// - `CODE_39`: Code 39 type.
    /// - `CODE_93`: Code 93 type.
    /// - `CODABAR`: Codabar type.
    /// - `DATA_MATRIX`: 2D Data Matrix type.
    /// - `ITF`: ITF type.
    /// - `EAN_13`: EAN-13 type.
    /// - `EAN_8`: EAN-8 type.
    /// - `QR_CODE`: 2D QR code type.
    /// - `UPC_A`: UPC-A type.
    /// - `UPC_E`: UPC-E type.
    /// - `PDF417`: PDF417 type.
    /// - `AZTEC`: 2D Aztec code type.
    /// - `DATABAR`: GS1 DataBar code type.
    #[prost(string, tag = "1")]
    pub format: ::prost::alloc::string::String,
    /// Value format describes the format of the value that a barcode
    /// encodes.
    /// The supported formats are:
    ///
    /// - `CONTACT_INFO`: Contact information.
    /// - `EMAIL`: Email address.
    /// - `ISBN`: ISBN identifier.
    /// - `PHONE`: Phone number.
    /// - `PRODUCT`: Product.
    /// - `SMS`: SMS message.
    /// - `TEXT`: Text string.
    /// - `URL`: URL address.
    /// - `WIFI`: Wifi information.
    /// - `GEO`: Geo-localization.
    /// - `CALENDAR_EVENT`: Calendar event.
    /// - `DRIVER_LICENSE`: Driver's license.
    #[prost(string, tag = "2")]
    pub value_format: ::prost::alloc::string::String,
    /// Raw value encoded in the barcode.
    /// For example: `'MEBKM:TITLE:Google;URL:<https://www.google.com;;'`.>
    #[prost(string, tag = "3")]
    pub raw_value: ::prost::alloc::string::String,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the vertex coordinates are in the same scale as the original image.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    /// X coordinate.
    #[prost(int32, tag = "1")]
    pub x: i32,
    /// Y coordinate (starts from the top of the image).
    #[prost(int32, tag = "2")]
    pub y: i32,
}
/// A vertex represents a 2D point in the image.
/// NOTE: the normalized vertex coordinates are relative to the original image
/// and range from 0 to 1.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedVertex {
    /// X coordinate.
    #[prost(float, tag = "1")]
    pub x: f32,
    /// Y coordinate (starts from the top of the image).
    #[prost(float, tag = "2")]
    pub y: f32,
}
/// A bounding polygon for the detected image annotation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingPoly {
    /// The bounding polygon vertices.
    #[prost(message, repeated, tag = "1")]
    pub vertices: ::prost::alloc::vec::Vec<Vertex>,
    /// The bounding polygon normalized vertices.
    #[prost(message, repeated, tag = "2")]
    pub normalized_vertices: ::prost::alloc::vec::Vec<NormalizedVertex>,
}
/// Document represents the canonical document resource in Document AI. It is an
/// interchange format that provides insights into documents and allows for
/// collaboration between users and Document AI to iterate and optimize for
/// quality.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Document {
    /// An IANA published MIME type (also referred to as media type). For more
    /// information, see
    /// <https://www.iana.org/assignments/media-types/media-types.xhtml.>
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
    /// Optional. UTF-8 encoded text in reading order from the document.
    #[prost(string, tag = "4")]
    pub text: ::prost::alloc::string::String,
    /// Styles for the \[Document.text][google.cloud.documentai.v1.Document.text\].
    #[deprecated]
    #[prost(message, repeated, tag = "5")]
    pub text_styles: ::prost::alloc::vec::Vec<document::Style>,
    /// Visual page layout for the \[Document][google.cloud.documentai.v1.Document\].
    #[prost(message, repeated, tag = "6")]
    pub pages: ::prost::alloc::vec::Vec<document::Page>,
    /// A list of entities detected on
    /// \[Document.text][google.cloud.documentai.v1.Document.text\]. For document
    /// shards, entities in this list may cross shard boundaries.
    #[prost(message, repeated, tag = "7")]
    pub entities: ::prost::alloc::vec::Vec<document::Entity>,
    /// Placeholder.  Relationship among
    /// \[Document.entities][google.cloud.documentai.v1.Document.entities\].
    #[prost(message, repeated, tag = "8")]
    pub entity_relations: ::prost::alloc::vec::Vec<document::EntityRelation>,
    /// Placeholder.  A list of text corrections made to
    /// \[Document.text][google.cloud.documentai.v1.Document.text\].  This is usually
    /// used for annotating corrections to OCR mistakes.  Text changes for a given
    /// revision may not overlap with each other.
    #[prost(message, repeated, tag = "14")]
    pub text_changes: ::prost::alloc::vec::Vec<document::TextChange>,
    /// Information about the sharding if this document is sharded part of a larger
    /// document. If the document is not sharded, this message is not specified.
    #[prost(message, optional, tag = "9")]
    pub shard_info: ::core::option::Option<document::ShardInfo>,
    /// Any error that occurred while processing this document.
    #[prost(message, optional, tag = "10")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Placeholder. Revision history of this document.
    #[prost(message, repeated, tag = "13")]
    pub revisions: ::prost::alloc::vec::Vec<document::Revision>,
    /// Original source document from the user.
    #[prost(oneof = "document::Source", tags = "1, 2")]
    pub source: ::core::option::Option<document::Source>,
}
/// Nested message and enum types in `Document`.
pub mod document {
    /// For a large document, sharding may be performed to produce several
    /// document shards. Each document shard contains this field to detail which
    /// shard it is.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShardInfo {
        /// The 0-based index of this shard.
        #[prost(int64, tag = "1")]
        pub shard_index: i64,
        /// Total number of shards.
        #[prost(int64, tag = "2")]
        pub shard_count: i64,
        /// The index of the first character in
        /// \[Document.text][google.cloud.documentai.v1.Document.text\] in the overall
        /// document global text.
        #[prost(int64, tag = "3")]
        pub text_offset: i64,
    }
    /// Annotation for common text style attributes. This adheres to CSS
    /// conventions as much as possible.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Style {
        /// Text anchor indexing into the
        /// \[Document.text][google.cloud.documentai.v1.Document.text\].
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// Text color.
        #[prost(message, optional, tag = "2")]
        pub color: ::core::option::Option<super::super::super::super::r#type::Color>,
        /// Text background color.
        #[prost(message, optional, tag = "3")]
        pub background_color: ::core::option::Option<
            super::super::super::super::r#type::Color,
        >,
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
        /// Font family such as `Arial`, `Times New Roman`.
        /// <https://www.w3schools.com/cssref/pr_font_font-family.asp>
        #[prost(string, tag = "8")]
        pub font_family: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `Style`.
    pub mod style {
        /// Font size with unit.
        #[allow(clippy::derive_partial_eq_without_eq)]
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
    /// A page in a \[Document][google.cloud.documentai.v1.Document\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Page {
        /// 1-based index for current
        /// \[Page][google.cloud.documentai.v1.Document.Page\] in a parent
        /// \[Document][google.cloud.documentai.v1.Document\]. Useful when a page is
        /// taken out of a \[Document][google.cloud.documentai.v1.Document\] for
        /// individual processing.
        #[prost(int32, tag = "1")]
        pub page_number: i32,
        /// Rendered image for this page. This image is preprocessed to remove any
        /// skew, rotation, and distortions such that the annotation bounding boxes
        /// can be upright and axis-aligned.
        #[prost(message, optional, tag = "13")]
        pub image: ::core::option::Option<page::Image>,
        /// Transformation matrices that were applied to the original document image
        /// to produce \[Page.image][google.cloud.documentai.v1.Document.Page.image\].
        #[prost(message, repeated, tag = "14")]
        pub transforms: ::prost::alloc::vec::Vec<page::Matrix>,
        /// Physical dimension of the page.
        #[prost(message, optional, tag = "2")]
        pub dimension: ::core::option::Option<page::Dimension>,
        /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for the page.
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
        /// A list of visually detected symbols on the page.
        #[prost(message, repeated, tag = "12")]
        pub symbols: ::prost::alloc::vec::Vec<page::Symbol>,
        /// A list of detected barcodes.
        #[prost(message, repeated, tag = "15")]
        pub detected_barcodes: ::prost::alloc::vec::Vec<page::DetectedBarcode>,
        /// Image Quality Scores.
        #[prost(message, optional, tag = "17")]
        pub image_quality_scores: ::core::option::Option<page::ImageQualityScores>,
        /// The history of this page.
        #[deprecated]
        #[prost(message, optional, tag = "16")]
        pub provenance: ::core::option::Option<Provenance>,
    }
    /// Nested message and enum types in `Page`.
    pub mod page {
        /// Dimension for the page.
        #[allow(clippy::derive_partial_eq_without_eq)]
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
        /// Rendered image contents for this page.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Image {
            /// Raw byte content of the image.
            #[prost(bytes = "vec", tag = "1")]
            pub content: ::prost::alloc::vec::Vec<u8>,
            /// Encoding mime type for the image.
            #[prost(string, tag = "2")]
            pub mime_type: ::prost::alloc::string::String,
            /// Width of the image in pixels.
            #[prost(int32, tag = "3")]
            pub width: i32,
            /// Height of the image in pixels.
            #[prost(int32, tag = "4")]
            pub height: i32,
        }
        /// Representation for transformation matrix, intended to be compatible and
        /// used with OpenCV format for image manipulation.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Matrix {
            /// Number of rows in the matrix.
            #[prost(int32, tag = "1")]
            pub rows: i32,
            /// Number of columns in the matrix.
            #[prost(int32, tag = "2")]
            pub cols: i32,
            /// This encodes information about what data type the matrix uses.
            /// For example, 0 (CV_8U) is an unsigned 8-bit image. For the full list
            /// of OpenCV primitive data types, please refer to
            /// <https://docs.opencv.org/4.3.0/d1/d1b/group__core__hal__interface.html>
            #[prost(int32, tag = "3")]
            pub r#type: i32,
            /// The matrix data.
            #[prost(bytes = "vec", tag = "4")]
            pub data: ::prost::alloc::vec::Vec<u8>,
        }
        /// Visual element describing a layout unit on a page.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Layout {
            /// Text anchor indexing into the
            /// \[Document.text][google.cloud.documentai.v1.Document.text\].
            #[prost(message, optional, tag = "1")]
            pub text_anchor: ::core::option::Option<super::TextAnchor>,
            /// Confidence of the current
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] within
            /// context of the object this layout is for. e.g. confidence can be for a
            /// single token, a table, a visual element, etc. depending on context.
            /// Range `[0, 1]`.
            #[prost(float, tag = "2")]
            pub confidence: f32,
            /// The bounding polygon for the
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\].
            #[prost(message, optional, tag = "3")]
            pub bounding_poly: ::core::option::Option<super::super::BoundingPoly>,
            /// Detected orientation for the
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\].
            #[prost(enumeration = "layout::Orientation", tag = "4")]
            pub orientation: i32,
        }
        /// Nested message and enum types in `Layout`.
        pub mod layout {
            /// Detected human reading orientation.
            #[derive(
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                ::prost::Enumeration
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
            impl Orientation {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Orientation::Unspecified => "ORIENTATION_UNSPECIFIED",
                        Orientation::PageUp => "PAGE_UP",
                        Orientation::PageRight => "PAGE_RIGHT",
                        Orientation::PageDown => "PAGE_DOWN",
                        Orientation::PageLeft => "PAGE_LEFT",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "ORIENTATION_UNSPECIFIED" => Some(Self::Unspecified),
                        "PAGE_UP" => Some(Self::PageUp),
                        "PAGE_RIGHT" => Some(Self::PageRight),
                        "PAGE_DOWN" => Some(Self::PageDown),
                        "PAGE_LEFT" => Some(Self::PageLeft),
                        _ => None,
                    }
                }
            }
        }
        /// A block has a set of lines (collected into paragraphs) that have a
        /// common line-spacing and orientation.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Block {
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for
            /// \[Block][google.cloud.documentai.v1.Document.Page.Block\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The history of this annotation.
            #[deprecated]
            #[prost(message, optional, tag = "3")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A collection of lines that a human would perceive as a paragraph.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Paragraph {
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for
            /// \[Paragraph][google.cloud.documentai.v1.Document.Page.Paragraph\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The  history of this annotation.
            #[deprecated]
            #[prost(message, optional, tag = "3")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A collection of tokens that a human would perceive as a line.
        /// Does not cross column boundaries, can be horizontal, vertical, etc.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Line {
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for
            /// \[Line][google.cloud.documentai.v1.Document.Page.Line\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The  history of this annotation.
            #[deprecated]
            #[prost(message, optional, tag = "3")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A detected token.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Token {
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for
            /// \[Token][google.cloud.documentai.v1.Document.Page.Token\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Detected break at the end of a
            /// \[Token][google.cloud.documentai.v1.Document.Page.Token\].
            #[prost(message, optional, tag = "2")]
            pub detected_break: ::core::option::Option<token::DetectedBreak>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
            /// The history of this annotation.
            #[deprecated]
            #[prost(message, optional, tag = "4")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// Nested message and enum types in `Token`.
        pub mod token {
            /// Detected break at the end of a
            /// \[Token][google.cloud.documentai.v1.Document.Page.Token\].
            #[allow(clippy::derive_partial_eq_without_eq)]
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
                    Clone,
                    Copy,
                    Debug,
                    PartialEq,
                    Eq,
                    Hash,
                    PartialOrd,
                    Ord,
                    ::prost::Enumeration
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
                impl Type {
                    /// String value of the enum field names used in the ProtoBuf definition.
                    ///
                    /// The values are not transformed in any way and thus are considered stable
                    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                    pub fn as_str_name(&self) -> &'static str {
                        match self {
                            Type::Unspecified => "TYPE_UNSPECIFIED",
                            Type::Space => "SPACE",
                            Type::WideSpace => "WIDE_SPACE",
                            Type::Hyphen => "HYPHEN",
                        }
                    }
                    /// Creates an enum from field names used in the ProtoBuf definition.
                    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                        match value {
                            "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                            "SPACE" => Some(Self::Space),
                            "WIDE_SPACE" => Some(Self::WideSpace),
                            "HYPHEN" => Some(Self::Hyphen),
                            _ => None,
                        }
                    }
                }
            }
        }
        /// A detected symbol.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Symbol {
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for
            /// \[Symbol][google.cloud.documentai.v1.Document.Page.Symbol\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "2")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// Detected non-text visual elements e.g. checkbox, signature etc. on the
        /// page.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct VisualElement {
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for
            /// \[VisualElement][google.cloud.documentai.v1.Document.Page.VisualElement\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Type of the
            /// \[VisualElement][google.cloud.documentai.v1.Document.Page.VisualElement\].
            #[prost(string, tag = "2")]
            pub r#type: ::prost::alloc::string::String,
            /// A list of detected languages together with confidence.
            #[prost(message, repeated, tag = "3")]
            pub detected_languages: ::prost::alloc::vec::Vec<DetectedLanguage>,
        }
        /// A table representation similar to HTML table structure.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Table {
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for
            /// \[Table][google.cloud.documentai.v1.Document.Page.Table\].
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
            /// The history of this table.
            #[prost(message, optional, tag = "5")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// Nested message and enum types in `Table`.
        pub mod table {
            /// A row of table cells.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct TableRow {
                /// Cells that make up this row.
                #[prost(message, repeated, tag = "1")]
                pub cells: ::prost::alloc::vec::Vec<TableCell>,
            }
            /// A cell representation inside the table.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct TableCell {
                /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for
                /// \[TableCell][google.cloud.documentai.v1.Document.Page.Table.TableCell\].
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
                pub detected_languages: ::prost::alloc::vec::Vec<
                    super::DetectedLanguage,
                >,
            }
        }
        /// A form field detected on the page.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FormField {
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for the
            /// \[FormField][google.cloud.documentai.v1.Document.Page.FormField\] name.
            /// e.g. `Address`, `Email`, `Grand total`, `Phone number`, etc.
            #[prost(message, optional, tag = "1")]
            pub field_name: ::core::option::Option<Layout>,
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for the
            /// \[FormField][google.cloud.documentai.v1.Document.Page.FormField\] value.
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
            ///
            /// - blank (this indicates the `field_value` is normal text)
            /// - `unfilled_checkbox`
            /// - `filled_checkbox`
            #[prost(string, tag = "5")]
            pub value_type: ::prost::alloc::string::String,
            /// Created for Labeling UI to export key text.
            /// If corrections were made to the text identified by the
            /// `field_name.text_anchor`, this field will contain the correction.
            #[prost(string, tag = "6")]
            pub corrected_key_text: ::prost::alloc::string::String,
            /// Created for Labeling UI to export value text.
            /// If corrections were made to the text identified by the
            /// `field_value.text_anchor`, this field will contain the correction.
            #[prost(string, tag = "7")]
            pub corrected_value_text: ::prost::alloc::string::String,
            /// The history of this annotation.
            #[prost(message, optional, tag = "8")]
            pub provenance: ::core::option::Option<super::Provenance>,
        }
        /// A detected barcode.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DetectedBarcode {
            /// \[Layout][google.cloud.documentai.v1.Document.Page.Layout\] for
            /// \[DetectedBarcode][google.cloud.documentai.v1.Document.Page.DetectedBarcode\].
            #[prost(message, optional, tag = "1")]
            pub layout: ::core::option::Option<Layout>,
            /// Detailed barcode information of the
            /// \[DetectedBarcode][google.cloud.documentai.v1.Document.Page.DetectedBarcode\].
            #[prost(message, optional, tag = "2")]
            pub barcode: ::core::option::Option<super::super::Barcode>,
        }
        /// Detected language for a structural component.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DetectedLanguage {
            /// The BCP-47 language code, such as `en-US` or `sr-Latn`. For more
            /// information, see
            /// <https://www.unicode.org/reports/tr35/#Unicode_locale_identifier.>
            #[prost(string, tag = "1")]
            pub language_code: ::prost::alloc::string::String,
            /// Confidence of detected language. Range `[0, 1]`.
            #[prost(float, tag = "2")]
            pub confidence: f32,
        }
        /// Image Quality Scores for the page image
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ImageQualityScores {
            /// The overall quality score. Range `[0, 1]` where 1 is perfect quality.
            #[prost(float, tag = "1")]
            pub quality_score: f32,
            /// A list of detected defects.
            #[prost(message, repeated, tag = "2")]
            pub detected_defects: ::prost::alloc::vec::Vec<
                image_quality_scores::DetectedDefect,
            >,
        }
        /// Nested message and enum types in `ImageQualityScores`.
        pub mod image_quality_scores {
            /// Image Quality Defects
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct DetectedDefect {
                /// Name of the defect type. Supported values are:
                ///
                /// - `quality/defect_blurry`
                /// - `quality/defect_noisy`
                /// - `quality/defect_dark`
                /// - `quality/defect_faint`
                /// - `quality/defect_text_too_small`
                /// - `quality/defect_document_cutoff`
                /// - `quality/defect_text_cutoff`
                /// - `quality/defect_glare`
                #[prost(string, tag = "1")]
                pub r#type: ::prost::alloc::string::String,
                /// Confidence of detected defect. Range `[0, 1]` where 1 indicates
                /// strong confidence of that the defect exists.
                #[prost(float, tag = "2")]
                pub confidence: f32,
            }
        }
    }
    /// An entity that could be a phrase in the text or a property that belongs to
    /// the document. It is a known entity type, such as a person, an organization,
    /// or location.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Optional. Provenance of the entity.
        /// Text anchor indexing into the
        /// \[Document.text][google.cloud.documentai.v1.Document.text\].
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// Required. Entity type from a schema e.g. `Address`.
        #[prost(string, tag = "2")]
        pub r#type: ::prost::alloc::string::String,
        /// Optional. Text value of the entity e.g. `1600 Amphitheatre Pkwy`.
        #[prost(string, tag = "3")]
        pub mention_text: ::prost::alloc::string::String,
        /// Optional. Deprecated.  Use `id` field instead.
        #[prost(string, tag = "4")]
        pub mention_id: ::prost::alloc::string::String,
        /// Optional. Confidence of detected Schema entity. Range `[0, 1]`.
        #[prost(float, tag = "5")]
        pub confidence: f32,
        /// Optional. Represents the provenance of this entity wrt. the location on
        /// the page where it was found.
        #[prost(message, optional, tag = "6")]
        pub page_anchor: ::core::option::Option<PageAnchor>,
        /// Optional. Canonical id. This will be a unique value in the entity list
        /// for this document.
        #[prost(string, tag = "7")]
        pub id: ::prost::alloc::string::String,
        /// Optional. Normalized entity value. Absent if the extracted value could
        /// not be converted or the type (e.g. address) is not supported for certain
        /// parsers. This field is also only populated for certain supported document
        /// types.
        #[prost(message, optional, tag = "9")]
        pub normalized_value: ::core::option::Option<entity::NormalizedValue>,
        /// Optional. Entities can be nested to form a hierarchical data structure
        /// representing the content in the document.
        #[prost(message, repeated, tag = "10")]
        pub properties: ::prost::alloc::vec::Vec<Entity>,
        /// Optional. The history of this annotation.
        #[prost(message, optional, tag = "11")]
        pub provenance: ::core::option::Option<Provenance>,
        /// Optional. Whether the entity will be redacted for de-identification
        /// purposes.
        #[prost(bool, tag = "12")]
        pub redacted: bool,
    }
    /// Nested message and enum types in `Entity`.
    pub mod entity {
        /// Parsed and normalized entity value.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct NormalizedValue {
            /// Optional. An optional field to store a normalized string.
            /// For some entity types, one of respective `structured_value` fields may
            /// also be populated. Also not all the types of `structured_value` will be
            /// normalized. For example, some processors may not generate `float`
            /// or `integer` normalized text by default.
            ///
            /// Below are sample formats mapped to structured values.
            ///
            /// - Money/Currency type (`money_value`) is in the ISO 4217 text format.
            /// - Date type (`date_value`) is in the ISO 8601 text format.
            /// - Datetime type (`datetime_value`) is in the ISO 8601 text format.
            #[prost(string, tag = "1")]
            pub text: ::prost::alloc::string::String,
            /// An optional structured entity value.
            /// Must match entity type defined in schema if
            /// known. If this field is present, the `text` field could also be
            /// populated.
            #[prost(
                oneof = "normalized_value::StructuredValue",
                tags = "2, 3, 4, 5, 6, 7, 8"
            )]
            pub structured_value: ::core::option::Option<
                normalized_value::StructuredValue,
            >,
        }
        /// Nested message and enum types in `NormalizedValue`.
        pub mod normalized_value {
            /// An optional structured entity value.
            /// Must match entity type defined in schema if
            /// known. If this field is present, the `text` field could also be
            /// populated.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum StructuredValue {
                /// Money value. See also:
                /// <https://github.com/googleapis/googleapis/blob/master/google/type/money.proto>
                #[prost(message, tag = "2")]
                MoneyValue(super::super::super::super::super::super::r#type::Money),
                /// Date value. Includes year, month, day. See also:
                /// <https://github.com/googleapis/googleapis/blob/master/google/type/date.proto>
                #[prost(message, tag = "3")]
                DateValue(super::super::super::super::super::super::r#type::Date),
                /// DateTime value. Includes date, time, and timezone. See also:
                /// <https://github.com/googleapis/googleapis/blob/master/google/type/datetime.proto>
                #[prost(message, tag = "4")]
                DatetimeValue(
                    super::super::super::super::super::super::r#type::DateTime,
                ),
                /// Postal address. See also:
                /// <https://github.com/googleapis/googleapis/blob/master/google/type/postal_address.proto>
                #[prost(message, tag = "5")]
                AddressValue(
                    super::super::super::super::super::super::r#type::PostalAddress,
                ),
                /// Boolean value. Can be used for entities with binary values, or for
                /// checkboxes.
                #[prost(bool, tag = "6")]
                BooleanValue(bool),
                /// Integer value.
                #[prost(int32, tag = "7")]
                IntegerValue(i32),
                /// Float value.
                #[prost(float, tag = "8")]
                FloatValue(f32),
            }
        }
    }
    /// Relationship between
    /// \[Entities][google.cloud.documentai.v1.Document.Entity\].
    #[allow(clippy::derive_partial_eq_without_eq)]
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
    /// Text reference indexing into the
    /// \[Document.text][google.cloud.documentai.v1.Document.text\].
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextAnchor {
        /// The text segments from the
        /// \[Document.text][google.cloud.documentai.v1.Document.text\].
        #[prost(message, repeated, tag = "1")]
        pub text_segments: ::prost::alloc::vec::Vec<text_anchor::TextSegment>,
        /// Contains the content of the text span so that users do
        /// not have to look it up in the text_segments.  It is always
        /// populated for formFields.
        #[prost(string, tag = "2")]
        pub content: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `TextAnchor`.
    pub mod text_anchor {
        /// A text segment in the
        /// \[Document.text][google.cloud.documentai.v1.Document.text\]. The indices
        /// may be out of bounds which indicate that the text extends into another
        /// document shard for large sharded documents. See
        /// \[ShardInfo.text_offset][google.cloud.documentai.v1.Document.ShardInfo.text_offset\]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TextSegment {
            /// \[TextSegment][google.cloud.documentai.v1.Document.TextAnchor.TextSegment\]
            /// start UTF-8 char index in the
            /// \[Document.text][google.cloud.documentai.v1.Document.text\].
            #[prost(int64, tag = "1")]
            pub start_index: i64,
            /// \[TextSegment][google.cloud.documentai.v1.Document.TextAnchor.TextSegment\]
            /// half open end UTF-8 char index in the
            /// \[Document.text][google.cloud.documentai.v1.Document.text\].
            #[prost(int64, tag = "2")]
            pub end_index: i64,
        }
    }
    /// Referencing the visual context of the entity in the
    /// \[Document.pages][google.cloud.documentai.v1.Document.pages\]. Page anchors
    /// can be cross-page, consist of multiple bounding polygons and optionally
    /// reference specific layout element types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PageAnchor {
        /// One or more references to visual page elements
        #[prost(message, repeated, tag = "1")]
        pub page_refs: ::prost::alloc::vec::Vec<page_anchor::PageRef>,
    }
    /// Nested message and enum types in `PageAnchor`.
    pub mod page_anchor {
        /// Represents a weak reference to a page element within a document.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PageRef {
            /// Required. Index into the
            /// \[Document.pages][google.cloud.documentai.v1.Document.pages\] element,
            /// for example using
            /// `\[Document.pages][page_refs.page\]` to locate the related page element.
            /// This field is skipped when its value is the default `0`. See
            /// <https://developers.google.com/protocol-buffers/docs/proto3#json.>
            #[prost(int64, tag = "1")]
            pub page: i64,
            /// Optional. The type of the layout element that is being referenced if
            /// any.
            #[prost(enumeration = "page_ref::LayoutType", tag = "2")]
            pub layout_type: i32,
            /// Optional. Deprecated.  Use
            /// \[PageRef.bounding_poly][google.cloud.documentai.v1.Document.PageAnchor.PageRef.bounding_poly\]
            /// instead.
            #[deprecated]
            #[prost(string, tag = "3")]
            pub layout_id: ::prost::alloc::string::String,
            /// Optional. Identifies the bounding polygon of a layout element on the
            /// page.
            #[prost(message, optional, tag = "4")]
            pub bounding_poly: ::core::option::Option<super::super::BoundingPoly>,
            /// Optional. Confidence of detected page element, if applicable. Range
            /// `[0, 1]`.
            #[prost(float, tag = "5")]
            pub confidence: f32,
        }
        /// Nested message and enum types in `PageRef`.
        pub mod page_ref {
            /// The type of layout that is being referenced.
            #[derive(
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                ::prost::Enumeration
            )]
            #[repr(i32)]
            pub enum LayoutType {
                /// Layout Unspecified.
                Unspecified = 0,
                /// References a
                /// \[Page.blocks][google.cloud.documentai.v1.Document.Page.blocks\]
                /// element.
                Block = 1,
                /// References a
                /// \[Page.paragraphs][google.cloud.documentai.v1.Document.Page.paragraphs\]
                /// element.
                Paragraph = 2,
                /// References a
                /// \[Page.lines][google.cloud.documentai.v1.Document.Page.lines\] element.
                Line = 3,
                /// References a
                /// \[Page.tokens][google.cloud.documentai.v1.Document.Page.tokens\]
                /// element.
                Token = 4,
                /// References a
                /// \[Page.visual_elements][google.cloud.documentai.v1.Document.Page.visual_elements\]
                /// element.
                VisualElement = 5,
                /// Refrrences a
                /// \[Page.tables][google.cloud.documentai.v1.Document.Page.tables\]
                /// element.
                Table = 6,
                /// References a
                /// \[Page.form_fields][google.cloud.documentai.v1.Document.Page.form_fields\]
                /// element.
                FormField = 7,
            }
            impl LayoutType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        LayoutType::Unspecified => "LAYOUT_TYPE_UNSPECIFIED",
                        LayoutType::Block => "BLOCK",
                        LayoutType::Paragraph => "PARAGRAPH",
                        LayoutType::Line => "LINE",
                        LayoutType::Token => "TOKEN",
                        LayoutType::VisualElement => "VISUAL_ELEMENT",
                        LayoutType::Table => "TABLE",
                        LayoutType::FormField => "FORM_FIELD",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "LAYOUT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                        "BLOCK" => Some(Self::Block),
                        "PARAGRAPH" => Some(Self::Paragraph),
                        "LINE" => Some(Self::Line),
                        "TOKEN" => Some(Self::Token),
                        "VISUAL_ELEMENT" => Some(Self::VisualElement),
                        "TABLE" => Some(Self::Table),
                        "FORM_FIELD" => Some(Self::FormField),
                        _ => None,
                    }
                }
            }
        }
    }
    /// Structure to identify provenance relationships between annotations in
    /// different revisions.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Provenance {
        /// The index of the revision that produced this element.
        #[prost(int32, tag = "1")]
        pub revision: i32,
        /// The Id of this operation.  Needs to be unique within the scope of the
        /// revision.
        #[deprecated]
        #[prost(int32, tag = "2")]
        pub id: i32,
        /// References to the original elements that are replaced.
        #[prost(message, repeated, tag = "3")]
        pub parents: ::prost::alloc::vec::Vec<provenance::Parent>,
        /// The type of provenance operation.
        #[prost(enumeration = "provenance::OperationType", tag = "4")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `Provenance`.
    pub mod provenance {
        /// The parent element the current element is based on. Used for
        /// referencing/aligning, removal and replacement operations.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Parent {
            /// The index of the index into current revision's parent_ids list.
            #[prost(int32, tag = "1")]
            pub revision: i32,
            /// The index of the parent item in the corresponding item list (eg. list
            /// of entities, properties within entities, etc.) in the parent revision.
            #[prost(int32, tag = "3")]
            pub index: i32,
            /// The id of the parent provenance.
            #[deprecated]
            #[prost(int32, tag = "2")]
            pub id: i32,
        }
        /// If a processor or agent does an explicit operation on existing elements.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum OperationType {
            /// Operation type unspecified. If no operation is specified a provenance
            /// entry is simply used to match against a `parent`.
            Unspecified = 0,
            /// Add an element.
            Add = 1,
            /// Remove an element identified by `parent`.
            Remove = 2,
            /// Replace an element identified by `parent`.
            Replace = 3,
            /// Request human review for the element identified by `parent`.
            EvalRequested = 4,
            /// Element is reviewed and approved at human review, confidence will be
            /// set to 1.0.
            EvalApproved = 5,
            /// Element is skipped in the validation process.
            EvalSkipped = 6,
        }
        impl OperationType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    OperationType::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
                    OperationType::Add => "ADD",
                    OperationType::Remove => "REMOVE",
                    OperationType::Replace => "REPLACE",
                    OperationType::EvalRequested => "EVAL_REQUESTED",
                    OperationType::EvalApproved => "EVAL_APPROVED",
                    OperationType::EvalSkipped => "EVAL_SKIPPED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "ADD" => Some(Self::Add),
                    "REMOVE" => Some(Self::Remove),
                    "REPLACE" => Some(Self::Replace),
                    "EVAL_REQUESTED" => Some(Self::EvalRequested),
                    "EVAL_APPROVED" => Some(Self::EvalApproved),
                    "EVAL_SKIPPED" => Some(Self::EvalSkipped),
                    _ => None,
                }
            }
        }
    }
    /// Contains past or forward revisions of this document.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Revision {
        /// Id of the revision.  Unique within the context of the document.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The revisions that this revision is based on.  This can include one or
        /// more parent (when documents are merged.)  This field represents the
        /// index into the `revisions` field.
        #[deprecated]
        #[prost(int32, repeated, packed = "false", tag = "2")]
        pub parent: ::prost::alloc::vec::Vec<i32>,
        /// The revisions that this revision is based on. Must include all the ids
        /// that have anything to do with this revision - eg. there are
        /// `provenance.parent.revision` fields that index into this field.
        #[prost(string, repeated, tag = "7")]
        pub parent_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The time that the revision was created.
        #[prost(message, optional, tag = "3")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Human Review information of this revision.
        #[prost(message, optional, tag = "6")]
        pub human_review: ::core::option::Option<revision::HumanReview>,
        /// Who/what made the change
        #[prost(oneof = "revision::Source", tags = "4, 5")]
        pub source: ::core::option::Option<revision::Source>,
    }
    /// Nested message and enum types in `Revision`.
    pub mod revision {
        /// Human Review information of the document.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct HumanReview {
            /// Human review state. e.g. `requested`, `succeeded`, `rejected`.
            #[prost(string, tag = "1")]
            pub state: ::prost::alloc::string::String,
            /// A message providing more details about the current state of processing.
            /// For example, the rejection reason when the state is `rejected`.
            #[prost(string, tag = "2")]
            pub state_message: ::prost::alloc::string::String,
        }
        /// Who/what made the change
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Source {
            /// If the change was made by a person specify the name or id of that
            /// person.
            #[prost(string, tag = "4")]
            Agent(::prost::alloc::string::String),
            /// If the annotation was made by processor identify the processor by its
            /// resource name.
            #[prost(string, tag = "5")]
            Processor(::prost::alloc::string::String),
        }
    }
    /// This message is used for text changes aka. OCR corrections.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextChange {
        /// Provenance of the correction.
        /// Text anchor indexing into the
        /// \[Document.text][google.cloud.documentai.v1.Document.text\].  There can
        /// only be a single `TextAnchor.text_segments` element.  If the start and
        /// end index of the text segment are the same, the text change is inserted
        /// before that index.
        #[prost(message, optional, tag = "1")]
        pub text_anchor: ::core::option::Option<TextAnchor>,
        /// The text that replaces the text identified in the `text_anchor`.
        #[prost(string, tag = "2")]
        pub changed_text: ::prost::alloc::string::String,
        /// The history of this annotation.
        #[deprecated]
        #[prost(message, repeated, tag = "3")]
        pub provenance: ::prost::alloc::vec::Vec<Provenance>,
    }
    /// Original source document from the user.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Optional. Currently supports Google Cloud Storage URI of the form
        ///     `gs://bucket_name/object_name`. Object versioning is not supported.
        ///     See [Google Cloud Storage Request
        ///     URIs](<https://cloud.google.com/storage/docs/reference-uris>) for more
        ///     info.
        #[prost(string, tag = "1")]
        Uri(::prost::alloc::string::String),
        /// Optional. Inline document content, represented as a stream of bytes.
        /// Note: As with all `bytes` fields, protobuffers use a pure binary
        /// representation, whereas JSON representations use base64.
        #[prost(bytes, tag = "2")]
        Content(::prost::alloc::vec::Vec<u8>),
    }
}
/// Payload message of raw document content (bytes).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDocument {
    /// Inline document content.
    #[prost(bytes = "vec", tag = "1")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    /// An IANA MIME type (RFC6838) indicating the nature and format of the
    /// \[content][google.cloud.documentai.v1.RawDocument.content\].
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Specifies a document stored on Cloud Storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDocument {
    /// The Cloud Storage object uri.
    #[prost(string, tag = "1")]
    pub gcs_uri: ::prost::alloc::string::String,
    /// An IANA MIME type (RFC6838) of the content.
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
/// Specifies a set of documents on Cloud Storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDocuments {
    /// The list of documents.
    #[prost(message, repeated, tag = "1")]
    pub documents: ::prost::alloc::vec::Vec<GcsDocument>,
}
/// Specifies all documents on Cloud Storage with a common prefix.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsPrefix {
    /// The URI prefix.
    #[prost(string, tag = "1")]
    pub gcs_uri_prefix: ::prost::alloc::string::String,
}
/// The common config to specify a set of documents used as input.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDocumentsInputConfig {
    /// The source.
    #[prost(oneof = "batch_documents_input_config::Source", tags = "1, 2")]
    pub source: ::core::option::Option<batch_documents_input_config::Source>,
}
/// Nested message and enum types in `BatchDocumentsInputConfig`.
pub mod batch_documents_input_config {
    /// The source.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The set of documents that match the specified Cloud Storage `gcs_prefix`.
        #[prost(message, tag = "1")]
        GcsPrefix(super::GcsPrefix),
        /// The set of documents individually specified on Cloud Storage.
        #[prost(message, tag = "2")]
        GcsDocuments(super::GcsDocuments),
    }
}
/// Config that controls the output of documents. All documents will be written
/// as a JSON file.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentOutputConfig {
    /// The destination of the results.
    #[prost(oneof = "document_output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<document_output_config::Destination>,
}
/// Nested message and enum types in `DocumentOutputConfig`.
pub mod document_output_config {
    /// The configuration used when outputting documents.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsOutputConfig {
        /// The Cloud Storage uri (a directory) of the output.
        #[prost(string, tag = "1")]
        pub gcs_uri: ::prost::alloc::string::String,
        /// Specifies which fields to include in the output documents.
        /// Only supports top level document and pages field so it must be in the
        /// form of `{document_field_name}` or `pages.{page_field_name}`.
        #[prost(message, optional, tag = "2")]
        pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
        /// Specifies the sharding config for the output document.
        #[prost(message, optional, tag = "3")]
        pub sharding_config: ::core::option::Option<gcs_output_config::ShardingConfig>,
    }
    /// Nested message and enum types in `GcsOutputConfig`.
    pub mod gcs_output_config {
        /// The sharding config for the output document.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ShardingConfig {
            /// The number of pages per shard.
            #[prost(int32, tag = "1")]
            pub pages_per_shard: i32,
            /// The number of overlapping pages between consecutive shards.
            #[prost(int32, tag = "2")]
            pub pages_overlap: i32,
        }
    }
    /// The destination of the results.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Output config to write the results to Cloud Storage.
        #[prost(message, tag = "1")]
        GcsOutputConfig(GcsOutputConfig),
    }
}
/// The schema defines the output of the processed document by a processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentSchema {
    /// Display name to show to users.
    #[prost(string, tag = "1")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the schema.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Entity types of the schema.
    #[prost(message, repeated, tag = "3")]
    pub entity_types: ::prost::alloc::vec::Vec<document_schema::EntityType>,
    /// Metadata of the schema.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<document_schema::Metadata>,
}
/// Nested message and enum types in `DocumentSchema`.
pub mod document_schema {
    /// EntityType is the wrapper of a label of the corresponding model with
    /// detailed attributes and limitations for entity-based processors. Multiple
    /// types can also compose a dependency tree to represent nested types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EntityType {
        /// User defined name for the type.
        #[prost(string, tag = "13")]
        pub display_name: ::prost::alloc::string::String,
        /// Name of the type. It must be unique within the schema file and
        /// cannot be a 'Common Type'.  Besides that we use the following naming
        /// conventions:
        ///
        /// - *use `snake_casing`*
        /// - name matching is case-sensitive
        /// - Maximum 64 characters.
        /// - Must start with a letter.
        /// - Allowed characters: ASCII letters `\[a-z0-9_-\]`.  (For backward
        ///    compatibility internal infrastructure and tooling can handle any ascii
        ///    character)
        /// - The `/` is sometimes used to denote a property of a type.  For example
        ///    `line_item/amount`.  This convention is deprecated, but will still be
        ///    honored for backward compatibility.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The entity type that this type is derived from.  For now, one and only
        /// one should be set.
        #[prost(string, repeated, tag = "2")]
        pub base_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Describing the nested structure, or composition of an entity.
        #[prost(message, repeated, tag = "6")]
        pub properties: ::prost::alloc::vec::Vec<entity_type::Property>,
        #[prost(oneof = "entity_type::ValueSource", tags = "14")]
        pub value_source: ::core::option::Option<entity_type::ValueSource>,
    }
    /// Nested message and enum types in `EntityType`.
    pub mod entity_type {
        /// Defines the a list of enum values.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EnumValues {
            /// The individual values that this enum values type can include.
            #[prost(string, repeated, tag = "1")]
            pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
        /// Defines properties that can be part of the entity type.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Property {
            /// The name of the property.  Follows the same guidelines as the
            /// EntityType name.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// A reference to the value type of the property.  This type is subject
            /// to the same conventions as the `Entity.base_types` field.
            #[prost(string, tag = "2")]
            pub value_type: ::prost::alloc::string::String,
            /// Occurrence type limits the number of instances an entity type appears
            /// in the document.
            #[prost(enumeration = "property::OccurrenceType", tag = "3")]
            pub occurrence_type: i32,
        }
        /// Nested message and enum types in `Property`.
        pub mod property {
            /// Types of occurrences of the entity type in the document.  Note: this
            /// represents the number of instances of an entity types, not number of
            /// mentions of a given entity instance.
            #[derive(
                Clone,
                Copy,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
                ::prost::Enumeration
            )]
            #[repr(i32)]
            pub enum OccurrenceType {
                /// Unspecified occurrence type.
                Unspecified = 0,
                /// There will be zero or one instance of this entity type.
                OptionalOnce = 1,
                /// The entity type will appear zero or multiple times.
                OptionalMultiple = 2,
                /// The entity type will only appear exactly once.
                RequiredOnce = 3,
                /// The entity type will appear once or more times.
                RequiredMultiple = 4,
            }
            impl OccurrenceType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        OccurrenceType::Unspecified => "OCCURRENCE_TYPE_UNSPECIFIED",
                        OccurrenceType::OptionalOnce => "OPTIONAL_ONCE",
                        OccurrenceType::OptionalMultiple => "OPTIONAL_MULTIPLE",
                        OccurrenceType::RequiredOnce => "REQUIRED_ONCE",
                        OccurrenceType::RequiredMultiple => "REQUIRED_MULTIPLE",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "OCCURRENCE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                        "OPTIONAL_ONCE" => Some(Self::OptionalOnce),
                        "OPTIONAL_MULTIPLE" => Some(Self::OptionalMultiple),
                        "REQUIRED_ONCE" => Some(Self::RequiredOnce),
                        "REQUIRED_MULTIPLE" => Some(Self::RequiredMultiple),
                        _ => None,
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ValueSource {
            /// If specified, lists all the possible values for this entity.  This
            /// should not be more than a handful of values.  If the number of values
            /// is >10 or could change frequently use the `EntityType.value_ontology`
            /// field and specify a list of all possible values in a value ontology
            /// file.
            #[prost(message, tag = "14")]
            EnumValues(EnumValues),
        }
    }
    /// Metadata for global schema behavior.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metadata {
        /// If true, a `document` entity type can be applied to subdocument (
        /// splitting). Otherwise, it can only be applied to the entire document
        /// (classification).
        #[prost(bool, tag = "1")]
        pub document_splitter: bool,
        /// If true, on a given page, there can be multiple `document` annotations
        /// covering it.
        #[prost(bool, tag = "2")]
        pub document_allow_multiple_labels: bool,
        /// If set, all the nested entities must be prefixed with the parents.
        #[prost(bool, tag = "6")]
        pub prefixed_naming_on_properties: bool,
        /// If set, we will skip the naming format validation in the schema. So the
        /// string values in `DocumentSchema.EntityType.name` and
        /// `DocumentSchema.EntityType.Property.name` will not be checked.
        #[prost(bool, tag = "7")]
        pub skip_naming_validation: bool,
    }
}
/// The common metadata for long running operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonOperationMetadata {
    /// The state of the operation.
    #[prost(enumeration = "common_operation_metadata::State", tag = "1")]
    pub state: i32,
    /// A message providing more details about the current state of processing.
    #[prost(string, tag = "2")]
    pub state_message: ::prost::alloc::string::String,
    /// A related resource to this operation.
    #[prost(string, tag = "5")]
    pub resource: ::prost::alloc::string::String,
    /// The creation time of the operation.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last update time of the operation.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `CommonOperationMetadata`.
pub mod common_operation_metadata {
    /// State of the longrunning operation.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// Operation is still running.
        Running = 1,
        /// Operation is being cancelled.
        Cancelling = 2,
        /// Operation succeeded.
        Succeeded = 3,
        /// Operation failed.
        Failed = 4,
        /// Operation is cancelled.
        Cancelled = 5,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Cancelling => "CANCELLING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Cancelled => "CANCELLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNING" => Some(Self::Running),
                "CANCELLING" => Some(Self::Cancelling),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "CANCELLED" => Some(Self::Cancelled),
                _ => None,
            }
        }
    }
}
/// A processor version is an implementation of a processor. Each processor
/// can have multiple versions, pre-trained by Google internally or up-trained
/// by the customer. At a time, a processor can only have one default version
/// version. So the processor's behavior (when processing documents) is defined
/// by a default version
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessorVersion {
    /// The resource name of the processor version.
    /// Format:
    /// `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processor_version}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The display name of the processor version.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// The schema of the processor version. Describes the output.
    #[prost(message, optional, tag = "12")]
    pub document_schema: ::core::option::Option<DocumentSchema>,
    /// The state of the processor version.
    #[prost(enumeration = "processor_version::State", tag = "6")]
    pub state: i32,
    /// The time the processor version was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The KMS key name used for encryption.
    #[prost(string, tag = "9")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// The KMS key version with which data is encrypted.
    #[prost(string, tag = "10")]
    pub kms_key_version_name: ::prost::alloc::string::String,
    /// Denotes that this ProcessorVersion is managed by google.
    #[prost(bool, tag = "11")]
    pub google_managed: bool,
    /// If set, information about the eventual deprecation of this version.
    #[prost(message, optional, tag = "13")]
    pub deprecation_info: ::core::option::Option<processor_version::DeprecationInfo>,
}
/// Nested message and enum types in `ProcessorVersion`.
pub mod processor_version {
    /// Information about the upcoming deprecation of this processor version.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeprecationInfo {
        /// The time at which this processor version will be deprecated.
        #[prost(message, optional, tag = "1")]
        pub deprecation_time: ::core::option::Option<::prost_types::Timestamp>,
        /// If set, the processor version that will be used as a replacement.
        #[prost(string, tag = "2")]
        pub replacement_processor_version: ::prost::alloc::string::String,
    }
    /// The possible states of the processor version.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The processor version is in an unspecified state.
        Unspecified = 0,
        /// The processor version is deployed and can be used for processing.
        Deployed = 1,
        /// The processor version is being deployed.
        Deploying = 2,
        /// The processor version is not deployed and cannot be used for processing.
        Undeployed = 3,
        /// The processor version is being undeployed.
        Undeploying = 4,
        /// The processor version is being created.
        Creating = 5,
        /// The processor version is being deleted.
        Deleting = 6,
        /// The processor version failed and is in an indeterminate state.
        Failed = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Deployed => "DEPLOYED",
                State::Deploying => "DEPLOYING",
                State::Undeployed => "UNDEPLOYED",
                State::Undeploying => "UNDEPLOYING",
                State::Creating => "CREATING",
                State::Deleting => "DELETING",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "DEPLOYED" => Some(Self::Deployed),
                "DEPLOYING" => Some(Self::Deploying),
                "UNDEPLOYED" => Some(Self::Undeployed),
                "UNDEPLOYING" => Some(Self::Undeploying),
                "CREATING" => Some(Self::Creating),
                "DELETING" => Some(Self::Deleting),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// The first-class citizen for Document AI. Each processor defines how to
/// extract structural information from a document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Processor {
    /// Output only. Immutable. The resource name of the processor.
    /// Format: `projects/{project}/locations/{location}/processors/{processor}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The processor type, e.g., `OCR_PROCESSOR`, `INVOICE_PROCESSOR`, etc.
    /// To get a list of processors types, see
    /// \[FetchProcessorTypes][google.cloud.documentai.v1.DocumentProcessorService.FetchProcessorTypes\].
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// The display name of the processor.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The state of the processor.
    #[prost(enumeration = "processor::State", tag = "4")]
    pub state: i32,
    /// The default processor version.
    #[prost(string, tag = "9")]
    pub default_processor_version: ::prost::alloc::string::String,
    /// Output only. Immutable. The http endpoint that can be called to invoke
    /// processing.
    #[prost(string, tag = "6")]
    pub process_endpoint: ::prost::alloc::string::String,
    /// The time the processor was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The KMS key used for encryption/decryption in CMEK scenarios.
    /// See <https://cloud.google.com/security-key-management.>
    #[prost(string, tag = "8")]
    pub kms_key_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Processor`.
pub mod processor {
    /// The possible states of the processor.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The processor is in an unspecified state.
        Unspecified = 0,
        /// The processor is enabled, i.e., has an enabled version which can
        /// currently serve processing requests and all the feature dependencies have
        /// been successfully initialized.
        Enabled = 1,
        /// The processor is disabled.
        Disabled = 2,
        /// The processor is being enabled, will become `ENABLED` if successful.
        Enabling = 3,
        /// The processor is being disabled, will become `DISABLED` if successful.
        Disabling = 4,
        /// The processor is being created, will become either `ENABLED` (for
        /// successful creation) or `FAILED` (for failed ones).
        /// Once a processor is in this state, it can then be used for document
        /// processing, but the feature dependencies of the processor might not be
        /// fully created yet.
        Creating = 5,
        /// The processor failed during creation or initialization of feature
        /// dependencies. The user should delete the processor and recreate one as
        /// all the functionalities of the processor are disabled.
        Failed = 6,
        /// The processor is being deleted, will be removed if successful.
        Deleting = 7,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Enabled => "ENABLED",
                State::Disabled => "DISABLED",
                State::Enabling => "ENABLING",
                State::Disabling => "DISABLING",
                State::Creating => "CREATING",
                State::Failed => "FAILED",
                State::Deleting => "DELETING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ENABLED" => Some(Self::Enabled),
                "DISABLED" => Some(Self::Disabled),
                "ENABLING" => Some(Self::Enabling),
                "DISABLING" => Some(Self::Disabling),
                "CREATING" => Some(Self::Creating),
                "FAILED" => Some(Self::Failed),
                "DELETING" => Some(Self::Deleting),
                _ => None,
            }
        }
    }
}
/// A processor type is responsible for performing a certain document
/// understanding task on a certain type of document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessorType {
    /// The resource name of the processor type.
    /// Format: `projects/{project}/processorTypes/{processor_type}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The processor type, e.g., `OCR_PROCESSOR`, `INVOICE_PROCESSOR`, etc.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// The processor category, used by UI to group processor types.
    #[prost(string, tag = "3")]
    pub category: ::prost::alloc::string::String,
    /// The locations in which this processor is available.
    #[prost(message, repeated, tag = "4")]
    pub available_locations: ::prost::alloc::vec::Vec<processor_type::LocationInfo>,
    /// Whether the processor type allows creation. If true, users can create a
    /// processor of this processor type. Otherwise, users need to request access.
    #[prost(bool, tag = "6")]
    pub allow_creation: bool,
    /// Launch stage of the processor type
    #[prost(enumeration = "super::super::super::api::LaunchStage", tag = "8")]
    pub launch_stage: i32,
    /// A set of Cloud Storage URIs of sample documents for this processor.
    #[prost(string, repeated, tag = "9")]
    pub sample_document_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ProcessorType`.
pub mod processor_type {
    /// The location information about where the processor is available.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationInfo {
        /// The location id, currently must be one of [us, eu].
        #[prost(string, tag = "1")]
        pub location_id: ::prost::alloc::string::String,
    }
}
/// Request message for the process document method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessRequest {
    /// Required. The resource name of the
    /// \[Processor][google.cloud.documentai.v1.Processor\] or
    /// \[ProcessorVersion][google.cloud.documentai.v1.ProcessorVersion\]
    /// to use for processing. If a
    /// \[Processor][google.cloud.documentai.v1.Processor\] is specified, the server
    /// will use its [default
    /// version]\[google.cloud.documentai.v1.Processor.default_processor_version\].
    /// Format: `projects/{project}/locations/{location}/processors/{processor}`,
    /// or
    /// `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Whether Human Review feature should be skipped for this request. Default to
    /// false.
    #[prost(bool, tag = "3")]
    pub skip_human_review: bool,
    /// Specifies which fields to include in ProcessResponse's document.
    /// Only supports top level document and pages field so it must be in the form
    /// of `{document_field_name}` or `pages.{page_field_name}`.
    #[prost(message, optional, tag = "6")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The document payload.
    #[prost(oneof = "process_request::Source", tags = "4, 5")]
    pub source: ::core::option::Option<process_request::Source>,
}
/// Nested message and enum types in `ProcessRequest`.
pub mod process_request {
    /// The document payload.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// An inline document proto.
        #[prost(message, tag = "4")]
        InlineDocument(super::Document),
        /// A raw document content (bytes).
        #[prost(message, tag = "5")]
        RawDocument(super::RawDocument),
    }
}
/// The status of human review on a processed document.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HumanReviewStatus {
    /// The state of human review on the processing request.
    #[prost(enumeration = "human_review_status::State", tag = "1")]
    pub state: i32,
    /// A message providing more details about the human review state.
    #[prost(string, tag = "2")]
    pub state_message: ::prost::alloc::string::String,
    /// The name of the operation triggered by the processed document. This field
    /// is populated only when the \[state\] is \[HUMAN_REVIEW_IN_PROGRESS\]. It has
    /// the same response type and metadata as the long running operation returned
    /// by \[ReviewDocument\] method.
    #[prost(string, tag = "3")]
    pub human_review_operation: ::prost::alloc::string::String,
}
/// Nested message and enum types in `HumanReviewStatus`.
pub mod human_review_status {
    /// The final state of human review on a processed document.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Human review state is unspecified. Most likely due to an internal error.
        Unspecified = 0,
        /// Human review is skipped for the document. This can happen because human
        /// review is not enabled on the processor or the processing request has
        /// been set to skip this document.
        Skipped = 1,
        /// Human review validation is triggered and passed, so no review is needed.
        ValidationPassed = 2,
        /// Human review validation is triggered and the document is under review.
        InProgress = 3,
        /// Some error happened during triggering human review, see the
        /// \[state_message\] for details.
        Error = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Skipped => "SKIPPED",
                State::ValidationPassed => "VALIDATION_PASSED",
                State::InProgress => "IN_PROGRESS",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "SKIPPED" => Some(Self::Skipped),
                "VALIDATION_PASSED" => Some(Self::ValidationPassed),
                "IN_PROGRESS" => Some(Self::InProgress),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
}
/// Response message for the process document method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessResponse {
    /// The document payload, will populate fields based on the processor's
    /// behavior.
    #[prost(message, optional, tag = "1")]
    pub document: ::core::option::Option<Document>,
    /// The status of human review on the processed document.
    #[prost(message, optional, tag = "3")]
    pub human_review_status: ::core::option::Option<HumanReviewStatus>,
}
/// Request message for batch process document method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchProcessRequest {
    /// Required. The resource name of
    /// \[Processor][google.cloud.documentai.v1.Processor\] or
    /// \[ProcessorVersion][google.cloud.documentai.v1.ProcessorVersion\].
    /// Format: `projects/{project}/locations/{location}/processors/{processor}`,
    /// or
    /// `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{processorVersion}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The input documents for batch process.
    #[prost(message, optional, tag = "5")]
    pub input_documents: ::core::option::Option<BatchDocumentsInputConfig>,
    /// The overall output config for batch process.
    #[prost(message, optional, tag = "6")]
    pub document_output_config: ::core::option::Option<DocumentOutputConfig>,
    /// Whether Human Review feature should be skipped for this request. Default to
    /// false.
    #[prost(bool, tag = "4")]
    pub skip_human_review: bool,
}
/// Response message for batch process document method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchProcessResponse {}
/// The long running operation metadata for batch process method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchProcessMetadata {
    /// The state of the current batch processing.
    #[prost(enumeration = "batch_process_metadata::State", tag = "1")]
    pub state: i32,
    /// A message providing more details about the current state of processing.
    /// For example, the error message if the operation is failed.
    #[prost(string, tag = "2")]
    pub state_message: ::prost::alloc::string::String,
    /// The creation time of the operation.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last update time of the operation.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The list of response details of each document.
    #[prost(message, repeated, tag = "5")]
    pub individual_process_statuses: ::prost::alloc::vec::Vec<
        batch_process_metadata::IndividualProcessStatus,
    >,
}
/// Nested message and enum types in `BatchProcessMetadata`.
pub mod batch_process_metadata {
    /// The status of a each individual document in the batch process.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IndividualProcessStatus {
        /// The source of the document, same as the \[input_gcs_source\] field in the
        /// request when the batch process started. The batch process is started by
        /// take snapshot of that document, since a user can move or change that
        /// document during the process.
        #[prost(string, tag = "1")]
        pub input_gcs_source: ::prost::alloc::string::String,
        /// The status processing the document.
        #[prost(message, optional, tag = "2")]
        pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
        /// The output_gcs_destination (in the request as `output_gcs_destination`)
        /// of the processed document if it was successful, otherwise empty.
        #[prost(string, tag = "3")]
        pub output_gcs_destination: ::prost::alloc::string::String,
        /// The status of human review on the processed document.
        #[prost(message, optional, tag = "5")]
        pub human_review_status: ::core::option::Option<super::HumanReviewStatus>,
    }
    /// Possible states of the batch processing operation.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// Request operation is waiting for scheduling.
        Waiting = 1,
        /// Request is being processed.
        Running = 2,
        /// The batch processing completed successfully.
        Succeeded = 3,
        /// The batch processing was being cancelled.
        Cancelling = 4,
        /// The batch processing was cancelled.
        Cancelled = 5,
        /// The batch processing has failed.
        Failed = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Waiting => "WAITING",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Cancelling => "CANCELLING",
                State::Cancelled => "CANCELLED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "WAITING" => Some(Self::Waiting),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "CANCELLING" => Some(Self::Cancelling),
                "CANCELLED" => Some(Self::Cancelled),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// Request message for fetch processor types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchProcessorTypesRequest {
    /// Required. The project of processor type to list.
    /// The available processor types may depend on the allow-listing on projects.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response message for fetch processor types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchProcessorTypesResponse {
    /// The list of processor types.
    #[prost(message, repeated, tag = "1")]
    pub processor_types: ::prost::alloc::vec::Vec<ProcessorType>,
}
/// Request message for list processor types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProcessorTypesRequest {
    /// Required. The location of processor type to list.
    /// The available processor types may depend on the allow-listing on projects.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of processor types to return.
    /// If unspecified, at most 100 processor types will be returned.
    /// The maximum value is 500; values above 500 will be coerced to 500.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Used to retrieve the next page of results, empty if at the end of the list.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for list processor types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProcessorTypesResponse {
    /// The processor types.
    #[prost(message, repeated, tag = "1")]
    pub processor_types: ::prost::alloc::vec::Vec<ProcessorType>,
    /// Points to the next page, otherwise empty.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for list all processors belongs to a project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProcessorsRequest {
    /// Required. The parent (project and location) which owns this collection of
    /// Processors. Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of processors to return.
    /// If unspecified, at most 50 processors will be returned.
    /// The maximum value is 100; values above 100 will be coerced to 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// We will return the processors sorted by creation time. The page token
    /// will point to the next processor.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for list processors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProcessorsResponse {
    /// The list of processors.
    #[prost(message, repeated, tag = "1")]
    pub processors: ::prost::alloc::vec::Vec<Processor>,
    /// Points to the next processor, otherwise empty.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for get processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProcessorRequest {
    /// Required. The processor resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for get processor version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProcessorVersionRequest {
    /// Required. The processor resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for list all processor versions belongs to a processor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProcessorVersionsRequest {
    /// Required. The parent (project, location and processor) to list all
    /// versions. Format:
    /// `projects/{project}/locations/{location}/processors/{processor}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of processor versions to return.
    /// If unspecified, at most 10 processor versions will be returned.
    /// The maximum value is 20; values above 20 will be coerced to 20.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// We will return the processor versions sorted by creation time. The page
    /// token will point to the next processor version.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for list processors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProcessorVersionsResponse {
    /// The list of processors.
    #[prost(message, repeated, tag = "1")]
    pub processor_versions: ::prost::alloc::vec::Vec<ProcessorVersion>,
    /// Points to the next processor, otherwise empty.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for the delete processor version method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProcessorVersionRequest {
    /// Required. The processor version resource name to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The long running operation metadata for delete processor version method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProcessorVersionMetadata {
    /// The basic metadata of the long running operation.
    #[prost(message, optional, tag = "1")]
    pub common_metadata: ::core::option::Option<CommonOperationMetadata>,
}
/// Request message for the deploy processor version method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployProcessorVersionRequest {
    /// Required. The processor version resource name to be deployed.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for the deploy processor version method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployProcessorVersionResponse {}
/// The long running operation metadata for deploy processor version method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployProcessorVersionMetadata {
    /// The basic metadata of the long running operation.
    #[prost(message, optional, tag = "1")]
    pub common_metadata: ::core::option::Option<CommonOperationMetadata>,
}
/// Request message for the undeploy processor version method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployProcessorVersionRequest {
    /// Required. The processor version resource name to be undeployed.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for the undeploy processor version method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployProcessorVersionResponse {}
/// The long running operation metadata for the undeploy processor version
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeployProcessorVersionMetadata {
    /// The basic metadata of the long running operation.
    #[prost(message, optional, tag = "1")]
    pub common_metadata: ::core::option::Option<CommonOperationMetadata>,
}
/// Request message for create a processor. Notice this request is sent to
/// a regionalized backend service, and if the processor type is not available
/// on that region, the creation will fail.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProcessorRequest {
    /// Required. The parent (project and location) under which to create the
    /// processor. Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The processor to be created, requires \[processor_type\] and
    /// \[display_name\] to be set. Also, the processor is under CMEK if CMEK fields
    /// are set.
    #[prost(message, optional, tag = "2")]
    pub processor: ::core::option::Option<Processor>,
}
/// Request message for the delete processor method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProcessorRequest {
    /// Required. The processor resource name to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The long running operation metadata for delete processor method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProcessorMetadata {
    /// The basic metadata of the long running operation.
    #[prost(message, optional, tag = "5")]
    pub common_metadata: ::core::option::Option<CommonOperationMetadata>,
}
/// Request message for the enable processor method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableProcessorRequest {
    /// Required. The processor resource name to be enabled.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for the enable processor method.
/// Intentionally empty proto for adding fields in future.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableProcessorResponse {}
/// The long running operation metadata for enable processor method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableProcessorMetadata {
    /// The basic metadata of the long running operation.
    #[prost(message, optional, tag = "5")]
    pub common_metadata: ::core::option::Option<CommonOperationMetadata>,
}
/// Request message for the disable processor method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableProcessorRequest {
    /// Required. The processor resource name to be disabled.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for the disable processor method.
/// Intentionally empty proto for adding fields in future.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableProcessorResponse {}
/// The long running operation metadata for disable processor method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableProcessorMetadata {
    /// The basic metadata of the long running operation.
    #[prost(message, optional, tag = "5")]
    pub common_metadata: ::core::option::Option<CommonOperationMetadata>,
}
/// Request message for the set default processor version method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDefaultProcessorVersionRequest {
    /// Required. The resource name of the
    /// \[Processor][google.cloud.documentai.v1.Processor\] to change default
    /// version.
    #[prost(string, tag = "1")]
    pub processor: ::prost::alloc::string::String,
    /// Required. The resource name of child
    /// \[ProcessorVersion][google.cloud.documentai.v1.ProcessorVersion\] to use as
    /// default. Format:
    /// `projects/{project}/locations/{location}/processors/{processor}/processorVersions/{version}`
    #[prost(string, tag = "2")]
    pub default_processor_version: ::prost::alloc::string::String,
}
/// Response message for set default processor version method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDefaultProcessorVersionResponse {}
/// The long running operation metadata for set default processor version
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDefaultProcessorVersionMetadata {
    /// The basic metadata of the long running operation.
    #[prost(message, optional, tag = "1")]
    pub common_metadata: ::core::option::Option<CommonOperationMetadata>,
}
/// Request message for review document method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReviewDocumentRequest {
    /// Required. The resource name of the HumanReviewConfig that the document will
    /// be reviewed with.
    #[prost(string, tag = "1")]
    pub human_review_config: ::prost::alloc::string::String,
    /// Whether the validation should be performed on the ad-hoc review request.
    #[prost(bool, tag = "3")]
    pub enable_schema_validation: bool,
    /// The priority of the human review task.
    #[prost(enumeration = "review_document_request::Priority", tag = "5")]
    pub priority: i32,
    /// The document schema of the human review task.
    #[prost(message, optional, tag = "6")]
    pub document_schema: ::core::option::Option<DocumentSchema>,
    /// The document payload.
    #[prost(oneof = "review_document_request::Source", tags = "4")]
    pub source: ::core::option::Option<review_document_request::Source>,
}
/// Nested message and enum types in `ReviewDocumentRequest`.
pub mod review_document_request {
    /// The priority level of the human review task.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Priority {
        /// The default priority level.
        Default = 0,
        /// The urgent priority level. The labeling manager should allocate labeler
        /// resource to the urgent task queue to respect this priority level.
        Urgent = 1,
    }
    impl Priority {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Priority::Default => "DEFAULT",
                Priority::Urgent => "URGENT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "URGENT" => Some(Self::Urgent),
                _ => None,
            }
        }
    }
    /// The document payload.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// An inline document proto.
        #[prost(message, tag = "4")]
        InlineDocument(super::Document),
    }
}
/// Response message for review document method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReviewDocumentResponse {
    /// The Cloud Storage uri for the human reviewed document if the review is
    /// succeeded.
    #[prost(string, tag = "1")]
    pub gcs_destination: ::prost::alloc::string::String,
    /// The state of the review operation.
    #[prost(enumeration = "review_document_response::State", tag = "2")]
    pub state: i32,
    /// The reason why the review is rejected by reviewer.
    #[prost(string, tag = "3")]
    pub rejection_reason: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ReviewDocumentResponse`.
pub mod review_document_response {
    /// Possible states of the review operation.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// The review operation is rejected by the reviewer.
        Rejected = 1,
        /// The review operation is succeeded.
        Succeeded = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Rejected => "REJECTED",
                State::Succeeded => "SUCCEEDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "REJECTED" => Some(Self::Rejected),
                "SUCCEEDED" => Some(Self::Succeeded),
                _ => None,
            }
        }
    }
}
/// The long running operation metadata for review document method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReviewDocumentOperationMetadata {
    /// The basic metadata of the long running operation.
    #[prost(message, optional, tag = "5")]
    pub common_metadata: ::core::option::Option<CommonOperationMetadata>,
    /// The Crowd Compute question ID.
    #[prost(string, tag = "6")]
    pub question_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod document_processor_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to call Cloud DocumentAI to process documents according to the
    /// processor's definition. Processors are built using state-of-the-art Google
    /// AI such as natural language, computer vision, and translation to extract
    /// structured information from unstructured or semi-structured documents.
    #[derive(Debug, Clone)]
    pub struct DocumentProcessorServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DocumentProcessorServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DocumentProcessorServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DocumentProcessorServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DocumentProcessorServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Processes a single document.
        pub async fn process_document(
            &mut self,
            request: impl tonic::IntoRequest<super::ProcessRequest>,
        ) -> Result<tonic::Response<super::ProcessResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/ProcessDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// LRO endpoint to batch process many documents. The output is written
        /// to Cloud Storage as JSON in the [Document] format.
        pub async fn batch_process_documents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchProcessRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/BatchProcessDocuments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetches processor types. Note that we do not use ListProcessorTypes here
        /// because it is not paginated.
        pub async fn fetch_processor_types(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchProcessorTypesRequest>,
        ) -> Result<tonic::Response<super::FetchProcessorTypesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/FetchProcessorTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the processor types that exist.
        pub async fn list_processor_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProcessorTypesRequest>,
        ) -> Result<tonic::Response<super::ListProcessorTypesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/ListProcessorTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all processors which belong to this project.
        pub async fn list_processors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProcessorsRequest>,
        ) -> Result<tonic::Response<super::ListProcessorsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/ListProcessors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a processor detail.
        pub async fn get_processor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProcessorRequest>,
        ) -> Result<tonic::Response<super::Processor>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/GetProcessor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a processor version detail.
        pub async fn get_processor_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProcessorVersionRequest>,
        ) -> Result<tonic::Response<super::ProcessorVersion>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/GetProcessorVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all versions of a processor.
        pub async fn list_processor_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProcessorVersionsRequest>,
        ) -> Result<
            tonic::Response<super::ListProcessorVersionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/ListProcessorVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the processor version, all artifacts under the processor version
        /// will be deleted.
        pub async fn delete_processor_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProcessorVersionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/DeleteProcessorVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deploys the processor version.
        pub async fn deploy_processor_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DeployProcessorVersionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/DeployProcessorVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Undeploys the processor version.
        pub async fn undeploy_processor_version(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeployProcessorVersionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/UndeployProcessorVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a processor from the type processor that the user chose.
        /// The processor will be at "ENABLED" state by default after its creation.
        pub async fn create_processor(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProcessorRequest>,
        ) -> Result<tonic::Response<super::Processor>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/CreateProcessor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the processor, unloads all deployed model artifacts if it was
        /// enabled and then deletes all artifacts associated with this processor.
        pub async fn delete_processor(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProcessorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/DeleteProcessor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Enables a processor
        pub async fn enable_processor(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableProcessorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/EnableProcessor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Disables a processor
        pub async fn disable_processor(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableProcessorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/DisableProcessor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Set the default (active) version of a
        /// [Processor][google.cloud.documentai.v1.Processor] that will be used in
        /// [ProcessDocument][google.cloud.documentai.v1.DocumentProcessorService.ProcessDocument]
        /// and
        /// [BatchProcessDocuments][google.cloud.documentai.v1.DocumentProcessorService.BatchProcessDocuments].
        pub async fn set_default_processor_version(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDefaultProcessorVersionRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/SetDefaultProcessorVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Send a document for Human Review. The input document should be processed by
        /// the specified processor.
        pub async fn review_document(
            &mut self,
            request: impl tonic::IntoRequest<super::ReviewDocumentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.documentai.v1.DocumentProcessorService/ReviewDocument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
