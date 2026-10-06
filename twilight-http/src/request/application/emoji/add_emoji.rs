use std::future::IntoFuture;

use crate::{
    Client, Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
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

pub struct AddApplicationEmojiFields {
    application_id: Id<ApplicationMarker>,
}

pub struct AddApplicationEmoji<'a> {
    body: AddApplicationEmojiBody<'a>,
    fields: AddApplicationEmojiFields,
    http: &'a Client,
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
            fields: AddApplicationEmojiFields { application_id },
            http,
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

impl Route for AddApplicationEmoji<'_> {
    type Fields = AddApplicationEmojiFields;

    const METHOD: Method = Method::Post;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("emojis")
            .build()
    }
}

impl TryIntoRequest for AddApplicationEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder_new::<Self>(self.fields)
            .json(&self.body)
            .build()
    }
}
