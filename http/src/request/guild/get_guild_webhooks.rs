use crate::{
    client::Client,
    request::{IntoRequest, Request},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{channel::Webhook, id::GuildId};

/// Get the webhooks of a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildWebhooks<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildWebhooks<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Webhook>> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetGuildWebhooks<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        Ok(Request::from_route(&Route::GetGuildWebhooks {
            guild_id: self.guild_id.get(),
        }))
    }
}
