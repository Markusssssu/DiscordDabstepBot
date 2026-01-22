use log::info;
use serenity::all::ResumedEvent;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
