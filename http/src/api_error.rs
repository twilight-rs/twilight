use serde::{
    de::{Error as DeError, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Helper macro for generating error codes.
///
/// Takes a list of tuples containing three arguments: display, name & code.
macro_rules! error_code {
    ($((
        $display:literal, $name:ident, $code:literal
    )),*) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum ErrorCode {
            $(
                #[doc = $display]
                $name
            ),*,
            /// A status code that Twilight doesn't have registered.
            ///
            /// Please report the number if you see this variant!
            Other(u64)
        }

        impl ErrorCode {
            pub const fn num(&self) -> u64 {
                match self {
                    $(Self::$name => $code),*,
                    Self::Other(other) => *other,
                }
            }
        }

        impl From<u64> for ErrorCode {
            fn from(int: u64) -> Self {
                match int {
                    $($code => Self::$name),*,
                    other => Self::Other(other),
                }
            }
        }

        impl Display for ErrorCode {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    $(Self::$name => f.write_str($display)),*,
                    Self::Other(number) => {
                        f.write_str("An error code Twilight doesn't have registered: ")?;

                        Display::fmt(number, f)
                    }

                }
            }
        }
    }
}

error_code![
    (
        "General error (such as a malformed request body, amongst other things)",
        GeneralError,
        0
    ),
    ("Unknown account", UnknownAccount, 10001),
    ("Unknown application", UnknownApplication, 10002),
    ("Unknown channel", UnknownChannel, 10003),
    ("Unknown guild", UnknownGuild, 10004),
    ("Unknown integration", UnknownIntegration, 10005),
    ("Unknown invite", UnknownInvite, 10006),
    ("Unknown member", UnknownMember, 10007),
    ("Unknown message", UnknownMessage, 10008),
    (
        "Unknown permission overwrite",
        UnknownPermissionOverwrite,
        10009
    ),
    ("Unknown provider",
    UnknownProvider , 10010),
    ("Unknown role",
    UnknownRole , 10011),
    ("Unknown token",
    UnknownToken , 10012),
    ("Unknown user",
    UnknownUser , 10013),
    ("Unknown emoji",
    UnknownEmoji , 10014),
    ("Unknown webhook",
    UnknownWebhook , 10015),
    ("Unknown webhook service",
    UnknownWebhookService , 10016),
    ("Unknown session",
    UnknownSession , 10020),
    ("Unknown ban",
    UnknownBan , 10026),
    ("Unknown SKU",
    UnknownSKU , 10027),
    ("Unknown Store Listing",
    UnknownStoreListing , 10028),
    ("Unknown entitlement",
    UnknownEntitlement , 10029),
    ("Unknown build",
    UnknownBuild , 10030),
    ("Unknown lobby",
    UnknownLobby , 10031),
    ("Unknown branch",
    UnknownBranch , 10032),
    ("Unknown store directory layout",
    UnknownStoreDirectoryLayout , 10033),
    ("Unknown redistributable",
    UnknownRedistributable , 10036),
    ("Unknown gift code",
    UnknownGiftCode , 10038),
    ("Unknown stream",
    UnknownStream , 10049),
    ("Unknown premium server subscribe cooldown",
    UnknownPremiumServerSubscribeCooldown , 10050),
    ("Unknown guild template",
    UnknownGuildTemplate , 10057),
    ("Unknown discoverable server category",
    UnknownDiscoverableServerCategory , 10059),
    ("Unknown sticker",
    UnknownSticker , 10060),
    ("Unknown interaction",
    UnknownInteraction , 10062),
    ("Unknown application command",
    UnknownApplicationCommand , 10063),
    ("Unknown application command permissions",
    UnknownApplicationCommandPermissions , 10066),
    ("Unknown Stage Instance",
    UnknownStageInstance , 10067),
    ("Unknown Guild Member Verification Form",
    UnknownGuildMemberVerificationForm , 10068),
    ("Unknown Guild Welcome Screen",
    UnknownGuildWelcomeScreen , 10069),
    ("Unknown guild scheduled event",
    UnknownGuildScheduledEvent , 10070),
    ("Unknown guild scheduled event user",
    UnknownGuildScheduledEventUser , 10071),
    ("Bots cannot use this endpoint",
    BotsCannotUseEndpoint , 20001),
    ("Only bots can use this endpoint",
    OnlyBotsCanUseEndpoint , 20002),
    ("Explicit content cannot be sent to the desired recipient(s)",
    ExplicitContentSendingBlocked , 20009),
    ("You are not authorized to perform this action on this application",
    UnauthorizedApplicationAction , 20012),
    ("This action cannot be performed due to slowmode rate limit",
    SlowModeRateLimitReached , 20016),
    ("Only the owner of this account can perform this action",
    NotAccountOwner , 20018),
    ("Message cannot be edited due to announcement rate limits",
    AnnouncementRateLimitReached , 20022),
    ("The channel you are writing has hit the write rate limit",
    ChannelRateLimitReached , 20028),
    ("The write action you are performing on the server has hit the write rate limit",
    WriteActionsReached , 20029),
    ("Your Stage topic, server name, server description, or channel names contain words that are not allowed",
    UnallowedWords , 20031),
    ("Guild premium subscription level too low",
    GuildPremiumTooLow , 20035),
    ("Maximum number of guilds reached (100)",
    MaximumGuildsReached , 30001),
    ("Maximum number of friends reached (1000)",
    MaximumFriendsReached , 30002),
    ("Maximum number of pins reached for the channel (50)",
    MaximumPinsReached , 30003),
    ("Maximum number of recipients reached (10)",
    MaximumRecipientsReached , 30004),
    ("Maximum number of guild roles reached (250)",
    MaximumRolesReached , 30005),
    ("Maximum number of webhooks reached (10)",
    MaximumWebhooksReached , 30007),
    ("Maximum number of emojis reached",
    MaximumEmojisReached , 30008),
    ("Maximum number of reactions reached (20)",
    MaximumReactionsReached , 30010),
    ("Maximum number of guild channels reached (500)",
    MaximumGuildChannelsReached , 30013),
    ("Maximum number of attachments in a message reached (10)",
    MaximumAttachmentsReached , 30015),
    ("Maximum number of invites reached (1000)",
    MaximumInvitesReached , 30016),
    ("Maximum number of animated emojis reached",
    MaximumAnimatedEmojisReached , 30018),
    ("Maximum number of server members reached",
    MaximumGuildMembersReached , 30019),
    ("Maximum number of server categories has been reached",
    MaximumServerCategoriesReached , 30030),
    ("Guild already has a template",
    GuildTemplateAlreadyExist , 30031),
    ("Max number of thread participants has been reached (1000)",
    ThreadMaxParticipants , 30033),
    ("Maximum number of bans for non-guild members have been exceeded",
    MaximumNonGuildBansReached , 30035),
    ("Maximum number of bans fetches has been reached",
    MaximumGuildBansFetchesReached , 30037),
    ("Maximum number of uncompleted guild scheduled events reached (100)",
    MaximumUncompletedEventsReached , 30038),
    ("Maximum number of stickers reached",
    MaximumStickersReached , 30039),
    ("Maximum number of prune requests has been reached. Try again later",
    MaximumPruneRequestsReached , 30040),
    ("Maximum number of guild widget settings updates has been reached. Try again later",
    MaximumGuildWidgets , 30042),
    ("Unauthorized. Provide a valid token and try again",
    Unauthorized , 40001),
    ("You need to verify your account in order to perform this action",
    AccountNeedsVerification , 40002),
    ("You are opening direct messages too fast",
    OpeningDirectMessageRateLimitReached , 40003),
    ("Request entity too large. Try sending something smaller in size",
    RequestEntityTooLarge , 40005),
    ("This feature has been temporarily disabled server-side",
    FeatureTemporarilyDisabled , 40006),
    ("The user is banned from this guild",
    UserBannedFromGuild , 40007),
    ("Target user is not connected to voice",
    UserNotInVoice , 40032),
    ("This message has already been crossposted",
    MessageAlreadyCrossposted , 40033),
    ("An application command with that name already exists",
    CommandNameAlreadyExists , 40041),
    ("Missing access",
    Missingaccess , 50001),
    ("Invalid account type",
    InvalidAccountType , 50002),
    ("Cannot execute action on a DM channel",
    InvalidDMChannelAction , 50003),
    ("Guild widget disabled",
    GuildWidgetDisabled , 50004),
    ("Cannot edit a message authored by another user",
    MessageNotAuthoredByUser , 50005),
    ("Cannot send an empty message",
    EmptyMessage , 50006),
    ("Cannot send messages to this user",
    CannotSendMessageToUser , 50007),
    ("Cannot send messages in a voice channel",
    CannotSendMessagesInVoiceChannel , 50008),
    ("Channel verification level is too high for you to gain access",
    VerificationLevelTooHigh , 50009),
    ("OAuth2 application does not have a bot",
    OAuthApplicationHasNoBot , 50010),
    ("OAuth2 application limit reached",
    OAuthApplicationLimitReached , 50011),
    ("Invalid OAuth2 state",
    InvalidOAuthSstate , 50012),
    ("You lack permissions to perform that action",
    PermissionsLacking , 50013),
    ("Invalid authentication token provided",
    InvalidAuthenticationTokenProvided , 50014),
    ("Note was too long",
    NoteTooLong , 50015),
    ("Provided too few or too many messages to delete. Must provide at least 2 and fewer than 100 messages to delete",
    InvalidMessageDeleteRange , 50016),
    ("A message can only be pinned to the channel it was sent in",
    MessagePinnedInWrongChannel , 50019),
    ("Invite code was either invalid or taken",
    InviteCodeInvalidOrTaken , 50020),
    ("Cannot execute action on a system message",
    InvalidActionOnSystemMessage , 50021),
    ("Cannot execute action on this channel type",
    CannotExecuteActionOnChannelType , 50024),
    ("Invalid OAuth2 access token provided",
    InvalidOAuthAccessToken , 50025),
    ("Missing required OAuth2 scope",
    MissingOAuthScope , 50026),
    ("Invalid webhook token provided",
    InvalidWebhookToken , 50027),
    ("Invalid role",
    InvalidRole , 50028),
    ("Invalid recipient(s)",
    InvalidRecipient , 50033),
    ("A message provided was too old to bulk delete",
    MessageTooOldToBulkDelete , 50034),
    ("Invalid form body (returned for both application/json and multipart/form-data bodies), or invalid Content-Type provided",
    InvalidFormBodyOrContentType , 50035),
    ("An invite was accepted to a guild the application's bot is not in",
    InviteAcceptedToGuildBotNotIn , 50036),
    ("Invalid API version provided",
    InvalidApiVersion , 50041),
    ("File uploaded exceeds the maximum size",
    FileTooLarge , 50045),
    ("Invalid file uploaded",
    InvalidFileUploaded , 50046),
    ("Cannot self-redeem this gift",
    CannotSelfRedeemGift , 50054),
    ("Invalid Guild",
    InvalidGuild , 50055),
    ("Payment source required to redeem gift",
    PaymentRequiredForGift , 50070),
    ("Cannot delete a channel required for Community guilds",
    CommunityGuildRequired , 50074),
    ("Invalid sticker sent",
    InvalidStickerSent , 50081),
    ("Tried to perform an operation on an archived thread, such as editing a message or adding a user to the thread",
    ThreadArchived , 50083),
    ("Invalid thread notification settings",
    ThreadInvalidNotificationSettings , 50084),
    ("`before` value is earlier than the thread creation date",
    ThreadInvalidBeforeValue , 50085),
    ("This server is not available in your location",
    ServerNotAvailableLocation , 50095),
    ("This server needs monetization enabled in order to perform this action",
    ServerNeedsMonetiazation , 50097),
    ("This server needs more boosts to perform this action",
    ServerNeedsBoosts , 50101),
    ("The request body contains invalid JSON",
    RequestInvalidJson , 50109),
    ("Two factor is required for this operation",
    TwoFactorRequired , 60003),
    ("No users with DiscordTag exist",
    NoSuchUser , 80004),
    ("Reaction was blocked",
    ReactionBlocked , 90001),
    ("API resource is currently overloaded. Try again a little later",
    ApiResourceOverloaded , 130_000),
    ("The Stage is already open",
    StageAlreadyOpen , 150_006),
    ("Cannot reply without permission to read message history",
    CannotReplyWithoutMessageHistory , 160_002),
    ("A thread has already been created for this message",
    ThreadAlreadyCreated , 160_004),
    ("Thread is locked",
    ThreadLocked , 160_005),
    ("Maximum number of active threads reached",
    MaxActiveThreads , 160_006),
    ("Maximum number of active announcement threads reached",
    MaxActiveAnnouncementThreads , 160_007),
    ("Invalid JSON for uploaded Lottie file",
    InvalidLottieJson , 170_001),
    ("Uploaded Lotties cannot contain rasterized images such as PNG or JPEG",
    InvalidLottieContent , 170_002),
    ("Sticker maximum framerate exceeded",
    StickerMaximumFramerateExceeded , 170_003),
    ("Sticker frame count exceeds maximum of 1000 frames",
    StickerFrameCountExceedsMaximum , 170_004),
    ("Lottie animation maximum dimensions exceeded",
    LottieDimensionsTooLarge , 170_005),
    ("Sticker frame rate is either too small or too large",
    InvalidStickerFrameRate , 170_006),
    ("Sticker animation duration exceeds maximum of 5 seconds",
    StickerAnimationDurationExceedsMaximum , 170_007),
    ("Cannot update a finished event",
    CannotUpdateFinishedEvent , 180_000),
    ("Failed to create stage needed for stage event",
    FailedToCreateStage , 180_002)
];

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
    /// Request has been ratelimited.
    Ratelimited(RatelimitedApiError),
    /// Something was wrong with the input when sending a message.
    Message(MessageApiError),
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
        f.write_str("Error code ")?;
        Display::fmt(&self.code.num(), f)?;
        f.write_str(": ")?;

        f.write_str(&self.message)
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

        f.write_str("ratelimited for ")?;
        Display::fmt(&self.retry_after, f)?;

        f.write_str("s")
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
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ErrorCode: Clone, Copy, Debug, Eq, PartialEq, Send, Sync);

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
    fn test_ratelimited_api_error() {
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

    /// Assert that deserializing an [`ApiError::Ratelimited`] variant uses
    /// the correct variant.
    ///
    /// Tests for [#1302], which was due to a previously ordered variant having
    /// higher priority for untagged deserialization.
    ///
    /// [#1302]: https://github.com/twilight-rs/twilight/issues/1302
    #[test]
    fn test_api_error_variant_ratelimited() {
        let expected = ApiError::Ratelimited(RatelimitedApiError {
            global: false,
            message: "You are being rate limited.".to_owned(),
            retry_after: 0.362,
        });

        serde_test::assert_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "RatelimitedApiError",
                    len: 3,
                },
                Token::Str("global"),
                Token::Bool(false),
                Token::Str("message"),
                Token::Str("You are being rate limited."),
                Token::Str("retry_after"),
                Token::F64(0.362),
                Token::StructEnd,
            ],
        );
    }
}
