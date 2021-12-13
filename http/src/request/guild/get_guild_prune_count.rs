use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    guild::GuildPrune,
    id::{GuildId, RoleId},
};
use twilight_validate::misc::{guild_prune_days as validate_guild_prune_days, ValidationError};

struct GetGuildPruneCountFields<'a> {
    days: Option<u64>,
    include_roles: &'a [RoleId],
}

/// Get the counts of guild members to be pruned.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildPruneCount<'a> {
    fields: GetGuildPruneCountFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildPruneCount<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: GetGuildPruneCountFields {
                days: None,
                include_roles: &[],
            },
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
    /// [`GuildPruneDays`]: twilight_validate::misc::ValidationErrorType::GuildPruneDays
    pub const fn days(mut self, days: u64) -> Result<Self, ValidationError> {
        if let Err(source) = validate_guild_prune_days(days) {
            return Err(source);
        }

        self.fields.days = Some(days);

        Ok(self)
    }

    /// List of roles to include when calculating prune count
    pub const fn include_roles(mut self, roles: &'a [RoleId]) -> Self {
        self.fields.include_roles = roles;

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<GuildPrune> {
        let request = Request::from_route(&Route::GetGuildPruneCount {
            days: self.fields.days,
            guild_id: self.guild_id.get(),
            include_roles: self.fields.include_roles,
        });

        self.http.request(request)
    }
}

#[cfg(test)]
mod test {
    use super::GetGuildPruneCount;
    use crate::Client;
    use twilight_model::id::GuildId;

    #[test]
    fn test_days() {
        fn days_valid(days: u64) -> bool {
            let client = Client::new("".to_owned());
            let count = GetGuildPruneCount::new(&client, GuildId::new(1).expect("non zero"));
            let days_result = count.days(days);
            days_result.is_ok()
        }

        assert!(!days_valid(0));
        assert!(days_valid(1));
        assert!(!days_valid(u64::max_value()));
    }
}
