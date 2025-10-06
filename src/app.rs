use std::{
    error::Error,
    io,
    path::PathBuf,
    process::Command,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    DefaultTerminal,
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::ui::draw_ui;

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub character_index: usize,
    pub paths: Vec<PathBuf>,
    pub filtered_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
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
            filtered_paths: Vec::new(),
        }
    }

    fn update_filter(&mut self) {
        if self.input.is_empty() {
            self.filtered_paths = self.paths.clone();
            return;
        }

        let matcher = SkimMatcherV2::default();
        let mut scored: Vec<_> = self
            .paths
            .iter()
            .filter_map(|p| {
                let text = p.to_string_lossy();
                matcher.fuzzy_match(&text, &self.input).map(|score| (score, p.clone()))
            })
            .collect();

        scored.sort_by_key(|(score, _)| -score);
        self.filtered_paths = scored.into_iter().map(|(_, p)| p).collect();
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
        self.update_filter();
    }

    fn delete_char(&mut self) {
        if self.character_index != 0 {
            let current_index = self.character_index;
            let from_left_to_current = current_index - 1;

            let before = self.input.chars().take(from_left_to_current);
            let after = self.input.chars().skip(current_index);

            self.input = before.chain(after).collect();
            self.move_cursor_left();
        }
        self.update_filter();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn select_project(&mut self) {
        if let Some(first_match) = self.filtered_paths.first() {
            let name = first_match
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let dir = first_match.to_string_lossy().to_string();
            let _ = self.open_or_create_tmux_session(&name, dir);
        }
    }

    fn open_or_create_tmux_session(&self, name: &str, dir: String) -> io::Result<()> {
        let status = Command::new("tmux")
            .arg("new-session")
            .arg("-As")
            .arg(name)
            .arg("-c")
            .arg(dir)
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("tmux exited with {}", status),
            ))
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal, paths: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
        self.paths = paths;
        self.filtered_paths = self.paths.clone();

        loop {
            terminal.draw(|f| draw_ui(f, &self))?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Tabbing => match key.code {
                        KeyCode::Char('e') => self.input_mode = InputMode::Editing,
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.select_project(),
                        KeyCode::Char(c) => self.enter_char(c),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Tab => self.input_mode = InputMode::Tabbing,
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}

