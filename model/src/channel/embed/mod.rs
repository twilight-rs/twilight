mod author;
mod field;
mod footer;
mod image;
mod provider;
mod thumbnail;
mod video;

pub use self::{
    author::EmbedAuthor, field::EmbedField, footer::EmbedFooter, image::EmbedImage,
    provider::EmbedProvider, thumbnail::EmbedThumbnail, video::EmbedVideo,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Embed {
    pub author: Option<EmbedAuthor>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub color: Option<u32>,
    pub description: Option<String>,
    #[cfg_attr(feature = "serde-support", serde(default))]
    pub fields: Vec<EmbedField>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    #[cfg_attr(feature = "serde-support", serde(rename = "type"))]
    pub kind: String,
    pub provider: Option<EmbedProvider>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub timestamp: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub video: Option<EmbedVideo>,
}
