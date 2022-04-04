use std::{fs};
use std::io::Write;
use bson::oid::ObjectId;
use log::*;
use serenity::framework::standard::{Args, CommandResult, macros::command};
use serenity::model::prelude::*;
use serenity::prelude::*;


use tokio::fs::File;

use crate::api_helper;
use crate::commands::{message_end};


#[command]
pub async fn export(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;
    let raffle_id = args.single::<String>().unwrap_or_default();
    let mut text = "".to_string();
    let file_name = format!("raffle_{}.export.txt", raffle_id).to_string();

    let raffle_oid = ObjectId::parse_str(raffle_id.as_str()).unwrap_or_default();
    match ObjectId::parse_str(raffle_id.as_str()) {
        Ok(_) => {
            info!("Starting outputting file");
            let mut file_ref = std::fs::File::create(file_name.clone()).expect("create failed");

            let tickets = api_helper::get_ticket("0".to_owned()).await.unwrap();

            for ticket in tickets {
                if ticket.raffle_id == raffle_oid {
                    for n in 0..ticket.amount {
                        file_ref.write_all(format!("{}\n", ticket.username).as_bytes()).expect("write failed");
                    }
                }
            }
            info!("Text written into file successfully");
            let f1 = File::open(file_name.clone()).await?;
            let file = vec![(&f1, file_name.as_ref())];
            msg.channel_id.send_files(&ctx.http, file, |m| m.content("raffle_export")).await;

            fs::remove_file(file_name.clone()).expect("Error removing file");

        }
        Err(_e) => text = "Expecting: ```~export <raffle_id>```".to_string()
    };


    typing.stop();
    message_end(&ctx, msg, text).await;

    Ok(())
}
