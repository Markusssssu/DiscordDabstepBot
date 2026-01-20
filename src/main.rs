#![allow(deprecated)]
mod commands;

use std::env;
use std::sync::Arc;
use std::collections::HashSet;
use dotenv::dotenv;

use tracing::{error, info};

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
// for general {quit, ping}
// for music {play, stop, next}
/*=================*/

#[group]
#[commands(join, leave, mute, unmute, quit, ping, play, stop, next)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().owners(owners).prefix("~"));

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
