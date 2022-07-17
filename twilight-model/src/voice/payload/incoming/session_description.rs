use crate::voice::{EncryptionMode, OpCode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SessionDescription {
    pub d: SessionDescriptionInfo,
    pub op: OpCode,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SessionDescriptionInfo {
    /// Selected encryption mode in [`SelectProtocol`].
    ///
    /// [`SelectProtocol`]: crate::voice::payload::outgoing::SelectProtocol
    pub mode: EncryptionMode,
    /// Key to encrypt and send voice data.
    pub secret_key: [u8; 32],
}
