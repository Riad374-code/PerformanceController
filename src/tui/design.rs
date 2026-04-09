use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    widgets::{Block, BorderType, Borders, List, ListDirection, ListState, Paragraph},
    Terminal,
};
use std::io;
use std::io::Stdout;
use std::rc::Rc;
use crate::tui::chat::{chat_box, ChatState};

pub fn init() -> Terminal<CrosstermBackend<Stdout>> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
}

pub fn frame_divider()-> Rc<[Rect]>{

    let chunks= Layout::default().direction(Direction::Horizontal).constraints([
        Constraint::Percentage(20), Constraint::Percentage(80)].as_ref()).split(Rect::new(0,0,150,40));
    chunks
}


//Box arrangements
pub fn left_box_select<'a>()-> List<'a>{
    let selections= vec!["CPU","GPU","Chat","RAM"];
    let list = List::new(selections)
        .style(Color::White)
        .highlight_style(Style::new().yellow().italic())
        .highlight_symbol("> ".red())
        .scroll_padding(1)
        .direction(ListDirection::TopToBottom)
        .repeat_highlight_symbol(true);

    list
}

//Block Design
pub fn left_box_specs<'a>()-> Block<'a>{
    let left_block= Block::default()
        .title(" Menu ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD));

    left_block

}

pub fn right_box_specs<'a>(choosen : &'a str)-> Block<'a>{
    let right_block= Block::default().title(choosen)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

    right_block
}



pub fn draw_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    chat_state: ChatState,
    state: &mut ListState // Pass by reference!
) -> io::Result<()> {
    terminal.draw(|f| {
        let chunks = frame_divider();

        // Use state.selected() to drive your logic
        let selected_index = state.selected().unwrap_or(0);

        let menu = left_box_select().block(left_box_specs());
        // render_stateful_widget will now correctly update 'state' with scroll offsets
        f.render_stateful_widget(menu, chunks[0], state);

        let selected_title = match selected_index {
            0 => "CPU",
            1 => "GPU",
            2 => "Chat",
            _ => "RAM",
        };
        let right_block= right_box_specs(selected_title);
        if state.selected()==Some(2){
            chat_box(f,chunks[1],&chat_state)
        }else {
            let content = Paragraph::new(format!("{} details go here...", selected_title)).block(right_block);
            f.render_widget(content, chunks[1]);
        }
    })?;

    Ok(())
}
