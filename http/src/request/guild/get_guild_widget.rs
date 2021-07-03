use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::{guild::GuildWidget, id::GuildId};

/// Get the guild widget.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#get-guild-widget
pub struct GetGuildWidget<'a> {
    fut: Option<PendingResponse<'a, GuildWidget>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildWidget<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetGuildWidget {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGuildWidget<'_>, GuildWidget);
