//! Presets of permissions for different scenarios, such as [`PERMISSIONS_TEXT`]
//! which contains all of the permissions applicable to a text channel.

use super::bitops;
use twilight_model::guild::Permissions;

/// Permissions associated with sending messages in a guild text channel.
pub const PERMISSIONS_MESSAGING: Permissions = Permissions::from_bits_truncate(
    Permissions::ATTACH_FILES.bits()
        | Permissions::EMBED_LINKS.bits()
        | Permissions::MENTION_EVERYONE.bits()
        | Permissions::SEND_TTS_MESSAGES.bits(),
);

/// Permissions associated with a guild only at the root level (i.e. not channel
/// related).
pub const PERMISSIONS_ROOT_ONLY: Permissions = Permissions::from_bits_truncate(
    Permissions::ADMINISTRATOR.bits()
        | Permissions::BAN_MEMBERS.bits()
        | Permissions::CHANGE_NICKNAME.bits()
        | Permissions::KICK_MEMBERS.bits()
        | Permissions::MANAGE_GUILD_EXPRESSIONS.bits()
        | Permissions::MANAGE_GUILD.bits()
        | Permissions::MANAGE_NICKNAMES.bits()
        | Permissions::VIEW_AUDIT_LOG.bits()
        | Permissions::VIEW_GUILD_INSIGHTS.bits(),
);

/// Permissions associated with guild channels, omitting those in guild stage
/// channels.
pub const PERMISSIONS_STAGE_OMIT: Permissions = bitops::remove(
    bitops::insert(PERMISSIONS_TEXT, PERMISSIONS_VOICE),
    PERMISSIONS_STAGE,
);

/// Permissions associated with guild channels, omitting those in guild text
/// channels.
pub const PERMISSIONS_TEXT_OMIT: Permissions = bitops::remove(
    bitops::insert(PERMISSIONS_STAGE, PERMISSIONS_VOICE),
    PERMISSIONS_TEXT,
);

/// Permissions associated with guild channels, omitting those in guild voice
/// channels.
pub const PERMISSIONS_VOICE_OMIT: Permissions = bitops::remove(
    bitops::insert(PERMISSIONS_STAGE, PERMISSIONS_TEXT),
    PERMISSIONS_VOICE,
);

/// Permissions associated with guild stage channels.
const PERMISSIONS_STAGE: Permissions = Permissions::from_bits_truncate(
    Permissions::CONNECT.bits()
        | Permissions::CREATE_INVITE.bits()
        | Permissions::MANAGE_CHANNELS.bits()
        | Permissions::MANAGE_ROLES.bits()
        | Permissions::MOVE_MEMBERS.bits()
        | Permissions::MUTE_MEMBERS.bits()
        | Permissions::REQUEST_TO_SPEAK.bits()
        | Permissions::VIEW_CHANNEL.bits(),
);

/// Permissions associated with guild text channels.
const PERMISSIONS_TEXT: Permissions = Permissions::from_bits_truncate(
    Permissions::ADD_REACTIONS.bits()
        | Permissions::ATTACH_FILES.bits()
        | Permissions::CREATE_INVITE.bits()
        | Permissions::EMBED_LINKS.bits()
        | Permissions::MANAGE_CHANNELS.bits()
        | Permissions::MANAGE_MESSAGES.bits()
        | Permissions::MANAGE_ROLES.bits()
        | Permissions::MANAGE_WEBHOOKS.bits()
        | Permissions::MENTION_EVERYONE.bits()
        | Permissions::READ_MESSAGE_HISTORY.bits()
        | Permissions::SEND_MESSAGES.bits()
        | Permissions::SEND_TTS_MESSAGES.bits()
        | Permissions::USE_EXTERNAL_EMOJIS.bits()
        | Permissions::USE_SLASH_COMMANDS.bits()
        | Permissions::VIEW_CHANNEL.bits(),
);

/// Permissions associated with guild voice channels.
const PERMISSIONS_VOICE: Permissions = Permissions::from_bits_truncate(
    Permissions::CONNECT.bits()
        | Permissions::CREATE_INVITE.bits()
        | Permissions::DEAFEN_MEMBERS.bits()
        | Permissions::MANAGE_CHANNELS.bits()
        | Permissions::MANAGE_ROLES.bits()
        | Permissions::MOVE_MEMBERS.bits()
        | Permissions::MUTE_MEMBERS.bits()
        | Permissions::PRIORITY_SPEAKER.bits()
        | Permissions::SPEAK.bits()
        | Permissions::STREAM.bits()
        | Permissions::USE_VAD.bits()
        | Permissions::VIEW_CHANNEL.bits(),
);

#[cfg(test)]
mod tests {
    use super::{PERMISSIONS_STAGE_OMIT, PERMISSIONS_TEXT_OMIT, PERMISSIONS_VOICE_OMIT};
    use twilight_model::guild::Permissions;

    #[test]
    fn permissions_stage_omitted() {
        let expected = Permissions::ADD_REACTIONS
            | Permissions::PRIORITY_SPEAKER
            | Permissions::STREAM
            | Permissions::SEND_MESSAGES
            | Permissions::SEND_TTS_MESSAGES
            | Permissions::MANAGE_MESSAGES
            | Permissions::EMBED_LINKS
            | Permissions::ATTACH_FILES
            | Permissions::READ_MESSAGE_HISTORY
            | Permissions::MENTION_EVERYONE
            | Permissions::USE_EXTERNAL_EMOJIS
            | Permissions::SPEAK
            | Permissions::DEAFEN_MEMBERS
            | Permissions::USE_VAD
            | Permissions::MANAGE_WEBHOOKS
            | Permissions::USE_SLASH_COMMANDS;

        assert_eq!(expected, PERMISSIONS_STAGE_OMIT);
    }

    #[test]
    fn permissions_text_omitted() {
        let expected = Permissions::CONNECT
            | Permissions::DEAFEN_MEMBERS
            | Permissions::MOVE_MEMBERS
            | Permissions::MUTE_MEMBERS
            | Permissions::PRIORITY_SPEAKER
            | Permissions::REQUEST_TO_SPEAK
            | Permissions::SPEAK
            | Permissions::STREAM
            | Permissions::USE_VAD;

        assert_eq!(expected, PERMISSIONS_TEXT_OMIT);
    }

    #[test]
    fn permissions_voice_omitted() {
        let expected = Permissions::ADD_REACTIONS
            | Permissions::ATTACH_FILES
            | Permissions::EMBED_LINKS
            | Permissions::MANAGE_MESSAGES
            | Permissions::MANAGE_WEBHOOKS
            | Permissions::MENTION_EVERYONE
            | Permissions::READ_MESSAGE_HISTORY
            | Permissions::REQUEST_TO_SPEAK
            | Permissions::SEND_MESSAGES
            | Permissions::SEND_TTS_MESSAGES
            | Permissions::USE_EXTERNAL_EMOJIS
            | Permissions::USE_SLASH_COMMANDS;

        assert_eq!(expected, PERMISSIONS_VOICE_OMIT);
    }
}
