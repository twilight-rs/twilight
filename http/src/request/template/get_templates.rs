use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::ListBody,
    routing::Route,
};
use twilight_model::{id::GuildId, template::Template};

/// Get a list of templates in a guild, by ID.
pub struct GetTemplates<'a> {
    fut: Option<PendingResponse<'a, ListBody<Template>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetTemplates<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetTemplates {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetTemplates<'_>, ListBody<Template>);
