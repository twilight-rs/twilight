use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::thread::ThreadMember,
    id::{
        Id,
        marker::{ChannelMarker, UserMarker},
    },
};
use twilight_validate::channel::{ChannelValidationError, thread_member_limit};

struct GetThreadMembersPath {
    after: Option<Id<UserMarker>>,
    channel_id: Id<ChannelMarker>,
    limit: Option<u32>,
    with_member: Option<bool>,
}

/// Returns the [`ThreadMember`]s of the thread.
///
/// [`ThreadMember`]: twilight_model::channel::thread::ThreadMember
#[must_use = "requests must be configured and executed"]
pub struct GetThreadMembers<'a> {
    http: &'a Client,
    path: Result<GetThreadMembersPath, ChannelValidationError>,
}

impl<'a> GetThreadMembers<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: Ok(GetThreadMembersPath {
                after: None,
                channel_id,
                limit: None,
                with_member: None,
            }),
        }
    }

    /// Fetch the thread members after the user ID.
    pub const fn after(mut self, after: Id<UserMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.after = Some(after);
        }

        self
    }

    /// The maximum number of thread members to return.
    ///
    /// Must be between 1 and 100. Defaults to 100.
    ///
    /// # Errors
    ///
    /// Returns a [`ChannelValidationErrorType::ThreadMemberLimitInvalid`] error type if the
    /// limit is not between 1 and 100.
    ///
    /// [`ChannelValidationErrorType::ThreadMemberLimitInvalid`]: twilight_validate::channel::ChannelValidationErrorType::ThreadMemberLimitInvalid
    pub fn limit(mut self, limit: u32) -> Self {
        self.path = self.path.and_then(|mut path| {
            thread_member_limit(limit)?;
            path.limit = Some(limit);

            Ok(path)
        });

        self
    }

    /// Include the associated guild members for each thread member.
    pub const fn with_member(mut self, with_member: bool) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.with_member = Some(with_member);
        }

        self
    }
}

impl IntoFuture for GetThreadMembers<'_> {
    type Output = Result<Response<ListBody<ThreadMember>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<ThreadMember>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetThreadMembers<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetThreadMembersPath {
            after,
            channel_id,
            limit,
            with_member,
        } = self.path.map_err(Error::validation)?;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("thread-members")
            .parameter(("after", after))
            .parameter(("limit", limit))
            .parameter(("with_member", with_member))
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Put, path))
    }
}
