use crate::channel::Channel;
use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChannelUpdate(pub Channel);
