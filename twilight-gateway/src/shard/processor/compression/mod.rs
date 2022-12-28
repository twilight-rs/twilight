#[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
mod inflater;

use super::r#impl::ReceivingEventError;

#[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
use inflater::Inflater;

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
    pub fn new(shard_id: [u64; 2]) -> Self {
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
    ///
    /// If compression is disabled then this will do nothing.
    ///
    /// Returns whether the inner buffer was extended.
    #[cfg_attr(
        not(any(feature = "zlib-stock", feature = "zlib-simd")),
        allow(clippy::unused_self, unused_variables)
    )]
    pub fn extend_binary(&mut self, bytes: &[u8]) -> bool {
        #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
        {
            self.inner.extend(bytes);

            true
        }

        #[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
        // Binary payloads are not received when compression is disabled.
        false
    }

    /// Extend the buffer with bytes from a Text websocket message.
    ///
    /// If compression is enabled then this will do nothing.
    ///
    /// Returns whether the inner buffer was extended.
    #[cfg_attr(
        any(feature = "zlib-stock", feature = "zlib-simd"),
        allow(clippy::unused_self, unused_variables)
    )]
    pub fn extend_text(&mut self, bytes: &[u8]) -> bool {
        #[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
        {
            self.inner.extend_from_slice(bytes);

            true
        }

        #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
        // Text payloads are not received when compression is enabled.
        false
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
        allow(clippy::unused_self)
    )]
    pub fn message_mut(&mut self) -> Result<Option<&mut [u8]>, ReceivingEventError> {
        #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
        {
            use super::r#impl::ReceivingEventErrorType;

            self.inner.msg().map_err(|source| ReceivingEventError {
                kind: ReceivingEventErrorType::Decompressing,
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
}

/// Add a toggle to a gateway connection URL depending on whether compression is
/// enabled.
///
/// If compression is enabled then the `compress` query parameter is appended
/// with a value of `zlib-stream`.
#[cfg_attr(
    not(any(feature = "zlib-stock", feature = "zlib-simd")),
    allow(clippy::ptr_arg, unused_variables)
)]
pub fn add_url_feature(buf: &mut String) {
    #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
    buf.push_str("&compress=zlib-stream");
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_url_features() {
        let mut buf = String::new();
        super::add_url_feature(&mut buf);

        #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
        {
            assert_eq!("&compress=zlib-stream", buf);
        }

        #[cfg(not(any(feature = "zlib-stock", feature = "zlib-simd")))]
        assert!(buf.is_empty());
    }
}
