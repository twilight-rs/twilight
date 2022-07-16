//! Efficiently decompress Discord gateway messages.
//!
//! This module contains the [`Inflater`], which decompresses messages sent over
//! the gateway. It does this by reusing buffers so only a few allocations happen
//! in the hot path.
//!
//! # Resizing buffers
//!
//! Buffers are resized after some heurestics:
//!
//! - if the data does not fit the buffer size is doubled; or
//! - at most once per minute the buffer will be resized down to the size of the
//!   most recent received message. This is especially useful since Discord
//!   generally sends the largest messages on startup.

use crate::ShardId;
use flate2::{Decompress, DecompressError, FlushDecompress};
use std::{mem, time::Instant};

/// The "magic number" deciding if a message is done or if another
/// message needs to be read.
///
/// The suffix is documented in the [Discord docs].
///
/// [Discord docs]: https://discord.com/developers/docs/topics/gateway#transport-compression-transport-compression-example
const ZLIB_SUFFIX: [u8; 4] = [0x00, 0x00, 0xff, 0xff];

/// Initial buffer size of 32 KB, this size is used for both the internal buffer
/// and the buffer containing messages to be read.
const INTERNAL_BUFFER_SIZE: usize = 32 * 1024;

/// Efficient decompressing of gateway messages sent by Discord.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Debug)]
pub struct Inflater {
    /// Zlib decompressor, which can have a considerable heap size given the
    /// main way this saves memory is by having a directonary to look up data.
    decompress: Decompress,
    /// Buffer for storing compressed data. Data is stored here via [`extend`].
    ///
    /// [`extend`]: Self::extend
    compressed: Vec<u8>,
    /// Internal buffer used to store intermidiate decompressed values in.
    ///
    /// Due to decompression sometimes needing to be invoked multiple times this
    /// buffer is used to keep the intermidiate values which are then copied to
    /// [`buffer`].
    internal_buffer: Vec<u8>,
    /// Buffer which gets handed to the user when it contains a full message.
    buffer: Vec<u8>,
    /// When the last resize happend, used to compute if it is time for another
    /// resize.
    last_resize: Instant,
    /// ID of the shard the inflater is owned by.
    ///
    /// Used solely for debugging purposes.
    shard_id: ShardId,
}

impl Inflater {
    /// Create a new inflater for a shard.
    pub fn new(shard_id: ShardId) -> Self {
        Self {
            buffer: Vec::with_capacity(INTERNAL_BUFFER_SIZE),
            compressed: Vec::new(),
            decompress: Decompress::new(true),
            internal_buffer: Vec::with_capacity(INTERNAL_BUFFER_SIZE),
            last_resize: Instant::now(),
            shard_id,
        }
    }

    /// Return a mutable reference to the buffer.
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.buffer.as_mut_slice()
    }

    /// Extend the internal compressed buffer with bytes.
    pub fn extend(&mut self, slice: &[u8]) {
        self.compressed.extend_from_slice(slice);
    }

    /// Decompress the next message if a complete payload was received.
    ///
    /// Returns `None` if an incomplete payload was received.
    ///
    /// # Errors
    ///
    /// This returns `flate2`'s `DecompressError` as its method's type signature
    /// indicates it can return an error, however in reality in versions up to
    /// 1.0.17 it won't.
    #[tracing::instrument(level = "trace")]
    pub fn msg(&mut self) -> Result<Option<&mut [u8]>, DecompressError> {
        let length = self.compressed.len();

        // Check if a partial payload was received. If it was, we can just
        // return that no decompressed message is available.
        if length < 4 || self.compressed[(length - 4)..] != ZLIB_SUFFIX {
            return Ok(None);
        }

        // Amount of bytes the decompressor has handled up to now.
        let before = self.decompress.total_in();

        // Offset of the `compressed` field, use to call `decompress_vec` multiple
        // times.
        let mut offset = 0;

        loop {
            self.internal_buffer.clear();

            // Use Sync to ensure data is flushed to the internal buffer.
            self.decompress.decompress_vec(
                &self.compressed[offset..],
                &mut self.internal_buffer,
                FlushDecompress::Sync,
            )?;

            // Compute offset of the next loop.
            offset = (self.decompress.total_in() - before)
                .try_into()
                .unwrap_or_default();
            // Move the intermidiate data into the user facing buffer.
            self.buffer.extend_from_slice(&self.internal_buffer[..]);

            let not_at_capacity = self.internal_buffer.len() < self.internal_buffer.capacity();

            // Break if the offset is outside of the buffer, otherwise it could
            // panic.
            if not_at_capacity || offset > self.compressed.len() {
                break;
            }
        }

        tracing::trace!(
            bytes_in = self.compressed.len(),
            bytes_out = self.buffer.len(),
            shard_id = %self.shard_id,
            "payload lengths",
        );

        self.compressed.clear();

        {
            // It doesn't matter if we lose precision for logging.
            #[allow(clippy::cast_precision_loss)]
            let saved_percentage =
                self.decompress.total_in() as f64 / self.decompress.total_out() as f64;
            let saved_percentage_readable = saved_percentage * 100.0;
            let saved_kib = (self.decompress.total_out() - self.decompress.total_in()) / 1_024;

            tracing::trace!(
                saved_kib = saved_kib,
                saved_percentage = %saved_percentage_readable,
                shard_id = %self.shard_id,
                total_in = self.decompress.total_in(),
                total_out = self.decompress.total_out(),
                "data saved",
            );
        }

        #[cfg(feature = "metrics")]
        self.inflater_metrics();

        tracing::trace!(capacity = self.buffer.capacity(), "capacity");

        Ok(Some(&mut self.buffer))
    }

    /// Clear the buffer and shrink it if the capacity is too large.
    ///
    /// If more than a minute has passed since last shrink another will be
    /// initiated.
    #[tracing::instrument(level = "trace")]
    pub fn clear(&mut self) {
        self.shrink();

        self.compressed.clear();
        self.internal_buffer.clear();
        self.buffer.clear();
    }

    /// Reset the state of the inflater back to its default state.
    pub fn reset(&mut self) {
        *self = Self::new(self.shard_id);
    }

    /// Take the buffer, replacing it with a new one.
    pub fn take(&mut self) -> Vec<u8> {
        mem::take(&mut self.buffer)
    }

    /// Log metrics about the inflater.
    #[cfg(feature = "metrics")]
    #[allow(clippy::cast_precision_loss)]
    fn inflater_metrics(&self) {
        metrics::gauge!(
            format!("Inflater-Capacity-{}", self.shard_id.number()),
            self.buffer.capacity() as f64
        );
        metrics::gauge!(
            format!("Inflater-In-{}", self.shard_id.number()),
            self.decompress.total_in() as f64
        );
        metrics::gauge!(
            format!("Inflater-Out-{}", self.shard_id.number()),
            self.decompress.total_out() as f64
        );
    }

    /// Shrink the capacity of the compressed buffer and payload buffer if at
    /// least 60 seconds have passed since the last shrink.
    fn shrink(&mut self) {
        if self.last_resize.elapsed().as_secs() < 60 {
            return;
        }

        self.compressed.shrink_to_fit();
        self.buffer.shrink_to_fit();

        tracing::trace!(
            capacity = self.compressed.capacity(),
            shard_id = %self.shard_id,
            "compressed capacity",
        );
        tracing::trace!(
            capacity = self.buffer.capacity(),
            shard_id = %self.shard_id,
            "buffer capacity",
        );

        self.last_resize = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::Inflater;
    use crate::ShardId;
    use std::error::Error;

    const MESSAGE: &[u8] = &[
        120, 156, 52, 201, 65, 10, 131, 48, 16, 5, 208, 187, 252, 117, 82, 98, 169, 32, 115, 21,
        35, 50, 53, 67, 27, 136, 81, 226, 216, 82, 66, 238, 222, 110, 186, 123, 240, 42, 20, 148,
        207, 148, 12, 142, 63, 182, 29, 212, 57, 131, 0, 170, 120, 10, 23, 189, 11, 235, 28, 179,
        74, 121, 113, 2, 221, 186, 107, 255, 251, 89, 11, 47, 2, 26, 49, 122, 60, 88, 229, 205, 31,
        187, 151, 96, 87, 142, 217, 14, 253, 16, 60, 76, 245, 88, 227, 82, 182, 195, 131, 220, 197,
        181, 9, 83, 107, 95, 0, 0, 0, 255, 255,
    ];
    const OUTPUT: &[u8] = &[
        123, 34, 116, 34, 58, 110, 117, 108, 108, 44, 34, 115, 34, 58, 110, 117, 108, 108, 44, 34,
        111, 112, 34, 58, 49, 48, 44, 34, 100, 34, 58, 123, 34, 104, 101, 97, 114, 116, 98, 101,
        97, 116, 95, 105, 110, 116, 101, 114, 118, 97, 108, 34, 58, 52, 49, 50, 53, 48, 44, 34, 95,
        116, 114, 97, 99, 101, 34, 58, 91, 34, 91, 92, 34, 103, 97, 116, 101, 119, 97, 121, 45,
        112, 114, 100, 45, 109, 97, 105, 110, 45, 56, 53, 56, 100, 92, 34, 44, 123, 92, 34, 109,
        105, 99, 114, 111, 115, 92, 34, 58, 48, 46, 48, 125, 93, 34, 93, 125, 125,
    ];
    const SHARD: ShardId = ShardId::new(2, 5);

    #[test]
    fn inflater() -> Result<(), Box<dyn Error>> {
        let mut inflater = Inflater::new(SHARD);
        inflater.extend(&MESSAGE[0..MESSAGE.len() - 2]);
        assert_eq!(None, inflater.msg()?);

        inflater.reset();
        inflater.extend(MESSAGE);

        // Check the state of fields.
        assert!(!inflater.compressed.is_empty());
        assert!(inflater.internal_buffer.is_empty());
        assert!(inflater.buffer.is_empty());
        assert_eq!(Some(OUTPUT), inflater.msg()?.as_deref());

        // Calling `msg` clears `compressed` and fills `buffer` and `internal_buffer`.
        assert!(inflater.compressed.is_empty());
        assert!(!inflater.buffer.is_empty());
        assert!(!inflater.internal_buffer.is_empty());

        assert_eq!(OUTPUT, inflater.buffer_mut());

        // Check to make sure `buffer` and `internal_buffer` haven't been cleared.
        assert!(!inflater.internal_buffer.is_empty());
        assert!(!inflater.buffer.is_empty());

        // Now clear the inflater and make sure all buffers are empty.
        inflater.clear();
        assert!(inflater.compressed.is_empty());
        assert!(inflater.internal_buffer.is_empty());
        assert!(inflater.buffer.is_empty());

        // Reset the inflater after extending it, bringing it back to a default
        // state.
        inflater.extend(b"test");
        assert!(!inflater.compressed.is_empty());
        inflater.reset();
        assert!(inflater.compressed.is_empty());

        Ok(())
    }
}
