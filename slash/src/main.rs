use bytes::Buf;
use ed25519_dalek::{PublicKey, Signature, Verifier, PUBLIC_KEY_LENGTH};
use hex::FromHex;
use once_cell::sync::Lazy;
use std::future::Future;
use twilight_model::applications::{
    CommandCallbackData, Interaction, InteractionResponse, InteractionResponseType, InteractionType
};
use twilight_model::channel::message::MessageFlags;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};

type GenericError = Box<dyn std::error::Error + Send + Sync>;

static PUB_KEY: Lazy<PublicKey> = Lazy::new(|| {
    PublicKey::from_bytes(
        &<[u8; PUBLIC_KEY_LENGTH] as FromHex>::from_hex(
            "BLOBSVED",
        )
        .unwrap(),
    )
    .unwrap()
});

async fn interaction_handler<F>(
    req: Request<Body>,
    f: impl Fn(Interaction) -> F,
) -> Result<Response<Body>, GenericError>
where
    F: Future<Output = Result<InteractionResponse, GenericError>>,
{
    if req.method() != &Method::POST {
        return Ok(Response::builder().status(405).body(Body::empty())?);
    }
    if req.uri().path() != "/" {
        return Ok(Response::builder().status(404).body(Body::empty())?);
    }

    let signature = if let Some(hex_sig) = req.headers().get("x-signature-ed25519") {
        Signature::new(FromHex::from_hex(hex_sig)?)
    } else {
        return Ok(Response::builder().status(400).body(Body::empty())?);
    };

    let whole_body = hyper::body::aggregate(req).await?;

    if let Err(_) = PUB_KEY.verify(&whole_body.bytes(), &signature) {
        return Ok(Response::builder().status(403).body(Body::empty())?);
    }
    println!("{}", String::from_utf8(whole_body.bytes().to_vec()).unwrap());
    let interaction = serde_json::from_slice::<Interaction>(whole_body.bytes())?;

    if let InteractionType::Ping = interaction.kind {
        let response = InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None,
        };

        let json = serde_json::to_vec(&response)?;
        
        return Ok(Response::builder().status(200).body(json.into())?);
    }
    
    let response = f(interaction).await?;

    let res_status = match response.kind {
        InteractionResponseType::Acknowledge => 202,
        _ => 200,
    };

    let json = serde_json::to_vec(&response)?;

    Ok(Response::builder().status(res_status).body(json.into())?)
}

async fn handler(i: Interaction) -> Result<InteractionResponse, GenericError> {
    let name = i.data.clone().unwrap().name.clone();
    match name.as_str() {
        "vroom" => vroom(i).await,
        "debug" => debug(i).await,
        _ => vroom(i).await,
    }
}

async fn debug(i: Interaction) -> Result<InteractionResponse, GenericError> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(CommandCallbackData {
            tts: None,
            content: format!("```rust\n{:?}\n```", i),
            embeds: vec![],
            flags: MessageFlags::empty(),
        }),
    })
}

async fn vroom(_: Interaction) -> Result<InteractionResponse, GenericError> {
    Ok(InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(CommandCallbackData {
            tts: None,
            content: "Vroom vroom".to_string(),
            embeds: vec![],
            flags: MessageFlags::empty(),
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
