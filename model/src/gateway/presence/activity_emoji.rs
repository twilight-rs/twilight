use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ActivityEmoji {
    pub name: String,
    pub id: Option<String>,
    pub animated: Option<bool>,
}
