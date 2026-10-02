use std::future::IntoFuture;

use twilight_model::{
    guild::screening::{JoinRequestList, JoinRequestStatus},
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

/// List join requests for guild, optionally filtered by application status.
///
/// Requires the [`MANAGE_GUILD`] permission.
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
pub struct GetGuildJoinRequests<'a> {
    /// Only return requests newer than the request specified.
    after: Option<Id<JoinRequestMarker>>,
    /// Only return requests older than the request specified.
    before: Option<Id<JoinRequestMarker>>,
    /// ID of the guild.
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    /// Maximum number of requests to return.
    limit: Option<u8>,
    /// Only return requests with this status.
    status: Option<JoinRequestStatus>,
}

impl<'a> GetGuildJoinRequests<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        status: Option<JoinRequestStatus>,
        limit: Option<u8>,
        before: Option<Id<JoinRequestMarker>>,
        after: Option<Id<JoinRequestMarker>>,
    ) -> Self {
        Self {
            after,
            before,
            guild_id,
            http,
            limit,
            status,
        }
    }
}

impl IntoFuture for GetGuildJoinRequests<'_> {
    type Output = Result<Response<JoinRequestList>, Error>;

    type IntoFuture = ResponseFuture<JoinRequestList>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildJoinRequests<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let request = Request::from_route(&Route::GetGuildJoinRequests {
            guild_id: self.guild_id.get(),
            status: self.status,
            limit: self.limit,
            before: self.before.map(Id::get),
            after: self.after.map(Id::get),
        });

        Ok(request)
    }
}
