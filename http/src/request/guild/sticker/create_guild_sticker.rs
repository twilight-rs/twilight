use crate::{
    client::Client,
    request::{multipart::Form, validate_inner, AuditLogReason, AuditLogReasonError, Request},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{channel::message::Sticker, id::GuildId};

use super::{StickerValidationError, StickerValidationErrorType};

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
/// use twilight_model::{
///     channel::message::sticker::StickerId,
///     id::GuildId,
/// };
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = GuildId::new(1).expect("non zero");
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
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> CreateGuildSticker<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        name: &'a str,
        description: &'a str,
        tags: &'a str,
        file: &'a [u8],
    ) -> Result<Self, StickerValidationError> {
        if !validate_inner::sticker_description(description) {
            return Err(StickerValidationError {
                kind: StickerValidationErrorType::DescriptionInvalid,
            });
        }

        if !validate_inner::sticker_name(name) {
            return Err(StickerValidationError {
                kind: StickerValidationErrorType::NameInvalid,
            });
        }

        if !validate_inner::sticker_tags(tags) {
            return Err(StickerValidationError {
                kind: StickerValidationErrorType::TagsInvalid,
            });
        }

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
        let mut request = Request::builder(&Route::CreateGuildSticker {
            guild_id: self.guild_id.get(),
        });

        let mut form = Form::new();

        form.part("name".as_bytes(), self.fields.name.as_bytes());

        form.part("description".as_bytes(), self.fields.description.as_bytes());

        form.part("tags".as_bytes(), self.fields.tags.as_bytes());

        form.part("file".as_bytes(), self.fields.file);

        request = request.form(form);

        self.http.request(request.build())
    }
}

impl<'a> AuditLogReason<'a> for CreateGuildSticker<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}
