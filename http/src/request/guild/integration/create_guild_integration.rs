use crate::json_to_vec;
use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::id::{GuildId, IntegrationId};

#[derive(Serialize)]
struct CreateGuildIntegrationFields<'a> {
    id: IntegrationId,
    #[serde(rename = "type")]
    kind: Cow<'a, str>,
}

/// Create a guild integration from the current user to the guild.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#create-guild-integration
pub struct CreateGuildIntegration<'a> {
    fields: CreateGuildIntegrationFields<'a>,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<Cow<'a, str>>,
}

impl<'a> CreateGuildIntegration<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        integration_id: IntegrationId,
        kind: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            fields: CreateGuildIntegrationFields {
                id: integration_id,
                kind: kind.into(),
            },
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<Cow<'a, str>>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                json_to_vec(&self.fields)?,
                headers,
                Route::CreateGuildIntegration {
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from((
                json_to_vec(&self.fields)?,
                Route::CreateGuildIntegration {
                    guild_id: self.guild_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateGuildIntegration<'_>, ());
