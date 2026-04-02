use std::env;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    User,
    AI,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message: String,
    // can be used validator, Only User or AI
    pub role: Role,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Choice {
    pub message: String,
}

pub async fn chat_ai(
    message: Message,
    history: Vec<Message>,
) -> Result<(String, Vec<Message>), Box<dyn std::error::Error>> {
    dotenv().ok();

    let base_url = "http://127.0.0.1:8045/v1";
    let api_key = env::var("AI_API_KEY")?;
    let model = env::var("model").unwrap_or_else(|_| "gemini-3-flash".to_string());

    let client = Client::new();

    let mut messages = history;
    messages.push(message);

    let res = client
        .post(format!("{}/chat/completions", base_url))
        .bearer_auth(api_key)
        .json(&serde_json::json!({
            "model": model,
            "messages": messages,
        }))
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;

    if !status.is_success() {
        return Err(format!("API error {}: {}", status, body).into());
    }

    let request: ChatResponse = serde_json::from_str(&body)?;

    let response = request
        .choices
        .first()
        .map(|choice| choice.message.clone())
        .ok_or("Not answered")?;

    messages.push(Message {
        message: response.clone(),
        role: Role::AI,
    });

    Ok((response, messages))
}