use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::{
        forum::{DefaultReaction, ForumLayout, ForumSortOrder, ForumTag},
        permission_overwrite::PermissionOverwrite,
        thread::AutoArchiveDuration,
        Channel, ChannelType, VideoQualityMode,
    },
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};
use twilight_validate::{
    channel::{
        bitrate as validate_bitrate, name as validate_name,
        rate_limit_per_user as validate_rate_limit_per_user, topic as validate_topic,
        ChannelValidationError,
    },
    request::{audit_reason as validate_audit_reason, ValidationError},
};

#[derive(Serialize)]
struct CreateGuildChannelFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    available_tags: Option<&'a [ForumTag]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_forum_layout: Option<ForumLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_reaction_emoji: Option<&'a DefaultReaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_sort_order: Option<ForumSortOrder>,
    /// Initial `rate_limit_per_user` to set on newly created threads in a channel.
    /// This field is copied to the thread at creation time and does not live update.
    ///
    /// This field is only applicable for text, announcement, media, and forum channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    default_thread_rate_limit_per_user: Option<u16>,
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
    rate_limit_per_user: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rtc_region: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_limit: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_quality_mode: Option<VideoQualityMode>,
}

/// Create a new request to create a guild channel.
///
/// All fields are optional except for name. The minimum length of the name is 1
/// UTF-16 characters and the maximum is 100 UTF-16 characters.
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildChannel<'a> {
    fields: Result<CreateGuildChannelFields<'a>, ChannelValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateGuildChannel<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: Id<GuildMarker>, name: &'a str) -> Self {
        let fields = Ok(CreateGuildChannelFields {
            available_tags: None,
            bitrate: None,
            default_auto_archive_duration: None,
            default_forum_layout: None,
            default_reaction_emoji: None,
            default_sort_order: None,
            default_thread_rate_limit_per_user: None,
            kind: None,
            name,
            nsfw: None,
            parent_id: None,
            permission_overwrites: None,
            position: None,
            rate_limit_per_user: None,
            rtc_region: None,
            topic: None,
            user_limit: None,
            video_quality_mode: None,
        })
        .and_then(|fields| {
            validate_name(name)?;

            Ok(fields)
        });

        Self {
            fields,
            guild_id,
            http,
            reason: Ok(None),
        }
    }

    /// Set the available tags for the forum.
    pub fn available_tags(mut self, available_tags: &'a [ForumTag]) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.available_tags = Some(available_tags);
        }

        self
    }

    /// For voice and stage channels, set the bitrate of the channel.
    ///
    /// Must be at least 8000.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`BitrateInvalid`] if the bitrate is invalid.
    ///
    /// [`BitrateInvalid`]: twilight_validate::channel::ChannelValidationErrorType::BitrateInvalid
    pub fn bitrate(mut self, bitrate: u32) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_bitrate(bitrate)?;

            fields.bitrate = Some(bitrate);

            Ok(fields)
        });

        self
    }

    /// Set the default auto archive duration for newly created threads in the
    /// channel.
    ///
    /// Automatic archive durations are not locked behind the guild's boost
    /// level.
    pub fn default_auto_archive_duration(
        mut self,
        auto_archive_duration: AutoArchiveDuration,
    ) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.default_auto_archive_duration = Some(auto_archive_duration);
        }

        self
    }

    /// Set the default forum layout for new forum channels.
    pub fn default_forum_layout(mut self, default_forum_layout: ForumLayout) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.default_forum_layout = Some(default_forum_layout);
        }

        self
    }

    /// Set the default reaction emoji for new forum threads.
    pub fn default_reaction_emoji(mut self, default_reaction_emoji: &'a DefaultReaction) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.default_reaction_emoji = Some(default_reaction_emoji);
        }

        self
    }

    /// Set the default sort order for newly created forum channels.
    pub fn default_sort_order(mut self, default_sort_order: ForumSortOrder) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.default_sort_order = Some(default_sort_order);
        }

        self
    }

    /// Set the default number of seconds that a user must wait before before they are
    /// able to send another message in any newly-created thread in the channel.
    ///
    /// This field is only applicable for text, announcement, media, and forum channels.
    /// The minimum is 0 and the maximum is 21600. This is also known as "Slow Mode". See
    /// [Discord Docs/Channel Object].
    ///
    /// [Discord Docs/Channel Object]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure
    pub fn default_thread_rate_limit_per_user(
        mut self,
        default_thread_rate_limit_per_user: u16,
    ) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_rate_limit_per_user(default_thread_rate_limit_per_user)?;

            fields.default_thread_rate_limit_per_user = Some(default_thread_rate_limit_per_user);

            Ok(fields)
        });

        self
    }

    /// Set the kind of channel.
    pub fn kind(mut self, kind: ChannelType) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.kind = Some(kind);
        }

        self
    }

    /// Set whether the channel is marked as NSFW.
    pub fn nsfw(mut self, nsfw: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.nsfw = Some(nsfw);
        }

        self
    }

    /// If this is specified, and the parent ID is a `ChannelType::CategoryChannel`, create this
    /// channel as a child of the category channel.
    pub fn parent_id(mut self, parent_id: Id<ChannelMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.parent_id = Some(parent_id);
        }

        self
    }

    /// Set the permission overwrites of a channel.
    pub fn permission_overwrites(
        mut self,
        permission_overwrites: &'a [PermissionOverwrite],
    ) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.permission_overwrites = Some(permission_overwrites);
        }

        self
    }

    /// Set the position of the channel.
    ///
    /// Positions are numerical and zero-indexed. If you place a channel at position 2, channels
    /// 2-n will shift down one position and the initial channel will take its place.
    pub fn position(mut self, position: u64) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.position = Some(position);
        }

        self
    }

    /// Set the number of seconds that a user must wait before before they are able to send another
    /// message.
    ///
    /// The minimum is 0 and the maximum is 21600. This is also known as "Slow
    /// Mode". See [Discord Docs/Channel Object].
    ///
    /// # Errors
    ///
    /// Returns an error of type [`RateLimitPerUserInvalid`] if the name is
    /// invalid.
    ///
    /// [`RateLimitPerUserInvalid`]: twilight_validate::channel::ChannelValidationErrorType::RateLimitPerUserInvalid
    /// [Discord Docs/Channel Object]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure
    pub fn rate_limit_per_user(mut self, rate_limit_per_user: u16) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_rate_limit_per_user(rate_limit_per_user)?;

            fields.rate_limit_per_user = Some(rate_limit_per_user);

            Ok(fields)
        });

        self
    }

    /// For voice and stage channels, set the channel's RTC region.
    pub fn rtc_region(mut self, rtc_region: &'a str) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.rtc_region = Some(rtc_region);
        }

        self
    }

    /// Set the topic.
    ///
    /// The maximum length is 1024 UTF-16 characters. See
    /// [Discord Docs/Channel Object].
    ///
    /// # Errors
    ///
    /// Returns an error of type [`TopicInvalid`] if the name is
    /// invalid.
    ///
    /// [`TopicInvalid`]: twilight_validate::channel::ChannelValidationErrorType::TopicInvalid
    /// [Discord Docs/Channel Object]: https://discordapp.com/developers/docs/resources/channel#channel-object-channel-structure
    pub fn topic(mut self, topic: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_topic(topic)?;

            fields.topic.replace(topic);

            Ok(fields)
        });

        self
    }

    /// For voice channels, set the user limit.
    ///
    /// Set to 0 for no limit. Limit can otherwise be between 1 and 99
    /// inclusive. See [Discord Docs/Modify Channel] for more details.
    ///
    /// [Discord Docs/Modify Channel]: https://discord.com/developers/docs/resources/channel#modify-channel-json-params-guild-channel
    pub fn user_limit(mut self, user_limit: u16) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.user_limit = Some(user_limit);
        }

        self
    }

    /// For voice channels, set the channel's video quality mode.
    pub fn video_quality_mode(mut self, video_quality_mode: VideoQualityMode) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.video_quality_mode = Some(video_quality_mode);
        }

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildChannel<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for CreateGuildChannel<'_> {
    type Output = Result<Response<Channel>, Error>;

    type IntoFuture = ResponseFuture<Channel>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateGuildChannel<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::CreateChannel {
            guild_id: self.guild_id.get(),
        })
        .json(&fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
