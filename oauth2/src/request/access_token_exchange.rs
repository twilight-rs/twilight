use super::super::{
    scope::{self, Scope},
    Client, GrantType, TokenType,
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use twilight_model::{channel::Webhook, id::ApplicationId};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AccessTokenExchangeRequestBody<'a> {
    /// ID of the application that was authorized.
    pub client_id: ApplicationId,
    /// Secret of the application that was authorized.
    pub client_secret: &'a str,
    /// Access token used to perform requests on behalf of the authorized user.
    pub code: &'a str,
    /// Type of grant approval.
    pub grant_type: GrantType,
    /// Redirect URi that the user was redirected to.
    pub redirect_uri: &'a str,
    /// List of scopes that the user granted.
    ///
    /// This is space-delimited.
    pub scope: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AccessTokenExchangeRequest<'a> {
    /// Body to send.
    pub body: AccessTokenExchangeRequestBody<'a>,
    /// Headers to send.
    pub headers: &'static [(&'static str, &'static str)],
    /// Base of the URL.
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AccessTokenExchangeResponse {
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
    /// Information about the webhook that was created by user authorization.
    ///
    /// This will be present with information about the newly created webhook if
    /// the [`WebhookIncoming`] scope was selected and approved.
    ///
    /// [`WebhookIncoming`]: ../../enum.Scope.html#variant.WebhookIncoming
    pub webhook: Option<Webhook>,
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

    /// Set the scopes for the access token exchange request.
    ///
    /// This must be the same scopes you requested in the authorization URL.
    ///
    /// Read about Discord's [scope documentation].
    ///
    /// [RFC 6749 § 3.3] on access token scopes.
    ///
    /// [`Bot`]: enum.Scope.html#variant.Bot
    /// [RFC 6749 § 3.3]: https://tools.ietf.org/html/rfc6749#section-3.3
    /// [scope documentation]: https://discord.com/developers/docs/topics/oauth2#shared-resources-oauth2-scopes
    pub fn scopes(&mut self, scopes: &'a [Scope]) -> &mut Self {
        self.scopes.replace(scopes);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::AccessTokenExchangeResponse;
    use crate::TokenType;
    use serde_test::Token;
    use twilight_model::{
        channel::{Webhook, WebhookType},
        id::{ChannelId, GuildId, WebhookId},
    };

    #[test]
    fn test_response_webhook() {
        let actual = AccessTokenExchangeResponse {
            access_token: "a".to_owned(),
            expires_in: 604_800,
            token_type: TokenType::Bearer,
            refresh_token: "b".to_owned(),
            scope: "webhook.incoming".to_owned(),
            webhook: Some(Webhook {
                avatar: None,
                channel_id: ChannelId(1),
                guild_id: Some(GuildId(2)),
                id: WebhookId(3),
                kind: WebhookType::Incoming,
                name: Some("test".to_owned()),
                token: Some("token".to_owned()),
                user: None,
            }),
        };

        serde_test::assert_tokens(
            &actual,
            &[
                Token::Struct {
                    name: "AccessTokenExchangeResponse",
                    len: 6,
                },
                Token::Str("access_token"),
                Token::Str("a"),
                Token::Str("expires_in"),
                Token::U64(604_800),
                Token::Str("refresh_token"),
                Token::Str("b"),
                Token::Str("scope"),
                Token::Str("webhook.incoming"),
                Token::Str("token_type"),
                Token::UnitVariant {
                    name: "TokenType",
                    variant: "Bearer",
                },
                Token::Str("webhook"),
                Token::Some,
                Token::Struct {
                    name: "Webhook",
                    len: 8,
                },
                Token::Str("avatar"),
                Token::None,
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("1"),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("2"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "WebhookId" },
                Token::Str("3"),
                Token::Str("type"),
                Token::U8(1),
                Token::Str("name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("token"),
                Token::Some,
                Token::Str("token"),
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
