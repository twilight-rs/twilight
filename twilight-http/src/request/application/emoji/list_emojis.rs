use std::future::IntoFuture;

use crate::{
    Client, Error, Response,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::ResponseFuture,
};
use twilight_model::{
    application::EmojiList,
    id::{Id, marker::ApplicationMarker},
};

pub struct ListApplicationEmojisPath {
    application_id: Id<ApplicationMarker>,
}

#[must_use = "requests must be configured and executed"]
pub struct ListApplicationEmojis<'a> {
    http: &'a Client,
    path: ListApplicationEmojisPath,
}

impl<'a> ListApplicationEmojis<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            http,
            path: ListApplicationEmojisPath { application_id },
        }
    }
}

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
        let path = Path::builder()
            .resource("applications")
            .id(self.path.application_id)
            .resource("emojis")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
