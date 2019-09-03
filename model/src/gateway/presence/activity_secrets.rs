use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    #[serde(rename = "match")]
    pub match_: Option<String>,
    pub spectate: Option<String>,
}
