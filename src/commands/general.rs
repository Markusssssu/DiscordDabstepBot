use serenity::all::MessageBuilder;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::ShardManagerContainer;

#[command]
#[owners_only]
async fn quit(ctx: &Context, msg: &Message) -> CommandResult {

    Ok(())
}

#[command]
#[owners_only]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {

    Ok(())
}