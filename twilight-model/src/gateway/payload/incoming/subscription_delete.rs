use crate::application::monetization::Subscription;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Sent when a Subscription for a Premium App has been deleted.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SubscriptionDelete(pub Subscription);

impl Deref for SubscriptionDelete {
    type Target = Subscription;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SubscriptionDelete {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
