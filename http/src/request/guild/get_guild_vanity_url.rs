use crate::request::prelude::*;
use futures::TryFutureExt;
use serde::Deserialize;
use twilight_model::id::GuildId;

#[derive(Deserialize)]
struct VanityUrl {
    code: String,
}

pub struct GetGuildVanityUrl<'a> {
    fut: Option<Pending<'a, String>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildVanityUrl<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        let fut = self
            .http
            .request::<VanityUrl>(Request::from(Route::GetGuildVanityUrl {
                guild_id: self.guild_id.0,
            }))
            .map_ok(|url| url.code);
        self.fut.replace(Box::pin(fut));

        Ok(())
    }
}

poll_req!(GetGuildVanityUrl<'_>, String);
