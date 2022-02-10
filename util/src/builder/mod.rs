//! Builders for large structs.
#![allow(clippy::module_name_repetitions)]

pub mod command;

mod interaction_response_data;

pub use self::interaction_response_data::InteractionResponseDataBuilder;
