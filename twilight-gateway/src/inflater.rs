//! Efficiently decompress Discord gateway events.
//!
//! The [`Inflater`] decompresses messages sent over the gateway by reusing a
//! common buffer so only one allocation happens in the hot path.
//!
//! # Shrinking compressed buffer
//!
//! The compressed message buffer gets shrank every minute to the size of the
//! most recent message. This is especially useful since Discord generally sends
//! the largest messages on startup.

use crate::ShardId;
use flate2::{Decompress, FlushDecompress};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    time::Instant,
};

/// An operation relating to compression failed.
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

/// The "magic number" deciding if a message is done or if another
/// message needs to be read.
///
/// The suffix is documented in the [Discord docs].
///
/// [Discord docs]: https://discord.com/developers/docs/topics/gateway#transport-compression-transport-compression-example
const ZLIB_SUFFIX: [u8; 4] = [0x00, 0x00, 0xff, 0xff];

/// Gateway event decompressor.
///
/// Each received compressed event gets inflated into a [`String`] who's input
/// and output size is recorded.
#[derive(Debug)]
pub struct Inflater {
    /// Common decompressed message buffer.
    ///
    /// Note that its capacity is static.
    buffer: Vec<u8>,
    /// Compressed message buffer.
    compressed: Vec<u8>,
    /// Zlib decompressor with a dictionary of past data.
    decompress: Decompress,
    /// When the last shrank occurred.
    last_shrank: Instant,
    /// Associated shard identifier.
    ///
    /// Used solely for recording metrics.
    #[cfg(feature = "metrics")]
    shard_id: ShardId,
}

impl Inflater {
    /// Common decompressed buffer size.
    const BUFFER_SIZE: usize = 32 * 1024;

    /// Create a new inflator for a shard.
    #[cfg_attr(not(feature = "metrics"), allow(unused_variables))]
    pub fn new(shard_id: ShardId) -> Self {
        Self {
            buffer: Vec::with_capacity(Self::BUFFER_SIZE),
            compressed: Vec::new(),
            decompress: Decompress::new(true),
            last_shrank: Instant::now(),
            #[cfg(feature = "metrics")]
            shard_id,
        }
    }

    /// Clear the compressed buffer and periodically shrink its capacity.
    fn clear(&mut self) {
        if self.last_shrank.elapsed().as_secs() > 60 {
            self.compressed.shrink_to_fit();

            tracing::trace!(
                compressed.capacity = self.compressed.capacity(),
                "shrank capacity to the size of the last message"
            );

            self.last_shrank = Instant::now();
        }

        self.compressed.clear();
    }

    /// Decompress message.
    ///
    /// Returns `None` if the message is incomplete, saving its content to be
    /// combined with the next one.
    ///
    /// # Errors
    ///
    /// Returns a [`CompressionErrorType::Decompressing`] error type if the
    /// message could not be decompressed.
    ///
    /// Returns a [`CompressionErrorType::NotUtf8`] error type if the
    /// decompressed message is not UTF-8.
    pub fn inflate(&mut self, message: &[u8]) -> Result<Option<String>, CompressionError> {
        self.compressed.extend_from_slice(message);
        let length = self.compressed.len();

        if length < 4 || self.compressed[(length - 4)..] != ZLIB_SUFFIX {
            return Ok(None);
        }

        debug_assert!(
            !self.compressed[0..(length - 4)]
                .windows(4)
                .any(|window| window == ZLIB_SUFFIX),
            "compressed buffer contains multiple messages"
        );

        let before = self.decompress.total_in();

        let mut compressed = 0;

        let mut message = Vec::new();

        loop {
            self.buffer.clear();

            // Use Sync to ensure data is flushed to the buffer.
            self.decompress
                .decompress_vec(
                    &self.compressed[compressed..],
                    &mut self.buffer,
                    FlushDecompress::Sync,
                )
                .map_err(|source| CompressionError {
                    kind: CompressionErrorType::Decompressing,
                    source: Some(Box::new(source)),
                })?;

            compressed = (self.decompress.total_in() - before).try_into().unwrap();

            message.extend_from_slice(&self.buffer);

            if compressed == self.compressed.len() {
                break;
            }

            tracing::trace!(bytes.compressed.remaining = self.compressed.len() - compressed);
        }

        {
            #[allow(clippy::cast_precision_loss)]
            let total_percentage_compressed =
                self.decompress.total_in() as f64 * 100.0 / self.decompress.total_out() as f64;
            let total_percentage_saved = 100.0 - total_percentage_compressed;
            let total_kib_saved = (self.decompress.total_out() - self.decompress.total_in()) / 1024;

            tracing::trace!(
                bytes.compressed = compressed,
                bytes.decompressed = message.len(),
                total_percentage_saved,
                "{total_kib_saved} KiB saved in total",
            );
        }

        self.clear();

        #[cfg(feature = "metrics")]
        self.metrics();

        String::from_utf8(message)
            .map(Some)
            .map_err(|source| CompressionError {
                kind: CompressionErrorType::NotUtf8,
                source: Some(Box::new(source)),
            })
    }

    /// Log metrics about the inflater.
    #[cfg(feature = "metrics")]
    #[allow(clippy::cast_precision_loss)]
    fn metrics(&self) {
        metrics::gauge!(
            format!("Inflater-In-{}", self.shard_id.number()),
            self.decompress.total_in() as f64
        );
        metrics::gauge!(
            format!("Inflater-Out-{}", self.shard_id.number()),
            self.decompress.total_out() as f64
        );
    }

    /// Reset the inflater state.
    pub fn reset(&mut self) {
        self.compressed = Vec::new();
        self.decompress.reset(true);
    }
}

#[cfg(test)]
mod tests {
    use super::Inflater;
    use crate::ShardId;

    const MESSAGE: &[u8] = &[
        120, 156, 52, 201, 65, 10, 131, 48, 16, 5, 208, 187, 252, 117, 82, 98, 169, 32, 115, 21,
        35, 50, 53, 67, 27, 136, 81, 226, 216, 82, 66, 238, 222, 110, 186, 123, 240, 42, 20, 148,
        207, 148, 12, 142, 63, 182, 29, 212, 57, 131, 0, 170, 120, 10, 23, 189, 11, 235, 28, 179,
        74, 121, 113, 2, 221, 186, 107, 255, 251, 89, 11, 47, 2, 26, 49, 122, 60, 88, 229, 205, 31,
        187, 151, 96, 87, 142, 217, 14, 253, 16, 60, 76, 245, 88, 227, 82, 182, 195, 131, 220, 197,
        181, 9, 83, 107, 95, 0, 0, 0, 255, 255,
    ];
    const OUTPUT: &str = r#"{"t":null,"s":null,"op":10,"d":{"heartbeat_interval":41250,"_trace":["[\"gateway-prd-main-858d\",{\"micros\":0.0}]"]}}"#;
    const SHARD: ShardId = ShardId::new(2, 5);

    #[test]
    fn decompress() {
        let mut inflator = Inflater::new(SHARD);
        assert!(inflator.compressed.is_empty());
        assert!(inflator.buffer.is_empty());
        assert_eq!(inflator.inflate(MESSAGE).unwrap(), Some(OUTPUT.to_owned()));

        assert!(!inflator.buffer.is_empty());
        assert!(inflator.compressed.is_empty());
    }

    #[test]
    fn invalid_is_none() {
        let mut inflator = Inflater::new(SHARD);
        assert_eq!(inflator.inflate(&[]).unwrap(), None);

        assert_eq!(
            inflator.inflate(&MESSAGE[..MESSAGE.len() - 2]).unwrap(),
            None
        );
    }

    #[test]
    fn reset() {
        let mut inflator = Inflater::new(SHARD);
        assert_eq!(
            inflator.inflate(&MESSAGE[..MESSAGE.len() - 2]).unwrap(),
            None
        );

        inflator.reset();
        assert_eq!(inflator.inflate(MESSAGE).unwrap(), Some(OUTPUT.to_owned()));
    }
}
