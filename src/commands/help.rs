use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use crate::commands::{message_begin, message_end};

#[command]
pub async fn help(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    message_begin(&ctx, msg).await;
    let typing= msg.channel_id.start_typing(&ctx.http)?;

    let mut text = String::new();
    text.push_str(format!("**RaffleBOT Help:**\n").as_ref());

    text.push_str(format!("``` \
    Commands: \n
    ~status \t [prints a status of raffles]\n
    ~raffle \t [prints commands for raffle]\n
    ~ticket \t [prints commands for tickets]\n
    ~output \t [print raffle tickets as file]```").as_ref());


    //    text.push_str("> this message will destruct in 5s");
    //let msg_id = msg.channel_id.say(&ctx.http, text).await?;
    message_end(&ctx, msg, text).await;
    Ok(())
}