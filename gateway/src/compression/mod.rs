//! todo

#![allow(unused)] // todo

#[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
mod inflater;

use crate::ShardId;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
use self::inflater::Inflater;

/// Query argument with zlib-stream enabled.
#[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
pub const COMPRESSION_FEATURES: &str = "&compress=zlib-stream";

/// No query arguments due to compression being disabled.
#[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
pub const COMPRESSION_FEATURES: &str = "";

/// Sending a command failed.
#[derive(Debug)]
pub struct CompressionError {
    /// Type of error.
    kind: CompressionErrorType,
    /// Source error if available.
    source: Option<Box<dyn Error + Send + Sync>>,
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
}

impl Display for CompressionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            CompressionErrorType::Decompressing => f.write_str("a frame could not be decompressed"),
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
}

/// Interface for working with buffers variable on the `zlib-stock` and
/// `zlib-simd` feature flags.
#[derive(Debug)]
pub struct Compression {
    /// Inflater for use with compression.
    #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
    inner: Inflater,
    /// Buffer for use without compression.
    #[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
    inner: Vec<u8>,
}

impl Compression {
    /// Create a new buffer, abstracting over an inflater if `zlib-stock` or
    /// `zlib-simd` features are enabled or a simple `Vec` if the features are
    /// disabled.
    #[cfg_attr(
        not(any(feature = "zlib-stock", feature = "zlib-simd")),
        allow(clippy::missing_const_for_fn, unused_variables)
    )]
    pub fn new(shard_id: ShardId) -> Self {
        Self {
            #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
            inner: Inflater::new(shard_id),
            #[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
            inner: Vec::new(),
        }
    }

    /// Mutable reference to the internal buffer slice.
    ///
    /// When compression is enabled this will mutably reference the inflater's
    /// buffer.
    ///
    /// When compression is disabled this will mutably reference the standard
    /// buffer.
    pub fn buffer_slice_mut(&mut self) -> &mut [u8] {
        #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
        {
            self.inner.buffer_mut()
        }

        #[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
        self.inner.as_mut_slice()
    }

    /// Clear the inner buffer.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// Extend the buffer with bytes from a Binary websocket message.
    pub fn extend(&mut self, bytes: &[u8]) {
        #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
        self.inner.extend(bytes);

        #[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
        self.inner.extend_from_slice(bytes);
    }

    /// Mutable reference to the inner completed message if compression is
    /// enabled.
    ///
    /// If compression is enabled and a message has completed then a mutable
    /// slice of the buffer is returned.
    ///
    /// If compression is enabled and a message has *not* completed then a
    /// successful `None` is returned.
    ///
    /// If compression is disabled then a successful `None` is returned.
    ///
    /// # Errors
    ///
    /// If compression is enabled then this returns a
    /// `ReceivingEventErrorType::Decompressing` error type if decompressing the
    /// message failed.
    #[cfg_attr(
        not(any(feature = "zlib-stock", feature = "zlib-simd")),
        allow(clippy::unnecessary_wraps, clippy::unused_self)
    )]
    pub fn message_mut(&mut self) -> Result<Option<&mut [u8]>, CompressionError> {
        #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
        {
            self.inner.msg().map_err(|source| CompressionError {
                kind: CompressionErrorType::Decompressing,
                source: Some(Box::new(source)),
            })
        }

        #[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
        Ok(None)
    }

    /// Reset the buffer for a new gateway session.
    pub fn reset(&mut self) {
        #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
        self.inner.reset();

        #[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
        self.clear();
    }

    /// Take the buffer, replacing it with a new one.
    pub fn take(&mut self) -> Vec<u8> {
        #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
        {
            self.inner.take()
        }

        #[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
        {
            mem::replace(&mut self.inner, Vec::new())
        }
    }
}
