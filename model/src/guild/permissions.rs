use bitflags::bitflags;

bitflags! {
    #[cfg_attr(feature = "serde-support", derive(serde::Deserialize, serde::Serialize))]
    pub struct Permissions: u64 {
        const CREATE_INVITE = 0x0000_0001;
        const KICK_MEMBERS = 0x0000_0002;
        const BAN_MEMBERS = 0x0000_0004;
        const ADMINISTRATOR = 0x0000_0008;
        const MANAGE_CHANNELS = 0x0000_0010;
        const MANAGE_GUILD = 0x0000_0020;
        const ADD_REACTIONS = 0x0000_0040;
        const VIEW_AUDIT_LOG = 0x0000_0080;
        const PRIORITY_SPEAKER = 0x0000_0100;
        const STREAM = 0x0000_0200;
        const VIEW_CHANNEL = 0x0000_0400;
        const SEND_MESSAGES = 0x0000_0800;
        const SEND_TTS_MESSAGES = 0x0000_1000;
        const MANAGE_MESSAGES = 0x0000_2000;
        const EMBED_LINKS = 0x0000_4000;
        const ATTACH_FILES = 0x0000_8000;
        const READ_MESSAGE_HISTORY = 0x0001_0000;
        const MENTION_EVERYONE = 0x0002_0000;
        const USE_EXTERNAL_EMOJIS = 0x0004_0000;
        const CONNECT = 0x0010_0000;
        const SPEAK = 0x0020_0000;
        const MUTE_MEMBERS = 0x0040_0000;
        const DEAFEN_MEMBERS = 0x0080_0000;
        const MOVE_MEMBERS = 0x0100_0000;
        const USE_VAD = 0x0200_0000;
        const CHANGE_NICKNAME = 0x0400_0000;
        const MANAGE_NICKNAMES = 0x0800_0000;
        const MANAGE_ROLES = 0x1000_0000;
        const MANAGE_WEBHOOKS = 0x2000_0000;
        const MANAGE_EMOJIS = 0x4000_0000;
    }
}
