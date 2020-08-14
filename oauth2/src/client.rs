use super::{
    access_token_exchange::AccessTokenExchangeBuilder,
    authorization_url::{AuthorizationUrlBuilder, RedirectUriInvalidError},
    refresh_token_exchange::RefreshTokenExchangeBuilder,
};
use twilight_model::id::ApplicationId;
use url::{ParseError, Url};

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateClientError<'a> {
    RedirectUriInvalid { source: ParseError, uri: &'a str },
}

#[derive(Clone, Debug)]
pub struct Client {
    client_id: ApplicationId,
    client_secret: String,
    redirect_uris: Vec<Url>,
}

impl Client {
    pub const BASE_AUTHORIZATION_URL: &'static str = "https://discord.com/api/oauth2/authorize";

    pub fn new<'a>(
        client_id: ApplicationId,
        client_secret: impl Into<String>,
        redirect_uris: &'a [&'a str],
    ) -> Result<Self, CreateClientError<'a>> {
        let iter = redirect_uris.iter();
        let mut uris = iter.size_hint().1.map_or_else(Vec::new, Vec::with_capacity);

        for item in iter {
            let uri = Url::parse(item)
                .map_err(|source| CreateClientError::RedirectUriInvalid { source, uri: item })?;

            uris.push(uri);
        }

        Ok(Self {
            client_id,
            client_secret: client_secret.into(),
            redirect_uris: uris,
        })
    }

    pub fn authorization_url<'a>(
        &'a self,
        redirect_uri: &'a str,
    ) -> Result<AuthorizationUrlBuilder<'a>, RedirectUriInvalidError> {
        AuthorizationUrlBuilder::new(self, redirect_uri)
    }

    pub fn access_token_exchange<'a>(&'a self, code: &'a str) -> AccessTokenExchangeBuilder<'a> {
        AccessTokenExchangeBuilder::new(self, code)
    }

    pub fn refresh_token_exchange<'a>(
        &'a self,
        refresh_token: &'a str,
    ) -> RefreshTokenExchangeBuilder<'a> {
        RefreshTokenExchangeBuilder::new(self, refresh_token)
    }

    /// Return an immutable reference to the configured client ID.
    pub fn client_id(&self) -> ApplicationId {
        self.client_id
    }

    /// Return an immutable reference to the configured client secret.
    pub fn client_secret(&self) -> &str {
        self.client_secret.as_ref()
    }

    /// Return an immutable reference to the configured redirect URIs.
    pub fn redirect_uris(&self) -> &[Url] {
        self.redirect_uris.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{Client, CreateClientError};
    use twilight_model::id::ApplicationId;
    use url::ParseError;

    #[test]
    fn test_client_create() {
        let client = Client::new(ApplicationId(1), "a", &["https://example.com"]).unwrap();

        assert_eq!(ApplicationId(1), client.client_id());
        assert_eq!("a", client.client_secret());
        let uris = client
            .redirect_uris()
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<_>>();
        assert_eq!(["https://example.com/"], uris.as_slice());
    }

    #[test]
    fn test_client_create_redirect_uri_invalid() {
        let actual = Client::new(ApplicationId(1), "a", &["b"]).unwrap_err();

        assert!(matches!(actual, CreateClientError::RedirectUriInvalid {
            source,
            uri,
        } if source == ParseError::RelativeUrlWithoutBase && uri == "b"));
    }
}
