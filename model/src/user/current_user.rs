use crate::id::UserId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
