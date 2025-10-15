use super::{RoleColors, RoleFlags, RoleTags};
use crate::{
    guild::Permissions,
    id::{marker::RoleMarker, Id},
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Role {
    #[deprecated(
        since = "0.17.0",
        note = "Deprecated by Discord, use `colors` instead."
    )]
    pub color: u32,
    pub colors: RoleColors,
    pub hoist: bool,
    /// Icon image hash.
    ///
    /// Present if the guild has the `ROLE_ICONS` feature and if the role has
    /// one.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
    pub id: Id<RoleMarker>,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub permissions: Permissions,
    pub position: i64,
    /// Flags for this role.
    pub flags: RoleFlags,
    /// Tags about the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<RoleTags>,
    /// Icon unicode emoji.
    ///
    /// Present if the guild has the `ROLE_ICONS` feature and if the role has
    /// one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,
}

impl Ord for Role {
    /// Compare two roles to each other using their position and ID.
    ///
    /// Roles are primarily ordered by their position in descending order. For example,
    /// a role with a position of 17 is considered a higher role than one with a
    /// position of 12.
    ///
    /// Discord does not guarantee that role positions are positive, unique, or contiguous. When
    /// two or more roles have the same position then the order is based on the roles' IDs in
    /// ascending order. For example, given two roles with positions of 10 then a role
    /// with an ID of 1 would be considered a higher role than one with an ID of 20.
    ///
    /// ### Examples
    ///
    /// Compare the position of two roles:
    ///
    /// ```
    /// # use twilight_model::{guild::{Permissions, Role, RoleFlags}, id::Id};
    /// # use std::cmp::Ordering;
    /// let role_a = Role {
    ///     id: Id::new(123),
    ///     position: 12,
    /// #   color: 0,
    /// #   hoist: true,
    /// #   icon: None,
    /// #   managed: false,
    /// #   mentionable: true,
    /// #   name: "test".to_owned(),
    /// #   permissions: Permissions::ADMINISTRATOR,
    /// #   flags: RoleFlags::empty(),
    /// #   tags: None,
    /// #   unicode_emoji: None,
    ///     // ...
    /// };
    /// let role_b = Role {
    ///     id: Id::new(456),
    ///     position: 13,
    /// #   color: 0,
    /// #   hoist: true,
    /// #   icon: None,
    /// #   managed: false,
    /// #   mentionable: true,
    /// #   name: "test".to_owned(),
    /// #   permissions: Permissions::ADMINISTRATOR,
    /// #   flags: RoleFlags::empty(),
    /// #   tags: None,
    /// #   unicode_emoji: None,
    ///     // ...
    /// };
    /// assert_eq!(Ordering::Less, role_a.cmp(&role_b));
    /// assert_eq!(Ordering::Greater, role_b.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_a.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_b.cmp(&role_b));
    /// ```
    ///
    /// Compare the position of two roles with the same position:
    ///
    /// ```
    /// # use twilight_model::{guild::{Permissions, Role, RoleFlags}, id::Id};
    /// # use std::cmp::Ordering;
    /// let role_a = Role {
    ///     id: Id::new(123),
    ///     position: 12,
    /// #   color: 0,
    /// #   hoist: true,
    /// #   icon: None,
    /// #   managed: false,
    /// #   mentionable: true,
    /// #   name: "test".to_owned(),
    /// #   permissions: Permissions::ADMINISTRATOR,
    /// #   flags: RoleFlags::empty(),
    /// #   tags: None,
    /// #   unicode_emoji: None,
    /// };
    /// let role_b = Role {
    ///     id: Id::new(456),
    ///     position: 12,
    /// #   color: 0,
    /// #   hoist: true,
    /// #   icon: None,
    /// #   managed: false,
    /// #   mentionable: true,
    /// #   name: "test".to_owned(),
    /// #   permissions: Permissions::ADMINISTRATOR,
    /// #   flags: RoleFlags::empty(),
    /// #   tags: None,
    /// #   unicode_emoji: None,
    /// };
    /// assert_eq!(Ordering::Greater, role_a.cmp(&role_b));
    /// assert_eq!(Ordering::Less, role_b.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_a.cmp(&role_a));
    /// assert_eq!(Ordering::Equal, role_b.cmp(&role_b));
    /// ```
    fn cmp(&self, other: &Self) -> Ordering {
        self.position
            .cmp(&other.position)
            .then(other.id.get().cmp(&self.id.get()))
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::{Permissions, Role};
    use crate::{
        guild::{RoleColors, RoleFlags},
        id::Id,
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        Role: color,
        hoist,
        icon,
        id,
        managed,
        mentionable,
        name,
        permissions,
        position,
        tags,
        unicode_emoji
    );

    assert_impl_all!(
        Role: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize
    );

    #[test]
    fn role() {
        let role = Role {
            color: 0,
            colors: RoleColors {
                primary_color: 0,
                secondary_color: None,
                tertiary_color: None,
            },
            hoist: true,
            icon: None,
            id: Id::new(123),
            managed: false,
            mentionable: true,
            name: "test".to_owned(),
            permissions: Permissions::ADMINISTRATOR,
            position: 12,
            flags: RoleFlags::IN_PROMPT,
            tags: None,
            unicode_emoji: None,
        };

        serde_test::assert_tokens(
            &role,
            &[
                Token::Struct {
                    name: "Role",
                    len: 9,
                },
                Token::Str("color"),
                Token::U32(0),
                Token::Str("colors"),
                Token::Struct {
                    name: "RoleColors",
                    len: 1,
                },
                Token::Str("primary_color"),
                Token::U64(0),
                Token::StructEnd,
                Token::Str("hoist"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("mentionable"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("permissions"),
                Token::Str("8"),
                Token::Str("position"),
                Token::I64(12),
                Token::Str("flags"),
                Token::U64(1),
                Token::StructEnd,
            ],
        );
    }
}
