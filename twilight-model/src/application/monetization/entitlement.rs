use serde::{Deserialize, Serialize};

use crate::{
    id::{
        marker::{ApplicationMarker, EntitlementMarker, GuildMarker, SKUMarker, UserMarker},
        Id,
    },
    util::Timestamp,
};

use super::entitlement_type::EntitlementType;

/// Entitlements in Discord represent that a user or guild has access to a premium offering in your application.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Entitlement {
    /// ID of the parent application.
    pub application_id: Id<ApplicationMarker>,
    /// Not applicable for App Subscriptions. Subscriptions are not consumed and will be `false`
    pub consumed: bool,
    /// Entitlement was deleted.
    pub deleted: bool,
    /// Date at which the entitlement is no longer valid. Not present when using test entitlements.
    pub ends_at: Option<Timestamp>,
    /// ID of the guild that is granted access to the entitlement's sku.
    pub guild_id: Option<Id<GuildMarker>>,
    /// ID of the entitlement.
    pub id: Id<EntitlementMarker>,
    /// Type of entitlement.
    #[serde(rename = "type")]
    pub kind: EntitlementType,
    /// ID of the SKU.
    pub sku_id: Id<SKUMarker>,
    /// Start date at which the entitlement is valid. Not present when using test entitlements.
    pub starts_at: Option<Timestamp>,
    /// ID of the user that is granted access to the entitlement's sku.
    pub user_id: Option<Id<UserMarker>>,
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use serde_test::Token;

    use super::Entitlement;
    use crate::application::monetization::entitlement_type::EntitlementType;
    use crate::id::Id;
    use crate::util::Timestamp;

    #[test]
    fn entitlement() -> Result<(), Box<dyn Error>> {
        let starts_at_str = "2022-09-14T17:00:18.704163+00:00";
        let ends_at_str = "2022-10-14T17:00:21.704163+00:00";
        let starts_at = Timestamp::parse(starts_at_str)?;
        let ends_at = Timestamp::parse(ends_at_str)?;

        let value = Entitlement {
            application_id: Id::new(1),
            consumed: false,
            deleted: false,
            ends_at: ends_at.into(),
            guild_id: Some(Id::new(10)),
            id: Id::new(2),
            kind: EntitlementType::ApplicationSubscription,
            sku_id: Id::new(3),
            starts_at: starts_at.into(),
            user_id: Some(Id::new(42)),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Entitlement",
                    len: 10,
                },
                Token::Str("application_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("consumed"),
                Token::Bool(false),
                Token::Str("deleted"),
                Token::Bool(false),
                Token::Str("ends_at"),
                Token::Some,
                Token::Str(ends_at_str),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("10"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(8),
                Token::Str("sku_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("starts_at"),
                Token::Some,
                Token::Str(starts_at_str),
                Token::Str("user_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("42"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
