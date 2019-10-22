use crate::request::prelude::*;
use dawn_model::id::GuildId;
use futures_util::TryFutureExt;
use serde::Deserialize;

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
