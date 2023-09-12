use super::{response::RawResponseFuture, request::RawRequest, connector};

#[derive(Debug)]
pub struct HttpClient {
    pub(crate) hyper: hyper::Client<connector::Connector>
}

impl HttpClient {
    pub fn new() -> Self {
        let connector = connector::create();

        let hyper = hyper::Client::builder().build(connector);

        HttpClient { hyper }
    }

    pub fn request(&self, req: RawRequest) -> RawResponseFuture {
        let inner = self.hyper.request(req.hyper);
        RawResponseFuture {
            inner
        }
    }
}
