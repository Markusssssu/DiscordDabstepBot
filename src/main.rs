#![allow(deprecated)]
mod commands;
mod utils;
mod events;
mod handler;

use crate::utils::*;
use crate::handler::*;

use std::env;

use tokio::sync::mpsc;

use dotenv::dotenv;
use reqwest::Client as HttpClient;

use songbird::SerenityInit;
use serenity::framework::standard::Configuration;
use serenity::framework::standard::StandardFramework;
use serenity::framework::standard::macros::group;
use serenity::prelude::*;

use crate::commands::admin::*;
use crate::commands::general::*;
use crate::commands::music::*;

pub struct ShardManagerContainer;

/*=====commands=====*/
// for admin {join, leave, mute, unmute}
// for general {quit, ping, help}
// for music {play, stop, next}
/*=================*/

#[group]
#[commands(join, leave, mute, unmute, quit, ping, play, stop, next, help)]
struct General;

#[tokio::main]
async fn main() {

    dotenv().ok();

    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

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

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    let _signal_err = tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}

