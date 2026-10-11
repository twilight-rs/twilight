//! Request builder and response body types for getting the job status of a
//! bulk operation on an invite's list of target users.

use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;
use twilight_model::util::Timestamp;

/// Status of a target user modification job.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Hash, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum TargetUsersJobStatusCode {
    /// The default value.
    Unspecified,
    /// The job is still being processed.
    Processing,
    /// The job has been completed successfully.
    Completed,
    /// The job has failed, see
    /// [`GetTargetUsersJobStatusResponseBody::error_message`] for more details.
    Failed,
    /// Status code unknown to the library.
    Other(u8),
}

impl From<u8> for TargetUsersJobStatusCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Unspecified,
            1 => Self::Processing,
            2 => Self::Completed,
            3 => Self::Failed,
            other => Self::Other(other),
        }
    }
}

impl From<TargetUsersJobStatusCode> for u8 {
    fn from(value: TargetUsersJobStatusCode) -> Self {
        match value {
            TargetUsersJobStatusCode::Unspecified => 0,
            TargetUsersJobStatusCode::Processing => 1,
            TargetUsersJobStatusCode::Completed => 2,
            TargetUsersJobStatusCode::Failed => 3,
            TargetUsersJobStatusCode::Other(other) => other,
        }
    }
}

/// Response body for [`GetTargetUsersJobStatus`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTargetUsersJobStatusResponseBody {
    /// When the job completed, if it has yet.
    pub completed_at: Option<Timestamp>,
    /// When the job was created.
    pub created_at: Timestamp,
    /// Provided error message.
    pub error_message: Option<String>,
    /// Number of users that have been processed.
    pub processed_users: u64,
    /// Current status of the job.
    pub status: TargetUsersJobStatusCode,
    /// Total number of users submitted to the job.
    pub total_users: u64,
}

/// Check the status of modifying an invite's target users.
///
/// Processing target users when creating or updating an invite is done
/// asynchronously. This endpoint allows you to check the status of that job.
///
/// Response is a 200 OK when there is a target user job.
///
/// Requires the caller to be the inviter, or have either the [`MANAGE_GUILD`]
/// or [`VIEW_AUDIT_LOG`] permissions.
///
/// # Examples
///
/// Get the job status of modifying an invite's target users:
///
/// ```rust,no_run
/// use std::env;
/// use twilight_http::{response::StatusCode, Client};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let invite_code = "code";
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.target_users_job_status(invite_code).await?;
///
/// let job_status = response.model().await?;
///
/// println!(
///     "job has processed {} out of {} users",
///     job_status.processed_users,
///     job_status.total_users,
/// );
/// # Ok(()) }
/// ```
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
/// [`VIEW_AUDIT_LOG`]: twilight_model::guild::Permissions::VIEW_AUDIT_LOG
#[must_use = "requests must be configured and executed"]
pub struct GetTargetUsersJobStatus<'a> {
    code: &'a str,
    http: &'a Client,
}

impl<'a> GetTargetUsersJobStatus<'a> {
    pub(crate) const fn new(http: &'a Client, code: &'a str) -> Self {
        Self { code, http }
    }
}

impl IntoFuture for GetTargetUsersJobStatus<'_> {
    type Output = Result<Response<GetTargetUsersJobStatusResponseBody>, Error>;

    type IntoFuture = ResponseFuture<GetTargetUsersJobStatusResponseBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetTargetUsersJobStatus<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetTargetUsersJobStatus {
            code: self.code,
        }))
    }
}
