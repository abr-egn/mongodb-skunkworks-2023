use std::error::Error;

use serde::{Serialize, Deserialize};

mod completion {
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct Request {
        pub model: &'static str,
        pub prompt: Option<String>,
        pub temperature: Option<f32>,
        pub max_tokens: Option<u32>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Response {
        pub choices: Vec<Choice>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Choice {
        pub text: String,
        pub index: usize,
        pub logprobs: serde_json::Value,
        pub finish_reason: String,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let client = reqwest::Client::new();

    let request = completion::Request {
        model: "text-davinci-003",
        prompt: Some("Say this is a test".to_string()),
        temperature: Some(0.0),
        max_tokens: Some(7),
    };
    let response = client.post("https://api.openai.com/v1/completions")
        .bearer_auth(&api_key)
        .json(&request)
        .send()
        .await?
        .json::<completion::Response>()
        .await?;
    println!("{:#?}", response);

    Ok(())
}
