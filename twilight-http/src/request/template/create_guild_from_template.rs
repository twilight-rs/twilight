use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::guild::Guild;
use twilight_validate::request::{guild_name as validate_guild_name, ValidationError};

#[derive(Serialize)]
struct CreateGuildFromTemplateFields<'a> {
    name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a str>,
}

/// Create a new guild based on a template.
///
/// This endpoint can only be used by bots in less than 10 guilds.
///
/// # Errors
///
/// Returns an error of type [`GuildName`] if the name length is too short or
/// too long.
///
/// [`GuildName`]: twilight_validate::request::ValidationErrorType::GuildName
#[must_use = "requests must be configured and executed"]
pub struct CreateGuildFromTemplate<'a> {
    fields: Result<CreateGuildFromTemplateFields<'a>, ValidationError>,
    http: &'a Client,
    template_code: &'a str,
}

impl<'a> CreateGuildFromTemplate<'a> {
    pub(crate) fn new(http: &'a Client, template_code: &'a str, name: &'a str) -> Self {
        let fields = Ok(CreateGuildFromTemplateFields { name, icon: None }).and_then(|fields| {
            validate_guild_name(name)?;

            Ok(fields)
        });

        Self {
            fields,
            http,
            template_code,
        }
    }

    /// Set the icon.
    ///
    /// This must be a Data URI, in the form of
    /// `data:image/{type};base64,{data}` where `{type}` is the image MIME type
    /// and `{data}` is the base64-encoded image. See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub fn icon(mut self, icon: &'a str) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.icon = Some(icon);
        }

        self
    }
}

impl IntoFuture for CreateGuildFromTemplate<'_> {
    type Output = Result<Response<Guild>, Error>;

    type IntoFuture = ResponseFuture<Guild>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateGuildFromTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::CreateGuildFromTemplate {
            template_code: self.template_code,
        })
        .json(&fields)
        .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_create_guild_from_template() -> Result<(), Box<dyn Error>> {
        let client = Client::new("token".into());

        {
            let expected = r#"{"name":"New Guild"}"#;
            let actual =
                CreateGuildFromTemplate::new(&client, "code", "New Guild").try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }

        {
            let expected = r#"{"name":"New Guild","icon":"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI"}"#;
            let actual = CreateGuildFromTemplate::new(&client, "code", "New Guild")
            .icon("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAUAAAAFCAYAAACNbyblAAAAHElEQVQI")
            .try_into_request()?;

            assert_eq!(Some(expected.as_bytes()), actual.body());
        }
        Ok(())
    }
}
