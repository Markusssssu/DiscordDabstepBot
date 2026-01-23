#![allow(deprecated)]

/*=========== Modules {Commands, Utils, Events, Hanlder} ============*/
mod commands;
mod utils;
mod events;
mod handler;

/*===================================================================*/

use std::env;
use dotenv::dotenv;

use reqwest::Client as HttpClient;

/*=========== Used {Songbird} ============*/

use songbird::SerenityInit;

/*=======================================*/

/*=========== Used {Serenity} ============*/

use serenity::framework::standard::Configuration;
use serenity::framework::standard::StandardFramework;
use serenity::framework::standard::macros::group;
use serenity::prelude::*;

/*=======================================*/

/*=========== Used Crates {Handler, Utils, Admin, General, Music} ============*/
use crate::utils::*;
use crate::handler::*;
use crate::commands::admin::*;
use crate::commands::general::*;
use crate::commands::music::*;

/*============================================================================*/

pub struct ShardManagerContainer;

/*=========== Commands {*} ============*/

// for admin {join, leave, mute, unmute}
// for general {quit, ping, help}
// for music {play, stop, next}

/*====================================*/

#[group]
#[commands(join, leave, mute, unmute, ping, play, stop, next, help, skip,)]
struct General;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    /*========== Thread #1 {Discord Bot} =============*/

    let bot = tokio::spawn(async move {
            let framework = StandardFramework::new().group(&GENERAL_GROUP);
            framework.configure(Configuration::new().prefix("~"));

            let intents = GatewayIntents::non_privileged()
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::GUILD_MEMBERS;

            let mut client = Client::builder(&token, intents)
                .event_handler(Handler)
                .framework(framework)
                .register_songbird()
                .type_map_insert::<HttpKey>(HttpClient::new())
                .await
                .expect("Err creating client");

            let _ = client
                .start()
                .await
                .map_err(|why| println!("Client ended: {:?}", why));

            let _signal_err = tokio::signal::ctrl_c().await;
            println!("Received Ctrl-C, shutting down.");
    });

    /*===============================================*/

    /*========== Thread #2 {Vosk - Microphone} =============*/

    let vosk = tokio::spawn(async move {

    });

    /*===============================================*/

    /*========== Thread #3 {Ratatui} =============*/

    let ratatui = tokio::spawn(async move {

    });

    /*===============================================*/

    /*========== Tokio::Select {Shutdown} =============*/

    tokio::select! {
        _ = bot => println!("Бот завершил работу"),
        _ = tokio::signal::ctrl_c() => println!("Получен Ctrl-C, выходим..."),
    }

    /*=================================================*/
}

