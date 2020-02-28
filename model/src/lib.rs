//! [![license badge][]][license link] [![rust badge]][rust link]
//!
//! # dawn-model
//!
//! See the [`dawn`] documentation for more information.
//!
//! `dawn-model` is a crate of only serde models defining the Discord APIs with
//! no implementations on top of them or functions to work with them.
//!
//! These are in a single crate for ease of use, a single point of definition,
//! and a sort of versioning of the Discord API. Similar to how a database
//! schema progresses in versions, the definition of the API also progresses in
//! versions.
//!
//! The types in this crate are reproducible: deserializing a payload into a
//! type, serializing it, and then deserializing it again will work.
//!
//! Defined are a number of modules defining types returned by or owned by
//! resource categories. For example, `gateway` are types used to interact with
//! and returned by the gateway API. `guild` contains types owned by the Guild
//! resource category. These types may be directly returned by, built on top of,
//! or extended by other crates.
//!
//! ### Installation
//!
//! This crate requires Rust 1.31+.
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! dawn-model = "0.1"
//! ```
//!
//! ### Features
//!
//! `dawn-model` has a single feature, `serde-support`. By default it is enabled.
//! This enables serde support of the models, which brings in four dependencies:
//!
//! - `serde`
//! - `serde_json`
//! - `serde-mappable-seq`
//! - `serde_repr`
//!
//! Enabling only the `serde` dependency will do nothing: all of them need to be
//! enabled for correct (de)serialization, so you need to enable
//! `serde-support`.
//!
//! If you don't need serde support, you can disable it:
//!
//! ```toml
//! [dependencies]
//! dawn-model = { default-features = false, git = "https://github.com/dawn-rs/dawn" }
//! ```
//!
//! ## License
//!
//! [ISC][LICENSE.md]
//!
//! [LICENSE.md]: https://github.com/dawn-rs/dawn/blob/master/LICENSE.md
//! [`dawn`]: https://docs.rs/dawn
//! [license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
//! [license link]: https://opensource.org/licenses/ISC
//! [rust badge]: https://img.shields.io/badge/rust-1.31+-93450a.svg?style=flat-square
//! [rust link]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unused,
    warnings
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools
)]

pub mod channel;
pub mod gateway;
pub mod guild;
pub mod id;
pub mod invite;
pub mod oauth;
pub mod user;
pub mod voice;
