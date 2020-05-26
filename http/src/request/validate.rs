/// Contains all of the input validation functions for requests.
///
/// This is in a centralised place so that the validation parameters can be kept
/// up-to-date more easily and because some of the checks are re-used across
/// different modules.

pub fn ban_delete_message_days(value: u64) -> bool {
    // <https://discordapp.com/developers/docs/resources/guild#create-guild-ban-query-string-params>
    value <= 7
}

pub fn channel_name(value: impl AsRef<str>) -> bool {
    _channel_name(value.as_ref())
}

fn _channel_name(value: &str) -> bool {
    let len = value.chars().count();

    // <https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure>
    len >= 2 && len <= 100
}

pub fn content_limit(value: impl AsRef<str>) -> bool {
    _content_limit(value.as_ref())
}

fn _content_limit(value: &str) -> bool {
    // <https://discordapp.com/developers/docs/resources/channel#create-message-params>
    value.chars().count() <= 2000
}

pub fn get_audit_log_limit(value: u64) -> bool {
    // <https://discordapp.com/developers/docs/resources/audit-log#get-guild-audit-log-query-string-parameters>
    value > 0 && value <= 100
}

pub fn get_channel_messages_limit(value: u64) -> bool {
    // <https://discordapp.com/developers/docs/resources/channel#get-channel-messages-query-string-params>
    value > 0 && value <= 100
}

pub fn get_current_user_guilds_limit(value: u64) -> bool {
    // <https://discordapp.com/developers/docs/resources/user#get-current-user-guilds-query-string-params>
    value > 0 && value <= 100
}

pub fn get_guild_members_limit(value: u64) -> bool {
    // <https://discordapp.com/developers/docs/resources/guild#list-guild-members-query-string-params>
    value > 0 && value <= 1000
}

pub fn get_reactions_limit(value: u64) -> bool {
    // <https://discordapp.com/developers/docs/resources/channel#get-reactions-query-string-params>
    value > 0 && value <= 100
}

pub fn guild_name(value: impl AsRef<str>) -> bool {
    _guild_name(value.as_ref())
}

fn _guild_name(value: &str) -> bool {
    let len = value.chars().count();

    // <https://discordapp.com/developers/docs/resources/guild#guild-object-guild-structure>
    len >= 2 && len <= 100
}

pub fn guild_prune_days(value: u64) -> bool {
    // <https://discordapp.com/developers/docs/resources/guild#get-guild-prune-count-query-string-params>
    value > 0
}

pub fn nickname(value: impl AsRef<str>) -> bool {
    _nickname(value.as_ref())
}

fn _nickname(value: &str) -> bool {
    let len = value.chars().count();

    // <https://discordapp.com/developers/docs/resources/user#usernames-and-nicknames>
    len > 0 && len <= 32
}

pub fn username(value: impl AsRef<str>) -> bool {
    // <https://discordapp.com/developers/docs/resources/user#usernames-and-nicknames>
    _username(value.as_ref())
}

fn _username(value: &str) -> bool {
    let len = value.chars().count();

    len >= 2 && len <= 32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ban_delete_message_days() {
        assert!(ban_delete_message_days(0));
        assert!(ban_delete_message_days(1));
        assert!(ban_delete_message_days(7));

        assert!(!ban_delete_message_days(8));
    }

    #[test]
    fn test_channel_name() {
        assert!(channel_name("aa"));
        assert!(channel_name("a".repeat(100)));

        assert!(!channel_name(""));
        assert!(!channel_name("a"));
        assert!(!channel_name("a".repeat(101)));
    }

    #[test]
    fn test_content_limit() {
        assert!(content_limit(""));
        assert!(content_limit("a".repeat(2000)));

        assert!(!content_limit("a".repeat(2001)));
    }

    #[test]
    fn test_get_audit_log_limit() {
        assert!(get_audit_log_limit(1));
        assert!(get_audit_log_limit(100));

        assert!(!get_audit_log_limit(0));
        assert!(!get_audit_log_limit(101));
    }

    #[test]
    fn test_get_channels_limit() {
        assert!(get_channel_messages_limit(1));
        assert!(get_channel_messages_limit(100));

        assert!(!get_channel_messages_limit(0));
        assert!(!get_channel_messages_limit(101));
    }

    #[test]
    fn test_get_current_user_guilds_limit() {
        assert!(get_current_user_guilds_limit(1));
        assert!(get_current_user_guilds_limit(100));

        assert!(!get_current_user_guilds_limit(0));
        assert!(!get_current_user_guilds_limit(101));
    }

    #[test]
    fn test_get_guild_members_limit() {
        assert!(get_guild_members_limit(1));
        assert!(get_guild_members_limit(1000));

        assert!(!get_guild_members_limit(0));
        assert!(!get_guild_members_limit(1001));
    }

    #[test]
    fn test_get_reactions_limit() {
        assert!(get_reactions_limit(1));
        assert!(get_reactions_limit(100));

        assert!(!get_reactions_limit(0));
        assert!(!get_reactions_limit(101));
    }

    #[test]
    fn test_guild_name() {
        assert!(guild_name("aa"));
        assert!(guild_name("a".repeat(100)));

        assert!(!guild_name(""));
        assert!(!guild_name("a"));
        assert!(!guild_name("a".repeat(101)));
    }

    #[test]
    fn test_guild_prune_days() {
        assert!(!guild_prune_days(0));
        assert!(guild_prune_days(1));
        assert!(guild_prune_days(100));
    }

    #[test]
    fn test_nickname() {
        assert!(nickname("a"));
        assert!(nickname("a".repeat(32)));

        assert!(!nickname(""));
        assert!(!nickname("a".repeat(33)));
    }

    #[test]
    fn test_username() {
        assert!(username("aa"));
        assert!(username("a".repeat(32)));

        assert!(!username("a"));
        assert!(!username("a".repeat(33)));
    }
}
