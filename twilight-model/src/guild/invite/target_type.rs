use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TargetType(u8);

impl TargetType {
    pub const STREAM: Self = Self::new(1);

    pub const EMBEDDED_APPLICATION: Self = Self::new(2);

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

impl_typed!(TargetType, u8);

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
