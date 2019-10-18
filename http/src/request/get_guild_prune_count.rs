use super::prelude::*;
use dawn_model::{guild::GuildPrune, id::GuildId};

#[derive(Default)]
struct GetGuildPruneCountFields {
    days: Option<u64>,
}

pub struct GetGuildPruneCount<'a> {
    fields: GetGuildPruneCountFields,
    fut: Option<Pending<'a, GuildPrune>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildPruneCount<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: impl Into<GuildId>) -> Self {
        Self {
            fields: GetGuildPruneCountFields::default(),
            fut: None,
            guild_id: guild_id.into(),
            http,
        }
    }

    pub fn days(mut self, days: u64) -> Self {
        self.fields.days.replace(days);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuildPruneCount {
                days: self.fields.days,
                guild_id: self.guild_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetGuildPruneCount<'_>, GuildPrune);
