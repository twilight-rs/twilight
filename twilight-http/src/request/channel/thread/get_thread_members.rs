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

struct GetThreadMembersFields {
    after: Option<Id<UserMarker>>,
    limit: Option<u32>,
    with_member: Option<bool>,
}

/// Returns the [`ThreadMember`]s of the thread.
///
/// [`ThreadMember`]: twilight_model::channel::thread::ThreadMember
#[must_use = "requests must be configured and executed"]
pub struct GetThreadMembers<'a> {
    channel_id: Id<ChannelMarker>,
    fields: Result<GetThreadMembersFields, ChannelValidationError>,
    http: &'a Client,
}

impl<'a> GetThreadMembers<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            channel_id,
            fields: Ok(GetThreadMembersFields {
                after: None,
                limit: None,
                with_member: None,
            }),
            http,
        }
    }

    /// Fetch the thread members after the user ID.
    pub fn after(mut self, after: Id<UserMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.after = Some(after);
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
        self.fields = self.fields.and_then(|mut fields| {
            thread_member_limit(limit)?;
            fields.limit = Some(limit);

            Ok(fields)
        });

        self
    }

    /// Include the associated guild members for each thread member.
    pub fn with_member(mut self, with_member: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.with_member = Some(with_member);
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
        let fields = self.fields.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetThreadMembers {
            after: fields.after.map(Id::get),
            channel_id: self.channel_id.get(),
            limit: fields.limit,
            with_member: fields.with_member,
        }))
    }
}
