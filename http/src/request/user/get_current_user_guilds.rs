use crate::request::prelude::*;
use dawn_model::{guild::PartialGuild, id::GuildId};

struct GetCurrentUserGuildsFields {
    after: Option<GuildId>,
    before: Option<GuildId>,
    limit: Option<u64>,
}

pub struct GetCurrentUserGuilds<'a> {
    fields: GetCurrentUserGuildsFields,
    fut: Option<Pending<'a, Vec<PartialGuild>>>,
    http: &'a Client,
}

impl<'a> GetCurrentUserGuilds<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self {
            fields: GetCurrentUserGuildsFields {
                after: None,
                before: None,
                limit: None,
            },
            fut: None,
            http,
        }
    }

    pub fn after(mut self, guild_id: GuildId) -> Self {
        self.fields.after.replace(guild_id);

        self
    }

    pub fn before(mut self, guild_id: GuildId) -> Self {
        self.fields.before.replace(guild_id);

        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.fields.limit.replace(limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetGuilds {
                after: self.fields.after.map(|x| x.0),
                before: self.fields.before.map(|x| x.0),
                limit: self.fields.limit,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetCurrentUserGuilds<'_>, Vec<PartialGuild>);
