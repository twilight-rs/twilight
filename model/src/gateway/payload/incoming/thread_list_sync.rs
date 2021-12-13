use crate::{
    channel::{thread::ThreadMember, Channel},
    id::{marker, Id},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadListSync {
    #[serde(default)]
    pub channel_ids: Vec<Id<marker::Guild>>,
    pub guild_id: Id<marker::Guild>,
    pub members: Vec<ThreadMember>,
    pub threads: Vec<Channel>,
}
