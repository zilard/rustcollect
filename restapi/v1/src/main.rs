use reqwest::Error;
use serde::Deserialize;

use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Data {
    error: Vec<i32>,
    result: HashMap<String, Ticker>,
}

#[derive(Deserialize, Debug)]
struct Ticker {
    XXBTZUSD: HashMap<String, a>,
}

#[derive(Deserialize, Debug)]
struct a {
    a: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://api.kraken.com/0/public/Ticker?pair={pair}",
        pair = "xbtusd"
    );

    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    //let users: Vec<User> = response.json().await?;
    //println!("{:?}", users);

    let result: HashMap<String, Data> = response.json().await?;

    println!("{:?}", result);

    Ok(())
}

/*
fn main() {

    query_api();

}
*/
