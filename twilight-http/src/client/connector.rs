//! HTTP connectors with different features.

/// HTTPS connector using `rustls` as a TLS backend.
#[cfg(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))]
type HttpsConnector<T> = hyper_rustls::HttpsConnector<T>;
/// HTTPS connector using `hyper-tls` as a TLS backend.
#[cfg(all(
    feature = "native-tls",
    not(any(feature = "rustls-native-roots", feature = "rustls-webpki-roots"))
))]
type HttpsConnector<T> = hyper_tls::HttpsConnector<T>;

/// HTTP connector using `hickory` as a DNS backend.
#[cfg(feature = "hickory")]
type HttpConnector = hyper_hickory::TokioHickoryHttpConnector;
/// HTTP connector.
#[cfg(not(feature = "hickory"))]
type HttpConnector = hyper_util::client::legacy::connect::HttpConnector;

/// Re-exported generic connector for use in the client.
#[cfg(any(
    feature = "native-tls",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
))]
pub type Connector = HttpsConnector<HttpConnector>;
/// Re-exported generic connector for use in the client.
#[cfg(not(any(
    feature = "native-tls",
    feature = "rustls-native-roots",
    feature = "rustls-webpki-roots"
)))]
pub type Connector = HttpConnector;

/// Create a connector with the specified features.
pub fn create() -> Connector {
    #[cfg(not(feature = "hickory"))]
    let mut connector = HttpConnector::new();
    #[cfg(feature = "hickory")]
    let mut connector = hyper_hickory::TokioHickoryResolver::default().into_http_connector();

    connector.enforce_http(false);

    #[cfg(feature = "rustls-native-roots")]
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .expect("no native root certificates found")
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
        feature = "native-tls",
        not(feature = "rustls-native-roots"),
        not(feature = "rustls-webpki-roots")
    ))]
    let connector = hyper_tls::HttpsConnector::new_with_connector(connector);

    connector
}
