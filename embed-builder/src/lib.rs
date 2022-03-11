#![doc = include_str!("../README.md")]
#![deprecated(since = "0.10.1", note = "use twilight_util::builder::embed")]
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
pub mod image_source;

mod author;
mod builder;
mod field;
mod footer;

pub use self::{
    author::EmbedAuthorBuilder,
    builder::{EmbedBuilder, EmbedError, EmbedErrorType},
    field::EmbedFieldBuilder,
    footer::EmbedFooterBuilder,
    image_source::ImageSource,
};
