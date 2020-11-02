use crate::channel::embed::Embed;
use crate::channel::message::MessageFlags;
use crate::guild::PartialMember;
use crate::id::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
}

/*
 * # InteractionType
 *
 * | Name               | Value |
 * |--------------------|-------|
 * | Ping               | 1     |
 * | ApplicationCommand | 2     |
 */
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
}

/*
 * # InteractionData
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

/*
 * # ApplicationCommandInteractionDataOption
 *
 * All options have names, and an option can either be a parameter and
 * input value--in which case `value` will be set--or it can denote a
 * subcommand or group--in which case it will contain a top-level key and
 * another array of `options`.
 *
 * `value` and `options` are mututally exclusive.
 *
 * | Field    | Type                                             |
 * |----------|--------------------------------------------------|
 * | name     | string                                           |
 * | value?   | OptionType                                       |
 * | options? | array of ApplicationCommandInteractionDataOption |
 */
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InteractionDataOption {
    pub name: String,
    pub value: Option<String>, // Or int
    pub options: Vec<InteractionDataOption>,
}

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

/*
 * # InteractionResponseType
 *
 * | Name                     | Value | Description                                      |
 * |--------------------------|-------|--------------------------------------------------|
 * | Pong                     | 1     | ACK a `Ping`                                     |
 * | Acknowledge              | 2     | received and will do something async             |
 * | ChannelMessage           | 3     | respond with a message, eating the user's input  |
 * | ChannelMessageWithSource | 4     | respond with a message, showing the user's input |
 */
#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
#[repr(u8)]
pub enum InteractionResponseType {
    Pong = 1,
    Acknowledge = 2,
    ChannelMessage = 3,
    ChannelMessageWithSource = 4,
}

/*
 * # InteractionApplicationCommandCallbackData
 *
 * | Name              | Value            |
 * |-------------------|------------------|
 * | tts?              | bool             |
 * | content           | string           |
 * | embeds?           | array of embeds  |
 * | allowed_mentions? | allowed mentions |
 * | flags             | int              |
 */
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CommandCallbackData {
    pub tts: Option<bool>,
    pub content: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
    // Allowed mentions will be later.
    pub flags: MessageFlags,
}
