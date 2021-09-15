use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::GuildId;

/// Delete a template by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct DeleteTemplate<'a> {
    guild_id: GuildId,
    http: &'a Client,
    template_code: &'a str,
}

impl<'a> DeleteTemplate<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId, template_code: &'a str) -> Self {
        Self {
            guild_id,
            http,
            template_code,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for DeleteTemplate<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::DeleteTemplate {
            guild_id: self.guild_id.get(),
            template_code: self.template_code,
        }))
    }
}
