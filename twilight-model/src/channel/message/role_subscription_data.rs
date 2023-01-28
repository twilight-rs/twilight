use crate::id::{marker::RoleSubscriptionSkuMarker, Id};
use serde::{Deserialize, Serialize};

/// Information about a role subscription that created a [`Message`].
///
/// [`Message`]: super::Message
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RoleSubscriptionData {
    /// Whether this notification is for a renewal rather than a new purchase.
    pub is_renewal: bool,
    /// ID of the SKU and listing that the user is subscribed to.
    pub role_subscription_listing_id: Id<RoleSubscriptionSkuMarker>,
    /// Name of the tier that the user is subscribed to.
    pub tier_name: String,
    /// Cumulative number of months that the user has been subscribed for.
    pub total_months_subscribed: u16,
}

#[cfg(test)]
mod tests {
    use super::RoleSubscriptionData;
    use crate::id::Id;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        RoleSubscriptionData: is_renewal,
        role_subscription_listing_id,
        tier_name,
        total_months_subscribed
    );
    assert_impl_all!(
        RoleSubscriptionData: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn serde() {
        let value = RoleSubscriptionData {
            is_renewal: true,
            role_subscription_listing_id: Id::new(1),
            tier_name: "sparkle".to_owned(),
            total_months_subscribed: 4,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "RoleSubscriptionData",
                    len: 4,
                },
                Token::Str("is_renewal"),
                Token::Bool(true),
                Token::Str("role_subscription_listing_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("tier_name"),
                Token::Str("sparkle"),
                Token::Str("total_months_subscribed"),
                Token::U16(4),
                Token::StructEnd,
            ],
        );
    }
}
