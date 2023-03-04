use super::ApplicationFlags;
use crate::id::{marker::ApplicationMarker, Id};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct PartialApplication {
    pub flags: ApplicationFlags,
    pub id: Id<ApplicationMarker>,
}
