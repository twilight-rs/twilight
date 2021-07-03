use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::id::GuildId;

/// Delete a guild permanently. The user must be the owner.
pub struct DeleteGuild<'a> {
    fut: Option<PendingResponse<'a, EmptyBody>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> DeleteGuild<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::DeleteGuild {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(DeleteGuild<'_>, EmptyBody);
