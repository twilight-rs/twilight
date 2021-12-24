use crate::{
    client::Client,
    error::Error as HttpError,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    id::{marker::GuildMarker, Id},
    template::Template,
};
use twilight_validate::request::{
    template_description as validate_template_description, template_name as validate_template_name,
    ValidationError,
};

#[derive(Serialize)]
struct UpdateTemplateFields<'a> {
    name: Option<&'a str>,
    description: Option<&'a str>,
}

/// Update the template's metadata, by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct UpdateTemplate<'a> {
    fields: UpdateTemplateFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    template_code: &'a str,
}

impl<'a> UpdateTemplate<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        template_code: &'a str,
    ) -> Self {
        Self {
            fields: UpdateTemplateFields {
                name: None,
                description: None,
            },
            guild_id,
            http,
            template_code,
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
    pub fn description(mut self, description: &'a str) -> Result<Self, ValidationError> {
        validate_template_description(description)?;

        self.fields.description.replace(description);

        Ok(self)
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
    pub fn name(mut self, name: &'a str) -> Result<Self, ValidationError> {
        validate_template_name(name)?;

        self.fields.name.replace(name);

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

impl TryIntoRequest for UpdateTemplate<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let mut request = Request::builder(&Route::UpdateTemplate {
            guild_id: self.guild_id.get(),
            template_code: self.template_code,
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
