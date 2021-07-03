use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::{guild::GuildPreview, id::GuildId};

/// For public guilds, get the guild preview.
///
/// This works even if the user is not in the guild.
pub struct GetGuildPreview<'a> {
    fut: Option<PendingResponse<'a, GuildPreview>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildPreview<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetGuildPreview {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGuildPreview<'_>, GuildPreview);
