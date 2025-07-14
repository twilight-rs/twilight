use crate::routing::Route;
use crate::{
    client::Client,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    Error,
};
use std::future::IntoFuture;
use twilight_model::guild::SoundboardSound;

#[must_use = "requests must be configured and executed"]
pub struct GetDefaultSoundboardSounds<'a> {
    http: &'a Client,
}

impl<'a> GetDefaultSoundboardSounds<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }
}

impl IntoFuture for GetDefaultSoundboardSounds<'_> {
    type Output = Result<Response<ListBody<SoundboardSound>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<SoundboardSound>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetDefaultSoundboardSounds<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetSoundboardDefaultSounds))
    }
}
