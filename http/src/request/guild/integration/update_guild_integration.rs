use crate::request::prelude::*;
use twilight_model::id::{GuildId, IntegrationId};

#[derive(Default, Serialize)]
struct UpdateGuildIntegrationFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_emoticons: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_behavior: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expire_grace_period: Option<u64>,
}

/// Update a guild's integration, by its id.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#modify-guild-integrationb
pub struct UpdateGuildIntegration<'a> {
    fields: UpdateGuildIntegrationFields,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    integration_id: IntegrationId,
    reason: Option<String>,
}

impl<'a> UpdateGuildIntegration<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, integration_id: IntegrationId) -> Self {
        Self {
            fields: UpdateGuildIntegrationFields::default(),
            fut: None,
            guild_id,
            http,
            integration_id,
            reason: None,
        }
    }

    /// Whether to enable emoticons for this integration.
    ///
    /// This only works for Twitch integrations.
    pub fn enable_emoticons(mut self, enable_emoticons: bool) -> Self {
        self.fields.enable_emoticons.replace(enable_emoticons);

        self
    }

    /// The behavior when an integration lapses.
    ///
    /// Refer to [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/guild#integration-object-integration-expire-behaviors
    pub fn expire_behavior(mut self, expire_behavior: u64) -> Self {
        self.fields.expire_behavior.replace(expire_behavior);

        self
    }

    /// Set the grace period in days that the integration will ignore lapsed subscriptions.
    pub fn expire_grace_period(mut self, expire_grace_period: u64) -> Self {
        self.fields.expire_grace_period.replace(expire_grace_period);

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                headers,
                Route::UpdateGuildIntegration {
                    guild_id: self.guild_id.0,
                    integration_id: self.integration_id.0,
                },
            ))
        } else {
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                Route::UpdateGuildIntegration {
                    guild_id: self.guild_id.0,
                    integration_id: self.integration_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for UpdateGuildIntegration<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(UpdateGuildIntegration<'_>, ());
