use super::error::{RatelimitError, RatelimitErrorType, RatelimitResult};
use hyper::header::{HeaderMap, HeaderValue};
use std::convert::TryFrom;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RatelimitHeaders {
    GlobalLimited {
        reset_after: u64,
    },
    None,
    Present {
        bucket: Option<String>,
        global: bool,
        limit: u64,
        remaining: u64,
        // when the bucket resets in unix ms
        reset: u64,
        // how long until it resets in ms
        reset_after: u64,
    },
}

impl RatelimitHeaders {
    pub fn global(&self) -> bool {
        match self {
            Self::GlobalLimited { .. } => true,
            Self::None => false,
            Self::Present { global, .. } => *global,
        }
    }
}

impl TryFrom<&'_ HeaderMap<HeaderValue>> for RatelimitHeaders {
    type Error = RatelimitError;

    fn try_from(map: &'_ HeaderMap<HeaderValue>) -> RatelimitResult<Self> {
        match parse_map(map) {
            Ok(v) => Ok(v),
            Err(why) => {
                // Now, there's a couple pairs of reasons we could have an error
                // here.
                //
                // The first set of reasons is:
                //
                // - Some headers are present, but not all;
                // - A required header is present, but it's just not very
                //   utf8y; or
                // - A required header is present, but it doesn't parse to the
                //   necessary type.
                //
                // In these cases, it's a legitimate error with the headers and
                // we should disregard it.
                //
                // The second set is:
                //
                // - The route isn't ratelimited (at least, not locally).
                //
                // This means that none of the headers are present. If that's
                // the case, then it's not limited (except for the global, of
                // course).

                let headers = &[
                    "x-ratelimit-bucket",
                    "x-ratelimit-limit",
                    "x-ratelimit-remaining",
                    "x-ratelimit-reset",
                ];

                if headers.iter().any(|k| map.contains_key(*k)) {
                    Err(why)
                } else if map.contains_key("x-ratelimit-global") {
                    Ok(Self::GlobalLimited {
                        reset_after: header_int(map, "x-ratelimit-reset-after")?,
                    })
                } else {
                    Ok(Self::None)
                }
            }
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn parse_map(map: &HeaderMap<HeaderValue>) -> RatelimitResult<RatelimitHeaders> {
    let bucket = header_str(map, "x-ratelimit-bucket")
        .ok()
        .map(ToOwned::to_owned);
    let global = header_bool(map, "x-ratelimit-global").unwrap_or(false);
    let limit = header_int(map, "x-ratelimit-limit")?;
    let remaining = header_int(map, "x-ratelimit-remaining")?;
    let reset = header_float(map, "x-ratelimit-reset")?;
    #[allow(clippy::cast_sign_loss)]
    let reset = (reset * 1000.).ceil() as u64;
    let reset_after = header_float(map, "x-ratelimit-reset-after")?;
    #[allow(clippy::cast_sign_loss)]
    let reset_after = (reset_after * 1000.).ceil() as u64;

    Ok(RatelimitHeaders::Present {
        bucket,
        global,
        limit,
        remaining,
        reset,
        reset_after,
    })
}

fn header_bool(map: &HeaderMap<HeaderValue>, name: &'static str) -> RatelimitResult<bool> {
    let value = map
        .get(name)
        .ok_or_else(|| RatelimitError::header_missing(name))?;

    let text = value.to_str().map_err(|source| {
        RatelimitError::header_not_utf8(name, value.as_bytes().to_owned(), source)
    })?;

    let end = text.parse().map_err(|source| RatelimitError {
        kind: RatelimitErrorType::ParsingBoolText {
            name,
            text: text.to_owned(),
        },
        source: Some(Box::new(source)),
    })?;

    Ok(end)
}

fn header_float(map: &HeaderMap<HeaderValue>, name: &'static str) -> RatelimitResult<f64> {
    let value = map
        .get(name)
        .ok_or_else(|| RatelimitError::header_missing(name))?;

    let text = value.to_str().map_err(|source| {
        RatelimitError::header_not_utf8(name, value.as_bytes().to_owned(), source)
    })?;

    let end = text.parse().map_err(|source| RatelimitError {
        kind: RatelimitErrorType::ParsingFloatText {
            name,
            text: text.to_owned(),
        },
        source: Some(Box::new(source)),
    })?;

    Ok(end)
}

fn header_int(map: &HeaderMap<HeaderValue>, name: &'static str) -> RatelimitResult<u64> {
    let value = map
        .get(name)
        .ok_or_else(|| RatelimitError::header_missing(name))?;

    let text = value.to_str().map_err(|source| {
        RatelimitError::header_not_utf8(name, value.as_bytes().to_owned(), source)
    })?;

    let end = text.parse().map_err(|source| RatelimitError {
        kind: RatelimitErrorType::ParsingIntText {
            name,
            text: text.to_owned(),
        },
        source: Some(Box::new(source)),
    })?;

    Ok(end)
}

fn header_str<'a>(map: &'a HeaderMap<HeaderValue>, name: &'static str) -> RatelimitResult<&'a str> {
    let value = map
        .get(name)
        .ok_or_else(|| RatelimitError::header_missing(name))?;

    let text = value.to_str().map_err(|source| {
        RatelimitError::header_not_utf8(name, value.as_bytes().to_owned(), source)
    })?;

    Ok(text)
}
