use super::Component;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Section {
    pub id: Option<i32>,
    // At the moment only TextDisplay works.
    pub components: Vec<Component>,
    // At the moment this can only be Thumbnail or Button.
    pub accessory: Box<Component>,
}
