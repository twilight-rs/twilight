use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, GuildMarker, UserMarker},
};

#[derive(Serialize)]
struct UpdateUserVoiceStateBody {
    channel_id: Id<ChannelMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suppress: Option<bool>,
}

struct UpdateUserVoiceStatePath {
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
}

/// Update another user's voice state.
#[must_use = "requests must be configured and executed"]
pub struct UpdateUserVoiceState<'a> {
    body: UpdateUserVoiceStateBody,
    http: &'a Client,
    path: UpdateUserVoiceStatePath,
}

impl<'a> UpdateUserVoiceState<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        channel_id: Id<ChannelMarker>,
    ) -> Self {
        Self {
            body: UpdateUserVoiceStateBody {
                channel_id,
                suppress: None,
            },
            http,
            path: UpdateUserVoiceStatePath { guild_id, user_id },
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
        self.body.suppress = Some(true);

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
        let UpdateUserVoiceStatePath { guild_id, user_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("voice-states")
            .id(user_id)
            .build()
            .map_err(Error::path)?;

        Request::builder(Method::Patch, path)
            .json(&self.body)
            .build()
    }
}
