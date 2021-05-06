use crate::request::prelude::*;
use twilight_model::{id::GuildId, invite::WelcomeScreen};

/// Get the guild's welcome screen.
pub struct GetGuildWelcomeScreen<'a> {
    fut: Option<PendingOption<'a>>,
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

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetGuildWelcomeScreen {
                    guild_id: self.guild_id.0,
                },
            ))));

        Ok(())
    }
}

poll_req!(opt, GetGuildWelcomeScreen<'_>, WelcomeScreen);
