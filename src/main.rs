use std::env;

use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json::{Value};
use reqwest::Error as RError;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<(), RError>  {
    let (event, _context) = event.into_parts();
    let query = event["query"].as_str().unwrap_or("福岡市中央区");
    let searched_name = search(&query).await?;

    println!("{}", format!("query: {}, searched_name: {:?}", query, searched_name));
    Ok(())
}

async fn search(query: &str) -> Result<String, RError> {
    let api_key = env::var("GEO_CODE_API_KEY").unwrap();
    let url = format!("https://map.yahooapis.jp/geocode/V1/geoCoder?appid={}&output=json", &api_key);
    let body = reqwest::get(format!("{}&query={}", url, query)).await?.text().await?;
    let v: Value = serde_json::from_str(&body).unwrap();
    
    Ok(v["Feature"][0]["Name"].to_string())
}
