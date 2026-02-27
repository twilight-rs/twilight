use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ApplicationMarker, EmojiMarker},
};

#[cfg(not(target_os = "wasi"))]
use crate::response::{Response, ResponseFuture};
use crate::{
    Client, Error,
    request::{Request, TryIntoRequest},
    routing::Route,
};

pub struct DeleteApplicationEmoji<'a> {
    application_id: Id<ApplicationMarker>,
    emoji_id: Id<EmojiMarker>,
    http: &'a Client,
}

impl<'a> DeleteApplicationEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        emoji_id: Id<EmojiMarker>,
    ) -> Self {
        Self {
            application_id,
            emoji_id,
            http,
        }
    }
}

#[cfg(not(target_os = "wasi"))]
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

impl TryIntoRequest for DeleteApplicationEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::DeleteApplicationEmoji {
            application_id: self.application_id.get(),
            emoji_id: self.emoji_id.get(),
        }))
    }
}
