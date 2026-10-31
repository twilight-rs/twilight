use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::GuildMarker};

pub struct DeleteTemplatePath<'a> {
    guild_id: Id<GuildMarker>,
    template_code: &'a str,
}

/// Delete a template by ID and code.
#[must_use = "requests must be configured and executed"]
pub struct DeleteTemplate<'a> {
    http: &'a Client,
    path: DeleteTemplatePath<'a>,
}

impl<'a> DeleteTemplate<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        template_code: &'a str,
    ) -> Self {
        Self {
            http,
            path: DeleteTemplatePath {
                guild_id,
                template_code,
            },
        }
    }
}

impl IntoFuture for DeleteTemplate<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("guilds")
            .id(self.path.guild_id)
            .resource("templates")
            .id(self.path.template_code)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Delete, path))
    }
}
