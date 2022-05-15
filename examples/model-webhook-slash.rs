use ed25519_dalek::{PublicKey, Verifier, PUBLIC_KEY_LENGTH};
use hex::FromHex;
use hyper::{
    header::CONTENT_TYPE,
    http::StatusCode,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server,
};
use once_cell::sync::Lazy;
use std::future::Future;
use twilight_model::{
    application::interaction::Interaction,
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

type GenericError = Box<dyn std::error::Error + Send + Sync>;

/// Public key given from Discord.
static PUB_KEY: Lazy<PublicKey> = Lazy::new(|| {
    PublicKey::from_bytes(&<[u8; PUBLIC_KEY_LENGTH] as FromHex>::from_hex("PUBLIC_KEY").unwrap())
        .unwrap()
});

/// Main request handler which will handle checking the signature.
///
/// Responses are made by giving a function that takes a Interaction and returns
/// a InteractionResponse or a error.
async fn interaction_handler<F>(
    req: Request<Body>,
    f: impl Fn(Interaction) -> F,
) -> Result<Response<Body>, GenericError>
where
    F: Future<Output = Result<InteractionResponse, GenericError>>,
{
    // Check that the method used is a POST, all other methods are not allowed.
    if req.method() != Method::POST {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())?);
    }

    // Check if the path the request is sent to is the root of the domain.
    //
    // This filter is for the purposes of this example. The user may filter by
    // any path they choose.
    if req.uri().path() != "/" {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())?);
    }

    // Extract the timestamp header for use later to check the signature.
    let timestamp = if let Some(ts) = req.headers().get("x-signature-timestamp") {
        ts.to_owned()
    } else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())?);
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
            .body(Body::empty())?);
    };

    // Fetch the whole body of the request as that is needed to check the
    // signature against.
    let whole_body = hyper::body::to_bytes(req).await?;

    // Check if the signature matches and else return a error response.
    if PUB_KEY
        .verify(
            vec![timestamp.as_bytes(), &whole_body].concat().as_ref(),
            &signature,
        )
        .is_err()
    {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    // Deserialize the body into a interaction.
    let interaction = serde_json::from_slice::<Interaction>(&whole_body)?;

    match interaction {
        // Return a Pong if a Ping is received.
        Interaction::Ping(_) => {
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
        Interaction::ApplicationCommand(_) => {
            // Run the handler to gain a response.
            let response = f(interaction).await?;

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
            .body(Body::empty())?),
    }
}

/// Interaction handler that matches on the name of the interaction that
/// have been dispatched from Discord.
async fn handler(i: Interaction) -> Result<InteractionResponse, GenericError> {
    match &i {
        Interaction::ApplicationCommand(cmd) => match cmd.data.name.as_ref() {
            "vroom" => vroom(i).await,
            "debug" => debug(i).await,
            _ => debug(i).await,
        },
        _ => Err("invalid interaction data".into()),
    }
}

/// Example of a handler that returns the formatted version of the interaction.
async fn debug(i: Interaction) -> Result<InteractionResponse, GenericError> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some(format!("```rust\n{i:?}\n```")),
            ..Default::default()
        }),
    })
}

/// Example of interaction that responds with a message saying "Vroom vroom".
async fn vroom(_: Interaction) -> Result<InteractionResponse, GenericError> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionResponseData {
            content: Some("Vroom vroom".to_owned()),
            ..Default::default()
        }),
    })
}

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // Local address to bind the service to.
    let addr = "127.0.0.1:3030".parse().unwrap();

    // Make the interaction handler into a service function.
    let interaction_service = make_service_fn(|_| async {
        Ok::<_, GenericError>(service_fn(|req| interaction_handler(req, handler)))
    });

    // Construct the server and serve the interaction service.
    let server = Server::bind(&addr).serve(interaction_service);

    // Start the server.
    server.await?;

    Ok(())
}
