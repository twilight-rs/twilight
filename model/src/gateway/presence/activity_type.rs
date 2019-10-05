#[cfg_attr(
    feature = "serde-support",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ActivityType {
    Playing = 0,
    Streaming = 1,
    Listening = 2,
}

impl Default for ActivityType {
    fn default() -> Self {
        Self::Playing
    }
}
