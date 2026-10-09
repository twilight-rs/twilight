use std::future::IntoFuture;

use serde::Serialize;
use twilight_model::{
    guild::screening::{JoinRequest, JoinRequestStatus},
    id::{
        Id,
        marker::{GuildMarker, JoinRequestMarker},
    },
};

use crate::{
    Client, Error, Response,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};

/// Approve or reject guild join request.
///
/// Requires the [`KICK_MEMBERS`] permission.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("token".to_owned());
///
/// let guild_id = Id::new(101);
/// let request_id = Id::new(102);
/// let new_status = JoinRequestStatus::Approved;
/// let application = client.update_guild_join_request(guild_id, request_id, new_status).await?.model().await?;
///
/// if new_status == application.application_status {
///     println!("User approved");
/// } else {
///     println!("Failed to approve user");
/// }
/// # Ok(()) }
/// ```
///
/// [`KICK_MEMBERS`]: twilight_model::guild::Permissions::KICK_MEMBERS
#[must_use = "application_status must be checked. Approving an already denied join request (and vice versa) can yield a success response, despite no change being applied."]
pub struct UpdateGuildJoinRequest<'a> {
    /// ID of the guild.
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    /// ID of the join request.
    request_id: Id<JoinRequestMarker>,
    // The new request status.
    new_status: JoinRequestStatus,
}

impl<'a> UpdateGuildJoinRequest<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        request_id: Id<JoinRequestMarker>,
        new_status: JoinRequestStatus,
    ) -> Self {
        Self {
            guild_id,
            http,
            request_id,
            new_status,
        }
    }
}

impl IntoFuture for UpdateGuildJoinRequest<'_> {
    type Output = Result<Response<JoinRequest>, Error>;

    type IntoFuture = ResponseFuture<JoinRequest>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

#[derive(Serialize)]
struct StatusPatch {
    action: JoinRequestStatus,
}

impl TryIntoRequest for UpdateGuildJoinRequest<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::UpdateGuildJoinRequest {
            guild_id: self.guild_id.get(),
            request_id: self.request_id.get(),
        })
        .json(&StatusPatch {
            action: self.new_status,
        })
        .build()
    }
}

#[cfg(test)]
mod tests {
    use super::{JoinRequestStatus, StatusPatch, UpdateGuildJoinRequest};
    use crate::{
        Client,
        request::{Request, TryIntoRequest},
        routing::Route,
    };
    use std::error::Error;
    use twilight_model::id::Id;

    #[test]
    fn request() -> Result<(), Box<dyn Error>> {
        let client = Client::new("token".to_string());
        let guild_id = Id::new(101);
        let request_id = Id::new(102);
        let new_status = JoinRequestStatus::Approved;

        let actual = UpdateGuildJoinRequest::new(&client, guild_id, request_id, new_status)
            .try_into_request()?;

        let expected = Request::builder(&Route::UpdateGuildJoinRequest {
            guild_id: guild_id.into(),
            request_id: request_id.into(),
        })
        .json(&StatusPatch { action: new_status })
        .build()?;

        assert_eq!(expected.body(), actual.body());
        assert_eq!(expected.path(), actual.path());

        Ok(())
    }
}
