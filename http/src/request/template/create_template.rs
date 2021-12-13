use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use serde::Serialize;
use twilight_model::{id::GuildId, template::Template};
use twilight_validate::misc::{
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
/// [`TemplateName`]: twilight_validate::misc::ValidationErrorType::TemplateName
#[must_use = "requests must be configured and executed"]
pub struct CreateTemplate<'a> {
    fields: CreateTemplateFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> CreateTemplate<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        name: &'a str,
    ) -> Result<Self, ValidationError> {
        validate_template_name(name)?;

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
    /// Returns an error of type [`TemplateDescription`] if the name length is
    /// too short or too long.
    ///
    /// [`TemplateDescription`]: twilight_validate::misc::ValidationErrorType::TemplateDescription
    pub fn description(mut self, description: &'a str) -> Result<Self, ValidationError> {
        validate_template_description(description)?;

        self.fields.description.replace(description);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Template> {
        let mut request = Request::builder(&Route::CreateTemplate {
            guild_id: self.guild_id.get(),
        });

        request = match request.json(&self.fields) {
            Ok(request) => request,
            Err(source) => return ResponseFuture::error(source),
        };

        self.http.request(request.build())
    }
}
