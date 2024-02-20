use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

/// Defines the criteria used to satisfy Onboarding constraints that are required for enabling.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum OnboardingMode {
    /// Counts only Default Channels towards constraints
    OnboardingDefault,
    /// Counts Default Channels and Questions towards constraint.
    OnboardingAdvanced,
    /// Variant is unknown to the library.
    Unknown(u8),
}

impl From<u8> for OnboardingMode {
    fn from(value: u8) -> Self {
        match value {
            0 => OnboardingMode::OnboardingDefault,
            1 => OnboardingMode::OnboardingAdvanced,
            unknown => OnboardingMode::Unknown(unknown),
        }
    }
}

impl From<OnboardingMode> for u8 {
    fn from(value: OnboardingMode) -> Self {
        match value {
            OnboardingMode::OnboardingDefault => 0,
            OnboardingMode::OnboardingAdvanced => 1,
            OnboardingMode::Unknown(unknown) => unknown,
        }
    }
}

impl OnboardingMode {
    /// Name of the onboarding mode.
    pub const fn name(self) -> &'static str {
        match self {
            Self::OnboardingDefault => "OnboardingDefault",
            Self::OnboardingAdvanced => "OnboardingAdvanced",
            Self::Unknown(_) => "Unknown",
        }
    }
}

impl Display for OnboardingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        OnboardingMode: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn variants() {
        serde_test::assert_tokens(&OnboardingMode::OnboardingDefault, &[Token::U8(0)]);
        serde_test::assert_tokens(&OnboardingMode::OnboardingAdvanced, &[Token::U8(1)]);
        serde_test::assert_tokens(&OnboardingMode::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!(
            "OnboardingDefault",
            OnboardingMode::OnboardingDefault.name()
        );
        assert_eq!(
            "OnboardingAdvanced",
            OnboardingMode::OnboardingAdvanced.name()
        );
        assert_eq!("Unknown", OnboardingMode::Unknown(99).name());
    }
}
