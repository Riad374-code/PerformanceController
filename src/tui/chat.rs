use crossterm::event::{Event, KeyCode, KeyEventKind};
use dotenvy::dotenv;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    User,
    AI,
}

impl Role {
    fn as_ollama_role(&self) -> &'static str {
        match self {
            Role::User => "user",
            Role::AI => "assistant",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub content: String,
    // can be used validator, Only User or AI
    pub role: Role,
}

impl Message {
    fn take_role(role: &Role) -> &'static str {
        role.as_ollama_role()
    }
}

#[derive(Serialize, Debug)]
pub struct OllamaChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: Option<Message>,
    pub done: bool,
    pub error: Option<String>,
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

    let base_url =
        env::var("OLLAMA_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".to_string());
    let model = env::var("OLLAMA_MODEL")
        .or_else(|_| env::var("model"))
        .unwrap_or_else(|_| "llama3.2".to_string());

    let client = Client::new();

    let mut messages = history;
    messages.push(message);

    let ollama_messages = messages
        .iter()
        .map(|m| Message {
            role: m.role.as_ollama_role().to_string(),
            content: m.content.clone(),
        })
        .collect::<Vec<Message>>();

    let payload = OllamaChatRequest {
        model,
        messages: ollama_messages,
        stream: false,
    };

    let res = client
        .post(format!("{}/api/chat", base_url))
        .json(&payload)
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;

    if !status.is_success() {
        return Err(format!("API error {}: {}", status, body).into());
    }

    let request: ChatResponse = serde_json::from_str(&body)?;

    if let Some(err) = request.error {
        return Err(err.into());
    }

    let response = request
        .message
        .map(|msg| msg.content)
        .ok_or("Not answered")?;

    messages.push(Message {
        content: response.clone(),
        role: self::take_role(&Role::AI).to_string(),
    });

    Ok((response, messages))
}

pub fn chat_box(frame: &mut Frame, area: Rect, state: &ChatState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(area);

    let chat_hist = state
        .history
        .iter()
        .map(|m| {
            let who = match m.role {
                Role::AI => "AI",
                Role::User => "You",
            };
            format!("{}: {}", who, m.content)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let chat_view = Paragraph::new(chat_hist)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    let input_title = if state.sending {
        "Input (sending...)"
    } else {
        "Input"
    };

    let input_text = if state.input.is_empty() {
        "Type your message and press Enter".to_string()
    } else {
        state.input.clone()
    };

    let input_view =
        Paragraph::new(input_text).block(Block::default().borders(Borders::ALL).title(input_title));
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

                match chat_ai(
                    Message {
                        content: text,
                        role: Role::User,
                    },
                    state.history.clone(),
                )
                .await
                {
                    Ok((.., history)) => {
                        state.history = history;
                        state.input.clear()
                    }
                    Err(e) => state.error = Some(e.to_string()),
                }

                state.sending = false;
            }
            _ => {}
        }
    }

    Ok(())
}
