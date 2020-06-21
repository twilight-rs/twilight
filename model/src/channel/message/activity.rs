use super::MessageActivityType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub kind: MessageActivityType,
    pub party_id: Option<String>,
}
