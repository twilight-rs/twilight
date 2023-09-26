use twilight_model::id::marker::{
    EntitlementMarker, EntitlementSkuMarker, GuildMarker, UserMarker,
};

use crate::Client;

use twilight_validate::request::{
    get_entitlements_limit as validate_get_entitlements_limit, ValidationError,
};

struct GetEntitlementsFields<'a> {
    after: Option<EntitlementMarker>,
    before: Option<EntitlementMarker>,
    exclude_ended: Option<bool>,
    guild_id: Option<GuildMarker>,
    limit: Option<u8>,
    sku_ids: Option<&'a [EntitlementSkuMarker]>,
    user_id: Option<UserMarker>,
}

/// Get all entitlements for a given app, active and expired.
#[must_use = "requests must be configured and executed"]
pub struct GetEntitlements<'a> {
    http: &'a Client,
    fields: GetEntitlementsFields<'a>,
}

impl<'a> GetEntitlements<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self {
            http,
            fields: GetEntitlementsFields {
                after: None,
                before: None,
                exclude_ended: None,
                guild_id: None,
                limit: None,
                sku_ids: None,
                user_id: None,
            },
        }
    }

    /// Retrieve entitlements after this time.
    pub const fn after(mut self, after: EntitlementMarker) -> Self {
        self.fields.after = Some(after);

        self
    }

    /// Retrieve entitlements before this time.
    pub const fn before(mut self, before: EntitlementMarker) -> Self {
        self.fields.before = Some(before);

        self
    }

    /// Whether to exclude ended entitlements.
    pub const fn exclude_ended(mut self, exclude_ended: bool) -> Self {
        self.fields.exclude_ended = Some(exclude_ended);

        self
    }

    /// Guild ID to look up entitlements for.
    pub const fn guild_id(mut self, guild_id: GuildMarker) -> Self {
        self.fields.guild_id = Some(guild_id);

        self
    }

    /// Number of entitlements to return. Set to 100 if unspecified.
    ///
    /// The minimum is 1 and the maximum is 100.
    ///
    /// # Errors
    ///
    /// Returns a [`GetEntitlementsError`] error type if the amount
    /// is less than 1 or greater than 100.
    ///
    /// [`GetEntitlementsError`]: twilight_validate::request::ValidationErrorType::GetEntitlements
    pub const fn limit(mut self, limit: u8) -> Result<Self, ValidationError> {
        if let Err(source) = validate_get_entitlements_limit(limit) {
            return Err(source);
        }

        self.fields.limit = Some(limit);

        Ok(self)
    }

    /// List of SKU IDs to check entitlements for.
    pub const fn sku_ids(mut self, sku_ids: &'a [EntitlementSkuMarker]) -> Self {
        self.fields.sku_ids = Some(sku_ids);

        self
    }

    /// User ID to look up entitlements for.
    pub const fn user_id(mut self, user_id: UserMarker) -> Self {
        self.fields.user_id = Some(user_id);

        self
    }
}
