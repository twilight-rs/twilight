use serde_repr::{Deserialize_repr, Serialize_repr};
/*
 * # InteractionType
 *
 * | Name               | Value |
 * |--------------------|-------|
 * | Ping               | 1     |
 * | ApplicationCommand | 2     |
 */
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}
