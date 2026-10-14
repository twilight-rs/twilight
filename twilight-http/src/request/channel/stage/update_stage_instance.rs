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
    channel::{StageInstance, stage_instance::PrivacyLevel},
    id::{Id, marker::ChannelMarker},
};
use twilight_validate::request::{ValidationError, stage_topic as validate_stage_topic};

#[derive(Serialize)]
struct UpdateStageInstanceBody<'a> {
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
    body: Result<UpdateStageInstanceBody<'a>, ValidationError>,
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
}

impl<'a> UpdateStageInstance<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            channel_id,
            body: Ok(UpdateStageInstanceBody {
                privacy_level: None,
                topic: None,
            }),
            http,
        }
    }

    /// Set the [`PrivacyLevel`] of the instance.
    pub const fn privacy_level(mut self, privacy_level: PrivacyLevel) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.privacy_level = Some(privacy_level);
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
        self.body = self.body.and_then(|mut body| {
            validate_stage_topic(topic)?;
            body.topic.replace(topic);

            Ok(body)
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
        let body = self.body.map_err(Error::validation)?;

        Request::builder(&Route::UpdateStageInstance {
            channel_id: self.channel_id.get(),
        })
        .json(&body)
        .build()
    }
}
