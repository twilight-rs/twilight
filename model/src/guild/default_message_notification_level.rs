use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr)]
#[repr(u8)]
pub enum DefaultMessageNotificationLevel {
    All = 0,
    Mentions = 1,
}
