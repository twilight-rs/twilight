use serde::{
    de::{Error as DeError, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ErrorCode {
    /// General error (such as a malformed request body, amongst other things)
    GeneralError,
    /// Unknown account
    UnknownAccount,
    /// Unknown application
    UnknownApplication,
    /// Unknown channel
    UnknownChannel,
    /// Unknown guild
    UnknownGuild,
    /// Unknown integration
    UnknownIntegration,
    /// Unknown invite
    UnknownInvite,
    /// Unknown member
    UnknownMember,
    /// Unknown message
    UnknownMessage,
    /// Unknown permission overwrite
    UnknownPermissionOverwrite,
    /// Unknown provider
    UnknownProvider,
    /// Unknown role
    UnknownRole,
    /// Unknown token
    UnknownToken,
    /// Unknown user
    UnknownUser,
    /// Unknown emoji
    UnknownEmoji,
    /// Unknown webhook
    UnknownWebhook,
    /// Unknown ban
    UnknownBan,
    /// Unknown SKU
    #[allow(clippy::upper_case_acronyms)]
    UnknownSKU,
    /// Unknown Store Listing
    UnknownStoreListing,
    /// Unknown entitlement
    UnknownEntitlement,
    /// Unknown build
    UnknownBuild,
    /// Unknown lobby
    UnknownLobby,
    /// Unknown branch
    UnknownBranch,
    /// Unknown redistributable
    UnknownRedistributable,
    /// Bots cannot use this endpoint
    BotsCannotUseEndpoint,
    /// Only bots can use this endpoint
    OnlyBotsCanUseEndpoint,
    /// Message cannot be edited due to announcement rate limits
    AnnouncementRateLimitReached,
    /// Maximum number of guilds reached (100)
    MaximumGuildsReached,
    /// Maximum number of friends reached (1000)
    MaximumFriendsReached,
    /// Maximum number of pins reached for the channel (50)
    MaximumPinsReached,
    /// Maximum number of guild roles reached (250)
    MaximumRolesReached,
    /// Maximum number of webhooks reached (10)
    MaximumWebhooksReached,
    /// Maximum number of reactions reached (20)
    MaximumReactionsReached,
    /// Maximum number of guild channels reached (500)
    MaximumGuildChannelsReached,
    /// Maximum number of attachments in a message reached (10)
    MaximumAttachmentsReached,
    /// Maximum number of invites reached (1000)
    MaximumInvitesReached,
    /// Unauthorized. Provide a valid token and try again
    Unauthorized,
    /// You need to verify your account in order to perform this action
    AccountNeedsVerification,
    /// Request entity too large. Try sending something smaller in size
    RequestEntityTooLarge,
    /// This feature has been temporarily disabled server-side
    FeatureTemporarilyDisabled,
    /// The user is banned from this guild
    UserBannedFromGuild,
    /// This message has already been crossposted
    MessageAlreadyCrossposted,
    /// Missing access
    Missingaccess,
    /// Invalid account type
    InvalidAccountType,
    /// Cannot execute action on a DM channel
    #[allow(clippy::upper_case_acronyms)]
    InvalidDMChannelAction,
    /// Guild widget disabled
    GuildWidgetDisabled,
    /// Cannot edit a message authored by another user
    MessageNotAuthoredByUser,
    /// Cannot send an empty message
    EmptyMessage,
    /// Cannot send messages to this user
    CannotSendMessageToUser,
    /// Cannot send messages in a voice channel
    CannotSendMessagesInVoiceChannel,
    /// Channel verification level is too high for you to gain access
    VerificationLevelTooHigh,
    /// OAuth2 application does not have a bot
    OAuthApplicationHasNoBot,
    /// OAuth2 application limit reached
    OAuthApplicationLimitReached,
    /// Invalid OAuth2 state
    InvalidOAuthSstate,
    /// You lack permissions to perform that action
    PermissionsLacking,
    /// Invalid authentication token provided
    InvalidAuthenticationTokenProvided,
    /// Note was too long
    NoteTooLong,
    /// Provided too few or too many messages to delete. Must provide at least 2 and fewer than 100 messages to delete
    InvalidMessageDeleteRange,
    /// A message can only be pinned to the channel it was sent in
    MessagePinnedInWrongChannel,
    /// Invite code was either invalid or taken
    InviteCodeInvalidOrTaken,
    /// Cannot execute action on a system message
    InvalidActionOnSystemMessage,
    /// Cannot execute action on this channel type
    CannotExecuteActionOnChannelType,
    /// Invalid OAuth2 access token provided
    InvalidOAuthAccessToken,
    /// Invalid webhook token provided
    InvalidWebhookToken,
    /// Invalid recipient(s)
    InvalidRecipient,
    /// A message provided was too old to bulk delete
    MessageTooOldToBulkDelete,
    /// Invalid form body (returned for both application/json and multipart/form-data bodies), or invalid Content-Type provided
    InvalidFormBodyOrContentType,
    /// An invite was accepted to a guild the application's bot is not in
    InviteAcceptedToGuildBotNotIn,
    /// Invalid API version provided
    InvalidApiVersion,
    /// Invalid sticker sent
    InvalidStickerSent,
    /// Reaction was blocked
    ReactionBlocked,
    /// API resource is currently overloaded. Try again a little later
    ApiResourceOverloaded,
    /// A status code that Twilight doesn't have registered.
    ///
    /// Please report the number if you see this variant!
    Other(u64),
}

impl ErrorCode {
    pub fn num(&self) -> u64 {
        match self {
            Self::GeneralError => 0,
            Self::UnknownAccount => 10001,
            Self::UnknownApplication => 10002,
            Self::UnknownChannel => 10003,
            Self::UnknownGuild => 10004,
            Self::UnknownIntegration => 10005,
            Self::UnknownInvite => 10006,
            Self::UnknownMember => 10007,
            Self::UnknownMessage => 10008,
            Self::UnknownPermissionOverwrite => 10009,
            Self::UnknownProvider => 10010,
            Self::UnknownRole => 10011,
            Self::UnknownToken => 10012,
            Self::UnknownUser => 10013,
            Self::UnknownEmoji => 10014,
            Self::UnknownWebhook => 10015,
            Self::UnknownBan => 10026,
            Self::UnknownSKU => 10027,
            Self::UnknownStoreListing => 10028,
            Self::UnknownEntitlement => 10029,
            Self::UnknownBuild => 10030,
            Self::UnknownLobby => 10031,
            Self::UnknownBranch => 10032,
            Self::UnknownRedistributable => 10036,
            Self::BotsCannotUseEndpoint => 20001,
            Self::OnlyBotsCanUseEndpoint => 20002,
            Self::AnnouncementRateLimitReached => 20022,
            Self::MaximumGuildsReached => 30001,
            Self::MaximumFriendsReached => 30002,
            Self::MaximumPinsReached => 30003,
            Self::MaximumRolesReached => 30005,
            Self::MaximumWebhooksReached => 30007,
            Self::MaximumReactionsReached => 30010,
            Self::MaximumGuildChannelsReached => 30013,
            Self::MaximumAttachmentsReached => 30015,
            Self::MaximumInvitesReached => 30016,
            Self::Unauthorized => 40001,
            Self::AccountNeedsVerification => 40002,
            Self::RequestEntityTooLarge => 40005,
            Self::FeatureTemporarilyDisabled => 40006,
            Self::UserBannedFromGuild => 40007,
            Self::MessageAlreadyCrossposted => 40033,
            Self::Missingaccess => 50001,
            Self::InvalidAccountType => 50002,
            Self::InvalidDMChannelAction => 50003,
            Self::GuildWidgetDisabled => 50004,
            Self::MessageNotAuthoredByUser => 50005,
            Self::EmptyMessage => 50006,
            Self::CannotSendMessageToUser => 50007,
            Self::CannotSendMessagesInVoiceChannel => 50008,
            Self::VerificationLevelTooHigh => 50009,
            Self::OAuthApplicationHasNoBot => 50010,
            Self::OAuthApplicationLimitReached => 50011,
            Self::InvalidOAuthSstate => 50012,
            Self::PermissionsLacking => 50013,
            Self::InvalidAuthenticationTokenProvided => 50014,
            Self::NoteTooLong => 50015,
            Self::InvalidMessageDeleteRange => 50016,
            Self::MessagePinnedInWrongChannel => 50019,
            Self::InviteCodeInvalidOrTaken => 50020,
            Self::InvalidActionOnSystemMessage => 50021,
            Self::CannotExecuteActionOnChannelType => 50024,
            Self::InvalidOAuthAccessToken => 50025,
            Self::InvalidWebhookToken => 50027,
            Self::InvalidRecipient => 50033,
            Self::MessageTooOldToBulkDelete => 50034,
            Self::InvalidFormBodyOrContentType => 50035,
            Self::InviteAcceptedToGuildBotNotIn => 50036,
            Self::InvalidApiVersion => 50041,
            Self::InvalidStickerSent => 50081,
            Self::ReactionBlocked => 90001,
            Self::ApiResourceOverloaded => 130_000,
            Self::Other(other) => *other,
        }
    }
}

impl From<u64> for ErrorCode {
    fn from(int: u64) -> Self {
        match int {
            0 => Self::GeneralError,
            10001 => Self::UnknownAccount,
            10002 => Self::UnknownApplication,
            10003 => Self::UnknownChannel,
            10004 => Self::UnknownGuild,
            10005 => Self::UnknownIntegration,
            10006 => Self::UnknownInvite,
            10007 => Self::UnknownMember,
            10008 => Self::UnknownMessage,
            10009 => Self::UnknownPermissionOverwrite,
            10010 => Self::UnknownProvider,
            10011 => Self::UnknownRole,
            10012 => Self::UnknownToken,
            10013 => Self::UnknownUser,
            10014 => Self::UnknownEmoji,
            10015 => Self::UnknownWebhook,
            10026 => Self::UnknownBan,
            10027 => Self::UnknownSKU,
            10028 => Self::UnknownStoreListing,
            10029 => Self::UnknownEntitlement,
            10030 => Self::UnknownBuild,
            10031 => Self::UnknownLobby,
            10032 => Self::UnknownBranch,
            10036 => Self::UnknownRedistributable,
            20001 => Self::BotsCannotUseEndpoint,
            20002 => Self::OnlyBotsCanUseEndpoint,
            20022 => Self::AnnouncementRateLimitReached,
            30001 => Self::MaximumGuildsReached,
            30002 => Self::MaximumFriendsReached,
            30003 => Self::MaximumPinsReached,
            30005 => Self::MaximumRolesReached,
            30007 => Self::MaximumWebhooksReached,
            30010 => Self::MaximumReactionsReached,
            30013 => Self::MaximumGuildChannelsReached,
            30015 => Self::MaximumAttachmentsReached,
            30016 => Self::MaximumInvitesReached,
            40001 => Self::Unauthorized,
            40002 => Self::AccountNeedsVerification,
            40005 => Self::RequestEntityTooLarge,
            40006 => Self::FeatureTemporarilyDisabled,
            40007 => Self::UserBannedFromGuild,
            40033 => Self::MessageAlreadyCrossposted,
            50001 => Self::Missingaccess,
            50002 => Self::InvalidAccountType,
            50003 => Self::InvalidDMChannelAction,
            50004 => Self::GuildWidgetDisabled,
            50005 => Self::MessageNotAuthoredByUser,
            50006 => Self::EmptyMessage,
            50007 => Self::CannotSendMessageToUser,
            50008 => Self::CannotSendMessagesInVoiceChannel,
            50009 => Self::VerificationLevelTooHigh,
            50010 => Self::OAuthApplicationHasNoBot,
            50011 => Self::OAuthApplicationLimitReached,
            50012 => Self::InvalidOAuthSstate,
            50013 => Self::PermissionsLacking,
            50014 => Self::InvalidAuthenticationTokenProvided,
            50015 => Self::NoteTooLong,
            50016 => Self::InvalidMessageDeleteRange,
            50019 => Self::MessagePinnedInWrongChannel,
            50020 => Self::InviteCodeInvalidOrTaken,
            50021 => Self::InvalidActionOnSystemMessage,
            50024 => Self::CannotExecuteActionOnChannelType,
            50025 => Self::InvalidOAuthAccessToken,
            50027 => Self::InvalidWebhookToken,
            50033 => Self::InvalidRecipient,
            50034 => Self::MessageTooOldToBulkDelete,
            50035 => Self::InvalidFormBodyOrContentType,
            50036 => Self::InviteAcceptedToGuildBotNotIn,
            50041 => Self::InvalidApiVersion,
            50081 => Self::InvalidStickerSent,
            90001 => Self::ReactionBlocked,
            130_000 => Self::ApiResourceOverloaded,
            other => Self::Other(other),
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::GeneralError => f.write_str("General error (such as a malformed request body, amongst other things)"),
            Self::UnknownAccount => f.write_str("Unknown account"),
            Self::UnknownApplication => f.write_str("Unknown application"),
            Self::UnknownChannel => f.write_str("Unknown channel"),
            Self::UnknownGuild => f.write_str("Unknown guild"),
            Self::UnknownIntegration => f.write_str("Unknown integration"),
            Self::UnknownInvite => f.write_str("Unknown invite"),
            Self::UnknownMember => f.write_str("Unknown member"),
            Self::UnknownMessage => f.write_str("Unknown message"),
            Self::UnknownPermissionOverwrite => f.write_str("Unknown permission overwrite"),
            Self::UnknownProvider => f.write_str("Unknown provider"),
            Self::UnknownRole => f.write_str("Unknown role"),
            Self::UnknownToken => f.write_str("Unknown token"),
            Self::UnknownUser => f.write_str("Unknown user"),
            Self::UnknownEmoji => f.write_str("Unknown emoji"),
            Self::UnknownWebhook => f.write_str("Unknown webhook"),
            Self::UnknownBan => f.write_str("Unknown ban"),
            Self::UnknownSKU => f.write_str("Unknown SKU"),
            Self::UnknownStoreListing => f.write_str("Unknown Store Listing"),
            Self::UnknownEntitlement => f.write_str("Unknown entitlement"),
            Self::UnknownBuild => f.write_str("Unknown build"),
            Self::UnknownLobby => f.write_str("Unknown lobby"),
            Self::UnknownBranch => f.write_str("Unknown branch"),
            Self::UnknownRedistributable => f.write_str("Unknown redistributable"),
            Self::BotsCannotUseEndpoint => f.write_str("Bots cannot use this endpoint"),
            Self::OnlyBotsCanUseEndpoint => f.write_str("Only bots can use this endpoint"),
            Self::AnnouncementRateLimitReached => f.write_str("Message cannot be edited due to announcement rate limits"),
            Self::MaximumGuildsReached => f.write_str("Maximum number of guilds reached (100)"),
            Self::MaximumFriendsReached => f.write_str("Maximum number of friends reached (1000)"),
            Self::MaximumPinsReached => f.write_str("Maximum number of pins reached for the channel (50)"),
            Self::MaximumRolesReached => f.write_str("Maximum number of guild roles reached (250)"),
            Self::MaximumWebhooksReached => f.write_str("Maximum number of webhooks reached (10)"),
            Self::MaximumReactionsReached => f.write_str("Maximum number of reactions reached (20)"),
            Self::MaximumGuildChannelsReached => f.write_str("Maximum number of guild channels reached (500)"),
            Self::MaximumAttachmentsReached => f.write_str("Maximum number of attachments in a message reached (10)"),
            Self::MaximumInvitesReached => f.write_str("Maximum number of invites reached (1000)"),
            Self::Unauthorized => f.write_str("Unauthorized. Provide a valid token and try again"),
            Self::AccountNeedsVerification => f.write_str("You need to verify your account in order to perform this action"),
            Self::RequestEntityTooLarge => f.write_str("Request entity too large. Try sending something smaller in size"),
            Self::FeatureTemporarilyDisabled => f.write_str("This feature has been temporarily disabled server-side"),
            Self::UserBannedFromGuild => f.write_str("The user is banned from this guild"),
            Self::MessageAlreadyCrossposted => f.write_str("This message has already been crossposted"),
            Self::Missingaccess => f.write_str("Missing access"),
            Self::InvalidAccountType => f.write_str("Invalid account type"),
            Self::InvalidDMChannelAction => f.write_str("Cannot execute action on a DM channel"),
            Self::GuildWidgetDisabled => f.write_str("Guild widget disabled"),
            Self::MessageNotAuthoredByUser => f.write_str("Cannot edit a message authored by another user"),
            Self::EmptyMessage => f.write_str("Cannot send an empty message"),
            Self::CannotSendMessageToUser => f.write_str("Cannot send messages to this user"),
            Self::CannotSendMessagesInVoiceChannel => f.write_str("Cannot send messages in a voice channel"),
            Self::VerificationLevelTooHigh => f.write_str("Channel verification level is too high for you to gain access"),
            Self::OAuthApplicationHasNoBot => f.write_str("OAuth2 application does not have a bot"),
            Self::OAuthApplicationLimitReached => f.write_str("OAuth2 application limit reached"),
            Self::InvalidOAuthSstate => f.write_str("Invalid OAuth2 state"),
            Self::PermissionsLacking => f.write_str("You lack permissions to perform that action"),
            Self::InvalidAuthenticationTokenProvided => f.write_str("Invalid authentication token provided"),
            Self::NoteTooLong => f.write_str("Note was too long"),
            Self::InvalidMessageDeleteRange => f.write_str("Provided too few or too many messages to delete. Must provide at least 2 and fewer than 100 messages to delete"),
            Self::MessagePinnedInWrongChannel => f.write_str("A message can only be pinned to the channel it was sent in"),
            Self::InviteCodeInvalidOrTaken => f.write_str("Invite code was either invalid or taken"),
            Self::InvalidActionOnSystemMessage => f.write_str("Cannot execute action on a system message"),
            Self::CannotExecuteActionOnChannelType => f.write_str("Cannot execute action on channel type"),
            Self::InvalidOAuthAccessToken => f.write_str("Invalid OAuth2 access token provided"),
            Self::InvalidWebhookToken => f.write_str("Invalid webhook token provided."),
            Self::InvalidRecipient => f.write_str("Invalid recipient(s)"),
            Self::MessageTooOldToBulkDelete => f.write_str("A message provided was too old to bulk delete"),
            Self::InvalidFormBodyOrContentType => f.write_str("Invalid form body (returned for both application/json and multipart/form-data bodies), or invalid Content-Type provided"),
            Self::InviteAcceptedToGuildBotNotIn => f.write_str("An invite was accepted to a guild the application's bot is not in"),
            Self::InvalidApiVersion => f.write_str("Invalid API version provided"),
            Self::InvalidStickerSent => f.write_str("Invalid sticker sent"),
            Self::ReactionBlocked => f.write_str("Reaction was blocked"),
            Self::ApiResourceOverloaded => f.write_str("API resource is currently overloaded. Try again a little later"),
            Self::Other(number) => write!(f, "An error code Twilight doesn't have registered: {}", number),
        }
    }
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ErrorCodeVisitor;

        impl<'de> Visitor<'de> for ErrorCodeVisitor {
            type Value = ErrorCode;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("a positive integer")
            }

            fn visit_u8<E: DeError>(self, value: u8) -> Result<Self::Value, E> {
                self.visit_u64(u64::from(value))
            }

            fn visit_u16<E: DeError>(self, value: u16) -> Result<Self::Value, E> {
                self.visit_u64(u64::from(value))
            }

            fn visit_u32<E: DeError>(self, value: u32) -> Result<Self::Value, E> {
                self.visit_u64(u64::from(value))
            }

            fn visit_u64<E: DeError>(self, int: u64) -> Result<Self::Value, E> {
                Ok(ErrorCode::from(int))
            }
        }

        deserializer.deserialize_u64(ErrorCodeVisitor)
    }
}

impl Serialize for ErrorCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.num())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum ApiError {
    General(GeneralApiError),
    /// Something was wrong with the input when sending a message.
    Message(MessageApiError),
    Ratelimited(RatelimitedApiError),
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::General(inner) => Display::fmt(inner, f),
            Self::Message(inner) => Display::fmt(inner, f),
            Self::Ratelimited(inner) => Display::fmt(inner, f),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct GeneralApiError {
    pub code: ErrorCode,
    pub message: String,
}

impl Display for GeneralApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!(
            "Error code {}: {}",
            self.code.num(),
            self.message
        ))
    }
}

/// Sending a message failed because the provided fields contained invalid
/// input.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct MessageApiError {
    /// Fields within a provided embed were invalid.
    pub embed: Option<Vec<MessageApiErrorEmbedField>>,
}

impl Display for MessageApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("message fields invalid: ")?;

        if let Some(embed) = &self.embed {
            f.write_str("embed (")?;

            let field_count = embed.len().saturating_sub(1);

            for (idx, field) in embed.iter().enumerate() {
                Display::fmt(field, f)?;

                if idx == field_count {
                    f.write_str(", ")?;
                }
            }

            f.write_str(")")?;
        }

        Ok(())
    }
}

/// Field within a [`MessageApiError`] [embed] list.
///
/// [embed]: MessageApiError::embed
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum MessageApiErrorEmbedField {
    /// Something was wrong with the provided fields.
    Fields,
    /// The provided timestamp wasn't a valid RFC3339 string.
    Timestamp,
}

impl Display for MessageApiErrorEmbedField {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(match self {
            Self::Fields => "fields",
            Self::Timestamp => "timestamp",
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct RatelimitedApiError {
    /// Whether the ratelimit is a global ratelimit.
    pub global: bool,
    /// Human readable message provided by the API.
    pub message: String,
    /// Amount of time to wait before retrying.
    pub retry_after: f64,
}

impl Display for RatelimitedApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("Got ")?;

        if self.global {
            f.write_str("global ")?;
        }

        write!(f, "ratelimited for {}s", self.retry_after)
    }
}

impl Eq for RatelimitedApiError {}

impl PartialEq for RatelimitedApiError {
    fn eq(&self, other: &Self) -> bool {
        self.global == other.global && self.message == other.message
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ApiError, ErrorCode, GeneralApiError, MessageApiError, MessageApiErrorEmbedField,
        RatelimitedApiError,
    };
    use serde_test::Token;

    #[test]
    fn test_api_error_deser() {
        let expected = GeneralApiError {
            code: ErrorCode::UnknownAccount,
            message: "Unknown account".to_owned(),
        };

        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "GeneralApiError",
                    len: 2,
                },
                Token::Str("code"),
                Token::U64(10001),
                Token::Str("message"),
                Token::Str("Unknown account"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_api_error_message() {
        let expected = ApiError::Message(MessageApiError {
            embed: Some(
                [
                    MessageApiErrorEmbedField::Fields,
                    MessageApiErrorEmbedField::Timestamp,
                ]
                .to_vec(),
            ),
        });

        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "MessageApiError",
                    len: 1,
                },
                Token::Str("embed"),
                Token::Some,
                Token::Seq { len: Some(2) },
                Token::UnitVariant {
                    name: "MessageApiErrorEmbedField",
                    variant: "fields",
                },
                Token::UnitVariant {
                    name: "MessageApiErrorEmbedField",
                    variant: "timestamp",
                },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_api_error_ratelimited() {
        let expected = RatelimitedApiError {
            global: true,
            message: "You are being rate limited.".to_owned(),
            retry_after: 6.457,
        };

        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "RatelimitedApiError",
                    len: 3,
                },
                Token::Str("global"),
                Token::Bool(true),
                Token::Str("message"),
                Token::Str("You are being rate limited."),
                Token::Str("retry_after"),
                Token::F64(6.457),
                Token::StructEnd,
            ],
        );
    }
}
