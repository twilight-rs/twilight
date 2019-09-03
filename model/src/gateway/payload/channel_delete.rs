use crate::channel::Channel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelDelete(pub Channel);
