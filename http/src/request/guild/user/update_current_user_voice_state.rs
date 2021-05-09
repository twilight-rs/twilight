use crate::request::prelude::*;
use twilight_model::id::{ChannelId, GuildId};

#[derive(Serialize)]
struct UpdateCurrentUserVoiceStateFields {
    channel_id: ChannelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    suppress: Option<bool>,
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    request_to_speak_timestamp: Option<Option<String>>,
}

/// Update the current user's voice state.
pub struct UpdateCurrentUserVoiceState<'a> {
    fields: UpdateCurrentUserVoiceStateFields,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> UpdateCurrentUserVoiceState<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, channel_id: ChannelId) -> Self {
        Self {
            fields: UpdateCurrentUserVoiceStateFields {
                channel_id,
                suppress: None,
                request_to_speak_timestamp: None,
            },
            fut: None,
            guild_id,
            http,
        }
    }

    /// Set the user's request to speak.
    ///
    /// Set to an empty string to remove an already-present request.
    ///
    /// # Caveats
    ///
    /// - You are able to set `request_to_speak_timestamp` to any present or
    /// future time.
    pub fn request_to_speak_timestamp(self, request_to_speak_timestamp: impl Into<String>) -> Self {
        Self::_request_to_speak_timestamp(self, request_to_speak_timestamp.into())
    }

    fn _request_to_speak_timestamp(mut self, request_to_speak_timestamp: String) -> Self {
        if request_to_speak_timestamp.is_empty() {
            self.fields.request_to_speak_timestamp.replace(None);
        } else {
            self.fields
                .request_to_speak_timestamp
                .replace(Some(request_to_speak_timestamp));
        }

        self
    }

    /// Toggle the user's suppress state.
    ///
    /// # Caveats
    ///
    /// - You must have the `MUTE_MEMBERS` permission to unsuppress yourself.
    /// You can always suppress yourself.
    pub fn suppress(mut self) -> Self {
        self.fields.suppress.replace(true);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from((
            crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
            Route::UpdateCurrentUserVoiceState {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateCurrentUserVoiceState<'_>, ());
