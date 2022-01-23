use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{guild::GuildWidgetSettings, id::{Id, marker::GuildMarker}};

/// Get the guild's widget settings.
///
/// Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/guild#get-guild-widget-settings
#[must_use = "requests must be configured and executed"]
pub struct GetGuildWidgetSettings<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildWidgetSettings<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<GuildWidgetSettings> {
        let request = Request::from_route(&Route::GetGuildWidgetSettings {
            guild_id: self.guild_id.get(),
        });

        self.http.request(request)
    }
}
