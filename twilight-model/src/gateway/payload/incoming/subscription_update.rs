use crate::application::monetization::Subscription;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Sent when a Subscription for a Premium App has been updated.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SubscriptionUpdate(pub Subscription);

impl Deref for SubscriptionUpdate {
    type Target = Subscription;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SubscriptionUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
