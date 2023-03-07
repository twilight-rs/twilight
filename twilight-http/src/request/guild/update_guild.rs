use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Nullable, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
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
    afk_channel_id: Option<Nullable<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afk_timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_message_notifications: Option<Nullable<DefaultMessageNotificationLevel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discovery_splash: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explicit_content_filter: Option<Nullable<ExplicitContentFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    splash: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_channel_id: Option<Nullable<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_channel_flags: Option<Nullable<SystemChannelFlags>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_level: Option<Nullable<VerificationLevel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules_channel_id: Option<Nullable<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_updates_channel_id: Option<Nullable<Id<ChannelMarker>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_locale: Option<Nullable<&'a str>>,
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
    fields: Result<UpdateGuildFields<'a>, ValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateGuild<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: Ok(UpdateGuildFields {
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
            }),
            guild_id,
            http,
            reason: Ok(None),
        }
    }

    /// Set the voice channel where AFK voice users are sent.
    pub fn afk_channel_id(mut self, afk_channel_id: Option<Id<ChannelMarker>>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.afk_channel_id = Some(Nullable(afk_channel_id));
        }

        self
    }

    /// Set how much time it takes for a voice user to be considered AFK.
    pub fn afk_timeout(mut self, afk_timeout: u64) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.afk_timeout = Some(afk_timeout);
        }

        self
    }

    /// Set the banner.
    ///
    /// This is a base64 encoded 16:9 PNG or JPEG image. Pass `None` to remove
    /// the banner.
    ///
    /// The server must have the `BANNER` feature.
    pub fn banner(mut self, banner: Option<&'a str>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.banner = Some(Nullable(banner));
        }

        self
    }

    /// Set the default message notification level. See
    /// [Discord Docs/Create Guild] for more information.
    ///
    /// [Discord Docs/Create Guild]: https://discord.com/developers/docs/resources/guild#create-guild
    pub fn default_message_notifications(
        mut self,
        default_message_notifications: Option<DefaultMessageNotificationLevel>,
    ) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.default_message_notifications = Some(Nullable(default_message_notifications));
        }

        self
    }

    /// Set the guild's discovery splash image.
    ///
    /// Requires the guild to have the `DISCOVERABLE` feature enabled.
    pub fn discovery_splash(mut self, discovery_splash: Option<&'a str>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.discovery_splash = Some(Nullable(discovery_splash));
        }

        self
    }

    /// Set the explicit content filter level.
    pub fn explicit_content_filter(
        mut self,
        explicit_content_filter: Option<ExplicitContentFilter>,
    ) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.explicit_content_filter = Some(Nullable(explicit_content_filter));
        }

        self
    }

    /// Set the enabled features of the guild.
    ///
    /// Attempting to add or remove the [`GuildFeature::Community`] feature requires the
    /// [`Permissions::ADMINISTRATOR`] permission.
    ///
    /// Attempting to add or remove the [`GuildFeature::Discoverable`] feature requires
    /// the [`Permissions::ADMINISTRATOR`] permission. Additionally the guild
    /// must pass all the discovery requirements.
    ///
    /// Attempting to add or remove the [`GuildFeature::InvitesDisabled`] feature requires
    /// the [`Permissions::MANAGE_GUILD`] permission.
    ///
    /// [`GuildFeature::Community`]: twilight_model::guild::GuildFeature::Community
    /// [`GuildFeature::Discoverable`]: twilight_model::guild::GuildFeature::Discoverable
    /// [`GuildFeature::InvitesDisabled`]: twilight_model::guild::GuildFeature::InvitesDisabled
    /// [`Permissions::ADMINISTRATOR`]: twilight_model::guild::Permissions::ADMINISTRATOR
    /// [`Permissions::MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
    pub fn features(mut self, features: &'a [&'a str]) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.features = Some(features);
        }

        self
    }

    /// Set the icon.
    ///
    /// This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type
    /// and `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub fn icon(mut self, icon: Option<&'a str>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.icon = Some(Nullable(icon));
        }

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
    pub fn name(mut self, name: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_guild_name(name)?;
            fields.name.replace(name);

            Ok(fields)
        });

        self
    }

    /// Transfer ownership to another user.
    ///
    /// Only works if the current user is the owner.
    pub fn owner_id(mut self, owner_id: Id<UserMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.owner_id = Some(owner_id);
        }

        self
    }

    /// Set the guild's splash image.
    ///
    /// Requires the guild to have the `INVITE_SPLASH` feature enabled.
    pub fn splash(mut self, splash: Option<&'a str>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.splash = Some(Nullable(splash));
        }

        self
    }

    /// Set the channel where events such as welcome messages are posted.
    pub fn system_channel(mut self, system_channel_id: Option<Id<ChannelMarker>>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.system_channel_id = Some(Nullable(system_channel_id));
        }

        self
    }

    /// Set the guild's [`SystemChannelFlags`].
    pub fn system_channel_flags(
        mut self,
        system_channel_flags: Option<SystemChannelFlags>,
    ) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.system_channel_flags = Some(Nullable(system_channel_flags));
        }

        self
    }

    /// Set the rules channel.
    ///
    /// Requires the guild to be `PUBLIC`. See [Discord Docs/Modify Guild].
    ///
    /// [Discord Docs/Modify Guild]: https://discord.com/developers/docs/resources/guild#modify-guild
    pub fn rules_channel(mut self, rules_channel_id: Option<Id<ChannelMarker>>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.rules_channel_id = Some(Nullable(rules_channel_id));
        }

        self
    }

    /// Set the public updates channel.
    ///
    /// Requires the guild to be `PUBLIC`.
    pub fn public_updates_channel(
        mut self,
        public_updates_channel_id: Option<Id<ChannelMarker>>,
    ) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.public_updates_channel_id = Some(Nullable(public_updates_channel_id));
        }

        self
    }

    /// Set the preferred locale for the guild.
    ///
    /// Defaults to `en-US`. Requires the guild to be `PUBLIC`.
    pub fn preferred_locale(mut self, preferred_locale: Option<&'a str>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.preferred_locale = Some(Nullable(preferred_locale));
        }

        self
    }

    /// Set the verification level.
    ///
    /// See [Discord Docs/Guild Object].
    ///
    /// [Discord Docs/Guild Object]: https://discord.com/developers/docs/resources/guild#guild-object-verification-level
    pub fn verification_level(mut self, verification_level: Option<VerificationLevel>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.verification_level = Some(Nullable(verification_level));
        }

        self
    }

    /// Set whether the premium progress bar is enabled.
    pub fn premium_progress_bar_enabled(mut self, premium_progress_bar_enabled: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.premium_progress_bar_enabled = Some(premium_progress_bar_enabled);
        }

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateGuild<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateGuild<'_> {
    type Output = Result<Response<PartialGuild>, Error>;

    type IntoFuture = ResponseFuture<PartialGuild>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateGuild<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::UpdateGuild {
            guild_id: self.guild_id.get(),
        })
        .json(&fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
