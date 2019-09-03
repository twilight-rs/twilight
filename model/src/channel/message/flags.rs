use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Deserialize, Serialize)]
    pub struct MessageFlags: u64 {
        const CROSSPOSTED = 1;
        const IS_CROSSPOST = 1 << 1;
        const SUPPRESS_EMBEDS = 1 << 2;
    }
}
