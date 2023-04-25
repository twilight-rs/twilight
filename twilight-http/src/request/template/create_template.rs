use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::template::Template,
    id::{marker::GuildMarker, Id},
};
use twilight_validate::request::{
    template_description as validate_template_description, template_name as validate_template_name,
    ValidationError,
};

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
/// Returns an error of type [`TemplateName`] if the name length is too short or
/// too long.
///
/// [`TemplateName`]: twilight_validate::request::ValidationErrorType::TemplateName
#[must_use = "requests must be configured and executed"]
pub struct CreateTemplate<'a> {
    fields: Result<CreateTemplateFields<'a>, ValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> CreateTemplate<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: Id<GuildMarker>, name: &'a str) -> Self {
        let fields = Ok(CreateTemplateFields {
            name,
            description: None,
        })
        .and_then(|fields| {
            validate_template_name(name)?;

            Ok(fields)
        });

        Self {
            fields,
            guild_id,
            http,
        }
    }

    /// Set the template's description.
    ///
    /// This must be less than or equal to 120 characters in length.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`TemplateDescription`] if the name length is
    /// too short or too long.
    ///
    /// [`TemplateDescription`]: twilight_validate::request::ValidationErrorType::TemplateDescription
    pub fn description(mut self, description: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_template_description(description)?;

            fields.description.replace(description);

            Ok(fields)
        });

        self
    }
}

impl IntoFuture for CreateTemplate<'_> {
    type Output = Result<Response<Template>, Error>;

    type IntoFuture = ResponseFuture<Template>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::CreateTemplate {
            guild_id: self.guild_id.get(),
        })
        .json(&fields)
        .build()
    }
}
