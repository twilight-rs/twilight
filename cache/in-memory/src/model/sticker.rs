use serde::Serialize;
use twilight_model::{
    channel::message::{
        sticker::{StickerFormatType, StickerId, StickerPackId, StickerType},
        Sticker,
    },
    id::{GuildId, UserId},
};

/// Representation of a cached [`Sticker`].
///
/// [`Sticker`]: twilight_model::channel::message::sticker::Sticker
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CachedSticker {
    /// Whether the sticker is available.
    pub available: bool,
    /// Description of the sticker.
    pub description: String,
    /// Format type.
    pub format_type: StickerFormatType,
    /// ID of the guild that owns the sticker.
    pub guild_id: Option<GuildId>,
    /// Unique ID of the sticker.
    pub id: StickerId,
    /// Kind of sticker.
    pub kind: StickerType,
    /// Name of the sticker.
    pub name: String,
    /// Unique ID of the pack the sticker is in.
    pub pack_id: Option<StickerPackId>,
    /// Sticker's sort order within a pack.
    pub sort_value: Option<u64>,
    /// CSV list of tags the sticker is assigned to, if any.
    pub tags: String,
    /// ID of the user that uploaded the sticker.
    pub user_id: Option<UserId>,
}

impl PartialEq<Sticker> for CachedSticker {
    fn eq(&self, other: &Sticker) -> bool {
        self.available == other.available
            && self.description == other.description
            && self.format_type == other.format_type
            && self.guild_id == other.guild_id
            && self.id == other.id
            && self.kind == other.kind
            && self.name == other.name
            && self.pack_id == other.pack_id
            && self.sort_value == other.sort_value
            && self.tags == other.tags
            && self.user_id == other.user.as_ref().map(|user| user.id)
    }
}

#[cfg(test)]
mod tests {
    use super::CachedSticker;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;
    use twilight_model::{
        channel::message::{
            sticker::{StickerFormatType, StickerId, StickerPackId, StickerType},
            Sticker,
        },
        id::{GuildId, UserId},
        user::{PremiumType, User, UserFlags},
    };

    assert_fields!(
        CachedSticker: available,
        description,
        format_type,
        guild_id,
        id,
        kind,
        name,
        pack_id,
        sort_value,
        tags,
        user_id
    );
    assert_impl_all!(CachedSticker: Clone, Debug, Eq, PartialEq);

    #[test]
    fn test_eq_sticker() {
        let sticker = Sticker {
            available: true,
            description: "sticker".into(),
            format_type: StickerFormatType::Png,
            guild_id: Some(GuildId(1)),
            id: StickerId(2),
            kind: StickerType::Guild,
            name: "stick".into(),
            pack_id: Some(StickerPackId(3)),
            sort_value: Some(1),
            tags: "foo,bar,baz".into(),
            user: Some(User {
                avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                bot: false,
                discriminator: "0001".to_owned(),
                email: Some("address@example.com".to_owned()),
                flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
                id: UserId(1),
                locale: Some("en-us".to_owned()),
                mfa_enabled: Some(true),
                name: "test".to_owned(),
                premium_type: Some(PremiumType::Nitro),
                public_flags: Some(UserFlags::EARLY_SUPPORTER | UserFlags::VERIFIED_BOT_DEVELOPER),
                system: Some(true),
                verified: Some(true),
            }),
        };

        let cached = CachedSticker {
            available: true,
            description: "sticker".into(),
            format_type: StickerFormatType::Png,
            guild_id: Some(GuildId(1)),
            id: StickerId(2),
            kind: StickerType::Guild,
            name: "stick".into(),
            pack_id: Some(StickerPackId(3)),
            sort_value: Some(1),
            tags: "foo,bar,baz".into(),
            user_id: Some(UserId(1)),
        };

        assert_eq!(cached, sticker);
    }
}
