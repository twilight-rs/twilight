#[cfg(feature = "compression")]
mod inflater;

use super::r#impl::ReceivingEventError;
use std::str;

#[cfg(feature = "compression")]
use inflater::Inflater;

/// Interface for working with buffers variable on the `compression` feature flag.
#[derive(Debug)]
pub struct Compression {
    /// Inflater for use with compression.
    #[cfg(feature = "compression")]
    inner: Inflater,
    /// Buffer for use without compression.
    #[cfg(not(feature = "compression"))]
    inner: Vec<u8>,
}

impl Compression {
    /// Create a new buffer, abstracting over an inflater if the `compression`
    /// feature is enabled or a simple `Vec` if the feature is disabled.
    #[cfg_attr(
        not(feature = "compression"),
        allow(clippy::missing_const_for_fn, unused_variables)
    )]
    pub fn new(shard_id: [u64; 2]) -> Self {
        Self {
            #[cfg(feature = "compression")]
            inner: Inflater::new(shard_id),
            #[cfg(not(feature = "compression"))]
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
        #[cfg(feature = "compression")]
        {
            self.inner.buffer_mut()
        }

        #[cfg(not(feature = "compression"))]
        self.inner.as_mut_slice()
    }

    /// Mutable reference to the internal buffer slice as a mutable string slice.
    ///
    /// # Safety
    ///
    /// Ensuring that the internal buffer slice is UTF-8 valid is left to the
    /// caller to determine.
    pub unsafe fn buffer_str_mut(&mut self) -> &mut str {
        // SAFETY: ensuring safety is left to the caller.
        str::from_utf8_unchecked_mut(self.buffer_slice_mut())
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
        not(feature = "compression"),
        allow(clippy::unused_self, unused_variables)
    )]
    pub fn extend_binary(&mut self, bytes: &[u8]) -> bool {
        #[cfg(feature = "compression")]
        {
            self.inner.extend(bytes);

            true
        }

        #[cfg(not(feature = "compression"))]
        // Binary payloads are not received when compression is disabled.
        false
    }

    /// Extend the buffer with bytes from a Text websocket message.
    ///
    /// If compression is enabled then this will do nothing.
    ///
    /// Returns whether the inner buffer was extended.
    #[cfg_attr(feature = "compression", allow(clippy::unused_self, unused_variables))]
    pub fn extend_text(&mut self, bytes: &[u8]) -> bool {
        #[cfg(not(feature = "compression"))]
        {
            self.inner.extend_from_slice(bytes);

            true
        }

        #[cfg(feature = "compression")]
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
    #[cfg_attr(not(feature = "compression"), allow(clippy::unused_self))]
    pub fn message_mut(&mut self) -> Result<Option<&mut [u8]>, ReceivingEventError> {
        #[cfg(feature = "compression")]
        {
            use super::r#impl::ReceivingEventErrorType;

            self.inner.msg().map_err(|source| ReceivingEventError {
                kind: ReceivingEventErrorType::Decompressing,
                source: Some(Box::new(source)),
            })
        }

        #[cfg(not(feature = "compression"))]
        Ok(None)
    }

    /// Reset the buffer for a new gateway session.
    pub fn reset(&mut self) {
        #[cfg(feature = "compression")]
        self.inner.reset();

        #[cfg(not(feature = "compression"))]
        self.clear();
    }
}

/// Add a toggle to a gateway connection URL depending on whether compression is
/// enabled.
///
/// If compression is enabled then the `compress` query parameter is appended
/// with a value of `zlib-stream`.
#[cfg_attr(not(feature = "compression"), allow(unused_variables))]
pub fn add_url_feature(buf: &mut String) {
    #[cfg(feature = "compression")]
    buf.push_str("&compress=zlib-stream");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_url_features() {
        let mut buf = String::new();
        super::add_url_feature(&mut buf);

        #[cfg(feature = "compression")]
        {
            assert_eq!("&compress=zlib-stream", buf);
        }

        #[cfg(not(feature = "compression"))]
        assert!(buf.is_empty());
    }
}
