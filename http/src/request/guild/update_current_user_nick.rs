use crate::request::prelude::*;
use twilight_model::id::GuildId;

#[derive(Serialize)]
struct UpdateCurrentUserNickFields {
    nick: String,
}

/// Changes the user's nickname in a guild.
pub struct UpdateCurrentUserNick<'a> {
    fields: UpdateCurrentUserNickFields,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> UpdateCurrentUserNick<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, nick: impl Into<String>) -> Self {
        Self {
            fields: UpdateCurrentUserNickFields { nick: nick.into() },
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from((
            crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
            Route::UpdateNickname {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateCurrentUserNick<'_>, ());
