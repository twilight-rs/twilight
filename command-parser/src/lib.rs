//! [![license badge][]][license link] [![rust badge]][rust link]
//!
//! # dawn-command-parser
//!
//! `dawn-command-parser` is a command parser for the [`dawn`] ecosystem.
//!
//! Included is a mutable configuration that allows you to specify the command
//! names, prefixes, ignored guilds and users, and more. The parser parses out
//! commands matching an available command and prefix and provides the command
//! arguments to you.
//!
//! ### Features
//!
//! There is a single feature, `model`, which determines whether to support
//! `dawn_model::channel::Message` as an input. All that this does is additional
//! checking of the author and the guild it was sent in to see if they're
//! ignored. If you don't need that, you can disable this feature. With this
//! feature the dependency tree is 1 dependency, and without it is 0.
//!
//! Using `model`:
//!
//! ```toml
//! [dependencies]
//! dawn-command-parser = "0.1"
//! ```
//!
//! Not using `dawn_model`:
//!
//! ```toml
//! [dependencies]
//! dawn-command-parser = { default-features = false, version = "0.1" }
//! ```
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
//! use dawn_command_parser::{Config, Output, Parser};
//!
//! let mut config = Config::new();
//!
//! // (Use `Config::add_command` to add a single command)
//! config.add_command("echo");
//! config.add_command("ping");
//!
//! // Add the prefix `"!"`.
//! // (Use `Config::add_prefixes` to add multiple prefixes)
//! config.add_prefix("!");
//!
//! let parser = Parser::new(config);
//!
//! // Now pass a command to the parser
//! match parser.parse_str("!echo a message") {
//!     Output::Command { name: "echo", arguments, .. } => {
//!         let content = arguments.as_str();
//!
//!         println!("Got an echo request to send `{}`", content);
//!     },
//!     Output::Command { name: "ping", .. } => {
//!         println!("Got a ping request");
//!     },
//!     Output::IgnoredGuild => println!("Message from ignored guild"),
//!     Output::IgnoredUser => println!("Message from ignored user"),
//!     Output::NoMatch => println!("Message didn't match a prefix and command"),
//!     // Ignore all other commands.
//!     _ => {},
//! }
//! ```
//!
//! [license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
//! [license link]: https://opensource.org/licenses/ISC
//! [rust badge]: https://img.shields.io/badge/rust-1.36+-93450a.svg?style=flat-square
//! [rust link]: https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html
//! [`dawn`]: https://github.com/dawn-rs/dawn

#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused,
    warnings
)]
#![allow(clippy::module_name_repetitions)]

mod arguments;
mod config;
mod parser;

pub use self::{
    arguments::Arguments,
    config::Config,
    parser::{Output, Parser},
};
