use twilight_model::id::{marker::ChannelMarker, Id};

use crate::client::Client;

#[must_use = "requests must be configured and executed"]
pub struct SetVoiceChannelStatus<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
}

