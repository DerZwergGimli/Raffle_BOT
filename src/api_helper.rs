use std::env;
use reqwest;
use reqwest::{Client, Error};
use serde_json::json;
use tracing::log::info;
use crate::model;
use crate::model::{Raffle, Ticket};

fn build_client() -> Client {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    client
}

//region add
pub async fn add_raffle(raffle: &Raffle) -> Result<String, Error> {
    let client = build_client();

    let mut baseurl = env::var("RAFFLE_API_URL").unwrap().to_owned();
    let url = format!("{}/raffle", baseurl);
    info!("info={}", url.clone());

    let mut result = client
        .post(&url)
        .bearer_auth(env::var("RAFFLE_API_KEY").unwrap())
        .json(raffle)
        .send()
        .await?;
    //info!("result={}", result.status());
    let mut text = result.text().await?;
    info!("text={:?}", text);
    Ok(text)

}

pub async fn add_ticket(ticket: &Ticket) -> Result<String, Error> {
    let client = build_client();

    let mut baseurl = env::var("RAFFLE_API_URL").unwrap().to_owned();
    let url = format!("{}/ticket", baseurl);
    info!("info={}", url.clone());

    let mut result = client
        .post(&url)
        .bearer_auth(env::var("RAFFLE_API_KEY").unwrap())
        .json(ticket)
        .send()
        .await?;
    //info!("result={}", result.status());
    let mut text = result.text().await?;
    info!("text={:?}", text);
    Ok(text)

}
//endregion


//region get
pub async fn get_raffle(id : String) -> Result<Vec<model::Raffle>, Error> {
    let client = build_client();

    let mut baseurl = env::var("RAFFLE_API_URL").unwrap().to_owned();
    let url = format!("{}/raffle/{}", baseurl, id);
    info!("info={}", url.clone());
    let mut result = client
        .get(&url)
        .bearer_auth(env::var("RAFFLE_API_KEY").unwrap())
        .send()
        .await?;
    //info!("result={}", result.status());
    let mut text = result.text().await?;
    info!("text={:?}", text);
    let data = serde_json::from_str(text.as_str());
    Ok(data.unwrap())

}

pub async fn get_ticket(id : String) -> Result<Vec<model::Ticket>, Error> {
    let client = build_client();

    let mut baseurl = env::var("RAFFLE_API_URL").unwrap().to_owned();
    let url = format!("{}/ticket/{}", baseurl, id);
    info!("info={}", url.clone());
    let mut result = client
        .get(&url)
        .bearer_auth(env::var("RAFFLE_API_KEY").unwrap())
        .send()
        .await?;
    //info!("result={}", result.status());
    let mut text = result.text().await?;
    info!("text={:?}", text);
    let data = serde_json::from_str(text.as_str());
    Ok(data.unwrap())

}
//endregion


//region del
pub async fn del_raffle(id : String) -> Result<String, Error> {
    let client = build_client();

    let mut baseurl = env::var("RAFFLE_API_URL").unwrap().to_owned();
    let url = format!("{}/raffle/{}", baseurl, id);
    info!("info={}", url.clone());
    let mut result = client
        .delete(&url)
        .bearer_auth(env::var("RAFFLE_API_KEY").unwrap())
        .send()
        .await?;
    //info!("result={}", result.status());
    let mut text = result.text().await?;
    info!("text={:?}", text);
    Ok(text)

}
//endregion


//region update
pub async fn update_raffle(id : String, raffle: &Raffle) -> Result<String, Error> {
    let client = build_client();

    let mut baseurl = env::var("RAFFLE_API_URL").unwrap().to_owned();
    let url = format!("{}/raffle/{}", baseurl, id);
    info!("info={}", url.clone());

    let mut result = client
        .patch(&url)
        .bearer_auth(env::var("RAFFLE_API_KEY").unwrap())
        .json(raffle)
        .send()
        .await?;

    let mut text = result.text().await?;
    info!("text={:?}", text);
    Ok(text)
}

pub async fn update_ticket(ticket: &Ticket) -> Result<String, Error> {
    Ok("NOT-IMPLEMENTED".to_string())

}
//endregion