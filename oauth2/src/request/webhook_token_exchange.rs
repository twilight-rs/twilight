//! Create requests and parse responses when exchanging an authorization code.
//!
//! This requires that the authorization scope was set to [`WebhookIncoming`].
//! Refer to [Discord's documentation] for additional information.
//!
//! [`WebhookIncoming`]: ../../enum.Scope.html#variant.WebhookIncoming
//! [Discord's documentation]: https://discord.com/developers/docs/topics/oauth2#webhooks

use super::{super::TokenType, access_token_exchange::AccessTokenExchangeRequest};
use serde::{Deserialize, Serialize};
use twilight_model::channel::Webhook;

pub type WebhookTokenExchangeRequest<'a> = AccessTokenExchangeRequest<'a>;

/// Response from exchange an authorization code when the [`WebhookIncoming`]
/// scope is specified. The request authorization URL can be built via
/// [`AuthorizationUrlBuilder::webhook`].
///
/// [`AuthorizationUrlBuilder::webhook`]: ../../authorization_url/struct.AuthorizationUrlBuilder.html#method.webhook
/// [`WebhookIncoming`]: ../../enum.Scope.html#variant.WebhookIncoming
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub struct WebhookTokenExchangeResponse {
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
    /// Webhook that the user created via authorization.
    pub webhook: Webhook,
}
