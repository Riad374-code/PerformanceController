mod component_details;
mod logging_init;
mod trainable_model_integ;
mod tui;

use crate::component_details::gpu;
use crate::tui::chat::{ChatState, Role};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::widgets::ListState;
use std::io;
use std::time::Duration;
use tui::chat::{Message, chat_ai};
use tui::{chat, design};

use component_details::gpu::get_gpu_details;
use trainable_model_integ::gpu_detailing::{Errors, get_perandtemp};
//-> std::io::Result<()>

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut terminal = design::init();
    let mut selected_index: usize = 0;
    let mut chat_state = ChatState::default();
    let mut state = ListState::default();
    state.select(Some(0)); // Start with the first item selected
    loop {
        design::draw_terminal(&mut terminal, chat_state.clone(), &mut state)?;

        if event::poll(Duration::from_millis(50))? {
            let ev = event::read()?;

            // If we are on the Chat tab, let the chat handle the keyboard

            // Handle exiting chat mode (e.g., Esc key)

            if let Event::Key(key) = ev {
                if key.kind != KeyEventKind::Press {
                    continue; // Only handle key presses, ignore releases and repeats
                }
                if state.selected() == Some(2) {
                    if key.code == KeyCode::Up {
                        let i = match state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    3
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        state.select(Some(i));
                    } else if key.code == KeyCode::Down {
                        let i = match state.selected() {
                            Some(i) => {
                                if i >= 3 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        state.select(Some(i));
                    } else {
                        // Forward all other key presses (chars, backspace, enter) to chat input logic.
                        chat::chat_event(ev, &mut chat_state).await.ok();
                    }
                } else if state.selected() == Some(1) {
                    if key.code == KeyCode::Up {
                        let i = match state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    3
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        state.select(Some(i));
                    } else if key.code == KeyCode::Down {
                        let i = match state.selected() {
                            Some(i) => {
                                if i >= 3 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        state.select(Some(i));
                    } else {
                        // Forward all other key presses to GPU details logic.
                        get_gpu_details(ev, &mut state).ok();
                    }
                } else {
                    // Standard menu navigation
                    if let Event::Key(key) = ev {
                        match key.code {
                            KeyCode::Up => {
                                let i = match state.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            3
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                state.select(Some(i));
                            }
                            KeyCode::Down => {
                                let i = match state.selected() {
                                    Some(i) => {
                                        if i >= 3 {
                                            0
                                        } else {
                                            i + 1
                                        }
                                    }
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

    disable_raw_mode()?;
    Ok(())
}

/**/
