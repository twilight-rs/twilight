use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::GuildPrune,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};
use twilight_validate::request::{guild_prune_days as validate_guild_prune_days, ValidationError};

struct GetGuildPruneCountFields<'a> {
    days: Option<u16>,
    include_roles: &'a [Id<RoleMarker>],
}

/// Get the counts of guild members to be pruned.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildPruneCount<'a> {
    fields: Result<GetGuildPruneCountFields<'a>, ValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetGuildPruneCount<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: Ok(GetGuildPruneCountFields {
                days: None,
                include_roles: &[],
            }),
            guild_id,
            http,
        }
    }

    /// Set the number of days that a user must be inactive before being
    /// able to be pruned.
    ///
    /// The number of days must be greater than 0, and less than or equal to 30.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`GuildPruneDays`] if the number of days is 0
    /// or more than 30.
    ///
    /// [`GuildPruneDays`]: twilight_validate::request::ValidationErrorType::GuildPruneDays
    pub fn days(mut self, days: u16) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_guild_prune_days(days)?;
            fields.days = Some(days);

            Ok(fields)
        });

        self
    }

    /// List of roles to include when calculating prune count
    pub fn include_roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.include_roles = roles;
        }

        self
    }
}

impl IntoFuture for GetGuildPruneCount<'_> {
    type Output = Result<Response<GuildPrune>, Error>;

    type IntoFuture = ResponseFuture<GuildPrune>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildPruneCount<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetGuildPruneCount {
            days: fields.days,
            guild_id: self.guild_id.get(),
            include_roles: fields.include_roles,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::GetGuildPruneCount;
    use crate::{request::TryIntoRequest, Client};
    use twilight_model::id::Id;

    #[test]
    fn days() {
        fn days_valid(days: u16) -> bool {
            let client = Client::new(String::new());

            GetGuildPruneCount::new(&client, Id::new(1))
                .days(days)
                .try_into_request()
                .is_ok()
        }

        assert!(!days_valid(0));
        assert!(days_valid(1));
        assert!(!days_valid(u16::MAX));
    }
}
