use crate::request::prelude::*;
use twilight_model::{id::GuildId, template::Template};

/// Get a list of templates in a guild, by ID.
pub struct GetTemplates<'a> {
    fut: Option<Pending<'a, Vec<Template>>>,
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

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetTemplates {
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetTemplates<'_>, Vec<Template>);
