pub(super) use super::{Pending, Request};
pub use crate::{client::Client, error::Result, routing::Route};
pub use reqwest::Method;
pub use serde::{Deserialize, Serialize};
pub use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
