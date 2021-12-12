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
use twilight_model::{
    id::{marker::GuildMarker, Id},
    template::Template,
};

/// Error returned when the template can not be created as configured.
#[derive(Debug)]
pub struct CreateTemplateError {
    kind: CreateTemplateErrorType,
}

impl CreateTemplateError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CreateTemplateErrorType {
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
        CreateTemplateErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

impl Display for CreateTemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            CreateTemplateErrorType::NameInvalid { .. } => {
                f.write_str("the template name is invalid")
            }
            CreateTemplateErrorType::DescriptionTooLarge { .. } => {
                f.write_str("the template description is too large")
            }
        }
    }
}

impl Error for CreateTemplateError {}

/// Type of [`CreateTemplateError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CreateTemplateErrorType {
    /// Name of the template is invalid.
    NameInvalid,
    /// Description of the template is invalid.
    DescriptionTooLarge,
}

#[derive(Serialize)]
struct CreateTemplateFields<'a> {
    name: &'a str,
    description: Option<&'a str>,
}

/// Create a template from the current state of the guild.
///
/// Requires the `MANAGE_GUILD` permission. The name must be at least 1 and at
/// most 100 characters in length.
///
/// # Errors
///
/// Returns a [`CreateTemplateErrorType::NameInvalid`] error type if the name is
/// invalid.
#[must_use = "requests must be configured and executed"]
pub struct CreateTemplate<'a> {
    fields: CreateTemplateFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> CreateTemplate<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        name: &'a str,
    ) -> Result<Self, CreateTemplateError> {
        if !validate_inner::template_name(&name) {
            return Err(CreateTemplateError {
                kind: CreateTemplateErrorType::NameInvalid,
            });
        }

        Ok(Self {
            fields: CreateTemplateFields {
                name,
                description: None,
            },
            guild_id,
            http,
        })
    }

    /// Set the template's description.
    ///
    /// This must be less than or equal to 120 characters in length.
    ///
    /// # Errors
    ///
    /// Returns a [`CreateTemplateErrorType::DescriptionTooLarge`] error type if
    /// the description is too large.
    pub fn description(mut self, description: &'a str) -> Result<Self, CreateTemplateError> {
        if !validate_inner::template_description(description) {
            return Err(CreateTemplateError {
                kind: CreateTemplateErrorType::DescriptionTooLarge,
            });
        }

        self.fields.description.replace(description);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Template> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateTemplate<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::CreateTemplate {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
