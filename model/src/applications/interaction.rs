use super::{InteractionData, InteractionType};
use crate::guild::PartialMember;
use crate::id::*;
use serde::{Deserialize, Serialize};

/*
 * # Interaction
 *
 * | Field          | Type                              |
 * |----------------|-----------------------------------|
 * | id             | snowflake                         |
 * | type           | InteractionType                   |
 * | data?\*        | ApplicationCommandInteractionData |
 * | guild_id       | snowflake                         |
 * | channel_id     | snowflake                         |
 * | member         | GuildMember                       |
 * | token          | string                            |
 */
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Interaction {
    pub id: InteractionId,
    #[serde(rename = "type")]
    pub kind: InteractionType,
    pub data: Option<InteractionData>,
    pub guild_id: GuildId,
    pub channel_id: ChannelId,
    pub member: PartialMember,
    pub token: String,
    pub version: u64,
}
