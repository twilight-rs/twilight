use std::future::IntoFuture;

use serde::Serialize;
use twilight_model::oauth::{Application, ApplicationFlags, InstallParams};

use crate::{
    client::Client,
    error::Error,
    request::{Nullable, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};

#[derive(Serialize)]
struct UpdateCurrentUserApplicationFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    cover_image: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_install_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    flags: Option<ApplicationFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    install_params: Option<InstallParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interactions_endpoint_url: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role_connections_verification_url: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<&'a str>>,
}

#[must_use = "requests must be configured and executed"]
pub struct UpdateCurrentUserApplication<'a> {
    fields: UpdateCurrentUserApplicationFields<'a>,
    http: &'a Client,
}

impl<'a> UpdateCurrentUserApplication<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self {
            fields: UpdateCurrentUserApplicationFields {
                cover_image: None,
                custom_install_url: None,
                description: None,
                flags: None,
                icon: None,
                install_params: None,
                interactions_endpoint_url: None,
                role_connections_verification_url: None,
                tags: None,
            },
            http,
        }
    }

    /// Set the cover image of the application.
    pub const fn cover_image(mut self, cover_image: &'a str) -> Self {
        self.fields.cover_image = Some(Nullable(Some(cover_image)));

        self
    }

    /// Set the custom install URL of the application.
    pub const fn custom_install_url(mut self, custom_install_url: &'a str) -> Self {
        self.fields.custom_install_url = Some(custom_install_url);

        self
    }

    /// Set the description of the application.
    pub const fn description(mut self, description: &'a str) -> Self {
        self.fields.description = Some(description);

        self
    }

    /// Set the flags of the application.
    pub const fn flags(mut self, flags: ApplicationFlags) -> Self {
        self.fields.flags = Some(flags);

        self
    }

    /// Set the icon of the application.
    pub const fn icon(mut self, icon: &'a str) -> Self {
        self.fields.icon = Some(Nullable(Some(icon)));

        self
    }

    /// Set the install params of the application.
    #[allow(clippy::missing_const_for_fn)]
    pub fn install_params(mut self, install_params: InstallParams) -> Self {
        self.fields.install_params = Some(install_params);

        self
    }

    /// Set the interactions endpoint URL of the application.
    pub const fn interactions_endpoint_url(mut self, interactions_endpoint_url: &'a str) -> Self {
        self.fields.interactions_endpoint_url = Some(Nullable(Some(interactions_endpoint_url)));

        self
    }

    /// Set the role connections verification URL of the application.
    pub const fn role_connections_verification_url(
        mut self,
        role_connections_verification_url: &'a str,
    ) -> Self {
        self.fields.role_connections_verification_url = Some(role_connections_verification_url);

        self
    }

    /// Set the tags of the application.
    pub fn tags(mut self, tags: Vec<&'a str>) -> Self {
        self.fields.tags = Some(tags);

        self
    }
}

impl IntoFuture for UpdateCurrentUserApplication<'_> {
    type Output = Result<Response<Application>, Error>;

    type IntoFuture = ResponseFuture<Application>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateCurrentUserApplication<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateCurrentUserApplication);

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
