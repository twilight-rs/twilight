use crate::{applications::interaction::InteractionType, id::InteractionId, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MessageInteraction {
    pub id: InteractionId,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    pub name: String,
    pub user: User,
}
