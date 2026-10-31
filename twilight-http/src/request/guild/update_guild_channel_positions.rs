use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    http::channel_position::Position,
    id::{Id, marker::GuildMarker},
};

#[derive(Serialize)]
#[serde(transparent)]
struct UpdateGuildChannelPositionsBody<'a>(&'a [Position]);

struct UpdateGuildChannelPositionsPath {
    guild_id: Id<GuildMarker>,
}

/// Modify the positions of the channels.
///
/// The minimum amount of channels to modify, is a swap between two channels.
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildChannelPositions<'a> {
    body: UpdateGuildChannelPositionsBody<'a>,
    path: UpdateGuildChannelPositionsPath,
    http: &'a Client,
}

impl<'a> UpdateGuildChannelPositions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        channel_positions: &'a [Position],
    ) -> Self {
        Self {
            body: UpdateGuildChannelPositionsBody(channel_positions),
            path: UpdateGuildChannelPositionsPath { guild_id },
            http,
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
        let UpdateGuildChannelPositionsPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("channels")
            .build()
            .map_err(Error::path)?;

        Request::builder(Method::Patch, path)
            .json(&self.body)
            .build()
    }
}
