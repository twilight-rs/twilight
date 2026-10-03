use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::template::Template,
    id::{Id, marker::GuildMarker},
};
use twilight_validate::request::{
    ValidationError, template_description as validate_template_description,
    template_name as validate_template_name,
};

#[derive(Serialize)]
struct CreateTemplateBody<'a> {
    name: &'a str,
    description: Option<&'a str>,
}

pub struct CreateTemplateFields {
    guild_id: Id<GuildMarker>,
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
    body: Result<CreateTemplateBody<'a>, ValidationError>,
    fields: CreateTemplateFields,
    http: &'a Client,
}

impl<'a> CreateTemplate<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: Id<GuildMarker>, name: &'a str) -> Self {
        let body = Ok(CreateTemplateBody {
            name,
            description: None,
        })
        .and_then(|fields| {
            validate_template_name(name)?;

            Ok(fields)
        });

        Self {
            body,
            fields: CreateTemplateFields { guild_id },
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
        self.body = self.body.and_then(|mut fields| {
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

impl Route for CreateTemplate<'_> {
    type Fields = CreateTemplateFields;

    const METHOD: Method = Method::Post;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("guilds")
            .id(fields.guild_id)
            .resource("templates")
            .build()
    }
}

impl TryIntoRequest for CreateTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;

        Request::builder_new::<Self>(self.fields)
            .json(&body)
            .build()
    }
}
