//! Efficiently decompress Discord gateway events.

use std::{error::Error, fmt};

#[cfg(all(
    any(feature = "zlib-stock", feature = "zlib-simd"),
    not(feature = "zstd")
))]
mod zlib;
#[cfg(feature = "zstd")]
mod zstd;

#[cfg(all(
    any(feature = "zlib-stock", feature = "zlib-simd"),
    not(feature = "zstd")
))]
pub use zlib::Decompressor;
#[cfg(feature = "zstd")]
pub use zstd::Decompressor;

/// Decompressed event buffer.
const BUFFER_SIZE: usize = 32 * 1024;

/// An operation relating to compression failed.
#[derive(Debug)]
pub struct CompressionError {
    /// Type of error.
    pub(crate) kind: CompressionErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl CompressionError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CompressionErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (CompressionErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }

    /// Shortcut to create a new error for a not UTF-8 message.
    pub(crate) fn from_utf8_error(source: std::string::FromUtf8Error) -> Self {
        Self {
            kind: CompressionErrorType::NotUtf8,
            source: Some(Box::new(source)),
        }
    }

    /// Shortcut to create a new error for an erroneous status code.
    #[cfg(feature = "zstd")]
    pub(crate) fn from_code(code: usize) -> Self {
        Self {
            kind: CompressionErrorType::Decompressing,
            source: Some(zstd_safe::get_error_name(code).into()),
        }
    }

    /// Shortcut to create a new error for a zlib decompression error.
    #[cfg(all(
        any(feature = "zlib-stock", feature = "zlib-simd"),
        not(feature = "zstd")
    ))]
    pub(crate) fn from_decompress(source: flate2::DecompressError) -> Self {
        Self {
            kind: CompressionErrorType::Decompressing,
            source: Some(source.into()),
        }
    }
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            CompressionErrorType::Decompressing => f.write_str("message could not be decompressed"),
            CompressionErrorType::NotUtf8 => f.write_str("decompressed message is not UTF-8"),
        }
    }
}

impl Error for CompressionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`CompressionError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CompressionErrorType {
    /// Decompressing a frame failed.
    Decompressing,
    /// Decompressed message is not UTF-8.
    NotUtf8,
}
