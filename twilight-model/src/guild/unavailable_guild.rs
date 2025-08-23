use crate::{
    id::{Id, marker::GuildMarker},
    util::mustbe::MustBeBool,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct UnavailableGuild {
    pub id: Id<GuildMarker>,
    pub unavailable: bool,
}

impl<'de> Deserialize<'de> for UnavailableGuild {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "UnavailableGuild")] // Tests expect this struct name
        struct UnavailableGuildIntermediate {
            id: Id<GuildMarker>,
            #[allow(unused)] // Only used in the derived impl
            unavailable: MustBeBool<true>,
        }

        let intermediate = UnavailableGuildIntermediate::deserialize(deserializer)?;

        Ok(Self {
            id: intermediate.id,
            unavailable: true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::UnavailableGuild;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn unavailable_guild() {
        let value = UnavailableGuild {
            id: Id::new(1),
            unavailable: true,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "UnavailableGuild",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("unavailable"),
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
