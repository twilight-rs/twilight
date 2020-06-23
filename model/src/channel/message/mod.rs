mod activity;
mod activity_type;
mod application;
mod flags;
mod kind;
mod reaction;
mod reference;

pub use self::{
    activity::MessageActivity, activity_type::MessageActivityType, application::MessageApplication,
    flags::MessageFlags, kind::MessageType, reaction::MessageReaction, reference::MessageReference,
};

use crate::{
    channel::{embed::Embed, Attachment, ChannelMention},
    guild::PartialMember,
    id::{ChannelId, GuildId, MessageId, RoleId, UserId, WebhookId},
    user::User,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Message {
    pub id: MessageId,
    pub activity: Option<MessageActivity>,
    pub application: Option<MessageApplication>,
    pub attachments: Vec<Attachment>,
    pub author: User,
    pub channel_id: ChannelId,
    pub content: String,
    pub edited_timestamp: Option<String>,
    pub embeds: Vec<Embed>,
    pub flags: Option<MessageFlags>,
    pub guild_id: Option<GuildId>,
    #[serde(rename = "type")]
    pub kind: MessageType,
    pub member: Option<PartialMember>,
    #[serde(default)]
    pub mention_channels: Vec<ChannelMention>,
    pub mention_everyone: bool,
    pub mention_roles: Vec<RoleId>,
    #[serde(with = "serde_mappable_seq")]
    pub mentions: HashMap<UserId, User>,
    pub pinned: bool,
    #[serde(default)]
    pub reactions: Vec<MessageReaction>,
    #[serde(rename = "message_reference")]
    pub reference: Option<MessageReference>,
    pub timestamp: String,
    pub tts: bool,
    pub webhook_id: Option<WebhookId>,
}

#[cfg(test)]
mod tests {
    use super::{Message, MessageFlags, MessageType};
    use crate::{
        guild::PartialMember,
        id::{ChannelId, GuildId, MessageId, UserId},
        user::User,
    };
    use std::collections::HashMap;

    #[test]
    fn test_message_deserialization() {
        let input = serde_json::json!({
            "attachments": [],
            "author": {
                "avatar": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "discriminator": "0001",
                "id": "3",
                "username": "test",
            },
            "channel_id": "2",
            "content": "ping",
            "edited_timestamp": null,
            "embeds": [],
            "flags": 0,
            "guild_id": "1",
            "id": "4",
            "member": {
                "deaf": false,
                "hoisted_role": null,
                "joined_at": "2020-01-01T00:00:00.000000+00:00",
                "mute": false,
                "nick": null,
                "premium_since": null,
                "roles": [],
            },
            "mention_everyone": false,
            "mention_roles": [],
            "mentions": [],
            "pinned": false,
            "timestamp": "2020-02-02T02:02:02.020000+00:00",
            "tts": false,
            "type": 0,
        });

        let expected = Message {
            activity: None,
            application: None,
            attachments: Vec::new(),
            author: User {
                avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                bot: false,
                discriminator: "0001".to_owned(),
                email: None,
                flags: None,
                id: UserId(3),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
            channel_id: ChannelId(2),
            content: "ping".to_owned(),
            edited_timestamp: None,
            embeds: Vec::new(),
            flags: Some(MessageFlags::empty()),
            guild_id: Some(GuildId(1)),
            id: MessageId(4),
            kind: MessageType::Regular,
            member: Some(PartialMember {
                deaf: false,
                joined_at: Some("2020-01-01T00:00:00.000000+00:00".to_owned()),
                mute: false,
                roles: Vec::new(),
            }),
            mention_channels: Vec::new(),
            mention_everyone: false,
            mention_roles: Vec::new(),
            mentions: HashMap::new(),
            pinned: false,
            reactions: Vec::new(),
            reference: None,
            timestamp: "2020-02-02T02:02:02.020000+00:00".to_owned(),
            tts: false,
            webhook_id: None,
        };

        assert_eq!(expected, serde_json::from_value(input).unwrap());
    }
}
