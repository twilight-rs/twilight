use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::GuildMarker};

struct DeleteTemplatePath<'a> {
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
        Ok(Request::from_route(&Route::DeleteTemplate {
            guild_id: self.path.guild_id.get(),
            template_code: self.path.template_code,
        }))
    }
}
