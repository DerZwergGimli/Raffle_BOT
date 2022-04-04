use std::{env};
use ascii_table::{Align, AsciiTable};
use bson::oid::ObjectId;
use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;
use crate::api_helper;
use crate::commands::message_end;
use crate::model::{Ticket};

#[command]
pub async fn ticket(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    //msg.delete(&ctx.http).await?;
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    let username = msg.author.nick_in(ctx, msg.guild_id.unwrap()).await.unwrap();

    let text = match args.single::<String>().unwrap_or_default().as_str() {
        "me" => me(username).await,
        "add" => if args.len() == 3 {
            add(args, username).await
        } else { "Expecting: ```~ticket add <raffle_id> <spl_tx_signature>```".to_string() }
        "list" => list().await,
        "delete" => if args.len() == 2 {
            "NOT-Implemented".to_string()
        } else { "Expecting: ```~ticket delete <raffle_id>```".to_string() },
        _ => "Expecting: ```~ticket <me/add/list/delete>```".to_string()
    };

    typing.stop();
    message_end(&ctx, msg, text).await;
    env::set_var("UPDATE_STATUS", "true");

    Ok(())
}


pub async fn add(mut args: Args, username: String) -> String {
    let raffle_id = args.single::<String>().unwrap();
    let spl_address = args.single::<String>().unwrap();

    let ticket = Ticket {
        id: Default::default(),
        raffle_id: ObjectId::parse_str(raffle_id.as_str()).unwrap_or_default(),
        username: username.to_owned(),
        spl_tx_signature: spl_address,
        amount: 0,
        date_created: 0,
        date_updated: 0,
    };

    let text = api_helper::add_ticket(&ticket).await.unwrap();
    text
}

pub async fn me(username: String) -> String {
    let raffles = api_helper::get_raffle("0".to_owned()).await.unwrap();
    let tickets = api_helper::get_ticket("0".to_owned()).await.unwrap();


    let mut text = String::new();
    text.push_str(format!(":tickets: **Ticket LIST for User {} **\n\n", username.clone()).as_ref());
    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(150);


    for raffle in raffles {
        let mut data: Vec<[String; 4]> = Vec::new();

        ascii_table.column(0).set_header("Ticked_ID").set_align(Align::Left);
        ascii_table.column(1).set_header("PlayerName").set_align(Align::Center);
        ascii_table.column(2).set_header("TicketAmount").set_align(Align::Center);
        ascii_table.column(3).set_header("Created [UTC]").set_align(Align::Right);

        for ticket in tickets.clone() {
            if ticket.raffle_id == raffle.id && ticket.username.contains(&username.clone()) {
                data.push([
                    ticket.id.to_string(),
                    ticket.username,
                    ticket.amount.to_string(),
                    ticket.date_created.to_string()]);
            }
        }
        if!data.is_empty(){
            text.push_str(format!("**{}** - {}\n", raffle.title, raffle.id).as_ref());
            text.push_str(format!("{}\n", raffle.description).as_ref());
            text.push_str(format!("```{} ``` \n", ascii_table.format(data.clone())).as_ref());
        }
    }

    text
}

pub async fn list() -> String {
    let raffles = api_helper::get_raffle("0".to_owned()).await.unwrap();
    let tickets = api_helper::get_ticket("0".to_owned()).await.unwrap();


    let mut text = String::new();
    text.push_str(format!("**Ticket LIST **\n\n").as_ref());

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(150);


    for raffle in raffles {
        let mut data: Vec<[String; 4]> = Vec::new();

        text.push_str(format!("**{}** - {}\n", raffle.title, raffle.id).as_ref());
        text.push_str(format!("{}\n", raffle.description).as_ref());

        ascii_table.column(0).set_header("Ticked_ID").set_align(Align::Left);
        ascii_table.column(1).set_header("PlayerName").set_align(Align::Center);
        ascii_table.column(2).set_header("TicketAmount").set_align(Align::Center);
        ascii_table.column(3).set_header("Created [UTC]").set_align(Align::Right);

        for ticket in tickets.clone() {
            if ticket.raffle_id == raffle.id {
                data.push([
                    ticket.id.to_string(),
                    ticket.username,
                    ticket.amount.to_string(),
                    ticket.date_created.to_string()]);
            }
        }
        text.push_str(format!("```{} ``` \n", ascii_table.format(data.clone())).as_ref());
    }

    text
}


#[command]
pub async fn delete(ctx: &Context, msg: &Message) -> CommandResult {
    let raffles = api_helper::get_raffle("0".to_owned()).await?;
    let tickets = api_helper::get_ticket("0".to_owned()).await?;


    let mut text = String::new();
    text.push_str(format!("**Ticket LIST **\n\n").as_ref());

    for raffle in raffles {
        text.push_str(format!("Raffle: ** {} **\n", raffle.title).as_ref());
        text.push_str(format!("> PlayerName \t TicketAmount\n").as_ref());
        for ticket in tickets.clone() {
            if ticket.raffle_id == raffle.id {
                text.push_str(format!("{} \t\t\t {}\n",
                                      ticket.username,
                                      ticket.amount).as_ref());
            }
        }
    }

    msg.channel_id.say(&ctx.http, text).await?;
    Ok(())
}





