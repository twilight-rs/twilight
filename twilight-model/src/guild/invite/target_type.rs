use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TargetType(u8);

impl TargetType {
    pub const STREAM: Self = Self::new(1);

    pub const EMBEDDED_APPLICATION: Self = Self::new(2);

    /// Create a new command type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`STREAM`][`Self::STREAM`].
    pub const fn new(target_type: u8) -> Self {
        Self(target_type)
    }

    /// Retrieve the value of the command type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::invite::TargetType;
    ///
    /// assert_eq!(2, TargetType::EMBEDDED_APPLICATION.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::EMBEDDED_APPLICATION => "EMBEDDED_APPLICATION",
            Self::STREAM => "STREAM",
            _ => return None,
        })
    }
}

impl Debug for TargetType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("TargetType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("TargetType").field(&self.0).finish()
        }
    }
}

impl From<u8> for TargetType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<TargetType> for u8 {
    fn from(value: TargetType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::TargetType;
    use serde_test::Token;

    const MAP: &[(TargetType, u8)] = &[
        (TargetType::STREAM, 1),
        (TargetType::EMBEDDED_APPLICATION, 2),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[Token::NewtypeStruct { name: "TargetType" }, Token::U8(*num)],
            );
            assert_eq!(*kind, TargetType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
