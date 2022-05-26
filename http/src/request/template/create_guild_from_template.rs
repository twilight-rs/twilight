use crate::{
    client::Client,
    error::Error as HttpError,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::guild::Guild;
use twilight_validate::request::{guild_name as validate_guild_name, ValidationError};

#[derive(Serialize)]
struct CreateGuildFromTemplateFields<'a> {
    name: &'a str,
    icon: Option<&'a str>,
}

/// Create a new guild based on a template.
///
/// This endpoint can only be used by bots in less than 10 guilds.
///
/// # Errors
///
/// Returns an error of type [`GuildName`] if the name length is too short or
/// too long.
///
/// [`GuildName`]: twilight_validate::request::ValidationErrorType::GuildName
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildFromTemplate<'a> {
    fields: CreateGuildFromTemplateFields<'a>,
    http: &'a Client,
    template_code: &'a str,
}

impl<'a> CreateGuildFromTemplate<'a> {
    pub(crate) fn new(
        http: &'a Client,
        template_code: &'a str,
        name: &'a str,
    ) -> Result<Self, ValidationError> {
        validate_guild_name(name)?;

        Ok(Self {
            fields: CreateGuildFromTemplateFields { name, icon: None },
            http,
            template_code,
        })
    }

    /// Set the icon.
    ///
    /// This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type
    /// and `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub const fn icon(mut self, icon: &'a str) -> Self {
        self.fields.icon = Some(icon);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Guild> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateGuildFromTemplate<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::CreateGuildFromTemplate {
            template_code: self.template_code,
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
