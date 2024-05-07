//! HTTP connectors with different features.

/// HTTPS connector using `rustls` as a TLS backend.
#[cfg(any(
    feature = "rustls-native-roots",
    feature = "rustls-platform-verifier",
    feature = "rustls-webpki-roots"
))]
type HttpsConnector<T> = hyper_rustls::HttpsConnector<T>;
/// HTTPS connector using `hyper-tls` as a TLS backend.
#[cfg(all(
    feature = "native-tls",
    not(any(
        feature = "rustls-native-roots",
        feature = "rustls-platform-verifier",
        feature = "rustls-webpki-roots"
    ))
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
    feature = "rustls-platform-verifier",
    feature = "rustls-webpki-roots",
))]
pub type Connector = HttpsConnector<HttpConnector>;
/// Re-exported generic connector for use in the client.
#[cfg(not(any(
    feature = "native-tls",
    feature = "rustls-native-roots",
    feature = "rustls-platform-verifier",
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

    #[cfg(any(
        feature = "rustls-native-roots",
        feature = "rustls-platform-verifier",
        feature = "rustls-webpki-roots"
    ))]
    let connector = {
        #[cfg(not(any(feature = "rustls-ring", feature = "rustls-aws_lc_rs")))]
        let crypto_provider = rustls::crypto::CryptoProvider::get_default()
            .expect("No default crypto provider installed or configured via crate features")
            .clone();
        #[cfg(feature = "rustls-aws_lc_rs")]
        let crypto_provider = rustls::crypto::aws_lc_rs::default_provider();
        #[cfg(all(feature = "rustls-ring", not(feature = "rustls-aws_lc_rs")))]
        let crypto_provider = rustls::crypto::ring::default_provider();

        #[cfg(all(
            feature = "rustls-native-roots",
            not(feature = "rustls-platform-verifier")
        ))]
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_provider_and_native_roots(crypto_provider)
            .expect("no native roots found")
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .wrap_connector(connector);
        #[cfg(feature = "rustls-platform-verifier")]
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_provider_and_platform_verifier(crypto_provider)
            .expect("no usable cipher suites in crypto provider")
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .wrap_connector(connector);
        #[cfg(all(
            feature = "rustls-webpki-roots",
            not(any(feature = "rustls-native-roots", feature = "rustls-platform-verifier"))
        ))]
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_provider_and_webpki_roots(crypto_provider)
            .expect("no usable cipher suites in crypto provider")
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .wrap_connector(connector);

        connector
    };

    #[cfg(all(
        feature = "native-tls",
        not(feature = "rustls-native-roots"),
        not(feature = "rustls-platform-verifier"),
        not(feature = "rustls-webpki-roots")
    ))]
    let connector = hyper_tls::HttpsConnector::new_with_connector(connector);

    connector
}
