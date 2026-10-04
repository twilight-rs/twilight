use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::Emoji,
    id::{
        Id,
        marker::{ApplicationMarker, EmojiMarker},
    },
};

use crate::{
    Client, Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};

#[derive(Serialize)]
struct UpdateApplicationEmojiBody<'a> {
    name: &'a str,
}

pub(crate) struct UpdateApplicationEmojiFields {
    application_id: Id<ApplicationMarker>,
    emoji_id: Id<EmojiMarker>,
}

pub struct UpdateApplicationEmoji<'a> {
    body: UpdateApplicationEmojiBody<'a>,
    fields: UpdateApplicationEmojiFields,
    http: &'a Client,
}

impl<'a> UpdateApplicationEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        emoji_id: Id<EmojiMarker>,
        name: &'a str,
    ) -> Self {
        Self {
            body: UpdateApplicationEmojiBody { name },
            fields: UpdateApplicationEmojiFields {
                application_id,
                emoji_id,
            },
            http,
        }
    }
}

impl IntoFuture for UpdateApplicationEmoji<'_> {
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

impl Route for UpdateApplicationEmoji<'_> {
    type Fields = UpdateApplicationEmojiFields;

    const METHOD: Method = Method::Patch;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("emojis")
            .id(fields.emoji_id)
            .build()
    }
}

impl TryIntoRequest for UpdateApplicationEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder_new::<Self>(self.fields)
            .json(&self.body)
            .build()
    }
}
