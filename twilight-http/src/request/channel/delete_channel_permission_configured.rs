use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{marker::ChannelMarker, Id};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

/// Clear the permissions for a target ID in a channel.
///
/// The `target_id` is a `u64`, but it should point to a `Id<RoleMarker>` or a `Id<UserMarker>`.
#[must_use = "requests must be configured and executed"]
pub struct DeleteChannelPermissionConfigured<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
    target_id: u64,
}

impl<'a> DeleteChannelPermissionConfigured<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        target_id: u64,
    ) -> Self {
        Self {
            channel_id,
            http,
            reason: Ok(None),
            target_id,
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteChannelPermissionConfigured<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for DeleteChannelPermissionConfigured<'_> {
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

impl TryIntoRequest for DeleteChannelPermissionConfigured<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeletePermissionOverwrite {
            channel_id: self.channel_id.get(),
            target_id: self.target_id,
        });

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
