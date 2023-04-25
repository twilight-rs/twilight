use crate::id::{
    marker::{ChannelMarker, EmojiMarker, OnboardingPromptOptionMarker, RoleMarker},
    Id,
};
use serde::{Deserialize, Serialize};

/// An emoji for a guild onboarding prompt.
/// This is used instead [`Emoji`] as both it's id and name can be `null` in prompt options.
///
/// [`Emoji`]: crate::guild::Emoji
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OnboardingPromptEmoji {
    name: Option<String>,
    id: Option<Id<EmojiMarker>>,
    #[serde(default)]
    animated: bool,
}

/// A prompt option for a guild onboarding screen.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OnboardingPromptOption {
    /// The channels opted into when this option is selected
    pub channel_ids: Vec<Id<ChannelMarker>>,
    /// The description of the option.
    pub description: Option<String>,
    /// The emoji of the option.
    pub emoji: OnboardingPromptEmoji,
    /// The id of the option
    pub id: Id<OnboardingPromptOptionMarker>,
    /// The roles assigned when this option is selected
    pub role_ids: Vec<Id<RoleMarker>>,
    /// The title of the option.
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::{OnboardingPromptEmoji, OnboardingPromptOption};
    use crate::id::{
        marker::{ChannelMarker, EmojiMarker, OnboardingPromptOptionMarker, RoleMarker},
        Id,
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
}
