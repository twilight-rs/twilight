use crate::{id::ApplicationId, user::UserFlags};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PartialApplication {
    pub flags: UserFlags,
    pub id: ApplicationId,
}
