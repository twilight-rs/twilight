use crate::gateway::opcode::OpCode;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identify {
    pub d: IdentifyInfo,
    pub op: OpCode,
}

impl Identify {
    pub fn new(
        shard: impl Into<Option<[u64; 2]>>,
        token: impl Into<String>,
        compression: bool,
        large_threshold: u64,
        v: u64,
        properties: IdentifyProperties,
    ) -> Self {
        Self {
            d: IdentifyInfo::new(shard, token, compression, large_threshold, v, properties),
            op: OpCode::Identify,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentifyInfo {
    pub compression: bool,
    pub large_threshold: u64,
    pub properties: IdentifyProperties,
    pub shard: Option<[u64; 2]>,
    pub token: String,
    pub v: u64,
}

impl IdentifyInfo {
    pub fn new(
        shard: impl Into<Option<[u64; 2]>>,
        token: impl Into<String>,
        compression: bool,
        large_threshold: u64,
        v: u64,
        properties: IdentifyProperties,
    ) -> Self {
        Self {
            compression,
            large_threshold,
            properties,
            shard: shard.into(),
            token: token.into(),
            v,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IdentifyProperties {
    #[serde(rename = "$browser")]
    pub browser: String,
    #[serde(rename = "device")]
    pub device: String,
    #[serde(rename = "$os")]
    pub os: String,
    #[serde(rename = "$referrer")]
    pub referrer: String,
    #[serde(rename = "$referring_domain")]
    pub referring_domain: String,
}

impl IdentifyProperties {
    pub fn new(
        browser: impl Into<String>,
        device: impl Into<String>,
        os: impl Into<String>,
        referrer: impl Into<String>,
        referring_domain: impl Into<String>,
    ) -> Self {
        Self {
            browser: browser.into(),
            device: device.into(),
            os: os.into(),
            referrer: referrer.into(),
            referring_domain: referring_domain.into(),
        }
    }
}
