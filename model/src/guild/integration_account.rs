use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}
