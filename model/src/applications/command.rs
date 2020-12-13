use crate::applications::CommandOption;
use crate::id::{ApplicationId, CommandId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Command {
    /* TODO: Should there be a specific struct in http where
     * this field is a Option, becuase it is only used when
     * creating commands.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub command_id: Option<CommandId>,
    pub application_id: ApplicationId,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub options: Vec<CommandOption>,
}
