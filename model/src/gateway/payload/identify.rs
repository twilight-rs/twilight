use super::update_status::UpdateStatusInfo;
use crate::gateway::{intents::GatewayIntents, opcode::OpCode};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Identify {
    pub d: IdentifyInfo,
    pub op: OpCode,
}

impl Identify {
    pub fn new(info: IdentifyInfo) -> Self {
        Self {
            d: info,
            op: OpCode::Identify,
        }
    }
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IdentifyInfo {
    pub compression: bool,
    pub guild_subscriptions: bool,
    pub intents: GatewayIntents,
    pub large_threshold: u64,
    pub presence: Option<UpdateStatusInfo>,
    pub properties: IdentifyProperties,
    pub shard: Option<[u64; 2]>,
    pub token: String,
    pub v: u64,
}

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IdentifyProperties {
    #[cfg_attr(feature = "serde-support", serde(rename = "$browser"))]
    pub browser: String,
    #[cfg_attr(feature = "serde-support", serde(rename = "$device"))]
    pub device: String,
    #[cfg_attr(feature = "serde-support", serde(rename = "$os"))]
    pub os: String,
    #[cfg_attr(feature = "serde-support", serde(rename = "$referrer"))]
    pub referrer: String,
    #[cfg_attr(feature = "serde-support", serde(rename = "$referring_domain"))]
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
