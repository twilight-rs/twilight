use crate::{
    id::{
        marker::{IntegrationMarker, RoleSubscriptionSkuMarker, UserMarker},
        Id,
    },
    util::is_false,
};
use serde::{Deserialize, Serialize};

/// Tags that a [`Role`] has.
///
/// [`Role`]: super::Role
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct RoleTags {
    /// Whether this role is available for purchase.
    #[serde(
        default,
        skip_serializing_if = "is_false",
        with = "crate::visitor::null_boolean"
    )]
    pub available_for_purchase: bool,
    /// ID of the bot the role belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub bot_id: Option<Id<UserMarker>>,
    /// Whether this role is a guild's linked role.
    #[serde(
        default,
        skip_serializing_if = "is_false",
        with = "crate::visitor::null_boolean"
    )]
    pub guild_connections: bool,
    /// ID of the integration the role belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub integration_id: Option<Id<IntegrationMarker>>,
    /// ID of the role's subscription SKU and listing.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub subscription_listing_id: Option<Id<RoleSubscriptionSkuMarker>>,
    /// Whether this is the guild's premium subscriber role.
    #[serde(
        default,
        skip_serializing_if = "is_false",
        with = "crate::visitor::null_boolean"
    )]
    pub premium_subscriber: bool,
}

#[cfg(test)]
mod tests {
    use super::RoleTags;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn bot() {
        let tags = RoleTags {
            available_for_purchase: false,
            bot_id: Some(Id::new(1)),
            guild_connections: false,
            integration_id: Some(Id::new(2)),
            premium_subscriber: false,
            subscription_listing_id: None,
        };

        serde_test::assert_tokens(
            &tags,
            &[
                Token::Struct {
                    name: "RoleTags",
                    len: 2,
                },
                Token::Str("bot_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("integration_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn premium_subscriber() {
        let tags = RoleTags {
            available_for_purchase: false,
            bot_id: None,
            guild_connections: false,
            integration_id: None,
            premium_subscriber: true,
            subscription_listing_id: None,
        };

        serde_test::assert_tokens(
            &tags,
            &[
                Token::Struct {
                    name: "RoleTags",
                    len: 1,
                },
                Token::Str("premium_subscriber"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn subscription() {
        let tags = RoleTags {
            available_for_purchase: true,
            bot_id: None,
            guild_connections: false,
            integration_id: Some(Id::new(1)),
            subscription_listing_id: Some(Id::new(2)),
            premium_subscriber: false,
        };

        serde_test::assert_tokens(
            &tags,
            &[
                Token::Struct {
                    name: "RoleTags",
                    len: 3,
                },
                Token::Str("available_for_purchase"),
                Token::None,
                Token::Str("integration_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("subscription_listing_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }

    /// Test that if all fields are None and the optional null fields are false,
    /// then serialize back into the source payload (where all fields are not
    /// present).
    #[test]
    fn none() {
        let tags = RoleTags {
            available_for_purchase: false,
            bot_id: None,
            guild_connections: false,
            integration_id: None,
            premium_subscriber: false,
            subscription_listing_id: None,
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
