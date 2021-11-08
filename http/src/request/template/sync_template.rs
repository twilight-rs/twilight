use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
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
        let request = Request::from_route(&Route::SyncTemplate {
            guild_id: self.guild_id.get(),
            template_code: self.template_code,
        });

        self.http.request(request)
    }
}
