use bitflags::bitflags;

bitflags! {
    #[cfg_attr(feature = "serde-support", derive(serde::Deserialize, serde::Serialize))]
    pub struct ActivityFlags: u64 {
        const INSTANCE = 0b001;
        const JOIN = 0b010;
        const SPECTATE = 0b011;
        const JOIN_REQUEST = 0b100;
        const SYNC = 0b101;
        const PLAY = 0b110;
    }
}
