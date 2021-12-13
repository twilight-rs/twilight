use crate::{
    client::Client,
    request::Request,
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use twilight_model::{channel::stage_instance::PrivacyLevel, id::ChannelId};
use twilight_validate::misc::{stage_topic as validate_stage_topic, ValidationError};

#[derive(Serialize)]
struct CreateStageInstanceFields<'a> {
    channel_id: ChannelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy_level: Option<PrivacyLevel>,
    topic: &'a str,
}

/// Create a new stage instance associated with a stage channel.
///
/// Requires the user to be a moderator of the stage channel.
#[must_use = "requests must be configured and executed"]
pub struct CreateStageInstance<'a> {
    fields: CreateStageInstanceFields<'a>,
    http: &'a Client,
}

impl<'a> CreateStageInstance<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        topic: &'a str,
    ) -> Result<Self, ValidationError> {
        validate_stage_topic(topic)?;

        Ok(Self {
            fields: CreateStageInstanceFields {
                channel_id,
                privacy_level: None,
                topic,
            },
            http,
        })
    }

    /// Set the [`PrivacyLevel`] of the instance.
    pub const fn privacy_level(mut self, privacy_level: PrivacyLevel) -> Self {
        self.fields.privacy_level = Some(privacy_level);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let mut request = Request::builder(&Route::CreateStageInstance);

        request = match request.json(&self.fields) {
            Ok(request) => request,
            Err(source) => return ResponseFuture::error(source),
        };

        self.http.request(request.build())
    }
}
