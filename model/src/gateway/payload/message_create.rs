use crate::channel::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MessageCreate(pub Message);
