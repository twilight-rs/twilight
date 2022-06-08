#![deny(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::pedantic,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::broken_intra_doc_links,
    unsafe_code,
    unused
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unnecessary_wraps,
    clippy::used_underscore_binding
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "builder")]
pub mod builder;

#[cfg(feature = "link")]
pub mod link;

#[cfg(feature = "permission-calculator")]
pub mod permission_calculator;

#[cfg(feature = "snowflake")]
pub mod snowflake;
