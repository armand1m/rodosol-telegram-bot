extern crate futures;
extern crate reqwest;
extern crate scraper;

use futures::future;
use futures::StreamExt;
use std::env;

use scraper::{Html, Selector};
use telegram_bot::prelude::*;
use telegram_bot::{Api, InputFileRef, Message, MessageKind, UpdateKind};
use tokio_compat_02::FutureExt;

#[derive(Debug)]
enum Command {
    TerceiraPonteNow,
    RodosolNow,
}

#[derive(Debug)]
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
    let selected_nodes = fragment.select(&nodes);
    let collected = selected_nodes.into_iter().collect::<Vec<_>>();

    if collected.len() == 0 {
        api.send(message.text_reply("Imagens indisponiveis no site da Rodosol."))
            .await?;
        return Ok(());
    }

    let _ = future::try_join_all(collected.into_iter().map(|node| {
        let photo_href = node.value().attr("href").unwrap();
        api.send(message.photo_reply(InputFileRef::new(photo_href)))
    }))
    .compat()
    .await
    .unwrap();

    Ok(())
}

fn get_command(message: &str, bot_name: &str) -> Option<Command> {
    if !message.starts_with("/") {
        return None;
    }

    // splits the bot name from the command, in case it is there
    let mut cmd = message.clone();
    if cmd.ends_with(bot_name) {
        cmd = cmd.rsplitn(2, '@').skip(1).next().unwrap();
    }

    match cmd {
        "/tp_now" => Some(Command::TerceiraPonteNow),
        "/rodosol_now" => Some(Command::RodosolNow),
        _ => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let bot_name = env::var("TELEGRAM_BOT_NAME").expect("TELEGRAM_BOT_NAME not set");
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    let mut stream = api.stream();

    // .compat() is needed here
    // because reqwest uses tokio 0.2
    // while telegram-bot uses tokio 1.x
    //
    // compat() is a trait implemented by the
    // tokio-compat-02 package to allow different libraries using
    // different tokio runtimes to work in the same process
    while let Some(update) = stream.next().compat().await {
        if let UpdateKind::Message(message) = update?.kind {
            let api = api.clone();

            if let MessageKind::Text { ref data, .. } = message.kind {
                let command = get_command(data.as_str(), bot_name.as_str());

                match command {
                    Some(Command::TerceiraPonteNow) => {
                        send_pictures(api, message, RoadType::TerceiraPonte).await?
                    }
                    Some(Command::RodosolNow) => {
                        send_pictures(api, message, RoadType::Rodosol).await?
                    }
                    _ => (),
                }
            }
        }
    }

    Ok(())
}
