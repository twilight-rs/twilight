use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::scheduled_event::GuildScheduledEventUser,
    id::{
        marker::{GuildMarker, ScheduledEventMarker, UserMarker},
        Id,
    },
};
use twilight_validate::request::{
    scheduled_event_get_users as validate_scheduled_event_get_users, ValidationError,
};

struct GetGuildScheduledEventUsersFields {
    after: Option<Id<UserMarker>>,
    before: Option<Id<UserMarker>>,
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
    fields: Result<GetGuildScheduledEventUsersFields, ValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildScheduledEventUsers<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        scheduled_event_id: Id<ScheduledEventMarker>,
    ) -> Self {
        Self {
            fields: Ok(GetGuildScheduledEventUsersFields {
                after: None,
                before: None,
                limit: None,
                scheduled_event_id,
                with_member: None,
            }),
            guild_id,
            http,
        }
    }

    /// Get users after this user ID.
    ///
    /// This is incompatible with [`before`], and has no effect if [`before`] is
    /// also set.
    ///
    /// [`before`]: Self::before
    pub fn after(mut self, after: Id<UserMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.after = Some(after);
        }

        self
    }

    /// Get users before this user ID.
    ///
    /// This is incompatible with [`after`].
    ///
    /// [`after`]: Self::after
    pub fn before(mut self, before: Id<UserMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.before = Some(before);
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
        self.fields = self.fields.and_then(|mut fields| {
            validate_scheduled_event_get_users(limit)?;
            fields.limit = Some(limit);

            Ok(fields)
        });

        self
    }

    /// Set whether to return member objects with each user.
    pub fn with_member(mut self, with_member: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.with_member = Some(with_member);
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
        let fields = self.fields.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetGuildScheduledEventUsers {
            after: fields.after.map(Id::get),
            before: fields.before.map(Id::get),
            guild_id: self.guild_id.get(),
            limit: fields.limit,
            scheduled_event_id: fields.scheduled_event_id.get(),
            with_member: fields.with_member.unwrap_or_default(),
        }))
    }
}
