use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{id::GuildId, template::Template};

/// Error returned when the template can not be created as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CreateTemplateError {
    /// Name of the template is invalid.
    NameInvalid {
        /// Provided name.
        name: String,
    },
    /// Description of the template is invalid.
    DescriptionTooLarge {
        /// Provided description.
        description: String,
    },
}

impl Display for CreateTemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid { .. } => f.write_str("the template name is invalid"),
            Self::DescriptionTooLarge { .. } => {
                f.write_str("the template description is too large")
            }
        }
    }
}

impl Error for CreateTemplateError {}

#[derive(Serialize)]
struct CreateTemplateFields {
    name: String,
    description: Option<String>,
}

/// Create a template from the current state of the guild.
///
/// Requires the `MANAGE_GUILD` permission. The name must be at least 1 and at
/// most 100 characters in length.
///
/// # Errors
///
/// Returns [`CreateTemplateError::NameInvalid`] when the name is invalid.
pub struct CreateTemplate<'a> {
    fields: CreateTemplateFields,
    fut: Option<Pending<'a, Template>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> CreateTemplate<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        name: impl Into<String>,
    ) -> Result<Self, CreateTemplateError> {
        Self::_new(http, guild_id, name.into())
    }

    fn _new(
        http: &'a Client,
        guild_id: GuildId,
        name: String,
    ) -> Result<Self, CreateTemplateError> {
        if !validate::template_name(&name) {
            return Err(CreateTemplateError::NameInvalid { name });
        }

        Ok(Self {
            fields: CreateTemplateFields {
                name,
                description: None,
            },
            guild_id,
            fut: None,
            http,
        })
    }

    /// Set the template's description.
    ///
    /// This must be less than or equal to 120 characters in length.
    ///
    /// # Errors
    ///
    /// Returns [`CreateTemplateError::DescriptionTooLarge`] when the
    /// description is too large.
    pub fn description(self, description: impl Into<String>) -> Result<Self, CreateTemplateError> {
        self._description(description.into())
    }

    fn _description(mut self, description: String) -> Result<Self, CreateTemplateError> {
        if !validate::template_description(&description) {
            return Err(CreateTemplateError::DescriptionTooLarge { description });
        }

        self.fields.description.replace(description);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
            Route::CreateTemplate {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateTemplate<'_>, Template);
