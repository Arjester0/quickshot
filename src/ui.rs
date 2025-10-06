use ratatui::{
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, InputMode};

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);
    let [help_area, input_area, list_area] = vertical.areas(frame.area());

    // --- Help Message ---
    let (msg, style) = match app.input_mode {
        InputMode::Tabbing => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to exit, ".into(),
                "e".bold(),
                " to enter text input mode, ".into(),
                "Enter".bold(),
                " to select project session".into(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                "Type to search projects... ".into(),
                "Press ".into(),
                "Esc".bold(),
                " to exit quickshot, ".into(),
                "Enter".bold(),
                " to open project, ".into(),
                "Tab".bold(),
                " to enter tab select".into(),
            ],
            Style::default(),
        ),
    };
    let text = Text::from(Line::from(msg)).patch_style(style);
    frame.render_widget(Paragraph::new(text), help_area);

    // --- Input Box ---
    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Tabbing => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::bordered().title("Search"));
    frame.render_widget(input, input_area);

    if let InputMode::Editing = app.input_mode {
        frame.set_cursor_position(Position::new(
            input_area.x + app.character_index as u16 + 1,
            input_area.y + 1,
        ));
    }

    // --- Project List ---
    let items: Vec<ListItem> = app
        .filtered_paths
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let content = Line::from(Span::raw(format!("{i}: {}", path.to_string_lossy())));
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items).block(Block::bordered().title("Projects"));
    frame.render_widget(list, list_area);
}

