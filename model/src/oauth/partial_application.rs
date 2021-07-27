use crate::{id::ApplicationId, oauth::current_application_info::ApplicationFlags};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PartialApplication {
    pub flags: ApplicationFlags,
    pub id: ApplicationId,
}
