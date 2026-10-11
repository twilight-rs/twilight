//! Request builder and CSV record type for getting an invite's target users.

use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::CsvBody},
    routing::Route,
};
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::UserMarker};

/// Row in a CSV file for a 200 OK response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTargetUsersResponseBodyRow {
    /// The target user.
    pub user_id: Id<UserMarker>,
}

/// Gets the users allowed to see and accept this invite.
///
/// Response is a 200 OK status code with a CSV file of records.
///
/// Requires the caller to be the inviter, or have either the [`MANAGE_GUILD`]
/// or [`VIEW_AUDIT_LOG`] permissions.
///
/// # Examples
///
/// Get the target users of an invite, printing the returned CSV file records:
///
/// ```rust,no_run
/// use std::env;
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let invite_code = "code";
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.target_users(invite_code).await?;
/// let rows = response.rows().await?;
///
/// for row in rows {
///     println!("- User ID: {}", row.user_id);
/// }
/// # Ok(()) }
/// ```
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
/// [`VIEW_AUDIT_LOG`]: twilight_model::guild::Permissions::VIEW_AUDIT_LOG
#[must_use = "requests must be configured and executed"]
pub struct GetTargetUsers<'a> {
    code: &'a str,
    http: &'a Client,
}

impl<'a> GetTargetUsers<'a> {
    pub(crate) const fn new(http: &'a Client, code: &'a str) -> Self {
        Self { code, http }
    }
}

impl IntoFuture for GetTargetUsers<'_> {
    type Output = Result<Response<CsvBody<GetTargetUsersResponseBodyRow>>, Error>;

    type IntoFuture = ResponseFuture<CsvBody<GetTargetUsersResponseBodyRow>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetTargetUsers<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetTargetUsers {
            code: self.code,
        }))
    }
}
