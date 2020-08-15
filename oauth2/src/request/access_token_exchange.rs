use super::super::{
    scope::{self, Scope},
    Client, GrantType, TokenType,
};
use serde::Serialize;
use std::fmt::Write;
use twilight_model::id::ApplicationId;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AccessTokenExchangeRequestBody<'a> {
    pub client_id: ApplicationId,
    pub client_secret: &'a str,
    pub code: &'a str,
    pub grant_type: GrantType,
    pub redirect_uri: &'a str,
    pub scope: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AccessTokenExchangeRequest<'a> {
    pub body: AccessTokenExchangeRequestBody<'a>,
    pub headers: &'static [(&'static str, &'static str)],
    pub url_base: &'static str,
}

impl AccessTokenExchangeRequest<'_> {
    /// Retrieve a URL with the body urlencoded as query parameters.
    ///
    /// This URL can be used to make a POST request with the specified
    /// [`headers`].
    ///
    /// [`headers`]: #structfield.url_base
    pub fn url(&self) -> String {
        let mut buf = self.url_base.to_owned();
        buf.push_str("?client_id=");
        let _ = write!(buf, "{}", self.body.client_id.0);
        buf.push_str("&client_secret=");
        buf.push_str(self.body.client_secret);
        buf.push_str("&code=");
        buf.push_str(self.body.code);
        buf.push_str("&redirect_uri=");
        buf.push_str(self.body.redirect_uri);
        buf.push_str("&scope=");
        buf.push_str(&urlencoding::encode(&self.body.scope));

        buf
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AccessTokenExchangeResponse {
    /// Access token to be used when making requests to the API on the user's
    /// behalf.
    pub access_token: String,
    /// Type of token provided.
    ///
    /// This will always be [`TokenType::Bearer`].
    ///
    /// [`TokenType::Bearer`]: ../enum.TokenType.html#variant.Bearer
    pub token_type: TokenType,
    /// Number of seconds from issuing that the access token is valid.
    ///
    /// After this duration, the refresh token must be exchanged for another
    /// access token and refresh token pair.
    pub expires_in: u64,
    /// Refresh token to use to exchange for another access token and refresh
    /// token pair.
    pub refresh_token: String,
    /// Space-delimited list of scopes that the token has had approved.
    pub scope: String,
}

pub struct AccessTokenExchangeBuilder<'a> {
    client: &'a Client,
    code: &'a str,
    scopes: Option<&'a [Scope]>,
}

impl<'a> AccessTokenExchangeBuilder<'a> {
    const BASE_URL: &'static str = "https://discord.com/api/v6/oauth2/token";

    pub(crate) fn new(client: &'a Client, code: &'a str) -> Self {
        Self {
            client,
            code,
            scopes: None,
        }
    }

    pub fn build(&'a self) -> AccessTokenExchangeRequest<'a> {
        let scope = self.scopes.map(scope::join).unwrap_or_default();

        AccessTokenExchangeRequest {
            body: AccessTokenExchangeRequestBody {
                client_id: self.client.client_id(),
                client_secret: self.client.client_secret(),
                code: self.code,
                grant_type: GrantType::AuthorizationCode,
                redirect_uri: self
                    .client
                    .redirect_uris()
                    .first()
                    .expect("redirect uri must be configured")
                    .as_ref(),
                scope,
            },
            headers: &[("Content-Type", "application/x-www-form-urlencoded")],
            url_base: Self::BASE_URL,
        }
    }

    pub fn scopes(&mut self, scopes: &'a [Scope]) -> &mut Self {
        self.scopes.replace(scopes);

        self
    }
}
