//! Request parameters for ratelimiting.
//!
//! This module contains the type definitions for parameters
//! relevant for ratelimiting.
//!
//! The [`super::Ratelimiter`] uses [`Path`]s and [`Method`]s to store
//! and associate buckets with routes.

use http::Method as HttpMethod;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

/// Request method.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Method {
    /// DELETE method.
    Delete,
    /// GET method.
    Get,
    /// PATCH method.
    Patch,
    /// POST method.
    Post,
    /// PUT method.
    Put,
}

impl Method {
    /// Convert the method into the equivalent [`http::Method`].
    #[must_use]
    pub const fn to_http(self) -> HttpMethod {
        match self {
            Self::Delete => HttpMethod::DELETE,
            Self::Get => HttpMethod::GET,
            Self::Patch => HttpMethod::PATCH,
            Self::Post => HttpMethod::POST,
            Self::Put => HttpMethod::PUT,
        }
    }
}

/// Error returned when a [`Path`] could not be parsed from a string.
#[derive(Debug)]
pub struct PathParseError {
    /// Detailed reason why this could not be parsed.
    kind: PathParseErrorType,
    /// Original error leading up to this one.
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl PathParseError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &PathParseErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (PathParseErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for PathParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            PathParseErrorType::IntegerParsing { .. } => {
                f.write_str("An ID in a segment was invalid")
            }
            PathParseErrorType::MessageIdWithoutMethod { .. } => {
                f.write_str("A message path was detected but the method wasn't given")
            }
            PathParseErrorType::NoMatch => f.write_str("There was no matched path"),
        }
    }
}

impl Error for PathParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`PathParseError`] specifying what failed to parse.
#[derive(Debug)]
#[non_exhaustive]
pub enum PathParseErrorType {
    /// The ID couldn't be parsed as an integer.
    IntegerParsing,
    /// When parsing into a [`Path::ChannelsIdMessagesId`] variant, the method
    /// must also be specified via its `TryFrom` impl.
    MessageIdWithoutMethod {
        /// The ID of the channel.
        channel_id: u64,
    },
    /// A static path for the provided path string wasn't found.
    NoMatch,
}

/// An enum representing a path, most useful for ratelimiting implementations.
// If adding to this enum, be sure to add to the `TryFrom` impl.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Path {
    /// Operating on global commands.
    ApplicationCommand(u64),
    /// Operating on a specific command.
    ApplicationCommandId(u64),
    /// Operating on commands in a guild.
    ApplicationGuildCommand(u64),
    /// Operating on a specific command in a guild.
    ApplicationGuildCommandId(u64),
    /// Operating on a channel.
    ChannelsId(u64),
    /// Operating on a channel's invites.
    ChannelsIdInvites(u64),
    /// Operating on a channel's messages.
    ChannelsIdMessages(u64),
    /// Operating on a channel's messages by bulk deleting.
    ChannelsIdMessagesBulkDelete(u64),
    /// Operating on an individual channel's message.
    ChannelsIdMessagesId(Method, u64),
    /// Crossposting an individual channel's message.
    ChannelsIdMessagesIdCrosspost(u64),
    /// Operating on an individual channel's message's reactions.
    ChannelsIdMessagesIdReactions(u64),
    /// Operating on an individual channel's message's reactions while
    /// specifying the user ID and emoji type.
    ChannelsIdMessagesIdReactionsUserIdType(u64),
    /// Operating on an individual channel's message's threads.
    ChannelsIdMessagesIdThreads(u64),
    /// Operating on a channel's permission overwrites by ID.
    ChannelsIdPermissionsOverwriteId(u64),
    /// Operating on a channel's pins.
    ChannelsIdPins(u64),
    /// Operating on a channel's individual pinned message.
    ChannelsIdPinsMessageId(u64),
    /// Operating on a group DM's recipients.
    ChannelsIdRecipients(u64),
    /// Operating on a thread's members.
    ChannelsIdThreadMembers(u64),
    /// Operating on a channel's threads.
    ChannelsIdThreads(u64),
    /// Operating on a channel's typing indicator.
    ChannelsIdTyping(u64),
    /// Operating on a channel's webhooks.
    ChannelsIdWebhooks(u64),
    /// Operating on a channel's followers.
    ChannelsIdFollowers(u64),
    /// Operating with the gateway information.
    Gateway,
    /// Operating with the gateway information tailored to the current user.
    GatewayBot,
    /// Operating on the guild resource.
    Guilds,
    /// Operating on one of user's guilds.
    GuildsId(u64),
    /// Operating on one of the user's guilds' bans.
    GuildsIdBans(u64),
    /// Operating on a ban from one of the user's guilds.
    GuildsIdBansId(u64),
    /// Operating on a ban from one of the user's guilds.
    GuildsIdAuditLogs(u64),
    /// Operating on specific member's ban from one of the user's guilds.
    GuildsIdBansUserId(u64),
    /// Operating on one of the user's guilds' channels.
    GuildsIdChannels(u64),
    /// Operating on one of the user's guilds' widget.
    GuildsIdWidget(u64),
    /// Operating on one of the user's guilds' emojis.
    GuildsIdEmojis(u64),
    /// Operating on an emoji from one of the user's guilds.
    GuildsIdEmojisId(u64),
    /// Operating on one of the user's guilds' integrations.
    GuildsIdIntegrations(u64),
    /// Operating on an integration from one of the user's guilds.
    GuildsIdIntegrationsId(u64),
    /// Operating on an integration from one of the user's guilds by synchronizing it.
    GuildsIdIntegrationsIdSync(u64),
    /// Operating on one of the user's guilds' invites.
    GuildsIdInvites(u64),
    /// Operating on one of the user's guilds' members.
    GuildsIdMembers(u64),
    /// Operating on a member from one of the user's guilds.
    GuildsIdMembersId(u64),
    /// Operating on a role of a member from one of the user's guilds.
    GuildsIdMembersIdRolesId(u64),
    /// Operating on the user's nickname in one of the user's guilds.
    GuildsIdMembersMeNick(u64),
    /// Operating on one of the user's guilds' members by searching.
    GuildsIdMembersSearch(u64),
    /// Operating on one of the user's guilds' MFA level.
    GuildsIdMfa(u64),
    /// Operating on one of the user's guilds' by previewing it.
    GuildsIdPreview(u64),
    /// Operating on one of the user's guilds' by pruning members.
    GuildsIdPrune(u64),
    /// Operating on the voice regions of one of the user's guilds.
    GuildsIdRegions(u64),
    /// Operating on the roles of one of the user's guilds.
    GuildsIdRoles(u64),
    /// Operating on a role of one of the user's guilds.
    GuildsIdRolesId(u64),
    /// Operating on the guild's scheduled events.
    GuildsIdScheduledEvents(u64),
    /// Operating on a particular guild's scheduled events.
    GuildsIdScheduledEventsId(u64),
    /// Operating on a particular guild's scheduled event users.
    GuildsIdScheduledEventsIdUsers(u64),
    /// Operating on one of the user's guilds' stickers.
    GuildsIdStickers(u64),
    /// Operating on one of the user's guilds' templates.
    GuildsIdTemplates(u64),
    /// Operating on a guild template.
    GuildsTemplatesCode(String),
    /// Operating on a template from one of the user's guilds.
    GuildsIdTemplatesCode(u64, String),
    /// Operating on one of the user's guilds' threads.
    GuildsIdThreads(u64),
    /// Operating on one of the user's guilds' vanity URL.
    GuildsIdVanityUrl(u64),
    /// Operating on one of the user's guilds' voice states.
    GuildsIdVoiceStates(u64),
    /// Operating on one of the user's guilds' welcome screen.
    GuildsIdWelcomeScreen(u64),
    /// Operating on one of the user's guilds' webhooks.
    GuildsIdWebhooks(u64),
    /// Operating on an invite.
    InvitesCode,
    /// Operating on an interaction's callback.
    InteractionCallback(u64),
    /// Operating on stage instances.
    StageInstances,
    /// Operating on sticker packs.
    StickerPacks,
    /// Operating on a sticker.
    Stickers,
    /// Operating on a sticker.
    UsersId,
    /// Operating on the user's application information.
    OauthApplicationsMe,
    /// Operating on the user's connections.
    UsersIdConnections,
    /// Operating on the user's private channels.
    UsersIdChannels,
    /// Operating on the state of a guild that the user is in.
    UsersIdGuilds,
    /// Operating on the state of a guild that the user is in.
    UsersIdGuildsId,
    /// Operating on the state of a guild that the user, as a member, is in.
    UsersIdGuildsIdMember,
    /// Operating on the voice regions available to the current user.
    VoiceRegions,
    /// Operating on a webhook as a bot.
    WebhooksId(u64),
    /// Operating on a webhook as a webhook.
    WebhooksIdToken(u64, String),
    /// Operating on a message created by a webhook.
    WebhooksIdTokenMessagesId(u64, String),
}

impl FromStr for Path {
    type Err = PathParseError;

    /// Parses a string into a path.
    ///
    /// The string *may* start with a slash (`/`), which will be ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use twilight_http_ratelimiting::Path;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// assert_eq!(Path::VoiceRegions, Path::from_str("/voice/regions")?);
    /// assert_eq!(
    ///     Path::ChannelsIdMessages(123),
    ///     Path::from_str("channels/123/messages")?,
    /// );
    /// # Ok(()) }
    /// ```
    #[allow(clippy::enum_glob_use, clippy::too_many_lines)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Path::*;

        /// Parse a string into a Discord ID.
        fn parse_id(id: &str) -> Result<u64, PathParseError> {
            id.parse().map_err(|source| PathParseError {
                kind: PathParseErrorType::IntegerParsing,
                source: Some(Box::new(source)),
            })
        }

        let skip = usize::from(s.starts_with('/'));

        let parts = s.split('/').skip(skip).collect::<Vec<&str>>();

        Ok(match parts[..] {
            ["applications", id, "commands"] => ApplicationCommand(parse_id(id)?),
            ["applications", id, "commands", _] => ApplicationCommandId(parse_id(id)?),
            ["applications", id, "guilds", _, "commands"]
            | ["applications", id, "guilds", _, "commands", "permissions"] => {
                ApplicationGuildCommand(parse_id(id)?)
            }
            ["applications", id, "guilds", _, "commands", _]
            | ["applications", id, "guilds", _, "commands", _, "permissions"] => {
                ApplicationGuildCommandId(parse_id(id)?)
            }
            ["channels", id] => ChannelsId(parse_id(id)?),
            ["channels", id, "followers"] => ChannelsIdFollowers(parse_id(id)?),
            ["channels", id, "invites"] => ChannelsIdInvites(parse_id(id)?),
            ["channels", id, "messages"] => ChannelsIdMessages(parse_id(id)?),
            ["channels", id, "messages", "bulk-delete"] => {
                ChannelsIdMessagesBulkDelete(parse_id(id)?)
            }
            ["channels", id, "messages", _] => {
                // can not map to path without method since they have different ratelimits
                return Err(PathParseError {
                    kind: PathParseErrorType::MessageIdWithoutMethod {
                        channel_id: parse_id(id)?,
                    },
                    source: None,
                });
            }
            ["channels", id, "messages", _, "crosspost"] => {
                ChannelsIdMessagesIdCrosspost(parse_id(id)?)
            }
            ["channels", id, "messages", _, "reactions"]
            | ["channels", id, "messages", _, "reactions", _] => {
                ChannelsIdMessagesIdReactions(parse_id(id)?)
            }
            ["channels", id, "messages", _, "reactions", _, _] => {
                ChannelsIdMessagesIdReactionsUserIdType(parse_id(id)?)
            }
            ["channels", id, "messages", _, "threads"] => {
                ChannelsIdMessagesIdThreads(parse_id(id)?)
            }
            ["channels", id, "permissions", _] => ChannelsIdPermissionsOverwriteId(parse_id(id)?),
            ["channels", id, "pins"] => ChannelsIdPins(parse_id(id)?),
            ["channels", id, "pins", _] => ChannelsIdPinsMessageId(parse_id(id)?),
            ["channels", id, "recipients"] | ["channels", id, "recipients", _] => {
                ChannelsIdRecipients(parse_id(id)?)
            }
            ["channels", id, "thread-members"] => ChannelsIdThreadMembers(parse_id(id)?),
            ["channels", id, "threads"] => ChannelsIdThreads(parse_id(id)?),
            ["channels", id, "typing"] => ChannelsIdTyping(parse_id(id)?),
            ["channels", id, "webhooks"] | ["channels", id, "webhooks", _] => {
                ChannelsIdWebhooks(parse_id(id)?)
            }
            ["gateway"] => Gateway,
            ["gateway", "bot"] => GatewayBot,
            ["guilds"] => Guilds,
            ["guilds", "templates", code] => GuildsTemplatesCode(code.to_string()),
            ["guilds", id] => GuildsId(parse_id(id)?),
            ["guilds", id, "audit-logs"] => GuildsIdAuditLogs(parse_id(id)?),
            ["guilds", id, "bans"] => GuildsIdBans(parse_id(id)?),
            ["guilds", id, "bans", _] => GuildsIdBansUserId(parse_id(id)?),
            ["guilds", id, "channels"] => GuildsIdChannels(parse_id(id)?),
            ["guilds", id, "widget" | "widget.json"] => GuildsIdWidget(parse_id(id)?),
            ["guilds", id, "emojis"] => GuildsIdEmojis(parse_id(id)?),
            ["guilds", id, "emojis", _] => GuildsIdEmojisId(parse_id(id)?),
            ["guilds", id, "integrations"] => GuildsIdIntegrations(parse_id(id)?),
            ["guilds", id, "integrations", _] => GuildsIdIntegrationsId(parse_id(id)?),
            ["guilds", id, "integrations", _, "sync"] => GuildsIdIntegrationsIdSync(parse_id(id)?),
            ["guilds", id, "invites"] => GuildsIdInvites(parse_id(id)?),
            ["guilds", id, "members"] => GuildsIdMembers(parse_id(id)?),
            ["guilds", id, "members", "search"] => GuildsIdMembersSearch(parse_id(id)?),
            ["guilds", id, "members", _] => GuildsIdMembersId(parse_id(id)?),
            ["guilds", id, "members", _, "roles", _] => GuildsIdMembersIdRolesId(parse_id(id)?),
            ["guilds", id, "members", "@me", "nick"] => GuildsIdMembersMeNick(parse_id(id)?),
            ["guilds", id, "preview"] => GuildsIdPreview(parse_id(id)?),
            ["guilds", id, "prune"] => GuildsIdPrune(parse_id(id)?),
            ["guilds", id, "regions"] => GuildsIdRegions(parse_id(id)?),
            ["guilds", id, "roles"] => GuildsIdRoles(parse_id(id)?),
            ["guilds", id, "roles", _] => GuildsIdRolesId(parse_id(id)?),
            ["guilds", id, "scheduled-events"] => GuildsIdScheduledEvents(parse_id(id)?),
            ["guilds", id, "scheduled-events", _] => GuildsIdScheduledEventsId(parse_id(id)?),
            ["guilds", id, "scheduled-events", _, "users"] => {
                GuildsIdScheduledEventsIdUsers(parse_id(id)?)
            }
            ["guilds", id, "stickers"] | ["guilds", id, "stickers", _] => {
                GuildsIdStickers(parse_id(id)?)
            }
            ["guilds", id, "templates"] => GuildsIdTemplates(parse_id(id)?),
            ["guilds", id, "templates", code] => {
                GuildsIdTemplatesCode(parse_id(id)?, code.to_string())
            }
            ["guilds", id, "threads", _] => GuildsIdThreads(parse_id(id)?),
            ["guilds", id, "vanity-url"] => GuildsIdVanityUrl(parse_id(id)?),
            ["guilds", id, "voice-states", _] => GuildsIdVoiceStates(parse_id(id)?),
            ["guilds", id, "welcome-screen"] => GuildsIdWelcomeScreen(parse_id(id)?),
            ["guilds", id, "webhooks"] => GuildsIdWebhooks(parse_id(id)?),
            ["invites", _] => InvitesCode,
            ["interactions", id, _, "callback"] => InteractionCallback(parse_id(id)?),
            ["stage-instances", _] => StageInstances,
            ["sticker-packs"] => StickerPacks,
            ["stickers", _] => Stickers,
            ["oauth2", "applications", "@me"] => OauthApplicationsMe,
            ["users", _] => UsersId,
            ["users", _, "connections"] => UsersIdConnections,
            ["users", _, "channels"] => UsersIdChannels,
            ["users", _, "guilds"] => UsersIdGuilds,
            ["users", _, "guilds", _] => UsersIdGuildsId,
            ["users", _, "guilds", _, "member"] => UsersIdGuildsIdMember,
            ["voice", "regions"] => VoiceRegions,
            ["webhooks", id] => WebhooksId(parse_id(id)?),
            ["webhooks", id, token] => WebhooksIdToken(parse_id(id)?, token.to_string()),
            ["webhooks", id, token, "messages", _] => {
                WebhooksIdTokenMessagesId(parse_id(id)?, token.to_string())
            }
            _ => {
                return Err(PathParseError {
                    kind: PathParseErrorType::NoMatch,
                    source: None,
                })
            }
        })
    }
}

impl TryFrom<(Method, &str)> for Path {
    type Error = PathParseError;

    fn try_from((method, s): (Method, &str)) -> Result<Self, Self::Error> {
        match Self::from_str(s) {
            Ok(v) => Ok(v),
            Err(why) => {
                if let PathParseErrorType::MessageIdWithoutMethod { channel_id } = why.kind() {
                    Ok(Self::ChannelsIdMessagesId(method, *channel_id))
                } else {
                    Err(why)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Path, PathParseError, PathParseErrorType};
    use crate::request::Method;
    use http::Method as HttpMethod;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug, hash::Hash, str::FromStr};

    assert_fields!(PathParseErrorType::MessageIdWithoutMethod: channel_id);
    assert_impl_all!(PathParseErrorType: Debug, Send, Sync);
    assert_impl_all!(PathParseError: Error, Send, Sync);
    assert_impl_all!(Path: Clone, Debug, Eq, Hash, PartialEq, Send, Sync);

    #[test]
    fn prefix_unimportant() -> Result<(), Box<dyn Error>> {
        assert_eq!(Path::Guilds, Path::from_str("guilds")?);
        assert_eq!(Path::Guilds, Path::from_str("/guilds")?);

        Ok(())
    }

    #[test]
    fn from_str() -> Result<(), Box<dyn Error>> {
        assert_eq!(Path::ChannelsId(123), Path::from_str("/channels/123")?);
        assert_eq!(Path::WebhooksId(123), Path::from_str("/webhooks/123")?);
        assert_eq!(Path::InvitesCode, Path::from_str("/invites/abc")?);

        Ok(())
    }

    #[test]
    fn message_id() -> Result<(), Box<dyn Error>> {
        assert!(matches!(
            Path::from_str("channels/123/messages/456")
                .unwrap_err()
                .kind(),
            PathParseErrorType::MessageIdWithoutMethod { channel_id: 123 },
        ));
        assert_eq!(
            Path::ChannelsIdMessagesId(Method::Get, 123),
            Path::try_from((Method::Get, "/channels/123/messages/456"))?,
        );

        Ok(())
    }

    assert_impl_all!(Method: Clone, Copy, Debug, Eq, PartialEq);

    #[test]
    fn method_conversions() {
        assert_eq!(HttpMethod::DELETE, Method::Delete.to_http());
        assert_eq!(HttpMethod::GET, Method::Get.to_http());
        assert_eq!(HttpMethod::PATCH, Method::Patch.to_http());
        assert_eq!(HttpMethod::POST, Method::Post.to_http());
        assert_eq!(HttpMethod::PUT, Method::Put.to_http());
    }
}
