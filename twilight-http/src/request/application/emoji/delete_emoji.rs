use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ApplicationMarker, EmojiMarker},
};

use crate::{
    Client, Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};

pub(in crate::request) struct DeleteApplicationEmojiPath {
    application_id: Id<ApplicationMarker>,
    emoji_id: Id<EmojiMarker>,
}

pub struct DeleteApplicationEmoji<'a> {
    http: &'a Client,
    path: DeleteApplicationEmojiPath,
}

impl<'a> DeleteApplicationEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        emoji_id: Id<EmojiMarker>,
    ) -> Self {
        Self {
            http,
            path: DeleteApplicationEmojiPath {
                application_id,
                emoji_id,
            },
        }
    }
}

impl IntoFuture for DeleteApplicationEmoji<'_> {
    type Output = Result<Response<()>, Error>;

    type IntoFuture = ResponseFuture<()>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl Route for DeleteApplicationEmoji<'_> {
    type Fields = DeleteApplicationEmojiPath;

    const METHOD: Method = Method::Delete;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("emojis")
            .id(fields.emoji_id)
            .build()
    }
}

impl TryIntoRequest for DeleteApplicationEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder_new::<Self>(self.path).build()
    }
}
