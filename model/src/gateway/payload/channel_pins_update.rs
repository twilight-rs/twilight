use crate::id::ChannelId;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ChannelPinsUpdate {
    pub channel_id: ChannelId,
    pub last_pin_timestamp: Option<String>,
}
