use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    id::{marker::GuildMarker, Id},
    template::Template,
};

/// Sync a template to the current state of the guild, by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct SyncTemplate<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    template_code: &'a str,
}

impl<'a> SyncTemplate<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        template_code: &'a str,
    ) -> Self {
        Self {
            guild_id,
            http,
            template_code,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Template> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for SyncTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::SyncTemplate {
            guild_id: self.guild_id.get(),
            template_code: self.template_code,
        }))
    }
}
