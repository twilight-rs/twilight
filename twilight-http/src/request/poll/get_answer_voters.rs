use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;
use twilight_model::{
    id::{
        Id,
        marker::{ChannelMarker, MessageMarker, UserMarker},
    },
    user::User,
};

#[derive(Serialize)]
pub struct GetAnswerVotersPath {
    after: Option<Id<UserMarker>>,
    answer_id: u8,
    channel_id: Id<ChannelMarker>,
    limit: Option<u8>,
    message_id: Id<MessageMarker>,
}

/// Gets the data for a poll answer.
#[must_use = "requests must be configured and executed"]
pub struct GetAnswerVoters<'a> {
    http: &'a Client,
    path: GetAnswerVotersPath,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetAnswerVotersResponse {
    pub users: Vec<User>,
}

impl<'a> GetAnswerVoters<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
        answer_id: u8,
    ) -> Self {
        Self {
            http,
            path: GetAnswerVotersPath {
                after: None,
                answer_id,
                channel_id,
                limit: None,
                message_id,
            },
        }
    }

    /// Set the user ID to get voters after.
    pub const fn after(mut self, after: Id<UserMarker>) -> Self {
        self.path.after.replace(after);

        self
    }

    /// Set the limit of voters to get.
    ///
    /// The minimum is 1 and the maximum is 100.
    pub const fn limit(mut self, limit: u8) -> Self {
        self.path.limit.replace(limit);

        self
    }
}

impl IntoFuture for GetAnswerVoters<'_> {
    type Output = Result<Response<GetAnswerVotersResponse>, Error>;
    type IntoFuture = ResponseFuture<GetAnswerVotersResponse>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetAnswerVoters<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("channels")
            .id(self.path.channel_id)
            .resource("polls")
            .id(self.path.message_id)
            .resource("answers")
            .id(self.path.answer_id)
            .parameter(("after", self.path.after))
            .parameter(("limit", self.path.limit))
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
