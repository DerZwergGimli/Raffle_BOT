use std::{env, thread, time};
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub mod help;
pub mod raffle;
pub mod ticket;
pub mod status;
pub mod output;

async fn message_begin() -> CommandResult {
    if env::var("DELETE_MESSAGE").unwrap_or("false".to_string()).contains("true") {
        //    typing = msg.channel_id.start_typing(&ctx.http)?;
    }
    Ok(())
}


async fn message_end(ctx: &&Context, msg: &Message, mut text: String) -> CommandResult {
    let destruct_timer = env::var("MESSAGE_TIMER").unwrap_or("5".to_string()).parse::<u64>().unwrap();

    if env::var("DELETE_MESSAGE").unwrap_or("false".to_string()).contains("true") {
        msg.delete(&ctx.http).await?;

        text.push_str(&*format!("\n > this message will destruct in {}s", destruct_timer));
    }



    let msg_id = msg.channel_id.say(&ctx.http, text).await?;

    if env::var("DELETE_MESSAGE").unwrap_or("false".to_string()).contains("true") {
        thread::sleep(time::Duration::from_secs(destruct_timer));
        msg.channel_id.delete_message(&ctx.http, msg_id).await?;
    }
    Ok(())
}

