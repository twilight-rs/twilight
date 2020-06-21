mod bot_connection_info;

pub use self::bot_connection_info::BotConnectionInfo;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ConnectionInfo {
    pub url: String,
}
