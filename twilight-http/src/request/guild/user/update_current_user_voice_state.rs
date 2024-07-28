use crate::{
    client::Client,
    error::Error,
    request::{Nullable, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker},
    Id,
};

#[derive(Serialize)]
struct UpdateCurrentUserVoiceStateFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suppress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_to_speak_timestamp: Option<Nullable<&'a str>>,
}

/// Update the current user's voice state.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCurrentUserVoiceState<'a> {
    fields: UpdateCurrentUserVoiceStateFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> UpdateCurrentUserVoiceState<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: UpdateCurrentUserVoiceStateFields {
                channel_id: None,
                suppress: None,
                request_to_speak_timestamp: None,
            },
            guild_id,
            http,
        }
    }

    /// Specify the ID of the stage channel which the user is currently connected to.
    ///
    /// # Caveats
    ///
    /// - `channel_id` must currently point to a stage channel.
    /// - User must already be connected to this stage channel.
    pub const fn channel_id(mut self, channel_id: Id<ChannelMarker>) -> Self {
        self.fields.channel_id = Some(channel_id);

        self
    }

    /// Set the user's request to speak.
    ///
    /// Set to an empty string to remove an already-present request.
    ///
    /// # Caveats
    ///
    /// - You are able to set `request_to_speak_timestamp` to any present or
    ///   future time.
    pub const fn request_to_speak_timestamp(mut self, request_to_speak_timestamp: &'a str) -> Self {
        if request_to_speak_timestamp.is_empty() {
            self.fields.request_to_speak_timestamp = Some(Nullable(None));
        } else {
            self.fields.request_to_speak_timestamp =
                Some(Nullable(Some(request_to_speak_timestamp)));
        }

        self
    }

    /// Toggle the user's suppress state.
    ///
    /// # Caveats
    ///
    /// - You must have the `MUTE_MEMBERS` permission to unsuppress yourself.
    ///   You can always suppress yourself.
    pub const fn suppress(mut self) -> Self {
        self.fields.suppress = Some(true);

        self
    }
}

impl IntoFuture for UpdateCurrentUserVoiceState<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateCurrentUserVoiceState<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::UpdateCurrentUserVoiceState {
            guild_id: self.guild_id.get(),
        })
        .json(&self.fields)
        .build()
    }
}
