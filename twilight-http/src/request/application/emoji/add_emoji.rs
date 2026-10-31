use std::future::IntoFuture;

use crate::{
    Client, Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};

use serde::Serialize;
use twilight_model::{
    guild::Emoji,
    id::{Id, marker::ApplicationMarker},
};

#[derive(Serialize)]
struct AddApplicationEmojiBody<'a> {
    image: &'a str,
    name: &'a str,
}

pub struct AddApplicationEmojiPath {
    application_id: Id<ApplicationMarker>,
}

pub struct AddApplicationEmoji<'a> {
    body: AddApplicationEmojiBody<'a>,
    http: &'a Client,
    path: AddApplicationEmojiPath,
}

impl<'a> AddApplicationEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        name: &'a str,
        image: &'a str,
    ) -> Self {
        Self {
            body: AddApplicationEmojiBody { image, name },
            http,
            path: AddApplicationEmojiPath { application_id },
        }
    }
}

impl IntoFuture for AddApplicationEmoji<'_> {
    type Output = Result<Response<Emoji>, Error>;

    type IntoFuture = ResponseFuture<Emoji>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for AddApplicationEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("applications")
            .id(self.path.application_id)
            .resource("emojis")
            .build()
            .map_err(Error::path)?;

        Request::builder(Method::Post, path)
            .json(&self.body)
            .build()
    }
}
