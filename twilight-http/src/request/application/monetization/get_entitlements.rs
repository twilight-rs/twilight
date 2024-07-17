use std::future::IntoFuture;

use twilight_model::{
    application::monetization::Entitlement,
    id::{
        marker::{ApplicationMarker, EntitlementMarker, GuildMarker, SkuMarker, UserMarker},
        Id,
    },
};

use crate::{
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
    Client, Error, Response,
};

use twilight_validate::request::{
    get_entitlements_limit as validate_get_entitlements_limit, ValidationError,
};

struct GetEntitlementsFields<'a> {
    after: Option<Id<EntitlementMarker>>,
    before: Option<Id<EntitlementMarker>>,
    exclude_ended: Option<bool>,
    guild_id: Option<Id<GuildMarker>>,
    limit: Option<u8>,
    sku_ids: &'a [Id<SkuMarker>],
    user_id: Option<Id<UserMarker>>,
}

/// Get all entitlements for a given app, active and expired.
#[must_use = "requests must be configured and executed"]
pub struct GetEntitlements<'a> {
    application_id: Id<ApplicationMarker>,
    fields: GetEntitlementsFields<'a>,
    http: &'a Client,
}

impl<'a> GetEntitlements<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            application_id,
            fields: GetEntitlementsFields {
                after: None,
                before: None,
                exclude_ended: None,
                guild_id: None,
                limit: None,
                sku_ids: &[],
                user_id: None,
            },
            http,
        }
    }

    /// Retrieve entitlements after this time.
    pub const fn after(mut self, after: Id<EntitlementMarker>) -> Self {
        self.fields.after = Some(after);

        self
    }

    /// Retrieve entitlements before this time.
    pub const fn before(mut self, before: Id<EntitlementMarker>) -> Self {
        self.fields.before = Some(before);

        self
    }

    /// Whether to exclude ended entitlements.
    pub const fn exclude_ended(mut self, exclude_ended: bool) -> Self {
        self.fields.exclude_ended = Some(exclude_ended);

        self
    }

    /// Guild ID to look up entitlements for.
    pub const fn guild_id(mut self, guild_id: Id<GuildMarker>) -> Self {
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
    pub fn limit(mut self, limit: u8) -> Result<Self, ValidationError> {
        validate_get_entitlements_limit(limit)?;

        self.fields.limit = Some(limit);

        Ok(self)
    }

    /// List of SKU IDs to check entitlements for.
    pub const fn sku_ids(mut self, sku_ids: &'a [Id<SkuMarker>]) -> Self {
        self.fields.sku_ids = sku_ids;

        self
    }

    /// User ID to look up entitlements for.
    pub const fn user_id(mut self, user_id: Id<UserMarker>) -> Self {
        self.fields.user_id = Some(user_id);

        self
    }
}

impl IntoFuture for GetEntitlements<'_> {
    type Output = Result<Response<ListBody<Entitlement>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Entitlement>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetEntitlements<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetEntitlements {
            after: self.fields.after.map(Id::get),
            application_id: self.application_id.get(),
            before: self.fields.before.map(Id::get),
            exclude_ended: self.fields.exclude_ended,
            guild_id: self.fields.guild_id.map(Id::get),
            limit: self.fields.limit,
            sku_ids: self.fields.sku_ids,
            user_id: self.fields.user_id.map(Id::get),
        }))
    }
}
