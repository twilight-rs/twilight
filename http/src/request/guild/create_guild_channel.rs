use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType, GuildChannel},
    id::{ChannelId, GuildId},
};

/// Returned when the channel can not be created as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CreateGuildChannelError {
    /// The length of the name is either fewer than 2 UTF-16 characters or
    /// more than 100 UTF-16 characters.
    NameInvalid {
        /// Provided name.
        name: String,
    },
    /// The seconds of the rate limit per user is more than 21600.
    RateLimitPerUserInvalid {
        /// Provided ratelimit.
        rate_limit_per_user: u64,
    },
    /// The length of the topic is more than 1024 UTF-16 characters.
    TopicInvalid {
        /// Provided topic.
        topic: String,
    },
}

impl Display for CreateGuildChannelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid { .. } => f.write_str("the length of the name is invalid"),
            Self::RateLimitPerUserInvalid { .. } => {
                f.write_str("the rate limit per user is invalid")
            }
            Self::TopicInvalid { .. } => f.write_str("the topic is invalid"),
        }
    }
}

impl Error for CreateGuildChannelError {}

#[derive(Serialize)]
struct CreateGuildChannelFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<u64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    kind: Option<ChannelType>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_limit: Option<u64>,
}

/// Create a new request to create a guild channel.
///
/// All fields are optional except for name. The minimum length of the name is 2 UTF-16 characters
/// and the maximum is 100 UTF-16 characters.
///
/// # Errors
///
/// Returns a [`CreateGuildChannelError::NameInvalid`] when the length of the name is either fewer
/// than 2 UTF-16 characters or more than 100 UTF-16 characters.
///
/// Returns a [`CreateGuildChannelError::RateLimitPerUserInvalid`] when the seconds of the rate
/// limit per user is more than 21600.
///
/// Returns a [`CreateGuildChannelError::TopicInvalid`] when the length of the topic is more than
/// 1024 UTF-16 characters.
pub struct CreateGuildChannel<'a> {
    fields: CreateGuildChannelFields,
    fut: Option<Pending<'a, GuildChannel>>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> CreateGuildChannel<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        name: impl Into<String>,
    ) -> Result<Self, CreateGuildChannelError> {
        Self::_new(http, guild_id, name.into())
    }

    fn _new(
        http: &'a Client,
        guild_id: GuildId,
        name: String,
    ) -> Result<Self, CreateGuildChannelError> {
        if !validate::channel_name(&name) {
            return Err(CreateGuildChannelError::NameInvalid { name });
        }

        Ok(Self {
            fields: CreateGuildChannelFields {
                bitrate: None,
                kind: None,
                name,
                nsfw: None,
                parent_id: None,
                permission_overwrites: None,
                position: None,
                rate_limit_per_user: None,
                topic: None,
                user_limit: None,
            },
            fut: None,
            guild_id,
            http,
            reason: None,
        })
    }

    /// Set the bitrate of the channel. Applicable to voice channels only.
    pub fn bitrate(mut self, bitrate: u64) -> Self {
        self.fields.bitrate.replace(bitrate);

        self
    }

    /// Set the kind of channel.
    pub fn kind(mut self, kind: ChannelType) -> Self {
        self.fields.kind.replace(kind);

        self
    }

    /// Set whether the channel is marked as NSFW.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.fields.nsfw.replace(nsfw);

        self
    }

    /// If this is specified, and the parent ID is a `ChannelType::CategoryChannel`, create this
    /// channel as a child of the category channel.
    pub fn parent_id(mut self, parent_id: impl Into<ChannelId>) -> Self {
        self.fields.parent_id.replace(parent_id.into());

        self
    }

    /// Set the permission overwrites of a channel.
    pub fn permission_overwrites(
        mut self,
        permission_overwrites: Vec<PermissionOverwrite>,
    ) -> Self {
        self.fields
            .permission_overwrites
            .replace(permission_overwrites);

        self
    }

    /// Set the position of the channel.
    ///
    /// Positions are numerical and zero-indexed. If you place a channel at position 2, channels
    /// 2-n will shift down one position and the initial channel will take its place.
    pub fn position(mut self, position: u64) -> Self {
        self.fields.position.replace(position);

        self
    }

    /// Set the number of seconds that a user must wait before before they are able to send another
    /// message.
    ///
    /// The minimum is 0 and the maximum is 21600. Refer to [the discord docs] for more details.
    /// This is also known as "Slow Mode".
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildChannelError::RateLimitPerUserInvalid`] if the amount is greater than
    /// 21600.
    ///
    /// [the discord docs]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure
    pub fn rate_limit_per_user(
        mut self,
        rate_limit_per_user: u64,
    ) -> Result<Self, CreateGuildChannelError> {
        if rate_limit_per_user > 21600 {
            return Err(CreateGuildChannelError::RateLimitPerUserInvalid {
                rate_limit_per_user,
            });
        }

        self.fields.rate_limit_per_user.replace(rate_limit_per_user);

        Ok(self)
    }

    /// Set the topic.
    ///
    /// The maximum length is 1024 UTF-16 characters. Refer to [the discord docs] for more details.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildChannelError::TopicInvalid`] if the topic length is
    /// too long.
    ///
    /// [the discord docs]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure
    pub fn topic(self, topic: impl Into<String>) -> Result<Self, CreateGuildChannelError> {
        self._topic(topic.into())
    }

    fn _topic(mut self, topic: String) -> Result<Self, CreateGuildChannelError> {
        if topic.chars().count() > 1024 {
            return Err(CreateGuildChannelError::TopicInvalid { topic });
        }

        self.fields.topic.replace(topic);

        Ok(self)
    }

    /// For voice channels, set the user limit.
    ///
    /// Set to 0 for no limit. Limit can otherwise be between 1 and 99 inclusive. Refer to [the
    /// discord docs] for more details.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#modify-channel-json-params
    pub fn user_limit(mut self, user_limit: u64) -> Self {
        self.fields.user_limit.replace(user_limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                crate::json_to_vec(&self.fields)?,
                headers,
                Route::CreateChannel {
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from((
                crate::json_to_vec(&self.fields)?,
                Route::CreateChannel {
                    guild_id: self.guild_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for CreateGuildChannel<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(CreateGuildChannel<'_>, GuildChannel);
