use twilight_model::guild::SoundboardSound;

use crate::{
    Client, Error, Response,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};

#[must_use]
pub struct ListDefaultSoundboardSounds<'a> {
    http: &'a Client,
}

impl<'a> ListDefaultSoundboardSounds<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }
}

impl IntoFuture for ListDefaultSoundboardSounds<'_> {
    type Output = Result<Response<Vec<SoundboardSound>>, Error>;

    type IntoFuture = ResponseFuture<Vec<SoundboardSound>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for ListDefaultSoundboardSounds<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::ListDefaultSoundboardSounds).build()
    }
}
