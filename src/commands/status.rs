use std::env;
use std::sync::Arc;

use ascii_table::{Align, AsciiTable};
use bson::oid::ObjectId;
use indicatif::ProgressBar;
use log::*;
use serde::de::Unexpected::Str;
use serde_json::{json, to_string};
use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::http::{CacheHttp, Http};
use serenity::model::guild::Target::User;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use structopt::StructOpt;

use crate::api_helper;
use crate::commands::{message_begin, message_end};
use crate::model::{Raffle, Ticket};

#[command]
pub async fn status(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    message_begin(&ctx, msg).await;
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    //
    let mut text = match args.single::<String>().unwrap_or_default().as_str() {
        "list" => build_status_message_short().await,
        "perm" => {
            msg.channel_id.say(&ctx.http, build_status_message_short().await).await?;
            "--- permanent message ---".to_string()
        }
        "full" => build_status_message().await,
        _ => "Expecting: ```~status <list/perm/full>```".to_string()
    };
    //
    typing.stop();
    message_end(&ctx, msg, text).await;

    //    text.push_str("> this message will destruct in 5s");
    //let msg_id = msg.channel_id.say(&ctx.http, text).await?;


    Ok(())
}

async fn build_status_message_short() -> String {
    let raffles = api_helper::get_raffle("0".to_owned()).await.unwrap();
    let tickets = api_helper::get_ticket("0".to_owned()).await.unwrap();

    let mut text = String::new();
    text.push_str(format!("**:tickets: Raffle Status :tickets:**\n\n").as_ref());

    for raffle in raffles {
        let mut table: Vec<[String; 3]> = Vec::new();

        let emote_state = match raffle.status.as_str() {
            "created" => ":yellow_circle:".to_string(),
            "running" => ":green_circle:".to_string(),
            "stopped" => ":red_circle:".to_string(),
            _ => "red_circle".to_string()
        };

        text.push_str(format!("{} \t**{}** \t ID={}\n", emote_state, raffle.title, raffle.id).as_ref());
        text.push_str(format!("\t\t  {}\n", raffle.description).as_ref());

        let mut tickets_sold = 0;
        for ticket in tickets.clone() {
            let mut count = 0;
            if ticket.raffle_id == raffle.id {
                tickets_sold += ticket.amount as i32;
            }
        }

        create_progress_view(&mut text, raffle, tickets_sold);
    }
    text.push_str(format!("\n> This message will auto-update").as_ref());
    text
}

fn create_progress_view(text: &mut String, raffle: Raffle, mut tickets_sold: i32) {
    text.push_str(format!("```Tickets SOLD: {}/{} \t", tickets_sold, raffle.ticket_amount).as_ref());
    text.push_str(&*format!("price_per_ticket: {}{}\n", raffle.ticket_price, raffle.ticket_token_name));

    let scale: f32 = 50 as f32 / raffle.ticket_amount as f32;
    text.push_str("[");
    for n in 1..50 {
        if (n < tickets_sold as i32 * scale as i32) {
            text.push_str("#");
        } else { text.push_str(" "); }
    }
    let percentage = tickets_sold as f32 / raffle.ticket_amount as f32 * 100.0;
    text.push_str(format!("] {:.0}% ```\n", percentage).as_str());
}


async fn build_status_message() -> String {
    let raffles = api_helper::get_raffle("0".to_owned()).await.unwrap();
    let tickets = api_helper::get_ticket("0".to_owned()).await.unwrap();

    let mut text = String::new();
    text.push_str(format!("**Raffle Status**\n\n").as_ref());

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(150);

    for raffle in raffles {
        let mut table: Vec<[String; 3]> = Vec::new();

        let emote_state = match raffle.status.as_str() {
            "created" => ":yellow_circle:".to_string(),
            "running" => ":green_circle:".to_string(),
            "stopped" => ":red_circle:".to_string(),
            _ => "red_circle".to_string()
        };

        text.push_str(format!("**{}** - {} {}\n", raffle.title, raffle.id, emote_state).as_ref());
        text.push_str(format!("{}\n", raffle.description).as_ref());

        let mut tickets_sold = 0;
        for ticket in tickets.clone() {
            let mut count = 0;
            if ticket.raffle_id == raffle.id {
                tickets_sold += ticket.amount as i32;
            }
        }


        ascii_table.column(0).set_header("PlayerName").set_align(Align::Left);
        ascii_table.column(1).set_header("TicketAmount").set_align(Align::Center);
        ascii_table.column(2).set_header("TimePrinted").set_align(Align::Center);


        for ticket in tickets.clone() {
            if ticket.raffle_id == raffle.id {
                let dt = chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(ticket.date_created as i64, 0), chrono::Utc);

                table.push([
                    ticket.username,
                    ticket.amount.to_string(),
                    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()]);
            }
        }
        create_progress_view(&mut text, raffle, tickets_sold);

        text.push_str(format!("\n\
        ```{}``` \n", ascii_table.format(table.clone())).as_ref());
    }
    text
}


pub async fn change_status_message(ctx: &Arc<Http>, guilds: &Vec<GuildId>, msg_id: &mut u64)
{
    let channel_id = env::var("CHANNEL_ID").unwrap_or("0".to_string()).parse::<u64>().unwrap();

    if msg_id.clone() != 0 as u64 {
        let msg = ctx.get_message(channel_id, msg_id.clone()).await;
        match msg {
            Ok(_) => msg.unwrap().delete(ctx.http()).await.unwrap(),
            Err(e) => error!("{}",e)
        };
    }

    let message_text = build_status_message_short().await;


    let mut message = ChannelId(channel_id)
        .send_message(&ctx, |m| {
            m.embed(|e| e
                .colour(0x00ffBB)
                .description(message_text)
            )
        }).await;

    *msg_id = message.unwrap().id.as_u64().clone();
}