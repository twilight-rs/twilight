//! Typed request builders, [multipart form support], a [manual request builder]
//! for low-level request construction, and [audit log reason] support.
//!
//! # Request Builders
//!
//! Requests are created in the form of builders. These can be `await`ed to
//! receive a [`Response`]. Every route of Discord's API has its own builder:
//! creating a message is performed via the [`CreateMessage`] builder; updating
//! a guild is done via [`UpdateGuild`]; and so on. All typed request builders
//! are instantiated via the primary [`Client`]. When the library doesn't yet
//! support a new feature or fine-grained support is required, requests can be
//! manually built via [`RequestBuilder`].
//!
//! # Audit Log Reasons
//!
//! Audit log reasons can be added to supported requests via the
//! [`AuditLogReason`] trait:
//!
//! ```no_run
//! # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # use twilight_model::id::Id;
//! #
//! # let guild_id = Id::new(1);
//! # let user_id = Id::new(2);
//! use twilight_http::{client::Client, request::AuditLogReason};
//!
//! let client = Client::new(std::env::var("DISCORD_TOKEN")?);
//! client
//!     .delete_ban(guild_id, user_id)
//!     .reason("ban expired")
//!     .await?;
//! # Ok(()) }
//! ```
//!
//! [`Client`]: crate::client::Client
//! [`CreateMessage`]: channel::message::CreateMessage
//! [`Response`]: crate::Response
//! [`UpdateGuild`]: guild::UpdateGuild
//! [audit log reason]: AuditLogReason
//! [manual request builder]: RequestBuilder
//! [multipart form support]: Form

pub mod application;
pub mod attachment;
pub mod channel;
pub mod guild;
pub mod poll;
pub mod scheduled_event;
pub mod sticker;
pub mod template;
pub mod user;

mod audit_reason;
mod base;
mod get_current_authorization_information;
mod get_gateway;
mod get_gateway_authed;
mod get_user_application;
mod get_voice_regions;
mod multipart;
mod try_into_request;
mod update_user_application;

pub use self::{
    audit_reason::AuditLogReason,
    base::{Request, RequestBuilder},
    get_current_authorization_information::GetCurrentAuthorizationInformation,
    get_gateway::GetGateway,
    get_gateway_authed::GetGatewayAuthed,
    get_user_application::GetUserApplicationInfo,
    get_voice_regions::GetVoiceRegions,
    multipart::Form,
    try_into_request::TryIntoRequest,
    update_user_application::UpdateCurrentUserApplication,
};
pub use twilight_http_ratelimiting::request::Method;

use crate::error::{Error, ErrorType};
use http::header::{HeaderName, HeaderValue};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::Serialize;
use std::iter;

/// Name of the audit log reason header.
const REASON_HEADER_NAME: &str = "x-audit-log-reason";

/// Type that either serializes to null or a value.
///
/// This is particularly useful when combined with an `Option` by allowing three
/// states via `Option<Nullable<T>>`: undefined, null, and T.
///
/// When the request value is `None` it can skip serialization, while if
/// `Nullable` is provided with `None` within it then it will serialize as
/// null. This mechanism is primarily used in patch requests.
#[derive(Serialize)]
struct Nullable<T>(Option<T>);

fn audit_header(reason: &str) -> Result<impl Iterator<Item = (HeaderName, HeaderValue)>, Error> {
    let header_name = HeaderName::from_static(REASON_HEADER_NAME);
    let encoded_reason = utf8_percent_encode(reason, NON_ALPHANUMERIC).to_string();
    let header_value = HeaderValue::from_str(&encoded_reason).map_err(|e| Error {
        kind: ErrorType::CreatingHeader {
            name: encoded_reason,
        },
        source: Some(Box::new(e)),
    })?;

    Ok(iter::once((header_name, header_value)))
}
