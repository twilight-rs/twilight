use crate::channel::Channel;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelUpdate(pub Channel);
