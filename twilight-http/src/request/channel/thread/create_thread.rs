use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::{Channel, ChannelType, thread::AutoArchiveDuration},
    id::{Id, marker::ChannelMarker},
};
use twilight_validate::{
    channel::{ChannelValidationError, is_thread as validate_is_thread, name as validate_name},
    request::{ValidationError, audit_reason as validate_audit_reason},
};

#[derive(Serialize)]
struct CreateThreadBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invitable: Option<bool>,
    #[serde(rename = "type")]
    kind: ChannelType,
    name: &'a str,
}

struct CreateThreadPath {
    channel_id: Id<ChannelMarker>,
}

/// Start a thread that is not connected to a message.
///
/// To make a [`PrivateThread`], the guild must also have the
/// `PRIVATE_THREADS` feature.
///
/// [`PrivateThread`]: twilight_model::channel::ChannelType::PrivateThread
#[must_use = "requests must be configured and executed"]
pub struct CreateThread<'a> {
    body: Result<CreateThreadBody<'a>, ChannelValidationError>,
    http: &'a Client,
    path: CreateThreadPath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateThread<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        name: &'a str,
        kind: ChannelType,
    ) -> Self {
        let body = Ok(CreateThreadBody {
            auto_archive_duration: None,
            invitable: None,
            kind,
            name,
        })
        .and_then(|body| {
            validate_name(name)?;
            validate_is_thread(kind)?;

            Ok(body)
        });

        Self {
            body,
            http,
            path: CreateThreadPath { channel_id },
            reason: Ok(None),
        }
    }

    /// Set the thread's auto archive duration.
    ///
    /// Automatic archive durations are not locked behind the guild's boost
    /// level.
    pub const fn auto_archive_duration(
        mut self,
        auto_archive_duration: AutoArchiveDuration,
    ) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.auto_archive_duration = Some(auto_archive_duration);
        }

        self
    }

    /// Whether non-moderators can add other non-moderators to a thread.
    pub const fn invitable(mut self, invitable: bool) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.invitable = Some(invitable);
        }

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateThread<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for CreateThread<'_> {
    type Output = Result<Response<Channel>, Error>;

    type IntoFuture = ResponseFuture<Channel>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateThread<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;

        let mut request = Request::builder_from_route(&Route::CreateThread {
            channel_id: self.path.channel_id.get(),
        })
        .json(&body);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}
