use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
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
///     .description("new description")
///     .await?
///     .model()
///     .await?;
///
/// println!("{sticker:#?}");
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildSticker<'a> {
    fields: Result<UpdateGuildStickerFields<'a>, StickerValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
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
            fields: Ok(UpdateGuildStickerFields {
                description: None,
                name: None,
                tags: None,
            }),
            http,
            reason: Ok(None),
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
    pub fn description(mut self, description: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_description(description)?;
            fields.description = Some(description);

            Ok(fields)
        });

        self
    }

    /// Set the sticker's name.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`NameInvalid`] if the length is invalid.
    ///
    /// [`NameInvalid`]: twilight_validate::sticker::StickerValidationErrorType::NameInvalid
    pub fn name(mut self, name: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_name(name)?;
            fields.name = Some(name);

            Ok(fields)
        });

        self
    }

    /// Set the sticker's tags.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`TagsInvalid`] if the length is invalid.
    ///
    /// [`TagsInvalid`]: twilight_validate::sticker::StickerValidationErrorType::TagsInvalid
    pub fn tags(mut self, tags: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_tags(tags)?;
            fields.tags = Some(tags);

            Ok(fields)
        });

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateGuildSticker<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateGuildSticker<'_> {
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

impl TryIntoRequest for UpdateGuildSticker<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        let mut request = Request::builder(&Route::UpdateGuildSticker {
            guild_id: self.guild_id.get(),
            sticker_id: self.sticker_id.get(),
        })
        .json(&fields);

        if let Ok(Some(reason)) = self.reason {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
