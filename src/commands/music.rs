use serenity::all::MessageBuilder;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::ShardManagerContainer;

#[command]
#[owners_only]
async fn play() -> CommandResult {
    Ok(())
}

#[command]
#[owners_only]
async fn stop() -> CommandResult {
    Ok(())
}

#[command]
#[owners_only]
async fn next() -> CommandResult {
    Ok(())
}




