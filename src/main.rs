mod commands;
mod database;

use dotenv::dotenv;
use std::env;

use serenity::prelude::*;
use serenity::{
    async_trait,
    client::Client,
    framework::standard::{
        macros::{command, group},
        CommandResult,
        StandardFramework
    },
    model::{
        gateway::{
            Ready,
            Activity
        },
        channel::Message,
        user::OnlineStatus
    }
};

use rusqlite::Connection;

use commands::{
    lucky_message::*,
    name_react::*,
    emoji::*,
    catchphrase::*
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as bot: {}", ready.user.name);
        
        ctx.set_presence(
            Some(Activity::playing("the game of life")),
            OnlineStatus::Online
        ).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        lucky_message(&ctx, &msg).await
            .expect("Failed to check for lucky messages");

        name_react(&ctx, &msg).await
            .expect("Failed to check for name reacts");
    }
}

#[group]
#[commands(pingus, emoji, catchphrase)]
struct Command;

#[tokio::main]
async fn main() {
    // load environment variables from .env
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .prefix("."))
        .group(&COMMAND_GROUP);

    // TODO: Create something to handle database actions more elegantly
    let conn = Connection::open("db.db3")
        .expect("Error connection to database");
    database::profile::setup(&conn)
        .expect("Error setting up database");

    let token = env::var("DISCORD_TOKEN").expect("Bot token is not set");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Error while running client: {:?}", why);
    }
}

#[command]
async fn pingus(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pongus!").await?;

    Ok(())
}