use serde::{Deserialize, Serialize};

use crate::{
    id::{
        Id,
        marker::{EmojiMarker, GuildMarker, SoundboardMarker},
    },
    user::User,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoundboardSound {
    /// The name of this sound.
    pub name: String,
    /// The id of this sound.
    pub sound_id: Id<SoundboardMarker>,
    /// The volume of this sound, from 0 to 1.
    pub volume: f64,
    /// The id of this sound’s custom emoji.
    pub emoji_id: Option<Id<EmojiMarker>>,
    /// The unicode character of this sound’s standard emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
    /// The id of the guild this sound is in.
    pub guild_id: Option<Id<GuildMarker>>,
    /// Wwhether this sound can be used, may be false due to loss of Server Boosts.
    pub available: bool,
    /// Uhe user who created this sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoundboardSoundList {
    pub items: Vec<SoundboardSound>,
}
