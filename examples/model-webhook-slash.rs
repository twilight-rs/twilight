use ed25519_dalek::{PUBLIC_KEY_LENGTH, Verifier, VerifyingKey};
use hex::FromHex;
use http_body_util::{BodyExt, Full};
use hyper::{
    Method, Request, Response,
    body::{Bytes, Incoming},
    header::CONTENT_TYPE,
    http::StatusCode,
    server::conn::http1,
    service::service_fn,
};
use hyper_util::rt::TokioIo;
use once_cell::sync::Lazy;
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;
use twilight_model::{
    application::interaction::{Interaction, InteractionType, application_command::CommandData},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

/// Public key given from Discord.
static PUB_KEY: Lazy<VerifyingKey> = Lazy::new(|| {
    VerifyingKey::from_bytes(&<[u8; PUBLIC_KEY_LENGTH] as FromHex>::from_hex("PUBLIC_KEY").unwrap())
        .unwrap()
});

/// Main request handler which will handle checking the signature.
///
/// Responses are made by giving a function that takes a Interaction and returns
/// a InteractionResponse or a error.
async fn interaction_handler<F>(
    req: Request<Incoming>,
    f: impl Fn(Box<CommandData>) -> F,
) -> anyhow::Result<Response<Full<Bytes>>>
where
    F: Future<Output = anyhow::Result<InteractionResponse>>,
{
    // Check that the method used is a POST, all other methods are not allowed.
    if req.method() != Method::POST {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Full::default())?);
    }

    // Check if the path the request is sent to is the root of the domain.
    //
    // This filter is for the purposes of this example. The user may filter by
    // any path they choose.
    if req.uri().path() != "/" {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::default())?);
    }

    // Extract the timestamp header for use later to check the signature.
    let timestamp = if let Some(ts) = req.headers().get("x-signature-timestamp") {
        ts.to_owned()
    } else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Full::default())?);
    };

    // Extract the signature to check against.
    let signature = if let Some(hex_sig) = req
        .headers()
        .get("x-signature-ed25519")
        .and_then(|v| v.to_str().ok())
    {
        hex_sig.parse().unwrap()
    } else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Full::default())?);
    };

    // Fetch the whole body of the request as that is needed to check the
    // signature against.
    let whole_body = req.collect().await?.to_bytes();

    // Check if the signature matches and else return a error response.
    if PUB_KEY
        .verify(
            [timestamp.as_bytes(), &whole_body].concat().as_ref(),
            &signature,
        )
        .is_err()
    {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Full::default())?);
    }

    // Deserialize the body into a interaction.
    let interaction = serde_json::from_slice::<Interaction>(&whole_body)?;

    match interaction.kind {
        // Return a Pong if a Ping is received.
        InteractionType::Ping => {
            let response = InteractionResponse {
                kind: InteractionResponseType::Pong,
                data: None,
            };

            let json = serde_json::to_vec(&response)?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "application/json")
                .body(json.into())?)
        }
        // Respond to a slash command.
        InteractionType::ApplicationCommand => {
            // Run the handler to gain a response.
            let data = interaction.data.unwrap().try_into().unwrap();
            let response = f(data).await?;

            // Serialize the response and return it back to Discord.
            let json = serde_json::to_vec(&response)?;

            Ok(Response::builder()
                .header(CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(json.into())?)
        }
        // Unhandled interaction types.
        _ => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Full::default())?),
    }
}

/// Interaction handler that matches on the name of the interaction that
/// have been dispatched from Discord.
async fn handler(data: Box<CommandData>) -> anyhow::Result<InteractionResponse> {
    match data.name.as_ref() {
        "vroom" => vroom(data).await,
        "debug" => debug(data).await,
        _ => debug(data).await,
    }
}

/// Example of a handler that returns the formatted version of the interaction.
async fn debug(data: Box<CommandData>) -> anyhow::Result<InteractionResponse> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some(format!("```rust\n{data:?}\n```")),
            ..Default::default()
        }),
    })
}

/// Example of interaction that responds with a message saying "Vroom vroom".
async fn vroom(_: Box<CommandData>) -> anyhow::Result<InteractionResponse> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some("Vroom vroom".to_owned()),
            ..Default::default()
        }),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // Select rustls backend
    rustls::crypto::ring::default_provider()
        .install_default()
        .unwrap();

    // Local address to bind the service to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    // Bind the server and serve the interaction service.
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (conn, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = http1::Builder::new()
                .serve_connection(
                    TokioIo::new(conn),
                    service_fn(|req| interaction_handler(req, handler)),
                )
                .await
            {
                tracing::error!("Error handling HTTP request: {e}");
            };
        });
    }
}
