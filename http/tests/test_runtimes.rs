use twilight_http::Client;

#[cfg(feature = "smol-runtime")]
#[ignore]
#[test]
fn test_runtime_smol() {
    let client = Client::new("foo");

    assert!(smol::run(client.gateway()).is_ok());
}

#[cfg(feature = "tokio-runtime")]
#[ignore]
#[tokio::test]
async fn test_runtime_tokio() {
    let client = Client::new("foo");
    assert!(client.gateway().await.is_ok());
}
