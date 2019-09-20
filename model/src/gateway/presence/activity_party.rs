use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ActivityParty {
    pub id: Option<String>,
    pub size: Option<[u64; 2]>,
}
