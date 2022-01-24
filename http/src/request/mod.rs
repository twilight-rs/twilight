pub mod application;
pub mod channel;
pub mod guild;
pub mod scheduled_event;
pub mod sticker;
pub mod template;
pub mod user;

mod attachment;
mod audit_reason;
mod base;
mod get_gateway;
mod get_gateway_authed;
mod get_user_application;
mod get_voice_regions;
mod multipart;
mod try_into_request;

pub use self::{
    attachment::AttachmentFile,
    audit_reason::{AuditLogReason, AuditLogReasonError},
    base::{Request, RequestBuilder},
    get_gateway::GetGateway,
    get_gateway_authed::GetGatewayAuthed,
    get_user_application::GetUserApplicationInfo,
    get_voice_regions::GetVoiceRegions,
    multipart::Form,
    try_into_request::TryIntoRequest,
};
pub use twilight_http_ratelimiting::request::Method;

use crate::error::{Error, ErrorType};
use hyper::header::{HeaderName, HeaderValue};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde::{Serialize, Serializer};
use std::iter;

/// Field that either serializes to null or a value.
///
/// This is particularly useful when combined with an `Option` by allowing three
/// states via `Option<NullableField<T>>`: undefined, null, and T.
///
/// When the request field is `None` a field can skip serialization, while if a
/// `NullableField` is provided with `None` within it then it will serialize as
/// null. This mechanism is primarily used in patch requests.
struct NullableField<T>(Option<T>);

impl<T: Serialize> Serialize for NullableField<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0.as_ref() {
            Some(inner) => serializer.serialize_some(inner),
            None => serializer.serialize_none(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct PartialAttachment<'a> {
    pub description: Option<&'a str>,
    pub filename: &'a str,
    pub id: u64,
}

pub(crate) fn audit_header(
    reason: &str,
) -> Result<impl Iterator<Item = (HeaderName, HeaderValue)>, Error> {
    let header_name = HeaderName::from_static("x-audit-log-reason");
    let encoded_reason = utf8_percent_encode(reason, NON_ALPHANUMERIC).to_string();
    let header_value = HeaderValue::from_str(&encoded_reason).map_err(|e| Error {
        kind: ErrorType::CreatingHeader {
            name: encoded_reason.clone(),
        },
        source: Some(Box::new(e)),
    })?;

    Ok(iter::once((header_name, header_value)))
}

const fn slice_is_empty<T>(slice: &[T]) -> bool {
    slice.is_empty()
}
