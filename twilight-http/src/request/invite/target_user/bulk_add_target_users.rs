use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::UserMarker};
use twilight_validate::invite_target_users::{
    InviteTargetUsersValidationError,
    bulk_add_target_users_ids as validate_bulk_add_target_users_ids,
};

#[derive(Serialize)]
struct BulkAddTargetUsersBody<'a> {
    user_ids: &'a [Id<UserMarker>],
}

/// Add multiple target users to an existing invite.
///
/// Response is a 204 No Content when successful. The status of the bulk
/// operation may be checked via the [`GetTargetUsersJobStatus`] request.
///
/// Requires the caller to be the inviter or have the [`MANAGE_GUILD`]
/// permission.
///
/// # Examples
///
/// Add users to an invite's list of target users in bulk:
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
/// let response = client.bulk_add_target_users(invite_code, &user_ids).await?;
///
/// assert!(response.status().is_success());
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns an [`Error`] of type [`ErrorType::Validation`] with an underlying
/// [`InviteTargetUsersValidationError`] of type
/// [`BulkAddTargetUsersIdsLimit`] if the
/// number of user IDs is invalid.
///
/// [`BulkAddTargetUsersIdsLimit`]: twilight_validate::invite_target_users::InviteTargetUsersValidationErrorType::BulkAddTargetUsersIdsLimit
/// [`ErrorType::Validation`]: crate::error::ErrorType::Validation
/// [`GetTargetUsersJobStatus`]: super::get_target_users_job_status::GetTargetUsersJobStatus
/// [`InviteTargetUsersValidationError`]: twilight_validate::invite_target_users::InviteTargetUsersValidationError
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct BulkAddTargetUsers<'a> {
    body: Result<BulkAddTargetUsersBody<'a>, InviteTargetUsersValidationError>,
    code: &'a str,
    http: &'a Client,
}

impl<'a> BulkAddTargetUsers<'a> {
    pub(crate) fn new(http: &'a Client, code: &'a str, user_ids: &'a [Id<UserMarker>]) -> Self {
        let body = Ok(BulkAddTargetUsersBody { user_ids }).and_then(|fields| {
            validate_bulk_add_target_users_ids(user_ids.len())?;

            Ok(fields)
        });

        Self { body, code, http }
    }
}

impl IntoFuture for BulkAddTargetUsers<'_> {
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

impl TryIntoRequest for BulkAddTargetUsers<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;

        Request::builder(&Route::BulkAddTargetUsers { code: self.code })
            .json(&body)
            .build()
    }
}
