//! Efficient parsing and storage of Discord image hashes.
//!
//! In a simple implementation, Discord image hashes are stored in a
//! [`std::string::String`]. Using an [`ImageHash`], only 17 bytes are required
//! to store a Discord hash.
//!
//! [`ImageHash::parse`] is used to parse provided bytes, along with
//! [`std::convert::TryFrom`], [`std::str::FromStr`], and `serde`
//! implementations. The input is assumed to be only in ASCII format.
//! [`ImageHash::bytes`] and [`ImageHash::is_animated`] may be used to
//! deconstruct the hash and [`ImageHash::new`] may be used to reconstruct one.

#![allow(dead_code, unused_mut)]

use serde::{
    de::{Deserialize, Deserializer, Error as DeError, Visitor},
    ser::{Serialize, Serializer},
};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/// Key indicating an animated image hash.
const ANIMATED_KEY: &str = "a_";

/// Length of an image hash.
const HASH_LEN: usize = 32;

/// Parsing an image hash into an efficient storage format via
/// [`ImageHash::parse`] failed.
#[derive(Debug)]
pub struct ImageHashParseError {
    kind: ImageHashParseErrorType,
}

impl ImageHashParseError {
    /// Error with an [`ImageHashParseErrorType::Format`] error type.
    const FORMAT: Self = ImageHashParseError {
        kind: ImageHashParseErrorType::Format,
    };

    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ImageHashParseErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        ImageHashParseErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }

    /// Instantiate a new error with an [`ImageHashParseErrorType::Range`] error
    /// type.
    const fn range(index: usize, value: u8) -> Self {
        Self {
            kind: ImageHashParseErrorType::Range { index, value },
        }
    }
}

impl Display for ImageHashParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            ImageHashParseErrorType::Format => {
                f.write_str("image hash isn't in a discord image hash format")
            }
            ImageHashParseErrorType::Range { index, value } => {
                f.write_str("value (")?;
                Display::fmt(&value, f)?;
                f.write_str(") at encountered index (")?;
                Display::fmt(&index, f)?;

                f.write_str(") is not an acceptable value")
            }
        }
    }
}

impl Error for ImageHashParseError {}

/// Type of [`ImageHashParseError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ImageHashParseErrorType {
    /// Input is either animated and not 34 characters long or is not animated
    /// and is not 32 characters long.
    Format,
    /// Input is out of the accepted range ('0' to '9', 'a' to 'f').
    Range {
        /// Index of the byte.
        index: usize,
        /// Byte of the value out of the acceptable range.
        value: u8,
    },
}

/// Efficient storage for Discord image hashes.
///
/// This works by storing image hashes as packed integers rather than
/// heap-allocated [`std::string::String`]s.
///
/// Parsing methods only support hashes provided by Discord's APIs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ImageHash {
    /// Whether the image is animated.
    ///
    /// This is denoted in the input by a prefixed `a_`.
    animated: bool,
    bytes: [u8; 16],
}

impl ImageHash {
    /// Instantiate a new hash from its raw parts.
    ///
    /// Parts can be obtained via [`is_animated`] and [`bytes`].
    ///
    /// # Examples
    ///
    /// Parse an image hash, deconstruct it, and then reconstruct it:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::util::ImageHash;
    ///
    /// let input = "1acefe340fafb4ecefae407f3abdb323";
    /// let parsed = ImageHash::parse(input.as_bytes())?;
    ///
    /// let (bytes, is_animated) = (parsed.bytes(), parsed.is_animated());
    ///
    /// let constructed = ImageHash::new(bytes, is_animated);
    /// assert_eq!(input, constructed.to_string());
    /// # Ok(()) }
    /// ```
    ///
    /// [`is_animated`]: Self::is_animated
    /// [`bytes`]: Self::bytes
    pub const fn new(bytes: [u8; 16], animated: bool) -> Self {
        Self { animated, bytes }
    }

    /// Parse an image hash into an efficient integer-based storage.
    ///
    /// # Examples
    ///
    /// Parse a static image hash:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::util::ImageHash;
    ///
    /// let input = "b2a6536641da91a0b59bd66557c56c36";
    /// let parsed = ImageHash::parse(input.as_bytes())?;
    ///
    /// assert!(!parsed.is_animated());
    /// assert_eq!(input, parsed.to_string());
    /// # Ok(()) }
    /// ```
    ///
    /// Parse an animated image hash:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::util::ImageHash;
    ///
    /// let input = "a_b2a6536641da91a0b59bd66557c56c36";
    /// let parsed = ImageHash::parse(input.as_bytes())?;
    ///
    /// assert!(parsed.is_animated());
    /// assert_eq!(input, parsed.to_string());
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`ImageHashParseErrorType::Format`] error type if the
    /// provided value isn't in a Discord image hash format. Refer to the
    /// variant's documentation for more details.
    ///
    /// Returns an [`ImageHashParseErrorType::Range`] error type if one of the
    /// hex values is outside of the accepted range. Refer to the variant's
    /// documentation for more details.
    pub const fn parse(value: &[u8]) -> Result<Self, ImageHashParseError> {
        /// Number of digits allocated in the half-byte.
        ///
        /// In other words, the number of numerical representations before
        /// reaching alphabetical representations in the half-byte.
        const DIGITS_ALLOCATED: u8 = 10;

        let animated = Self::starts_with(value, ANIMATED_KEY.as_bytes());

        let mut seeking_idx = if animated { ANIMATED_KEY.len() } else { 0 };
        let mut storage_idx = 0;

        if value.len() - seeking_idx != HASH_LEN {
            return Err(ImageHashParseError::FORMAT);
        }

        let mut bits = 0;

        while seeking_idx < value.len() {
            let byte = match value[seeking_idx] {
                byte @ b'0'..=b'9' => byte - b'0',
                byte @ b'a'..=b'f' => byte - b'a' + DIGITS_ALLOCATED,
                other => return Err(ImageHashParseError::range(seeking_idx, other)),
            };

            bits |= (byte as u128) << 124_usize.saturating_sub(storage_idx * 4);
            seeking_idx += 1;
            storage_idx += 1;
        }

        Ok(Self {
            animated,
            bytes: bits.to_le_bytes(),
        })
    }

    /// Efficient packed bytes of the hash.
    ///
    /// Can be paired with [`is_animated`] for use in [`new`] to recreate the
    /// efficient image hash.
    ///
    /// # Examples
    ///
    /// Parse an image hash and then check the packed bytes:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::util::ImageHash;
    ///
    /// let input = b"f49d812ca33c1cbbeec96b9f64487c7c";
    /// let hash = ImageHash::parse(input)?;
    /// let bytes = hash.bytes();
    ///
    /// // Byte correlates to 12 (c) followed by 7 (7).
    /// assert_eq!(0b0111_1100, bytes[0]);
    ///
    /// // Byte correlates to 4 (4) followed by 15 (f).
    /// assert_eq!(0b1111_0100, bytes[15]);
    /// # Ok(()) }
    /// ```
    ///
    /// [`is_animated`]: Self::is_animated
    /// [`new`]: Self::new
    pub const fn bytes(self) -> [u8; 16] {
        self.bytes
    }

    /// Whether the hash is for an animated image.
    ///
    /// # Examples
    ///
    /// Parse an animated image hash prefixed with `a_` and another static image
    /// hash that is not prefixed with `a_`:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::util::ImageHash;
    ///
    /// let animated_input = "a_5145104ad8e8c9e765883813e4abbcc8";
    /// let animated_hash = ImageHash::parse(animated_input.as_bytes())?;
    /// assert!(animated_hash.is_animated());
    ///
    /// let static_input = "c7e7c4b8469d790cb9b293759e60953d";
    /// let static_hash = ImageHash::parse(static_input.as_bytes())?;
    /// assert!(!static_hash.is_animated());
    /// # Ok(()) }
    /// ```
    pub const fn is_animated(self) -> bool {
        self.animated
    }

    /// Create an iterator over the [nibbles] of the hexadecimal image hash.
    ///
    /// # Examples
    ///
    /// Parse an image hash and then iterate over the nibbles:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::util::ImageHash;
    ///
    /// let input = b"1d9811c4cd3782148915c522b02878fc";
    /// let hash = ImageHash::parse(input)?;
    /// let mut nibbles = hash.nibbles();
    ///
    /// assert_eq!(Some(b'1'), nibbles.next());
    /// assert_eq!(Some(b'd'), nibbles.next());
    /// assert_eq!(Some(b'c'), nibbles.nth(29));
    /// assert!(nibbles.next().is_none());
    /// # Ok(()) }
    /// ```
    ///
    /// [nibbles]: https://en.wikipedia.org/wiki/Nibble
    pub const fn nibbles(self) -> Nibbles {
        Nibbles::new(self)
    }

    /// Determine whether a haystack starts with a needle.
    const fn starts_with(haystack: &[u8], needle: &[u8]) -> bool {
        if needle.len() > haystack.len() {
            return false;
        }

        let mut idx = 0;

        while idx < needle.len() {
            if haystack[idx] != needle[idx] {
                return false;
            }

            idx += 1;
        }

        true
    }
}

impl<'de> Deserialize<'de> for ImageHash {
    /// Parse an image hash string into an efficient decimal store.
    ///
    /// # Examples
    ///
    /// Refer to [`ImageHash::parse`]'s documentation for examples.
    ///
    /// # Errors
    ///
    /// Returns an [`ImageHashParseErrorType::Format`] error type if the
    /// provided value isn't in a Discord image hash format. Refer to the
    /// variant's documentation for more details.
    ///
    /// Returns an [`ImageHashParseErrorType::Range`] error type if one of the
    /// hex values is outside of the accepted range. Refer to the variant's
    /// documentation for more details.
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ImageHashVisitor;

        impl Visitor<'_> for ImageHashVisitor {
            type Value = ImageHash;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("image hash")
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                ImageHash::parse(v.as_bytes()).map_err(DeError::custom)
            }
        }

        deserializer.deserialize_any(ImageHashVisitor)
    }
}

impl Display for ImageHash {
    /// Format the image hash as a hex string.
    ///
    /// # Examples
    ///
    /// Parse a hash and then format it to ensure it matches the input:
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_model::util::ImageHash;
    ///
    /// let input = "a_b0e09d6697b11e9c79a89e5e3756ddee";
    /// let parsed = ImageHash::parse(input.as_bytes())?;
    ///
    /// assert_eq!(input, parsed.to_string());
    /// # Ok(()) }
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.is_animated() {
            f.write_str(ANIMATED_KEY)?;
        }

        for hex_value in self.nibbles() {
            let legible = char::from(hex_value);

            Display::fmt(&legible, f)?;
        }

        Ok(())
    }
}

impl FromStr for ImageHash {
    type Err = ImageHashParseError;

    /// Parse an image hash string into an efficient decimal store.
    ///
    /// # Examples
    ///
    /// Refer to [`ImageHash::parse`]'s documentation for examples.
    ///
    /// # Errors
    ///
    /// Returns an [`ImageHashParseErrorType::Format`] error type if the
    /// provided value isn't in a Discord image hash format. Refer to the
    /// variant's documentation for more details.
    ///
    /// Returns an [`ImageHashParseErrorType::Range`] error type if one of the
    /// hex values is outside of the accepted range. Refer to the variant's
    /// documentation for more details.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s.as_bytes())
    }
}

impl Serialize for ImageHash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl TryFrom<&[u8]> for ImageHash {
    type Error = ImageHashParseError;

    /// Parse an image hash string into an efficient decimal store.
    ///
    /// # Examples
    ///
    /// Refer to [`ImageHash::parse`]'s documentation for examples.
    ///
    /// # Errors
    ///
    /// Returns an [`ImageHashParseErrorType::Format`] error type if the
    /// provided value isn't in a Discord image hash format. Refer to the
    /// variant's documentation for more details.
    ///
    /// Returns an [`ImageHashParseErrorType::Range`] error type if one of the
    /// hex values is outside of the accepted range. Refer to the variant's
    /// documentation for more details.
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<&str> for ImageHash {
    type Error = ImageHashParseError;

    /// Parse an image hash string into an efficient decimal store.
    ///
    /// # Examples
    ///
    /// Refer to [`ImageHash::parse`]'s documentation for examples.
    ///
    /// # Errors
    ///
    /// Returns an [`ImageHashParseErrorType::Format`] error type if the
    /// provided value isn't in a Discord image hash format. Refer to the
    /// variant's documentation for more details.
    ///
    /// Returns an [`ImageHashParseErrorType::Range`] error type if one of the
    /// hex values is outside of the accepted range. Refer to the variant's
    /// documentation for more details.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes())
    }
}

/// Iterator over the [nibbles] of an image hash.
///
/// # Examples
///
/// Parse an image hash and then iterate over the nibbles:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use twilight_model::util::ImageHash;
///
/// let input = b"1d9811c4cd3782148915c522b02878fc";
/// let hash = ImageHash::parse(input)?;
/// let mut nibbles = hash.nibbles();
///
/// assert_eq!(Some(b'1'), nibbles.next());
/// assert_eq!(Some(b'd'), nibbles.next());
/// assert_eq!(Some(b'c'), nibbles.nth(29));
/// assert!(nibbles.next().is_none());
/// # Ok(()) }
/// ```
///
/// [nibbles]: https://en.wikipedia.org/wiki/Nibble
#[derive(Debug)]
pub struct Nibbles {
    /// Current index of the iterator.
    idx: usize,
    /// Hash being iterated over.
    inner: ImageHash,
}

impl Nibbles {
    /// Create a new iterator over an image hash, creating nibbles.
    const fn new(inner: ImageHash) -> Self {
        Self {
            idx: usize::MAX,
            inner,
        }
    }

    /// Advance the index in the iterator by the provided amount.
    fn advance_idx_by(&mut self, by: usize) {
        self.idx = if self.idx == usize::MAX {
            0
        } else {
            let mut new_idx = self.idx.saturating_add(by);

            if new_idx == usize::MAX {
                new_idx -= 1;
            }

            new_idx
        }
    }

    /// Parse the byte at the stored index.
    const fn byte(&self) -> Option<u8> {
        const BITS_IN_HALF_BYTE: u8 = 4;

        /// Greatest index that the hash byte array can be indexed into.
        const BYTE_ARRAY_BOUNDARY: usize = HASH_LEN - 1;

        const RIGHT_MASK: u8 = (1 << BITS_IN_HALF_BYTE) - 1;

        if self.idx >= HASH_LEN {
            return None;
        }

        let (byte, left) = ((BYTE_ARRAY_BOUNDARY - self.idx) / 2, self.idx % 2 == 0);

        let store = self.inner.bytes[byte];

        let bits = if left {
            store >> BITS_IN_HALF_BYTE
        } else {
            store & RIGHT_MASK
        };

        Some(Self::nibble(bits))
    }

    /// Convert 4 bits in a byte integer to a nibble.
    ///
    /// Values 0-9 correlate to representations '0' through '9', while values
    /// 10-15 correlate to representations 'a' through 'f'.
    const fn nibble(value: u8) -> u8 {
        if value < 10 {
            b'0' + value
        } else {
            b'a' + (value - 10)
        }
    }
}

impl DoubleEndedIterator for Nibbles {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.idx == usize::MAX {
            return None;
        }

        self.idx = self.idx.checked_sub(1)?;

        self.byte()
    }
}

impl ExactSizeIterator for Nibbles {
    fn len(&self) -> usize {
        HASH_LEN
    }
}

impl Iterator for Nibbles {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_idx_by(1);

        self.byte()
    }

    // Optimization to avoid the iterator from calling `next` n times.
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.advance_idx_by(n.saturating_add(1));

        self.byte()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (HASH_LEN, Some(HASH_LEN))
    }
}

#[cfg(test)]
mod tests {
    use super::{ImageHash, ImageHashParseError, ImageHashParseErrorType, Nibbles};
    use static_assertions::assert_impl_all;
    use std::{
        error::Error,
        fmt::{Debug, Display},
        hash::Hash,
    };

    assert_impl_all!(
        Nibbles: Debug,
        DoubleEndedIterator,
        ExactSizeIterator,
        Iterator,
        Send,
        Sync
    );
    assert_impl_all!(ImageHashParseErrorType: Debug, Send, Sync);
    assert_impl_all!(ImageHashParseError: Error, Send, Sync);
    assert_impl_all!(
        ImageHash: Clone,
        Debug,
        Display,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );

    /// Test that reconstruction of parted hashes is correct.
    #[test]
    fn new() -> Result<(), ImageHashParseError> {
        let source = ImageHash::parse(b"85362c0262ef125a1182b1fad66b6a89")?;
        let (bytes, animated) = (source.bytes(), source.is_animated());

        let reconstructed = ImageHash::new(bytes, animated);
        assert_eq!(reconstructed, source);

        Ok(())
    }

    #[test]
    fn parse() -> Result<(), ImageHashParseError> {
        let actual = ImageHash::parse(b"77450a7713f093adaebab32b18dacc46")?;
        let expected = [
            70, 204, 218, 24, 43, 179, 186, 174, 173, 147, 240, 19, 119, 10, 69, 119,
        ];
        assert_eq!(actual.bytes(), expected);

        Ok(())
    }

    #[test]
    fn display() -> Result<(), ImageHashParseError> {
        assert_eq!(
            "58ec815c650e72f8eb31eec52e54b3b5",
            ImageHash::parse(b"58ec815c650e72f8eb31eec52e54b3b5")?.to_string()
        );
        assert_eq!(
            "a_e382aeb1574bf3e4fe852f862bc4919c",
            ImageHash::parse(b"a_e382aeb1574bf3e4fe852f862bc4919c")?.to_string()
        );

        Ok(())
    }

    /// Test that various formats are indeed invalid.
    #[test]
    fn parse_format() {
        const INPUTS: &[&[u8]] = &[
            b"not correct length",
            b"",
            b"a_",
            // `a_` followed by 33 bytes.
            b"a_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            // 31 bytes.
            b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ];

        for input in INPUTS {
            assert!(matches!(
                ImageHash::parse(input).unwrap_err().kind(),
                &ImageHashParseErrorType::Format
            ));
        }
    }

    #[test]
    fn parse_range() {
        let mut input = [b'a'; 32];
        input[17] = b'-';

        assert!(matches!(
            ImageHash::parse(&input).unwrap_err().kind(),
            ImageHashParseErrorType::Range {
                index: 17,
                value: b'-',
            }
        ));
    }

    #[test]
    fn nibbles() -> Result<(), ImageHashParseError> {
        const INPUT: &[u8] = b"39eb706d6fbaeb22837c350993b97b42";

        let hash = ImageHash::parse(INPUT)?;
        let mut iter = hash.nibbles();

        for byte in INPUT.iter().copied() {
            assert_eq!(Some(byte), iter.next());
        }

        assert!(iter.next().is_none());

        Ok(())
    }

    /// Test that the [`core::iter::DoubleEndedIterator`] implementation on
    /// [`Nibbles`] functions like a double ended iterator should.
    #[test]
    fn nibbles_double_ended() -> Result<(), ImageHashParseError> {
        const INPUT: &[u8] = b"e72bbdec903c420b7aa9c45fc7994ac8";

        let hash = ImageHash::parse(INPUT)?;
        let mut iter = hash.nibbles();

        // Since we haven't started the iterator there should be no previous
        // item.
        assert!(iter.next_back().is_none());

        // This should be index 0 in the input.
        assert_eq!(Some(b'e'), iter.next());

        // We're at index 0, so there's nothing before that.
        assert!(iter.next_back().is_none());

        // Try somewhere in the middle.
        assert_eq!(Some(b'e'), iter.nth(5));

        // Skip all the way past the rest of the input to the last byte.
        assert_eq!(Some(b'8'), iter.nth(24));

        // Now that we're at the end, any additional retrievals will return no
        // item.
        assert!(iter.next().is_none());

        // Last input.
        assert_eq!(Some(b'8'), iter.next_back());

        // And finally, the next one should be None again.
        assert!(iter.next().is_none());

        Ok(())
    }

    /// Test that image hash parsing correctly identifies animated hashes by its
    /// `a_` prefix.
    #[test]
    fn is_animated() -> Result<(), ImageHashParseError> {
        assert!(ImageHash::parse(b"a_06c16474723fe537c283b8efa61a30c8")?.is_animated());
        assert!(!ImageHash::parse(b"06c16474723fe537c283b8efa61a30c8")?.is_animated());

        Ok(())
    }
}
