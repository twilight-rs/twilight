//! Builders for large structs.
#![allow(clippy::missing_const_for_fn)]

pub mod command;
pub mod embed;
pub mod interaction_response;
pub mod message;

mod interaction_response_data;

#[allow(deprecated)]
pub use self::interaction_response_data::InteractionResponseDataBuilder;
