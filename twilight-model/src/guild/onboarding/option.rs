use crate::{
    guild::Emoji,
    id::{
        marker::{ChannelMarker, OnboardingPromptOptionMarker, RoleMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

/// A prompt option for a guild onboarding screen.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OnboardingPromptOption {
    /// The channels opted into when this option is selected
    pub channel_ids: Vec<Id<ChannelMarker>>,
    /// The description of the option.
    pub description: Option<String>,
    /// The emoji of the option.
    pub emoji: Emoji,
    /// The id of the option
    pub id: Id<OnboardingPromptOptionMarker>,
    /// The roles assigned when this option is selected
    pub role_ids: Vec<Id<RoleMarker>>,
    /// The title of the option.
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::OnboardingPromptOption;
    use crate::{
        guild::Emoji,
        id::{
            marker::{ChannelMarker, EmojiMarker, OnboardingPromptOptionMarker, RoleMarker},
            Id,
        },
    };
    use serde_test::Token;

    #[test]
    fn prompt_option() {
        let option = OnboardingPromptOption {
            channel_ids: Vec::from([
                Id::<ChannelMarker>::new(1),
                Id::<ChannelMarker>::new(2),
                Id::<ChannelMarker>::new(3),
            ]),
            description: Some(String::from("an option description")),
            emoji: Emoji {
                animated: None,
                available: None,
                id: Some(Id::<EmojiMarker>::new(7)),
                managed: None,
                name: Some(String::from("test")),
                require_colons: None,
                roles: None,
                user: None,
            },
            id: Id::<OnboardingPromptOptionMarker>::new(123_456_789),
            role_ids: Vec::from([
                Id::<RoleMarker>::new(4),
                Id::<RoleMarker>::new(5),
                Id::<RoleMarker>::new(6),
            ]),
            title: String::from("an option"),
        };

        serde_test::assert_tokens(
            &option,
            &[
                Token::Struct {
                    name: "OnboardingPromptOption",
                    len: 6,
                },
                Token::Str("channel_ids"),
                Token::Seq { len: Some(3) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::SeqEnd,
                Token::Str("description"),
                Token::Some,
                Token::Str("an option description"),
                Token::Str("emoji"),
                Token::Struct {
                    name: "Emoji",
                    len: 2,
                },
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("7"),
                Token::Str("name"),
                Token::Some,
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123456789"),
                Token::Str("role_ids"),
                Token::Seq { len: Some(3) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::SeqEnd,
                Token::Str("title"),
                Token::Str("an option"),
                Token::StructEnd,
            ],
        )
    }
}
