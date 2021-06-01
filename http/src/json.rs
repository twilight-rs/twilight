#[cfg(not(feature = "simd-json"))]
pub use serde_json::to_vec;
#[cfg(feature = "simd-json")]
pub use simd_json::to_vec;

use serde::Deserialize;

#[cfg(not(feature = "simd-json"))]
use serde_json::{from_slice as inner_from_slice, Result as JsonResult};
#[cfg(feature = "simd-json")]
use simd_json::{from_slice as inner_from_slice, Result as JsonResult};

// Function will automatically cast mutable references to immutable for
// `serde_json`.
pub fn from_slice<'a, T: Deserialize<'a>>(s: &'a mut [u8]) -> JsonResult<T> {
    inner_from_slice(s)
}
