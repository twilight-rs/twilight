use crate::{
    Client, Error, Response,
    request::{Request, TryIntoRequest},
    response::{ResponseFuture, marker::ListBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    application::monetization::Subscription,
    id::{
        Id,
        marker::{SkuMarker, SubscriptionMarker, UserMarker},
    },
};
use twilight_validate::request::{
    ValidationError, get_sku_subscriptions_limit as validate_get_sku_subscriptions_limit,
};

struct GetSKUSubscriptionsFields {
    after: Option<Id<SubscriptionMarker>>,
    before: Option<Id<SubscriptionMarker>>,
    limit: Option<u8>,
    user_id: Option<Id<UserMarker>>,
}

/// Returns all subscriptions containing the SKU, filtered by user.
#[must_use = "requests must be configured and executed"]
pub struct GetSKUSubscriptions<'a> {
    fields: Result<GetSKUSubscriptionsFields, ValidationError>,
    http: &'a Client,
    sku_id: Id<SkuMarker>,
}

impl<'a> GetSKUSubscriptions<'a> {
    pub(crate) const fn new(http: &'a Client, sku_id: Id<SkuMarker>) -> Self {
        Self {
            fields: Ok(GetSKUSubscriptionsFields {
                after: None,
                before: None,
                limit: None,
                user_id: None,
            }),
            http,
            sku_id,
        }
    }

    /// List subscriptions after this ID.
    pub const fn after(mut self, after: Id<SubscriptionMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.after = Some(after);
        }

        self
    }

    /// List subscriptions before this ID.
    pub const fn before(mut self, before: Id<SubscriptionMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.before = Some(before);
        }

        self
    }

    /// Number of subscriptions to return.
    ///
    /// The minimum is 1 and the maximum is 100.
    ///
    /// # Errors
    ///
    /// Returns a [`GetSKUSubscriptionsError`] error type if the amount
    /// is less than 1 or greater than 100.
    ///
    /// [`GetSKUSubscriptionsError`]: twilight_validate::request::ValidationErrorType::GetSKUSubscriptions
    pub fn limit(mut self, limit: u8) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_get_sku_subscriptions_limit(limit)?;

            fields.limit = Some(limit);

            Ok(fields)
        });

        self
    }

    /// User ID for which to return subscriptions. Required except for OAuth queries.
    pub const fn user_id(mut self, user_id: Id<UserMarker>) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.user_id = Some(user_id);
        }

        self
    }
}

impl IntoFuture for GetSKUSubscriptions<'_> {
    type Output = Result<Response<ListBody<Subscription>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Subscription>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetSKUSubscriptions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::GetSKUSubscriptions {
            after: fields.after.map(Id::get),
            before: fields.before.map(Id::get),
            limit: fields.limit.map(u64::from),
            sku_id: self.sku_id.get(),
            user_id: fields.user_id.map(Id::get),
        }))
    }
}
