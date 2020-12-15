use super::{CommandCallbackData, InteractionResponseType};
use serde::{Deserialize, Serialize};
/*
 * # Interaction Response
 *
 * | Field | Type                                      | Description                  |
 * |-------|-------------------------------------------|------------------------------|
 * | type  | InteractionResponseType                   | the type of response         |
 * | data? | InteractionApplicationCommandCallbackData | an optional response message |
 */
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    pub kind: InteractionResponseType,
    pub data: Option<CommandCallbackData>,
}
