mod logging_init;
mod component_details;
mod tui;

use tui::chat::{Message,chat_ai};
use std::io;
use std::time::Duration;
use tui::{design,chat};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::tui::chat::{ChatState, Role};
//-> std::io::Result<()>

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut terminal = design::init();
    let mut selected_index: usize =0;
    let mut chat_state=ChatState::default();

    loop {
        design::draw_terminal(&mut terminal, selected_index,chat_state.clone())?;

        if event::poll(Duration::from_millis(50))? {
            let ev = event::read()?;

            // If we are on the Chat tab, let the chat handle the keyboard
            if selected_index == 2 {
                // Handle exiting chat mode (e.g., Esc key)
                if let Event::Key(key) = ev {
                    if key.code == KeyCode::Esc {
                        selected_index = 0; // Go back to CPU
                        continue;
                    }
                }

                // Otherwise, send the event to your chat logic
                // Note: chat_event is async, so we await it here
                chat::chat_event(ev, &mut chat_state).await.ok();
            } else {
                // Standard menu navigation
                if let Event::Key(key) = ev {
                    match key.code {
                        KeyCode::Up => {
                            if selected_index>0{
                                selected_index-=1;
                            } else { selected_index=3; }
                        }
                        KeyCode::Down => { if selected_index==3{
                            selected_index=0;
                        }else { selected_index+=1; }
                        }
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }


    Ok(())
}