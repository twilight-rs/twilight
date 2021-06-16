use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::{id::GuildId, template::Template};

/// Sync a template to the current state of the guild, by ID and code.
pub struct SyncTemplate<'a> {
    fut: Option<PendingResponse<'a, Template>>,
    guild_id: GuildId,
    http: &'a Client,
    template_code: String,
}

impl<'a> SyncTemplate<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        template_code: impl Into<String>,
    ) -> Self {
        Self::_new(http, guild_id, template_code.into())
    }

    fn _new(http: &'a Client, guild_id: GuildId, template_code: String) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
            template_code,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::SyncTemplate {
            guild_id: self.guild_id.0,
            template_code: self.template_code.clone(),
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(SyncTemplate<'_>, Template);
