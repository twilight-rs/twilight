use crate::util::known_string::KnownString;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Status(KnownString<16>);

impl Status {
    pub const DO_NOT_DISTURB: Self = Self::from_bytes(b"dnd");

    pub const IDLE: Self = Self::from_bytes(b"idle");

    pub const INVISIBLE: Self = Self::from_bytes(b"invisible");

    pub const OFFLINE: Self = Self::from_bytes(b"offline");

    pub const ONLINE: Self = Self::from_bytes(b"online");

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::DO_NOT_DISTURB => "DO_NOT_DISTURB",
            Self::IDLE => "IDLE",
            Self::INVISIBLE => "INVISIBLE",
            Self::OFFLINE => "OFFLINE",
            Self::ONLINE => "ONLINE",
            _ => return None,
        })
    }
}

impl_typed!(Status, String);

#[cfg(test)]
mod tests {
    use super::Status;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash, str::FromStr, string::ToString};

    assert_impl_all!(
        Status: AsRef<str>,
        Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        FromStr,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync,
        ToString,
        TryFrom<&'static str>,
    );

    const MAP: &[(Status, &str)] = &[
        (Status::DO_NOT_DISTURB, "dnd"),
        (Status::IDLE, "idle"),
        (Status::INVISIBLE, "invisible"),
        (Status::OFFLINE, "offline"),
        (Status::ONLINE, "online"),
    ];

    #[test]
    fn variants() {
        for (kind, name) in MAP {
            serde_test::assert_tokens(
                kind,
                &[Token::NewtypeStruct { name: "Status" }, Token::Str(name)],
            );
            assert_eq!(Some(*kind), Status::new(name));
            assert_eq!(*name, kind.as_ref());
            assert_eq!(Ok(*kind), Status::from_str(name));
            assert_eq!(Ok(*kind), Status::try_from(*name));
            assert_eq!(name, &kind.to_string());
            assert_eq!(*name, kind.get());
        }
    }
}
