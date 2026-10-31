use crate::{
    Client, Error, Response,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::ResponseFuture,
};
use std::future::IntoFuture;
use twilight_model::{
    application::monetization::Subscription,
    id::{
        Id,
        marker::{SkuMarker, SubscriptionMarker},
    },
};

struct GetSKUSubscriptionPath {
    sku_id: Id<SkuMarker>,
    subscription_id: Id<SubscriptionMarker>,
}

/// Returns all subscriptions containing the SKU, filtered by user.
#[must_use = "requests must be configured and executed"]
pub struct GetSKUSubscription<'a> {
    http: &'a Client,
    path: GetSKUSubscriptionPath,
}

impl<'a> GetSKUSubscription<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        sku_id: Id<SkuMarker>,
        subscription_id: Id<SubscriptionMarker>,
    ) -> Self {
        Self {
            http,
            path: GetSKUSubscriptionPath {
                sku_id,
                subscription_id,
            },
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
        let GetSKUSubscriptionPath {
            sku_id,
            subscription_id,
        } = self.path;

        let path = Path::builder()
            .resource("skus")
            .id(sku_id)
            .resource("subscriptions")
            .id(subscription_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
