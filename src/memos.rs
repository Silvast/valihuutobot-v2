use rss::{Channel, Item};
use std::error::Error;
use reqwest;
use urlencoding::encode;
use serde_json::Value;
use regex::Regex;


#[derive(Debug)]
pub struct Memo
{
    title: String,
    link: String,
    description: String,
    content: String
}

pub fn find_text_between_brackets(input: &str) -> Vec<String> {
    let re = Regex::new(r"\[([^\[\]]+)\]").unwrap();
    let mut results = Vec::new();

    for cap in re.captures_iter(input) {
        if let Some(matched) = cap.get(1) {
            results.push(matched.as_str().to_string());
        }
    }
    results
}

pub async fn fetch_memo(url: &str) -> String {
    let response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap();
    body
}

pub async fn get_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://www.eduskunta.fi/_layouts/15/feed.aspx?xsl=1&web=%2FFI%2Frss%2Dfeeds&page=8192fae7-172f-46ba-8605-75c1e750007a&wp=3527e156-7a72-443c-8698-9b5596317471&pageurl=%2FFI%2Frss%2Dfeeds%2FSivut%2FTaysistuntojen%2Dpoytakirjat%2DRSS%2Easpx")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub async fn edit_memo(item: &Item) -> Result<(), Box<dyn Error>> {
    let prefix = "https://avoindata.eduskunta.fi/api/v1/tables/VaskiData/rows?perPage=10&page=0&columnName=Eduskuntatunnus&columnValue=";
    let suffix = encode(item.title().unwrap_or_default()).to_string();

    let url = format!("{}{}", prefix, suffix);
    // println!("URL: {}", url);

    let memo = Memo {
        title: item.title().unwrap_or_default().to_string(),
        link: item.link().unwrap_or_default().to_string(),
        description: item.description().unwrap_or_default().to_string(),
        content: String::from(fetch_memo(&url).await),
    };

    let json_data: Value = match serde_json::from_str(&memo.content) {
        Ok(data) => data,
        Err(err) => {
            println!("Error: {}", err);
            return Err(Box::new(err));
        }
    };
    let mut shout_data: Vec<String> = Vec::<String>::new();

    if let Some(rows) = json_data["rowData"].as_array() {
        for row in rows {
          
            let shouts: Vec<String> = find_text_between_brackets(&String::from(row[1].as_str().unwrap()));
            shout_data.extend(shouts.into_iter().filter(|x| x != "Puhemies koputtaa").collect::<Vec<String>>());

        }
    }

    println!("Shouts: {:?}", shout_data);
    // send to sqs queue 1. Memo data 2. shout_data
Ok(())
}
    