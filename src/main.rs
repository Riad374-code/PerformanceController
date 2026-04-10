mod logging_init;
mod component_details;
mod tui;
mod trainable_model_integ;

use tui::chat::{Message,chat_ai};
use std::io;
use std::time::Duration;
use tui::{design,chat};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::widgets::ListState;
use crate::component_details::gpu;
use crate::tui::chat::{ChatState, Role};

use trainable_model_integ::gpu_detailing::{get_perandtemp, Errors};
//-> std::io::Result<()>

#[tokio::main]
async fn main() -> io::Result<()> {
    if let Err(e) = get_perandtemp() {
        return Err(io::Error::other(format!("get_perandtemp failed: {:?}", e)));
    }
    Ok(())
}

/*enable_raw_mode()?;

    let mut terminal = design::init();
    let mut selected_index: usize =0;
    let mut chat_state=ChatState::default();
    let mut state = ListState::default();
    loop {
        design::draw_terminal(&mut terminal,chat_state.clone(),&mut state)?;

        if event::poll(Duration::from_millis(50))? {
            let ev = event::read()?;

            // If we are on the Chat tab, let the chat handle the keyboard

                // Handle exiting chat mode (e.g., Esc key)

            if let Event::Key(key) = ev {
                if state.selected() == Some(2) {
                    if key.code == KeyCode::Up {
                        let i = match state.selected() {
                            Some(i) => if i == 0 { 3 } else { i - 1 },
                            None => 0,
                        };
                        state.select(Some(i));
                    } else if key.code == KeyCode::Down {
                        let i = match state.selected() {
                            Some(i) => if i >= 3 { 0 } else { i + 1 },
                            None => 0,
                        };
                        state.select(Some(i));
                    } else if key.code == KeyCode::Enter {
                        if key.code == KeyCode::Esc {
                            state.select(Some(0)); // Go back to CPU
                            continue;
                        } else if key.code == KeyCode::Char('q') {
                            break;
                        }
                        // Otherwise, send the event to your chat logic
                        // Note: chat_event is async, so we await it here
                        chat::chat_event(ev, &mut chat_state).await.ok();
                    }
                }else {
                        // Standard menu navigation
                    if let Event::Key(key) = ev {
                        match key.code {
                            KeyCode::Up => {
                                let i = match state.selected() {
                                    Some(i) => if i == 0 { 3 } else { i - 1 },
                                    None => 0,
                                };
                                state.select(Some(i));
                            }
                            KeyCode::Down => {
                                let i = match state.selected() {
                                    Some(i) => if i >= 3 { 0 } else { i + 1 },
                                    None => 0,
                                };
                                state.select(Some(i));
                            }
                            KeyCode::Char('q') => break,
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    disable_raw_mode()?;*/