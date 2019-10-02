mod bot_connection_info;

pub use self::bot_connection_info::BotConnectionInfo;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ConnectionInfo {
    pub url: String,
}
