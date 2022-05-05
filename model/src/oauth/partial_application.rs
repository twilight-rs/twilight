use super::ApplicationFlags;
use crate::id::{marker::ApplicationMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PartialApplication {
    pub flags: ApplicationFlags,
    pub id: Id<ApplicationMarker>,
}
