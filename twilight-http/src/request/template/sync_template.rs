use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::template::Template,
    id::{Id, marker::GuildMarker},
};

pub struct SyncTemplatePath<'a> {
    guild_id: Id<GuildMarker>,
    template_code: &'a str,
}

/// Sync a template to the current state of the guild, by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct SyncTemplate<'a> {
    http: &'a Client,
    path: SyncTemplatePath<'a>,
}

impl<'a> SyncTemplate<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        template_code: &'a str,
    ) -> Self {
        Self {
            http,
            path: SyncTemplatePath {
                guild_id,
                template_code,
            },
        }
    }
}

impl IntoFuture for SyncTemplate<'_> {
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

impl TryIntoRequest for SyncTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("guilds")
            .id(self.path.guild_id)
            .resource("templates")
            .id(self.path.template_code)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Put, path))
    }
}
