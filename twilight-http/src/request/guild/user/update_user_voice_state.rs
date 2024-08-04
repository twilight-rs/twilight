use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{ChannelMarker, GuildMarker, UserMarker},
    Id,
};

#[derive(Serialize)]
struct UpdateUserVoiceStateFields {
    channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suppress: Option<bool>,
}

/// Update another user's voice state.
#[must_use = "requests must be configured and executed"]
pub struct UpdateUserVoiceState<'a> {
    fields: UpdateUserVoiceStateFields,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> UpdateUserVoiceState<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Self {
        Self {
            fields: UpdateUserVoiceStateFields {
                channel_id,
                suppress: None,
            },
            guild_id,
            http,
            user_id,
        }
    }

    /// Toggle the user's suppress state.
    ///
    /// # Caveats
    ///
    /// - You must have the [`MUTE_MEMBERS`] permission to use this method.
    /// - When unsuppressed, non-bot users will have their
    ///   `request_to_speak_timestamp` set to the current time. Bot users will
    ///   not.
    /// - When suppressed, the user will have their `request_to_speak_timestamp`
    ///   removed.
    ///
    /// [`MUTE_MEMBERS`]: twilight_model::guild::Permissions::MUTE_MEMBERS
    pub const fn suppress(mut self) -> Self {
        self.fields.suppress = Some(true);

        self
    }
}

impl IntoFuture for UpdateUserVoiceState<'_> {
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

impl TryIntoRequest for UpdateUserVoiceState<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::UpdateUserVoiceState {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        })
        .json(&self.fields)
        .build()
    }
}
