mod message;

pub use self::message::CreateForumThreadMessage;

use self::message::CreateForumThreadMessageFields;
use crate::{
    client::Client,
    error::Error,
    request::{attachment::AttachmentManager, Nullable, Request},
    response::ResponseFuture,
    routing::Route,
};
use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::{thread::AutoArchiveDuration, Channel, Message},
    id::{
        marker::{ChannelMarker, TagMarker},
        Id,
    },
};

#[derive(Deserialize, Serialize)]
pub struct ForumThread {
    #[serde(flatten)]
    pub channel: Channel,
    pub message: Message,
}

#[derive(Serialize)]
struct CreateForumThreadFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    applied_tags: Option<&'a [Id<TagMarker>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_archive_duration: Option<AutoArchiveDuration>,
    message: CreateForumThreadMessageFields<'a>,
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    rate_limit_per_user: Option<u16>,
}

/// Creates a new thread in a forum channel.
///
/// Requires the [`SEND_MESSAGES`] permission.
///
/// [`SEND_MESSAGES`]: twilight_model::guild::Permissions::SEND_MESSAGES
#[must_use = "requests must be configured and executed"]
pub struct CreateForumThread<'a> {
    attachment_manager: AttachmentManager<'a>,
    channel_id: Id<ChannelMarker>,
    fields: CreateForumThreadFields<'a>,
    http: &'a Client,
}

impl<'a> CreateForumThread<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        name: &'a str,
    ) -> Self {
        Self {
            attachment_manager: AttachmentManager::new(),
            channel_id,
            fields: CreateForumThreadFields {
                applied_tags: None,
                auto_archive_duration: None,
                message: CreateForumThreadMessageFields {
                    allowed_mentions: None,
                    attachments: None,
                    components: None,
                    content: None,
                    embeds: None,
                    flags: None,
                    payload_json: None,
                    sticker_ids: None,
                },
                name,
                rate_limit_per_user: None,
            },
            http,
        }
    }

    /// Set the forum thread's applied tags.
    pub const fn applied_tags(mut self, applied_tags: &'a [Id<TagMarker>]) -> Self {
        self.fields.applied_tags = Some(applied_tags);

        self
    }

    /// Set the default auto archive duration for newly created threads in the
    /// channel.
    ///
    /// Automatic archive durations are not locked behind the guild's boost
    /// level.
    pub const fn auto_archive_duration(
        mut self,
        auto_archive_duration: AutoArchiveDuration,
    ) -> Self {
        self.fields.auto_archive_duration = Some(auto_archive_duration);

        self
    }

    pub const fn message(self) -> CreateForumThreadMessage<'a> {
        CreateForumThreadMessage::new(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    fn exec(self) -> ResponseFuture<ForumThread> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }

    fn try_into_request(mut self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::CreateForumThread {
            channel_id: self.channel_id.get(),
        });

        // Set the default allowed mentions if required.
        if self.fields.message.allowed_mentions.is_none() {
            if let Some(allowed_mentions) = self.http.default_allowed_mentions() {
                self.fields.message.allowed_mentions = Some(Nullable(Some(allowed_mentions)));
            }
        }

        // Determine whether we need to use a multipart/form-data body or a JSON
        // body.
        if !self.attachment_manager.is_empty() {
            let form = if let Some(payload_json) = self.fields.message.payload_json {
                self.attachment_manager.build_form(payload_json)
            } else {
                self.fields.message.attachments =
                    Some(self.attachment_manager.get_partial_attachments());

                let fields = crate::json::to_vec(&self.fields).map_err(Error::json)?;

                self.attachment_manager.build_form(fields.as_ref())
            };

            request = request.form(form);
        } else if let Some(payload_json) = self.fields.message.payload_json {
            request = request.body(payload_json.to_vec());
        } else {
            request = request.json(&self.fields);
        }

        request.build()
    }
}
