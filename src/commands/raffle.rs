use serenity::framework::standard::{macros::command, Args, CommandResult, CommandError};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::api_helper;
use crate::model::Raffle;
use structopt::StructOpt;
use std::error::Error;
use Result;
use std::{thread, time};
use ascii_table::{Align, AsciiTable};
use serde_json::to_string;

#[command]
pub async fn raffle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    //msg.delete(&ctx.http).await?;

    let mut text = match args.single::<String>().unwrap_or_default().as_str() {
        "add" => add(ctx, msg, args).await,
        "list" => list(ctx, msg, args).await,
        "delete" => if args.len() == 2 {
            delete(ctx, msg, args).await
        } else { "Expecting: ```~raffle delete <raffle_id>```".to_string() },
        "start" => update_status(ctx, msg, args, "running".to_string()).await,
        "stop" => update_status(ctx, msg, args, "stopped".to_string()).await,
        "title" => update_title(ctx, msg, args).await,
        _ => "Expecting: ```~raffle <add/list/delete>```".to_string()
    };

    text.push_str("\n> this message will destruct in 5s");
    let msg_id = msg.channel_id.say(&ctx.http, text).await?;
    //Wait until message delete
    //thread::sleep(time::Duration::from_secs(5));
    //msg.channel_id.delete_message(&ctx.http, msg_id).await?;

    Ok(())
}

async fn add(ctx: &Context, msg: &Message, mut args: Args) -> String {

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

pub async fn list(ctx: &Context, msg: &Message, mut args: Args) -> String {
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

pub async fn list_old(ctx: &Context, msg: &Message, mut args: Args) -> String {
    let number = args.single::<String>().unwrap_or("0".to_owned());

    let raffles = api_helper::get_raffle(number).await.unwrap();
    let mut text = String::new();
    for raffle in raffles {
        text.push_str(format!("**{}** \n", raffle.title).as_ref());
        text.push_str(format!("> ID: **{}** \n", raffle.id.to_string()).as_ref());
        text.push_str(format!("> Status: {} \n", raffle.status).as_ref());
        text.push_str(format!("> Description: {:?} \n", raffle.description).as_ref());
        text.push_str(format!("> Ticket-Amount: {:?}\n", raffle.ticket_amount).as_ref());
        text.push_str(format!("> Ticket-Price: {:?}\n", raffle.ticket_price).as_ref());
        text.push_str(format!("> Accepted Token: {}\n", raffle.ticket_token_name).as_ref());
        text.push_str(format!("> Rule: {}\n", raffle.rule).as_ref());
        text.push_str(format!("\n").as_ref());
    }

    text
}

pub async fn delete(ctx: &Context, msg: &Message, mut args: Args) -> String {
    let number = args.single::<String>().unwrap_or("0".to_owned());

    let message = api_helper::del_raffle(number).await.unwrap_or("Error in raffle_id".to_string());

    message
}

async fn update_status(ctx: &Context, msg: &Message, mut args: Args, status: String) -> String {

    let id = args.single::<String>().unwrap();

    if id != "0".to_string() {
        let raffle_id = args.single::<String>().unwrap_or("0".to_owned());
        let mut raffles = api_helper::get_raffle(raffle_id).await.unwrap();

        raffles[0].status = status;

        format!("Updated: {}", api_helper::update_raffle(id, &raffles[0]).await.unwrap())
    }
    else {
        "Select one raffle.".to_string()
    }
}

async fn update_title(ctx: &Context, msg: &Message, mut args: Args) -> String {
    let id = args.single::<String>().unwrap();
    let new_title = args.single::<String>().unwrap();

    if id != "0".to_string() || new_title.is_empty() {
        let raffle_id = args.single::<String>().unwrap_or("0".to_owned());
        let mut raffles = api_helper::get_raffle(raffle_id).await.unwrap();

        raffles[0].title = new_title;

        format!("Updated: {}", api_helper::update_raffle(id, &raffles[0]).await.unwrap())
    }
    else {
        "Select one raffle AND add a title.".to_string()
    }
}