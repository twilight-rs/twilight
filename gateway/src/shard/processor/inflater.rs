use flate2::{Decompress, DecompressError, FlushDecompress};
use std::{convert::TryInto, mem};

const ZLIB_SUFFIX: [u8; 4] = [0x00, 0x00, 0xff, 0xff];
const INTERNAL_BUFFER_SIZE: usize = 32 * 1024;

#[derive(Debug)]
pub struct Inflater {
    decompress: Decompress,
    compressed: Vec<u8>,
    internal_buffer: Vec<u8>,
    buffer: Vec<u8>,
    countdown_to_resize: u8,
    shard: [u64; 2],
}

impl Inflater {
    /// Create a new inflater for a shard.
    pub fn new(shard: [u64; 2]) -> Self {
        Self {
            buffer: Vec::with_capacity(INTERNAL_BUFFER_SIZE),
            compressed: Vec::new(),
            countdown_to_resize: u8::max_value(),
            decompress: Decompress::new(true),
            internal_buffer: Vec::with_capacity(INTERNAL_BUFFER_SIZE),
            shard,
        }
    }

    /// Extend the internal compressed buffer with bytes.
    pub fn extend(&mut self, slice: &[u8]) {
        self.compressed.extend_from_slice(&slice);
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

        let before = self.decompress.total_in();
        let mut offset = 0;

        loop {
            self.internal_buffer.clear();

            self.decompress.decompress_vec(
                &self.compressed[offset..],
                &mut self.internal_buffer,
                FlushDecompress::Sync,
            )?;

            offset = (self.decompress.total_in() - before)
                .try_into()
                .unwrap_or_default();
            self.buffer.extend_from_slice(&self.internal_buffer[..]);

            let not_at_capacity = self.internal_buffer.len() < self.internal_buffer.capacity();

            if not_at_capacity || offset > self.compressed.len() {
                break;
            }
        }

        tracing::trace!(
            bytes_in = self.compressed.len(),
            bytes_out = self.buffer.len(),
            shard_id = self.shard[0],
            shard_total = self.shard[1],
            "payload lengths",
        );
        self.compressed.clear();

        // It doesn't matter if we lose precision for logging.
        #[allow(clippy::cast_precision_loss)]
        let saved_percentage =
            self.decompress.total_in() as f64 / self.decompress.total_out() as f64;
        let saved_percentage_readable = saved_percentage * 100.0;

        let saved_kib = (self.decompress.total_out() - self.decompress.total_in()) / 1_024;

        tracing::trace!(
            saved_kib = saved_kib,
            saved_percentage = %saved_percentage_readable,
            shard_id = self.shard[0],
            shard_total = self.shard[1],
            total_in = self.decompress.total_in(),
            total_out = self.decompress.total_out(),
            "data saved",
        );

        #[cfg(feature = "metrics")]
        self.inflater_metrics();

        tracing::trace!("capacity: {}", self.buffer.capacity());
        Ok(Some(&mut self.buffer))
    }

    /// Clear the buffer and shrink it if the capacity is too large.
    ///
    /// If the capacity is 4 times larger than the buffer length then the
    /// capacity will be shrunk to the length.
    #[tracing::instrument(level = "trace")]
    pub fn clear(&mut self) {
        self.countdown_to_resize -= 1;

        self.shrink_if_too_large();

        self.compressed.clear();
        self.internal_buffer.clear();
        self.buffer.clear();
    }

    /// Reset the state of the inflater back to its default state.
    pub fn reset(&mut self) {
        let _ = mem::replace(self, Self::new(self.shard));
    }

    /// Log metrics about the inflater.
    #[cfg(feature = "metrics")]
    fn inflater_metrics(&self) {
        metrics::gauge!(
            format!("Inflater-Capacity-{}", self.shard[0]),
            self.buffer.capacity().try_into().unwrap_or(-1)
        );
        metrics::gauge!(
            format!("Inflater-In-{}", self.shard[0]),
            self.decompress.total_in().try_into().unwrap_or(-1)
        );
        metrics::gauge!(
            format!("Inflater-Out-{}", self.shard[0]),
            self.decompress.total_out().try_into().unwrap_or(-1)
        );
    }

    /// Shrink the capacity of the compressed buffer and payload buffer if the
    /// payload buffer length is less than 25% of its capacity.
    fn shrink_if_too_large(&mut self) {
        // Only shrink capacity if it is less than 4 times the size. Doing it
        // all the time will cause performance issues. So, if it's greater,
        // don't do anything.
        if self.countdown_to_resize != u8::MIN || self.buffer.len() < self.buffer.capacity() / 4 {
            return;
        }

        self.compressed.shrink_to_fit();
        self.buffer.shrink_to_fit();

        tracing::trace!(
            capacity = self.compressed.capacity(),
            shard_id = self.shard[0],
            shard_total = self.shard[1],
            "compressed capacity",
        );
        tracing::trace!(
            capacity = self.buffer.capacity(),
            shard_id = self.shard[0],
            shard_total = self.shard[1],
            "buffer capacity",
        );

        self.countdown_to_resize = u8::MAX;
    }
}
