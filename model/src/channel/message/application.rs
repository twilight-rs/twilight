use crate::id::ApplicationId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageApplication {
    pub cover_image: Option<String>,
    pub description: String,
    pub icon: Option<String>,
    pub id: ApplicationId,
    pub name: String,
}
