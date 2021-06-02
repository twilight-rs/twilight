use crate::{
    client::Client,
    error::Error,
    request::{Pending, Request},
    routing::Route,
};
use serde::Serialize;
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

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::builder(Route::UpdateNickname {
            guild_id: self.guild_id.0,
        })
        .json(&self.fields)?
        .build();

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(UpdateCurrentUserNick<'_>, ());
