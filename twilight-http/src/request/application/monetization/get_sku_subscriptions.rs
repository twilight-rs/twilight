use crate::{
    Client, Error, Response,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{ResponseFuture, marker::ListBody},
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

struct GetSKUSubscriptionsPath {
    after: Option<Id<SubscriptionMarker>>,
    before: Option<Id<SubscriptionMarker>>,
    limit: Option<u8>,
    sku_id: Id<SkuMarker>,
    user_id: Option<Id<UserMarker>>,
}

/// Returns all subscriptions containing the SKU, filtered by user.
#[must_use = "requests must be configured and executed"]
pub struct GetSKUSubscriptions<'a> {
    http: &'a Client,
    path: Result<GetSKUSubscriptionsPath, ValidationError>,
}

impl<'a> GetSKUSubscriptions<'a> {
    pub(crate) const fn new(http: &'a Client, sku_id: Id<SkuMarker>) -> Self {
        Self {
            http,
            path: Ok(GetSKUSubscriptionsPath {
                after: None,
                before: None,
                limit: None,
                sku_id,
                user_id: None,
            }),
        }
    }

    /// List subscriptions after this ID.
    pub const fn after(mut self, after: Id<SubscriptionMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.after = Some(after);
        }

        self
    }

    /// List subscriptions before this ID.
    pub const fn before(mut self, before: Id<SubscriptionMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.before = Some(before);
        }

        self
    }

    /// Number of subscriptions to return.
    ///
    /// The minimum is 1 and the maximum is 100.
    ///
    /// The Discord default is 50.
    ///
    /// # Errors
    ///
    /// Returns a [`GetSKUSubscriptionsError`] error type if the amount
    /// is less than 1 or greater than 100.
    ///
    /// [`GetSKUSubscriptionsError`]: twilight_validate::request::ValidationErrorType::GetSKUSubscriptions
    pub fn limit(mut self, limit: u8) -> Self {
        self.path = self.path.and_then(|mut path| {
            validate_get_sku_subscriptions_limit(limit)?;

            path.limit = Some(limit);

            Ok(path)
        });

        self
    }

    /// User ID for which to return subscriptions. Required except for OAuth queries.
    pub const fn user_id(mut self, user_id: Id<UserMarker>) -> Self {
        if let Ok(path) = self.path.as_mut() {
            path.user_id = Some(user_id);
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
        let GetSKUSubscriptionsPath {
            after,
            before,
            limit,
            sku_id,
            user_id,
        } = self.path.map_err(Error::validation)?;

        let path = Path::builder()
            .resource("skus")
            .id(sku_id)
            .resource("subscriptions")
            .parameter(("after", after))
            .parameter(("before", before))
            .parameter(("limit", limit))
            .parameter(("user_id", user_id))
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
