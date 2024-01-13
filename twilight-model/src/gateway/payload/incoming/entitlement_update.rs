use crate::application::monetization::Entitlement;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EntitlementUpdate(pub Entitlement);

impl Deref for EntitlementUpdate {
    type Target = Entitlement;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntitlementUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
