//! [![license badge][]][license link] [![rust badge]][rust link]
//!
//! # dawn-command-parser
//!
//! `dawn-command-parser` is a command parser for the [`dawn`] ecosystem.
//!
//! Included is a mutable configuration that allows you to specify the command
//! names and prefixes. The parser parses out commands matching an available
//! command and prefix and provides the command arguments to you.
//!
//! # Installation
//!
//! `dawn-command-parser` requires at least Rust 1.36.0.
//!
//! Add the following to your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! dawn-command-parser = "0.1"
//! ```
//!
//! ### Examples
//!
//! A simple parser for a bot with one prefix (`"!"`) and two commands: `"echo"`
//! and `"ping"`:
//!
//! ```rust,no_run
//! use dawn_command_parser::{Command, Config, Parser};
//!
//! let mut config = Config::new();
//!
//! // (Use `Config::add_command` to add a single command)
//! config.command("echo").add();
//! config.command("ping").add();
//!
//! // Add the prefix `"!"`.
//! // (Use `Config::add_prefixes` to add multiple prefixes)
//! config.add_prefix("!");
//!
//! let parser = Parser::new(config);
//!
//! // Now pass a command to the parser
//! match parser.parse("!echo a message") {
//!     Some(Command { name: "echo", arguments, .. }) => {
//!         let content = arguments.as_str();
//!
//!         println!("Got an echo request to send `{}`", content);
//!     },
//!     Some(Command { name: "ping", .. }) => {
//!         println!("Got a ping request");
//!     },
//!     // Ignore all other commands.
//!     Some(_) => {},
//!     None => println!("Message didn't match a prefix and command"),
//! }
//! ```
//!
//! [license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
//! [license link]: https://opensource.org/licenses/ISC
//! [rust badge]: https://img.shields.io/badge/rust-1.36+-93450a.svg?style=flat-square
//! [rust link]: https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html
//! [`dawn`]: https://dawn.valley.cafe

#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused,
    warnings
)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

mod arguments;
mod builder;
mod casing;
mod config;
mod parser;

pub use self::{
    arguments::Arguments,
    builder::CommandBuilder,
    casing::CaseSensitivity,
    config::Config,
    parser::{Command, Parser},
};
