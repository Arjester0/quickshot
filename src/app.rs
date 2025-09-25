use std::path::{Path, PathBuf};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};
use std::error::Error;
use std::process::Command;
use std::io;

pub struct App {
    input: String,
    input_mode: InputMode,
    character_index: usize,
    paths: Vec<PathBuf>,
} 

enum InputMode {
    Editing,
    Tabbing,
} 

impl App {
    pub const fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Editing,
            character_index: 0,
            paths: Vec::new(),
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left); 
    } 

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right); 
    } 

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index(); 
        self.input.insert(index, new_char); 
        self.move_cursor_right(); 
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0; 
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect(); 
            self.move_cursor_left();
        } 
    } 

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    } 

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn select_project(&mut self) {
        self.input.clear();
        self.reset_cursor();
        let input_content: usize = self.input.parse().expect("balls");
        let path_strings: Vec<String> = self
            .paths
            .iter()
            .enumerate()
            .map(|(i, m)| m.to_string_lossy().into_owned())
            .collect();
        let dir = path_strings[input_content].clone(); 
        let name = dir.clone(); 
        // iterating through paths and picking based off number 
        // TODO: make this a string to string search 
        self.open_or_create_tmux_session(name.trim(), dir); 
    }

    fn open_or_create_tmux_session(&self, name: &str, dir: String) -> io::Result<()> {
        // Spawn tmux attached to the current terminal
        let status = Command::new("tmux")
            .arg("new-session")
            .arg("-As").arg(name)   // attach if exists, else create
            .arg("-c").arg(dir)     // set starting directory
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, format!("tmux exited with {}", status)))
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal, paths: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
        self.paths = paths;
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Tabbing => match key.code {
                        KeyCode::Char('e') => {
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Esc => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.select_project(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => return Ok(()), 
                        KeyCode::Tab => self.input_mode = InputMode::Tabbing,
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, messages_area] = vertical.areas(frame.area());

        let (msg, style) = match self.input_mode {
            InputMode::Tabbing=> (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to enter text input mode, ".bold(),
                    "Enter".bold(),
                    " to select project session".into(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing => (
                vec![
                    "Type in the number of what project session you want to open... ".into(),
                    "Press ".into(),
                    "Esc".bold(),
                    " to exit quickshot, ".into(),
                    "Enter".bold(),
                    " to select the project session, ".into(),
                    "Tab".bold(),
                    " to enter tab select".into(),
                ],
                Style::default(),
            ),
        };
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Tabbing => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, input_area);
        match self.input_mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            InputMode::Tabbing => {}

            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                input_area.x + self.character_index as u16 + 1,
                // Move one line down, from the border to the input line
                input_area.y + 1,
            )),
        }


        let paths: Vec<ListItem> = self
            .paths
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {}", m.to_string_lossy())));
                ListItem::new(content)
            })
            .collect();
        let paths = List::new(paths).block(Block::bordered().title("Projects"));
        frame.render_widget(paths, messages_area);
    }
} 
