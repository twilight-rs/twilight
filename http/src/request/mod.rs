pub mod application;
pub mod attachment;
pub mod channel;
pub mod guild;
pub mod scheduled_event;
pub mod sticker;
pub mod template;
pub mod user;

mod audit_reason;
mod base;
mod get_gateway;
mod get_gateway_authed;
mod get_user_application;
mod get_voice_regions;
mod multipart;
mod try_into_request;

pub use self::{
    audit_reason::AuditLogReason,
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

/// Serialize image data as a string.
///
/// Part of a backported fix for #1744. Remove after 0.11.x.
#[allow(clippy::ref_option_ref)]
fn serialize_image<S: Serializer>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&String::from_utf8_lossy(data))
}

/// Serialize optional image data as a string.
///
/// Part of a backported fix for #1744. Remove after 0.11.x.
#[allow(clippy::ref_option_ref)]
fn serialize_optional_image<S: Serializer>(
    maybe_data: &Option<&[u8]>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(data) = maybe_data {
        serializer.serialize_some(&String::from_utf8_lossy(data))
    } else {
        serializer.serialize_none()
    }
}

/// Serialize optional and image data as a string.
///
/// Part of a backported fix for #1744. Remove after 0.11.x.
#[allow(clippy::ref_option_ref)]
fn serialize_optional_nullable_image<S: Serializer>(
    maybe_data: &Option<Nullable<&[u8]>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(data) = maybe_data.as_ref().and_then(|field| field.0) {
        serializer.serialize_some(&String::from_utf8_lossy(data))
    } else {
        serializer.serialize_none()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Serializer;
    use std::io::Cursor;

    use crate::request::Nullable;

    #[test]
    fn serialize_image() {
        let mut buf = Cursor::new(Vec::new());
        let mut serializer = Serializer::new(&mut buf);
        super::serialize_image(b"test", &mut serializer).unwrap();
        assert_eq!(br#""test""#, buf.into_inner().as_slice());
    }

    #[test]
    fn serialize_optional_image_some() {
        let mut buf = Cursor::new(Vec::new());
        let mut serializer = Serializer::new(&mut buf);
        super::serialize_optional_image(&Some(b"test"), &mut serializer).unwrap();
        assert_eq!(br#""test""#, buf.into_inner().as_slice());
    }

    #[test]
    fn serialize_optional_image_none() {
        let mut buf = Cursor::new(Vec::new());
        let mut serializer = Serializer::new(&mut buf);
        super::serialize_optional_image(&None, &mut serializer).unwrap();
        assert_eq!(b"null", buf.into_inner().as_slice());
    }

    #[test]
    fn serialize_optional_nullable_image_none() {
        let mut buf = Cursor::new(Vec::new());
        let mut serializer = Serializer::new(&mut buf);
        super::serialize_optional_nullable_image(&None, &mut serializer).unwrap();
        assert_eq!(b"null", buf.into_inner().as_slice());
    }

    #[test]
    fn serialize_optional_nullable_image_some_null() {
        let mut buf = Cursor::new(Vec::new());
        let mut serializer = Serializer::new(&mut buf);
        super::serialize_optional_nullable_image(&Some(Nullable(None)), &mut serializer).unwrap();
        assert_eq!(b"null", buf.into_inner().as_slice());
    }

    #[test]
    fn serialize_optional_nullable_image_some_value() {
        let mut buf = Cursor::new(Vec::new());
        let mut serializer = Serializer::new(&mut buf);
        super::serialize_optional_nullable_image(&Some(Nullable(Some(b"test"))), &mut serializer)
            .unwrap();
        assert_eq!(br#""test""#, buf.into_inner().as_slice());
    }
}
