use std::{env};
use ascii_table::{Align, AsciiTable};
use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::api_helper;
use crate::commands::{message_begin, message_end};
use crate::model::Raffle;

#[command]
pub async fn raffle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    message_begin().await;
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    // Make sure to check for permission here
    let user_role = msg.author.has_role(
        &ctx,
        GuildId(env::var("GUILD_ID").unwrap_or("0".to_string()).parse::<u64>().unwrap()),
        RoleId(env::var("ROLE_ID").unwrap_or("0".to_string()).parse::<u64>().unwrap()),
    ).await.unwrap_or(false);


    let text: String;
    if user_role {
        text = match args.single::<String>().unwrap_or_default().as_str() {
            "add" => add(args).await,
            "list" => list(args).await,
            "delete" => if args.len() == 2 {
                delete(args).await
            } else { "Expecting: ```~raffle delete <raffle_id>```".to_string() },
            "start" => update_status(args, "running".to_string()).await,
            "stop" => update_status(args, "stopped".to_string()).await,
            "title" => update_title(args).await,
            _ => "Expecting: ```~raffle <add/list/delete/start/stop>```".to_string()
        };
    } else {
        text = "Unauthenticated: User has the wrong role!".to_string();
    }

    typing.stop();
    message_end(&ctx, msg, text).await;
    env::set_var("UPDATE_STATUS", "true");

    Ok(())
}

/*async fn message_end(ctx: &&Context, msg: &Message, mut text: String) {
    if env::var("DELETE_MESSAGE").unwrap_or("false".to_string()).contains("true") {
        text.push_str("\n> this message will destruct in 5s");
    }
    let msg_id = msg.channel_id.say(&ctx.http, text).await?;
    if env::var("DELETE_MESSAGE").unwrap_or("false".to_string()).contains("true") {
        thread::sleep(time::Duration::from_secs(5));
        msg.channel_id.delete_message(&ctx.http, msg_id).await?;
    }
}

async fn message_begin(ctx: &&Context, msg: &Message) {
    if env::var("DELETE_MESSAGE").unwrap_or("false".to_string()).contains("true") {
        msg.delete(&ctx.http).await?;
    }
}*/

async fn add(mut args: Args) -> String {
    if args.len() == 6 {
        let title = args.single::<String>().unwrap();
        let ticket_price = args.single::<f32>().unwrap();
        let ticket_amount = args.single::<u16>().unwrap();
        let ticket_currency = args.single::<String>().unwrap();
        let description = args.single::<String>().unwrap();

        let raffle = Raffle {
            id: Default::default(),
            title,
            description,
            status: "".to_string(),
            ticket_amount,
            ticket_price,
            ticket_token_name: ticket_currency,
            rule: "none".to_string(),
            date_created: 0,
            date_updated: 0,
        };

        let text = api_helper::add_raffle(&raffle).await.unwrap();
        text
    } else { "Expecting: ```~raffle add <title> <ticket_price> <ticket_amount> <ticket_currency> \"<description>\"```".to_string() }
}

pub async fn list(mut args: Args) -> String {
    let raffle_id = args.single::<String>().unwrap_or("0".to_owned());

    let raffles = api_helper::get_raffle(raffle_id).await.unwrap();
    let mut text = String::new();


    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(150);

    ascii_table.column(0).set_header("ID").set_align(Align::Left);
    ascii_table.column(1).set_header("Title").set_align(Align::Center);
    ascii_table.column(2).set_header("Status").set_align(Align::Center);
    ascii_table.column(3).set_header("Amount").set_align(Align::Center);
    ascii_table.column(4).set_header("Price").set_align(Align::Right);
    ascii_table.column(5).set_header("TOKEN").set_align(Align::Left);

    text.push_str(format!("**RaffleList**").as_ref());

    let mut data: Vec<[String; 6]> = Vec::new();

    for raffle in raffles {
        data.push([raffle.id.to_string(), raffle.title, raffle.status, raffle.ticket_amount.to_string(), raffle.ticket_price.to_string(), raffle.ticket_token_name]);
    }
    text.push_str(format!("```{} ``` \n", ascii_table.format(data.clone())).as_ref());

    text
}

pub async fn delete(mut args: Args) -> String {
    let number = args.single::<String>().unwrap_or("0".to_owned());

    let message = api_helper::del_raffle(number).await.unwrap_or("Error in raffle_id".to_string());

    message
}

async fn update_status(mut args: Args, status: String) -> String {
    let id = args.single::<String>().unwrap_or("0".to_string());
    if id.clone() != "0".to_string() {
       let mut raffles = api_helper::get_raffle(id.clone()).await.unwrap();

        raffles[0].status = status;

        format!("Updated: {}", api_helper::update_raffle(id, &raffles[0]).await.unwrap())
    } else {
        "Expecting: ```~raffle <start/stop> <raffle_id>```".to_string()
    }
}

async fn update_title(mut args: Args) -> String {
    let id = args.single::<String>().unwrap();
    let new_title = args.single::<String>().unwrap();

    if id != "0".to_string() || new_title.is_empty() {
        let raffle_id = args.single::<String>().unwrap_or("0".to_owned());
        let mut raffles = api_helper::get_raffle(raffle_id).await.unwrap();

        raffles[0].title = new_title;

        format!("Updated: {}", api_helper::update_raffle(id, &raffles[0]).await.unwrap())
    } else {
        "Expecting: ```~raffle update <raffle_id> <new_title>```".to_string()
    }
}