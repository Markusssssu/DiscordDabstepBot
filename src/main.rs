use std::{env, sync::{Arc, Mutex}, time::Duration};
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

use vosk::{DecodingState, Model, Recognizer};

use hound::WavReader;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // let intents = GatewayIntents::GUILD_MESSAGES
    //     | GatewayIntents::DIRECT_MESSAGES
    //     | GatewayIntents::MESSAGE_CONTENT;
    //
    // let mut client =
    //     Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");
    //
    // println!("API Token {}", token);
    //
    // if let Err(why) = client.start().await {
    //     println!("Client error: {why:?}");
    // }

    let mut args = env::args();
    args.next();

    let model_path = args.next().expect("A model path was not provided");
    let wav_path = args
        .next()
        .expect("A path for the wav file to be read was not provided");

    let mut reader = WavReader::open(wav_path).expect("Could not create the WAV reader");
    let samples = reader
        .samples()
        .collect::<hound::Result<Vec<i16>>>()
        .expect("Could not read WAV file");

    let model = Model::new(model_path).expect("Could not create the model");
    let mut recognizer = Recognizer::new(&model, reader.spec().sample_rate as f32)
        .expect("Could not create the recognizer");

    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    for sample in samples.chunks(100) {
        recognizer.accept_waveform(sample).unwrap();
        println!("{:#?}", recognizer.partial_result());
    }

    println!("{:#?}", recognizer.final_result().multiple().unwrap());
}