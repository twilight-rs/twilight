use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::ListBody,
    routing::Route,
};
use twilight_model::{id::GuildId, voice::VoiceRegion};

/// Get voice region data for the guild.
///
/// Can return VIP servers if the guild is VIP-enabled.
pub struct GetGuildVoiceRegions<'a> {
    fut: Option<PendingResponse<'a, ListBody<VoiceRegion>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildVoiceRegions<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetGuildVoiceRegions {
            guild_id: self.guild_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetGuildVoiceRegions<'_>, ListBody<VoiceRegion>);
