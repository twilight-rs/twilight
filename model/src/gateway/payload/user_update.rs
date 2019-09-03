use crate::user::CurrentUser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserUpdate(pub CurrentUser);
