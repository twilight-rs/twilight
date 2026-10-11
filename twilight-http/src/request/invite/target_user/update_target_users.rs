//! Request builder and response body for updating an invite's target users.

use crate::{
    client::Client,
    error::Error,
    request::{Form, Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
    routing::Route,
};
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::UserMarker};

/// Response body when the response status code is 400 Bad Request.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateTargetUsersBadRequestResponseBody {
    /// Reasons for the response being a Bad Request.
    ///
    /// This is documented by Discord as having entries such as:
    ///
    /// - "Line 5: invalid user ID - must be a valid Discord snowflake"
    /// - "Line 6: invalid user ID format"
    pub target_users_file: Vec<String>,
}

struct UpdateTargetUsersForm<'a> {
    target_users: &'a [Id<UserMarker>],
}

/// Update the users allowed to see and accept this invite.
///
/// The status of the bulk operation may be checked via the
/// [`GetTargetUsersJobStatus`] request.
///
/// Response is a 400 Bad Request if the list of users contains invalid user
/// IDs.
///
/// Requires the caller to be the inviter or have the [`MANAGE_GUILD`]
/// permission.
///
/// # Examples
///
/// Replace and update an invite's list of target users:
///
/// ```rust,no_run
/// use std::env;
/// use twilight_http::Client;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use twilight_model::id::{marker::UserMarker, Id};
/// # fn get_users_to_invite() -> Vec<Id<UserMarker>> { Vec::from([Id::new(1)]) }
/// # let invite_code = "code";
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let user_ids = get_users_to_invite();
/// let response = client.update_target_users(invite_code, &user_ids).await?;
///
/// assert!(response.status().is_success());
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns an [`Error`] of type [`ErrorType::WritingCsv`] if the request's CSV
/// file couldn't be formatted.
///
/// [`ErrorType::WritingCsv`]: crate::error::ErrorType::WritingCsv
/// [`GetTargetUsersJobStatus`]: super::get_target_users_job_status::GetTargetUsersJobStatus
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct UpdateTargetUsers<'a> {
    code: &'a str,
    form: UpdateTargetUsersForm<'a>,
    http: &'a Client,
}

impl<'a> UpdateTargetUsers<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        code: &'a str,
        target_users: &'a [Id<UserMarker>],
    ) -> Self {
        Self {
            code,
            form: UpdateTargetUsersForm { target_users },
            http,
        }
    }
}

impl IntoFuture for UpdateTargetUsers<'_> {
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

impl TryIntoRequest for UpdateTargetUsers<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let csv = super::file(self.form.target_users)?;

        let form = Form::new().file_part(b"target_users_file", b"file.csv", csv.as_slice());

        Request::builder(&Route::UpdateTargetUsers { code: self.code })
            .form(form)
            .build()
    }
}
