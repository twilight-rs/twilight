use crate::{
    client::Client,
    error::Error as HttpError,
    request::{self, AuditLogReason, NullableField, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, PartialGuild, SystemChannelFlags,
        VerificationLevel,
    },
    id::{
        marker::{ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
};
use twilight_validate::request::{
    audit_reason as validate_audit_reason, guild_name as validate_guild_name, ValidationError,
};

#[derive(Serialize)]
struct UpdateGuildFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    afk_channel_id: Option<NullableField<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afk_timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_message_notifications: Option<NullableField<DefaultMessageNotificationLevel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_splash: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explicit_content_filter: Option<NullableField<ExplicitContentFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    splash: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_channel_id: Option<NullableField<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_channel_flags: Option<NullableField<SystemChannelFlags>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_level: Option<NullableField<VerificationLevel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules_channel_id: Option<NullableField<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_updates_channel_id: Option<NullableField<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_locale: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    premium_progress_bar_enabled: Option<bool>,
}

/// Update a guild.
///
/// All endpoints are optional. See [Discord Docs/Modify Guild].
///
/// [Discord Docs/Modify Guild]: https://discord.com/developers/docs/resources/guild#modify-guild
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuild<'a> {
    fields: UpdateGuildFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> UpdateGuild<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: UpdateGuildFields {
                afk_channel_id: None,
                afk_timeout: None,
                banner: None,
                default_message_notifications: None,
                discovery_splash: None,
                explicit_content_filter: None,
                features: None,
                icon: None,
                name: None,
                owner_id: None,
                splash: None,
                system_channel_id: None,
                system_channel_flags: None,
                verification_level: None,
                rules_channel_id: None,
                public_updates_channel_id: None,
                preferred_locale: None,
                premium_progress_bar_enabled: None,
            },
            guild_id,
            http,
            reason: None,
        }
    }

    /// Set the voice channel where AFK voice users are sent.
    pub const fn afk_channel_id(mut self, afk_channel_id: Option<Id<ChannelMarker>>) -> Self {
        self.fields.afk_channel_id = Some(NullableField(afk_channel_id));

        self
    }

    /// Set how much time it takes for a voice user to be considered AFK.
    pub const fn afk_timeout(mut self, afk_timeout: u64) -> Self {
        self.fields.afk_timeout = Some(afk_timeout);

        self
    }

    /// Set the banner.
    ///
    /// This is a base64 encoded 16:9 PNG or JPEG image. Pass `None` to remove
    /// the banner.
    ///
    /// The server must have the `BANNER` feature.
    pub const fn banner(mut self, banner: Option<&'a str>) -> Self {
        self.fields.banner = Some(NullableField(banner));

        self
    }

    /// Set the default message notification level. See
    /// [Discord Docs/Create Guild] for more information.
    ///
    /// [Discord Docs/Create Guild]: https://discord.com/developers/docs/resources/guild#create-guild
    pub const fn default_message_notifications(
        mut self,
        default_message_notifications: Option<DefaultMessageNotificationLevel>,
    ) -> Self {
        self.fields.default_message_notifications =
            Some(NullableField(default_message_notifications));

        self
    }

    /// Set the guild's discovery splash image.
    ///
    /// Requires the guild to have the `DISCOVERABLE` feature enabled.
    pub const fn discovery_splash(mut self, discovery_splash: Option<&'a str>) -> Self {
        self.fields.discovery_splash = Some(NullableField(discovery_splash));

        self
    }

    /// Set the explicit content filter level.
    pub const fn explicit_content_filter(
        mut self,
        explicit_content_filter: Option<ExplicitContentFilter>,
    ) -> Self {
        self.fields.explicit_content_filter = Some(NullableField(explicit_content_filter));

        self
    }

    /// Set the enabled features of the guild.
    pub const fn features(mut self, features: &'a [&'a str]) -> Self {
        self.fields.features = Some(features);

        self
    }

    /// Set the icon.
    ///
    /// This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type
    /// and `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub const fn icon(mut self, icon: Option<&'a str>) -> Self {
        self.fields.icon = Some(NullableField(icon));

        self
    }

    /// Set the name of the guild.
    ///
    /// The minimum length is 1 UTF-16 character and the maximum is 100 UTF-16
    /// characters.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`GuildName`] if the name length is too short
    /// or too long.
    ///
    /// [`GuildName`]: twilight_validate::request::ValidationErrorType::GuildName
    pub fn name(mut self, name: &'a str) -> Result<Self, ValidationError> {
        validate_guild_name(name)?;

        self.fields.name.replace(name);

        Ok(self)
    }

    /// Transfer ownership to another user.
    ///
    /// Only works if the current user is the owner.
    pub const fn owner_id(mut self, owner_id: Id<UserMarker>) -> Self {
        self.fields.owner_id = Some(owner_id);

        self
    }

    /// Set the guild's splash image.
    ///
    /// Requires the guild to have the `INVITE_SPLASH` feature enabled.
    pub const fn splash(mut self, splash: Option<&'a str>) -> Self {
        self.fields.splash = Some(NullableField(splash));

        self
    }

    /// Set the channel where events such as welcome messages are posted.
    pub const fn system_channel(mut self, system_channel_id: Option<Id<ChannelMarker>>) -> Self {
        self.fields.system_channel_id = Some(NullableField(system_channel_id));

        self
    }

    /// Set the guild's [`SystemChannelFlags`].
    pub const fn system_channel_flags(
        mut self,
        system_channel_flags: Option<SystemChannelFlags>,
    ) -> Self {
        self.fields.system_channel_flags = Some(NullableField(system_channel_flags));

        self
    }

    /// Set the rules channel.
    ///
    /// Requires the guild to be `PUBLIC`. See [Discord Docs/Modify Guild].
    ///
    /// [Discord Docs/Modify Guild]: https://discord.com/developers/docs/resources/guild#modify-guild
    pub const fn rules_channel(mut self, rules_channel_id: Option<Id<ChannelMarker>>) -> Self {
        self.fields.rules_channel_id = Some(NullableField(rules_channel_id));

        self
    }

    /// Set the public updates channel.
    ///
    /// Requires the guild to be `PUBLIC`.
    pub const fn public_updates_channel(
        mut self,
        public_updates_channel_id: Option<Id<ChannelMarker>>,
    ) -> Self {
        self.fields.public_updates_channel_id = Some(NullableField(public_updates_channel_id));

        self
    }

    /// Set the preferred locale for the guild.
    ///
    /// Defaults to `en-US`. Requires the guild to be `PUBLIC`.
    pub const fn preferred_locale(mut self, preferred_locale: Option<&'a str>) -> Self {
        self.fields.preferred_locale = Some(NullableField(preferred_locale));

        self
    }

    /// Set the verification level.
    ///
    /// See [Discord Docs/Guild Object].
    ///
    /// [Discord Docs/Guild Object]: https://discord.com/developers/docs/resources/guild#guild-object-verification-level
    pub const fn verification_level(
        mut self,
        verification_level: Option<VerificationLevel>,
    ) -> Self {
        self.fields.verification_level = Some(NullableField(verification_level));

        self
    }

    /// Set whether the premium progress bar is enabled.
    pub const fn premium_progress_bar_enabled(
        mut self,
        premium_progress_bar_enabled: bool,
    ) -> Self {
        self.fields.premium_progress_bar_enabled = Some(premium_progress_bar_enabled);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<PartialGuild> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateGuild<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl TryIntoRequest for UpdateGuild<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::UpdateGuild {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields)?;

        if let Some(reason) = &self.reason {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
