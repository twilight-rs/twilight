use crate::id::UserId;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Default, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CurrentUser {
    pub id: UserId,
    pub avatar: Option<String>,
    #[serde(default)]
    pub bot: bool,
    pub discriminator: String,
    pub email: Option<String>,
    pub mfa_enabled: bool,
    #[serde(rename = "username")]
    pub name: String,
    pub verified: bool,
}

impl Hash for CurrentUser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
