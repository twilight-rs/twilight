use super::update_presence::UpdatePresencePayload;
use crate::gateway::{intents::Intents, opcode::OpCode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Identify {
    pub d: IdentifyInfo,
    pub op: OpCode,
}

impl Identify {
    pub const fn new(info: IdentifyInfo) -> Self {
        Self {
            d: info,
            op: OpCode::Identify,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IdentifyInfo {
    pub compress: bool,
    pub intents: Intents,
    pub large_threshold: u64,
    pub presence: Option<UpdatePresencePayload>,
    pub properties: IdentifyProperties,
    pub shard: Option<[u64; 2]>,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IdentifyProperties {
    #[serde(rename = "$browser")]
    pub browser: String,
    #[serde(rename = "$device")]
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
