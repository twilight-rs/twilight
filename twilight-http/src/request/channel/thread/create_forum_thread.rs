use crate::request::Nullable;
use crate::{
    client::Client,
    error::Error,
    request::{attachment::PartialAttachment, Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    application::component::Component,
    channel::{
        embed::Embed,
        message::{AllowedMentions, MessageFlags},
    },
    id::{
        marker::{ChannelMarker, StickerMarker},
        Id,
    },
};

#[derive(Serialize)]
struct CreateForumThreadFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_archive_duration: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<ForumThreadMessageParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<u16>,
}

/// Contents of the first message for the forum thread to be created.
#[derive(Serialize)]
pub struct ForumThreadMessageParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_mentions: Option<Nullable<&'a AllowedMentions>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<PartialAttachment<'a>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<&'a [Component]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    embeds: Option<&'a [Embed]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sticker_ids: Option<&'a [Id<StickerMarker>]>,
}

/// Creates a new thread in a forum channel.
///
/// Requires the [`SEND_MESSAGES`] permission.
///
/// [`SEND_MESSAGES`]: twilight_model::guild::Permissions::SEND_MESSAGES
#[must_use = "requests must be configured and executed"]
pub struct CreateForumThread<'a> {
    channel_id: Id<ChannelMarker>,
    fields: CreateForumThreadFields<'a>,
    http: &'a Client,
}

impl<'a> CreateForumThread<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            channel_id,
            fields: CreateForumThreadFields {
                auto_archive_duration: None,
                message: None,
                name: None,
                rate_limit_per_user: None,
            },
            http,
        }
    }
}
