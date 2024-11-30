use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer, Error as DeError, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt::{Formatter, Result as FmtResult};

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct Permissions: u64 {
        const CREATE_INVITE = 1;
        const KICK_MEMBERS = 1 << 1;
        const BAN_MEMBERS = 1 << 2;
        const ADMINISTRATOR = 1 << 3;
        const MANAGE_CHANNELS = 1 << 4;
        const MANAGE_GUILD = 1 << 5;
        const ADD_REACTIONS = 1 << 6;
        const VIEW_AUDIT_LOG = 1 << 7;
        const PRIORITY_SPEAKER = 1 << 8;
        const STREAM = 1 << 9;
        const VIEW_CHANNEL = 1 << 10;
        /// Allows for sending messages and creating forum threads, but not
        /// sending messages in forum threads.
        const SEND_MESSAGES = 1 << 11;
        const SEND_TTS_MESSAGES = 1 << 12;
        const MANAGE_MESSAGES = 1 << 13;
        const EMBED_LINKS = 1 << 14;
        const ATTACH_FILES = 1 << 15;
        const READ_MESSAGE_HISTORY = 1 << 16;
        const MENTION_EVERYONE = 1 << 17;
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        const CONNECT = 1 << 20;
        const SPEAK = 1 << 21;
        const MUTE_MEMBERS = 1 << 22;
        const DEAFEN_MEMBERS = 1 << 23;
        const MOVE_MEMBERS = 1 << 24;
        const USE_VAD = 1 << 25;
        const CHANGE_NICKNAME = 1 << 26;
        const MANAGE_NICKNAMES = 1 << 27;
        const MANAGE_ROLES = 1 << 28;
        const MANAGE_WEBHOOKS = 1 << 29;
        #[deprecated(since = "0.15.2", note = "use `MANAGE_GUILD_EXPRESSIONS` instead")]
        const MANAGE_EMOJIS_AND_STICKERS = 1 << 30;
        /// Allows management and editing of emojis, stickers, and soundboard sounds.
        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;
        const USE_SLASH_COMMANDS = 1 << 31;
        const REQUEST_TO_SPEAK = 1 << 32;
        /// Allows for creating, editing, and deleting scheduled events.
        const MANAGE_EVENTS = 1 << 33;
        /// Allows for deleting and archiving threads, and viewing all private threads.
        const MANAGE_THREADS = 1 << 34;
        /// Allows for creating public threads.
        const CREATE_PUBLIC_THREADS = 1 << 35;
        /// Allows for creating private threads.
        const CREATE_PRIVATE_THREADS = 1 << 36;
        /// Allows the usage of custom stickers from other servers.
        const USE_EXTERNAL_STICKERS = 1 << 37;
        /// Allows for sending messages in threads.
        const SEND_MESSAGES_IN_THREADS = 1 << 38;
        /// Allows for using activities (applications with the `EMBEDDED`
        /// flag) in a voice channel.
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;
        /// Allows for timing out users to prevent them from sending or reacting
        /// to messages in chat and threads, and from speaking in voice and
        /// stage channels.
        ///
        /// See Discord's article on [Guild Timeouts].
        ///
        /// [Guild Timeouts]: https://support.discord.com/hc/en-us/articles/4413305239191-Time-Out-FAQ
        const MODERATE_MEMBERS = 1 << 40;
        /// Allows for viewing role subscription insights.
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        /// Allows for using soundboard in a voice channel
        const USE_SOUNDBOARD = 1 << 42;
        /// Allows the usage of custom soundboard sounds from other servers
        const USE_EXTERNAL_SOUNDS = 1 << 45;
        /// Allows sending voice messages
        const SEND_VOICE_MESSAGES = 1 << 46;
        /// Allows sending polls.
        const SEND_POLLS = 1 << 49;
        /// Allows user-installed apps to send public responses. When
        /// disabled, users will still be allowed to use their apps
        /// but the responses will be ephemeral. This only applies to
        /// apps not also installed to the server.
        const USE_EXTERNAL_APPS = 1 << 50;
    }
}

struct PermissionsVisitor;

impl Visitor<'_> for PermissionsVisitor {
    type Value = Permissions;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("integer or string permissions")
    }

    fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Permissions::from_bits_truncate(v))
    }

    fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
        #[allow(clippy::map_err_ignore)]
        let num = v
            .parse()
            .map_err(|_| DeError::custom("permissions is not valid bitflags"))?;

        self.visit_u64(num)
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(PermissionsVisitor)
    }
}

impl Serialize for Permissions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.bits().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Permissions;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{
        fmt::{Binary, Debug, LowerHex, Octal, UpperHex},
        hash::Hash,
        ops::{
            BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign,
        },
    };

    assert_impl_all!(
        Permissions: Binary,
        BitAnd,
        BitAndAssign,
        BitOr,
        BitOrAssign,
        BitXor,
        BitXorAssign,
        Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Extend<Permissions>,
        FromIterator<Permissions>,
        Hash,
        LowerHex,
        Not,
        Octal,
        PartialEq,
        Send,
        Serialize,
        Sub,
        SubAssign,
        Sync,
        UpperHex
    );
    const_assert_eq!(Permissions::CREATE_INVITE.bits(), 1);
    const_assert_eq!(Permissions::KICK_MEMBERS.bits(), 1 << 1);
    const_assert_eq!(Permissions::BAN_MEMBERS.bits(), 1 << 2);
    const_assert_eq!(Permissions::ADMINISTRATOR.bits(), 1 << 3);
    const_assert_eq!(Permissions::MANAGE_CHANNELS.bits(), 1 << 4);
    const_assert_eq!(Permissions::MANAGE_GUILD.bits(), 1 << 5);
    const_assert_eq!(Permissions::ADD_REACTIONS.bits(), 1 << 6);
    const_assert_eq!(Permissions::VIEW_AUDIT_LOG.bits(), 1 << 7);
    const_assert_eq!(Permissions::PRIORITY_SPEAKER.bits(), 1 << 8);
    const_assert_eq!(Permissions::STREAM.bits(), 1 << 9);
    const_assert_eq!(Permissions::VIEW_CHANNEL.bits(), 1 << 10);
    const_assert_eq!(Permissions::SEND_MESSAGES.bits(), 1 << 11);
    const_assert_eq!(Permissions::SEND_TTS_MESSAGES.bits(), 1 << 12);
    const_assert_eq!(Permissions::MANAGE_MESSAGES.bits(), 1 << 13);
    const_assert_eq!(Permissions::EMBED_LINKS.bits(), 1 << 14);
    const_assert_eq!(Permissions::ATTACH_FILES.bits(), 1 << 15);
    const_assert_eq!(Permissions::READ_MESSAGE_HISTORY.bits(), 1 << 16);
    const_assert_eq!(Permissions::MENTION_EVERYONE.bits(), 1 << 17);
    const_assert_eq!(Permissions::USE_EXTERNAL_EMOJIS.bits(), 1 << 18);
    const_assert_eq!(Permissions::VIEW_GUILD_INSIGHTS.bits(), 1 << 19);
    const_assert_eq!(Permissions::CONNECT.bits(), 1 << 20);
    const_assert_eq!(Permissions::SPEAK.bits(), 1 << 21);
    const_assert_eq!(Permissions::MUTE_MEMBERS.bits(), 1 << 22);
    const_assert_eq!(Permissions::DEAFEN_MEMBERS.bits(), 1 << 23);
    const_assert_eq!(Permissions::MOVE_MEMBERS.bits(), 1 << 24);
    const_assert_eq!(Permissions::USE_VAD.bits(), 1 << 25);
    const_assert_eq!(Permissions::CHANGE_NICKNAME.bits(), 1 << 26);
    const_assert_eq!(Permissions::MANAGE_NICKNAMES.bits(), 1 << 27);
    const_assert_eq!(Permissions::MANAGE_ROLES.bits(), 1 << 28);
    const_assert_eq!(Permissions::MANAGE_WEBHOOKS.bits(), 1 << 29);
    const_assert_eq!(Permissions::MANAGE_GUILD_EXPRESSIONS.bits(), 1 << 30);
    const_assert_eq!(Permissions::USE_SLASH_COMMANDS.bits(), 1 << 31);
    const_assert_eq!(Permissions::REQUEST_TO_SPEAK.bits(), 1 << 32);
    const_assert_eq!(Permissions::MANAGE_EVENTS.bits(), 1 << 33);
    const_assert_eq!(Permissions::MANAGE_THREADS.bits(), 1 << 34);
    const_assert_eq!(Permissions::CREATE_PUBLIC_THREADS.bits(), 1 << 35);
    const_assert_eq!(Permissions::CREATE_PRIVATE_THREADS.bits(), 1 << 36);
    const_assert_eq!(Permissions::USE_EXTERNAL_STICKERS.bits(), 1 << 37);
    const_assert_eq!(Permissions::SEND_MESSAGES_IN_THREADS.bits(), 1 << 38);
    const_assert_eq!(Permissions::USE_EMBEDDED_ACTIVITIES.bits(), 1 << 39);
    const_assert_eq!(Permissions::MODERATE_MEMBERS.bits(), 1 << 40);
    const_assert_eq!(
        Permissions::VIEW_CREATOR_MONETIZATION_ANALYTICS.bits(),
        1 << 41
    );
    const_assert_eq!(Permissions::USE_SOUNDBOARD.bits(), 1 << 42);
    const_assert_eq!(Permissions::USE_EXTERNAL_SOUNDS.bits(), 1 << 45);
    const_assert_eq!(Permissions::SEND_VOICE_MESSAGES.bits(), 1 << 46);
    const_assert_eq!(Permissions::SEND_POLLS.bits(), 1 << 49);
    const_assert_eq!(Permissions::USE_EXTERNAL_APPS.bits(), 1 << 50);

    #[test]
    fn serde() {
        serde_test::assert_tokens(&Permissions::CREATE_INVITE, &[Token::Str("1")]);
        // Deserialization truncates unknown bits.
        serde_test::assert_de_tokens(&Permissions::empty(), &[Token::Str("9223372036854775808")]);
    }
}
