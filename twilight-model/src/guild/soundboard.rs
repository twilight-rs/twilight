use serde::{Deserialize, Serialize};

use crate::{
    id::{
        marker::{EmojiMarker, GuildMarker, SoundboardSoundMarker},
        Id,
    },
    user::User,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SoundboardSound {
    pub available: bool,
    pub emoji_id: Option<Id<EmojiMarker>>,
    pub emoji_name: Option<String>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub name: String,
    pub sound_id: Id<SoundboardSoundMarker>,
    pub user: Option<User>,
    pub volume: f64,
}

#[cfg(test)]
mod tests {
    // TODO: stub
}
