use crate::{
    client::Client,
    request::Request,
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    application::command::permissions::GuildCommandPermissions,
    id::{
        marker::{ApplicationMarker, GuildMarker},
        Id,
    },
};

/// Get command permissions for all commands from the current application in a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildCommandPermissions<'a> {
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildCommandPermissions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
    ) -> Self {
        Self {
            application_id,
            guild_id,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<GuildCommandPermissions>> {
        let request = Request::from_route(&Route::GetGuildCommandPermissions {
            application_id: self.application_id.get(),
            guild_id: self.guild_id.get(),
        });

        self.http.request(request)
    }
}
