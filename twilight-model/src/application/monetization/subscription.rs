use super::SubscriptionStatus;
use crate::{
    id::{
        Id,
        marker::{EntitlementMarker, SkuMarker, SubscriptionMarker, UserMarker},
    },
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

/// Subscriptions in Discord represent a user making recurring payments for at
/// least one SKU over an ongoing period. Successful payments grant the user
/// access to entitlements associated with the SKU.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    /// When the subscription was canceled.
    pub canceled_at: Option<Timestamp>,
    /// ISO3166-1 alpha-2 country code of the payment source used to purchase
    /// the subscription. Missing unless queried with a private OAuth scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// End of the current subscription period.
    pub current_period_end: Timestamp,
    /// Start of the current subscription period.
    pub current_period_start: Timestamp,
    /// List of entitlements granted for this subscription.
    pub entitlement_ids: Vec<Id<EntitlementMarker>>,
    /// ID of the subscription.
    pub id: Id<SubscriptionMarker>,
    /// List of SKUs that this user will be subscribed to at renewal.
    pub renewal_sku_ids: Option<Vec<Id<SkuMarker>>>,
    /// List of SKUs subscribed to.
    pub sku_ids: Vec<Id<SkuMarker>>,
    /// Current status of the subscription.
    pub status: SubscriptionStatus,
    /// ID of the user who is subscribed.
    pub user_id: Id<UserMarker>,
}

#[cfg(test)]
mod tests {
    use super::{Subscription, SubscriptionStatus};
    use crate::id::Id;
    use crate::util::Timestamp;
    use serde_test::Token;
    use std::error::Error;

    #[test]
    fn entitlement() -> Result<(), Box<dyn Error>> {
        let value = Subscription {
            canceled_at: None,
            country: None,
            current_period_end: Timestamp::parse("2024-09-27T19:48:44.406602+00:00")?,
            current_period_start: Timestamp::parse("2024-08-27T19:48:44.406602+00:00")?,
            entitlement_ids: Vec::new(),
            id: Id::new(1),
            renewal_sku_ids: Some(Vec::new()),
            sku_ids: Vec::from([Id::new(2)]),
            status: SubscriptionStatus::Active,
            user_id: Id::new(3),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Subscription",
                    len: 9,
                },
                Token::Str("canceled_at"),
                Token::None,
                Token::Str("current_period_end"),
                Token::Str("2024-09-27T19:48:44.406602+00:00"),
                Token::Str("current_period_start"),
                Token::Str("2024-08-27T19:48:44.406602+00:00"),
                Token::Str("entitlement_ids"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("renewal_sku_ids"),
                Token::Some,
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("sku_ids"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::SeqEnd,
                Token::Str("status"),
                Token::U8(0),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
