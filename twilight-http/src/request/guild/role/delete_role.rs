use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker},
    Id,
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

/// Delete a role in a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct DeleteRole<'a> {
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    role_id: Id<RoleMarker>,
    reason: Option<&'a str>,
}

impl<'a> DeleteRole<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
    ) -> Self {
        Self {
            guild_id,
            http,
            role_id,
            reason: None,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        self.into_future()
    }
}

impl<'a> AuditLogReason<'a> for DeleteRole<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl IntoFuture for DeleteRole<'_> {
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

impl TryIntoRequest for DeleteRole<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeleteRole {
            guild_id: self.guild_id.get(),
            role_id: self.role_id.get(),
        });

        if let Some(reason) = &self.reason {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
