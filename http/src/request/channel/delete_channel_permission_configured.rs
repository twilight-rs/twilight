use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{marker::ChannelMarker, Id};

/// Clear the permissions for a target ID in a channel.
///
/// The `target_id` is a `u64`, but it should point to a `Id<RoleMarker>` or a `Id<UserMarker>`.
#[must_use = "requests must be configured and executed"]
pub struct DeleteChannelPermissionConfigured<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
    reason: Option<&'a str>,
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
            reason: None,
            target_id,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteChannelPermissionConfigured<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for DeleteChannelPermissionConfigured<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeletePermissionOverwrite {
            channel_id: self.channel_id.get(),
            target_id: self.target_id,
        });

        if let Some(reason) = self.reason {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
