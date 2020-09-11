use unicase::UniCase;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
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
