use std::future::IntoFuture;

#[cfg(not(target_os = "wasi"))]
use crate::{Response, response::ResponseFuture};

use crate::{
    Client, Error,
    request::{Request, TryIntoRequest},
    routing::Route,
};
use twilight_model::{
    application::EmojiList,
    id::{Id, marker::ApplicationMarker},
};

#[must_use = "requests must be configured and executed"]
pub struct ListApplicationEmojis<'a> {
    http: &'a Client,
    application_id: Id<ApplicationMarker>,
}

impl<'a> ListApplicationEmojis<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            http,
            application_id,
        }
    }
}

#[cfg(not(target_os = "wasi"))]
impl IntoFuture for ListApplicationEmojis<'_> {
    type Output = Result<Response<EmojiList>, Error>;

    type IntoFuture = ResponseFuture<EmojiList>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for ListApplicationEmojis<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetApplicationEmojis {
            application_id: self.application_id.get(),
        }))
    }
}
