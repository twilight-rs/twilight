//! Zlib transport compression.

use super::CompressionError;

/// Gateway event decompressor.
#[derive(Debug)]
pub struct Decompressor {
    /// Common decompressed message buffer.
    buffer: Box<[u8]>,
    /// Reusable zlib decompressor stream.
    decompress: flate2::Decompress,
    /// Partial event data buffer.
    partial: Vec<u8>,
}

impl Decompressor {
    /// Zlib synchronization marker.
    const SYNC: [u8; 4] = [0x00, 0x00, 0xff, 0xff];

    /// Create a new decompressor for a shard.
    pub fn new() -> Self {
        Self {
            buffer: vec![0; super::BUFFER_SIZE].into_boxed_slice(),
            decompress: flate2::Decompress::new(true),
            partial: Vec::new(),
        }
    }

    /// Decompress message.
    ///
    /// Returns `None` if the message is incomplete, copying it to be combined
    /// with the next message.
    ///
    /// # Errors
    ///
    /// Returns a [`CompressionErrorType::Decompressing`] error type if the
    /// message could not be decompressed.
    ///
    /// Returns a [`CompressionErrorType::NotUtf8`] error type if the
    /// decompressed message is not UTF-8.
    pub fn decompress(&mut self, message: &[u8]) -> Result<Option<String>, CompressionError> {
        // Try to bypass the `partial` buffer for single complete messages.
        let message = if self.partial.is_empty() {
            if !message.ends_with(&Self::SYNC) {
                self.partial.extend_from_slice(message);
                return Ok(None);
            }
            message
        } else {
            self.partial.extend_from_slice(message);
            if !message.ends_with(&Self::SYNC) {
                return Ok(None);
            }
            &self.partial
        };

        let total_in = self.decompress.total_in();
        let mut processed = 0;

        // Decompressed message. `Vec::extend_from_slice` efficiently allocates
        // only what's necessary.
        let mut decompressed = Vec::new();

        loop {
            let total_out = self.decompress.total_out();

            // Use Sync to ensure data is flushed to the buffer.
            self.decompress
                .decompress(
                    &message[processed..],
                    &mut self.buffer,
                    flate2::FlushDecompress::Sync,
                )
                .map_err(CompressionError::from_decompress)?;

            processed = (self.decompress.total_in() - total_in) as usize;
            let produced = (self.decompress.total_out() - total_out) as usize;

            decompressed.extend_from_slice(&self.buffer[..produced]);

            // Break when message has been fully decompressed.
            if processed == message.len() {
                break;
            }
        }

        self.partial.clear();

        String::from_utf8(decompressed)
            .map(Some)
            .map_err(CompressionError::from_utf8_error)
    }

    /// Reset the decompressor's internal state.
    pub fn reset(&mut self) {
        self.decompress.reset(true);
        self.partial.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::Decompressor;

    const MESSAGE: &[u8] = &[
        120, 156, 52, 201, 65, 10, 131, 48, 16, 5, 208, 187, 252, 117, 82, 98, 169, 32, 115, 21,
        35, 50, 53, 67, 27, 136, 81, 226, 216, 82, 66, 238, 222, 110, 186, 123, 240, 42, 20, 148,
        207, 148, 12, 142, 63, 182, 29, 212, 57, 131, 0, 170, 120, 10, 23, 189, 11, 235, 28, 179,
        74, 121, 113, 2, 221, 186, 107, 255, 251, 89, 11, 47, 2, 26, 49, 122, 60, 88, 229, 205, 31,
        187, 151, 96, 87, 142, 217, 14, 253, 16, 60, 76, 245, 88, 227, 82, 182, 195, 131, 220, 197,
        181, 9, 83, 107, 95, 0, 0, 0, 255, 255,
    ];
    const OUTPUT: &str = r#"{"t":null,"s":null,"op":10,"d":{"heartbeat_interval":41250,"_trace":["[\"gateway-prd-main-858d\",{\"micros\":0.0}]"]}}"#;

    #[test]
    fn complete_message() {
        let mut decompressor = Decompressor::new();
        assert!(decompressor.partial.is_empty());
        assert_eq!(
            decompressor.decompress(MESSAGE).unwrap(),
            Some(OUTPUT.to_owned())
        );

        assert!(decompressor.partial.is_empty());
    }

    #[test]
    fn split_message() {
        let mut decompressor = Decompressor::new();
        assert!(decompressor.partial.is_empty());
        assert_eq!(
            decompressor
                .decompress(&MESSAGE[0..MESSAGE.len() / 2])
                .unwrap(),
            None
        );
        assert!(!decompressor.partial.is_empty());

        assert_eq!(
            decompressor
                .decompress(&MESSAGE[MESSAGE.len() / 2..])
                .unwrap(),
            Some(OUTPUT.to_owned()),
        );
        assert!(decompressor.partial.is_empty());
    }

    #[test]
    fn invalid_is_none() {
        let mut decompressor = Decompressor::new();
        assert_eq!(decompressor.decompress(&[]).unwrap(), None);

        assert_eq!(
            decompressor
                .decompress(&MESSAGE[..MESSAGE.len() - 2])
                .unwrap(),
            None
        );
    }

    #[test]
    fn reset() {
        let mut decompressor = Decompressor::new();
        assert_eq!(
            decompressor
                .decompress(&MESSAGE[..MESSAGE.len() - 2])
                .unwrap(),
            None
        );

        decompressor.reset();
        assert_eq!(
            decompressor.decompress(MESSAGE).unwrap(),
            Some(OUTPUT.to_owned())
        );
    }
}
