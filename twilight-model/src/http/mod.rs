//! Models used when sending data to Discord.

pub mod attachment;
pub mod interaction;
pub mod permission_overwrite;

mod reaction_type;

pub use self::reaction_type::RequestReactionType;
