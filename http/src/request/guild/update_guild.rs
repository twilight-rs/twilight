use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    guild::{
        DefaultMessageNotificationLevel,
        ExplicitContentFilter,
        PartialGuild,
        VerificationLevel,
    },
    id::{ChannelId, GuildId, UserId},
};

#[derive(Clone, Debug)]
pub enum UpdateGuildError {
    /// The name length is either fewer than 2 UTF-8 characters or more than
    /// 100 UTF-8 characters.
    NameInvalid,
}

impl Display for UpdateGuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid => f.write_str("the name's length is invalid"),
        }
    }
}

impl Error for UpdateGuildError {}

#[derive(Default, Serialize)]
struct UpdateGuildFields {
    afk_channel_id: Option<ChannelId>,
    afk_timeout: Option<u64>,
    default_message_notifications: Option<DefaultMessageNotificationLevel>,
    explicit_content_filter: Option<ExplicitContentFilter>,
    icon: Option<String>,
    name: Option<String>,
    owner_id: Option<UserId>,
    region: Option<String>,
    splash: Option<String>,
    system_channel_id: Option<ChannelId>,
    verification_level: Option<VerificationLevel>,
    rules_channel_id: Option<ChannelId>,
    public_updates_channel_id: Option<ChannelId>,
    preferred_locale: Option<String>,
}

pub struct UpdateGuild<'a> {
    fields: UpdateGuildFields,
    fut: Option<Pending<'a, PartialGuild>>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> UpdateGuild<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: UpdateGuildFields::default(),
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    pub fn afk_channel_id(mut self, afk_channel_id: impl Into<ChannelId>) -> Self {
        self.fields.afk_channel_id.replace(afk_channel_id.into());

        self
    }

    pub fn afk_timeout(mut self, afk_timeout: u64) -> Self {
        self.fields.afk_timeout.replace(afk_timeout);

        self
    }

    pub fn default_message_notifications(
        mut self,
        default_message_notifications: DefaultMessageNotificationLevel,
    ) -> Self {
        self.fields
            .default_message_notifications
            .replace(default_message_notifications);

        self
    }

    pub fn explicit_content_filter(
        mut self,
        explicit_content_filter: ExplicitContentFilter,
    ) -> Self {
        self.fields
            .explicit_content_filter
            .replace(explicit_content_filter);

        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.fields.icon.replace(icon.into());

        self
    }

    /// Set the name of the guild.
    ///
    /// The minimum length is 2 UTF-8 characters and the maximum is 100 UTF-8
    /// characters.
    ///
    /// # Erroors
    ///
    /// Returns [`UpdateGuildError::NameInvalid`] if the name length is too
    /// short or too long.
    ///
    /// [`UpdateGuildError::NameInvalid`]: enum.UpdateGuildError.html#variant.NameInvalid
    pub fn name(self, name: impl Into<String>) -> Result<Self, UpdateGuildError> {
        self._name(name.into())
    }

    fn _name(mut self, name: String) -> Result<Self, UpdateGuildError> {
        if !validate::guild_name(&name) {
            return Err(UpdateGuildError::NameInvalid);
        }

        self.fields.name.replace(name);

        Ok(self)
    }

    pub fn owner_id(mut self, owner_id: impl Into<UserId>) -> Self {
        self.fields.owner_id.replace(owner_id.into());

        self
    }

    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.fields.region.replace(region.into());

        self
    }

    pub fn splash(mut self, splash: impl Into<String>) -> Self {
        self.fields.splash.replace(splash.into());

        self
    }

    pub fn system_channel(mut self, system_channel_id: impl Into<ChannelId>) -> Self {
        self.fields
            .system_channel_id
            .replace(system_channel_id.into());

        self
    }

    pub fn rules_channel(mut self, rules_channel_id: impl Into<ChannelId>) -> Self {
        self.fields
            .rules_channel_id
            .replace(rules_channel_id.into());

        self
    }

    pub fn public_updates_channel(
        mut self,
        public_updates_channel_id: impl Into<ChannelId>,
    ) -> Self {
        self.fields
            .public_updates_channel_id
            .replace(public_updates_channel_id.into());

        self
    }

    pub fn preferred_locale(mut self, preferred_locale: impl Into<String>) -> Self {
        self.fields
            .preferred_locale
            .replace(preferred_locale.into());

        self
    }

    pub fn verification_level(mut self, verification_level: VerificationLevel) -> Self {
        self.fields.verification_level.replace(verification_level);

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
                Route::UpdateGuild {
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from((
                serde_json::to_vec(&self.fields)?,
                Route::UpdateGuild {
                    guild_id: self.guild_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(UpdateGuild<'_>, PartialGuild);
