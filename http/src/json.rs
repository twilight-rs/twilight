use crate::error::{Error, ErrorType};

#[cfg(not(feature = "simd-json"))]
pub use serde_json::to_vec;
#[cfg(feature = "simd-json")]
pub use simd_json::to_vec;

use hyper::body::Bytes;
use serde::Deserialize;

#[cfg(not(feature = "simd-json"))]
use serde_json::Result as JsonResult;
#[cfg(feature = "simd-json")]
use simd_json::Result as JsonResult;

pub fn from_bytes<'a, T: Deserialize<'a>>(bytes: &'a Bytes) -> JsonResult<T> {
    #[cfg(not(feature = "simd-json"))]
    {
        serde_json::from_slice(&bytes)
    }

    #[cfg(feature = "simd-json")]
    {
        // Bytes does not implement DerefMut so we have to allocate
        simd_json::from_slice(&mut bytes.to_vec())
    }
}

pub fn parse_bytes<'a, T: Deserialize<'a>>(bytes: &'a Bytes) -> Result<T, Error> {
    from_bytes(bytes).map_err(|source| Error {
        kind: ErrorType::Parsing {
            body: bytes.to_vec(),
        },
        source: Some(Box::new(source)),
    })
}
