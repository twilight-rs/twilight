use crate::id::ApplicationId;
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
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
