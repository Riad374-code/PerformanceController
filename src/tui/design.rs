use ratatui::{
    backend::CrosstermBackend, // Connects Ratatui to the Crossterm library
    widgets::{Block, Borders, Paragraph}, // The visual components
    layout::{Layout, Constraint, Direction}, // To split the screen into sections
    Terminal, // The main terminal interface
};
use std::io;
use std::io::Stdout;
use ratatui::prelude::Color;
use ratatui_core::layout::Rect;

pub fn init() -> Terminal<CrosstermBackend<Stdout>> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
}

pub fn allign_screen_size(mut terminal:Terminal<CrosstermBackend<Stdout>>)->Result<(), io::Error>{
    terminal.draw(|mut f| {
        //let area=Rect::new(20,20,20,20);
        let area = f.area();
        let block =Block::default().borders(Borders::ALL).title("System Info");
        f.render_widget(block, area);
    })?;
    Ok(())
}

