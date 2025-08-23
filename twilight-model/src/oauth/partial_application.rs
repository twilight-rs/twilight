use super::ApplicationFlags;
use crate::id::{Id, marker::ApplicationMarker};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PartialApplication {
    pub flags: ApplicationFlags,
    pub id: Id<ApplicationMarker>,
}
