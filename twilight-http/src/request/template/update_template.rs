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
struct UpdateTemplateBody<'a> {
    name: Option<&'a str>,
    description: Option<&'a str>,
}

#[derive(Serialize)]
pub struct UpdateTemplateFields<'a> {
    guild_id: Id<GuildMarker>,
    template_code: &'a str,
}

/// Update the template's metadata, by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct UpdateTemplate<'a> {
    body: Result<UpdateTemplateBody<'a>, ValidationError>,
    fields: UpdateTemplateFields<'a>,
    http: &'a Client,
}

impl<'a> UpdateTemplate<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        template_code: &'a str,
    ) -> Self {
        Self {
            body: Ok(UpdateTemplateBody {
                name: None,
                description: None,
            }),
            fields: UpdateTemplateFields {
                guild_id,
                template_code,
            },
            http,
        }
    }

    /// Set the description.
    ///
    /// This must be at most 120 characters in length.
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

    /// Set the name.
    ///
    /// This must be at least 1, and at most 100 characters in length.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`TemplateName`] if the name length is too
    /// short or too long.
    ///
    /// [`TemplateName`]: twilight_validate::request::ValidationErrorType::TemplateName
    pub fn name(mut self, name: &'a str) -> Self {
        self.body = self.body.and_then(|mut fields| {
            validate_template_name(name)?;
            fields.name.replace(name);

            Ok(fields)
        });

        self
    }
}

impl IntoFuture for UpdateTemplate<'_> {
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

impl<'a> Route for UpdateTemplate<'a> {
    type Fields = UpdateTemplateFields<'a>;

    const METHOD: Method = Method::Patch;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("guilds")
            .id(fields.guild_id)
            .resource("templates")
            .string_id(fields.template_code)
            .build()
    }
}

impl TryIntoRequest for UpdateTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;

        Request::builder_new::<Self>(self.fields)
            .json(&body)
            .build()
    }
}
