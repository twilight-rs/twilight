use crate::user::CurrentUser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UserUpdate(pub CurrentUser);
