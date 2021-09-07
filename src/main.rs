use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::collections::HashMap;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {

    // Load settings
    let mut settings = config::Config::default();
    settings
        // Add in `./botSettings.toml`
        .merge(config::File::with_name("botSettings")).unwrap();

    let settings_map = settings.try_into::<HashMap<String, String>>().unwrap();

    // Print out the botSettings (as a HashMap)
    println!("{:?}",
             settings_map);

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(settings_map.get("prefix").expect("You have not set a prefix in botSettings.toml!"))) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the settings
    let mut client = Client::builder(settings_map.get("token").expect("You have not set your token in botSettings.toml!"))
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}