extern crate reqwest;
extern crate scraper;

use futures::StreamExt;
use std::env;

use scraper::{Html, Selector};
use telegram_bot::prelude::*;
use telegram_bot::{Api, Error, InputFileRef, Message, MessageKind, UpdateKind};
use tokio_compat_02::FutureExt;

enum RoadType {
    TerceiraPonte,
    Rodosol,
}

// TODO: we're using Box<dyn std::error:Error> here to avoid
// having to define a new error type for the operations of this
// bot. At some point, would be adequate to do so.
// This also makes this function incompatible with threads,
// since the compiler cannot guarantee safety due to the dynamic nature
async fn send_pictures(
    api: Api,
    message: Message,
    road_type: RoadType,
) -> Result<(), Box<dyn std::error::Error>> {
    let chat = message.chat.clone();
    let body_response = reqwest::get("https://www.rodosol.com.br/de-olho-na-via/")
        .await?
        .text()
        .await?;

    let fragment = Html::parse_document(&body_response);
    let selector = match road_type {
        RoadType::Rodosol => "[rel='prettyPhoto[RD]']",
        RoadType::TerceiraPonte => "[rel='prettyPhoto[TP]']",
    };

    let nodes = Selector::parse(selector).unwrap();

    // TODO: send these images in parallel
    for node in fragment.select(&nodes) {
        let photo_href = node.value().attr("href").unwrap();
        api.send(chat.photo(InputFileRef::new(photo_href))).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    let mut stream = api.stream();
    // .compat() is needed here
    // because reqwest uses tokio 0.2
    // while telegram-bot uses tokio 1.x
    while let Some(update) = stream.next().compat().await {
        if let UpdateKind::Message(message) = update?.kind {
            let api = api.clone();

            match message.kind {
                MessageKind::Text { ref data, .. } if data.as_str() == "/tp_now" => {
                    if let Err(err) = send_pictures(api, message, RoadType::TerceiraPonte).await {
                        eprintln!("Error: {:?}", err);
                    }
                }
                MessageKind::Text { ref data, .. } if data.as_str() == "/rodosol_now" => {
                    if let Err(err) = send_pictures(api, message, RoadType::Rodosol).await {
                        eprintln!("Error: {:?}", err);
                    }
                }
                _ => (),
            };
        }
    }

    Ok(())
}
