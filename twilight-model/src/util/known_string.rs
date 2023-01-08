//! Bytes container on the stack intended for efficient, constant-time string
//! storage.

use serde::{
    de::{Deserialize, Deserializer, Error as DeError, Visitor},
    ser::{Serialize, Serializer},
};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    str::{self, FromStr},
};

/// Bytes container with some abstractions intended for storing strings.
///
/// We want to be able to pattern match types with string values, but
/// there's some tricky aspects about storing types with string values that
/// results in our having to store bytes.
///
/// Say that we have a type like this:
///
/// ```compile_fail
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// pub struct Letter(String);
///
/// impl Letter {
///     pub const A: Self = Self("A".to_owned());
/// }
/// ```
///
/// This clearly won't compile because String is a heap allocation, but we're
/// needing to work in constant expressions. Similarly, storing &'static str
/// won't work because deserialization from serde contexts won't have static
/// results -- well, they can operate off a known list of values, but then we
/// can't have unknown values, which defeats the purpose of the exercise.
///
/// This leads us to considering another solution: [`Cow`]. [`Cow`]s can store
/// both borrowed and owned strings. However, this also fails to compile:
///
/// ```compile_fail
/// use serde::{Deserialize, Serialize};
/// use std::borrow::Cow;
///
/// #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// pub struct Letter(Cow<'static, str>);
///
/// impl Letter {
///     pub const A: Self = Self(Cow::Borrowed("A"));
///     pub const B: Self = Self(Cow::Borrowed("B"));
///     pub const C: Self = Self(Cow::Borrowed("C"));
///
///     fn print_sound(&self) {
///         println!("{}", match self {
///             &Self::A => "ayy",
///             &Self::B => "bee",
///             &Self::C => "sea",
///         });
///     }
/// }
/// ```
///
/// The reason for this is unobvious: it's because `Cow` doesn't derive `Eq`
/// and `PartialEq`. It can't because String doesn't in constant expressions. We
/// get this error on each of the constant evaluations:
///
/// > to use a constant of type `Cow` in a pattern, `Cow` must be annotated with
/// > `#[derive(PartialEq, Eq)]`
///
/// This brings us to another solution: storing an array of bytes. Because
/// arrays are on the stack and derive `Eq` and `PartialEq`, we *can* pattern
/// match them:
///
/// ```
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// pub struct Letter([u8; 1]);
///
/// impl Letter {
///     pub const A: Self = Self::new(b"A");
///     pub const B: Self = Self::new(b"B");
///     pub const C: Self = Self::new(b"C");
///
///     const fn new(input: &[u8]) -> Self {
///         Self([input[0]])
///     }
///
///     fn print_sound(self) {
///         println!(
///             "{}",
///             match self {
///                 Self::A => "ayy",
///                 Self::B => "bee",
///                 Self::C => "sea",
///                 _ => "unknown",
///             }
///         );
///     }
/// }
/// ```
///
/// As a bonus, we get the efficiency of storing on the stack, low allocation
/// sizes (subject to the length of the bytes array), and we get to derive Copy,
/// which means match statements look pleasant.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct KnownString<const LENGTH: usize> {
    bytes: [u8; LENGTH],
}

impl<const LENGTH: usize> KnownString<LENGTH> {
    /// Create a known value from a string.
    ///
    /// Returns `None` if the string is larger than the container size.
    pub fn from_str(value: &str) -> Option<Self> {
        if value.len() > LENGTH {
            return None;
        }

        let mut bytes = [0; LENGTH];

        for (index, byte) in value.as_bytes().iter().enumerate() {
            bytes[index] = *byte;
        }

        Some(Self { bytes })
    }

    /// Create a known value from a slice of bytes.
    ///
    /// # Panics
    ///
    /// Panics if the input is larger than the allocated number of bytes. This
    /// is okay to do since this is only called by Twilight's associated
    /// constants and methods with constrained types, such as
    /// [`AutoArchiveDuration::new`].
    ///
    /// [`AutoArchiveDuration::new`]: crate::channel::thread::AutoArchiveDuration::new
    pub const fn from_bytes(input: &[u8]) -> Self {
        assert!(
            input.len() <= LENGTH,
            "input is greater than allocated length"
        );

        let mut bytes = [0; LENGTH];
        let mut index = 0;

        while index < input.len() {
            let byte = input[index];

            if byte == 0x00 {
                break;
            }

            bytes[index] = byte;
            index += 1;
        }

        Self { bytes }
    }

    /// Parse the known value as a string, trimming null bytes.
    ///
    /// # Panics
    ///
    /// Panics if the value is not UTF-8 valid.
    pub fn get(&self) -> &str {
        let string = str::from_utf8(&self.bytes).unwrap();

        string.trim_matches(char::from(0))
    }
}

impl<const LENGTH: usize> AsRef<str> for KnownString<LENGTH> {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl<const LENGTH: usize> Debug for KnownString<LENGTH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.get())
    }
}

pub struct KnownStringVisitor<const LENGTH: usize>;

impl<'de, const LENGTH: usize> Visitor<'de> for KnownStringVisitor<LENGTH> {
    type Value = KnownString<LENGTH>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("string")
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        KnownString::from_str(v).ok_or_else(|| DeError::custom("string is too long"))
    }

    fn visit_string<E: DeError>(self, v: String) -> Result<Self::Value, E> {
        KnownString::from_str(&v).ok_or_else(|| DeError::custom("string is too long"))
    }
}

impl<'de, const LENGTH: usize> Deserialize<'de> for KnownString<LENGTH> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(KnownStringVisitor::<LENGTH> {})
    }
}

impl<const LENGTH: usize> FromStr for KnownString<LENGTH> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        KnownString::try_from(s)
    }
}

impl<const LENGTH: usize> Serialize for KnownString<LENGTH> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self.as_ref())
    }
}

impl<const LENGTH: usize> ToString for KnownString<LENGTH> {
    fn to_string(&self) -> String {
        self.get().to_owned()
    }
}

impl<const LENGTH: usize> TryFrom<&str> for KnownString<LENGTH> {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::KnownString;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash, str::FromStr, string::ToString};

    assert_impl_all!(
        KnownString<1>: AsRef<str>,
        Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        FromStr,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
        ToString,
        TryFrom<&'static str>,
    );

    #[test]
    fn new() {
        let string = KnownString::<64>::from_str("BOT").unwrap();

        let mut with_null_bytes = [0; 64];
        with_null_bytes[0] = b'B';
        with_null_bytes[1] = b'O';
        with_null_bytes[2] = b'T';
        assert_eq!(&string.bytes, &with_null_bytes);
    }

    #[test]
    fn get() {
        assert_eq!(
            "identify",
            KnownString::<64>::from_str("identify").unwrap().get()
        );
    }

    #[test]
    fn serde() {
        let string = KnownString::<64>::from_str("test").unwrap();

        serde_test::assert_tokens(&string, &[Token::Str("test")]);
    }

    #[test]
    fn equality() {
        assert_eq!(
            KnownString::<64>::from_str("test").unwrap(),
            KnownString::<64>::from_str("test").unwrap()
        );
        assert_ne!(
            KnownString::<64>::from_str("foo"),
            KnownString::<64>::from_str("bar")
        );
    }
}
