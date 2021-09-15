use crate::{
    client::Client,
    request::{IntoRequest, Request},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{guild::GuildPreview, id::GuildId};

/// For public guilds, get the guild preview.
///
/// This works even if the user is not in the guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildPreview<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildPreview<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<GuildPreview> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetGuildPreview<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        Ok(Request::from_route(&Route::GetGuildPreview {
            guild_id: self.guild_id.get(),
        }))
    }
}
