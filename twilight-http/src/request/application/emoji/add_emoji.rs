use std::future::IntoFuture;

use crate::{
    Client, Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
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

pub struct AddApplicationEmoji<'a> {
    application_id: Id<ApplicationMarker>,
    body: AddApplicationEmojiBody<'a>,
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
            application_id,
            body: AddApplicationEmojiBody { image, name },
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

impl TryIntoRequest for AddApplicationEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::AddApplicationEmoji {
            application_id: self.application_id.get(),
        });

        request = request.json(&self.body);

        request.build()
    }
}
