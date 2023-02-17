use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::Guild,
    id::{marker::GuildMarker, Id},
};

struct GetGuildFields {
    with_counts: bool,
}

/// Get information about a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetGuild<'a> {
    fields: GetGuildFields,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuild<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: GetGuildFields { with_counts: false },
            guild_id,
            http,
        }
    }

    /// Sets if you want to receive `approximate_member_count` and `approximate_presence_count` in
    /// the guild structure.
    pub const fn with_counts(mut self, with: bool) -> Self {
        self.fields.with_counts = with;

        self
    }
}

impl IntoFuture for GetGuild<'_> {
    type Output = Result<Response<Guild>, Error>;

    type IntoFuture = ResponseFuture<Guild>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuild<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuild {
            guild_id: self.guild_id.get(),
            with_counts: self.fields.with_counts,
        }))
    }
}
