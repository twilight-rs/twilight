use crate::voice::{EncryptionMode, OpCode};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Ready {
    pub d: ReadyInfo,
    pub op: OpCode,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ReadyInfo {
    /// IP address of voice server.
    pub ip: IpAddr,
    /// Supported encryption modes.
    pub modes: Vec<EncryptionMode>,
    /// UDP port of voice server.
    pub port: u16,
    /// User's unique voice identifier.
    ///
    /// Needed to map users and emitted audio.
    pub ssrc: u32,
}
