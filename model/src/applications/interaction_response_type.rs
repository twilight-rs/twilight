use serde_repr::{Deserialize_repr, Serialize_repr};
/*
 * # InteractionResponseType
 *
 * | Name                     | Value | Description                                      |
 * |--------------------------|-------|--------------------------------------------------|
 * | Pong 	                    1 	ACK a Ping
 * | Acknowledge 	            2 	ACK a command without sending a message, eating the user's input
 * | ChannelMessage 	            3 	respond with a message, eating the user's input
 * | ChannelMessageWithSource 	    4 	respond with a message, showing the user's input
 * | ACKWithSource 	            5 	ACK a command without sending a message, showing the user's input
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
    ACKWithSource = 5,
}
