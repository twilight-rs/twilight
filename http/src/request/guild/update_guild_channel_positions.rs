use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use serde::Serialize;
use twilight_model::id::{ChannelId, GuildId};

#[derive(Serialize)]
pub struct Position {
    id: ChannelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    lock_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<ChannelId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<u64>,
}

impl From<(ChannelId, u64)> for Position {
    fn from((id, position): (ChannelId, u64)) -> Self {
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
/// This function accepts an `Iterator` of `(ChannelId, u64)`. It also accepts
/// an `Iterator` of `Position`, which has extra fields.
pub struct UpdateGuildChannelPositions<'a> {
    fut: Option<PendingResponse<'a, EmptyBody>>,
    guild_id: GuildId,
    http: &'a Client,
    positions: Vec<Position>,
}

impl<'a> UpdateGuildChannelPositions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        channel_positions: impl Iterator<Item = impl Into<Position>>,
    ) -> Self {
        let positions = channel_positions.map(Into::into).collect();

        Self {
            fut: None,
            guild_id,
            http,
            positions,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::builder(Route::UpdateGuildChannels {
            guild_id: self.guild_id.0,
        })
        .json(&self.positions)?
        .build();

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(UpdateGuildChannelPositions<'_>, EmptyBody);
