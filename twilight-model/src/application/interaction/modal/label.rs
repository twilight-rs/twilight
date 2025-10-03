use crate::application::interaction::modal::ModalInteractionComponent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModalInteractionLabel {
    pub id: i32,
    pub component: Box<ModalInteractionComponent>,
}
