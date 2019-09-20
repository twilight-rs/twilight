use crate::channel::Reaction;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReactionRemove(pub Reaction);
