use std::{collections::HashSet, env, sync::Arc};
use std::time::Duration;

use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use serenity::model::id::GuildId;
use tracing::{error, info};

use commands::{help::*, output::*, raffle::*, status::*, ticket::*};

use crate::commands::status;

mod commands;
mod api_helper;
mod model;


pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
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

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let sleep_time = env::var("LOOP_SLEEP").unwrap_or("0".to_string()).parse::<u64>().unwrap();
        if sleep_time != 0 {
            info!("Starting message update loop...");
            let mut message_id: u64 = 0;
            loop {
                if env::var("UPDATE_STATUS").unwrap_or("true".to_string()).parse::<bool>().unwrap() {
                    status::change_status_message(&ctx.http, &mut message_id).await;
                    info!("Message posted");
                    env::set_var("UPDATE_STATUS", "false");
                }
                tokio::time::sleep(Duration::from_secs(sleep_time)).await;
            }
        }
    }
}


#[group]
#[commands(
help,
raffle,
ticket,
status,
export)]
struct General;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    //Make sure API is up before continuing
    loop {
        match api_helper::get_raffle("0".to_owned()).await {
            Ok(_) => {
                info!("API-Connected gonna continue");
                break;
            }
            Err(e) => {
                error!("Unable to connect to API: {}",e);
                info!("Retrying after 10s");
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        };
    }


    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let http = Http::new_with_token(&token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework =
        StandardFramework::new().configure(|c| c.owners(owners).prefix("~"))
            .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
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
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}