use crate::{
    guild::Emoji,
    id::{
        marker::{ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

/// Sent when someone sends an effect, such as an emoji reaction, in a voice channel the current user is connected to.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceChannelEffectSend {
    /// The ID of the emoji animation, for emoji reaction effects
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_id: Option<u64>,
    /// The type of emoji animation, for emoji reaction effects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_type: Option<VoiceChannelEffectAnimationType>,
    /// ID of the channel the effect was sent in.
    pub channel_id: Id<ChannelMarker>,
    /// The emoji sent, for emoji reaction effects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,
    /// ID of the guild the effect was sent in.
    pub guild_id: Id<GuildMarker>,
    /// ID of the user who sent the effect
    pub user_id: Id<UserMarker>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum VoiceChannelEffectAnimationType {
    /// A fun animation, sent by a Nitro subscriber.
    Premium,
    /// The standard animation.
    Basic,
}

#[cfg(test)]
mod tests {
    use super::{VoiceChannelEffectAnimationType, VoiceChannelEffectSend};
    use crate::{guild::Emoji, id::Id};
    use serde_test::Token;

    #[test]
    fn voice_channel_effect_send() {
        let value = VoiceChannelEffectSend {
            animation_id: Some(42),
            animation_type: Some(VoiceChannelEffectAnimationType::Premium),
            channel_id: Id::new(1),
            emoji: Some(Emoji {
                animated: true,
                available: true,
                id: Id::new(6),
                managed: true,
                name: "test".to_string(),
                require_colons: true,
                roles: vec![],
                user: None,
            }),
            guild_id: Id::new(42),
            user_id: Id::new(24),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "VoiceChannelEffectSend",
                    len: 6,
                },
                Token::Str("animation_id"),
                Token::Some,
                Token::U64(42),
                Token::Str("animation_type"),
                Token::Some,
                Token::Enum {
                    name: "VoiceChannelEffectAnimationType",
                },
                Token::Str("Premium"),
                Token::Unit,
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("emoji"),
                Token::Some,
                Token::Struct {
                    name: "Emoji",
                    len: 6,
                },
                Token::Str("animated"),
                Token::Bool(true),
                Token::Str("available"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::Str("managed"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("require_colons"),
                Token::Bool(true),
                Token::StructEnd,
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("42"),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("24"),
                Token::StructEnd,
            ],
        )
    }
}
