use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::thread::ThreadMember,
    id::{
        marker::{ChannelMarker, UserMarker},
        Id,
    },
};
use twilight_validate::channel::{thread_member_limit, ChannelValidationError};

/// Returns the [`ThreadMember`]s of the thread.
///
/// [`ThreadMember`]: twilight_model::channel::thread::ThreadMember
#[must_use = "requests must be configured and executed"]
pub struct GetThreadMembers<'a> {
    after: Option<Id<UserMarker>>,
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
    limit: Option<u32>,
    with_member: Option<bool>,
}

impl<'a> GetThreadMembers<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            after: None,
            channel_id,
            http,
            limit: None,
            with_member: None,
        }
    }

    /// Fetch the thread members after the user ID.
    pub const fn after(mut self, after: Id<UserMarker>) -> Self {
        self.after = Some(after);

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
    pub fn limit(mut self, limit: u32) -> Result<Self, ChannelValidationError> {
        thread_member_limit(limit)?;
        self.limit = Some(limit);

        Ok(self)
    }

    /// Include the associated guild members for each thread member.
    pub const fn with_member(mut self, with_member: bool) -> Self {
        self.with_member = Some(with_member);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<ListBody<ThreadMember>> {
        self.into_future()
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
        Ok(Request::from_route(&Route::GetThreadMembers {
            after: self.after.map(Id::get),
            channel_id: self.channel_id.get(),
            limit: self.limit,
            with_member: self.with_member,
        }))
    }
}
