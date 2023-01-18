use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ExplicitContentFilter(u8);

impl ExplicitContentFilter {
    pub const NONE: Self = Self::new(0);
    pub const MEMBERS_WITHOUT_ROLE: Self = Self::new(1);
    pub const ALL_MEMBERS: Self = Self::new(2);

    /// Create a new explicit content filter from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`NONE`][`Self::NONE`].
    pub const fn new(explicit_content_filter: u8) -> Self {
        Self(explicit_content_filter)
    }

    /// Retrieve the value of the explicit content filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::ExplicitContentFilter;
    ///
    /// assert_eq!(2, ExplicitContentFilter::ALL_MEMBERS.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

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

impl Debug for ExplicitContentFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("ExplicitContentFilter")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("ExplicitContentFilter")
                .field(&self.0)
                .finish()
        }
    }
}

impl From<u8> for ExplicitContentFilter {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<ExplicitContentFilter> for u8 {
    fn from(value: ExplicitContentFilter) -> Self {
        value.get()
    }
}

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
