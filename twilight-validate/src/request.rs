//! Contains all other input validation functions.
//!
//! These functions are generally not related to a specific Discord model.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    time::{SystemTime, UNIX_EPOCH},
};
use twilight_model::id::Id;
use twilight_model::id::marker::{ChannelMarker, RoleMarker};
use twilight_model::util::Timestamp;

/// The maximum audit log reason length in UTF-16 codepoints.
pub const AUDIT_REASON_MAX: usize = 512;

/// Maximum length of an auto moderation block action's custom message.
pub const AUTO_MODERATION_ACTION_BLOCK_CUSTOM_MESSAGE_LENGTH_MAX: usize = 150;

/// Maximum amount of mentions that triggers an auto moderation action.
pub const AUTO_MODERATION_METADATA_MENTION_TOTAL_LIMIT: u8 = 50;

/// Maximum amount of keywords allowed in the keyword filter.
pub const AUTO_MODERATION_METADATA_KEYWORD_FILTER_MAX: usize = 1000;

/// Maximum length of a keyword in the keyword filter.
pub const AUTO_MODERATION_METADATA_KEYWORD_FILTER_LENGTH_MAX: usize = 60;

/// Maximum amount of allowed keywords when using the custom keywords.
pub const AUTO_MODERATION_METADATA_KEYWORD_ALLOW_LIST_MAX: usize = 100;

/// Maximum amount of allowed keywords when using a keyword preset.
pub const AUTO_MODERATION_METADATA_PRESET_ALLOW_LIST_MAX: usize = 1000;

/// Maximum length of a keyword preset keyword.
pub const AUTO_MODERATION_METADATA_PRESET_ALLOW_LIST_LENGTH_MAX: usize = 60;

/// Maximum length of a allowed keyword.
pub const AUTO_MODERATION_METADATA_KEYWORD_ALLOW_LIST_LENGTH_MAX: usize = 60;

/// Maximum amount of regex patterns that trigger the auto moderation.
pub const AUTO_MODERATION_METADATA_REGEX_PATTERNS_MAX: usize = 10;

/// Maximum length of a regex pattern.
pub const AUTO_MODERATION_METADATA_REGEX_PATTERNS_LENGTH_MAX: usize = 260;

/// Maximum amount of seconds (`2_419_200` this is equivalent to `28` days) to disable communication for.
pub const AUTO_MODERATION_ACTION_METADATA_DURATION_SECONDS_MAX: u32 = 2_419_200;

/// Maximum amount of exempt roles for the auto moderation rule.
pub const AUTO_MODERATION_EXEMPT_ROLES_MAX: usize = 20;

/// Maximum amount of exempt channels for the auto moderation rule.
pub const AUTO_MODERATION_EXEMPT_CHANNELS_MAX: usize = 50;

/// Maximum amount of seconds (`604_800` this is equivalent to `7` days) for messages to be deleted upon ban.
pub const CREATE_GUILD_BAN_DELETE_MESSAGE_SECONDS_MAX: u32 = 604_800;

/// Maximum amount of time a member can be timed out for.
pub const COMMUNICATION_DISABLED_MAX_DURATION: i64 = 28 * 24 * 60 * 60;

/// Maximum amount of messages to get.
pub const GET_CHANNEL_MESSAGES_LIMIT_MAX: u16 = 100;

/// Minimum amount of messages to get.
pub const GET_CHANNEL_MESSAGES_LIMIT_MIN: u16 = 1;

/// Maximum amount of guilds to get.
pub const GET_CURRENT_USER_GUILDS_LIMIT_MAX: u16 = 200;

/// Minimum amount of guilds to get.
pub const GET_CURRENT_USER_GUILDS_LIMIT_MIN: u16 = 1;

/// Maximum amount of entitlements to get.
pub const GET_ENTITLEMENTS_LIMIT_MAX: u8 = 100;

/// Minimum amount of entitlements to get.
pub const GET_ENTITLEMENTS_LIMIT_MIN: u8 = 1;

/// Maximum amount of audit log entries to list.
pub const GET_GUILD_AUDIT_LOG_LIMIT_MAX: u16 = 100;

/// Minimum amount of audit log entries to list.
pub const GET_GUILD_AUDIT_LOG_LIMIT_MIN: u16 = 1;

/// Maximum amount of guild bans to list.
pub const GET_GUILD_BANS_LIMIT_MAX: u16 = 1000;

/// Maximum amount of guild members to list.
pub const GET_GUILD_MEMBERS_LIMIT_MAX: u16 = 1000;

/// Minimum amount of guild members to list.
pub const GET_GUILD_MEMBERS_LIMIT_MIN: u16 = 1;

/// Maximum amount of users to return when getting reactions.
pub const GET_REACTIONS_LIMIT_MIN: u16 = 1;

/// Minimum amount of users to return when getting reactions.
pub const GET_REACTIONS_LIMIT_MAX: u16 = 100;

/// Maximum length of a guild's name.
pub const GUILD_NAME_LENGTH_MAX: usize = 100;

/// Minimum length of a guild's name.
pub const GUILD_NAME_LENGTH_MIN: usize = 2;

/// Maximum amount of days to prune users from a guild.
pub const GUILD_PRUNE_DAYS_MAX: u16 = 30;

/// Minimum amount of days to prune users from a guild.
pub const GUILD_PRUNE_DAYS_MIN: u16 = 1;

/// Maximum length of an invite's age, in seconds.
pub const INVITE_AGE_MAX: u32 = 604_800;

/// Maximum uses of an invite, if not unlimited.
pub const INVITE_USES_MAX: u16 = 100;

/// Maximum length of a maximum.
pub const NICKNAME_LIMIT_MAX: usize = 32;

/// Minimum length of a nickname.
pub const NICKNAME_LIMIT_MIN: usize = 1;

/// Maximum length of a scheduled event's description.
pub const SCHEDULED_EVENT_DESCRIPTION_MAX: usize = 1000;

/// Minimum length of a scheduled event's description.
pub const SCHEDULED_EVENT_DESCRIPTION_MIN: usize = 1;

/// Maximum amount of scheduled event users to get.
pub const SCHEDULED_EVENT_GET_USERS_MAX: u16 = 100;

/// Minimum amount of scheduled event users to get.
pub const SCHEDULED_EVENT_GET_USERS_MIN: u16 = 1;

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

/// Maximum length of a webhook username.
pub const WEBHOOK_USERNAME_LIMIT_MAX: usize = 80;

/// Minimum length of a webhook username.
pub const WEBHOOK_USERNAME_LIMIT_MIN: usize = 2;

/// Forbidden substrings in usernames.
const USERNAME_INVALID_SUBSTRINGS: [&str; 5] = ["@", "#", ":", "```", "discord"];

/// Forbidden usernames.
const USERNAME_INVALID_STRINGS: [&str; 2] = ["everyone", "here"];

/// Forbidden webhook usernames.
const WEBHOOK_INVALID_STRINGS: [&str; 1] = ["clyde"];

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
            ValidationErrorType::AutoModerationBlockActionCustomMessageLimit { len } => {
                f.write_str("provided auto moderation block action custom message length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_ACTION_BLOCK_CUSTOM_MESSAGE_LENGTH_MAX, f)
            }
            ValidationErrorType::AutoModerationMetadataMentionTotalLimit { limit } => {
                f.write_str("provided auto moderation metadata mention_total_limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_METADATA_MENTION_TOTAL_LIMIT, f)
            }
            ValidationErrorType::AutoModerationMetadataAllowList { len } => {
                f.write_str("provided auto moderation metadata allow_list length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_METADATA_KEYWORD_ALLOW_LIST_MAX, f)
            }
            ValidationErrorType::AutoModerationMetadataAllowListItem { len, substring } => {
                f.write_str("provided auto moderation metadata allow_list item ")?;
                Display::fmt(substring, f)?;
                f.write_str(" length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_METADATA_KEYWORD_ALLOW_LIST_LENGTH_MAX, f)
            }
            ValidationErrorType::AutoModerationMetadataKeywordFilter { len } => {
                f.write_str("provided auto moderation metadata keyword_filter length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_METADATA_KEYWORD_FILTER_MAX, f)
            }
            ValidationErrorType::AutoModerationMetadataKeywordFilterItem { len, substring } => {
                f.write_str("provided auto moderation metadata keyword_filter item ")?;
                Display::fmt(substring, f)?;
                f.write_str(" length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_METADATA_KEYWORD_FILTER_LENGTH_MAX, f)
            }
            ValidationErrorType::AutoModerationMetadataPresetAllowList { len } => {
                f.write_str("provided auto moderation metadata allow_list length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_METADATA_PRESET_ALLOW_LIST_MAX, f)
            }
            ValidationErrorType::AutoModerationMetadataPresetAllowListItem { len, substring } => {
                f.write_str("provided auto moderation metadata allow_list item ")?;
                Display::fmt(substring, f)?;
                f.write_str(" length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_METADATA_PRESET_ALLOW_LIST_LENGTH_MAX, f)
            }
            ValidationErrorType::AutoModerationActionMetadataDurationSeconds { seconds } => {
                f.write_str("provided auto moderation action timeout duration is ")?;
                Display::fmt(seconds, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_ACTION_METADATA_DURATION_SECONDS_MAX, f)
            }
            ValidationErrorType::AutoModerationMetadataRegexPatterns { len } => {
                f.write_str("provided auto moderation metadata regex_patterns length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_METADATA_REGEX_PATTERNS_MAX, f)
            }
            ValidationErrorType::AutoModerationMetadataRegexPatternsItem { len, substring } => {
                f.write_str("provided auto moderation metadata regex_patterns item ")?;
                Display::fmt(substring, f)?;
                f.write_str(" length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_METADATA_REGEX_PATTERNS_LENGTH_MAX, f)
            }
            ValidationErrorType::AutoModerationExemptRoles { len } => {
                f.write_str("provided auto moderation exempt_roles length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_EXEMPT_ROLES_MAX, f)
            }
            ValidationErrorType::AutoModerationExemptChannels { len } => {
                f.write_str("provided auto moderation exempt_channels length is ")?;
                Display::fmt(len, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&AUTO_MODERATION_EXEMPT_CHANNELS_MAX, f)
            }
            ValidationErrorType::CreateGuildBanDeleteMessageSeconds {
                seconds: delete_message_seconds,
            } => {
                f.write_str("provided create guild ban delete_message_seconds is ")?;
                Display::fmt(delete_message_seconds, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&CREATE_GUILD_BAN_DELETE_MESSAGE_SECONDS_MAX, f)
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
            ValidationErrorType::GetEntitlements { limit } => {
                f.write_str("provided get entitlements limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&GET_ENTITLEMENTS_LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&GET_ENTITLEMENTS_LIMIT_MAX, f)
            }
            ValidationErrorType::GetGuildAuditLog { limit } => {
                f.write_str("provided get guild audit log limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at least ")?;
                Display::fmt(&GET_GUILD_MEMBERS_LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&GET_GUILD_MEMBERS_LIMIT_MAX, f)
            }
            ValidationErrorType::GetGuildBans { limit } => {
                f.write_str("provided get guild bans limit is ")?;
                Display::fmt(limit, f)?;
                f.write_str(", but it must be at most ")?;

                Display::fmt(&GET_GUILD_BANS_LIMIT_MAX, f)
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
            ValidationErrorType::Username { len, substring }
            | ValidationErrorType::WebhookUsername { len, substring } => {
                f.write_str("provided username")?;

                if let Some(len) = len {
                    f.write_str(" length is ")?;
                    Display::fmt(len, f)?;
                    f.write_str(", but it must be at least ")?;
                    Display::fmt(&USERNAME_LIMIT_MIN, f)?;
                    f.write_str(" and at most ")?;
                    Display::fmt(&USERNAME_LIMIT_MAX, f)?;
                }

                if let Some(substring) = substring {
                    if len.is_some() {
                        f.write_str(", and")?;
                    }

                    f.write_str(" cannot contain ")?;
                    Display::fmt(substring, f)?;
                }

                Ok(())
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
    /// Provided block action custom message was too long.
    AutoModerationBlockActionCustomMessageLimit {
        /// Invalid limit.
        len: usize,
    },
    /// Provided limit was too large.
    AutoModerationMetadataMentionTotalLimit {
        /// Invalid limit.
        limit: u8,
    },
    /// Provided keyword filter was invalid.
    AutoModerationMetadataKeywordFilter {
        /// Invalid length.
        len: usize,
    },
    /// Provided keyword was invalid.
    AutoModerationMetadataKeywordFilterItem {
        /// Invalid length.
        len: usize,
        /// Invalid substring.
        substring: String,
    },
    /// Provided keyword allow list was invalid.
    AutoModerationMetadataAllowList {
        /// Invalid length.
        len: usize,
    },
    /// Provided allow list item was invalid.
    AutoModerationMetadataAllowListItem {
        /// Invalid length.
        len: usize,
        /// Invalid substring.
        substring: String,
    },
    /// Provided keyword preset allow list was invalid.
    AutoModerationMetadataPresetAllowList {
        /// Invalid length.
        len: usize,
    },
    /// Provided keyword preset allow list item was invalid.
    AutoModerationMetadataPresetAllowListItem {
        /// Invalid length.
        len: usize,
        /// Invalid substring.
        substring: String,
    },
    /// Provided seconds to disable communication was invalid.
    AutoModerationActionMetadataDurationSeconds {
        /// Invalid seconds.
        seconds: u32,
    },
    /// Provided regex patterns was invalid.
    AutoModerationMetadataRegexPatterns {
        /// Invalid length.
        len: usize,
    },
    /// Provided regex patterns item was invalid.
    AutoModerationMetadataRegexPatternsItem {
        /// Invalid length.
        len: usize,
        /// Invalid substring.
        substring: String,
    },
    /// Provided exempt roles was invalid.
    AutoModerationExemptRoles {
        /// Invalid length.
        len: usize,
    },
    /// Provided exempt channels was invalid.
    AutoModerationExemptChannels {
        /// Invalid length.
        len: usize,
    },
    /// Provided create guild ban delete message seconds was invalid.
    CreateGuildBanDeleteMessageSeconds {
        /// Invalid seconds.
        seconds: u32,
    },
    /// Provided timestamp is too far in the future.
    CommunicationDisabledUntil {
        /// Invalid timestamp.
        timestamp: Timestamp,
    },
    /// Provided get channel messages limit was invalid.
    GetChannelMessages {
        /// Invalid limit.
        limit: u16,
    },
    /// Provided get current user guilds limit was invalid.
    GetCurrentUserGuilds {
        /// Invalid limit.
        limit: u16,
    },
    /// Provided get entitlements limit was invalid.
    GetEntitlements {
        /// Invalid limit.
        limit: u8,
    },
    /// Provided get guild audit log limit was invalid.
    GetGuildAuditLog {
        /// Invalid limit.
        limit: u16,
    },
    /// Provided get guild bans limit was invalid.
    GetGuildBans {
        /// Invalid limit.
        limit: u16,
    },
    /// Provided get guild members limit was invalid.
    GetGuildMembers {
        /// Invalid limit.
        limit: u16,
    },
    /// Provided get reactions limit was invalid.
    GetReactions {
        /// Invalid limit.
        limit: u16,
    },
    /// Provided guild name was invalid.
    GuildName {
        /// Invalid length.
        len: usize,
    },
    /// Provided guild prune days was invalid.
    GuildPruneDays {
        /// Invalid days.
        days: u16,
    },
    /// Provided invite max age was invalid.
    InviteMaxAge {
        /// Invalid age.
        max_age: u32,
    },
    /// Provided invite max uses was invalid.
    InviteMaxUses {
        /// Invalid age.
        max_uses: u16,
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
        limit: u16,
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
    /// Provided username was invalid.
    Username {
        /// Invalid length.
        len: Option<usize>,
        /// Invalid substring.
        substring: Option<&'static str>,
    },
    /// Provided webhook username was invalid.
    WebhookUsername {
        /// Invalid length.
        len: Option<usize>,
        /// Invalid substring.
        substring: Option<&'static str>,
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

/// Ensure that an auto moderation block action's `custom_message` is correct.
///
/// The length must be at most [`AUTO_MODERATION_ACTION_BLOCK_CUSTOM_MESSAGE_LENGTH_MAX`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationBlockActionCustomMessageLimit`] if the
/// length is invalid.
///
/// [`AutoModerationBlockActionCustomMessageLimit`]: ValidationErrorType::AutoModerationBlockActionCustomMessageLimit
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-metadata
pub fn auto_moderation_block_action_custom_message_limit(
    custom_message: impl AsRef<str>,
) -> Result<(), ValidationError> {
    let len = custom_message.as_ref().chars().count();

    if len <= AUTO_MODERATION_ACTION_BLOCK_CUSTOM_MESSAGE_LENGTH_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationBlockActionCustomMessageLimit { len },
        })
    }
}

/// Ensure that an auto moderation rule's `mention_total_limit` is correct.
///
/// The length must be at most [`AUTO_MODERATION_METADATA_MENTION_TOTAL_LIMIT`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationMetadataMentionTotalLimit`] if the
/// length is invalid.
///
/// [`AutoModerationMetadataMentionTotalLimit`]: ValidationErrorType::AutoModerationMetadataMentionTotalLimit
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata
pub const fn auto_moderation_metadata_mention_total_limit(
    limit: u8,
) -> Result<(), ValidationError> {
    if limit <= AUTO_MODERATION_METADATA_MENTION_TOTAL_LIMIT {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationMetadataMentionTotalLimit { limit },
        })
    }
}

/// Ensure that an auto moderation rule's `keyword_filter` is correct.
///
/// The length must be at most [`AUTO_MODERATION_METADATA_KEYWORD_FILTER_LENGTH_MAX`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationMetadataKeywordFilter`] if the
/// length is invalid.
///
/// Returns an error of type [`AutoModerationMetadataKeywordFilterItem`] if any
/// of the keywords are invalid.
///
/// [`AutoModerationMetadataKeywordFilter`]: ValidationErrorType::AutoModerationMetadataKeywordFilter
/// [`AutoModerationMetadataKeywordFilterItem`]: ValidationErrorType::AutoModerationMetadataKeywordFilterItem
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata-field-limits
pub fn auto_moderation_metadata_keyword_filter(
    keywords: &[impl AsRef<str>],
) -> Result<(), ValidationError> {
    let len = keywords.len();

    if len <= AUTO_MODERATION_METADATA_KEYWORD_FILTER_MAX {
        for keyword in keywords {
            auto_moderation_metadata_keyword_filter_item(keyword)?;
        }

        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationMetadataKeywordFilter { len },
        })
    }
}

/// Ensure that an auto moderation rule's `keyword_filter` item is correct.
///
/// The length must be at most [`AUTO_MODERATION_METADATA_KEYWORD_FILTER_LENGTH_MAX`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationMetadataKeywordFilterItem`] if the
/// length is invalid.
///
/// [`AutoModerationMetadataKeywordFilterItem`]: ValidationErrorType::AutoModerationMetadataKeywordFilterItem
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata-field-limits
pub fn auto_moderation_metadata_keyword_filter_item(
    keyword: impl AsRef<str>,
) -> Result<(), ValidationError> {
    let len = keyword.as_ref().chars().count();

    if len <= AUTO_MODERATION_METADATA_KEYWORD_FILTER_LENGTH_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationMetadataKeywordFilterItem {
                len,
                substring: keyword.as_ref().to_string(),
            },
        })
    }
}

/// Ensure that an auto moderation rule's `allow_list` is correct.
///
/// The length must be at most [`AUTO_MODERATION_METADATA_PRESET_ALLOW_LIST_MAX`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationMetadataAllowList`] if the
/// length is invalid.
///
/// Returns an error of type [`AutoModerationMetadataAllowListItem`] if any of
/// the items are invalid.
///
/// [`AutoModerationMetadataAllowList`]: ValidationErrorType::AutoModerationMetadataAllowList
/// [`AutoModerationMetadataAllowListItem`]: ValidationErrorType::AutoModerationMetadataAllowListItem
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata-field-limits
pub fn auto_moderation_metadata_keyword_allow_list(
    keywords: &[impl AsRef<str>],
) -> Result<(), ValidationError> {
    let len = keywords.len();

    if len <= AUTO_MODERATION_METADATA_KEYWORD_ALLOW_LIST_MAX {
        for keyword in keywords {
            auto_moderation_metadata_keyword_allow_list_item(keyword)?;
        }

        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationMetadataAllowList { len },
        })
    }
}

/// Ensure that an auto moderation rule's `allow_list` item is correct.
///
/// The length must be at most [`AUTO_MODERATION_METADATA_PRESET_ALLOW_LIST_MAX`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationMetadataAllowListItem`] if the
/// length is invalid.
///
/// [`AutoModerationMetadataAllowListItem`]: ValidationErrorType::AutoModerationMetadataAllowListItem
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata-field-limits
pub fn auto_moderation_metadata_keyword_allow_list_item(
    keyword: impl AsRef<str>,
) -> Result<(), ValidationError> {
    let len = keyword.as_ref().chars().count();

    if len <= AUTO_MODERATION_METADATA_KEYWORD_FILTER_LENGTH_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationMetadataAllowListItem {
                len,
                substring: keyword.as_ref().to_string(),
            },
        })
    }
}

/// Ensure that an auto moderation rule's `allow_list` is correct when using presets.
///
/// The length must be at most [`AUTO_MODERATION_METADATA_PRESET_ALLOW_LIST_MAX`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationMetadataPresetAllowList`] if the
/// length is invalid.
///
/// Returns an error of type [`AutoModerationMetadataPresetAllowListItem`] if any of
/// the items are invalid.
///
/// [`AutoModerationMetadataPresetAllowList`]: ValidationErrorType::AutoModerationMetadataPresetAllowList
/// [`AutoModerationMetadataPresetAllowListItem`]: ValidationErrorType::AutoModerationMetadataPresetAllowListItem
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata-field-limits
pub fn auto_moderation_metadata_preset_allow_list(
    keywords: &[impl AsRef<str>],
) -> Result<(), ValidationError> {
    let len = keywords.len();

    if len <= AUTO_MODERATION_METADATA_PRESET_ALLOW_LIST_MAX {
        for keyword in keywords {
            auto_moderation_metadata_preset_allow_list_item(keyword)?;
        }

        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationMetadataPresetAllowList { len },
        })
    }
}

/// Ensure that an auto moderation rule's `allow_list` item is correct when using presets.
///
/// The length must be at most [`AUTO_MODERATION_METADATA_PRESET_ALLOW_LIST_LENGTH_MAX`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationMetadataPresetAllowListItem`] if the
/// length is invalid.
///
/// [`AutoModerationMetadataPresetAllowListItem`]: ValidationErrorType::AutoModerationMetadataPresetAllowListItem
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata-field-limits
pub fn auto_moderation_metadata_preset_allow_list_item(
    keyword: impl AsRef<str>,
) -> Result<(), ValidationError> {
    let len = keyword.as_ref().chars().count();

    if len <= AUTO_MODERATION_METADATA_PRESET_ALLOW_LIST_LENGTH_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationMetadataPresetAllowListItem {
                len,
                substring: keyword.as_ref().to_string(),
            },
        })
    }
}

/// Ensure that an auto moderation rule's `regex_patterns` is correct.
///
/// The length must be at most [`AUTO_MODERATION_METADATA_REGEX_PATTERNS_MAX`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationMetadataRegexPatterns`] if the
/// length is invalid.
///
/// Returns an error of type [`AutoModerationMetadataRegexPatternsItem`] if any of
/// the items are invalid.
///
/// [`AutoModerationMetadataRegexPatterns`]: ValidationErrorType::AutoModerationMetadataRegexPatterns
/// [`AutoModerationMetadataRegexPatternsItem`]: ValidationErrorType::AutoModerationMetadataRegexPatternsItem
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata-field-limits
pub fn auto_moderation_metadata_regex_patterns(
    patterns: &[impl AsRef<str>],
) -> Result<(), ValidationError> {
    let len = patterns.len();

    if len <= AUTO_MODERATION_METADATA_REGEX_PATTERNS_MAX {
        for pattern in patterns {
            auto_moderation_metadata_regex_patterns_item(pattern)?;
        }

        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationMetadataRegexPatterns { len },
        })
    }
}

/// Ensure that an auto moderation rule's `regex_patterns` item is correct.
///
/// The length must be at most [`AUTO_MODERATION_METADATA_REGEX_PATTERNS_LENGTH_MAX`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationMetadataRegexPatternsItem`] if the
/// length is invalid.
///
/// [`AutoModerationMetadataRegexPatternsItem`]: ValidationErrorType::AutoModerationMetadataRegexPatternsItem
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata-field-limits
pub fn auto_moderation_metadata_regex_patterns_item(
    pattern: impl AsRef<str>,
) -> Result<(), ValidationError> {
    let len = pattern.as_ref().chars().count();

    if len <= AUTO_MODERATION_METADATA_REGEX_PATTERNS_LENGTH_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationMetadataRegexPatternsItem {
                len,
                substring: pattern.as_ref().to_string(),
            },
        })
    }
}

/// Ensure that the `duration_seconds` field for an auto moderation action
/// metadata is correct.
///
/// The duration must be at most [`AUTO_MODERATION_ACTION_METADATA_DURATION_SECONDS_MAX`].
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationActionMetadataDurationSeconds`] if the
/// duration is invalid.
///
/// [`AutoModerationActionMetadataDurationSeconds`]: ValidationErrorType::AutoModerationActionMetadataDurationSeconds
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object-action-metadata
pub const fn auto_moderation_action_metadata_duration_seconds(
    seconds: u32,
) -> Result<(), ValidationError> {
    if seconds <= AUTO_MODERATION_ACTION_METADATA_DURATION_SECONDS_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationActionMetadataDurationSeconds { seconds },
        })
    }
}

/// Ensure that the `exempt_roles` field for an auto moderation rule is correct.
///
/// The length must be at most [`AUTO_MODERATION_EXEMPT_ROLES_MAX`]. This is based
/// on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationExemptRoles`] if the length is
/// invalid.
///
/// [`AutoModerationExemptRoles`]: ValidationErrorType::AutoModerationExemptRoles
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-auto-moderation-rule-structure
pub const fn auto_moderation_exempt_roles(roles: &[Id<RoleMarker>]) -> Result<(), ValidationError> {
    let len = roles.len();

    if len <= AUTO_MODERATION_EXEMPT_ROLES_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationExemptRoles { len },
        })
    }
}

/// Ensure that the `exempt_channels` field for an auto moderation rule is correct.
///
/// The length must be at most [`AUTO_MODERATION_EXEMPT_CHANNELS_MAX`]. This is based
/// on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`AutoModerationExemptChannels`] if the length is
/// invalid.
///
/// [`AutoModerationExemptChannels`]: ValidationErrorType::AutoModerationExemptChannels
/// [this documentation entry]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-auto-moderation-rule-structure
pub const fn auto_moderation_exempt_channels(
    channels: &[Id<ChannelMarker>],
) -> Result<(), ValidationError> {
    let len = channels.len();

    if len <= AUTO_MODERATION_EXEMPT_CHANNELS_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::AutoModerationExemptChannels { len },
        })
    }
}

/// Ensure that the delete message seconds amount for the Create Guild Ban request
/// is correct.
///
/// The seconds must be at most [`CREATE_GUILD_BAN_DELETE_MESSAGE_SECONDS_MAX`]. This
/// is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`CreateGuildBanDeleteMessageSeconds`] if the seconds is
/// invalid.
///
/// [`CreateGuildBanDeleteMessageSeconds`]: ValidationErrorType::CreateGuildBanDeleteMessageSeconds
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild#create-guild-ban
pub const fn create_guild_ban_delete_message_seconds(seconds: u32) -> Result<(), ValidationError> {
    if seconds <= CREATE_GUILD_BAN_DELETE_MESSAGE_SECONDS_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::CreateGuildBanDeleteMessageSeconds { seconds },
        })
    }
}

/// Validate that a timeout time is not too far in the future.
///
/// The time must not be farther than 28 days in the future.
///
/// # Errors
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
pub const fn get_channel_messages_limit(limit: u16) -> Result<(), ValidationError> {
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
pub const fn get_current_user_guilds_limit(limit: u16) -> Result<(), ValidationError> {
    if limit >= GET_CURRENT_USER_GUILDS_LIMIT_MIN && limit <= GET_CURRENT_USER_GUILDS_LIMIT_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GetCurrentUserGuilds { limit },
        })
    }
}

/// Ensure that the limit for the Get Entitlements endpoint is correct.
///
/// The limit must be at least [`GET_ENTITLEMENTS_LIMIT_MIN`] and at most
/// [`GET_ENTITLEMENTS_LIMIT_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`GetEntitlements`] if the limit is invalid.
///
/// [`GetEntitlements`]: ValidationErrorType::GetEntitlements
/// [this documentation entry]: https://discord.com/developers/docs/monetization/entitlements#list-entitlements
pub const fn get_entitlements_limit(limit: u8) -> Result<(), ValidationError> {
    if limit >= GET_ENTITLEMENTS_LIMIT_MIN && limit <= GET_ENTITLEMENTS_LIMIT_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GetEntitlements { limit },
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
pub const fn get_guild_audit_log_limit(limit: u16) -> Result<(), ValidationError> {
    if limit >= GET_GUILD_AUDIT_LOG_LIMIT_MIN && limit <= GET_GUILD_AUDIT_LOG_LIMIT_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GetGuildAuditLog { limit },
        })
    }
}

/// Ensure that the limit for the Get Guild Bans endpoint is correct.
///
/// The limit must be at most [`GET_GUILD_BANS_LIMIT_MAX`]. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`GetGuildBans`] if the limit is invalid.
///
/// [`GetGuildBans`]: ValidationErrorType::GetGuildBans
/// [this documentation entry]: https://discord.com/developers/docs/resources/guild#get-guild-bans
pub const fn get_guild_bans_limit(limit: u16) -> Result<(), ValidationError> {
    if limit <= GET_GUILD_BANS_LIMIT_MAX {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::GetGuildBans { limit },
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
pub const fn get_reactions_limit(limit: u16) -> Result<(), ValidationError> {
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
pub const fn guild_prune_days(days: u16) -> Result<(), ValidationError> {
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
pub const fn invite_max_age(max_age: u32) -> Result<(), ValidationError> {
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
pub const fn invite_max_uses(max_uses: u16) -> Result<(), ValidationError> {
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
pub const fn scheduled_event_get_users(limit: u16) -> Result<(), ValidationError> {
    if limit >= SCHEDULED_EVENT_GET_USERS_MIN && limit <= SCHEDULED_EVENT_GET_USERS_MAX {
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

/// Ensure that a username is correct.
///
/// The length must be at least [`USERNAME_LIMIT_MIN`] and at most
/// [`USERNAME_LIMIT_MAX`]. It must also be free of certain substrings. This is
/// based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`Username`] if the length is invalid.
///
/// [`Username`]: ValidationErrorType::Username
/// [this documentation entry]: https://discord.com/developers/docs/resources/user#usernames-and-nicknames
pub fn username(value: impl AsRef<str>) -> Result<(), ValidationError> {
    let value = value.as_ref();
    let len = value.chars().count();

    let range = USERNAME_LIMIT_MIN..=USERNAME_LIMIT_MAX;
    let invalid_len = (!range.contains(&len)).then_some(len);

    let invalid_substring = USERNAME_INVALID_SUBSTRINGS
        .into_iter()
        .find(|invalid_substring| value.contains(invalid_substring))
        .or_else(|| {
            USERNAME_INVALID_STRINGS
                .into_iter()
                .find(|invalid_string| value == *invalid_string)
        });

    if invalid_len.is_none() && invalid_substring.is_none() {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::Username {
                len: invalid_len,
                substring: invalid_substring,
            },
        })
    }
}

/// Ensure that a webhook is correct.
///
/// The length must be at least [`WEBHOOK_USERNAME_LIMIT_MIN`] and at most
/// [`WEBHOOK_USERNAME_LIMIT_MAX`]. It must also be free of certain substrings.
/// This is based on [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`WebhookUsername`] if the length is invalid.
///
/// [`WebhookUsername`]: ValidationErrorType::WebhookUsername
/// [this documentation entry]: https://discord.com/developers/docs/resources/webhook#create-webhook
pub fn webhook_username(value: impl AsRef<str>) -> Result<(), ValidationError> {
    let value = value.as_ref();
    let len = value.chars().count();

    let range = WEBHOOK_USERNAME_LIMIT_MIN..=WEBHOOK_USERNAME_LIMIT_MAX;
    let invalid_len = (!range.contains(&len)).then_some(len);

    let invalid_substring = WEBHOOK_INVALID_STRINGS
        .into_iter()
        .find(|invalid_string| value == *invalid_string);

    if invalid_len.is_none() && invalid_substring.is_none() {
        Ok(())
    } else {
        Err(ValidationError {
            kind: ValidationErrorType::WebhookUsername {
                len: invalid_len,
                substring: invalid_substring,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn username_variants() {
        let expected = format!(
            "provided username length is 200, but it must be at least {USERNAME_LIMIT_MIN} and at \
            most {USERNAME_LIMIT_MAX}, and cannot contain :"
        );
        let actual = ValidationError {
            kind: ValidationErrorType::Username {
                len: Some(200),
                substring: Some(":"),
            },
        };
        assert_eq!(expected, actual.to_string());

        let expected = format!(
            "provided username length is 200, but it must be at least {USERNAME_LIMIT_MIN} and at \
            most {USERNAME_LIMIT_MAX}",
        );
        let actual = ValidationError {
            kind: ValidationErrorType::Username {
                len: Some(200),
                substring: None,
            },
        };
        assert_eq!(expected, actual.to_string());

        let expected = "provided username cannot contain :".to_string();
        let actual = ValidationError {
            kind: ValidationErrorType::Username {
                len: None,
                substring: Some(":"),
            },
        };
        assert_eq!(expected, actual.to_string());
    }

    #[test]
    fn audit_reason_length() {
        assert!(audit_reason("").is_ok());
        assert!(audit_reason("a").is_ok());
        assert!(audit_reason("a".repeat(500)).is_ok());
        assert!(audit_reason("a".repeat(512)).is_ok());

        assert!(audit_reason("a".repeat(513)).is_err());
    }

    #[test]
    fn auto_moderation_block_action_custom_message() {
        assert!(auto_moderation_block_action_custom_message_limit("").is_ok());
        assert!(auto_moderation_block_action_custom_message_limit("a".repeat(150)).is_ok());
        assert!(matches!(
            auto_moderation_block_action_custom_message_limit("a".repeat(151))
                .unwrap_err()
                .kind,
            ValidationErrorType::AutoModerationBlockActionCustomMessageLimit { len: 151 }
        ));
    }

    #[test]
    fn auto_moderation_metadata_mention_total() {
        assert!(auto_moderation_metadata_mention_total_limit(0).is_ok());
        assert!(auto_moderation_metadata_mention_total_limit(1).is_ok());
        assert!(auto_moderation_metadata_mention_total_limit(50).is_ok());

        assert!(auto_moderation_metadata_mention_total_limit(51).is_err());
    }

    #[test]
    fn auto_moderation_metadata_keyword_filter_max() {
        let mut keywords = (0..1000).map(|_| "a").collect::<Vec<_>>();

        assert!(auto_moderation_metadata_keyword_filter(&[] as &[&str]).is_ok());
        assert!(auto_moderation_metadata_keyword_filter(&["a".repeat(60)]).is_ok());
        assert!(auto_moderation_metadata_keyword_filter(&keywords).is_ok());

        keywords.push("a");

        assert!(auto_moderation_metadata_keyword_filter(&["a".repeat(61)]).is_err());
        assert!(auto_moderation_metadata_keyword_filter(&keywords).is_err());
    }

    #[test]
    fn auto_moderation_metadata_keyword_allow_list_max() {
        let mut allow_list = (0..100).map(|_| "a").collect::<Vec<_>>();

        assert!(auto_moderation_metadata_keyword_allow_list(&[] as &[&str]).is_ok());
        assert!(auto_moderation_metadata_keyword_allow_list(&["a".repeat(60)]).is_ok());

        allow_list.push("a");

        assert!(auto_moderation_metadata_keyword_allow_list(&["a".repeat(61)]).is_err());
        assert!(auto_moderation_metadata_keyword_allow_list(&allow_list).is_err());
    }

    #[test]
    fn auto_moderation_metadata_preset_allow_list_max() {
        let mut allow_list = (0..1000).map(|_| "a").collect::<Vec<_>>();

        assert!(auto_moderation_metadata_preset_allow_list(&[] as &[&str]).is_ok());
        assert!(auto_moderation_metadata_preset_allow_list(&["a".repeat(60)]).is_ok());

        allow_list.push("a");

        assert!(auto_moderation_metadata_preset_allow_list(&["a".repeat(61)]).is_err());
        assert!(auto_moderation_metadata_preset_allow_list(&allow_list).is_err());
    }

    #[test]
    fn auto_moderation_metadata_regex_patterns_max() {
        let mut patterns = (0..10).map(|_| "a").collect::<Vec<_>>();

        assert!(auto_moderation_metadata_regex_patterns(&[] as &[&str]).is_ok());
        assert!(auto_moderation_metadata_regex_patterns(&["a".repeat(260)]).is_ok());

        patterns.push("a");

        assert!(auto_moderation_metadata_regex_patterns(&["a".repeat(261)]).is_err());
        assert!(auto_moderation_metadata_regex_patterns(&patterns).is_err());
    }

    #[test]
    fn auto_moderation_exempt_roles_max() {
        let mut roles = (1..=20).map(Id::new).collect::<Vec<_>>();

        assert!(auto_moderation_exempt_roles(&[]).is_ok());
        assert!(auto_moderation_exempt_roles(&roles).is_ok());

        roles.push(Id::new(21));

        assert!(auto_moderation_exempt_roles(&roles).is_err());
    }

    #[test]
    fn auto_moderation_exempt_channels_max() {
        let mut channels = (1..=50).map(Id::new).collect::<Vec<_>>();

        assert!(auto_moderation_exempt_channels(&[]).is_ok());
        assert!(auto_moderation_exempt_channels(&channels).is_ok());

        channels.push(Id::new(51));

        assert!(auto_moderation_exempt_channels(&channels).is_err());
    }

    #[test]
    fn auto_moderation_action_metadata_duration_seconds_max() {
        assert!(auto_moderation_action_metadata_duration_seconds(0).is_ok());
        assert!(auto_moderation_action_metadata_duration_seconds(1).is_ok());
        assert!(auto_moderation_action_metadata_duration_seconds(2_419_200).is_ok());

        assert!(auto_moderation_action_metadata_duration_seconds(2_419_201).is_err());
    }

    #[test]
    fn create_guild_ban_delete_message_seconds_max() {
        assert!(create_guild_ban_delete_message_seconds(0).is_ok());
        assert!(create_guild_ban_delete_message_seconds(1).is_ok());
        assert!(create_guild_ban_delete_message_seconds(604_800).is_ok());

        assert!(create_guild_ban_delete_message_seconds(604_801).is_err());
    }

    #[test]
    fn communication_disabled_until_max() {
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
    fn get_channel_messages_limit_count() {
        assert!(get_channel_messages_limit(1).is_ok());
        assert!(get_channel_messages_limit(100).is_ok());

        assert!(get_channel_messages_limit(0).is_err());
        assert!(get_channel_messages_limit(101).is_err());
    }

    #[test]
    fn get_current_user_guilds_limit_count() {
        assert!(get_current_user_guilds_limit(1).is_ok());
        assert!(get_current_user_guilds_limit(200).is_ok());

        assert!(get_current_user_guilds_limit(0).is_err());
        assert!(get_current_user_guilds_limit(201).is_err());
    }

    #[test]
    fn get_guild_log_limit_count() {
        assert!(get_guild_audit_log_limit(1).is_ok());
        assert!(get_guild_audit_log_limit(100).is_ok());

        assert!(get_guild_audit_log_limit(0).is_err());
        assert!(get_guild_audit_log_limit(101).is_err());
    }

    #[test]
    fn get_guild_bans_limit_count() {
        assert!(get_guild_bans_limit(0).is_ok());
        assert!(get_guild_bans_limit(1000).is_ok());

        assert!(get_guild_bans_limit(1001).is_err());
    }

    #[test]
    fn get_guild_members_limit_count() {
        assert!(get_guild_members_limit(1).is_ok());
        assert!(get_guild_members_limit(1000).is_ok());

        assert!(get_guild_members_limit(0).is_err());
        assert!(get_guild_members_limit(1001).is_err());
    }

    #[test]
    fn get_reactions_limit_count() {
        assert!(get_reactions_limit(1).is_ok());
        assert!(get_reactions_limit(100).is_ok());

        assert!(get_reactions_limit(0).is_err());
        assert!(get_reactions_limit(101).is_err());
    }

    #[test]
    fn guild_name_length() {
        assert!(guild_name("aa").is_ok());
        assert!(guild_name("a".repeat(100)).is_ok());

        assert!(guild_name("").is_err());
        assert!(guild_name("a").is_err());
        assert!(guild_name("a".repeat(101)).is_err());
    }

    #[test]
    fn guild_prune_days_length() {
        assert!(guild_prune_days(1).is_ok());
        assert!(guild_prune_days(30).is_ok());

        assert!(guild_prune_days(0).is_err());
        assert!(guild_prune_days(31).is_err());
        assert!(guild_prune_days(100).is_err());
    }

    #[test]
    fn invite_max_age_length() {
        assert!(invite_max_age(0).is_ok());
        assert!(invite_max_age(86_400).is_ok());
        assert!(invite_max_age(604_800).is_ok());

        assert!(invite_max_age(604_801).is_err());
    }

    #[test]
    fn invite_max_uses_count() {
        assert!(invite_max_uses(0).is_ok());
        assert!(invite_max_uses(100).is_ok());

        assert!(invite_max_uses(101).is_err());
    }

    #[test]
    fn nickname_length() {
        assert!(nickname("a").is_ok());
        assert!(nickname("a".repeat(32)).is_ok());

        assert!(nickname("").is_err());
        assert!(nickname("a".repeat(33)).is_err());
    }

    #[test]
    fn scheduled_event_description_length() {
        assert!(scheduled_event_description("a").is_ok());
        assert!(scheduled_event_description("a".repeat(1000)).is_ok());

        assert!(scheduled_event_description("").is_err());
        assert!(scheduled_event_description("a".repeat(1001)).is_err());
    }

    #[test]
    fn scheduled_event_get_users_length() {
        assert!(scheduled_event_get_users(0).is_err());
        assert!(scheduled_event_get_users(101).is_err());
        assert!(scheduled_event_get_users(100).is_ok());
        assert!(scheduled_event_get_users(1).is_ok());
    }

    #[test]
    fn scheduled_event_name_length() {
        assert!(scheduled_event_name("a").is_ok());
        assert!(scheduled_event_name("a".repeat(100)).is_ok());

        assert!(scheduled_event_name("").is_err());
        assert!(scheduled_event_name("a".repeat(101)).is_err());
    }

    #[test]
    fn search_guild_members_limit_count() {
        assert!(search_guild_members_limit(1).is_ok());
        assert!(search_guild_members_limit(1000).is_ok());

        assert!(search_guild_members_limit(0).is_err());
        assert!(search_guild_members_limit(1001).is_err());
    }

    #[test]
    fn stage_topic_length() {
        assert!(stage_topic("a").is_ok());
        assert!(stage_topic("a".repeat(120)).is_ok());

        assert!(stage_topic("").is_err());
        assert!(stage_topic("a".repeat(121)).is_err());
    }

    #[test]
    fn template_description_length() {
        assert!(template_description("").is_ok());
        assert!(template_description("a").is_ok());
        assert!(template_description("a".repeat(120)).is_ok());

        assert!(template_description("a".repeat(121)).is_err());
    }

    #[test]
    fn template_name_length() {
        assert!(template_name("a").is_ok());
        assert!(template_name("a".repeat(100)).is_ok());

        assert!(template_name("").is_err());
        assert!(template_name("a".repeat(101)).is_err());
    }

    #[test]
    fn username_length() {
        assert!(username("aa").is_ok());
        assert!(username("a".repeat(32)).is_ok());

        assert!(username("a").is_err());
        assert!(username("a".repeat(33)).is_err());

        assert!(username("no @ in username").is_err());
        assert!(username("no # in username").is_err());
        assert!(username("no : in username").is_err());
        assert!(username(r"no ``` in username").is_err());
        assert!(username("no discord in username").is_err());
        assert!(username("everyone").is_err());
        assert!(username("here").is_err());
    }

    #[test]
    fn webhook_username_length() {
        assert!(webhook_username("aa").is_ok());
        assert!(webhook_username("a".repeat(80)).is_ok());

        assert!(webhook_username("a").is_err());
        assert!(webhook_username("a".repeat(81)).is_err());

        assert!(webhook_username("clyde").is_err());
    }
}
