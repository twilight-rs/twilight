//! Mutable `bitflags` operations are not `const`, but since immutable
//! operations - i.e. `bits` and `from_bits_truncate` - are `const`, we can just
//! create wrapping operations over them.

use twilight_model::guild::Permissions;

/// Insert permissions into another.
pub const fn insert(permissions: Permissions, other: Permissions) -> Permissions {
    Permissions::from_bits_truncate(permissions.bits() | other.bits())
}

/// Remove permissions from another.
pub const fn remove(permissions: Permissions, other: Permissions) -> Permissions {
    Permissions::from_bits_truncate(permissions.bits() & !other.bits())
}

#[cfg(test)]
mod tests {
    use twilight_model::guild::Permissions;

    #[test]
    fn insert() {
        let actual = super::insert(
            Permissions::KICK_MEMBERS,
            Permissions::BAN_MEMBERS | Permissions::CONNECT,
        );
        let expected = Permissions::BAN_MEMBERS | Permissions::CONNECT | Permissions::KICK_MEMBERS;

        assert_eq!(actual, expected);
    }

    #[test]
    fn insert_duplicate() {
        let expected = Permissions::BAN_MEMBERS | Permissions::KICK_MEMBERS;
        let actual = super::insert(expected, Permissions::KICK_MEMBERS);

        assert_eq!(actual, expected);
    }

    #[test]
    fn remove() {
        let actual = super::remove(
            Permissions::BAN_MEMBERS | Permissions::KICK_MEMBERS,
            Permissions::BAN_MEMBERS,
        );

        assert_eq!(actual, Permissions::KICK_MEMBERS);
    }

    #[test]
    fn remove_nonexistent() {
        let actual = super::remove(Permissions::KICK_MEMBERS, Permissions::BAN_MEMBERS);

        assert_eq!(actual, Permissions::KICK_MEMBERS);
    }
}
