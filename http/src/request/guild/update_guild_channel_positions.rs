use crate::{
    client::Client,
    request::Request,
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};

#[derive(Serialize)]
pub struct Position {
    id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lock_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u64>,
}

impl From<(Id<ChannelMarker>, u64)> for Position {
    fn from((id, position): (Id<ChannelMarker>, u64)) -> Self {
        Self {
            id,
            lock_permissions: None,
            parent_id: None,
            position: Some(position),
        }
    }
}

/// Modify the positions of the channels.
///
/// The minimum amount of channels to modify, is a swap between two channels.
///
/// This function accepts an `Iterator` of `(Id<ChannelMarker>, u64)`. It also accepts
/// an `Iterator` of `Position`, which has extra fields.
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

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let mut request = Request::builder(&Route::UpdateGuildChannels {
            guild_id: self.guild_id.get(),
        });

        request = match request.json(&self.positions) {
            Ok(request) => request,
            Err(source) => return ResponseFuture::error(source),
        };

        self.http.request(request.build())
    }
}
