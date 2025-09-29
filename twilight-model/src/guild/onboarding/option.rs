use crate::{
    guild::Emoji,
    id::{
        Id,
        marker::{ChannelMarker, EmojiMarker, OnboardingPromptOptionMarker, RoleMarker},
    },
};
use serde::{Deserialize, Serialize};

/// An emoji for a guild onboarding prompt.
/// This is used instead of [`Emoji`] as both it's id and name can be `null` in
/// prompt options.
///
/// [`Emoji`]: crate::guild::Emoji
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OnboardingPromptEmoji {
    pub name: Option<String>,
    pub id: Option<Id<EmojiMarker>>,
    #[serde(default)]
    pub animated: bool,
}

impl From<Emoji> for OnboardingPromptEmoji {
    fn from(value: Emoji) -> Self {
        Self {
            animated: value.animated,
            id: Some(value.id),
            name: Some(value.name),
        }
    }
}

/// A prompt option for a guild onboarding screen.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OnboardingPromptOption {
    /// Channels opted into when this option is selected.
    pub channel_ids: Vec<Id<ChannelMarker>>,
    /// Description of the option.
    pub description: Option<String>,
    /// Emoji of the option.
    pub emoji: OnboardingPromptEmoji,
    /// ID of the option.
    pub id: Id<OnboardingPromptOptionMarker>,
    /// Roles assigned when this option is selected.
    pub role_ids: Vec<Id<RoleMarker>>,
    /// Title of the option.
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::{OnboardingPromptEmoji, OnboardingPromptOption};
    use crate::{
        guild::Emoji,
        id::{
            Id,
            marker::{ChannelMarker, EmojiMarker, OnboardingPromptOptionMarker, RoleMarker},
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
            emoji: OnboardingPromptEmoji {
                animated: false,
                id: Some(Id::<EmojiMarker>::new(7)),
                name: Some(String::from("test")),
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
                    name: "OnboardingPromptEmoji",
                    len: 3,
                },
                Token::Str("name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("7"),
                Token::Str("animated"),
                Token::Bool(false),
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

    #[test]
    fn conversion() {
        let emoji = Emoji {
            animated: false,
            available: false,
            id: Id::<EmojiMarker>::new(7),
            managed: false,
            name: String::from("test"),
            require_colons: false,
            roles: Vec::new(),
            user: None,
        };

        let emoji: OnboardingPromptEmoji = emoji.into();

        assert!(!emoji.animated);
        assert_eq!(emoji.id, Some(Id::<EmojiMarker>::new(7)));
        assert_eq!(emoji.name, Some(String::from("test")));
    }
}
