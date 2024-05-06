use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    http::channel_position::Position,
    id::{marker::GuildMarker, Id},
};

/// Modify the positions of the channels.
///
/// The minimum amount of channels to modify, is a swap between two channels.
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildChannelPositions<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    positions: &'a [Position],
}

impl<'a> UpdateGuildChannelPositions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        channel_positions: &'a [Position],
    ) -> Self {
        Self {
            guild_id,
            http,
            positions: channel_positions,
        }
    }
}

impl IntoFuture for UpdateGuildChannelPositions<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateGuildChannelPositions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::UpdateGuildChannels {
            guild_id: self.guild_id.get(),
        })
        .json(&self.positions)
        .build()
    }
}
