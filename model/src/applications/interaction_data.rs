use super::InteractionDataOption;
use crate::id::CommandId;
use serde::{Deserialize, Serialize};

/*
 * # ApplicationCommandInteractionData
 *
 * | Field   | Type                                             | Description                       |
 * |---------|--------------------------------------------------|-----------------------------------|
 * | id      | snowflake                                        | the ID of the invoked command     |
 * | name    | string                                           | the name of the invoked command   |
 * | options | array of ApplicationCommandInteractionDataOption | the params + values from the user |
 */

/// Interactions have a common data field with unique information based on the InteractionType
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum InteractionData {
    Ping,
    ApplicationCommand(ApplicationCommandInteractionData),
}

impl InteractionData {
    pub fn name(&self) -> &'static str {
        match self {
            InteractionData::Ping => "Ping",
            InteractionData::ApplicationCommand(_) => "ApplicationCommand",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, Default)]
pub struct ApplicationCommandInteractionData {
    pub id: CommandId,
    pub name: String,
    #[serde(default)]
    pub options: Vec<InteractionDataOption>,
}
