use crate::request::prelude::*;
use twilight_model::id::{ChannelId, GuildId};

#[derive(Serialize)]
struct Position {
    id: ChannelId,
    position: u64,
}

/// Modify the positions of the channels.
///
/// The minimum amount of channels to modify, is a swap between two channels.
pub struct UpdateGuildChannelPositions<'a> {
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    positions: Vec<Position>,
}

impl<'a> UpdateGuildChannelPositions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        channel_positions: impl Iterator<Item = (ChannelId, u64)>,
    ) -> Self {
        let positions = channel_positions
            .map(|(id, position)| Position { id, position })
            .collect::<Vec<_>>();

        Self {
            fut: None,
            guild_id,
            http,
            positions,
        }
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::builder(Route::UpdateGuildChannels {
            guild_id: self.guild_id.0,
        })
        .json(&self.positions)?
        .build();

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(UpdateGuildChannelPositions<'_>, ());
