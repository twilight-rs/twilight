use flate2::{Decompress, DecompressError, FlushDecompress};
use log::{trace, warn};

const ZLIB_SUFFIX: [u8; 4] = [0x00, 0x00, 0xff, 0xff];
const INTERNAL_BUFFER_SIZE: usize = 32 * 1024;
const COUNTDOWN: u64 = 255;

pub struct Inflater {
    decompress: Decompress,
    compressed: Vec<u8>,
    internal_buffer: Vec<u8>,
    buffer: Vec<u8>,
    countdown_to_resize: u64,
}

impl Inflater {
    pub fn new() -> Self {
        Self {
            decompress: Decompress::new(true),
            compressed: Vec::new(),
            internal_buffer: Vec::with_capacity(INTERNAL_BUFFER_SIZE),
            buffer: Vec::with_capacity(1 * 1024),
            countdown_to_resize: COUNTDOWN,
        }
    }

    pub fn extend(&mut self, slice: &[u8]) {
        self.compressed.extend_from_slice(&slice);
    }

    pub fn msg(&mut self) -> Result<Option<&[u8]>, DecompressError> {
        let length = self.compressed.len();
        if length >= 4 && self.compressed[(length - 4)..] == ZLIB_SUFFIX {
            // There is a event to be decompressed
            let before = self.decompress.total_in();
            let mut offset = 0;
            loop {
                self.internal_buffer.clear();

                self.decompress.decompress_vec(
                    &self.compressed[offset..],
                    &mut self.internal_buffer,
                    FlushDecompress::Sync,
                )?;

                offset = (self.decompress.total_in() - before) as usize;
                self.buffer.extend_from_slice(&self.internal_buffer[..]);
                if self.internal_buffer.len() < self.internal_buffer.capacity()
                    || offset > self.compressed.len()
                {
                    break;
                }
            }

            trace!("in:out: {}:{}", self.compressed.len(), self.buffer.len());
            self.compressed.clear();
            trace!(
                "Data saved: {}KiB ({:.2}%)",
                ((self.decompress.total_out() - self.decompress.total_in()) / 1024),
                (self.decompress.total_in() as f64 / self.decompress.total_out() as f64 * 100.0)
            );
            trace!("Capacity: {}", self.buffer.capacity());
            Ok(Some(&self.buffer))
        } else {
            Ok(None)
        }
    }

    // Clear the buffer, and shrink it if it has more space
    // enough to grow the leng more than 4 times.
    pub fn clear(&mut self) {
        self.countdown_to_resize -= 1;

        dbg!(self.countdown_to_resize);
        // Only shrink capacity if it is less than 4
        // times the size, this is to prevent too
        // frequent shrinking.
        let cap = self.buffer.capacity();
        if self.countdown_to_resize < 1 && self.buffer.len() < cap * 4 {
            self.compressed.shrink_to_fit();
            self.buffer.shrink_to_fit();
            self.countdown_to_resize = COUNTDOWN;
        }
        self.compressed.clear();
        self.internal_buffer.clear();
        self.buffer.clear();
    }

    pub fn reset(&mut self) {
        self.decompress.reset(true);
        self.compressed.clear();
        self.internal_buffer.clear();
        self.buffer.clear();
        self.countdown_to_resize = COUNTDOWN;
    }
}
