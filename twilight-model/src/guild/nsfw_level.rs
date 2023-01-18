use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct NSFWLevel(u8);

impl NSFWLevel {
    pub const DEFAULT: Self = Self::new(0);
    pub const EXPLICIT: Self = Self::new(1);
    pub const SAFE: Self = Self::new(2);
    pub const AGE_RESTRICTED: Self = Self::new(3);

    /// Create a new NSFW Level from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`EXPLICIT`][`Self::EXPLICIT`].
    pub const fn new(connection_visibility: u8) -> Self {
        Self(connection_visibility)
    }

    /// Retrieve the value of the NSFW Level.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::NSFWLevel;
    ///
    /// assert_eq!(2, NSFWLevel::SAFE.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::AGE_RESTRICTED => "AGE_RESTRICTED",
            Self::DEFAULT => "DEFAULT",
            Self::EXPLICIT => "EXPLICIT",
            Self::SAFE => "SAFE",
            _ => return None,
        })
    }
}

impl Debug for NSFWLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("NSFWLevel")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("NSFWLevel").field(&self.0).finish()
        }
    }
}

impl From<u8> for NSFWLevel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<NSFWLevel> for u8 {
    fn from(value: NSFWLevel) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::NSFWLevel;
    use serde_test::Token;

    const MAP: &[(NSFWLevel, u8)] = &[
        (NSFWLevel::DEFAULT, 0),
        (NSFWLevel::EXPLICIT, 1),
        (NSFWLevel::SAFE, 2),
        (NSFWLevel::AGE_RESTRICTED, 3),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[Token::NewtypeStruct { name: "NSFWLevel" }, Token::U8(*num)],
            );
            assert_eq!(*kind, NSFWLevel::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
