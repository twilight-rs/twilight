use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::ListBody,
    routing::Route,
};
use twilight_model::{guild::GuildIntegration, id::GuildId};

/// Get the guild's integrations.
pub struct GetGuildIntegrations<'a> {
    fut: Option<PendingResponse<'a, ListBody<GuildIntegration>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildIntegrations<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetGuildIntegrations {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGuildIntegrations<'_>, ListBody<GuildIntegration>);
