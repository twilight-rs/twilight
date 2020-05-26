use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType, GuildChannel},
    id::{ChannelId, GuildId},
};

#[derive(Clone, Debug)]
pub enum CreateGuildChannelError {
    /// The length of the name is either fewer than 2 UTF-16 characters or
    /// more than 100 UTF-16 characters.
    NameInvalid,
    /// The seconds of the rate limit per user is more than 21600.
    RateLimitPerUserInvalid,
    /// The length of the topic is more than 1024 UTF-16 characters.
    TopicInvalid,
}

impl Display for CreateGuildChannelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid => f.write_str("the length of the name is invalid"),
            Self::RateLimitPerUserInvalid => f.write_str("the rate limit per user is invalid"),
            Self::TopicInvalid => f.write_str("the topic is invalid"),
        }
    }
}

impl Error for CreateGuildChannelError {}

#[derive(Serialize)]
struct CreateGuildChannelFields {
    bitrate: Option<u64>,
    #[serde(rename = "type")]
    kind: Option<ChannelType>,
    name: String,
    nsfw: Option<bool>,
    parent_id: Option<ChannelId>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    position: Option<u64>,
    rate_limit_per_user: Option<u64>,
    topic: Option<String>,
    user_limit: Option<u64>,
}

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
            return Err(CreateGuildChannelError::NameInvalid);
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

    pub fn bitrate(mut self, bitrate: u64) -> Self {
        self.fields.bitrate.replace(bitrate);

        self
    }

    pub fn kind(mut self, kind: ChannelType) -> Self {
        self.fields.kind.replace(kind);

        self
    }

    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.fields.nsfw.replace(nsfw);

        self
    }

    pub fn parent_id(mut self, parent_id: impl Into<ChannelId>) -> Self {
        self.fields.parent_id.replace(parent_id.into());

        self
    }

    pub fn permission_overwrites(
        mut self,
        permission_overwrites: Vec<PermissionOverwrite>,
    ) -> Self {
        self.fields
            .permission_overwrites
            .replace(permission_overwrites);

        self
    }

    pub fn position(mut self, position: u64) -> Self {
        self.fields.position.replace(position);

        self
    }

    /// Set the number of seconds that a user must wait before before able to
    /// send a message again.
    ///
    /// The minimum is 0 and the maximum is 21600.
    ///
    /// # Errors
    ///
    /// Returns [`GetGuildPruneCountError::RateLimitPerUserInvalid`] if the
    /// amount is greater than 21600.
    ///
    /// [`GetGuildPruneCountError::RateLimitPerUserInvalid`]: enum.GetGuildPruneCountError.html#variant.RateLimitPerUserInvalid
    pub fn rate_limit_per_user(
        mut self,
        rate_limit_per_user: u64,
    ) -> Result<Self, CreateGuildChannelError> {
        // <https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure>
        if rate_limit_per_user > 21600 {
            return Err(CreateGuildChannelError::RateLimitPerUserInvalid);
        }

        self.fields.rate_limit_per_user.replace(rate_limit_per_user);

        Ok(self)
    }

    /// Set the topic.
    ///
    /// The maximum length is 1024 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildChannel::TopicInvalid`] if the topic length is
    /// too long.
    ///
    /// [`CreateGuildChannel::TopicInvalid`]: enum.CreateGuildChannel.html#variant.TopicInvalid
    pub fn topic(self, topic: impl Into<String>) -> Result<Self, CreateGuildChannelError> {
        self._topic(topic.into())
    }

    fn _topic(mut self, topic: String) -> Result<Self, CreateGuildChannelError> {
        // <https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure>
        if topic.chars().count() > 1024 {
            return Err(CreateGuildChannelError::TopicInvalid);
        }

        self.fields.topic.replace(topic);

        Ok(self)
    }

    pub fn user_limit(mut self, user_limit: u64) -> Self {
        self.fields.user_limit.replace(user_limit);

        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                serde_json::to_vec(&self.fields)?,
                headers,
                Route::CreateChannel {
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from((
                serde_json::to_vec(&self.fields)?,
                Route::CreateChannel {
                    guild_id: self.guild_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateGuildChannel<'_>, GuildChannel);
