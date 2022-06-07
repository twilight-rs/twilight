use crate::{
    client::Client,
    error::Error as HttpError,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::{stage_instance::PrivacyLevel, StageInstance},
    id::{marker::ChannelMarker, Id},
};
use twilight_validate::request::{stage_topic as validate_stage_topic, ValidationError};

#[derive(Serialize)]
struct UpdateStageInstanceFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy_level: Option<PrivacyLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    topic: Option<&'a str>,
}

/// Update fields of an existing stage instance.
///
/// Requires the user to be a moderator of the stage channel.
#[must_use = "requests must be configured and executed"]
pub struct UpdateStageInstance<'a> {
    channel_id: Id<ChannelMarker>,
    fields: UpdateStageInstanceFields<'a>,
    http: &'a Client,
}

impl<'a> UpdateStageInstance<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            channel_id,
            fields: UpdateStageInstanceFields {
                privacy_level: None,
                topic: None,
            },
            http,
        }
    }

    /// Set the [`PrivacyLevel`] of the instance.
    pub const fn privacy_level(mut self, privacy_level: PrivacyLevel) -> Self {
        self.fields.privacy_level = Some(privacy_level);

        self
    }

    /// Set the new topic of the instance.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`StageTopic`] if the length is invalid.
    ///
    /// [`StageTopic`]: twilight_validate::request::ValidationErrorType::StageTopic
    pub fn topic(mut self, topic: &'a str) -> Result<Self, ValidationError> {
        validate_stage_topic(topic)?;

        self.fields.topic.replace(topic);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<StageInstance> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateStageInstance<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::UpdateStageInstance {
            channel_id: self.channel_id.get(),
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
