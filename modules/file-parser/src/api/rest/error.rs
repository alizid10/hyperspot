use modkit::api::problem::{GtsError as _, Problem};

use crate::domain::error::DomainError;
use crate::errors::{
    DownloadErrorV1, FileNotFoundV1, InvalidRequestV1, InvalidUrlV1, IoErrorV1,
    NoParserAvailableV1, ParseErrorV1, UnsupportedFileTypeV1,
};

impl From<DomainError> for Problem {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::FileNotFound { path } => FileNotFoundV1 { path }.into_problem(),
            DomainError::UnsupportedFileType { extension } => {
                UnsupportedFileTypeV1 { extension }.into_problem()
            }
            DomainError::NoParserAvailable { extension } => {
                NoParserAvailableV1 { extension }.into_problem()
            }
            DomainError::ParseError { message } => ParseErrorV1 { message }.into_problem(),
            DomainError::IoError { message } => {
                tracing::error!(error = %message, "IO error in file parser");
                IoErrorV1.into_problem()
            }
            DomainError::InvalidUrl { url } => InvalidUrlV1 { url }.into_problem(),
            DomainError::DownloadError { message } => {
                tracing::error!(error = %message, "Download error in file parser");
                DownloadErrorV1.into_problem()
            }
            DomainError::InvalidRequest { message } => InvalidRequestV1 { message }.into_problem(),
        }
    }
}
