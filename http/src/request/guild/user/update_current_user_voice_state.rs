use crate::request::prelude::*;
use twilight_model::id::{ChannelId, GuildId};

#[derive(Serialize)]
struct UpdateCurrentUserVoiceStateFields {
    channel_id: ChannelId,
    #[serde(skip_serializing_if = "Option::is_none")]
    suppress: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_to_speak_timestamp: Option<String>,
}

/// Update the current user's voice state.
///
/// All paramaters are optional.
///
/// # Caveats
///
/// - `channel_id` must currently point to a stage channel.
/// - Current user must have already joined `channel_id`.
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
    /// # Caveats
    ///
    /// - You are able to set `request_to_speak_timestamp` to any present or
    /// future time.
    pub fn request_to_speak_timestamp(self, request_to_speak_timestamp: impl Into<String>) -> Self {
        Self::_request_to_speak_timestamp(self, request_to_speak_timestamp.into())
    }

    fn _request_to_speak_timestamp(mut self, request_to_speak_timestamp: String) -> Self {
        self.fields
            .request_to_speak_timestamp
            .replace(request_to_speak_timestamp);

        self
    }

    /// Toggle the user's suppress state.
    ///
    /// # Caveats
    ///
    /// - You must have the `MUTE_MEMBERS` permission to unsupress yourself. You
    /// can always suppress yourself.
    pub fn suppress(mut self) -> Self {
        self.fields.suppress.replace(true);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from((
            crate::json_to_vec(&self.fields)?,
            Route::UpdateCurrentUserVoiceState {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateCurrentUserVoiceState<'_>, ());
