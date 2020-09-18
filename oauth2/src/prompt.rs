use serde::{Deserialize, Serialize};

/// Whether to prompt the user again when they have already authorized the
/// application.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum Prompt {
    /// Always ask the user for consent.
    Consent,
    /// Don't ask the user for consent.
    None,
}

impl Prompt {
    /// Return the name of the prompt.
    ///
    /// This is equivalent to what you would get when serializing it.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_oauth2::Prompt;
    ///
    /// assert_eq!("consent", Prompt::Consent.name());
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Self::Consent => "consent",
            Self::None => "none",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Prompt;

    #[test]
    fn test_prompts() {
        assert_eq!("consent", Prompt::Consent.name());
        assert_eq!("none", Prompt::None.name());
    }
}
