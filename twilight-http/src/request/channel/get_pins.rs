use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::channel::message::PinsListing;
use twilight_model::id::{Id, marker::ChannelMarker};
use twilight_model::util::Timestamp;
use twilight_validate::request::{ValidationError, pin_limit as validate_pin_limit};

pub struct GetPinsQueryFields {
    before: Option<Timestamp>,
    limit: Option<i32>,
}

/// Get the pins of a channel.
#[must_use = "requests must be configured and executed"]
pub struct GetPins<'a> {
    channel_id: Id<ChannelMarker>,
    fields: Result<GetPinsQueryFields, ValidationError>,
    http: &'a Client,
}

impl<'a> GetPins<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            channel_id,
            fields: Ok(GetPinsQueryFields {
                before: None,
                limit: None,
            }),
            http,
        }
    }

    pub const fn before(mut self, timestamp: Timestamp) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.before = Some(timestamp);
        }

        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_pin_limit(limit)?;

            fields.limit = Some(limit);

            Ok(fields)
        });

        self
    }
}

impl IntoFuture for GetPins<'_> {
    type Output = Result<Response<PinsListing>, Error>;

    type IntoFuture = ResponseFuture<PinsListing>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetPins<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetPins {
            channel_id: self.channel_id.get(),
            limit: fields.limit,
            before: fields.before.map(|t| t.iso_8601().to_string()),
        }))
    }
}
