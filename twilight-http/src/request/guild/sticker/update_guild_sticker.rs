use crate::{
    client::Client,
    error::Error as HttpError,
    request::{AuditLogReason, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::message::sticker::Sticker,
    id::{
        marker::{GuildMarker, StickerMarker},
        Id,
    },
};
use twilight_validate::{
    request::{audit_reason as validate_audit_reason, ValidationError},
    sticker::{
        description as validate_description, name as validate_name, tags as validate_tags,
        StickerValidationError,
    },
};

#[derive(Serialize)]
struct UpdateGuildStickerFields<'a> {
    description: Option<&'a str>,
    name: Option<&'a str>,
    tags: Option<&'a str>,
}

/// Updates a sticker in a guild, and returns the updated sticker.
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
/// let sticker_id = Id::new(2);
/// let sticker = client
///     .update_guild_sticker(guild_id, sticker_id)
///     .description("new description")?
///     .exec()
///     .await?
///     .model()
///     .await?;
///
/// println!("{sticker:#?}");
/// # Ok(()) }
/// ```
pub struct UpdateGuildSticker<'a> {
    fields: UpdateGuildStickerFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Option<&'a str>,
    sticker_id: Id<StickerMarker>,
}

impl<'a> UpdateGuildSticker<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        sticker_id: Id<StickerMarker>,
    ) -> Self {
        Self {
            guild_id,
            fields: UpdateGuildStickerFields {
                description: None,
                name: None,
                tags: None,
            },
            http,
            reason: None,
            sticker_id,
        }
    }

    /// Set the sticker's description.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`DescriptionInvalid`] if the length is invalid.
    ///
    /// [`DescriptionInvalid`]: twilight_validate::sticker::StickerValidationErrorType::DescriptionInvalid
    pub fn description(mut self, description: &'a str) -> Result<Self, StickerValidationError> {
        validate_description(description)?;

        self.fields.description = Some(description);

        Ok(self)
    }

    /// Set the sticker's name.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameInvalid`] if the length is invalid.
    ///
    /// [`NameInvalid`]: twilight_validate::sticker::StickerValidationErrorType::NameInvalid
    pub fn name(mut self, name: &'a str) -> Result<Self, StickerValidationError> {
        validate_name(name)?;

        self.fields.name = Some(name);

        Ok(self)
    }

    /// Set the sticker's tags.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`TagsInvalid`] if the length is invalid.
    ///
    /// [`TagsInvalid`]: twilight_validate::sticker::StickerValidationErrorType::TagsInvalid
    pub fn tags(mut self, tags: &'a str) -> Result<Self, StickerValidationError> {
        validate_tags(tags)?;

        self.fields.tags = Some(tags);

        Ok(self)
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

impl<'a> AuditLogReason<'a> for UpdateGuildSticker<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl TryIntoRequest for UpdateGuildSticker<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let request = Request::builder(&Route::UpdateGuildSticker {
            guild_id: self.guild_id.get(),
            sticker_id: self.sticker_id.get(),
        })
        .json(&self.fields)?;

        Ok(request.build())
    }
}
