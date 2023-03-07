use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
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
    fields: Result<UpdateStageInstanceFields<'a>, ValidationError>,
    http: &'a Client,
}

impl<'a> UpdateStageInstance<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            channel_id,
            fields: Ok(UpdateStageInstanceFields {
                privacy_level: None,
                topic: None,
            }),
            http,
        }
    }

    /// Set the [`PrivacyLevel`] of the instance.
    pub fn privacy_level(mut self, privacy_level: PrivacyLevel) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.privacy_level = Some(privacy_level);
        }

        self
    }

    /// Set the new topic of the instance.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`StageTopic`] if the length is invalid.
    ///
    /// [`StageTopic`]: twilight_validate::request::ValidationErrorType::StageTopic
    pub fn topic(mut self, topic: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_stage_topic(topic)?;
            fields.topic.replace(topic);

            Ok(fields)
        });

        self
    }
}

impl IntoFuture for UpdateStageInstance<'_> {
    type Output = Result<Response<StageInstance>, Error>;

    type IntoFuture = ResponseFuture<StageInstance>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateStageInstance<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::UpdateStageInstance {
            channel_id: self.channel_id.get(),
        })
        .json(&fields)
        .build()
    }
}
