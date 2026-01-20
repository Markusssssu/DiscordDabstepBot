use serenity::all::MessageBuilder;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::ShardManagerContainer;

#[command]
#[owners_only]
async fn join() -> CommandResult {

    Ok(())
}

#[command]
#[owners_only]
async fn leave() -> CommandResult {

    Ok(())
}

#[command]
#[owners_only]
async fn mute() -> CommandResult {

    Ok(())
}

#[command]
#[owners_only]
async fn unmute() -> CommandResult {

    Ok(())
}







