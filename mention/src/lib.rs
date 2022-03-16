#![allow(clippy::module_name_repetitions)]
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
    unused,
    warnings
)]
#![doc = include_str!("../README.md")]

pub mod fmt;
pub mod parse;
pub mod timestamp;

#[doc(no_inline)]
pub use fmt::Mention;

#[doc(no_inline)]
pub use parse::ParseMention;
