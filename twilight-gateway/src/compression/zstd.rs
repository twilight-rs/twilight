//! Zstd transport compression.

use super::CompressionError;
use std::fmt;

/// Gateway event decompressor.
pub struct Decompressor {
    /// Common decompressed message buffer.
    buffer: Box<[u8]>,
    /// Reusable zstd decompression context.
    ctx: zstd_safe::DCtx<'static>,
}

impl Decompressor {
    /// Create a new decompressor for a shard.
    pub fn new() -> Self {
        Self {
            buffer: vec![0; super::BUFFER_SIZE].into_boxed_slice(),
            ctx: zstd_safe::DCtx::create(),
        }
    }

    /// Decompress a message.
    ///
    /// # Errors
    ///
    /// Returns a [`CompressionErrorType::Decompressing`] error type if the
    /// message could not be decompressed.
    ///
    /// Returns a [`CompressionErrorType::NotUtf8`] error type if the
    /// decompressed message is not UTF-8.
    pub fn decompress(&mut self, message: &[u8]) -> Result<String, CompressionError> {
        let mut input = zstd_safe::InBuffer::around(message);

        // Decompressed message. `Vec::extend_from_slice` efficiently allocates
        // only what's necessary.
        let mut decompressed = Vec::new();

        loop {
            let mut output = zstd_safe::OutBuffer::around(self.buffer.as_mut());

            self.ctx
                .decompress_stream(&mut output, &mut input)
                .map_err(CompressionError::from_code)?;

            decompressed.extend_from_slice(output.as_slice());

            // Break when message has been fully decompressed.
            if input.pos == input.src.len() && output.pos() != output.capacity() {
                break;
            }
        }

        String::from_utf8(decompressed).map_err(CompressionError::from_utf8_error)
    }

    /// Reset the decompressor's internal state.
    pub fn reset(&mut self) {
        self.ctx
            .reset(zstd_safe::ResetDirective::SessionOnly)
            .expect("resetting session is infallible");
    }
}

impl fmt::Debug for Decompressor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Decompressor")
            .field("buffer", &self.buffer)
            .field("ctx", &"<decompression context>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Decompressor;

    const MESSAGE: [u8; 117] = [
        40, 181, 47, 253, 0, 64, 100, 3, 0, 66, 7, 25, 28, 112, 137, 115, 116, 40, 208, 203, 85,
        255, 167, 74, 75, 126, 203, 222, 231, 255, 151, 18, 211, 212, 171, 144, 151, 210, 255, 51,
        4, 49, 34, 71, 98, 2, 36, 253, 122, 141, 99, 203, 225, 11, 162, 47, 133, 241, 6, 201, 82,
        245, 91, 206, 247, 164, 226, 156, 92, 108, 130, 123, 11, 95, 199, 15, 61, 179, 117, 157,
        28, 37, 65, 64, 25, 250, 182, 8, 199, 205, 44, 73, 47, 19, 218, 45, 27, 14, 245, 202, 81,
        82, 122, 167, 121, 71, 173, 61, 140, 190, 15, 3, 1, 0, 36, 74, 18,
    ];
    const OUTPUT: &str = r#"{"t":null,"s":null,"op":10,"d":{"heartbeat_interval":41250,"_trace":["[\"gateway-prd-us-east1-c-7s4x\",{\"micros\":0.0}]"]}}"#;

    #[test]
    fn message() {
        let mut decompressor = Decompressor::new();
        assert_eq!(decompressor.decompress(&MESSAGE).unwrap(), OUTPUT);
    }

    #[test]
    fn reset() {
        let mut decompressor = Decompressor::new();
        decompressor
            .decompress(&MESSAGE[..MESSAGE.len() - 2])
            .unwrap();

        assert!(decompressor.decompress(&MESSAGE).is_err());
        decompressor.reset();
        assert_eq!(decompressor.decompress(&MESSAGE).unwrap(), OUTPUT);
    }
}
