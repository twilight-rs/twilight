use crate::request::{prelude::*, validate};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::guild::Guild;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CreateGuildFromTemplateError {
    /// Name of the guild is either fewer than 2 UTF-16 characters or more than 100 UTF-16
    /// characters.
    NameInvalid {
        /// Provided name.
        name: String,
    },
}

impl Display for CreateGuildFromTemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NameInvalid { .. } => f.write_str("the guild name is invalid"),
        }
    }
}

impl Error for CreateGuildFromTemplateError {}

#[derive(Serialize)]
struct CreateGuildFromTemplateFields {
    name: String,
    icon: Option<String>,
}

/// Create a new guild based on a template.
///
/// This endpoint can only be used by bots in less than 10 guilds.
///
/// # Errors
///
/// Returns [`CreateGuildFromTemplateError::NameInvalid`] when the name is
/// invalid.
pub struct CreateGuildFromTemplate<'a> {
    fields: CreateGuildFromTemplateFields,
    fut: Option<Pending<'a, Guild>>,
    http: &'a Client,
    template_code: String,
}

impl<'a> CreateGuildFromTemplate<'a> {
    pub(crate) fn new(
        http: &'a Client,
        template_code: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<Self, CreateGuildFromTemplateError> {
        Self::_new(http, template_code.into(), name.into())
    }

    fn _new(
        http: &'a Client,
        template_code: String,
        name: String,
    ) -> Result<Self, CreateGuildFromTemplateError> {
        if !validate::guild_name(&name) {
            return Err(CreateGuildFromTemplateError::NameInvalid { name });
        }

        Ok(Self {
            fields: CreateGuildFromTemplateFields { name, icon: None },
            fut: None,
            http,
            template_code,
        })
    }

    /// Set the icon.
    ///
    /// This must be a Data URI, in the form of `data:image/{type};base64,{data}` where `{type}` is
    /// the image MIME type and `{data}` is the base64-encoded image. Refer to [the discord docs]
    /// for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/reference#image-data
    pub fn icon(self, icon: impl Into<String>) -> Self {
        self._icon(icon.into())
    }

    fn _icon(mut self, icon: String) -> Self {
        self.fields.icon.replace(icon);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
            Route::CreateGuildFromTemplate {
                template_code: self.template_code.clone(),
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateGuildFromTemplate<'_>, Guild);
