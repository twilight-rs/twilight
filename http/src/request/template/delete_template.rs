use crate::request::prelude::*;
use twilight_model::id::GuildId;

/// Delete a template by ID and code.
pub struct DeleteTemplate<'a> {
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    template_code: String,
}

impl<'a> DeleteTemplate<'a> {
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

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::DeleteTemplate {
                guild_id: self.guild_id.0,
                template_code: self.template_code.clone(),
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteTemplate<'_>, ());
