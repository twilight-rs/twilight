use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
