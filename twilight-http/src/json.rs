#[cfg(not(feature = "simd-json"))]
pub use serde_json::{to_vec, Error as JsonError};
#[cfg(feature = "simd-json")]
pub use simd_json::{to_vec, Error as JsonError};

use serde::de::DeserializeOwned;

#[cfg(not(feature = "simd-json"))]
use serde_json::Result as JsonResult;
#[cfg(feature = "simd-json")]
use simd_json::Result as JsonResult;

pub fn from_bytes<T: DeserializeOwned>(bytes: &[u8]) -> JsonResult<T> {
    #[cfg(not(feature = "simd-json"))]
    {
        serde_json::from_slice(bytes)
    }

    #[cfg(feature = "simd-json")]
    {
        // Bytes does not implement DerefMut so we have to allocate
        simd_json::from_slice(&mut bytes.to_vec())
    }
}
