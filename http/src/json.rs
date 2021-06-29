#[cfg(not(feature = "simd-json"))]
pub use serde_json::to_vec;
#[cfg(feature = "simd-json")]
pub use simd_json::to_vec;

use crate::error::{Error, ErrorType};
use hyper::body::Bytes;
use serde::de::DeserializeOwned;

#[cfg(not(feature = "simd-json"))]
use serde_json::Result as JsonResult;
#[cfg(feature = "simd-json")]
use simd_json::Result as JsonResult;

pub fn from_bytes<T: DeserializeOwned>(bytes: &Bytes) -> JsonResult<T> {
    #[cfg(not(feature = "simd-json"))]
    {
        serde_json::from_slice::<T>(&bytes)
    }

    #[cfg(feature = "simd-json")]
    {
        // Bytes does not implement DerefMut so we have to allocate
        simd_json::from_slice(&mut bytes.to_vec())
    }
}

pub fn parse_bytes<T: DeserializeOwned>(bytes: &Bytes) -> Result<T, Error> {
    from_bytes(bytes).map_err(|source| Error {
        kind: ErrorType::Parsing {
            body: bytes.to_vec(),
        },
        source: Some(Box::new(source)),
    })
}
