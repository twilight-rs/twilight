use crate::id::{
    marker::{EmojiMarker, TagMarker},
    Id,
};
use serde::{Deserialize, Serialize};

/// Emoji to use as the default way to react to a forum post.
///
/// Exactly one of `emoji_id` and `emoji_name` must be set.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DefaultReaction {
    /// ID of custom guild emoji.
    ///
    /// Conflicts with `emoji_name`.
    pub emoji_id: Option<Id<EmojiMarker>>,
    /// Unicode emoji character.
    ///
    /// Conflicts with `emoji_id`.
    pub emoji_name: Option<String>,
}

/// Layout of a [channel] that is a [forum].
///
/// [channel]: super::Channel
/// [forum]: super::ChannelType::GuildForum
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum ForumLayout {
    /// Display posts as a collection of tiles.
    GalleryView,
    /// Display posts as a list.
    ListView,
    /// No default has been set for the forum channel.
    NotSet,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl ForumLayout {
    pub const fn name(self) -> &'static str {
        match self {
            Self::ListView => "ListView",
            Self::NotSet => "NotSet",
            Self::GalleryView => "GalleryView",
            Self::Unknown(_) => "Unknown",
        }
    }
}

impl From<u8> for ForumLayout {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NotSet,
            1 => Self::ListView,
            2 => Self::GalleryView,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<ForumLayout> for u8 {
    fn from(value: ForumLayout) -> Self {
        match value {
            ForumLayout::NotSet => 0,
            ForumLayout::ListView => 1,
            ForumLayout::GalleryView => 2,
            ForumLayout::Unknown(unknown) => unknown,
        }
    }
}

/// Layout of a [channel] that is a [forum].
///
/// [channel]: super::Channel
/// [forum]: super::ChannelType::GuildForum
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum ForumSortOrder {
    /// Sort forum posts by creation time (from most recent to oldest).
    CreationDate,
    /// Sort forum posts by activity.
    LatestActivity,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl ForumSortOrder {
    pub const fn name(self) -> &'static str {
        match self {
            Self::CreationDate => "CreationDate",
            Self::LatestActivity => "LatestActivity",
            Self::Unknown(_) => "Unknown",
        }
    }
}

impl From<u8> for ForumSortOrder {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::LatestActivity,
            1 => Self::CreationDate,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<ForumSortOrder> for u8 {
    fn from(value: ForumSortOrder) -> Self {
        match value {
            ForumSortOrder::LatestActivity => 0,
            ForumSortOrder::CreationDate => 1,
            ForumSortOrder::Unknown(unknown) => unknown,
        }
    }
}

/// Tag that is able to be applied to a thread in a [`GuildForum`] [`Channel`].
///
/// May at most contain one of `emoji_id` and `emoji_name`.
///
/// [`Channel`]: super::Channel
/// [`GuildForum`]: super::ChannelType::GuildForum
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ForumTag {
    /// ID of custom guild emoji.
    ///
    /// Some guilds can have forum tags that have an ID of 0; if this is the
    /// case, then the emoji ID is `None`.
    ///
    /// Conflicts with `emoji_name`.
    #[serde(with = "crate::visitor::zeroable_id")]
    pub emoji_id: Option<Id<EmojiMarker>>,
    /// Unicode emoji character.
    ///
    /// Conflicts with `emoji_name`.
    pub emoji_name: Option<String>,
    /// ID of the tag.
    pub id: Id<TagMarker>,
    /// Whether the tag can only be added or removed by [`Member`]s with the
    /// [`MANAGE_THREADS`] permission.
    ///
    /// [`MANAGE_THREADS`]: crate::guild::Permissions::MANAGE_THREADS
    /// [`Member`]: crate::guild::Member
    pub moderated: bool,
    /// Name of the tag (0--20 characters).
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::{DefaultReaction, ForumLayout, ForumSortOrder, ForumTag};
    use crate::id::{
        marker::{EmojiMarker, TagMarker},
        Id,
    };
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        ForumLayout: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
    assert_impl_all!(
        ForumSortOrder: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    const EMOJI_ID: Id<EmojiMarker> = Id::new(1);
    const TAG_ID: Id<TagMarker> = Id::new(2);

    #[test]
    fn default_reaction() {
        let value = DefaultReaction {
            emoji_id: None,
            emoji_name: Some("name".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "DefaultReaction",
                    len: 2,
                },
                Token::Str("emoji_id"),
                Token::None,
                Token::Str("emoji_name"),
                Token::Some,
                Token::Str("name"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn forum_layout() {
        const MAP: &[(ForumLayout, u8, &str)] = &[
            (ForumLayout::NotSet, 0, "NotSet"),
            (ForumLayout::ListView, 1, "ListView"),
            (ForumLayout::GalleryView, 2, "GalleryView"),
            (ForumLayout::Unknown(3), 3, "Unknown"),
        ];

        for (layout, number, name) in MAP {
            assert_eq!(layout.name(), *name);
            assert_eq!(u8::from(*layout), *number);
            assert_eq!(ForumLayout::from(*number), *layout);
            assert_tokens(layout, &[Token::U8(*number)]);
        }
    }

    #[test]
    fn forum_sort_order() {
        const MAP: &[(ForumSortOrder, u8, &str)] = &[
            (ForumSortOrder::LatestActivity, 0, "LatestActivity"),
            (ForumSortOrder::CreationDate, 1, "CreationDate"),
            (ForumSortOrder::Unknown(100), 100, "Unknown"),
        ];

        for (layout, number, name) in MAP {
            assert_eq!(layout.name(), *name);
            assert_eq!(u8::from(*layout), *number);
            assert_eq!(ForumSortOrder::from(*number), *layout);
            assert_tokens(layout, &[Token::U8(*number)]);
        }
    }

    #[test]
    fn forum_tag() {
        let value = ForumTag {
            emoji_id: Some(EMOJI_ID),
            emoji_name: None,
            id: TAG_ID,
            moderated: false,
            name: "other".into(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ForumTag",
                    len: 5,
                },
                Token::Str("emoji_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("emoji_name"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("moderated"),
                Token::Bool(false),
                Token::Str("name"),
                Token::Str("other"),
                Token::StructEnd,
            ],
        );
    }

    /// Assert that an emoji ID can be deserialized from a string value of "0".
    ///
    /// This is a bug on Discord's end that has consistently been causing issues
    /// for Twilight users.
    #[test]
    fn forum_tag_emoji_id_zero() {
        let value = ForumTag {
            emoji_id: None,
            emoji_name: None,
            id: TAG_ID,
            moderated: true,
            name: "tag".into(),
        };

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ForumTag",
                    len: 5,
                },
                Token::Str("emoji_id"),
                Token::Str("0"),
                Token::Str("emoji_name"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("moderated"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("tag"),
                Token::StructEnd,
            ],
        );
    }
}
