use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::Emoji,
    id::{
        marker::{ApplicationMarker, EmojiMarker},
        Id,
    },
};

use crate::{
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
    Client, Error,
};

#[derive(Serialize)]
struct EditApplicationEmojiFields<'a> {
    name: &'a str,
}

pub struct UpdateApplicationEmoji<'a> {
    fields: EditApplicationEmojiFields<'a>,
    application_id: Id<ApplicationMarker>,
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
            fields: EditApplicationEmojiFields { name },
            application_id,
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

        request = request.json(&self.fields);

        request.build()
    }
}
