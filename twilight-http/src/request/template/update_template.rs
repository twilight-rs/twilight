use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
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

pub struct UpdateTemplatePath<'a> {
    guild_id: Id<GuildMarker>,
    template_code: &'a str,
}

/// Update the template's metadata, by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct UpdateTemplate<'a> {
    body: Result<UpdateTemplateBody<'a>, ValidationError>,
    http: &'a Client,
    path: UpdateTemplatePath<'a>,
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
            http,
            path: UpdateTemplatePath {
                guild_id,
                template_code,
            },
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
        self.body = self.body.and_then(|mut body| {
            validate_template_description(description)?;
            body.description.replace(description);

            Ok(body)
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
        self.body = self.body.and_then(|mut body| {
            validate_template_name(name)?;
            body.name.replace(name);

            Ok(body)
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

impl TryIntoRequest for UpdateTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("guilds")
            .id(self.path.guild_id)
            .resource("templates")
            .id(self.path.template_code)
            .build()
            .map_err(Error::path)?;
        let body = self.body.map_err(Error::validation)?;

        Request::builder(Method::Patch, path).json(&body).build()
    }
}
