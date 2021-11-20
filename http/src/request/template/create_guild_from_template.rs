use crate::{
    client::Client,
    error::Error as HttpError,
    request::{validate_inner, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::guild::Guild;

#[derive(Debug)]
pub struct CreateGuildFromTemplateError {
    kind: CreateGuildFromTemplateErrorType,
}

impl CreateGuildFromTemplateError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CreateGuildFromTemplateErrorType {
        &self.kind
    }

    /// Consumes the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type nad the source error.
    #[must_use = "consuming the error int its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        CreateGuildFromTemplateErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for CreateGuildFromTemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            CreateGuildFromTemplateErrorType::NameInvalid { .. } => {
                f.write_str("the guild name is invalid")
            }
        }
    }
}

impl Error for CreateGuildFromTemplateError {}

/// Type of [`CreateGuildFromTemplateError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CreateGuildFromTemplateErrorType {
    /// Name of the guild is either fewer than 2 UTF-16 characters or more than 100 UTF-16
    /// characters.
    NameInvalid,
}

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
/// Returns a [`CreateGuildFromTemplateErrorType::NameInvalid`] error type if
/// the name is invalid.
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
    ) -> Result<Self, CreateGuildFromTemplateError> {
        if !validate_inner::guild_name(&name) {
            return Err(CreateGuildFromTemplateError {
                kind: CreateGuildFromTemplateErrorType::NameInvalid,
            });
        }

        Ok(Self {
            fields: CreateGuildFromTemplateFields { name, icon: None },
            http,
            template_code,
        })
    }

    /// Set the icon.
    ///
    /// This must be a Data URI, in the form of `data:image/{type};base64,{data}` where `{type}` is
    /// the image MIME type and `{data}` is the base64-encoded image. Refer to [the discord docs]
    /// for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/reference#image-data
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
