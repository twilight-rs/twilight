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
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};

#[derive(Serialize)]
struct UpdateApplicationEmojiBody<'a> {
    name: &'a str,
}

pub struct UpdateApplicationEmojiPath {
    application_id: Id<ApplicationMarker>,
    emoji_id: Id<EmojiMarker>,
}

pub struct UpdateApplicationEmoji<'a> {
    body: UpdateApplicationEmojiBody<'a>,
    http: &'a Client,
    path: UpdateApplicationEmojiPath,
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
            http,
            path: UpdateApplicationEmojiPath {
                application_id,
                emoji_id,
            },
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

impl TryIntoRequest for UpdateApplicationEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("applications")
            .id(self.path.application_id)
            .resource("emojis")
            .id(self.path.emoji_id)
            .build()
            .map_err(Error::path)?;

        Request::builder(Method::Patch, path)
            .json(&self.body)
            .build()
    }
}
