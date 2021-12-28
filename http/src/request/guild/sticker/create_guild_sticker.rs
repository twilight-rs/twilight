use crate::{
    client::Client,
    error::Error,
    request::{
        multipart::{self, Form},
        AuditLogReason, AuditLogReasonError, Request, TryIntoRequest,
    },
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    channel::message::Sticker,
    id::{marker::GuildMarker, Id},
};
use twilight_validate::sticker::{
    description as validate_description, name as validate_name, tags as validate_tags,
    StickerValidationError,
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
/// let guild_id = Id::new(1).expect("non zero");
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
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for CreateGuildSticker<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        const BOUNDARY_TERMINATOR: &[u8; 2] = b"--";
        const DESCRIPTION: &[u8; 11] = b"description";
        const FILE: &[u8; 4] = b"file";
        const KEY_POST: &[u8; 1] = br#"""#;
        const KEY_PRE: &[u8; 38] = br#"Content-Disposition: form-data; name=""#;
        const NAME: &[u8; 4] = b"name";
        const NEWLINE: &[u8; 2] = b"\r\n";
        const TAGS: &[u8; 4] = b"tags";

        let mut request = Request::builder(&Route::CreateGuildSticker {
            guild_id: self.guild_id.get(),
        });

        let boundary = multipart::random_boundary();

        let mut capacity = 0;

        capacity += BOUNDARY_TERMINATOR.len() * 6;
        capacity += boundary.len() * 5;
        capacity += KEY_PRE.len() * 4;
        capacity += KEY_POST.len() * 4;
        capacity += NEWLINE.len() * 12;

        capacity += NAME.len();
        capacity += self.fields.name.len();

        capacity += DESCRIPTION.len();
        capacity += self.fields.description.len();

        capacity += TAGS.len();
        capacity += self.fields.tags.len();

        capacity += FILE.len();
        capacity += self.fields.file.len();

        let mut buffer = Vec::with_capacity(capacity);

        // Write the first boundary.
        buffer.extend(BOUNDARY_TERMINATOR);
        buffer.extend(&boundary);
        buffer.extend(NEWLINE);

        // Write the name to the form.
        buffer.extend(KEY_PRE);
        buffer.extend(NAME);
        buffer.extend(KEY_POST);
        buffer.extend(NEWLINE);
        buffer.extend(NEWLINE);
        buffer.extend(self.fields.name.as_bytes());
        buffer.extend(NEWLINE);
        buffer.extend(BOUNDARY_TERMINATOR);
        buffer.extend(&boundary);
        buffer.extend(NEWLINE);

        // Write the description to the form.
        buffer.extend(KEY_PRE);
        buffer.extend(DESCRIPTION);
        buffer.extend(KEY_POST);
        buffer.extend(NEWLINE);
        buffer.extend(NEWLINE);
        buffer.extend(self.fields.description.as_bytes());
        buffer.extend(NEWLINE);
        buffer.extend(BOUNDARY_TERMINATOR);
        buffer.extend(&boundary);
        buffer.extend(NEWLINE);

        // Write the tags to the form.
        buffer.extend(KEY_PRE);
        buffer.extend(TAGS);
        buffer.extend(KEY_POST);
        buffer.extend(NEWLINE);
        buffer.extend(NEWLINE);
        buffer.extend(self.fields.tags.as_bytes());
        buffer.extend(NEWLINE);
        buffer.extend(BOUNDARY_TERMINATOR);
        buffer.extend(&boundary);
        buffer.extend(NEWLINE);

        // Write the file to the form.
        buffer.extend(KEY_PRE);
        buffer.extend(FILE);
        buffer.extend(KEY_POST);
        buffer.extend(NEWLINE);
        buffer.extend(NEWLINE);
        buffer.extend(self.fields.file);
        buffer.extend(NEWLINE);
        buffer.extend(BOUNDARY_TERMINATOR);
        buffer.extend(&boundary);
        buffer.extend(NEWLINE);

        request = request.form(Form::new(boundary, buffer));

        Ok(request.build())
    }
}
