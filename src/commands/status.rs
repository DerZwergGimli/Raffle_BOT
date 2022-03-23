use ascii_table::{Align, AsciiTable};
use serde_json::to_string;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::api_helper;
use crate::model::{Raffle, Ticket};
use structopt::StructOpt;
use log::*;
use serde::de::Unexpected::Str;
use serenity::http::CacheHttp;

use bson::oid::ObjectId;
use serenity::model::guild::Target::User;
use indicatif::ProgressBar;

#[command]
pub async fn status(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let raffles = api_helper::get_raffle("0".to_owned()).await.unwrap();
    let tickets = api_helper::get_ticket("0".to_owned()).await.unwrap();


    let mut text = String::new();
    text.push_str(format!("**Raffle Status**\n\n").as_ref());

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(150);

    for raffle in raffles {
        let mut table: Vec<[String; 3]> = Vec::new();

        text.push_str(format!("**{}** - {}\n", raffle.title, raffle.id).as_ref());
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
        text.push_str(format!("```Tickets SOLD: {}/{} \n", tickets_sold, raffle.ticket_amount).as_ref());

        let scale:f32 = 50 as f32/ raffle.ticket_amount as f32;
        text.push_str("[");
        for n in 1..50{
            if(n < tickets_sold as i32 * scale as i32){
                text.push_str("#");
            }
            else { text.push_str(" "); }
        }
        text.push_str("]\n");

        text.push_str(format!("{} ``` \n", ascii_table.format(table.clone())).as_ref());
    }

    //    text.push_str("> this message will destruct in 5s");
    let msg_id = msg.channel_id.say(&ctx.http, text).await?;

    Ok(())
}


