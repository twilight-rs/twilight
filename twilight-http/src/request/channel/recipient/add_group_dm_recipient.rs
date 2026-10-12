use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, UserMarker},
};

#[derive(Serialize)]
struct AddGroupDMRecipientBody<'a> {
    /// Access token of a user that has granted the application the [`GDM_JOIN`]
    /// scope.
    ///
    /// [`GDM_JOIN`]: twilight_model::oauth::scope::GDM_JOIN
    access_token: &'a str,
    /// Nickname of the user being added.
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<&'a str>,
}

/// Add a recipient to a Group DM using their access token.
///
/// The user must have granted the application an access token with the
/// [`GDM_JOIN`] scope.
///
/// [`GDM_JOIN`]: twilight_model::oauth::scope::GDM_JOIN
#[must_use = "requests must be configured and executed"]
pub struct AddGroupDMRecipient<'a> {
    body: AddGroupDMRecipientBody<'a>,
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> AddGroupDMRecipient<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
        access_token: &'a str,
    ) -> Self {
        Self {
            body: AddGroupDMRecipientBody {
                access_token,
                nick: None,
            },
            channel_id,
            http,
            user_id,
        }
    }

    /// Nickname of the user being added.
    pub const fn nick(mut self, nick: &'a str) -> Self {
        self.body.nick = Some(nick);

        self
    }
}

impl IntoFuture for AddGroupDMRecipient<'_> {
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

impl TryIntoRequest for AddGroupDMRecipient<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::AddGroupDMRecipient {
            channel_id: self.channel_id.get(),
            user_id: self.user_id.get(),
        })
        .json(&self.body)
        .build()
    }
}
