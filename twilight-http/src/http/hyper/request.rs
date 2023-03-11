use http::{Method, Uri, HeaderMap, HeaderValue};
use hyper::Body;

use crate::{Error, error::ErrorType};

pub struct RawRequest {
    pub(crate) hyper: hyper::Request<Body>
}
