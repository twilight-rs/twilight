use crate::{
    application::interaction::InteractionType,
    id::{marker::InteractionMarker, Id},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageInteraction {
    pub id: Id<InteractionMarker>,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    pub name: String,
    pub user: User,
}
