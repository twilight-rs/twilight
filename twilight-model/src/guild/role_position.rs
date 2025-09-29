use crate::id::{Id, marker::RoleMarker};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
/// Data used to update the positions of roles.
pub struct RolePosition {
    /// Role identifier.
    pub id: Id<RoleMarker>,
    /// Sorting position of the role.
    pub position: u64,
}

#[cfg(test)]
mod tests {
    use super::{Id, RolePosition};
    use serde_test::Token;

    #[test]
    fn role_position() {
        let role_position = RolePosition {
            id: Id::new(123),
            position: 12,
        };

        serde_test::assert_tokens(
            &role_position,
            &[
                Token::Struct {
                    name: "RolePosition",
                    len: 2,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("123"),
                Token::Str("position"),
                Token::U64(12),
                Token::StructEnd,
            ],
        );
    }
}
