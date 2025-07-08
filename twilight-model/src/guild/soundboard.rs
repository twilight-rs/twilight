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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<Id<EmojiMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    pub name: String,
    pub sound_id: Id<SoundboardSoundMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    pub volume: f64,
}

#[cfg(test)]
mod tests {
    use super::SoundboardSound;
    use std::fmt::Debug;
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};

    assert_fields!(
        SoundboardSound: available,
        emoji_id,
        emoji_name,
        guild_id,
        name,
        sound_id,
        user,
        volume
    );

    assert_impl_all!(
        SoundboardSound: Clone, Debug, Deserialize<'static>, PartialEq, Serialize
    );

    #[test]
    fn soundboard_sound() {
        // TODO: stub
    }
}
