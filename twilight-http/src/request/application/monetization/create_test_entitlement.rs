use std::future::IntoFuture;

use serde::ser::{Serialize, SerializeStruct, Serializer};
use twilight_model::{
    application::monetization::Entitlement,
    id::{
        marker::{ApplicationMarker, GuildMarker, SkuMarker, UserMarker},
        Id,
    },
};

use crate::{
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
    Client, Error, Response,
};

/// Owner of a test entitlement.
pub enum CreateTestEntitlementOwner {
    Guild(Id<GuildMarker>),
    User(Id<UserMarker>),
}

impl CreateTestEntitlementOwner {
    pub const fn id(&self) -> u64 {
        match self {
            CreateTestEntitlementOwner::Guild(id) => id.get(),
            CreateTestEntitlementOwner::User(id) => id.get(),
        }
    }

    pub const fn kind(&self) -> u8 {
        match self {
            CreateTestEntitlementOwner::Guild(_) => 1,
            CreateTestEntitlementOwner::User(_) => 2,
        }
    }
}

struct CreateTestEntitlementFields {
    sku_id: Id<SkuMarker>,
    owner: CreateTestEntitlementOwner,
}

impl Serialize for CreateTestEntitlementFields {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CreateTestEntitlementFields", 2)?;
        state.serialize_field("sku_id", &self.sku_id.get())?;
        state.serialize_field("owner_id", &self.owner.id())?;
        state.serialize_field("owner_type", &self.owner.kind())?;
        state.end()
    }
}

pub struct CreateTestEntitlement<'a> {
    application_id: Id<ApplicationMarker>,
    fields: CreateTestEntitlementFields,
    http: &'a Client,
}

impl<'a> CreateTestEntitlement<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        sku_id: Id<SkuMarker>,
        owner: CreateTestEntitlementOwner,
    ) -> Self {
        Self {
            application_id,
            fields: CreateTestEntitlementFields { sku_id, owner },
            http,
        }
    }
}

impl IntoFuture for CreateTestEntitlement<'_> {
    type Output = Result<Response<Entitlement>, Error>;

    type IntoFuture = ResponseFuture<Entitlement>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateTestEntitlement<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::CreateTestEntitlement {
            application_id: self.application_id.get(),
        })
        .json(&self.fields)
        .build()
    }
}

#[cfg(test)]
mod tests {
    use serde_test::Token;
    use twilight_model::id::Id;

    use super::{CreateTestEntitlementFields, CreateTestEntitlementOwner};

    #[test]
    fn fields_serialization() {
        let value = CreateTestEntitlementFields {
            sku_id: Id::new(1),
            owner: CreateTestEntitlementOwner::Guild(Id::new(2)),
        };

        serde_test::assert_ser_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CreateTestEntitlementFields",
                    len: 2,
                },
                Token::Str("sku_id"),
                Token::U64(1),
                Token::Str("owner_id"),
                Token::U64(2),
                Token::Str("owner_type"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }
}
