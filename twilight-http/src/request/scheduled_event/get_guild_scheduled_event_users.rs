use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::scheduled_event::GuildScheduledEventUser,
    id::{
        Id,
        marker::{GuildMarker, ScheduledEventMarker, UserMarker},
    },
};
use twilight_validate::request::{
    ValidationError, scheduled_event_get_users as validate_scheduled_event_get_users,
};

pub struct GetGuildScheduledEventUsersPath {
    after: Option<Id<UserMarker>>,
    before: Option<Id<UserMarker>>,
    guild_id: Id<GuildMarker>,
    limit: Option<u16>,
    scheduled_event_id: Id<ScheduledEventMarker>,
    with_member: Option<bool>,
}

/// Get a list of users subscribed to a scheduled event.
///
/// Users are returned in ascending order by `user_id`. [`before`] and [`after`]
/// both take a user id. If both are specified, only [`before`] is respected.
/// The default [`limit`] is 100. See
/// [Discord Docs/Get Guild Scheduled Event Users].
///
/// [`after`]: GetGuildScheduledEventUsers::after
/// [`before`]: GetGuildScheduledEventUsers::before
/// [`limit`]: GetGuildScheduledEventUsers::limit
/// [Discord Docs/Get Guild Scheduled Event Users]: https://discord.com/developers/docs/resources/guild-scheduled-event#get-guild-scheduled-event-users
#[must_use = "requests must be configured and executed"]
pub struct GetGuildScheduledEventUsers<'a> {
    http: &'a Client,
    path: Result<GetGuildScheduledEventUsersPath, ValidationError>,
}

impl<'a> GetGuildScheduledEventUsers<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        scheduled_event_id: Id<ScheduledEventMarker>,
    ) -> Self {
        Self {
            http,
            path: Ok(GetGuildScheduledEventUsersPath {
                after: None,
                before: None,
                guild_id,
                limit: None,
                scheduled_event_id,
                with_member: None,
            }),
        }
    }

    /// Get users after this user ID.
    ///
    /// This is incompatible with [`before`], and has no effect if [`before`] is
    /// also set.
    ///
    /// [`before`]: Self::before
    pub const fn after(mut self, after: Id<UserMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.after = Some(after);
        }

        self
    }

    /// Get users before this user ID.
    ///
    /// This is incompatible with [`after`].
    ///
    /// [`after`]: Self::after
    pub const fn before(mut self, before: Id<UserMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.before = Some(before);
        }

        self
    }

    /// Set the limit of users to return.
    ///
    /// If not specified, the default is 100.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`ScheduledEventGetUsers`] if the limit is
    /// invalid.
    ///
    /// [`ScheduledEventGetUsers`]: twilight_validate::request::ValidationErrorType::ScheduledEventGetUsers
    pub fn limit(mut self, limit: u16) -> Self {
        self.path = self.path.and_then(|mut path| {
            validate_scheduled_event_get_users(limit)?;
            path.limit = Some(limit);

            Ok(path)
        });

        self
    }

    /// Set whether to return member objects with each user.
    pub const fn with_member(mut self, with_member: bool) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.with_member = Some(with_member);
        }

        self
    }
}

impl IntoFuture for GetGuildScheduledEventUsers<'_> {
    type Output = Result<Response<ListBody<GuildScheduledEventUser>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<GuildScheduledEventUser>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildScheduledEventUsers<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = self.path.map_err(Error::validation)?;

        let builder = Path::builder()
            .resource("guilds")
            .id(path.guild_id)
            .resource("scheduled-events")
            .id(path.scheduled_event_id)
            .resource("users")
            .parameter(("after", path.after))
            .parameter(("before", path.before))
            .parameter(("limit", path.limit));

        let path = if path.with_member.unwrap_or_default() {
            builder.parameter(("with_member", true)).build()
        } else {
            builder.build()
        };
        let path = path.map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}

#[cfg(test)]
mod tests {
    use super::GetGuildScheduledEventUsers;
    use crate::{
        client::Client,
        error::ErrorType,
        request::{Method, TryIntoRequest},
    };
    use std::error::Error;
    use twilight_model::id::Id;

    #[test]
    fn validation_error_kind() {
        let client = Client::new(String::new());

        let error = GetGuildScheduledEventUsers::new(&client, Id::new(1), Id::new(2))
            .limit(0)
            .try_into_request()
            .unwrap_err();

        assert!(matches!(error.kind(), ErrorType::Validation));
    }

    #[test]
    fn path_and_method() -> Result<(), Box<dyn Error>> {
        let client = Client::new(String::new());

        let request =
            GetGuildScheduledEventUsers::new(&client, Id::new(1), Id::new(2)).try_into_request()?;

        assert_eq!(Method::Get, request.method());
        assert_eq!("guilds/1/scheduled-events/2/users", request.path().as_ref());

        Ok(())
    }

    #[test]
    fn path_with_member() -> Result<(), Box<dyn Error>> {
        let client = Client::new(String::new());

        let request = GetGuildScheduledEventUsers::new(&client, Id::new(1), Id::new(2))
            .with_member(true)
            .try_into_request()?;

        assert_eq!(
            "guilds/1/scheduled-events/2/users?with_member=true",
            request.path().as_ref()
        );

        Ok(())
    }
}
