use crate::{
    client::Client,
    request::{self, AuditLogReason, AuditLogReasonError, IntoRequest, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};

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
    reason: Option<&'a str>,
}

impl<'a> DeleteInvite<'a> {
    pub(crate) const fn new(http: &'a Client, code: &'a str) -> Self {
        Self {
            code,
            http,
            reason: None,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteInvite<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl IntoRequest for DeleteInvite<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        let mut request = Request::builder(&Route::DeleteInvite { code: self.code });

        if let Some(reason) = self.reason {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
