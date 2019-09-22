use crate::id::ApplicationId;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MessageApplication {
    pub id: ApplicationId,
    pub cover_image: Option<String>,
    pub description: String,
    pub icon: Option<String>,
    pub name: String,
}

impl Hash for MessageApplication {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
