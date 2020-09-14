use unicase::UniCase;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CaseSensitivity {
    Insensitive(UniCase<String>),
    Sensitive(String),
}

impl AsRef<str> for CaseSensitivity {
    fn as_ref(&self) -> &str {
        match self {
            Self::Insensitive(u) => u.as_str(),
            Self::Sensitive(s) => s.as_str(),
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
