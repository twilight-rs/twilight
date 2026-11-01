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
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};

#[derive(Serialize)]
struct UpdateApplicationEmojiBody<'a> {
    name: &'a str,
}

pub struct UpdateApplicationEmoji<'a> {
    application_id: Id<ApplicationMarker>,
    body: UpdateApplicationEmojiBody<'a>,
    emoji_id: Id<EmojiMarker>,
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
            application_id,
            body: UpdateApplicationEmojiBody { name },
            emoji_id,
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

impl TryIntoRequest for UpdateApplicationEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateApplicationEmoji {
            application_id: self.application_id.get(),
            emoji_id: self.emoji_id.get(),
        });

        request = request.json(&self.body);

        request.build()
    }
}
