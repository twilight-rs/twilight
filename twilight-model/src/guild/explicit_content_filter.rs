use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ExplicitContentFilter(u8);

impl ExplicitContentFilter {
    pub const NONE: Self = Self::new(0);

    pub const MEMBERS_WITHOUT_ROLE: Self = Self::new(1);

    pub const ALL_MEMBERS: Self = Self::new(2);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::ALL_MEMBERS => "ALL_MEMBERS",
            Self::MEMBERS_WITHOUT_ROLE => "MEMBERS_WITHOUT_ROLE",
            Self::NONE => "NONE",
            _ => return None,
        })
    }
}

impl_typed!(ExplicitContentFilter, u8);

#[cfg(test)]
mod tests {
    use super::ExplicitContentFilter;
    use serde_test::Token;

    const MAP: &[(ExplicitContentFilter, u8)] = &[
        (ExplicitContentFilter::NONE, 0),
        (ExplicitContentFilter::MEMBERS_WITHOUT_ROLE, 1),
        (ExplicitContentFilter::ALL_MEMBERS, 2),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "ExplicitContentFilter",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, ExplicitContentFilter::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
