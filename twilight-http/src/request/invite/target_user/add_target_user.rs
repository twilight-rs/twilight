use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::UserMarker};

/// Add a user to an invite's list of target users.
///
/// Response is a 204 No Content when successful.
///
/// Requires the caller to be the inviter or have the [`MANAGE_GUILD`]
/// permission.
///
/// # Examples
///
/// Add a user to an invite's list of target users:
///
/// ```rust,no_run
/// use std::env;
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use twilight_model::id::{marker::UserMarker, Id};
/// # let invite_code = "code";
/// # let user_id = Id::<UserMarker>::new(1);
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.add_target_user(invite_code, user_id).await?;
///
/// assert!(response.status().is_success());
/// # Ok(()) }
/// ```
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct AddTargetUser<'a> {
    code: &'a str,
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> AddTargetUser<'a> {
    pub(crate) const fn new(http: &'a Client, code: &'a str, user_id: Id<UserMarker>) -> Self {
        Self {
            code,
            http,
            user_id,
        }
    }
}

impl IntoFuture for AddTargetUser<'_> {
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

impl TryIntoRequest for AddTargetUser<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::AddTargetUser {
            code: self.code,
            user_id: self.user_id.get(),
        }))
    }
}
