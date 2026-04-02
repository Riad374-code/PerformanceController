mod logging_init;
mod component_details;
mod tui;

use std::io;
use std::time::Duration;
use tui::design;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use nvml_wrapper::enum_wrappers::device::Brand;
use logging_init::init_logging;
use component_details::gpu;
use crate::tui::design::draw_terminal;

//-> std::io::Result<()>
fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut terminal = design::init();
    let mut selected_index: usize =0;

    loop {
        design::draw_terminal(&mut terminal, selected_index)?;

        if event::poll(Duration::from_millis(150))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        if selected_index >0 {
                            selected_index -=1;
                        }
                    }
                    KeyCode::Down => {
                        selected_index = (selected_index +1).min(2);
                    }
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}