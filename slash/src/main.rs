use bytes::Buf;
use ed25519_dalek::{PublicKey, Signature, Verifier, PUBLIC_KEY_LENGTH};
use hex::FromHex;
use once_cell::sync::Lazy;
use std::convert::TryInto;
use std::future::Future;
use twilight_model::applications::{
    CommandCallbackData, GuildInteraction, Interaction, InteractionData, InteractionEnvelope,
    InteractionEnvelopeParseError, InteractionResponse, InteractionResponseType, InteractionType,
};
use twilight_model::channel::message::MessageFlags;

use hyper::service::{make_service_fn, service_fn};
use hyper::{http::StatusCode, Body, Method, Request, Response, Server};

type GenericError = Box<dyn std::error::Error + Send + Sync>;

static PUB_KEY: Lazy<PublicKey> = Lazy::new(|| {
    PublicKey::from_bytes(&<[u8; PUBLIC_KEY_LENGTH] as FromHex>::from_hex("BLOBSVED").unwrap())
        .unwrap()
});

async fn interaction_handler<F>(
    req: Request<Body>,
    f: impl Fn(GuildInteraction) -> F,
) -> Result<Response<Body>, GenericError>
where
    F: Future<Output = Result<InteractionResponse, GenericError>>,
{
    if req.method() != &Method::POST {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())?);
    }
    if req.uri().path() != "/" {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())?);
    }

    let timestamp = if let Some(ts) = req.headers().get("x-signature-timestamp") {
        ts.to_owned()
    } else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())?);
    };

    let signature = if let Some(hex_sig) = req.headers().get("x-signature-ed25519") {
        Signature::new(FromHex::from_hex(hex_sig)?)
    } else {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())?);
    };

    let whole_body = hyper::body::aggregate(req).await?;

    if let Err(_) = PUB_KEY.verify(
        vec![timestamp.as_bytes(), whole_body.bytes()]
            .concat()
            .as_ref(),
        &signature,
    ) {
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::empty())?);
    }
    println!(
        "{}",
        String::from_utf8(whole_body.bytes().to_vec()).unwrap()
    );

    let interaction = match serde_json::from_slice::<InteractionEnvelope>(whole_body.bytes())
        .map_err(|e| InteractionEnvelopeParseError::DecodeError(Box::new(e)))
        .and_then(|i| i.try_into())
    {
        Ok(i) => i,
        Err(_) => {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())?);
        }
    };

    match interaction {
        Interaction::Global(i) if i.kind == InteractionType::Ping => {
            let response = InteractionResponse {
                kind: InteractionResponseType::Pong,
                data: None,
            };

            let json = serde_json::to_vec(&response)?;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(json.into())?)
        }
        Interaction::WithGuildId(i) => {
            let response = f(i).await?;

            let res_status = match response.kind {
                InteractionResponseType::Acknowledge => StatusCode::ACCEPTED,
                _ => StatusCode::OK,
            };

            let json = serde_json::to_vec(&response)?;

            Ok(Response::builder().status(res_status).body(json.into())?)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())?),
    }
}

async fn handler(i: GuildInteraction) -> Result<InteractionResponse, GenericError> {
    match i.data {
        InteractionData::ApplicationCommand(ref cmd) => match cmd.name.as_ref() {
            "vroom" => vroom(i).await,
            "debug" => debug(i).await,
            _ => debug(i).await,
        },
        _ => Err("invalid interaction data".into()),
    }
}

async fn debug(i: GuildInteraction) -> Result<InteractionResponse, GenericError> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(CommandCallbackData {
            tts: None,
            content: format!("```rust\n{:?}\n```", i),
            embeds: vec![],
            // flags: MessageFlags::empty(),
        }),
    })
}

async fn vroom(_: GuildInteraction) -> Result<InteractionResponse, GenericError> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(CommandCallbackData {
            tts: None,
            content: "Vroom vroom".to_string(),
            embeds: vec![],
            // flags: MessageFlags::empty(),
        }),
    })
}

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3030".parse().unwrap();

    //let interaction_service = service_fn(|req| interaction_handler(req, vroom));

    let interaction_service = make_service_fn(|_| async {
        Ok::<_, GenericError>(service_fn(|req| interaction_handler(req, handler)))
    });

    let server = Server::bind(&addr).serve(interaction_service);

    server.await?;

    Ok(())
}
