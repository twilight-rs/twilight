#[cfg(feature = "simd-json")]
pub use dep_simd_json::{to_vec, Deserializer as JsonDeserializer, Error as JsonError};
#[cfg(not(feature = "simd-json"))]
pub use serde_json::{to_vec, Deserializer as JsonDeserializer, Error as JsonError};

use serde::de::DeserializeOwned;

#[cfg(feature = "simd-json")]
use dep_simd_json::Result as JsonResult;
#[cfg(not(feature = "simd-json"))]
use serde_json::Result as JsonResult;

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
