use crate::id::Id;
use crate::id::marker::GuildMarker;
use crate::util::ImageHash;
use serde::{Deserialize, Serialize};

/// The Primary Guild data (also known as guild tag(s)) provided with the [`User`] object.
///
/// A primary guild tag has a 2-4 length text content and a badge, in Discord laid out as \[BADGE]\[TAG].
///
/// [`User`]: twilight_model::user::User
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PrimaryGuild {
    /// The id of the user's primary guild.
    pub identity_guild_id: Option<Id<GuildMarker>>,
    /// Whether the user is displaying their primary guild's server tag.
    ///
    /// Is `None` if the tag was non-manually cleared by the user. (e.g. user left server, server stopped supporting guild tags)
    pub identity_enabled: Option<bool>,
    /// The text content of the guild tag, within 2-4 characters.
    pub tag: Option<String>,
    /// The guild tag's badge hash.
    pub badge: Option<ImageHash>,
}
