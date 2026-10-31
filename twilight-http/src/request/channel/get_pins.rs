use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::channel::message::PinsListing;
use twilight_model::id::{Id, marker::ChannelMarker};
use twilight_model::util::Timestamp;
use twilight_validate::request::{ValidationError, pin_limit as validate_pin_limit};

struct GetPinsPath {
    before: Option<Timestamp>,
    channel_id: Id<ChannelMarker>,
    limit: Option<u16>,
}

/// Get the pins of a channel.
#[must_use = "requests must be configured and executed"]
pub struct GetPins<'a> {
    http: &'a Client,
    path: Result<GetPinsPath, ValidationError>,
}

impl<'a> GetPins<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: Ok(GetPinsPath {
                before: None,
                channel_id,
                limit: None,
            }),
        }
    }

    /// Sets the timestamp filter to only retrieve pins before the provided timestamp.
    ///
    /// [docs]
    ///
    /// [docs]: https://discord.com/developers/docs/resources/message#get-channel-pins-query-string-params
    pub const fn before(mut self, timestamp: Timestamp) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.before = Some(timestamp);
        }

        self
    }

    /// Sets the limit of pins to retrieve in this request. (1-50) (default: 50)
    ///
    /// [docs]
    ///
    /// [docs]: https://discord.com/developers/docs/resources/message#get-channel-pins-query-string-params
    pub fn limit(mut self, limit: u16) -> Self {
        self.path = self.path.and_then(|mut path| {
            validate_pin_limit(limit)?;

            path.limit = Some(limit);

            Ok(path)
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
        let GetPinsPath {
            before,
            channel_id,
            limit,
        } = self.path.map_err(Error::validation)?;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("messages")
            .subresource("pins")
            .parameter(("before", before))
            .parameter(("limit", limit))
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
