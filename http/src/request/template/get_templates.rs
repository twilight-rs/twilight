use crate::{
    client::Client,
    request::Request,
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{id::GuildId, template::Template};

/// Get a list of templates in a guild, by ID.
#[must_use = "requests must be configured and executed"]
pub struct GetTemplates<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetTemplates<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Template>> {
        let request = Request::from_route(&Route::GetTemplates {
            guild_id: self.guild_id.get(),
        });

        self.http.request(request)
    }
}
