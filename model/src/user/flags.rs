use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Deserialize, Serialize)]
    pub struct UserFlags: u64 {
        const DISCORD_EMPLOYEE = 1;
        const DISCORD_PARTNER = 1 << 1;
        const HYPESQUAD_EVENTS = 1 << 2;
        const BUG_HUNTER = 1 << 3;
        const HOUSE_BRAVERY = 1 << 6;
        const HOUSE_BRILLIANCE = 1 << 7;
        const HOUSE_BALANCE = 1 << 8;
        const EARLY_SUPPORTER = 1 << 9;
        const TEAM_USER = 1 << 10;
    }
}
