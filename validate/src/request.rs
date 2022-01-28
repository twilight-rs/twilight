//! Contains all other input validation functions.
//!
//! These functions are generally not related to a specific Discord model.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    time::{SystemTime, UNIX_EPOCH},
};
use twilight_model::datetime::Timestamp;

/// The maximum audit log reason length in UTF-16 codepoints.
pub const AUDIT_REASON_MAX: usize = 512;

/// Maximum amount of days for messages to be deleted upon ban.
pub const CREATE_GUILD_BAN_DELETE_MESSAGE_DAYS_MAX: u8 = 7;

/// Maximum amount of time a member can be timed out for.
pub const COMMUNICATION_DISABLED_MAX_DURATION: i64 = 28 * 24 * 60 * 60;

/// Maximum amount of messages to get.
pub const GET_CHANNEL_MESSAGES_LIMIT_MAX: u8 = 100;

/// Minimum amount of messages to get.
pub const GET_CHANNEL_MESSAGES_LIMIT_MIN: u8 = 1;

/// Maximum amount of guilds to get.
pub const GET_CURRENT_USER_GUILDS_LIMIT_MAX: u8 = 200;

/// Minimum amount of guilds to get.
pub const GET_CURRENT_USER_GUILDS_LIMIT_MIN: u8 = 1;

/// Maximum amount of audit log entries to list.
pub const GET_GUILD_AUDIT_LOG_LIMIT_MAX: u8 = 100;

/// Minimum amount of audit log entries to list.
pub const GET_GUILD_AUDIT_LOG_LIMIT_MIN: u8 = 1;

/// Maximum amount of guild members to list.
pub const GET_GUILD_MEMBERS_LIMIT_MAX: u16 = 1000;

/// Minimum amount of guild members to list.
pub const GET_GUILD_MEMBERS_LIMIT_MIN: u16 = 1;

/// Maximum amount of users to return when getting reactions.
pub const GET_REACTIONS_LIMIT_MIN: u64 = 1;

/// Minimum amount of users to return when getting reactions.
pub const GET_REACTIONS_LIMIT_MAX: u64 = 100;

/// Maximum length of a guild's name.
pub const GUILD_NAME_LENGTH_MAX: usize = 100;

/// Minimum length of a guild's name.
pub const GUILD_NAME_LENGTH_MIN: usize = 2;

/// Maximum amount of days to prune users from a guild.
pub const GUILD_PRUNE_DAYS_MAX: u8 = 30;

/// Minimum amount of days to prune users from a guild.
pub const GUILD_PRUNE_DAYS_MIN: u8 = 1;

/// Maximum length of an invite's age, in seconds.
pub const INVITE_AGE_MAX: u64 = 604_800;

/// Maximum uses of an invite, if not unlimited.
pub const INVITE_USES_MAX: u64 = 100;

/// Maximum length of a maximum.
pub const NICKNAME_LIMIT_MAX: usize = 32;

/// Minimum length of a nickname.
pub const NICKNAME_LIMIT_MIN: usize = 1;

/// Maximum length of a scheduled event's description.
pub const SCHEDULED_EVENT_DESCRIPTION_MAX: usize = 1000;

/// Minimum length of a scheduled event's description.
pub const SCHEDULED_EVENT_DESCRIPTION_MIN: usize = 1;

/// Maximum amount of scheduled event users to get.
pub const SCHEDULED_EVENT_GET_USERS_MAX: u64 = 100;

/// Minimum amount of scheduled event users to get.
pub const SCHEDULED_EVENT_GET_USERS_MIN: u64 = 1;

/// Maximum length of a scheduled event's name.
pub const SCHEDULED_EVENT_NAME_MAX: usize = 100;

/// Minimum length of a scheduled event's name.
pub const SCHEDULED_EVENT_NAME_MIN: usize = 1;

/// Maximum amount of guild members to search for.
pub const SEARCH_GUILD_MEMBERS_LIMIT_MAX: u16 = 1000;

/// Minimum amount of guild members to search for.
pub const SEARCH_GUILD_MEMBERS_LIMIT_MIN: u16 = 1;

/// Maximum stage instance topic length.
pub const STAGE_TOPIC_LENGTH_MAX: usize = 120;

/// Minimum stage instance topic length.
pub const STAGE_TOPIC_LENGTH_MIN: usize = 1;

/// Maximum length of a guild template description.
pub const TEMPLATE_DESCRIPTION_LENGTH_MAX: usize = 120;

/// Maximum length of a guild template name.
pub const TEMPLATE_NAME_LENGTH_MAX: usize = 100;

/// Minimum length of a guild template name.
pub const TEMPLATE_NAME_LENGTH_MIN: usize = 1;

/// Maximum length of a username.
pub const USERNAME_LIMIT_MAX: usize = 32;

/// Minimum length of a username.
pub const USERNAME_LIMIT_MIN: usize = 2;

/// A field is not valid.
#[derive(Debug)]
pub struct ValidationError {
    /// Type of error that occurred.
    kind: ValidationErrorType,
}

impl ValidationError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ValidationErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (ValidationErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for ValidationError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ValidationErrorType::AuditReason { len } => {
                f.write_str("provided audit reason length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUDIT_REASON_MAX, f)
            }
            ValidationErrorType::CreateGuildBanDeleteMessageDays {
                days: delete_message_days,
            } => {
                f.write_str("provided create guild ban delete_message_days is ")?;
                Display::fmt(delete_message_days, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&CREATE_GUILD_BAN_DELETE_MESSAGE_DAYS_MAX, f)
            }
            ValidationErrorType::CommunicationDisabledUntil { .. } => {
                f.write_str("provided timestamp is too far in the future")
            }
            ValidationErrorType::GetChannelMessages { limit } => {
                f.write_str("provided get guild members limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&GET_CHANNEL_MESSAGES_LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&GET_CHANNEL_MESSAGES_LIMIT_MAX, f)
            }
            ValidationErrorType::GetCurrentUserGuilds { limit } => {
                f.write_str("provided get current user guilds limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&GET_CURRENT_USER_GUILDS_LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&GET_CURRENT_USER_GUILDS_LIMIT_MAX, f)
            }
            ValidationErrorType::GetGuildAuditLog { limit } => {
                f.write_str("provided get guild audit log limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&GET_GUILD_MEMBERS_LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&GET_GUILD_MEMBERS_LIMIT_MAX, f)
            }
            ValidationErrorType::GetGuildMembers { limit } => {
                f.write_str("provided get guild members limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&GET_GUILD_MEMBERS_LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&GET_GUILD_MEMBERS_LIMIT_MAX, f)
            }
            ValidationErrorType::GetReactions { limit } => {
                f.write_str("provided get reactions limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&GET_REACTIONS_LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&GET_REACTIONS_LIMIT_MAX, f)
            }
            ValidationErrorType::GuildName { len } => {
                f.write_str("provided guild name length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&GUILD_NAME_LENGTH_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&GUILD_NAME_LENGTH_MAX, f)
            }
            ValidationErrorType::GuildPruneDays { days } => {
                f.write_str("provided prune days is ")?;
                Display::fmt(days, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&GUILD_PRUNE_DAYS_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&GUILD_PRUNE_DAYS_MAX, f)
            }
            ValidationErrorType::InviteMaxAge { max_age } => {
                f.write_str("provided invite max_age is ")?;
                Display::fmt(max_age, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&INVITE_AGE_MAX, f)
            }
            ValidationErrorType::InviteMaxUses { max_uses } => {
                f.write_str("provided invite max_uses is ")?;
                Display::fmt(max_uses, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&INVITE_USES_MAX, f)
            }
            ValidationErrorType::Nickname { len } => {
                f.write_str("provided nickname length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&NICKNAME_LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&NICKNAME_LIMIT_MAX, f)
            }
            ValidationErrorType::ScheduledEventDescription { len } => {
                f.write_str("provided scheduled event description is length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&SCHEDULED_EVENT_DESCRIPTION_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&SCHEDULED_EVENT_DESCRIPTION_MAX, f)
            }
            ValidationErrorType::ScheduledEventGetUsers { limit } => {
                f.write_str("provided scheduled event get users limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&SCHEDULED_EVENT_GET_USERS_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&SCHEDULED_EVENT_GET_USERS_MAX, f)
            }
            ValidationErrorType::ScheduledEventName { len } => {
                f.write_str("provided scheduled event name is length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&SCHEDULED_EVENT_NAME_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&SCHEDULED_EVENT_NAME_MAX, f)
            }
            ValidationErrorType::SearchGuildMembers { limit } => {
                f.write_str("provided search guild members limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&SEARCH_GUILD_MEMBERS_LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&SEARCH_GUILD_MEMBERS_LIMIT_MAX, f)
            }
            ValidationErrorType::StageTopic { len } => {
                f.write_str("provided stage instance topic length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&STAGE_TOPIC_LENGTH_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&STAGE_TOPIC_LENGTH_MAX, f)
            }
            ValidationErrorType::TemplateDescription { len } => {
                f.write_str("provided guild template description topic length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&TEMPLATE_DESCRIPTION_LENGTH_MAX, f)
            }
            ValidationErrorType::TemplateName { len } => {
                f.write_str("provided guild template name length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&TEMPLATE_NAME_LENGTH_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&TEMPLATE_NAME_LENGTH_MAX, f)
            }
            ValidationErrorType::Username { len } => {
                f.write_str("provided username length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&USERNAME_LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&USERNAME_LIMIT_MAX, f)
            }
        }
    }
}

impl Error for ValidationError {}

/// Type of [`ValidationError`] that occurred.
#[derive(Debug)]
pub enum ValidationErrorType {
    /// Provided audit reason was too large.
    AuditReason {
        /// Invalid length.
        len: usize,
    },
    /// Provided create guild ban delete message days was invalid.
    CreateGuildBanDeleteMessageDays {
        /// Invalid days.
        days: u8,
    },
    /// Provided timestamp is too far in the future.
    CommunicationDisabledUntil {
        /// Invalid timestamp.
        timestamp: Timestamp,
    },
    /// Provided get channel messages limit was invalid.
    GetChannelMessages {
        /// Invalid limit.
        limit: u8,
    },
    /// Provided get current user guilds limit was invalid.
    GetCurrentUserGuilds {
        /// Invalid limit.
        limit: u8,
    },
    /// Provided get guild audit log limit was invalid.
    GetGuildAuditLog {
        /// Invalid limit.
        limit: u8,
    },
    /// Provided get guild members limit was invalid.
    GetGuildMembers {
        /// Invalid limit.
        limit: u16,
    },
    /// Provided get reactions limit was invalid.
    GetReactions {
        /// Invalid limit.
        limit: u64,
    },
    /// Provided guild name was invalid.
    GuildName {
        /// Invalid length.
        len: usize,
    },
    /// Provided guild prune days was invalid.
    GuildPruneDays {
        /// Invalid days.
        days: u8,
    },
    /// Provided invite max age was invalid.
    InviteMaxAge {
        /// Invalid age.
        max_age: u64,
    },
    /// Provided invite max uses was invalid.
    InviteMaxUses {
        /// Invalid age.
        max_uses: u64,
    },
    /// Provided nickname length was invalid.
    Nickname {
        /// Invalid length.
        len: usize,
    },
    /// Scheduled event description is invalid.
    ScheduledEventDescription {
        /// Invalid length.
        len: usize,
    },
    /// Scheduled event get users limit is invalid.
    ScheduledEventGetUsers {
        /// Invalid limit.
        limit: u64,
    },
    /// Scheduled event name is invalid.
    ScheduledEventName {
        /// Invalid length.
        len: usize,
    },
    /// Provided search guild members limit was invalid.
    SearchGuildMembers {
        /// Invalid limit.
        limit: u16,
    },
    /// Provided stage instance topic was invalid.
    StageTopic {
        /// Invalid length.
        len: usize,
    },
    /// Provided guild template description was invalid.
    TemplateDescription {
        /// Invalid length.
        len: usize,
    },
    /// Provided guild template name was invalid.
    TemplateName {
        /// Invalid length.
        len: usize,
    },
    /// Provided username length was invalid.
    Username {
        /// Invalid length.
        len: usize,
    },
}

/// Ensure that an audit reason is correct.
///
/// The length must be at most [`AUDIT_REASON_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AuditReason`] if the length is invalid.
///
/// [`AuditReason`]: ValidationErrorType::AuditReason
/// [this documentation entry]: https://discord.com/developers/docs/resources/audit-log#audit-log-entry-object-audit-log-entry-structure
pub fn audit_reason(audit_reason: impl AsRef<str>) -> Result<(), ValidationError> {
    let len = audit_reason.as_ref().chars().count();

    if len <= AUDIT_REASON_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AuditReason { len },
        })
    }
}

/// Ensure that the delete message days amount for the Create Guild Ban request
/// is correct.
///
/// The days must be at most [`CREATE_GUILD_BAN_DELETE_MESSAGE_DAYS_MAX`]. This
/// is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`CreateGuildBanDeleteMessageDays`] if the days is
/// invalid.
///
/// [`CreateGuildBanDeleteMessageDays`]: ValidationErrorType::CreateGuildBanDeleteMessageDays
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild#create-guild-ban
pub const fn create_guild_ban_delete_message_days(days: u8) -> Result<(), ValidationError> {
    if days <= CREATE_GUILD_BAN_DELETE_MESSAGE_DAYS_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::CreateGuildBanDeleteMessageDays { days },
        })
    }
}

/// Validate that a timeout time is not too far in the future.
///
/// The time must not be farther than 28 days in the future.
///
/// # Errors
///
///
#[allow(clippy::cast_possible_wrap)] // casting of unix timestamp should never wrap
pub fn communication_disabled_until(timestamp: Timestamp) -> Result<(), ValidationError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| ValidationError {
            kind: ValidationErrorType::CommunicationDisabledUntil { timestamp },
        })?;

    let end = timestamp.as_secs();

    if end - now.as_secs() as i64 <= COMMUNICATION_DISABLED_MAX_DURATION {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::CommunicationDisabledUntil { timestamp },
        })
    }
}

/// Ensure that the limit for the Get Channel Messages request is correct.
///
/// The limit must be at least [`GET_CHANNEL_MESSAGES_LIMIT_MIN`] and at most
/// [`GET_CHANNEL_MESSAGES_LIMIT_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`GetChannelMessages`] if the limit is invalid.
///
/// [`GetChannelMessages`]: ValidationErrorType::GetChannelMessages
/// [this documentation entry]: https://discord.com/developers/docs/resources/channel#get-channel-messages
pub const fn get_channel_messages_limit(limit: u8) -> Result<(), ValidationError> {
    if limit >= GET_CHANNEL_MESSAGES_LIMIT_MIN && limit <= GET_CHANNEL_MESSAGES_LIMIT_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GetChannelMessages { limit },
        })
    }
}

/// Ensure that the limit for the Get Current User Guilds request is correct.
///
/// The limit must be at least [`GET_CURRENT_USER_GUILDS_LIMIT_MIN`] and at most
/// [`GET_CURRENT_USER_GUILDS_LIMIT_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`GetCurrentUserGuilds`] if the limit is invalid.
///
/// [`GetCurrentUserGuilds`]: ValidationErrorType::GetCurrentUserGuilds
/// [this documentation entry]: https://discord.com/developers/docs/resources/user#get-current-user-guilds
pub const fn get_current_user_guilds_limit(limit: u8) -> Result<(), ValidationError> {
    if limit >= GET_CURRENT_USER_GUILDS_LIMIT_MIN && limit <= GET_CURRENT_USER_GUILDS_LIMIT_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GetCurrentUserGuilds { limit },
        })
    }
}

/// Ensure that the limit for the Get Guild Audit Log endpoint is correct.
///
/// The limit must be at least [`GET_GUILD_AUDIT_LOG_LIMIT_MIN`] and at most
/// [`GET_GUILD_AUDIT_LOG_LIMIT_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`GetGuildAuditLog`] if the limit is invalid.
///
/// [`GetGuildAuditLog`]: ValidationErrorType::GetGuildAuditLog
/// [this documentation entry]: https://discord.com/developers/docs/resources/audit-log#get-guild-audit-log
pub const fn get_guild_audit_log_limit(limit: u8) -> Result<(), ValidationError> {
    if limit >= GET_GUILD_AUDIT_LOG_LIMIT_MIN && limit <= GET_GUILD_AUDIT_LOG_LIMIT_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GetGuildAuditLog { limit },
        })
    }
}

/// Ensure that the limit for the Get Guild Members endpoint is correct.
///
/// The limit must be at least [`GET_GUILD_MEMBERS_LIMIT_MIN`] and at most
/// [`GET_GUILD_MEMBERS_LIMIT_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`GetGuildMembers`] if the limit is invalid.
///
/// [`GetGuildMembers`]: ValidationErrorType::GetGuildMembers
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild#list-guild-members
pub const fn get_guild_members_limit(limit: u16) -> Result<(), ValidationError> {
    if limit >= GET_GUILD_MEMBERS_LIMIT_MIN && limit <= GET_GUILD_MEMBERS_LIMIT_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GetGuildMembers { limit },
        })
    }
}

/// Ensure that the limit for the Get Reactions endpoint is correct.
///
/// The limit must be at least [`GET_REACTIONS_LIMIT_MIN`] and at most
/// [`GET_REACTIONS_LIMIT_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`GetReactions`] if the limit is invalid.
///
/// [`GetReactions`]: ValidationErrorType::GetReactions
/// [this documentation entry]: https://discord.com/developers/docs/resources/channel#get-reactions
pub const fn get_reactions_limit(limit: u64) -> Result<(), ValidationError> {
    if limit >= GET_REACTIONS_LIMIT_MIN && limit <= GET_REACTIONS_LIMIT_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GetReactions { limit },
        })
    }
}

/// Ensure that a guild name's length is correct.
///
/// The length must be at least [`GUILD_NAME_LENGTH_MIN`] and at most
/// [`GUILD_NAME_LENGTH_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`GuildName`] if the length is invalid.
///
/// [`GuildName`]: ValidationErrorType::GuildName
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild#guild-object
pub fn guild_name(name: impl AsRef<str>) -> Result<(), ValidationError> {
    let len = name.as_ref().chars().count();

    if (GUILD_NAME_LENGTH_MIN..=GUILD_NAME_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GuildName { len },
        })
    }
}

/// Ensure that the days to prune members from a guild is correct.
///
/// The days must be at least [`GUILD_PRUNE_DAYS_MIN`] and at most
/// [`GUILD_PRUNE_DAYS_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`GuildPruneDays`] if the days is invalid.
///
/// [`GuildPruneDays`]: ValidationErrorType::GuildPruneDays
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild#get-guild-prune-count
pub const fn guild_prune_days(days: u8) -> Result<(), ValidationError> {
    if days >= GUILD_PRUNE_DAYS_MIN && days <= GUILD_PRUNE_DAYS_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GuildPruneDays { days },
        })
    }
}

/// Ensure that the invite max age is correct.
///
/// The age must be at most [`INVITE_AGE_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`InviteMaxAge`] if the age is invalid.
///
/// [`InviteMaxAge`]: ValidationErrorType::InviteMaxAge
/// [this documentation entry]: https://discord.com/developers/docs/resources/channel#create-channel-invite
pub const fn invite_max_age(max_age: u64) -> Result<(), ValidationError> {
    if max_age <= INVITE_AGE_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::InviteMaxAge { max_age },
        })
    }
}

/// Ensure that the invite max uses is correct.
///
/// The age must be at most [`INVITE_USES_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`InviteMaxUses`] if the uses is invalid.
///
/// [`InviteMaxUses`]: ValidationErrorType::InviteMaxUses
/// [this documentation entry]: https://discord.com/developers/docs/resources/channel#create-channel-invite
pub const fn invite_max_uses(max_uses: u64) -> Result<(), ValidationError> {
    if max_uses <= INVITE_USES_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::InviteMaxUses { max_uses },
        })
    }
}

/// Ensure that the nickname length is correct.
///
/// The length must be at least [`NICKNAME_LIMIT_MIN`] and at most
/// [`NICKNAME_LIMIT_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`Nickname`] if the length is invalid.
///
/// [`Nickname`]: ValidationErrorType::Nickname
/// [this documentation entry]: https://discord.com/developers/docs/resources/user#usernames-and-nicknames
pub fn nickname(nickname: impl AsRef<str>) -> Result<(), ValidationError> {
    let len = nickname.as_ref().chars().count();

    if (NICKNAME_LIMIT_MIN..=NICKNAME_LIMIT_MAX).contains(&len) {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::Nickname { len },
        })
    }
}

/// Ensure that a scheduled event's description is correct.
///
/// The length must be at least [`SCHEDULED_EVENT_DESCRIPTION_MIN`] and at most
/// [`SCHEDULED_EVENT_DESCRIPTION_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`ScheduledEventDescription`] if the length is
/// invalid.
///
/// [`ScheduledEventDescription`]: ValidationErrorType::ScheduledEventDescription
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-structure
pub fn scheduled_event_description(description: impl AsRef<str>) -> Result<(), ValidationError> {
    let len = description.as_ref().chars().count();

    if (SCHEDULED_EVENT_DESCRIPTION_MIN..=SCHEDULED_EVENT_DESCRIPTION_MAX).contains(&len) {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::ScheduledEventDescription { len },
        })
    }
}

/// Ensure that a scheduled event's get users limit amount is correct.
///
/// The length must be at least [`SCHEDULED_EVENT_GET_USERS_MIN`] and at most
/// [`SCHEDULED_EVENT_GET_USERS_MAX`]. This is based on [this documentation
/// entry].
///
/// # Errors
///
/// Returns an error of type [`ScheduledEventGetUsers`] if the limit is invalid.
///
/// [`ScheduledEventGetUsers`]: ValidationErrorType::ScheduledEventGetUsers
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild-scheduled-event#get-guild-scheduled-event-users-query-string-params
pub const fn scheduled_event_get_users(limit: u64) -> Result<(), ValidationError> {
    if limit <= SCHEDULED_EVENT_GET_USERS_MIN && limit >= SCHEDULED_EVENT_GET_USERS_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::ScheduledEventGetUsers { limit },
        })
    }
}

/// Ensure that a scheduled event's name is correct.
///
/// The length must be at least [`SCHEDULED_EVENT_NAME_MIN`] and at most
/// [`SCHEDULED_EVENT_NAME_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`ScheduledEventName`] if the length is invalid.
///
/// [`ScheduledEventName`]: ValidationErrorType::ScheduledEventName
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild-scheduled-event#guild-scheduled-event-object-guild-scheduled-event-structure
pub fn scheduled_event_name(name: impl AsRef<str>) -> Result<(), ValidationError> {
    let len = name.as_ref().chars().count();

    if (SCHEDULED_EVENT_NAME_MIN..=SCHEDULED_EVENT_NAME_MAX).contains(&len) {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::ScheduledEventName { len },
        })
    }
}

/// Ensure that the limit for the Search Guild Members endpoint is correct.
///
/// The limit must be at least [`SEARCH_GUILD_MEMBERS_LIMIT_MIN`] and at most
/// [`SEARCH_GUILD_MEMBERS_LIMIT_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`SearchGuildMembers`] if the limit is invalid.
///
/// [`SearchGuildMembers`]: ValidationErrorType::SearchGuildMembers
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild#search-guild-members
pub const fn search_guild_members_limit(limit: u16) -> Result<(), ValidationError> {
    if limit >= SEARCH_GUILD_MEMBERS_LIMIT_MIN && limit <= SEARCH_GUILD_MEMBERS_LIMIT_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::SearchGuildMembers { limit },
        })
    }
}

/// Ensure that the stage instance's topic length is correct.
///
/// The length must be at least [`STAGE_TOPIC_LENGTH_MIN`] and at most
/// [`STAGE_TOPIC_LENGTH_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`StageTopic`] if the length is invalid.
///
/// [`StageTopic`]: ValidationErrorType::StageTopic
/// [this documentation entry]: https://discord.com/developers/docs/resources/stage-instance#stage-instance-object
pub fn stage_topic(topic: impl AsRef<str>) -> Result<(), ValidationError> {
    let len = topic.as_ref().chars().count();

    if (STAGE_TOPIC_LENGTH_MIN..=STAGE_TOPIC_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::StageTopic { len },
        })
    }
}

/// Ensure that a guild template's description length is correct.
///
/// The length must be at most [`TEMPLATE_DESCRIPTION_LENGTH_MAX`]. This is
/// based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`TemplateDescription`] if the length is invalid.
///
/// [`TemplateDescription`]: ValidationErrorType::TemplateDescription
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild-template#guild-template-object-guild-template-structure
pub fn template_description(description: impl AsRef<str>) -> Result<(), ValidationError> {
    let len = description.as_ref().chars().count();

    if len <= TEMPLATE_DESCRIPTION_LENGTH_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::TemplateDescription { len },
        })
    }
}

/// Ensure that a guild template's name length is correct.
///
/// The length must be at least [`TEMPLATE_NAME_LENGTH_MIN`] and at most
/// [`TEMPLATE_NAME_LENGTH_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`TemplateName`] if the length is invalid.
///
/// [`TemplateName`]: ValidationErrorType::TemplateName
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild-template#guild-template-object-guild-template-structure
pub fn template_name(name: impl AsRef<str>) -> Result<(), ValidationError> {
    let len = name.as_ref().chars().count();

    if (TEMPLATE_NAME_LENGTH_MIN..=TEMPLATE_NAME_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::TemplateName { len },
        })
    }
}

/// Ensure that the username length is correct.
///
/// The length must be at least [`USERNAME_LIMIT_MIN`] and at most
/// [`USERNAME_LIMIT_MAX`]. This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`Username`] if the length is invalid.
///
/// [`Username`]: ValidationErrorType::Username
/// [this documentation entry]: https://discord.com/developers/docs/resources/user#usernames-and-nicknames
pub fn username(value: impl AsRef<str>) -> Result<(), ValidationError> {
    let len = value.as_ref().chars().count();

    if (USERNAME_LIMIT_MIN..=USERNAME_LIMIT_MAX).contains(&len) {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::Username { len },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_reason() {
        assert!(audit_reason("").is_ok());
        assert!(audit_reason("a").is_ok());
        assert!(audit_reason("a".repeat(500)).is_ok());
        assert!(audit_reason("a".repeat(512)).is_ok());

        assert!(audit_reason("a".repeat(513)).is_err());
    }

    #[test]
    fn test_create_guild_ban_delete_message_days() {
        assert!(create_guild_ban_delete_message_days(0).is_ok());
        assert!(create_guild_ban_delete_message_days(1).is_ok());
        assert!(create_guild_ban_delete_message_days(7).is_ok());

        assert!(create_guild_ban_delete_message_days(8).is_err());
    }

    #[test]
    fn test_communication_disabled_until() {
        #[allow(clippy::cast_possible_wrap)]
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let ok_timestamp =
            Timestamp::from_secs(now + COMMUNICATION_DISABLED_MAX_DURATION - 1000).unwrap();
        assert!(communication_disabled_until(ok_timestamp).is_ok());

        let err_timestamp =
            Timestamp::from_secs(now + COMMUNICATION_DISABLED_MAX_DURATION + 1000).unwrap();
        assert!(communication_disabled_until(err_timestamp).is_err());
    }

    #[test]
    fn test_get_channel_messages_limit() {
        assert!(get_channel_messages_limit(1).is_ok());
        assert!(get_channel_messages_limit(100).is_ok());

        assert!(get_channel_messages_limit(0).is_err());
        assert!(get_channel_messages_limit(101).is_err());
    }

    #[test]
    fn test_get_current_user_guilds_limit() {
        assert!(get_current_user_guilds_limit(1).is_ok());
        assert!(get_current_user_guilds_limit(200).is_ok());

        assert!(get_current_user_guilds_limit(0).is_err());
        assert!(get_current_user_guilds_limit(201).is_err());
    }

    #[test]
    fn test_get_audit_log_limit() {
        assert!(get_guild_audit_log_limit(1).is_ok());
        assert!(get_guild_audit_log_limit(100).is_ok());

        assert!(get_guild_audit_log_limit(0).is_err());
        assert!(get_guild_audit_log_limit(101).is_err());
    }

    #[test]
    fn test_get_guild_members_limit() {
        assert!(get_guild_members_limit(1).is_ok());
        assert!(get_guild_members_limit(1000).is_ok());

        assert!(get_guild_members_limit(0).is_err());
        assert!(get_guild_members_limit(1001).is_err());
    }

    #[test]
    fn test_get_reactions_limit() {
        assert!(get_reactions_limit(1).is_ok());
        assert!(get_reactions_limit(100).is_ok());

        assert!(get_reactions_limit(0).is_err());
        assert!(get_reactions_limit(101).is_err());
    }

    #[test]
    fn test_guild_name() {
        assert!(guild_name("aa").is_ok());
        assert!(guild_name("a".repeat(100)).is_ok());

        assert!(guild_name("").is_err());
        assert!(guild_name("a").is_err());
        assert!(guild_name("a".repeat(101)).is_err());
    }

    #[test]
    fn test_guild_prune_days() {
        assert!(guild_prune_days(1).is_ok());
        assert!(guild_prune_days(30).is_ok());

        assert!(guild_prune_days(0).is_err());
        assert!(guild_prune_days(31).is_err());
        assert!(guild_prune_days(100).is_err());
    }

    #[test]
    fn test_invite_max_age() {
        assert!(invite_max_age(0).is_ok());
        assert!(invite_max_age(86_400).is_ok());
        assert!(invite_max_age(604_800).is_ok());

        assert!(invite_max_age(604_801).is_err());
    }

    #[test]
    fn test_invite_max_uses() {
        assert!(invite_max_uses(0).is_ok());
        assert!(invite_max_uses(100).is_ok());

        assert!(invite_max_uses(101).is_err());
    }

    #[test]
    fn test_nickname() {
        assert!(nickname("a").is_ok());
        assert!(nickname("a".repeat(32)).is_ok());

        assert!(nickname("").is_err());
        assert!(nickname("a".repeat(33)).is_err());
    }

    #[test]
    fn test_scheduled_event_description() {
        assert!(scheduled_event_description("a").is_ok());
        assert!(scheduled_event_description("a".repeat(1000)).is_ok());

        assert!(scheduled_event_description("").is_err());
        assert!(scheduled_event_description("a".repeat(1001)).is_err());
    }

    #[test]
    fn test_scheduled_event_name() {
        assert!(scheduled_event_name("a").is_ok());
        assert!(scheduled_event_name("a".repeat(100)).is_ok());

        assert!(scheduled_event_name("").is_err());
        assert!(scheduled_event_name("a".repeat(101)).is_err());
    }

    #[test]
    fn test_search_guild_members_limit() {
        assert!(search_guild_members_limit(1).is_ok());
        assert!(search_guild_members_limit(1000).is_ok());

        assert!(search_guild_members_limit(0).is_err());
        assert!(search_guild_members_limit(1001).is_err());
    }

    #[test]
    fn test_stage_topic() {
        assert!(stage_topic("a").is_ok());
        assert!(stage_topic("a".repeat(120)).is_ok());

        assert!(stage_topic("").is_err());
        assert!(stage_topic("a".repeat(121)).is_err());
    }

    #[test]
    fn test_template_description() {
        assert!(template_description("").is_ok());
        assert!(template_description("a").is_ok());
        assert!(template_description("a".repeat(120)).is_ok());

        assert!(template_description("a".repeat(121)).is_err());
    }

    #[test]
    fn test_template_name() {
        assert!(template_name("a").is_ok());
        assert!(template_name("a".repeat(100)).is_ok());

        assert!(template_name("").is_err());
        assert!(template_name("a".repeat(101)).is_err());
    }

    #[test]
    fn test_username() {
        assert!(username("aa").is_ok());
        assert!(username("a".repeat(32)).is_ok());

        assert!(username("a").is_err());
        assert!(username("a".repeat(33)).is_err());
    }
}
