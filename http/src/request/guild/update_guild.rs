use crate::request::prelude::*;
use dawn_model::{
    guild::{
        DefaultMessageNotificationLevel,
        ExplicitContentFilter,
        PartialGuild,
        VerificationLevel,
    },
    id::{ChannelId, GuildId, UserId},
};

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

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.fields.name.replace(name.into());

        self
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

    pub fn system_channel_id(mut self, system_channel_id: impl Into<ChannelId>) -> Self {
        self.fields
            .system_channel_id
            .replace(system_channel_id.into());

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
