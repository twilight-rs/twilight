use std::future::IntoFuture;

use crate::{
    Client, Error, Response,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::ResponseFuture,
};
use twilight_model::{
    application::EmojiList,
    id::{Id, marker::ApplicationMarker},
};

pub struct ListApplicationEmojisFields {
    application_id: Id<ApplicationMarker>,
}

#[must_use = "requests must be configured and executed"]
pub struct ListApplicationEmojis<'a> {
    fields: ListApplicationEmojisFields,
    http: &'a Client,
}

impl<'a> ListApplicationEmojis<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            fields: ListApplicationEmojisFields { application_id },
            http,
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

impl Route for ListApplicationEmojis<'_> {
    type Fields = ListApplicationEmojisFields;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("emojis")
            .build()
    }
}

impl TryIntoRequest for ListApplicationEmojis<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
