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
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InteractionData {
    pub id: CommandId,
    pub name: String,
    #[serde(default)]
    pub options: Vec<InteractionDataOption>,
}
