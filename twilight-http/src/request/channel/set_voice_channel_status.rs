use crate::client::Client;
use crate::request::{Request, TryIntoRequest};
use crate::response::marker::EmptyBody;
use crate::response::ResponseFuture;
use crate::routing::Route;
use crate::{Error, Response};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{marker::ChannelMarker, Id};
use twilight_validate::channel::{status as validate_status, ChannelValidationError};

#[derive(Serialize)]
pub(crate) struct SetVoiceChannelStatusFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<&'a str>,
}

/// Sets the status of a voice channel.
#[must_use = "requests must be configured and executed"]
pub struct SetVoiceChannelStatus<'a> {
    channel_id: Id<ChannelMarker>,
    fields: Result<SetVoiceChannelStatusFields<'a>, ChannelValidationError>,
    http: &'a Client,
}

impl<'a> SetVoiceChannelStatus<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            channel_id,
            fields: Ok(SetVoiceChannelStatusFields { status: None }),
            http,
        }
    }

    pub fn status(mut self, status: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_status(status)?;
            fields.status.replace(status);

            Ok(fields)
        });

        self
    }
}

impl IntoFuture for SetVoiceChannelStatus<'_> {
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

impl TryIntoRequest for SetVoiceChannelStatus<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::SetVoiceChannelStatus {
            channel_id: self.channel_id.get(),
        })
        .json(&fields)
        .build()
    }
}
