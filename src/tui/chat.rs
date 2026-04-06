use std::env;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

#[derive(Serialize, Deserialize, Debug,Clone)]
pub enum Role {
    User,
    AI,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Message {
    pub message: String,
    // can be used validator, Only User or AI
    pub role: Role,
}


#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    choices: Vec<AssistantMessage>,
}

#[derive(Deserialize, Debug,Clone)]
pub struct AssistantMessage {
    pub model: String,
    pub created_at: String,
    pub message:Message,
    pub done: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ChatState {
    pub input: String,
    pub history: Vec<Message>,
    pub sending: bool,
    pub error: Option<String>,
}

pub async fn chat_ai(
    message: Message,
    history: Vec<Message>,
) -> Result<(String, Vec<Message>), Box<dyn std::error::Error>> {
    dotenv().ok();

    let base_url = "http://127.0.0.1:11434";
    let api_key = env::var("AI_API_KEY")?;
    let model = env::var("model").unwrap_or_else(|_| "gemini-3-flash".to_string());

    let client = Client::new();

    let mut messages = history;
    messages.push(message);

    println!("Requesting...");

    let res = client
        .post(format!("{}/api/chat", base_url))
        .json(&serde_json::json!({
            "model": model,
            "messages": messages,
        }))
        .send()
        .await?;

    println!("Request completed");

    let status = res.status();
    let body = res.text().await?;
    println!("Status: {}", status);
    println!("Response: {}", body);

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
        message: response.message.clone(),
        role: Role::AI,
    });

    Ok((response.message, messages))
}

pub fn chat_box(frame: &mut Frame, area: Rect, state: &ChatState) {

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);


    let chat_hist= state.history.iter().map(|m|{
        let who= match m.role{
            Role::AI => "AI",
            Role::User => "You",
        };
        format!("{}: {}",who,m.message)
    }).collect::<Vec<String>>().join("\n");

    let chat_view = Paragraph::new(chat_hist).block(Block::default().borders(Borders::ALL)).wrap(Wrap { trim: true });

    let mut input="Sending";
    if state.sending {
        input="Input (sending...)";
    }else{
        input = "Input (not sending...)";
    }

    let input_view=Paragraph::new(input).block(Block::default().borders(Borders::ALL));
    frame.render_widget(chat_view, chunks[0]);
    frame.render_widget(input_view, chunks[1]);

}

pub async fn chat_event(
    event: Event,
    state: &mut ChatState,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Event::Key(key) = event {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }

        match key.code {
            KeyCode::Char(c) => state.input.push(c),
            KeyCode::Backspace => {
                state.input.pop();
            }
            KeyCode::Enter => {
                let text = state.input.trim().to_string();
                if text.is_empty() {
                    return Ok(());
                }

                state.sending = true;
                state.error = None;

                match chat_ai(Message{message:text,role:Role::User},state.history.clone()).await {
                    Ok((.., history)) => {
                        state.history= history;
                        state.input.clear()},
                    Err(e) => state.error = Some(e.to_string()),
                }

                state.sending = false;
            }
            _ => {}
        }
    }

    Ok(())
}