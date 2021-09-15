use crate::{
    client::Client,
    request::{IntoRequest, Request},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{guild::VanityUrl, id::GuildId};

/// Get a guild's vanity url, if there is one.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildVanityUrl<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildVanityUrl<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<VanityUrl> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetGuildVanityUrl<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        Ok(Request::from_route(&Route::GetGuildVanityUrl {
            guild_id: self.guild_id.get(),
        }))
    }
}
