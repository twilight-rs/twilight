use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::ChannelMarker};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

struct DeleteChannelPermissionConfiguredPath {
    channel_id: Id<ChannelMarker>,
    target_id: u64,
}

/// Clear the permissions for a target ID in a channel.
///
/// The `target_id` is a `u64`, but it should point to a `Id<RoleMarker>` or a `Id<UserMarker>`.
#[must_use = "requests must be configured and executed"]
pub struct DeleteChannelPermissionConfigured<'a> {
    http: &'a Client,
    path: DeleteChannelPermissionConfiguredPath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> DeleteChannelPermissionConfigured<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        target_id: u64,
    ) -> Self {
        Self {
            http,
            path: DeleteChannelPermissionConfiguredPath {
                channel_id,
                target_id,
            },
            reason: Ok(None),
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
            channel_id: self.path.channel_id.get(),
            target_id: self.path.target_id,
        });

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
