use crate::{
    channel::message::allowed_mentions::{AllowedMentions, ParseTypes},
    id::{RoleId, UserId},
};

#[derive(Clone, Default, Debug, Eq, PartialEq)]
pub struct AllowedMentionsBuilder(AllowedMentions);

impl AllowedMentionsBuilder {
    /// Create a new [`AllowedMentionsBuilder`].
    pub const fn new() -> Self {
        Self(AllowedMentions {
            parse: Vec::new(),
            users: Vec::new(),
            roles: Vec::new(),
            replied_user: false,
        })
    }

    /// Allow parsing of `@everyone`.
    pub fn everyone(mut self) -> Self {
        self.0.parse.push(ParseTypes::Everyone);

        self
    }

    /// When replying, whether to mention the target user.
    pub const fn replied_user(mut self) -> Self {
        self.0.replied_user = true;

        self
    }

    /// Allow parsing of all roles.
    pub fn roles(mut self) -> Self {
        self.0.parse.push(ParseTypes::Roles);

        self
    }

    /// Allow parsing of specific [`RoleId`]s.
    ///
    /// [`roles`] and this method are mutually exclusive. The builder will favor
    /// specific role ids.
    ///
    /// [`roles`]: Self::roles
    pub fn role_ids(mut self, role_ids: impl IntoIterator<Item = RoleId>) -> Self {
        self.0.roles.extend(role_ids);

        self
    }

    /// Allow parsing of all users.
    pub fn users(mut self) -> Self {
        self.0.parse.push(ParseTypes::Users);

        self
    }

    /// Allow parsing of  specific [`UserId`]s.
    ///
    /// [`users`] and this method are mutually exclusive. The builder will favor
    /// specific user ids.
    ///
    /// [`users`]: Self::users
    pub fn user_ids(mut self, user_ids: impl IntoIterator<Item = UserId>) -> Self {
        self.0.users.extend(user_ids);

        self
    }

    /// Build the [`AllowedMentions`] struct.
    ///
    /// Note: This clears possible validation errors by removing data.  For
    /// example, if both `users` and `user_ids` are called, the effect of
    /// `users` will be nullified and only the users specified with `user_ids`
    /// will be built.
    pub fn build(mut self) -> AllowedMentions {
        if !self.0.users.is_empty() {
            self.0.parse = self
                .0
                .parse
                .into_iter()
                .filter(|t| *t != ParseTypes::Users)
                .collect();
        }

        if !self.0.roles.is_empty() {
            self.0.parse = self
                .0
                .parse
                .into_iter()
                .filter(|t| *t != ParseTypes::Roles)
                .collect();
        }

        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{AllowedMentions, ParseTypes},
        AllowedMentionsBuilder,
    };
    use crate::id::{RoleId, UserId};

    #[test]
    fn test_max_mentioned() {
        let value = AllowedMentionsBuilder::new()
            .everyone()
            .replied_user()
            .users()
            .roles()
            .build();

        assert_eq!(
            value,
            AllowedMentions {
                parse: vec![ParseTypes::Everyone, ParseTypes::Users, ParseTypes::Roles],
                users: vec![],
                roles: vec![],
                replied_user: true
            },
        );
    }

    #[test]
    fn test_validation() {
        let value = AllowedMentionsBuilder::new()
            .users()
            .user_ids(vec![
                UserId::new(100).expect("non zero"),
                UserId::new(200).expect("non zero"),
            ])
            .roles()
            .role_ids(vec![RoleId::new(300).expect("non zero")])
            .build();

        assert_eq!(
            value,
            AllowedMentions {
                parse: vec![],
                users: vec![
                    UserId::new(100).expect("non zero"),
                    UserId::new(200).expect("non zero")
                ],
                roles: vec![RoleId::new(300).expect("non zero")],
                replied_user: false,
            },
        );
    }
}
