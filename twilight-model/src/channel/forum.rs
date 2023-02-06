use crate::id::{
    marker::{EmojiMarker, TagMarker},
    Id,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

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
/// [forum]: super::ChannelType::GUILD_FORUM
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ForumLayout(u8);

impl ForumLayout {
    /// Display posts as a collection of tiles.
    pub const GALLERY_VIEW: Self = Self::new(2);

    /// Display posts as a list.
    pub const LIST_VIEW: Self = Self::new(1);

    /// No default has been set for the forum channel.
    pub const NOT_SET: Self = Self::new(0);

    /// Create a new forum layout from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`GALLERY_VIEW`][`Self::GALLERY_VIEW`].
    pub const fn new(forum_layout: u8) -> Self {
        Self(forum_layout)
    }

    /// Retrieve the value of the forum layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::forum::ForumLayout;
    ///
    /// assert_eq!(1, ForumLayout::LIST_VIEW.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::GALLERY_VIEW => "GALLERY_VIEW",
            Self::LIST_VIEW => "LIST_VIEW",
            Self::NOT_SET => "NOT_SET",
            _ => return None,
        })
    }
}

impl Debug for ForumLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("ForumLayout")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("ForumLayout").field(&self.0).finish()
        }
    }
}

impl From<u8> for ForumLayout {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<ForumLayout> for u8 {
    fn from(value: ForumLayout) -> Self {
        value.get()
    }
}

/// Layout of a [channel] that is a [forum].
///
/// [channel]: super::Channel
/// [forum]: super::ChannelType::GUILD_FORUM
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ForumSortOrder(u8);

impl ForumSortOrder {
    /// Sort forum posts by activity.
    pub const LATEST_ACTIVITY: Self = Self::new(0);

    /// Sort forum posts by creation time (from most recent to oldest).
    pub const CREATION_DATE: Self = Self::new(1);

    /// Create a new forum sort order from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`CREATION_DATE`][`Self::CREATION_DATE`].
    pub const fn new(forum_sort_order: u8) -> Self {
        Self(forum_sort_order)
    }

    /// Retrieve the value of the forum sort order.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::forum::ForumSortOrder;
    ///
    /// assert_eq!(0, ForumSortOrder::LATEST_ACTIVITY.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::CREATION_DATE => "CREATION_DATE",
            Self::LATEST_ACTIVITY => "LATEST_ACTIVITY",
            _ => return None,
        })
    }
}

impl Debug for ForumSortOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("ForumSortOrder")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("ForumSortOrder").field(&self.0).finish()
        }
    }
}

impl From<u8> for ForumSortOrder {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<ForumSortOrder> for u8 {
    fn from(value: ForumSortOrder) -> Self {
        value.get()
    }
}

/// Tag that is able to be applied to a thread in a [`GUILD_FORUM`] [`Channel`].
///
/// May at most contain one of `emoji_id` and `emoji_name`.
///
/// [`Channel`]: super::Channel
/// [`GUILD_FORUM`]: super::ChannelType::GUILD_FORUM
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
            (ForumLayout::NOT_SET, 0, "NOT_SET"),
            (ForumLayout::LIST_VIEW, 1, "LIST_VIEW"),
            (ForumLayout::GALLERY_VIEW, 2, "GALLERY_VIEW"),
        ];

        for (layout, number, name) in MAP {
            assert_eq!(u8::from(*layout), *number);
            assert_eq!(ForumLayout::from(*number), *layout);
            assert_tokens(
                layout,
                &[
                    Token::NewtypeStruct {
                        name: "ForumLayout",
                    },
                    Token::U8(*number),
                ],
            );
            assert_eq!(layout.name(), Some(*name));
        }
    }

    #[test]
    fn forum_sort_order() {
        const MAP: &[(ForumSortOrder, u8, &str)] = &[
            (ForumSortOrder::LATEST_ACTIVITY, 0, "LATEST_ACTIVITY"),
            (ForumSortOrder::CREATION_DATE, 1, "CREATION_DATE"),
        ];

        for (layout, number, name) in MAP {
            assert_eq!(layout.name(), Some(*name));
            assert_eq!(u8::from(*layout), *number);
            assert_eq!(ForumSortOrder::from(*number), *layout);
            assert_tokens(
                layout,
                &[
                    Token::NewtypeStruct {
                        name: "ForumSortOrder",
                    },
                    Token::U8(*number),
                ],
            );
        }
    }

    /// Assert the (de)serialization of a forum tag with an emoji name and no
    /// emoji ID.
    #[test]
    fn forum_tag_emoji_name() {
        let value = ForumTag {
            emoji_id: None,
            emoji_name: Some("emoji".to_owned()),
            id: TAG_ID,
            moderated: true,
            name: "tag".into(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ForumTag",
                    len: 5,
                },
                Token::Str("emoji_id"),
                Token::None,
                Token::Str("emoji_name"),
                Token::Some,
                Token::Str("emoji"),
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
                Token::Some,
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
                Token::U64(0),
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

        serde_test::assert_de_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ForumTag",
                    len: 5,
                },
                Token::Str("emoji_id"),
                Token::Unit,
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
