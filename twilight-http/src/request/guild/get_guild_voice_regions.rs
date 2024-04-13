use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    voice::VoiceRegion,
};

/// Get voice region data for the guild.
///
/// Can return VIP servers if the guild is VIP-enabled.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildVoiceRegions<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildVoiceRegions<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }
}

impl IntoFuture for GetGuildVoiceRegions<'_> {
    type Output = Result<Response<ListBody<VoiceRegion>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<VoiceRegion>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildVoiceRegions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildVoiceRegions {
            guild_id: self.guild_id.get(),
        }))
    }
}
