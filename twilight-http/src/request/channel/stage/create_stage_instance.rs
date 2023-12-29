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
    id::{
        marker::{ChannelMarker, ScheduledEventMarker},
        Id,
    },
};
use twilight_validate::request::{stage_topic as validate_stage_topic, ValidationError};

#[derive(Serialize)]
struct CreateStageInstanceFields<'a> {
    channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guild_scheduled_event_id: Option<Id<ScheduledEventMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy_level: Option<PrivacyLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_start_notification: Option<bool>,
    topic: &'a str,
}

/// Create a new stage instance associated with a stage channel.
///
/// Requires the user to be a moderator of the stage channel.
#[must_use = "requests must be configured and executed"]
pub struct CreateStageInstance<'a> {
    fields: Result<CreateStageInstanceFields<'a>, ValidationError>,
    http: &'a Client,
}

impl<'a> CreateStageInstance<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: Id<ChannelMarker>, topic: &'a str) -> Self {
        let fields = Ok(CreateStageInstanceFields {
            channel_id,
            guild_scheduled_event_id: None,
            privacy_level: None,
            send_start_notification: None,
            topic,
        })
        .and_then(|fields| {
            validate_stage_topic(topic)?;

            Ok(fields)
        });

        Self { fields, http }
    }

    /// Set the guild scheduled event associated with this stage instance.
    pub fn guild_scheduled_event_id(
        mut self,
        guild_scheduled_event_id: Id<ScheduledEventMarker>,
    ) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.guild_scheduled_event_id = Some(guild_scheduled_event_id);
        }

        self
    }

    /// Set the [`PrivacyLevel`] of the instance.
    pub fn privacy_level(mut self, privacy_level: PrivacyLevel) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.privacy_level = Some(privacy_level);
        }

        self
    }

    /// Set whether to notify everyone when a stage starts.
    ///
    /// The stage moderator must have [`Permissions::MENTION_EVERYONE`] for this
    /// notification to be sent.
    ///
    /// [`Permissions::MENTION_EVERYONE`]: twilight_model::guild::Permissions::MENTION_EVERYONE
    pub fn send_start_notification(mut self, send_start_notification: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.send_start_notification = Some(send_start_notification);
        }

        self
    }
}

impl IntoFuture for CreateStageInstance<'_> {
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

impl TryIntoRequest for CreateStageInstance<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::CreateStageInstance)
            .json(&fields)
            .build()
    }
}
