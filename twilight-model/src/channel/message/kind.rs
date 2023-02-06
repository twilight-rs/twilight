use crate::guild::Permissions;
use serde::{Deserialize, Serialize};

/// Type of a [`Message`].
///
/// Refer to [Discord Docs/Message Types] for more information.
///
/// [Discord Docs/Message Types]: https://discord.com/developers/docs/resources/channel#message-object-message-types
/// [`Message`]: super::Message
#[allow(missing_docs)]
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageType(u8);

impl MessageType {
    /// Regular message.
    pub const REGULAR: Self = Self::new(0);

    /// System message denoting a recipient has been added to a group.
    pub const RECIPIENT_ADD: Self = Self::new(1);

    /// System message denoting a recipient has been removed from a group.
    pub const RECIPIENT_REMOVE: Self = Self::new(2);

    /// System message denoting a call state, e.g. missed, started.
    pub const CALL: Self = Self::new(3);

    /// System message denoting a channel's name has been changed.
    pub const CHANNEL_NAME_CHANGE: Self = Self::new(4);

    /// System message denoting a channel's icon has been changed.
    pub const CHANNEL_ICON_CHANGE: Self = Self::new(5);

    /// System message denoting a message has been pinned.
    pub const CHANNEL_MESSAGE_PINNED: Self = Self::new(6);

    /// System message denoting a member has joined a guild.
    pub const USER_JOIN: Self = Self::new(7);

    /// System message denoting a user nitro boosted a guild.
    pub const GUILD_BOOST: Self = Self::new(8);

    /// System message denoting a user nitro boosted a guild to level 1.
    pub const GUILD_BOOST_TIER1: Self = Self::new(9);

    /// System message denoting a user nitro boosted a guild to level 2.
    pub const GUILD_BOOST_TIER2: Self = Self::new(10);

    /// System message denoting a user nitro boosted a guild to level 3.
    pub const GUILD_BOOST_TIER3: Self = Self::new(11);

    /// System message denoting a channel has been followed.
    pub const CHANNEL_FOLLOW_ADD: Self = Self::new(12);

    /// System message denoting a guild has been disqualified for Server Discovery.
    pub const GUILD_DISCOVERY_DISQUALIFIED: Self = Self::new(14);

    /// System message denoting a guild has been redisqualified for Server Discovery.
    pub const GUILD_DISCOVERY_REQUALIFIED: Self = Self::new(15);

    /// System message denoting an initial warning for Server Discovery disqualification.
    pub const GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING: Self = Self::new(16);

    /// System message denoting a final warning for Server Discovery disqualification.
    pub const GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING: Self = Self::new(17);

    /// Message is about a new thread.
    pub const THREAD_CREATED: Self = Self::new(18);

    /// Message is an inline reply.
    pub const REPLY: Self = Self::new(19);

    /// Message is a chat input command.
    pub const CHAT_INPUT_COMMAND: Self = Self::new(20);

    /// Message is the starter for a thread.
    pub const THREAD_STARTER_MESSAGE: Self = Self::new(21);

    /// Message is a reminder for a scheduled event.
    pub const GUILD_INVITE_REMINDER: Self = Self::new(22);

    /// Message is a context menu command use.
    pub const CONTEXT_MENU_COMMAND: Self = Self::new(23);

    /// Message is an auto moderation action.
    pub const AUTO_MODERATION_ACTION: Self = Self::new(24);

    /// System message denoting a user subscribed to a role.
    pub const ROLE_SUBSCRIPTION_PURCHASE: Self = Self::new(25);

    /// System message denoting a interaction premium upsell.
    pub const INTERACTION_PREMIUM_UPSELL: Self = Self::new(26);

    /// System message denoting a guild application premium subscription.
    pub const GUILD_APPLICATION_PREMIUM_SUBSCRIPTION: Self = Self::new(32);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::REGULAR => "REGULAR",
            Self::RECIPIENT_ADD => "RECIPIENT_ADD",
            Self::RECIPIENT_REMOVE => "RECIPIENT_REMOVE",
            Self::CALL => "CALL",
            Self::CHANNEL_NAME_CHANGE => "CHANNEL_NAME_CHANGE",
            Self::CHANNEL_ICON_CHANGE => "CHANNEL_ICON_CHANGE",
            Self::CHANNEL_MESSAGE_PINNED => "CHANNEL_MESSAGE_PINNED",
            Self::USER_JOIN => "USER_JOIN",
            Self::GUILD_BOOST => "GUILD_BOOST",
            Self::GUILD_BOOST_TIER1 => "GUILD_BOOST_TIER1",
            Self::GUILD_BOOST_TIER2 => "GUILD_BOOST_TIER2",
            Self::GUILD_BOOST_TIER3 => "GUILD_BOOST_TIER3",
            Self::CHANNEL_FOLLOW_ADD => "CHANNEL_FOLLOW_ADD",
            Self::GUILD_DISCOVERY_DISQUALIFIED => "GUILD_DISCOVERY_DISQUALIFIED",
            Self::GUILD_DISCOVERY_REQUALIFIED => "GUILD_DISCOVERY_REQUALIFIED",
            Self::GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING => {
                "GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING"
            }
            Self::GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING => {
                "GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING"
            }
            Self::THREAD_CREATED => "THREAD_CREATED",
            Self::REPLY => "REPLY",
            Self::CHAT_INPUT_COMMAND => "CHAT_INPUT_COMMAND",
            Self::THREAD_STARTER_MESSAGE => "THREAD_STARTER_MESSAGE",
            Self::GUILD_INVITE_REMINDER => "GUILD_INVITE_REMINDER",
            Self::CONTEXT_MENU_COMMAND => "CONTEXT_MENU_COMMAND",
            Self::AUTO_MODERATION_ACTION => "AUTO_MODERATION_ACTION",
            Self::INTERACTION_PREMIUM_UPSELL => "INTERACTION_PREMIUM_UPSELL",
            Self::GUILD_APPLICATION_PREMIUM_SUBSCRIPTION => {
                "GUILD_APPLICATION_PREMIUM_SUBSCRIPTION"
            }
            _ => return None,
        })
    }

    /// Whether the message can be deleted, not taking permissions into account.
    /// Some message types can't be deleted, even by server administrators.
    ///
    /// Some message types can only be deleted with certain permissions. For
    /// example, [`AUTO_MODERATION_ACTION`][`Self::AUTO_MODERATION_ACTION`] can only
    /// be deleted if the user has the
    /// [Manage Messages] permission.
    ///
    /// To check whether a message can be deleted while taking permissions into
    /// account, use
    /// [`deletable_with_permissions`][`Self::deletable_with_permissions`].
    ///
    /// [Manage Messages]: Permissions::MANAGE_MESSAGES
    pub const fn deletable(self) -> bool {
        matches!(
            self,
            Self::REGULAR
                | Self::CHANNEL_MESSAGE_PINNED
                | Self::USER_JOIN
                | Self::GUILD_BOOST
                | Self::GUILD_BOOST_TIER1
                | Self::GUILD_BOOST_TIER2
                | Self::GUILD_BOOST_TIER3
                | Self::CHANNEL_FOLLOW_ADD
                | Self::THREAD_CREATED
                | Self::REPLY
                | Self::CHAT_INPUT_COMMAND
                | Self::GUILD_INVITE_REMINDER
                | Self::CONTEXT_MENU_COMMAND
                | Self::AUTO_MODERATION_ACTION
                | Self::ROLE_SUBSCRIPTION_PURCHASE
                | Self::INTERACTION_PREMIUM_UPSELL
        )
    }

    /// Whether the message can be deleted, taking permissions into account.
    /// Some message types can't be deleted, even by server administrators.
    ///
    /// Some message types can only be deleted with certain permissions. For
    /// example, [`AUTO_MODERATION_ACTION`][`Self::AUTO_MODERATION_ACTION`] can only
    /// be deleted if the user has the [Manage Messages] permission.
    ///
    /// To check whether a message can be deleted *without* taking permissions
    /// into account, use [`deletable`][`Self::deletable`].
    ///
    /// [Manage Messages]: Permissions::MANAGE_MESSAGES
    pub const fn deletable_with_permissions(self, permissions: Permissions) -> bool {
        let required_permissions = match self {
            Self::AUTO_MODERATION_ACTION => Permissions::MANAGE_MESSAGES,
            _ => Permissions::empty(),
        };

        if !permissions.contains(required_permissions) {
            return false;
        }

        self.deletable()
    }
}

impl_typed!(MessageType, u8);

#[cfg(test)]
mod tests {
    use super::MessageType;
    use crate::guild::Permissions;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        MessageType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[allow(clippy::too_many_lines)]
    #[test]
    fn message_type() {
        const MAP: &[(MessageType, u8, bool)] = &[
            (MessageType::REGULAR, 0, true),
            (MessageType::RECIPIENT_ADD, 1, false),
            (MessageType::RECIPIENT_REMOVE, 2, false),
            (MessageType::CALL, 3, false),
            (MessageType::CHANNEL_NAME_CHANGE, 4, false),
            (MessageType::CHANNEL_ICON_CHANGE, 5, false),
            (MessageType::CHANNEL_MESSAGE_PINNED, 6, true),
            (MessageType::USER_JOIN, 7, true),
            (MessageType::GUILD_BOOST, 8, true),
            (MessageType::GUILD_BOOST_TIER1, 9, true),
            (MessageType::GUILD_BOOST_TIER2, 10, true),
            (MessageType::GUILD_BOOST_TIER3, 11, true),
            (MessageType::CHANNEL_FOLLOW_ADD, 12, true),
            (MessageType::GUILD_DISCOVERY_DISQUALIFIED, 14, false),
            (MessageType::GUILD_DISCOVERY_REQUALIFIED, 15, false),
            (
                MessageType::GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING,
                16,
                false,
            ),
            (
                MessageType::GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING,
                17,
                false,
            ),
            (MessageType::THREAD_CREATED, 18, true),
            (MessageType::REPLY, 19, true),
            (MessageType::CHAT_INPUT_COMMAND, 20, true),
            (MessageType::THREAD_STARTER_MESSAGE, 21, false),
            (MessageType::GUILD_INVITE_REMINDER, 22, true),
            (MessageType::CONTEXT_MENU_COMMAND, 23, true),
            (MessageType::AUTO_MODERATION_ACTION, 24, true),
            (MessageType::ROLE_SUBSCRIPTION_PURCHASE, 25, true),
            (MessageType::INTERACTION_PREMIUM_UPSELL, 26, true),
            (
                MessageType::GUILD_APPLICATION_PREMIUM_SUBSCRIPTION,
                32,
                false,
            ),
        ];

        for (message_type, number, deletable) in MAP {
            assert_eq!(*message_type, MessageType::from(*number));
            assert_eq!(*number, u8::from(*message_type));
            assert_eq!(*deletable, message_type.deletable());
            serde_test::assert_tokens(
                message_type,
                &[
                    Token::NewtypeStruct {
                        name: "MessageType",
                    },
                    Token::U8(*number),
                ],
            );
        }
    }

    #[test]
    fn deletable_with_permissions() {
        assert!(MessageType::AUTO_MODERATION_ACTION
            .deletable_with_permissions(Permissions::MANAGE_MESSAGES));
        assert!(
            !MessageType::AUTO_MODERATION_ACTION.deletable_with_permissions(Permissions::empty())
        );
    }
}
