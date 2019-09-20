mod author;
mod field;
mod footer;
mod image;
mod provider;
mod thumbnail;
mod video;

pub use self::{
    author::EmbedAuthor,
    field::EmbedField,
    footer::EmbedFooter,
    image::EmbedImage,
    provider::EmbedProvider,
    thumbnail::EmbedThumbnail,
    video::EmbedVideo,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Embed {
    pub author: Option<EmbedAuthor>,
    pub color: u32,
    pub description: Option<String>,
    #[serde(default)]
    pub fields: Vec<EmbedField>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    #[serde(rename = "type")]
    pub kind: String,
    pub provider: Option<EmbedProvider>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub timestamp: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub video: Option<EmbedVideo>,
}
