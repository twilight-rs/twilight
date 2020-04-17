use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, Channel, ChannelType},
    id::ChannelId,
};

#[derive(Clone, Debug)]
pub enum UpdateChannelError {
    /// The length of the name is either fewer than 2 UTF-8 characters or
    /// more than 100 UTF-8 characters.
    NameInvalid,
    /// The seconds of the rate limit per user is more than 21600.
    RateLimitPerUserInvalid,
    /// The length of the topic is more than 1024 UTF-8 characters.
    TopicInvalid,
}

impl Display for UpdateChannelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid => f.write_str("the length of the name is invalid"),
            Self::RateLimitPerUserInvalid => f.write_str("the rate limit per user is invalid"),
            Self::TopicInvalid => f.write_str("the topic is invalid"),
        }
    }
}

impl Error for UpdateChannelError {}

#[derive(Default, Serialize)]
struct UpdateChannelFields {
    bitrate: Option<u64>,
    name: Option<String>,
    nsfw: Option<bool>,
    parent_id: Option<ChannelId>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    position: Option<u64>,
    rate_limit_per_user: Option<u64>,
    topic: Option<String>,
    user_limit: Option<u64>,
    #[serde(rename = "type")]
    kind: Option<ChannelType>,
}

pub struct UpdateChannel<'a> {
    channel_id: ChannelId,
    fields: UpdateChannelFields,
    fut: Option<Pending<'a, Channel>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> UpdateChannel<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: UpdateChannelFields::default(),
            fut: None,
            http,
            reason: None,
        }
    }

    pub fn bitrate(mut self, bitrate: u64) -> Self {
        self.fields.bitrate.replace(bitrate);

        self
    }

    /// Set the name.
    ///
    /// The minimum length is 2 UTF-8 characters and the maximum is 100 UTF-8
    /// characters.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateChannelError::NameInvalid`] if the name length is
    /// too short or too long.
    ///
    /// [`UpdateChannelError::NameInvalid`]: enum.UpdateChannelError.html#variant.NameInvalid
    pub fn name(self, name: impl Into<String>) -> Result<Self, UpdateChannelError> {
        self._name(name.into())
    }

    fn _name(mut self, name: String) -> Result<Self, UpdateChannelError> {
        if !validate::channel_name(&name) {
            return Err(UpdateChannelError::NameInvalid);
        }

        self.fields.name.replace(name);

        Ok(self)
    }

    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.fields.nsfw.replace(nsfw);

        self
    }

    pub fn parent_id(mut self, parent_id: ChannelId) -> Self {
        self.fields.parent_id.replace(parent_id);

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
    /// Returns [`UpdateChannelError::RateLimitPerUserInvalid`] if the
    /// amount is greater than 21600.
    ///
    /// [`UpdateChannelError::RateLimitPerUserInvalid`]: enum.UpdateChannelError.html#variant.RateLimitPerUserInvalid
    pub fn rate_limit_per_user(
        mut self,
        rate_limit_per_user: u64,
    ) -> Result<Self, UpdateChannelError> {
        // <https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure>
        if rate_limit_per_user > 21600 {
            return Err(UpdateChannelError::RateLimitPerUserInvalid);
        }

        self.fields.rate_limit_per_user.replace(rate_limit_per_user);

        Ok(self)
    }

    /// Set the topic.
    ///
    /// The maximum length is 1024 UTF-8 characters.
    ///
    /// # Errors
    ///
    /// Returns [`CreateGuildChannel::TopicInvalid`] if the topic length is
    /// too long.
    ///
    /// [`CreateGuildChannel::TopicInvalid`]: enum.CreateGuildChannel.html#variant.TopicInvalid
    pub fn topic(self, topic: impl Into<String>) -> Result<Self, UpdateChannelError> {
        self._topic(topic.into())
    }

    fn _topic(mut self, topic: String) -> Result<Self, UpdateChannelError> {
        // <https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure>
        if topic.chars().count() > 1024 {
            return Err(UpdateChannelError::TopicInvalid);
        }

        self.fields.topic.replace(topic);

        Ok(self)
    }

    pub fn user_limit(mut self, user_limit: u64) -> Self {
        self.fields.user_limit.replace(user_limit);

        self
    }

    pub fn kind(mut self, kind: ChannelType) -> Self {
        self.fields.kind.replace(kind);

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
                Route::UpdateChannel {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from((
                serde_json::to_vec(&self.fields)?,
                Route::UpdateChannel {
                    channel_id: self.channel_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(UpdateChannel<'_>, Channel);
