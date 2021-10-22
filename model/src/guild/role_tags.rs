use crate::{
    id::{IntegrationId, UserId},
    util::is_false,
};
use serde::{Deserialize, Serialize};

/// The role tags' `premium_subscriber` field is tricky. It's an optional null.
///
/// If the field is present, then the value is null, meaning that the role is a
/// premium subscriber. If the field is not present, it means that the role is
/// *not* a premium subscriber.
mod premium_subscriber {
    use serde::{
        de::{Deserializer, Error as DeError, Visitor},
        ser::Serializer,
    };
    use std::fmt::{Formatter, Result as FmtResult};

    struct PremiumSubscriberVisitor;

    impl<'de> Visitor<'de> for PremiumSubscriberVisitor {
        type Value = bool;

        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("null")
        }

        fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(true)
        }

        // `visit_none` is used by `serde_json` when a present `null` value is
        // encountered, but other implementations - such as `simd_json` - may
        // use `visit_unit` instead.
        fn visit_unit<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(true)
        }
    }

    // Clippy will say this bool can be taken by value, but we need it to be
    // passed by reference because that's what serde does.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(_: &bool, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
        deserializer.deserialize_option(PremiumSubscriberVisitor)
    }
}

/// Tags that a [`Role`] has.
///
/// [`Role`]: super::Role
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleTags {
    /// ID of the bot the role belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<UserId>,
    /// ID of the integration the role belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<IntegrationId>,
    /// Whether this is the guild's premium subscriber role.
    #[serde(default, skip_serializing_if = "is_false", with = "premium_subscriber")]
    pub premium_subscriber: bool,
}

#[cfg(test)]
mod tests {
    use super::RoleTags;
    use crate::id::{IntegrationId, UserId};
    use serde_test::Token;

    #[test]
    fn test_role_tags_all() {
        let tags = RoleTags {
            bot_id: Some(UserId::new(1).expect("non zero")),
            integration_id: Some(IntegrationId::new(2).expect("non zero")),
            premium_subscriber: true,
        };

        serde_test::assert_tokens(
            &tags,
            &[
                Token::Struct {
                    name: "RoleTags",
                    len: 3,
                },
                Token::Str("bot_id"),
                Token::Some,
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::Str("integration_id"),
                Token::Some,
                Token::NewtypeStruct {
                    name: "IntegrationId",
                },
                Token::Str("2"),
                Token::Str("premium_subscriber"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    /// Test that if all fields are None and `premium_subscriber` is false, then
    /// serialize back into the source payload (where all fields are not
    /// present).
    #[test]
    fn test_role_tags_none() {
        let tags = RoleTags {
            bot_id: None,
            integration_id: None,
            premium_subscriber: false,
        };

        serde_test::assert_tokens(
            &tags,
            &[
                Token::Struct {
                    name: "RoleTags",
                    len: 0,
                },
                Token::StructEnd,
            ],
        );
    }
}
