use serde::{Deserialize, Serialize};

use crate::id::{
    marker::{ChannelMarker, EmojiMarker, OnboardingPromptOptionMarker, RoleMarker},
    Id,
};

/// A prompt option for a guild onboarding screen.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct OnboardingPromptOption {
    /// The channels opted into when this option is selected
    #[serde(default)]
    channel_ids: Vec<Id<ChannelMarker>>,
    /// The description of the option.
    description: String,
    /// The emoji id if the emoji is custom.
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_id: Option<Id<EmojiMarker>>,
    /// The emoji name if custom, the unicode character if standard, or [`None`] if no emoji is set
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji_name: Option<String>,
    /// The id of the option
    id: Id<OnboardingPromptOptionMarker>,
    /// The roles assigned when this option is selected
    #[serde(default)]
    role_ids: Vec<Id<RoleMarker>>,
    /// The title of the option.
    title: String,
}

#[cfg(test)]
mod tests {
    use serde_test::Token;

    use super::OnboardingPromptOption;
    use crate::id::{
        marker::{ChannelMarker, EmojiMarker, OnboardingPromptOptionMarker, RoleMarker},
        Id,
    };

    #[test]
    fn prompt_option() {
        let option = OnboardingPromptOption {
            channel_ids: Vec::from([
                Id::<ChannelMarker>::new(1),
                Id::<ChannelMarker>::new(2),
                Id::<ChannelMarker>::new(3),
            ]),
            description: String::from("an option description"),
            emoji_id: Some(Id::<EmojiMarker>::new(7)),
            emoji_name: Some(String::from("test")),
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
                    len: 7,
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
                Token::Str("an option description"),
                Token::Str("emoji_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("7"),
                Token::Str("emoji_name"),
                Token::Some,
                Token::Str("test"),
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
