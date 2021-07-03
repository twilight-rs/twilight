use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::ListBody,
    routing::Route,
};
use twilight_model::{channel::Webhook, id::GuildId};

/// Get the webhooks of a guild.
pub struct GetGuildWebhooks<'a> {
    fut: Option<PendingResponse<'a, ListBody<Webhook>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildWebhooks<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetGuildWebhooks {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGuildWebhooks<'_>, ListBody<Webhook>);
