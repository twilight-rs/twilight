use serde::{Deserialize, Serialize};

use super::Component;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Container {
    pub id: Option<i32>,
    pub accent_color: Option<u32>,
    pub spoiler: Option<bool>,
    // At the moment this can only be ActionRow, TextDisplay, Section,
    // MediaGallery, Separator, File.
    pub components: Vec<Component>,
}
