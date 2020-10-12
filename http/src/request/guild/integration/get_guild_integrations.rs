use crate::request::prelude::*;
use twilight_model::{guild::GuildIntegration, id::GuildId};

/// Get the guild's integrations.
pub struct GetGuildIntegrations<'a> {
    fut: Option<Pending<'a, Vec<GuildIntegration>>>,
    guild_id: GuildId,
    http: &'a Client,
    include_applications: bool,
}

impl<'a> GetGuildIntegrations<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            include_applications: false,
        }
    }

    /// Sets if you want to receive bot and OAuth2 webhook integrations,
    /// otherwise it will only include Twitch and YouTube.
    pub fn include_applications(mut self, include: bool) -> Self {
        self.include_applications = include;

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildIntegrations {
                guild_id: self.guild_id.0,
                include_applications: self.include_applications,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildIntegrations<'_>, Vec<GuildIntegration>);
