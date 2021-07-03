use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::{id::GuildId, invite::WelcomeScreen};

/// Get the guild's welcome screen.
pub struct GetGuildWelcomeScreen<'a> {
    fut: Option<PendingResponse<'a, WelcomeScreen>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildWelcomeScreen<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetGuildWelcomeScreen {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGuildWelcomeScreen<'_>, WelcomeScreen);
