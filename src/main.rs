#![allow(deprecated)]
mod commands;
mod utils;
mod events;

use crate::utils::*;

use std::env;
use std::sync::Arc;
use std::collections::HashSet;

use dotenv::dotenv;
use tracing::{error, info};
use reqwest::Client as HttpClient;

use songbird::events::{Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent};
use songbird::SerenityInit;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::utils::MessageBuilder;
use serenity::framework::standard::Configuration;
use serenity::framework::standard::StandardFramework;
use serenity::framework::standard::macros::group;
use serenity::gateway::ShardManager;
use serenity::model::event::ResumedEvent;
use serenity::http::Http;
use serenity::Result as SerenityResult;
use serenity::prelude::*;

use crate::commands::admin::*;
use crate::commands::general::*;
use crate::commands::music::*;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

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

