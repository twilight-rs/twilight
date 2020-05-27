use crate::request::prelude::*;
use twilight_model::{guild::Guild, id::GuildId};

#[derive(Default)]
struct GetGuildFields {
    with_counts: bool,
}

pub struct GetGuild<'a> {
    fields: GetGuildFields,
    fut: Option<Pending<'a, Option<Guild>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuild<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: GetGuildFields::default(),
            fut: None,
            guild_id,
            http,
        }
    }

    /// Sets if you want to receive `approximate_member_count` and
    /// `approximate_presence_count` in the guld structure.
    pub fn with_counts(mut self, with: bool) -> Self {
        self.fields.with_counts = with;

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuild {
                guild_id: self.guild_id.0,
                with_counts: self.fields.with_counts,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuild<'_>, Option<Guild>);
