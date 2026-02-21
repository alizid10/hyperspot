//! Error catalog for `file_parser` — explicit GTS error definitions.

use gts_macros::struct_to_gts_schema;
use modkit_errors::{BaseErrorV1, GtsError};

// ---------------------------------------------------------------------------
// File Not Found — 404
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.file_parser.errors.file_not_found.v1~",
    description = "File not found",
    properties = "path",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct FileNotFoundV1 {
    pub path: String,
}

impl GtsError for FileNotFoundV1 {
    const STATUS: u16 = 404;
    const TITLE: &'static str = "File Not Found";
}

// ---------------------------------------------------------------------------
// Unsupported File Type — 400
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.file_parser.errors.unsupported_file_type.v1~",
    description = "Unsupported file type",
    properties = "extension",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct UnsupportedFileTypeV1 {
    pub extension: String,
}

impl GtsError for UnsupportedFileTypeV1 {
    const STATUS: u16 = 400;
    const TITLE: &'static str = "Unsupported File Type";
}

// ---------------------------------------------------------------------------
// No Parser Available — 415
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.file_parser.errors.no_parser_available.v1~",
    description = "No parser available for file type",
    properties = "extension",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct NoParserAvailableV1 {
    pub extension: String,
}

impl GtsError for NoParserAvailableV1 {
    const STATUS: u16 = 415;
    const TITLE: &'static str = "No Parser Available";
}

// ---------------------------------------------------------------------------
// Parse Error — 422
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.file_parser.errors.parse_error.v1~",
    description = "File parsing error",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct ParseErrorV1 {
    pub message: String,
}

impl GtsError for ParseErrorV1 {
    const STATUS: u16 = 422;
    const TITLE: &'static str = "Parse Error";
}

// ---------------------------------------------------------------------------
// IO Error — 500
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.file_parser.errors.io_error.v1~",
    description = "IO error during file processing",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct IoErrorV1;

impl GtsError for IoErrorV1 {
    const STATUS: u16 = 500;
    const TITLE: &'static str = "IO Error";
}

// ---------------------------------------------------------------------------
// Invalid URL — 400
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.file_parser.errors.invalid_url.v1~",
    description = "Invalid URL provided",
    properties = "url",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InvalidUrlV1 {
    pub url: String,
}

impl GtsError for InvalidUrlV1 {
    const STATUS: u16 = 400;
    const TITLE: &'static str = "Invalid URL";
}

// ---------------------------------------------------------------------------
// Download Error — 502
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.file_parser.errors.download_error.v1~",
    description = "File download error",
    properties = "",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct DownloadErrorV1;

impl GtsError for DownloadErrorV1 {
    const STATUS: u16 = 502;
    const TITLE: &'static str = "Download Error";
}

// ---------------------------------------------------------------------------
// Invalid Request — 400
// ---------------------------------------------------------------------------

#[struct_to_gts_schema(
    dir_path = "schemas",
    schema_id = "gts.cf.core.errors.err.v1~cf.file_parser.errors.invalid_request.v1~",
    description = "Invalid request",
    properties = "message",
    base = BaseErrorV1,
)]
#[derive(Debug)]
pub struct InvalidRequestV1 {
    pub message: String,
}

impl GtsError for InvalidRequestV1 {
    const STATUS: u16 = 400;
    const TITLE: &'static str = "Invalid Request";
}
