use std::fs::File;
use std::io::BufReader;
use rss::{Channel, Item};
use std::error::Error;
use reqwest;
use urlencoding::encode;
mod text_handling;


#[derive(Debug)]
struct Memo
{
    title: String,
    link: String,
    description: String,
    content: String
}

async fn fetch_memo(url: &str) -> String {
    let response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap();
    body
}

async fn get_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://www.eduskunta.fi/_layouts/15/feed.aspx?xsl=1&web=%2FFI%2Frss%2Dfeeds&page=8192fae7-172f-46ba-8605-75c1e750007a&wp=3527e156-7a72-443c-8698-9b5596317471&pageurl=%2FFI%2Frss%2Dfeeds%2FSivut%2FTaysistuntojen%2Dpoytakirjat%2DRSS%2Easpx")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

async fn edit_memo(item: &Item) {
    let prefix = String::from("https://avoindata.eduskunta.fi/api/v1/tables/VaskiData/rows?perPage=10&page=0&columnName=Eduskuntatunnus&columnValue=");
    let suffix = encode(item.title().unwrap()).to_string();
    let url = format!("{prefix}{suffix}");
    let memo = Memo {
        title: item.title().unwrap().to_string(),
        link: item.link().unwrap().to_string(),
        description: item.description().unwrap().to_string(),
        content: String::from(fetch_memo(&url).await)
    };
    println!("Memo: {:?}", memo);
    let content = text_handling::Content::new(&memo.content);
    let shouts: Vec<String> = text_handling::get_shouts(&content);
    println!("Shouts: {:?}", shouts);
    //try post_shouts_bluesky(&shouts).await;
}

#[tokio::main]
async fn main() {
    match get_feed().await {
        Ok(channel) => {
            edit_memo(channel.items().get(0).unwrap()).await;
        }
        Err(e) => {
            println!("Error: {}", e);
        }

    }
}
