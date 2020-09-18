use super::super::{
    scope::{self, Scope},
    Client, GrantType, TokenType,
};
use serde::Serialize;
use std::fmt::Write;
use twilight_model::id::ApplicationId;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct RefreshTokenExchangeRequestBody<'a> {
    /// ID of the application that was authorized.
    pub client_id: ApplicationId,
    /// Secret of the application that was authorized.
    pub client_secret: &'a str,
    /// Type of grant approval.
    pub grant_type: GrantType,
    /// Redirect URi that the user was redirected to.
    pub redirect_uri: &'a str,
    /// Refresh token provided in the access token exchange response.
    pub refresh_token: &'a str,
    /// List of scopes that the user granted.
    ///
    /// This is space-delimited.
    pub scope: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct RefreshTokenExchangeRequest<'a> {
    /// Body to send.
    pub body: RefreshTokenExchangeRequestBody<'a>,
    /// Headers to send.
    pub headers: &'static [(&'static str, &'static str)],
    /// Base of the URL.
    ///
    /// Use the [`url`] method for the full URL with query parameters.
    ///
    /// [`url`]: #method.url
    pub url_base: &'static str,
}

impl RefreshTokenExchangeRequest<'_> {
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
        buf.push_str("&redirect_uri=");
        buf.push_str(self.body.redirect_uri);
        buf.push_str("&refresh_token=");
        buf.push_str(self.body.refresh_token);
        buf.push_str("&scope=");
        buf.push_str(&urlencoding::encode(&self.body.scope));

        buf
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct RefreshTokenExchangeResponse {
    /// Access token to be used when making requests to the API on the user's
    /// behalf.
    pub access_token: String,
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
    /// Type of token provided.
    ///
    /// This will always be [`TokenType::Bearer`].
    ///
    /// [`TokenType::Bearer`]: ../enum.TokenType.html#variant.Bearer
    pub token_type: TokenType,
}

pub struct RefreshTokenExchangeBuilder<'a> {
    client: &'a Client,
    refresh_token: &'a str,
    scopes: Option<&'a [Scope]>,
}

impl<'a> RefreshTokenExchangeBuilder<'a> {
    const BASE_URL: &'static str = "https://discord.com/api/v6/oauth2/token";

    pub(crate) fn new(client: &'a Client, refresh_token: &'a str) -> Self {
        Self {
            client,
            refresh_token,
            scopes: None,
        }
    }

    pub fn build(&'a self) -> RefreshTokenExchangeRequest<'a> {
        RefreshTokenExchangeRequest {
            body: RefreshTokenExchangeRequestBody {
                client_id: self.client.client_id(),
                client_secret: self.client.client_secret(),
                grant_type: GrantType::RefreshToken,
                redirect_uri: self
                    .client
                    .redirect_uris()
                    .first()
                    .expect("redirect uri must be configured")
                    .as_ref(),
                refresh_token: self.refresh_token,
                scope: self.scopes.map(scope::join).unwrap_or_default(),
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

#[cfg(test)]
mod tests {
    use super::{Client, GrantType, RefreshTokenExchangeRequestBody, Scope};
    use twilight_model::id::ApplicationId;

    #[test]
    fn test_refresh_token_exchange_request() {
        let client = Client::new(ApplicationId(1), "a", &["https://example.com"]).unwrap();
        let refresh_token = "b";
        let mut builder = client.refresh_token_exchange(refresh_token);
        let req = builder.build();
        assert_eq!(
            req.headers,
            &[("Content-Type", "application/x-www-form-urlencoded")]
        );
        assert_eq!(req.url_base, "https://discord.com/api/v6/oauth2/token");
        assert_eq!(
            req.body,
            RefreshTokenExchangeRequestBody {
                client_id: ApplicationId(1),
                client_secret: "a",
                grant_type: GrantType::RefreshToken,
                redirect_uri: "https://example.com/",
                refresh_token,
                scope: String::new(),
            }
        );

        builder.scopes(&[Scope::Guilds, Scope::GdmJoin]);
        let req = builder.build();
        assert_eq!(
            req.body,
            RefreshTokenExchangeRequestBody {
                client_id: ApplicationId(1),
                client_secret: "a",
                grant_type: GrantType::RefreshToken,
                redirect_uri: "https://example.com/",
                refresh_token,
                scope: "guilds gdm.join".to_owned(),
            }
        );
    }
}
