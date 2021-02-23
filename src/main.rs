use serde::Deserialize;
use reqwest::Error;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
}



#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("http://localhost:3001/api/v1/token");
    println!("{}", request_url);

    let mut token_body = HashMap::new();
    token_body.insert("grant_type", "client_credentials");
    token_body.insert("client_id", "my_client_id");
    token_body.insert("client_secret", "my_client_secret");

    let client = reqwest::Client::new();
    let response = client.post(&request_url)
        .json(&token_body)
        .send()
        .await?;

    let tr: TokenResponse = response.json().await?;
    println!("{:?}", tr);
    let token = tr.access_token;
    println!("{:?}", token);
    Ok(())
}
