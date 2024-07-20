use std::future::IntoFuture;

use crate::{
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
    Client, Error,
};

use serde::Serialize;
use twilight_model::{
    guild::Emoji,
    id::{marker::ApplicationMarker, Id},
};

#[derive(Serialize)]
struct AddApplicationEmojiFields<'a> {
    image: &'a str,
    name: &'a str,
}

pub struct AddApplicationEmoji<'a> {
    fields: AddApplicationEmojiFields<'a>,
    application_id: Id<ApplicationMarker>,
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
            fields: AddApplicationEmojiFields { image, name },
            application_id,
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

        request = request.json(&self.fields);

        request.build()
    }
}
