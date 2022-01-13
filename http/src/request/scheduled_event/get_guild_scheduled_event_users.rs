use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    id::{
        marker::{GuildMarker, ScheduledEventMarker, UserMarker},
        Id,
    },
    scheduled_event::GuildScheduledEventUser,
};
use twilight_validate::request::{
    scheduled_event_get_users as validate_scheduled_event_get_users, ValidationError,
};

/// Get a list of users subscribed to a scheduled event.
///
/// Users are returned in ascending order by `user_id`. [`before`] and [`after`]
/// both take a user id. If both are specified, only [`before`] is respected.
/// The default [`limit`] is 100. See [the Discord docs] for more information.
///
/// [`after`]: GetGuildScheduledEventUsers::after
/// [`before`]: GetGuildScheduledEventUsers::before
/// [`limit`]: GetGuildScheduledEventUsers::limit
/// [the Discord docs]: https://discord.com/developers/docs/resources/guild-scheduled-event#get-guild-scheduled-event-users
#[must_use = "requests must be configured and executed"]
pub struct GetGuildScheduledEventUsers<'a> {
    after: Option<Id<UserMarker>>,
    before: Option<Id<UserMarker>>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    limit: Option<u64>,
    scheduled_event_id: Id<ScheduledEventMarker>,
    with_member: Option<bool>,
}

impl<'a> GetGuildScheduledEventUsers<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        scheduled_event_id: Id<ScheduledEventMarker>,
    ) -> Self {
        Self {
            after: None,
            before: None,
            guild_id,
            http,
            limit: None,
            scheduled_event_id,
            with_member: None,
        }
    }

    /// Get users after this user ID.
    ///
    /// This is incompatible with [`before`], and has no effect if [`before`] is
    /// also set.
    ///
    /// [`before`]: Self::before
    pub const fn after(mut self, after: Id<UserMarker>) -> Self {
        self.after = Some(after);

        self
    }

    /// Get users before this user ID.
    ///
    /// This is incompatible with [`after`].
    ///
    /// [`after`]: Self::after
    pub const fn before(mut self, before: Id<UserMarker>) -> Self {
        self.before = Some(before);

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
    pub const fn limit(mut self, limit: u64) -> Result<Self, ValidationError> {
        validate_scheduled_event_get_users(limit)?;

        self.limit = Some(limit);

        Ok(self)
    }

    /// Set whether to return member objects with each user.
    pub const fn with_member(mut self, with_member: bool) -> Self {
        self.with_member = Some(with_member);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<GuildScheduledEventUser>> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildScheduledEventUsers<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildScheduledEventUsers {
            after: self.after.map(Id::get),
            before: self.before.map(Id::get),
            guild_id: self.guild_id.get(),
            limit: self.limit,
            scheduled_event_id: self.scheduled_event_id.get(),
            with_member: self.with_member.unwrap_or_default(),
        }))
    }
}
