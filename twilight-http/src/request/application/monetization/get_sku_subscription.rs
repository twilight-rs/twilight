use crate::{
    Client, Error, Response,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    application::monetization::Subscription,
    id::{
        Id,
        marker::{SkuMarker, SubscriptionMarker},
    },
};

/// Returns all subscriptions containing the SKU, filtered by user.
#[must_use = "requests must be configured and executed"]
pub struct GetSKUSubscription<'a> {
    http: &'a Client,
    sku_id: Id<SkuMarker>,
    subscription_id: Id<SubscriptionMarker>,
}

impl<'a> GetSKUSubscription<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        sku_id: Id<SkuMarker>,
        subscription_id: Id<SubscriptionMarker>,
    ) -> Self {
        Self {
            http,
            sku_id,
            subscription_id,
        }
    }
}

impl IntoFuture for GetSKUSubscription<'_> {
    type Output = Result<Response<Subscription>, Error>;

    type IntoFuture = ResponseFuture<Subscription>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetSKUSubscription<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetSKUSubscription {
            subscription_id: self.subscription_id.get(),
            sku_id: self.sku_id.get(),
        }))
    }
}
