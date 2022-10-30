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
    /// Conflicts with `emoji_name`.
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
    use super::{DefaultReaction, ForumTag};
    use crate::id::{
        marker::{EmojiMarker, TagMarker},
        Id,
    };
    use serde_test::Token;

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
}
