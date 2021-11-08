use crate::{
    client::Client,
    request::Request,
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
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

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<VoiceRegion>> {
        let request = Request::from_route(&Route::GetGuildVoiceRegions {
            guild_id: self.guild_id.get(),
        });

        self.http.request(request)
    }
}
