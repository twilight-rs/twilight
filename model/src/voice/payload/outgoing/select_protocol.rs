use crate::voice::{EncryptionMode, OpCode};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

pub struct SelectProtocol {
    pub d: SelectProtocolInfo,
    pub op: OpCode,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SelectProtocolInfo {
    pub protocol: String,
    pub data: SelectProtocolData,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SelectProtocolData {
    /// IP address of voice server.
    pub address: IpAddr,
    /// Encryption mode for this connection.
    pub mode: EncryptionMode,
    /// Port of voice server.
    pub port: u16,
}
