use crate::voice::{speaking_flags::SpeakingFlags, OpCode};
use serde::{Deserialize, Serialize};

/// At least one [`Speaking`] payload must be sent before sending voice data.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Speaking {
    pub d: SpeakingInfo,
    pub op: OpCode,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SpeakingInfo {
    pub delay: u64,
    pub speaking: SpeakingFlags,
    /// Unique voice identifier
    ///
    /// Received in [`Ready`].
    ///
    /// [`Ready`]: crate::voice::payload::incoming::Ready
    pub ssrc: u32,
}
