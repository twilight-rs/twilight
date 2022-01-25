use crate::{
    client::Client,
    error::Error as HttpError,
    request::{self, AuditLogReason, AuditLogReasonError, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType, GuildChannel},
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};
use twilight_validate::channel::{
    name as validate_name, rate_limit_per_user as validate_rate_limit_per_user,
    topic as validate_topic, ChannelValidationError,
};

#[derive(Serialize)]
struct CreateGuildChannelFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<u64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    kind: Option<ChannelType>,
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permission_overwrites: Option<&'a [PermissionOverwrite]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_limit: Option<u64>,
}

/// Create a new request to create a guild channel.
///
/// All fields are optional except for name. The minimum length of the name is 1
/// UTF-16 characters and the maximum is 100 UTF-16 characters.
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildChannel<'a> {
    fields: CreateGuildChannelFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> CreateGuildChannel<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        name: &'a str,
    ) -> Result<Self, ChannelValidationError> {
        validate_name(name)?;

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
            guild_id,
            http,
            reason: None,
        })
    }

    /// Set the bitrate of the channel. Applicable to voice channels only.
    pub const fn bitrate(mut self, bitrate: u64) -> Self {
        self.fields.bitrate = Some(bitrate);

        self
    }

    /// Set the kind of channel.
    pub const fn kind(mut self, kind: ChannelType) -> Self {
        self.fields.kind = Some(kind);

        self
    }

    /// Set whether the channel is marked as NSFW.
    pub const fn nsfw(mut self, nsfw: bool) -> Self {
        self.fields.nsfw = Some(nsfw);

        self
    }

    /// If this is specified, and the parent ID is a `ChannelType::CategoryChannel`, create this
    /// channel as a child of the category channel.
    pub const fn parent_id(mut self, parent_id: Id<ChannelMarker>) -> Self {
        self.fields.parent_id = Some(parent_id);

        self
    }

    /// Set the permission overwrites of a channel.
    pub const fn permission_overwrites(
        mut self,
        permission_overwrites: &'a [PermissionOverwrite],
    ) -> Self {
        self.fields.permission_overwrites = Some(permission_overwrites);

        self
    }

    /// Set the position of the channel.
    ///
    /// Positions are numerical and zero-indexed. If you place a channel at position 2, channels
    /// 2-n will shift down one position and the initial channel will take its place.
    pub const fn position(mut self, position: u64) -> Self {
        self.fields.position = Some(position);

        self
    }

    /// Set the number of seconds that a user must wait before before they are able to send another
    /// message.
    ///
    /// The minimum is 0 and the maximum is 21600. Refer to [the Discord Docs/Channel Object] for more details.
    /// This is also known as "Slow Mode".
    ///
    /// # Errors
    ///
    /// Returns an error of type [`RateLimitPerUserInvalid`] if the name is
    /// invalid.
    ///
    /// [`RateLimitPerUserInvalid`]: twilight_validate::channel::ChannelValidationErrorType::RateLimitPerUserInvalid
    /// [the Discord Docs/Channel Object]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure
    pub const fn rate_limit_per_user(
        mut self,
        rate_limit_per_user: u64,
    ) -> Result<Self, ChannelValidationError> {
        if let Err(source) = validate_rate_limit_per_user(rate_limit_per_user) {
            return Err(source);
        }

        self.fields.rate_limit_per_user = Some(rate_limit_per_user);

        Ok(self)
    }

    /// Set the topic.
    ///
    /// The maximum length is 1024 UTF-16 characters. Refer to [the Discord Docs/Channel Object] for more details.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`TopicInvalid`] if the name is
    /// invalid.
    ///
    /// [the Discord Docs/Channel Object]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure
    /// [the discord docs]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure
    pub fn topic(mut self, topic: &'a str) -> Result<Self, ChannelValidationError> {
        validate_topic(topic)?;

        self.fields.topic.replace(topic);

        Ok(self)
    }

    /// For voice channels, set the user limit.
    ///
    /// Set to 0 for no limit. Limit can otherwise be between 1 and 99 inclusive. Refer to [the
    /// Discord Docs/Modify Channel] for more details.
    ///
    /// [the Discord Docs/Modify Channel]: https://discord.com/developers/docs/resources/channel#modify-channel-json-params-guild-channel
    pub const fn user_limit(mut self, user_limit: u64) -> Self {
        self.fields.user_limit = Some(user_limit);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<GuildChannel> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildChannel<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for CreateGuildChannel<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::CreateChannel {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields)?;

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
