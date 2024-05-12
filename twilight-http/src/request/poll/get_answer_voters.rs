use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;
use twilight_model::{
    id::{
        marker::{ChannelMarker, MessageMarker, UserMarker},
        Id,
    },
    user::User,
};

#[derive(Serialize)]
struct GetAnswerVotersFields {
    after: Option<Id<UserMarker>>,
    answer_id: u8,
    channel_id: Id<ChannelMarker>,
    limit: Option<u8>,
    message_id: Id<MessageMarker>,
}

/// Gets the data for a poll answer.
#[must_use = "requests must be configured and executed"]
pub struct GetAnswerVoters<'a> {
    fields: GetAnswerVotersFields,
    http: &'a Client,
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
            fields: GetAnswerVotersFields {
                after: None,
                answer_id,
                channel_id,
                limit: None,
                message_id,
            },
            http,
        }
    }

    /// Set the user ID to get voters after.
    pub fn after(mut self, after: Id<UserMarker>) -> Self {
        self.fields.after.replace(after);

        self
    }

    /// Set the limit of voters to get.
    ///
    /// The minimum is 1 and the maximum is 100.
    pub fn limit(mut self, limit: u8) -> Self {
        self.fields.limit.replace(limit);

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
        Ok(Request::from_route(&Route::GetAnswerVoters {
            after: self.fields.after.map(Id::get),
            answer_id: self.fields.answer_id,
            channel_id: self.fields.channel_id.get(),
            limit: self.fields.limit,
            message_id: self.fields.message_id.get(),
        }))
    }
}
