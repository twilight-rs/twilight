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
struct UpdateTemplateFields<'a> {
    name: Option<&'a str>,
    description: Option<&'a str>,
}

/// Update the template's metadata, by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct UpdateTemplate<'a> {
    fields: Result<UpdateTemplateFields<'a>, ValidationError>,
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
            fields: Ok(UpdateTemplateFields {
                name: None,
                description: None,
            }),
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
    pub fn description(mut self, description: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
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
        self.fields = self.fields.and_then(|mut fields| {
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

impl TryIntoRequest for UpdateTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::UpdateTemplate {
            guild_id: self.guild_id.get(),
            template_code: self.template_code,
        })
        .json(&fields)
        .build()
    }
}
