use crate::{
    client::Client,
    error::Error,
    request::{multipart::Form, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::message::Sticker,
    id::{marker::GuildMarker, Id},
};
use twilight_validate::{
    request::{audit_reason as validate_audit_reason, ValidationError},
    sticker::{
        description as validate_description, name as validate_name, tags as validate_tags,
        StickerValidationError,
    },
};

struct CreateGuildStickerFields<'a> {
    description: &'a str,
    file: &'a [u8],
    name: &'a str,
    tags: &'a str,
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
    fields: Result<CreateGuildStickerFields<'a>, StickerValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
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
        let fields = Ok(CreateGuildStickerFields {
            description,
            file,
            name,
            tags,
        })
        .and_then(|fields| {
            validate_description(description)?;
            validate_name(name)?;
            validate_tags(tags)?;

            Ok(fields)
        });

        Self {
            fields,
            guild_id,
            http,
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
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::CreateGuildSticker {
            guild_id: self.guild_id.get(),
        });

        let form = Form::new()
            .part(b"description", fields.description.as_bytes())
            .part(b"file", fields.file)
            .part(b"name", fields.name.as_bytes())
            .part(b"tags", fields.tags.as_bytes());

        request = request.form(form);

        request.build()
    }
}
