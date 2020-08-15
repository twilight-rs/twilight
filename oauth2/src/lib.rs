//! # `twilight-oauth2`
//!
//! `twilight-oauth2` is a library for the [`twilight-rs`] ecosystem with
//! support for creating requests and parsing response bodies for Discord's
//! [OAuth 2 API].
//!
//! ## Installation
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! twilight-oauth2 = { branch = "trunk", git = "https://github.com/twilight-rs/twilight" }
//! ```
//!
//! [`twilight-rs`]: https://github.com/twilight-rs/twilight
//! [OAuth 2 API]: https://discord.com/developers/docs/topics/oauth2

pub mod authorization_url;
pub mod client;
pub mod request;

mod grant_type;
mod prompt;
mod scope;
mod token_type;

pub use self::{
    client::Client, grant_type::GrantType, prompt::Prompt, scope::Scope, token_type::TokenType,
};
