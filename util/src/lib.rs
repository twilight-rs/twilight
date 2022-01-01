//! # twilight-util
//!
//! [![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]
//!
//! `twilight-util` is a set of utility types and functions for the [`twilight-rs`] ecosystem to
//! augment or enhance default functionality.
//!
//! ## Features
//!
//! ### `builder`
//!
//! Provides builders for large structs.
//!
//! ### `link`
//!
//! Provides implementations for parsing and formatting entities' URLs, such as
//! webhook URLs.
//!
//! ### `permission-calculator`
//!
//! Allows the use of a calculator to determine the permissions of a member in
//! a guild or channel.
//!
//! [`twilight-rs`]: https://github.com/twilight-rs/twilight
//! [codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
//! [codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
//! [discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
//! [discord link]: https://discord.gg/7jj8n7D
//! [github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
//! [github link]: https://github.com/twilight-rs/twilight
//! [license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
//! [license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
//! [rust badge]: https://img.shields.io/badge/rust-1.57+-93450a.svg?style=for-the-badge&logo=rust
//! [Discord snowflakes]: https://discord.com/developers/docs/reference#snowflakes

#![deny(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::pedantic,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code,
    unused,
    warnings
)]
#![allow(clippy::semicolon_if_nothing_returned)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "builder")]
pub mod builder;

#[cfg(feature = "link")]
pub mod link;

#[cfg(feature = "permission-calculator")]
pub mod permission_calculator;
