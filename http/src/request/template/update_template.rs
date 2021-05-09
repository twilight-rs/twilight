use crate::request::prelude::*;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{id::GuildId, template::Template};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum UpdateTemplateError {
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

impl Display for UpdateTemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid { .. } => f.write_str("the template name is invalid"),
            Self::DescriptionTooLarge { .. } => {
                f.write_str("the template description is too large")
            }
        }
    }
}

impl Error for UpdateTemplateError {}

#[derive(Serialize)]
struct UpdateTemplateFields {
    name: Option<String>,
    description: Option<String>,
}

/// Update the template's metadata, by ID and code.
pub struct UpdateTemplate<'a> {
    fields: UpdateTemplateFields,
    fut: Option<Pending<'a, Template>>,
    guild_id: GuildId,
    http: &'a Client,
    template_code: String,
}

impl<'a> UpdateTemplate<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        template_code: impl Into<String>,
    ) -> Self {
        Self::_new(http, guild_id, template_code.into())
    }

    fn _new(http: &'a Client, guild_id: GuildId, template_code: String) -> Self {
        Self {
            fields: UpdateTemplateFields {
                name: None,
                description: None,
            },
            fut: None,
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
    /// Returns [`UpdateTemplateError::DescriptionTooLarge`] when the
    /// description is too large.
    pub fn description(self, description: impl Into<String>) -> Result<Self, UpdateTemplateError> {
        self._description(description.into())
    }

    fn _description(mut self, description: String) -> Result<Self, UpdateTemplateError> {
        if !validate::template_description(&description) {
            return Err(UpdateTemplateError::DescriptionTooLarge { description });
        }

        self.fields.description.replace(description);

        Ok(self)
    }

    /// Set the name.
    ///
    /// This must be at least 1, and at most 100 characters in length.
    ///
    /// # Errors
    ///
    /// Returns [`UpdateTemplateError::NameInvalid`] when the name is invalid.
    pub fn name(self, name: impl Into<String>) -> Result<Self, UpdateTemplateError> {
        self._name(name.into())
    }

    fn _name(mut self, name: String) -> Result<Self, UpdateTemplateError> {
        if !validate::template_name(&name) {
            return Err(UpdateTemplateError::NameInvalid { name });
        }

        self.fields.name.replace(name);

        Ok(self)
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
            Route::UpdateTemplate {
                guild_id: self.guild_id.0,
                template_code: self.template_code.clone(),
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateTemplate<'_>, Template);
