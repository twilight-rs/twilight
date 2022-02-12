use crate::{
    client::Client,
    error::Error,
    request::{multipart::Form, AuditLogReason, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
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
///         &[23,23,23,23]
///     )?
///     .exec()
///     .await?
///     .model()
///     .await?;
///
/// println!("{:#?}", sticker);
/// # Ok(()) }
/// ```
pub struct CreateGuildSticker<'a> {
    fields: CreateGuildStickerFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> CreateGuildSticker<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        name: &'a str,
        description: &'a str,
        tags: &'a str,
        file: &'a [u8],
    ) -> Result<Self, StickerValidationError> {
        validate_description(description)?;

        validate_name(name)?;

        validate_tags(tags)?;

        Ok(Self {
            fields: CreateGuildStickerFields {
                description,
                file,
                name,
                tags,
            },
            guild_id,
            http,
            reason: None,
        })
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Sticker> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildSticker<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl TryIntoRequest for CreateGuildSticker<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::CreateGuildSticker {
            guild_id: self.guild_id.get(),
        });

        let form = Form::new()
            .part(b"description", self.fields.description.as_bytes())
            .part(b"file", self.fields.file)
            .part(b"name", self.fields.name.as_bytes())
            .part(b"tags", self.fields.tags.as_bytes());

        request = request.form(form);

        Ok(request.build())
    }
}
