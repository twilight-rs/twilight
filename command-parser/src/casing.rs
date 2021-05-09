use unicase::UniCase;

/// Case sensitivity of a command.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CaseSensitivity {
    /// A case-insensitive command. "ping" and "Ping" are equivalent.
    Insensitive(UniCase<String>),
    /// A case-sensitive command. "ping" and "Ping" are distinguished from each
    /// other.
    Sensitive(String),
}

impl CaseSensitivity {
    pub fn is_sensitive(&self) -> bool {
        matches!(self, Self::Sensitive(_))
    }
}

impl AsRef<str> for CaseSensitivity {
    fn as_ref(&self) -> &str {
        match self {
            Self::Insensitive(u) => u.as_str(),
            Self::Sensitive(s) => s.as_str(),
        }
    }
}

impl AsMut<str> for CaseSensitivity {
    fn as_mut(&mut self) -> &mut str {
        match self {
            Self::Insensitive(u) => u.as_mut_str(),
            Self::Sensitive(s) => s.as_mut_str(),
        }
    }
}

impl PartialEq<str> for CaseSensitivity {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::Insensitive(u) => u == &UniCase::new(other),
            Self::Sensitive(s) => s == other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CaseSensitivity;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        CaseSensitivity: AsRef<str>,
        Clone,
        Debug,
        Eq,
        Hash,
        PartialEq,
        PartialEq<str>,
        Send,
        Sync
    );
}
