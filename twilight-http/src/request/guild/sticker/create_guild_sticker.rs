use crate::{
    client::Client,
    error::Error,
    request::{AuditLogReason, Method, Request, TryIntoRequest, multipart::Form, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::message::Sticker,
    id::{Id, marker::GuildMarker},
};
use twilight_validate::{
    request::{ValidationError, audit_reason as validate_audit_reason},
    sticker::{
        StickerValidationError, description as validate_description, name as validate_name,
        tags as validate_tags,
    },
};

struct CreateGuildStickerBody<'a> {
    description: &'a str,
    file: &'a [u8],
    name: &'a str,
    tags: &'a str,
}

struct CreateGuildStickerPath {
    guild_id: Id<GuildMarker>,
}

/// Creates a sticker in a guild, and returns the created sticker.
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
/// let guild_id = Id::new(1);
/// let sticker = client
///     .create_guild_sticker(
///         guild_id,
///         &"sticker name",
///         &"sticker description",
///         &"sticker,tags",
///         &[23, 23, 23, 23],
///     )
///     .await?
///     .model()
///     .await?;
///
/// println!("{sticker:#?}");
/// # Ok(()) }
/// ```
pub struct CreateGuildSticker<'a> {
    body: Result<CreateGuildStickerBody<'a>, StickerValidationError>,
    http: &'a Client,
    path: CreateGuildStickerPath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateGuildSticker<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        name: &'a str,
        description: &'a str,
        tags: &'a str,
        file: &'a [u8],
    ) -> Self {
        let body = Ok(CreateGuildStickerBody {
            description,
            file,
            name,
            tags,
        })
        .and_then(|body| {
            validate_description(description)?;
            validate_name(name)?;
            validate_tags(tags)?;

            Ok(body)
        });

        Self {
            body,
            http,
            path: CreateGuildStickerPath { guild_id },
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildSticker<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for CreateGuildSticker<'_> {
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

impl TryIntoRequest for CreateGuildSticker<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let CreateGuildStickerPath { guild_id } = self.path;

        let body = self.body.map_err(Error::validation)?;
        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("stickers")
            .build()
            .map_err(Error::path)?;

        let mut request = Request::builder(Method::Post, path);

        let form = Form::new()
            .part(b"description", body.description.as_bytes())
            .part(b"file", body.file)
            .part(b"name", body.name.as_bytes())
            .part(b"tags", body.tags.as_bytes());

        request = request.form(form);

        request.build()
    }
}
