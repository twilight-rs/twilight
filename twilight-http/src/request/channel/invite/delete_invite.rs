use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

/// Delete an invite by its code.
///
/// Requires the [`MANAGE_CHANNELS`] permission on the channel this invite
/// belongs to, or [`MANAGE_GUILD`] to remove any invite across the guild.
///
/// [`MANAGE_CHANNELS`]: twilight_model::guild::Permissions::MANAGE_CHANNELS
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct DeleteInvite<'a> {
    code: &'a str,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteInvite<'a> {
    pub(crate) const fn new(http: &'a Client, code: &'a str) -> Self {
        Self {
            code,
            http,
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteInvite<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for DeleteInvite<'_> {
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

impl TryIntoRequest for DeleteInvite<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeleteInvite { code: self.code });

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
