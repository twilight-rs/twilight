//! HTTP connectors with different features.

/// HTTPS connector using `rustls` as a TLS backend.
#[cfg(feature = "hyper-rustls")]
type HttpsConnector<T> = hyper_rustls::HttpsConnector<T>;
/// HTTPS connector using `hyper-tls` as a TLS backend.
#[cfg(all(feature = "hyper-tls", not(feature = "hyper-rustls")))]
type HttpsConnector<T> = hyper_tls::HttpsConnector<T>;

/// HTTP connector using `trust-dns` as a DNS backend.
#[cfg(feature = "trust-dns")]
type HttpConnector = hyper_trust_dns::TrustDnsHttpConnector;
/// HTTP connector.
#[cfg(not(feature = "trust-dns"))]
type HttpConnector = hyper::client::HttpConnector;

/// Re-exported generic connector for use in the client.
#[cfg(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
))]
pub type Connector = HttpsConnector<HttpConnector>;
/// Re-exported generic connector for use in the client.
#[cfg(not(any(
    feature = "native",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
)))]
pub type Connector = HttpConnector;

/// Create a connector with the specified features.
pub fn create() -> Connector {
    #[cfg(not(feature = "trust-dns"))]
    let mut connector = hyper::client::HttpConnector::new();
    #[cfg(feature = "trust-dns")]
    let mut connector = hyper_trust_dns::new_trust_dns_http_connector();

    connector.enforce_http(false);

    #[cfg(feature = "rustls-native-roots")]
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .wrap_connector(connector);
    #[cfg(all(feature = "rustls-webpki-roots", not(feature = "rustls-native-roots")))]
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_or_http()
        .enable_http1()
        .enable_http2()
        .wrap_connector(connector);
    #[cfg(all(
        feature = "hyper-tls",
        not(feature = "rustls-native-roots"),
        not(feature = "rustls-webpki-roots")
    ))]
    let connector = hyper_tls::HttpsConnector::new_with_connector(connector);

    connector
}
