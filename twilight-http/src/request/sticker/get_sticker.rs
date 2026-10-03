use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::message::sticker::Sticker,
    id::{Id, marker::StickerMarker},
};

pub struct GetStickerFields {
    sticker_id: Id<StickerMarker>,
}

/// Returns a single sticker by its ID.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let id = Id::new(123);
/// let sticker = client.sticker(id).await?.model().await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetSticker<'a> {
    fields: GetStickerFields,
    http: &'a Client,
}

impl<'a> GetSticker<'a> {
    pub(crate) const fn new(http: &'a Client, sticker_id: Id<StickerMarker>) -> Self {
        Self {
            fields: GetStickerFields { sticker_id },
            http,
        }
    }
}

impl IntoFuture for GetSticker<'_> {
    type Output = Result<Response<Sticker>, Error>;

    type IntoFuture = ResponseFuture<Sticker>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl Route for GetSticker<'_> {
    type Fields = GetStickerFields;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("stickers")
            .id(fields.sticker_id)
            .build()
    }
}

impl TryIntoRequest for GetSticker<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}
